import * as vscode from 'vscode';
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
  TransportKind,
} from 'vscode-languageclient/node';

let client: LanguageClient | undefined;

// ============================================================
// Language Registry
// ============================================================

interface LanguageInfo {
  /** mds file extension (e.g., '.ts.md') */
  ext: string;
  /** VS Code language ID (e.g., 'typescript') */
  languageId: string;
  /** Code block labels that identify this language */
  labels: string[];
  /** Virtual file extension for embedded docs (e.g., '.ts') */
  virtualExt: string;
}

const LANGUAGE_REGISTRY: Record<string, LanguageInfo> = {};

/** Dynamically create a LanguageInfo for any unknown extension */
function createDynamicLanguageInfo(key: string): LanguageInfo {
  return {
    ext: `.${key}.md`,
    languageId: key,
    labels: [key],
    virtualExt: `.${key}`,
  };
}

const LANG_ALIASES: Record<string, string> = {};

function normalizeLangKey(key: string): string {
  const lower = key.toLowerCase();
  return LANG_ALIASES[lower] || lower;
}

/** Build a label → LanguageInfo lookup map */
function buildLabelMap(): Map<string, LanguageInfo> {
  const map = new Map<string, LanguageInfo>();
  for (const info of Object.values(LANGUAGE_REGISTRY)) {
    for (const label of info.labels) {
      map.set(label.toLowerCase(), info);
    }
  }
  return map;
}

let labelToLang = buildLabelMap();

// ============================================================
// Config-based Language Discovery
// ============================================================

/**
 * Discover active languages from mds.config.toml files in the workspace
 * and user settings. Core languages (ts, py, rs) are always included.
 */
async function discoverLanguages(
  config: vscode.WorkspaceConfiguration
): Promise<LanguageInfo[]> {
  await discoverDescriptorLanguages();
  const langKeys = new Set<string>(Object.keys(LANGUAGE_REGISTRY));

  // From user settings (additionalLanguages: ['.go.md', '.java.md'])
  for (const ext of config.get<string[]>('additionalLanguages', [])) {
    const m = ext.match(/^\.(\w+)\.md$/);
    if (m) {
      langKeys.add(normalizeLangKey(m[1]));
    }
  }

  // From workspace mds.config.toml files
  try {
    const files = await vscode.workspace.findFiles(
      '**/mds.config.toml',
      '{**/node_modules/**,**/target/**}'
    );
    for (const uri of files) {
      try {
        const bytes = await vscode.workspace.fs.readFile(uri);
        const text = new TextDecoder().decode(bytes);
        // Match [quality.LANG] or [adapters.LANG] sections
        const re = /\[(?:quality|adapters)\.(\w+)\]/g;
        let match;
        while ((match = re.exec(text)) !== null) {
          langKeys.add(normalizeLangKey(match[1]));
        }
      } catch {
        // Skip unreadable files
      }
    }
  } catch {
    // Workspace search not available
  }

  // From actual .{ext}.md files in mds authoring roots
  try {
    for (const pattern of [
      '**/src-md/**/*.md',
      '**/.mds/source/**/*.md',
      '**/.mds/test/**/*.md',
    ]) {
      const mdFiles = await vscode.workspace.findFiles(
        pattern,
        '{**/node_modules/**,**/target/**}'
      );
      for (const uri of mdFiles) {
        const fileName = uri.path.split('/').pop() || '';
        const m = fileName.match(/\.(\w+)\.md$/);
        if (m) {
          langKeys.add(normalizeLangKey(m[1]));
        }
      }
    }
  } catch {
    // Workspace search not available
  }

  return [...langKeys]
    .map((k) => LANGUAGE_REGISTRY[k] || createDynamicLanguageInfo(k));
}

async function discoverDescriptorLanguages(): Promise<void> {
  try {
    const files = await vscode.workspace.findFiles(
      '**/descriptors/languages/**/*.toml',
      '{**/node_modules/**,**/target/**}'
    );
    for (const uri of files) {
      try {
        const text = new TextDecoder().decode(await vscode.workspace.fs.readFile(uri));
        const info = languageInfoFromDescriptor(text);
        if (info) {
          LANGUAGE_REGISTRY[normalizeLangKey(info.labels[0])] = info;
          for (const label of info.labels) {
            LANG_ALIASES[label.toLowerCase()] = normalizeLangKey(info.labels[0]);
          }
        }
      } catch {
        // Skip unreadable or incomplete descriptors.
      }
    }
    labelToLang = buildLabelMap();
  } catch {
    // Workspace search not available.
  }
}

function languageInfoFromDescriptor(text: string): LanguageInfo | undefined {
  const id = stringField(text, 'id');
  const primaryExt = sectionStringField(text, 'language', 'primary_ext') || id;
  if (!id || !primaryExt) {
    return undefined;
  }
  const aliases = arrayField(text, 'aliases');
  const labels = [...new Set([id, ...aliases, primaryExt])];
  const vscodeId = sectionStringField(text, 'language', 'vscode_id') || id;
  return {
    ext: `.${primaryExt}.md`,
    languageId: vscodeId,
    labels,
    virtualExt: `.${primaryExt}`,
  };
}

function stringField(text: string, key: string): string | undefined {
  return text.match(new RegExp(`^${key}\\s*=\\s*"([^"]+)"`, 'm'))?.[1];
}

function sectionStringField(text: string, section: string, key: string): string | undefined {
  const match = text.match(new RegExp(`\\[${section}\\]([\\s\\S]*?)(?:\\n\\[|$)`));
  return match ? stringField(match[1], key) : undefined;
}

function arrayField(text: string, key: string): string[] {
  const values = text.match(new RegExp(`^${key}\\s*=\\s*\\[([^\\]]*)\\]`, 'm'))?.[1] || '';
  return [...values.matchAll(/"([^"]+)"/g)].map((match) => match[1]);
}

// ============================================================
// Code Block Parsing
// ============================================================

interface CodeBlock {
  /** VS Code language ID (e.g., 'typescript') */
  languageId: string;
  /** Virtual file extension (e.g., '.ts') */
  virtualExt: string;
  /** First content line (line after opening ```) */
  startLine: number;
  /** Last content line (line before closing ```) */
  endLine: number;
  /** Index among all code blocks in the document */
  index: number;
}

interface EmbeddedDocumentProvider extends vscode.TextDocumentContentProvider {
  refresh(uri: vscode.Uri): void;
}

function parseCodeBlocks(document: vscode.TextDocument): CodeBlock[] {
  const blocks: CodeBlock[] = [];
  let inBlock = false;
  let fenceLen = 0;
  let langId = '';
  let vExt = '';
  let start = 0;

  for (let i = 0; i < document.lineCount; i++) {
    const text = document.lineAt(i).text;
    if (!inBlock) {
      const opened = parseFenceLine(text);
      if (opened && opened.label) {
        const label = opened.label.toLowerCase();
        const info = labelToLang.get(label);
        inBlock = true;
        fenceLen = opened.markerLen;
        langId = info?.languageId || label;
        vExt = info?.virtualExt || `.${label}`;
        start = i + 1;
      }
    } else if (isClosingFence(text, fenceLen)) {
      if (i > start) {
        blocks.push({
          languageId: langId,
          virtualExt: vExt,
          startLine: start,
          endLine: i - 1,
          index: blocks.length,
        });
      }
      inBlock = false;
      fenceLen = 0;
    }
  }
  return blocks;
}

function parseFenceLine(text: string): { markerLen: number; label: string } | undefined {
  const trimmed = text.trimStart();
  const markerLen = trimmed.match(/^`*/)?.[0].length || 0;
  if (markerLen < 3) {
    return undefined;
  }
  const rest = trimmed.slice(markerLen).trim();
  const label = rest.split(/\s+/)[0] || '';
  return { markerLen, label };
}

function isClosingFence(text: string, openLen: number): boolean {
  const parsed = parseFenceLine(text);
  return !!parsed && parsed.markerLen >= openLen && parsed.label === '';
}

/** Cached code blocks per document URI */
const blockCache = new Map<string, CodeBlock[]>();

function getCodeBlocks(document: vscode.TextDocument): CodeBlock[] {
  const key = document.uri.toString();
  let blocks = blockCache.get(key);
  if (!blocks) {
    blocks = parseCodeBlocks(document);
    blockCache.set(key, blocks);
  }
  return blocks;
}

function findBlockAtPosition(
  document: vscode.TextDocument,
  line: number
): CodeBlock | undefined {
  return getCodeBlocks(document).find(
    (b) => line >= b.startLine && line <= b.endLine
  );
}

function extractBlockContent(
  document: vscode.TextDocument,
  block: CodeBlock
): string {
  const lines: string[] = [];
  for (
    let i = block.startLine;
    i <= block.endLine && i < document.lineCount;
    i++
  ) {
    lines.push(document.lineAt(i).text);
  }
  return lines.join('\n');
}

// ============================================================
// Embedded Language Support
// ============================================================

/**
 * Cache for shadow documents used for embedded language delegation.
 * Key: "{sourceUri}#{blockIndex}", Value: { content, doc }
 */
const shadowCache = new Map<
  string,
  { content: string; doc: vscode.TextDocument }
>();

let embeddedProvider: EmbeddedDocumentProvider | undefined;

function shadowCacheKey(sourceUri: string, blockIndex: number): string {
  return `${sourceUri}#${blockIndex}`;
}

/**
 * Get or create a shadow document for a code block.
 * Shadow documents are untitled documents with the appropriate language ID,
 * allowing VS Code's built-in language services to provide features.
 */
async function getOrCreateShadowDoc(
  document: vscode.TextDocument,
  block: CodeBlock
): Promise<vscode.TextDocument | undefined> {
  const key = shadowCacheKey(document.uri.toString(), block.index);
  const content = extractBlockContent(document, block);

  const cached = shadowCache.get(key);
  if (cached && cached.content === content && !cached.doc.isClosed) {
    return cached.doc;
  }

  try {
    const doc = await vscode.workspace.openTextDocument(embeddedBlockUri(document, block));
    if (doc.languageId !== block.languageId) {
      await vscode.languages.setTextDocumentLanguage(doc, block.languageId);
    }
    shadowCache.set(key, { content, doc });
    return doc;
  } catch {
    return undefined;
  }
}

function embeddedBlockUri(
  document: vscode.TextDocument,
  block: CodeBlock
): vscode.Uri {
  const sourcePath = document.uri.path.replace(/\.md$/, '');
  return vscode.Uri.from({
    scheme: 'mds-embedded',
    authority: document.uri.authority,
    path: `${sourcePath}.block${block.index}${block.virtualExt}`,
    query: new URLSearchParams({
      source: document.uri.toString(),
      block: block.index.toString(),
      startLine: block.startLine.toString(),
      endLine: block.endLine.toString(),
    }).toString(),
  });
}

function sourcePositionFromEmbedded(
  block: CodeBlock,
  position: vscode.Position
): vscode.Position {
  return new vscode.Position(position.line + block.startLine, position.character);
}

function sourceRangeFromEmbedded(
  block: CodeBlock,
  range: vscode.Range
): vscode.Range {
  return new vscode.Range(
    sourcePositionFromEmbedded(block, range.start),
    sourcePositionFromEmbedded(block, range.end)
  );
}

function mapEmbeddedLocationToSource(
  sourceDocument: vscode.TextDocument,
  block: CodeBlock,
  location: vscode.Location
): vscode.Location {
  const embeddedUri = embeddedBlockUri(sourceDocument, block).toString();
  if (location.uri.toString() !== embeddedUri) {
    return location;
  }
  return new vscode.Location(
    sourceDocument.uri,
    sourceRangeFromEmbedded(block, location.range)
  );
}

function mapCompletionItemToSource(
  block: CodeBlock,
  item: vscode.CompletionItem
): vscode.CompletionItem {
  const textEdit = item.textEdit;
  if (textEdit instanceof vscode.TextEdit) {
    item.textEdit = new vscode.TextEdit(
      sourceRangeFromEmbedded(block, textEdit.range),
      textEdit.newText
    );
  }
  item.additionalTextEdits = item.additionalTextEdits?.map(
    (edit) => new vscode.TextEdit(sourceRangeFromEmbedded(block, edit.range), edit.newText)
  );
  return item;
}

/**
 * Register embedded language feature providers for code blocks.
 * These providers detect when the cursor is inside a code block and
 * delegate to the appropriate language's built-in providers via
 * shadow documents.
 */
function registerEmbeddedLanguageProviders(
  context: vscode.ExtensionContext
): void {
  const selector: vscode.DocumentSelector = { language: 'mds-markdown' };

  // Completion provider for embedded code blocks
  context.subscriptions.push(
    vscode.languages.registerCompletionItemProvider(
      selector,
      {
        async provideCompletionItems(document, position, _token, context) {
          const block = findBlockAtPosition(document, position.line);
          if (!block) {
            return undefined;
          }

          const shadowDoc = await getOrCreateShadowDoc(document, block);
          if (!shadowDoc) {
            return undefined;
          }

          const virtualPos = new vscode.Position(
            position.line - block.startLine,
            position.character
          );

          try {
            const result =
              await vscode.commands.executeCommand<vscode.CompletionList>(
                'vscode.executeCompletionItemProvider',
                shadowDoc.uri,
                virtualPos,
                context.triggerCharacter
              );
            if (!result) {
              return undefined;
            }
            result.items = result.items.map((item) =>
              mapCompletionItemToSource(block, item)
            );
            return result;
          } catch {
            return undefined;
          }
        },
      },
      '.',
      ':',
      '('
    )
  );

  // Hover provider for embedded code blocks
  context.subscriptions.push(
    vscode.languages.registerHoverProvider(selector, {
      async provideHover(document, position) {
        const block = findBlockAtPosition(document, position.line);
        if (!block) {
          return undefined;
        }

        const shadowDoc = await getOrCreateShadowDoc(document, block);
        if (!shadowDoc) {
          return undefined;
        }

        const virtualPos = new vscode.Position(
          position.line - block.startLine,
          position.character
        );

        try {
          const hovers = await vscode.commands.executeCommand<vscode.Hover[]>(
            'vscode.executeHoverProvider',
            shadowDoc.uri,
            virtualPos
          );
          if (hovers && hovers.length > 0) {
            return hovers[0];
          }
        } catch {
          // Fall through
        }
        return undefined;
      },
    })
  );

  // Definition provider for embedded code blocks
  context.subscriptions.push(
    vscode.languages.registerDefinitionProvider(selector, {
      async provideDefinition(document, position) {
        const block = findBlockAtPosition(document, position.line);
        if (!block) {
          return undefined;
        }

        const shadowDoc = await getOrCreateShadowDoc(document, block);
        if (!shadowDoc) {
          return undefined;
        }

        const virtualPos = new vscode.Position(
          position.line - block.startLine,
          position.character
        );

        try {
          const locations =
            await vscode.commands.executeCommand<vscode.Location[]>(
              'vscode.executeDefinitionProvider',
              shadowDoc.uri,
              virtualPos
            );
          return locations?.map((location) =>
            mapEmbeddedLocationToSource(document, block, location)
          ) || undefined;
        } catch {
          return undefined;
        }
      },
    })
  );
}

/**
 * Register the virtual document content provider for the mds-embedded scheme.
 * This provides document content for embedded code block URIs.
 */
function registerVirtualDocumentProvider(
  context: vscode.ExtensionContext
): void {
  const scheme = 'mds-embedded';

  embeddedProvider = new (class implements EmbeddedDocumentProvider {
    private readonly onDidChangeEmitter = new vscode.EventEmitter<vscode.Uri>();
    readonly onDidChange = this.onDidChangeEmitter.event;

    refresh(uri: vscode.Uri): void {
      this.onDidChangeEmitter.fire(uri);
    }

    provideTextDocumentContent(uri: vscode.Uri): string {
      const params = new URLSearchParams(uri.query);
      const sourceUriStr = params.get('source');
      const startLine = parseInt(params.get('startLine') || '0', 10);
      const endLine = parseInt(params.get('endLine') || '0', 10);

      if (!sourceUriStr) {
        return '';
      }

      const sourceUri = vscode.Uri.parse(sourceUriStr);
      const doc = vscode.workspace.textDocuments.find(
        (d) => d.uri.toString() === sourceUri.toString()
      );
      if (!doc) {
        return '';
      }

      const lines: string[] = [];
      for (let i = startLine; i <= endLine && i < doc.lineCount; i++) {
        lines.push(doc.lineAt(i).text);
      }
      return lines.join('\n');
    }
  })();

  context.subscriptions.push(
    vscode.workspace.registerTextDocumentContentProvider(scheme, embeddedProvider)
  );
}

function registerPreviewCommands(context: vscode.ExtensionContext): void {
  async function preview(command: string, uri?: vscode.Uri): Promise<void> {
    const target = uri || vscode.window.activeTextEditor?.document.uri;
    if (!target) {
      return;
    }
    await vscode.commands.executeCommand(command, target);
  }

  context.subscriptions.push(
    vscode.commands.registerCommand('mds.openPreview', (uri?: vscode.Uri) =>
      preview('markdown.showPreview', uri)
    ),
    vscode.commands.registerCommand('mds.openPreviewToSide', (uri?: vscode.Uri) =>
      preview('markdown.showPreviewToSide', uri)
    )
  );
}

// ============================================================
// mds File Detection
// ============================================================

/**
 * Determine if a file URI is an mds-managed file.
 * True if: in an mds authoring root, or matches a known .{ext}.md pattern.
 */
function isMdsFile(uri: vscode.Uri, extPattern: string): boolean {
  const path = uri.fsPath;
  // Files in legacy src-md/ or fixed .mds/source/.mds/test directories
  if (/[/\\](?:src-md|\.mds[/\\](?:source|test))[/\\]/.test(path)) {
    return true;
  }
  // Files matching known mds extension patterns (e.g., .ts.md, .go.md)
  if (extPattern) {
    const re = new RegExp(`(?:${extPattern})$`);
    if (re.test(path)) {
      return true;
    }
  }
  return false;
}

// ============================================================
// Activation
// ============================================================

export async function activate(context: vscode.ExtensionContext) {
  const config = vscode.workspace.getConfiguration('mds.lsp');
  if (!config.get<boolean>('enabled', true)) {
    return;
  }

  const serverPath = resolveServerPath(config, context.extensionPath);
  if (!serverPath) {
    vscode.window.showWarningMessage(
      'mds-lsp binary not found. Install with: cargo install --git https://github.com/owo-x-project/owox-mds mds-lsp, or set mds.lsp.path in settings.'
    );
    return;
  }

  // Discover active languages from config files and settings
  const activeLanguages = await discoverLanguages(config);

  const logLevel = config.get<string>('logLevel', 'info');

  const serverOptions: ServerOptions = {
    command: serverPath,
    args: [],
    transport: TransportKind.stdio,
    options: {
      env: {
        ...process.env,
        RUST_LOG: `mds_lsp=${logLevel}`,
      },
    },
  };

  // Dynamic document selector based on discovered languages
  const documentSelector: {
    scheme: string;
    language?: string;
    pattern?: string;
  }[] = [
      { scheme: 'file', language: 'mds-markdown' },
      { scheme: 'file', pattern: '**/mds.config.toml' },
      { scheme: 'file', pattern: '**/package.md' },
      { scheme: 'file', pattern: '**/src-md/**/*.md' },
      { scheme: 'file', pattern: '**/.mds/source/**/*.md' },
      { scheme: 'file', pattern: '**/.mds/test/**/*.md' },
    ];
  for (const lang of activeLanguages) {
    documentSelector.push({ scheme: 'file', pattern: `**/*${lang.ext}` });
  }

  // Dynamic file watchers based on discovered languages
  const fileEvents = [
    vscode.workspace.createFileSystemWatcher('**/mds.config.toml'),
    vscode.workspace.createFileSystemWatcher('**/package.md'),
    vscode.workspace.createFileSystemWatcher('**/src-md/**/*.md'),
    vscode.workspace.createFileSystemWatcher('**/.mds/source/**/*.md'),
    vscode.workspace.createFileSystemWatcher('**/.mds/test/**/*.md'),
  ];
  for (const lang of activeLanguages) {
    fileEvents.push(
      vscode.workspace.createFileSystemWatcher(`**/*${lang.ext}`)
    );
  }

  const clientOptions: LanguageClientOptions = {
    documentSelector,
    synchronize: { fileEvents },
    outputChannel: vscode.window.createOutputChannel('mds Language Server'),
  };

  client = new LanguageClient(
    'mds-lsp',
    'mds Language Server',
    serverOptions,
    clientOptions
  );

  // Register embedded language support
  registerPreviewCommands(context);
  registerVirtualDocumentProvider(context);
  registerEmbeddedLanguageProviders(context);

  // Invalidate code block cache on document changes
  context.subscriptions.push(
    vscode.workspace.onDidChangeTextDocument((e) => {
      const uriStr = e.document.uri.toString();
      blockCache.delete(uriStr);
      // Invalidate shadow docs for changed source
      for (const key of shadowCache.keys()) {
        if (key.startsWith(uriStr + '#')) {
          const cached = shadowCache.get(key);
          if (cached) {
            embeddedProvider?.refresh(cached.doc.uri);
          }
          shadowCache.delete(key);
        }
      }
    }),
    vscode.workspace.onDidCloseTextDocument((doc) => {
      const uriStr = doc.uri.toString();
      blockCache.delete(uriStr);
      for (const key of shadowCache.keys()) {
        if (key.startsWith(uriStr + '#')) {
          shadowCache.delete(key);
        }
      }
    })
  );

  // Watch for config changes to detect new language extensions
  context.subscriptions.push(
    vscode.workspace.onDidChangeConfiguration((e) => {
      if (e.affectsConfiguration('mds.lsp')) {
        vscode.window
          .showInformationMessage(
            'mds LSP configuration changed. Restart to apply.',
            'Restart'
          )
          .then((selection) => {
            if (selection === 'Restart') {
              vscode.commands.executeCommand('workbench.action.reloadWindow');
            }
          });
      }
    })
  );

  // Auto-associate .md files in mds authoring roots with mds-markdown language
  const mdsExtPattern = activeLanguages.map((l) => l.ext).join('|').replace(/\./g, '\\.');
  context.subscriptions.push(
    vscode.workspace.onDidOpenTextDocument((doc) => {
      if (doc.languageId === 'markdown' && isMdsFile(doc.uri, mdsExtPattern)) {
        vscode.languages.setTextDocumentLanguage(doc, 'mds-markdown');
      }
    })
  );

  // Set language for already-open documents
  for (const doc of vscode.workspace.textDocuments) {
    if (doc.languageId === 'markdown' && isMdsFile(doc.uri, mdsExtPattern)) {
      vscode.languages.setTextDocumentLanguage(doc, 'mds-markdown');
    }
  }

  await client.start();
}

export async function deactivate(): Promise<void> {
  if (client) {
    await client.stop();
  }
}

function resolveServerPath(
  config: vscode.WorkspaceConfiguration,
  extensionPath: string
): string | undefined {
  const configPath = config.get<string>('path', '');
  if (configPath) {
    return configPath;
  }

  const { execFileSync } = require('child_process');
  const path = require('path');
  const fs = require('fs');

  const platformKey = bundledPlatformKey();
  if (platformKey) {
    const bundled = path.join(
      extensionPath,
      'server',
      platformKey,
      process.platform === 'win32' ? 'mds-lsp.exe' : 'mds-lsp'
    );
    if (fs.existsSync(bundled)) {
      return bundled;
    }
  }

  // Try to find mds-lsp in PATH
  try {
    const cmd = process.platform === 'win32' ? 'where' : 'which';
    const result = execFileSync(cmd, ['mds-lsp'], {
      encoding: 'utf-8',
      timeout: 5000,
    }).trim();
    if (result) {
      return result.split('\n')[0].trim();
    }
  } catch {
    // Not found in PATH
  }

  // Try to find mds-lsp next to the mds binary
  try {
    const cmd = process.platform === 'win32' ? 'where' : 'which';
    const mdsPath = execFileSync(cmd, ['mds'], {
      encoding: 'utf-8',
      timeout: 5000,
    }).trim().split('\n')[0].trim();
    if (mdsPath) {
      const lspPath = path.join(path.dirname(mdsPath), 'mds-lsp');
      const lspPathExt = process.platform === 'win32' ? lspPath + '.exe' : lspPath;
      if (fs.existsSync(lspPathExt)) {
        return lspPathExt;
      }
    }
  } catch {
    // mds not found in PATH either
  }

  // Try common cargo install location
  const home = process.env.HOME || process.env.USERPROFILE || '';
  if (home) {
    const cargoLsp = path.join(home, '.cargo', 'bin', 'mds-lsp');
    const cargoLspExt = process.platform === 'win32' ? cargoLsp + '.exe' : cargoLsp;
    if (fs.existsSync(cargoLspExt)) {
      return cargoLspExt;
    }
  }

  return undefined;
}

function bundledPlatformKey(): string | undefined {
  if (process.platform === 'linux' && process.arch === 'x64') {
    return 'linux-x64';
  }
  if (process.platform === 'darwin' && process.arch === 'arm64') {
    return 'darwin-arm64';
  }
  if (process.platform === 'win32' && process.arch === 'x64') {
    return 'win32-x64';
  }
  return undefined;
}

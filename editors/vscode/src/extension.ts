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

/**
 * Registry of known languages.
 * Add entries here to support new languages automatically.
 */
const LANGUAGE_REGISTRY: Record<string, LanguageInfo> = {
  // Core languages (supported by mds-core Lang enum)
  ts: {
    ext: '.ts.md',
    languageId: 'typescript',
    labels: ['typescript', 'ts'],
    virtualExt: '.ts',
  },
  py: {
    ext: '.py.md',
    languageId: 'python',
    labels: ['python', 'py'],
    virtualExt: '.py',
  },
  rs: {
    ext: '.rs.md',
    languageId: 'rust',
    labels: ['rust', 'rs'],
    virtualExt: '.rs',
  },
  // Add new languages here when mds-core gains support.
  // Example:
  // go: {
  //   ext: '.go.md',
  //   languageId: 'go',
  //   labels: ['go', 'golang'],
  //   virtualExt: '.go',
  // },
};

/** Alias map for normalizing language keys */
const LANG_ALIASES: Record<string, string> = {
  typescript: 'ts',
  python: 'py',
  rust: 'rs',
};

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

const labelToLang = buildLabelMap();

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
  const langKeys = new Set<string>(['ts', 'py', 'rs']);

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

  return [...langKeys]
    .map((k) => LANGUAGE_REGISTRY[k])
    .filter((info): info is LanguageInfo => !!info);
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

function parseCodeBlocks(document: vscode.TextDocument): CodeBlock[] {
  const blocks: CodeBlock[] = [];
  const openRe = /^\s*```(\w+)(?:\s.*)?$/;
  const closeRe = /^\s*```\s*$/;
  let inBlock = false;
  let langId = '';
  let vExt = '';
  let start = 0;

  for (let i = 0; i < document.lineCount; i++) {
    const text = document.lineAt(i).text;
    if (!inBlock) {
      const m = openRe.exec(text);
      if (m) {
        const label = m[1].toLowerCase();
        const info = labelToLang.get(label);
        inBlock = true;
        langId = info?.languageId || label;
        vExt = info?.virtualExt || `.${label}`;
        start = i + 1;
      }
    } else if (closeRe.test(text)) {
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
    }
  }
  return blocks;
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
    const doc = await vscode.workspace.openTextDocument({
      language: block.languageId,
      content,
    });
    shadowCache.set(key, { content, doc });
    return doc;
  } catch {
    return undefined;
  }
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
            return result || undefined;
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
          return locations || undefined;
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

  const provider = new (class implements vscode.TextDocumentContentProvider {
    onDidChangeEmitter = new vscode.EventEmitter<vscode.Uri>();
    onDidChange = this.onDidChangeEmitter.event;

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
    vscode.workspace.registerTextDocumentContentProvider(scheme, provider)
  );
}

// ============================================================
// Activation
// ============================================================

export async function activate(context: vscode.ExtensionContext) {
  const config = vscode.workspace.getConfiguration('mds.lsp');
  if (!config.get<boolean>('enabled', true)) {
    return;
  }

  const serverPath = resolveServerPath(config);
  if (!serverPath) {
    vscode.window.showWarningMessage(
      'mds-lsp binary not found. Install it or set mds.lsp.path in settings.'
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
    ];
  for (const lang of activeLanguages) {
    documentSelector.push({ scheme: 'file', pattern: `**/*${lang.ext}` });
  }

  // Dynamic file watchers based on discovered languages
  const fileEvents = [
    vscode.workspace.createFileSystemWatcher('**/mds.config.toml'),
    vscode.workspace.createFileSystemWatcher('**/package.md'),
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

  await client.start();
}

export async function deactivate(): Promise<void> {
  if (client) {
    await client.stop();
  }
}

function resolveServerPath(
  config: vscode.WorkspaceConfiguration
): string | undefined {
  const configPath = config.get<string>('path', '');
  if (configPath) {
    return configPath;
  }

  // Try to find mds-lsp in PATH
  const { execFileSync } = require('child_process');
  try {
    const cmd = process.platform === 'win32' ? 'where' : 'which';
    const result = execFileSync(cmd, ['mds-lsp'], {
      encoding: 'utf-8',
      timeout: 5000,
    }).trim();
    if (result) {
      // Return the first line (which/where may return multiple)
      return result.split('\n')[0].trim();
    }
  } catch {
    // Not found in PATH
  }

  return undefined;
}

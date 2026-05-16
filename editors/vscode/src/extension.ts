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
// Generated File LSP Bridge
// ============================================================

const RESOLVE_GENERATED_POSITION_COMMAND = 'mds.resolveGeneratedPosition';
const REMAP_GENERATED_LOCATIONS_COMMAND = 'mds.remapGeneratedLocations';
const REMAP_GENERATED_RANGE_COMMAND = 'mds.remapGeneratedRange';

interface BridgePosition {
  line: number;
  character: number;
}

interface BridgeRange {
  start: BridgePosition;
  end: BridgePosition;
}

interface BridgeLocation {
  uri: string;
  range: BridgeRange;
}

type DiagnosticsByMarkdownUri = Map<
  string,
  { uri: vscode.Uri; diagnostics: vscode.Diagnostic[] }
>;

const generatedDiagnosticMirrorCache = new Map<string, DiagnosticsByMarkdownUri>();

let generatedDiagnosticsCollection: vscode.DiagnosticCollection | undefined;

async function executeBridgeCommand<T>(
  command: string,
  args: Record<string, unknown>
): Promise<T | undefined> {
  if (!client) {
    return undefined;
  }

  try {
    const result = await client.sendRequest<T | null>('workspace/executeCommand', {
      command,
      arguments: [args],
    });
    return result ?? undefined;
  } catch {
    return undefined;
  }
}

function toBridgePosition(position: vscode.Position): BridgePosition {
  return {
    line: position.line,
    character: position.character,
  };
}

function fromBridgePosition(position: BridgePosition): vscode.Position {
  return new vscode.Position(position.line, position.character);
}

function toBridgeRange(range: vscode.Range): BridgeRange {
  return {
    start: toBridgePosition(range.start),
    end: toBridgePosition(range.end),
  };
}

function fromBridgeRange(range: BridgeRange): vscode.Range {
  return new vscode.Range(
    fromBridgePosition(range.start),
    fromBridgePosition(range.end)
  );
}

function toBridgeLocation(location: vscode.Location): BridgeLocation {
  return {
    uri: location.uri.toString(),
    range: toBridgeRange(location.range),
  };
}

function fromBridgeLocation(location: BridgeLocation): vscode.Location {
  return new vscode.Location(
    vscode.Uri.parse(location.uri),
    fromBridgeRange(location.range)
  );
}

async function resolveGeneratedPosition(
  markdownUri: vscode.Uri,
  position: vscode.Position
): Promise<vscode.Location | undefined> {
  const resolved = await executeBridgeCommand<BridgeLocation | null>(
    RESOLVE_GENERATED_POSITION_COMMAND,
    {
      markdown_uri: markdownUri.toString(),
      position: toBridgePosition(position),
    }
  );
  return resolved ? fromBridgeLocation(resolved) : undefined;
}

async function remapGeneratedRange(
  uri: vscode.Uri,
  range: vscode.Range
): Promise<vscode.Location | undefined> {
  const remapped = await executeBridgeCommand<BridgeLocation | null>(
    REMAP_GENERATED_RANGE_COMMAND,
    {
      uri: uri.toString(),
      range: toBridgeRange(range),
    }
  );
  return remapped ? fromBridgeLocation(remapped) : undefined;
}

async function remapGeneratedLocations(
  locations: readonly vscode.Location[]
): Promise<Array<vscode.Location | undefined> | undefined> {
  const remapped = await executeBridgeCommand<Array<BridgeLocation | null> | null>(
    REMAP_GENERATED_LOCATIONS_COMMAND,
    {
      locations: locations.map((location) => toBridgeLocation(location)),
    }
  );
  return remapped?.map((location) =>
    location ? fromBridgeLocation(location) : undefined
  );
}

function isDefinitionLink(
  value: vscode.Location | vscode.DefinitionLink
): value is vscode.DefinitionLink {
  return 'targetUri' in value;
}

function definitionTargetsToLocations(
  definitions: vscode.Location[] | vscode.DefinitionLink[] | undefined
): vscode.Location[] {
  if (!definitions) {
    return [];
  }

  return definitions.map((definition) =>
    isDefinitionLink(definition)
      ? new vscode.Location(
        definition.targetUri,
        definition.targetSelectionRange || definition.targetRange
      )
      : definition
  );
}

async function openGeneratedDocument(
  markdownUri: vscode.Uri,
  position: vscode.Position
): Promise<{ document: vscode.TextDocument; position: vscode.Position } | undefined> {
  const resolved = await resolveGeneratedPosition(markdownUri, position);
  if (!resolved) {
    return undefined;
  }

  try {
    const document = await vscode.workspace.openTextDocument(resolved.uri);
    return {
      document,
      position: resolved.range.start,
    };
  } catch {
    return undefined;
  }
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

        try {
          const generated = await openGeneratedDocument(document.uri, position);
          if (generated) {
            const hovers = await vscode.commands.executeCommand<vscode.Hover[]>(
              'vscode.executeHoverProvider',
              generated.document.uri,
              generated.position
            );
            const hover = hovers?.[0];
            if (hover) {
              if (!hover.range) {
                return hover;
              }

              const remapped = await remapGeneratedRange(
                generated.document.uri,
                hover.range
              );
              if (remapped?.uri.toString() === document.uri.toString()) {
                return new vscode.Hover(hover.contents, remapped.range);
              }
              return new vscode.Hover(hover.contents);
            }
          }
        } catch {
          // Fall through to shadow document fallback.
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

        try {
          const generated = await openGeneratedDocument(document.uri, position);
          if (generated) {
            const definitions =
              await vscode.commands.executeCommand<
                vscode.Location[] | vscode.DefinitionLink[]
              >(
                'vscode.executeDefinitionProvider',
                generated.document.uri,
                generated.position
              );
            const generatedLocations = definitionTargetsToLocations(definitions);
            if (generatedLocations.length > 0) {
              const remapped = await remapGeneratedLocations(generatedLocations);
              if (remapped) {
                const generatedUri = generated.document.uri.toString();
                const mappedLocations = generatedLocations.flatMap((location, index) => {
                  const markdownLocation = remapped[index];
                  if (markdownLocation) {
                    return [markdownLocation];
                  }
                  if (location.uri.toString() === generatedUri) {
                    return [];
                  }
                  return [location];
                });
                if (mappedLocations.length > 0) {
                  return mappedLocations;
                }
              }
            }
          }
        } catch {
          // Fall through to shadow document fallback.
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

// ============================================================
// Generated Diagnostics Mirror
// ============================================================

function isGeneratedDiagnosticUri(uri: vscode.Uri): boolean {
  return uri.scheme === 'file' && !uri.path.endsWith('.md');
}

function cloneDiagnosticForMarkdown(
  range: vscode.Range,
  diagnostic: vscode.Diagnostic
): vscode.Diagnostic {
  const mirrored = new vscode.Diagnostic(
    range,
    diagnostic.message,
    diagnostic.severity
  );
  mirrored.code = diagnostic.code;
  mirrored.source = diagnostic.source || 'generated';
  mirrored.tags = diagnostic.tags ? [...diagnostic.tags] : undefined;
  return mirrored;
}

function appendMirroredDiagnostic(
  diagnosticsByMarkdownUri: DiagnosticsByMarkdownUri,
  location: vscode.Location,
  diagnostic: vscode.Diagnostic
): void {
  const key = location.uri.toString();
  const existing = diagnosticsByMarkdownUri.get(key);
  if (existing) {
    existing.diagnostics.push(
      cloneDiagnosticForMarkdown(location.range, diagnostic)
    );
    return;
  }

  diagnosticsByMarkdownUri.set(key, {
    uri: location.uri,
    diagnostics: [cloneDiagnosticForMarkdown(location.range, diagnostic)],
  });
}

async function remapDiagnosticsForGeneratedUri(
  uri: vscode.Uri
): Promise<DiagnosticsByMarkdownUri> {
  const diagnosticsByMarkdownUri: DiagnosticsByMarkdownUri = new Map();
  const diagnostics = vscode.languages.getDiagnostics(uri);

  await Promise.all(
    diagnostics.map(async (diagnostic) => {
      const location = await remapGeneratedRange(uri, diagnostic.range);
      if (!location || !location.uri.path.endsWith('.md')) {
        return;
      }
      appendMirroredDiagnostic(diagnosticsByMarkdownUri, location, diagnostic);
    })
  );

  return diagnosticsByMarkdownUri;
}

function rebuildGeneratedDiagnosticsCollection(): void {
  if (!generatedDiagnosticsCollection) {
    return;
  }

  const aggregate = new Map<
    string,
    { uri: vscode.Uri; diagnostics: vscode.Diagnostic[] }
  >();
  for (const diagnosticsByMarkdownUri of generatedDiagnosticMirrorCache.values()) {
    for (const [key, value] of diagnosticsByMarkdownUri) {
      const existing = aggregate.get(key);
      if (existing) {
        existing.diagnostics.push(...value.diagnostics);
        continue;
      }
      aggregate.set(key, {
        uri: value.uri,
        diagnostics: [...value.diagnostics],
      });
    }
  }

  generatedDiagnosticsCollection.clear();
  if (aggregate.size === 0) {
    return;
  }

  generatedDiagnosticsCollection.set(
    [...aggregate.values()].map((entry) =>
      [entry.uri, entry.diagnostics] as [vscode.Uri, vscode.Diagnostic[]]
    )
  );
}

async function refreshGeneratedDiagnosticsMirror(
  uris: readonly vscode.Uri[]
): Promise<void> {
  const generatedUris = [...new Map(
    uris
      .filter((uri) => isGeneratedDiagnosticUri(uri))
      .map((uri) => [uri.toString(), uri])
  ).values()];
  if (generatedUris.length === 0) {
    return;
  }

  await Promise.all(
    generatedUris.map(async (uri) => {
      const remapped = await remapDiagnosticsForGeneratedUri(uri);
      const key = uri.toString();
      if (remapped.size === 0) {
        generatedDiagnosticMirrorCache.delete(key);
        return;
      }
      generatedDiagnosticMirrorCache.set(key, remapped);
    })
  );

  rebuildGeneratedDiagnosticsCollection();
}

function registerGeneratedDiagnosticsMirror(
  context: vscode.ExtensionContext
): void {
  generatedDiagnosticsCollection = vscode.languages.createDiagnosticCollection(
    'mds-generated-mirror'
  );

  context.subscriptions.push(
    generatedDiagnosticsCollection,
    vscode.languages.onDidChangeDiagnostics((event) => {
      void refreshGeneratedDiagnosticsMirror(event.uris);
    })
  );

  void refreshGeneratedDiagnosticsMirror(
    vscode.languages.getDiagnostics().map(([uri]) => uri)
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
  // Files in the canonical .mds/source or .mds/test authoring roots
  if (/[/\\]\.mds[/\\](?:source|test)[/\\]/.test(path)) {
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
  registerGeneratedDiagnosticsMirror(context);
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

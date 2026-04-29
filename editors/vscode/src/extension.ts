import * as path from 'path';
import * as vscode from 'vscode';
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
  TransportKind,
} from 'vscode-languageclient/node';

let client: LanguageClient | undefined;

export async function activate(context: vscode.ExtensionContext) {
  const config = vscode.workspace.getConfiguration('mds.lsp');
  const enabled = config.get<boolean>('enabled', true);

  if (!enabled) {
    return;
  }

  const serverPath = resolveServerPath(config);
  if (!serverPath) {
    vscode.window.showWarningMessage(
      'mds-lsp binary not found. Install it or set mds.lsp.path in settings.'
    );
    return;
  }

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

  // Build document selector from known languages + config-based extensions
  const documentSelector = buildDocumentSelector(config);

  const clientOptions: LanguageClientOptions = {
    documentSelector,
    synchronize: {
      fileEvents: [
        vscode.workspace.createFileSystemWatcher('**/mds.config.toml'),
        vscode.workspace.createFileSystemWatcher('**/*.ts.md'),
        vscode.workspace.createFileSystemWatcher('**/*.py.md'),
        vscode.workspace.createFileSystemWatcher('**/*.rs.md'),
      ],
    },
    outputChannel: vscode.window.createOutputChannel('mds Language Server'),
  };

  client = new LanguageClient(
    'mds-lsp',
    'mds Language Server',
    serverOptions,
    clientOptions
  );

  // Register virtual document provider for embedded languages
  registerEmbeddedLanguageSupport(context);

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

function buildDocumentSelector(
  config: vscode.WorkspaceConfiguration
): { scheme: string; language?: string; pattern?: string }[] {
  const selector: { scheme: string; language?: string; pattern?: string }[] = [
    { scheme: 'file', language: 'mds-markdown' },
    { scheme: 'file', pattern: '**/*.ts.md' },
    { scheme: 'file', pattern: '**/*.py.md' },
    { scheme: 'file', pattern: '**/*.rs.md' },
    { scheme: 'file', pattern: '**/mds.config.toml' },
    { scheme: 'file', pattern: '**/package.md' },
  ];

  // Add additional language patterns from config
  const additionalLanguages = config.get<string[]>('additionalLanguages', []);
  for (const ext of additionalLanguages) {
    selector.push({ scheme: 'file', pattern: `**/*${ext}` });
  }

  return selector;
}

/**
 * Register embedded language support for code blocks within mds Markdown.
 *
 * This creates virtual documents for code blocks, enabling language servers
 * for TypeScript, Python, and Rust to provide features within those blocks.
 */
function registerEmbeddedLanguageSupport(
  context: vscode.ExtensionContext
): void {
  const scheme = 'mds-embedded';

  const provider = new (class implements vscode.TextDocumentContentProvider {
    onDidChangeEmitter = new vscode.EventEmitter<vscode.Uri>();
    onDidChange = this.onDidChangeEmitter.event;

    provideTextDocumentContent(uri: vscode.Uri): string {
      // URI format: mds-embedded://{language}/{encoded-path}?startLine={n}&endLine={n}
      const params = new URLSearchParams(uri.query);
      const sourcePath = decodeURIComponent(uri.path.substring(1));
      const startLine = parseInt(params.get('startLine') || '0', 10);
      const endLine = parseInt(params.get('endLine') || '0', 10);

      // Read the source file and extract the code block content
      const doc = vscode.workspace.textDocuments.find(
        (d) => d.uri.fsPath === sourcePath
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

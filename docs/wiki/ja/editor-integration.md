# エディタ統合 (LSP)

mds は Language Server Protocol (LSP) に準拠したサーバー `mds-lsp` を提供しています。エディタ上で mds Markdown ファイルのリアルタイム検証、コードナビゲーション、補完、ホバー情報を利用できます。

**主な機能:**

| 機能 | 説明 |
| --- | --- |
| リアルタイム診断 | セクション構造、テーブル形式、言語一致、config 検証、リンク検証 |
| Go to Definition | Uses テーブルの Target から参照先の実装 Markdown へジャンプ |
| Find References | Expose された名前がどこで Uses されているか検索 |
| Document Symbols | セクション見出しのアウトライン表示 |
| Workspace Symbols | `src-md/` 全体のモジュール名検索 |
| 補完 | セクション名、テーブルカラム名、コードブロック言語、スニペット |
| Hover | セクション説明、参照先モジュールの Purpose 表示 |
| Code Action | 欠損セクションの自動追加（Quick Fix） |

## インストール

### install.sh（推奨）

インストールスクリプトで `mds` と `mds-lsp` の両方がインストールされます:

```bash
curl -fsSL https://raw.githubusercontent.com/owo-x-project/owox-mds/main/install.sh | sh
```

### ソースからビルド（開発者向け）

```bash
./.github/script/sync-build.sh
cd .build/rust
cargo build -p mds-lsp --release
cp target/release/mds-lsp /usr/local/bin/
```

### 動作確認

```bash
mds-lsp --version   # バージョン表示（未対応の場合は stdio で待機開始）
```

`mds-lsp` は stdio トランスポートで動作します。エディタが起動時に自動でプロセスを管理します。

## VSCode

### 拡張のインストール

```bash
cd editors/vscode
npm install
npm run compile
```

開発中は VSCode の「Extension Development Host」で起動:

1. `editors/vscode` を VSCode で開く
2. F5 で Extension Development Host を起動
3. `.ts.md`, `.py.md`, `.rs.md` ファイルを開く

### 設定項目

| 設定 | デフォルト | 説明 |
| --- | --- | --- |
| `mds.lsp.path` | `""` | mds-lsp バイナリのパス。空の場合は PATH から検索 |
| `mds.lsp.enabled` | `true` | LSP サーバーの有効/無効 |
| `mds.lsp.logLevel` | `"info"` | ログレベル: error, warn, info, debug, trace |
| `mds.lsp.additionalLanguages` | `[]` | 追加の言語拡張子（例: `[".go.md", ".java.md"]`） |

### 自動有効化

拡張は `mds.config.toml` がワークスペースに存在する場合に自動的に有効化されます。`mds.config.toml` の `[adapters]` セクションで有効な言語アダプターを検出し、対応する拡張子のファイルに対して LSP 機能を提供します。

新しい言語を追加した場合は `mds.lsp.additionalLanguages` に拡張子を追加するだけで対応できます。

### シンタックスハイライト

mds 固有の TextMate grammar が含まれています:

- **セクション見出し**: `## Purpose`, `## Types` 等がハイライトされます
- **Uses テーブル**: `From` カラムの `builtin`, `package`, `workspace`, `internal` がキーワードとしてハイライト
- **コードブロック**: TypeScript, Python, Rust の埋め込みシンタックスハイライト

### スニペット

VSCode スニペットも含まれています:

| プレフィックス | 説明 |
| --- | --- |
| `mds-doc` | 完全な実装ドキュメントテンプレート |
| `mds-uses` | Uses テーブル（ヘッダー付き） |
| `mds-use-row` | Uses テーブルの1行 |
| `mds-section` | セクション見出し |
| `mds-code-section` | コードセクション（Uses + コードブロック） |
| `mds-code` | コードブロック |
| `mds-config` | 基本的な mds.config.toml |
| `mds-config-full` | 全セクション付き mds.config.toml |

## Neovim

### nvim-lspconfig での設定例

```lua
local lspconfig = require('lspconfig')
local configs = require('lspconfig.configs')

if not configs.mds_lsp then
  configs.mds_lsp = {
    default_config = {
      cmd = { 'mds-lsp' },
      filetypes = { 'markdown' },
      root_dir = lspconfig.util.root_pattern('mds.config.toml'),
      settings = {},
    },
  }
end

lspconfig.mds_lsp.setup({
  on_attach = function(client, bufnr)
    -- 必要に応じてキーマップを設定
  end,
})
```

### ファイルタイプの設定

```lua
vim.filetype.add({
  extension = {
    ['ts.md'] = 'markdown',
    ['py.md'] = 'markdown',
    ['rs.md'] = 'markdown',
  },
})
```

## Helix

`languages.toml` に以下を追加:

```toml
[[language]]
name = "markdown"
language-servers = ["mds-lsp"]
file-types = ["md"]

[language-server.mds-lsp]
command = "mds-lsp"
```

## その他のエディタ

mds-lsp は標準の LSP プロトコル（stdio トランスポート）に準拠しているため、LSP をサポートする任意のエディタで利用可能です。

必要な設定:

1. コマンド: `mds-lsp`
2. トランスポート: stdio
3. ルートパターン: `mds.config.toml`
4. ファイルタイプ: `*.ts.md`, `*.py.md`, `*.rs.md`, `mds.config.toml`, `package.md`

## LSP 機能一覧

### Phase 1: リアルタイム診断

ファイルを開いた時・編集時に自動で検証が実行されます。

| 検証対象 | 内容 |
| --- | --- |
| セクション構造 | Purpose, Contract, Types, Source, Cases, Test の存在チェック |
| テーブル形式 | Uses テーブルの From, Target, Expose, Summary カラム検証 |
| 言語一致 | ファイル拡張子（.ts.md 等）とコードブロック言語ラベルの一致 |
| コードブロック | import/use/require 文の混入禁止チェック |
| Markdown リンク | ローカルリンクの参照先存在チェック |
| mds.config.toml | TOML 構文、必須フィールド、サポートされるキーの検証 |
| package.md | セクション構造、Package テーブルの検証 |

### Phase 2: コードナビゲーション

| 機能 | 説明 |
| --- | --- |
| Go to Definition | Uses テーブルの Target セルから参照先の `.{lang}.md` ファイルへジャンプ |
| Find References | 現在のファイルがどの実装 Markdown の Uses で参照されているか検索 |
| Document Symbols | `##` / `###` 見出しをシンボルとしてアウトライン表示 |
| Workspace Symbols | `src-md/` 配下の全モジュール名を検索 |

### Phase 3: コードアシスト

| 機能 | 説明 |
| --- | --- |
| セクション名補完 | `## ` の後に Purpose, Contract 等の候補を表示 |
| テーブルカラム補完 | テーブル行内で From, Target 等の候補を表示 |
| コードブロック言語補完 | ` ``` ` の後にファイルから推測した言語を候補表示 |
| スニペット | ドキュメントテンプレート、Uses 行、コードブロック |
| Hover | セクション見出しの説明、Uses ターゲットの Purpose を表示 |
| Code Action | 欠損セクションの自動追加（Quick Fix） |

## トラブルシューティング

### LSP サーバーが起動しない

```bash
# mds-lsp が PATH にあるか確認
which mds-lsp

# 手動で起動テスト（Ctrl+C で終了）
mds-lsp
```

### ログの確認

VSCode の場合、Output パネルで「mds Language Server」チャンネルを選択してログを確認できます。

ログレベルを変更するには:

```json
{
  "mds.lsp.logLevel": "debug"
}
```

### 診断が表示されない

1. `mds.config.toml` が存在し、`enabled = true` になっているか確認
2. ファイルが `src-md/` ディレクトリ配下にあるか確認
3. ファイル拡張子が `.ts.md`, `.py.md`, `.rs.md` のいずれかであるか確認

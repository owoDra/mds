# LSP 開発ガイド

このページでは、mds-lsp（Language Server Protocol サーバー）の開発、デバッグ、テスト、機能追加の方法を説明します。

mds-lsp の利用方法については [エディタ統合 (LSP)](editor-integration.md) を参照してください。

## アーキテクチャ概要

mds-lsp は以下のコンポーネントで構成されています。

```
src-md/mds-lsp/
├── src/
│   ├── main.rs.md        # エントリポイント（stdio トランスポート起動）
│   ├── lib.rs.md         # ライブラリルート
│   ├── server.rs.md      # LanguageServer トレイト実装
│   ├── state.rs.md       # ワークスペース状態管理
│   ├── convert.rs.md     # 型変換ユーティリティ
│   ├── labels.rs.md      # セクション名・テーブルヘッダー定義
│   └── capabilities/
│       ├── mod.rs.md         # capability モジュールの再エクスポート
│       ├── diagnostics.rs.md # 診断（エラー・警告）生成
│       ├── completion.rs.md  # 補完候補の提供
│       ├── hover.rs.md       # ホバー情報の提供
│       ├── navigation.rs.md  # 定義ジャンプ・参照検索
│       ├── symbols.rs.md     # ドキュメント/ワークスペースシンボル
│       └── code_action.rs.md # コードアクション（Quick Fix）
└── tests/
    ├── capabilities.rs.md # capability の単体テスト
    └── diagnostics.rs.md  # 診断の統合テスト
```

### 主要な依存クレート

| クレート | 用途 |
| --- | --- |
| `mds-core` | コアロジック（Markdown 解析、モデル、設定、診断） |
| `tower-lsp` | LSP プロトコル実装フレームワーク |
| `tokio` | 非同期ランタイム |
| `tracing` | 構造化ログ |

### 状態モデル

`state.rs` に定義されたワークスペース状態がすべての capability の基盤です。

| 構造体 | 役割 |
| --- | --- |
| `WorkspaceState` | グローバル状態。ワークスペースフォルダ、開いているファイル、パッケージ一覧を保持 |
| `SharedState` | `Arc<RwLock<WorkspaceState>>` のエイリアス。全ハンドラで共有 |
| `PackageState` | パッケージごとの情報とワークスペースインデックス |
| `WorkspaceIndex` | `ImplDoc` のマップ、expose 名の逆引きインデックス |
| `OpenFile` | 開いているファイルの URI、テキスト、バージョン、言語を追跡 |

### リクエストの流れ

1. エディタがリクエストを送信（例: `textDocument/completion`）
2. `tower-lsp` がリクエストをデシリアライズし `MdsLanguageServer` のメソッドを呼び出す
3. `server.rs` のハンドラが `state` を読み取り、該当する `capabilities::*` 関数を呼び出す
4. capability 関数が `mds-core` のモデルを使って結果を生成
5. `server.rs` がレスポンスを返す

## 現在サポートしている capability

| Capability | 概要 |
| --- | --- |
| `textDocument/publishDiagnostics` | セクション欠落、見出し深さ違反、コードブロック言語不一致、設定ファイルエラー |
| `textDocument/completion` | セクション名、テーブルカラム名、コードブロック言語ラベル、スニペット |
| `textDocument/hover` | セクション見出しの説明、Uses テーブルの対象モジュール情報 |
| `textDocument/definition` | Uses テーブルのターゲットから対応ファイルへのジャンプ |
| `textDocument/references` | expose 名を Uses するすべてのファイルを列挙 |
| `textDocument/documentSymbol` | `##` / `###` 見出しをシンボルとして返す |
| `workspace/symbol` | expose 名の検索 |
| `textDocument/codeAction` | 不足セクションの追加 Quick Fix |

## ビルドとテスト

### ビルド

```bash
./scripts/sync-build.sh
cd .build/rust
cargo build -p mds-lsp
```

リリースビルド:

```bash
cargo build -p mds-lsp --release
```

### テスト実行

```bash
# mds-lsp のテストのみ
cargo test -p mds-lsp

# 全クレートのテスト
cargo test
```

### コード品質チェック

```bash
# フォーマットチェック + Clippy + テスト
cargo fmt --check && cargo clippy -- -D warnings && cargo test
```

## デバッグ

### ログレベルの設定

mds-lsp は `tracing` クレートを使用しています。`RUST_LOG` 環境変数でログレベルを制御します。

```bash
# 詳細ログで直接起動
RUST_LOG=mds_lsp=debug mds-lsp

# トレースレベル（最も詳細）
RUST_LOG=mds_lsp=trace mds-lsp
```

VSCode 拡張から起動する場合は、設定 `mds.lsp.logLevel` で制御できます。ログは「出力」パネルの「mds Language Server」チャンネルに表示されます。

### stdio での手動テスト

mds-lsp は stdio トランスポートで動作します。手動でリクエストを送信してテストできます:

```bash
# LSP サーバーを起動（stdin でリクエストを待つ）
RUST_LOG=mds_lsp=debug mds-lsp
```

以下の形式で JSON-RPC リクエストを送信します:

```
Content-Length: {length}\r\n
\r\n
{"jsonrpc":"2.0","id":1,"method":"initialize","params":{...}}
```

### VSCode 拡張のデバッグ

1. `editors/vscode` ディレクトリを VSCode で開く
2. `npm install && npm run compile` を実行
3. F5 で Extension Development Host を起動
4. Extension Development Host で `.ts.md` ファイルを開く
5. 「出力」パネル → 「mds Language Server」チャンネルでログを確認
6. ブレークポイントは拡張の TypeScript コードに設定可能

LSP サーバー（Rust 側）をデバッグする場合:

1. `mds.lsp.path` に debug ビルドのパスを設定（例: `.build/rust/target/debug/mds-lsp`）
2. VSCode 拡張を再起動
3. Rust 側のデバッグは `tracing` ログで行うか、別途 `rust-lldb`/`rust-gdb` を使用

## 新しい capability の追加手順

### 1. capability モジュールの作成

`src-md/mds-lsp/src/capabilities/` に新しい implementation md を作成します。

```rust
// capabilities/my_feature.rs
use tower_lsp::lsp_types::*;
use crate::state::WorkspaceState;

pub fn provide_my_feature(
    text: &str,
    position: Position,
    // 必要な引数...
) -> Vec<MyResult> {
    // 実装
    vec![]
}
```

### 2. モジュールの登録

`capabilities/mod.rs` にモジュールを追加:

```rust
pub mod my_feature;
```

### 3. サーバーハンドラの追加

`server.rs` で `LanguageServer` トレイトのメソッドを実装:

```rust
async fn my_feature(&self, params: MyFeatureParams) -> Result<Vec<MyResult>> {
    let state = self.state.read().await;
    // state からデータを取得し、capability 関数を呼び出す
    Ok(capabilities::my_feature::provide_my_feature(...))
}
```

### 4. ServerCapabilities の更新

`server.rs` の `initialize` メソッドで対応する capability を宣言:

```rust
Ok(InitializeResult {
    capabilities: ServerCapabilities {
        my_feature_provider: Some(...),
        ..Default::default()
    },
    ..
})
```

### 5. テストの追加

`tests/capabilities.rs` または新しいテストファイルにテストを追加:

```rust
#[test]
fn my_feature_basic_case() {
    let text = "## Purpose\n\nSample document\n";
    let position = Position { line: 0, character: 5 };
    let result = provide_my_feature(text, position);
    assert!(!result.is_empty());
}
```

## VSCode 拡張の開発

### 構造

```
editors/vscode/
├── package.json              # 拡張マニフェスト
├── src/
│   └── extension.ts          # 拡張のエントリポイント
├── syntaxes/
│   └── mds-markdown.tmLanguage.json  # TextMate grammar
├── snippets/
│   └── mds.json              # スニペット定義
└── language-configuration.json # 言語設定
```

### 言語の動的検出

`extension.ts` はワークスペースの `mds.config.toml` ファイルを読み取り、`[quality.*]` や `[adapters.*]` セクションから有効な言語を動的に検出します。これにより、新しい言語アダプターが追加されてもコード変更なしで対応できます。

検出された言語に基づいて:
- LSP クライアントの document selector が動的に構築される
- ファイル監視対象が動的に設定される
- コードブロック内の埋め込み言語サポートが有効化される

`LANGUAGE_REGISTRY` に新しい言語を追加するだけで、拡張側の対応が完了します。

### 埋め込み言語サポート

mds Markdown 内のコードブロックに対して、対応言語の IDE 機能（補完、ホバー、定義ジャンプ）を提供する仕組みがあります。

動作の流れ:

1. ドキュメントを解析してコードブロックの位置と言語を特定
2. カーソルがコードブロック内にある場合、コード内容で shadow document を作成
3. VS Code の `executeCommand` API で対応言語のプロバイダーに委譲
4. 結果をユーザーに返す

現在の制限事項:
- shadow document はファイルコンテキスト（import 等）を持たないため、型推論は限定的
- 言語サーバーが untitled ドキュメントを処理できる場合のみ機能する
- コードブロック外の定義へのジャンプはサポートされない

### TextMate grammar の更新

新しい言語のシンタックスハイライトを追加するには、`syntaxes/mds-markdown.tmLanguage.json` の `repository` に新しいコードブロックパターンを追加します:

```json
{
  "mds-code-block-newlang": {
    "begin": "^(\\\\s*```)(newlang|nl)\\\\s*$",
    "end": "^(\\\\s*```\\\\s*)$",
    "contentName": "meta.embedded.block.newlang",
    "patterns": [{ "include": "source.newlang" }]
  }
}
```

そして `package.json` の `embeddedLanguages` にマッピングを追加します。

## トラブルシューティング

### LSP サーバーが起動しない

```bash
# バイナリの確認
which mds-lsp
mds-lsp --version

# 手動で起動を試みる
RUST_LOG=mds_lsp=debug mds-lsp
```

### 診断が表示されない

1. 対象ファイルの拡張子が正しいか確認（`.ts.md`, `.py.md`, `.rs.md`）
2. ワークスペースに `mds.config.toml` があるか確認
3. VSCode の「出力」パネルで mds-lsp のログを確認
4. `mds.lsp.enabled` が `true` であるか確認

### 変更が反映されない

```bash
# Rust 側を変更した場合
cd crates
cargo build -p mds-lsp

# VSCode 拡張側を変更した場合
cd editors/vscode
npm run compile
```

VSCode で `Ctrl+Shift+P` → 「Developer: Reload Window」で拡張を再読み込みします。

### テストが失敗する

```bash
# 依存クレートの変更がないか確認
cargo test -p mds-core
cargo test -p mds-lsp

# 特定のテストのみ実行
cargo test -p mds-lsp -- test_name --nocapture
```

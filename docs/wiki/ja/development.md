# 開発ガイド

このページでは、mds 本体の開発に参加するための環境構築、ビルド、テスト、デバッグの手順を説明します。

mds を利用してプロジェクトを運用する場合は、[はじめに](getting-started.md)を参照してください。

## 前提条件

| ツール | バージョン | 用途 |
| --- | --- | --- |
| Rust | 1.86 以上 | コア処理のビルドとテスト |
| Node.js | 24 以上 | npm パッケージのビルドとテスト |
| Python | 3.13 以上 | Python パッケージのビルドとテスト |
| uv | 最新 | Python の依存管理 |
| Git | 最新 | バージョン管理 |

すべての言語環境を一度にセットアップする必要はありません。Rust のみで始めることができます。

## 環境構築

### Rust 環境（必須）

```bash
# Rust ツールチェーンのインストール
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 品質ツールの追加
rustup component add rustfmt clippy

# ビルド確認
cd crates
cargo build
```

### Node.js 環境（TypeScript 関連を扱う場合）

```bash
# Node.js 24+ をインストール（nvm 利用の例）
nvm install 24
nvm use 24

# npm パッケージの依存インストール
cd packages
npm install
```

### Python 環境（Python 関連を扱う場合）

```bash
# uv のインストール
python3 -m pip install --user uv

# Python パッケージの依存インストール
cd python/mds_cli
uv sync
```

## リポジトリ構造

```
mds/
├── crates/                  # Rust ワークスペース
│   ├── mds-core/            # コアライブラリ（解析・検証・生成・init）
│   │   ├── src/
│   │   │   ├── adapter/     # 言語アダプター
│   │   │   ├── config/      # mds.config.toml の解析
│   │   │   ├── diagnostics/ # 診断メッセージ
│   │   │   ├── generation/  # コード生成
│   │   │   ├── init/        # mds init の実装
│   │   │   ├── markdown/    # Markdown 解析
│   │   │   ├── model/       # データモデル
│   │   │   └── ...
│   │   └── tests/           # 統合テスト
│   ├── mds-cli/             # CLI エントリポイント
│   │   └── src/
│   │       ├── main.rs      # メイン関数
│   │       ├── args/        # 引数解析
│   │       └── wizard.rs    # 対話型 init ウィザード
│   └── mds-lang-rs/         # Rust 言語アダプター
├── packages/                # npm パッケージ配布
├── python/                  # Python パッケージ配布
├── docs/
│   ├── project/             # 設計正本（要件・仕様・ADR）
│   └── wiki/ja/             # 利用者向けドキュメント
├── examples/                # サンプルプロジェクト
├── result/                  # 動作確認用の出力
└── Makefile                 # 開発タスクのショートカット
```

## ビルド

### Rust ビルド

```bash
cd crates
cargo build                # デバッグビルド
cargo build --release      # リリースビルド
```

### 特定パッケージのビルド

```bash
cargo build -p mds-core    # コアのみ
cargo build -p mds-cli     # CLI のみ
```

## テスト

### 全テスト実行

```bash
cd crates
cargo test
```

### 特定のテストを実行

```bash
cargo test -p mds-core                          # mds-core のテストのみ
cargo test -p mds-core -- parser_generation      # 名前で絞り込み
cargo test -p mds-cli -- args                    # CLI 引数テストのみ
```

### テストの書き方

- ユニットテストは対象モジュール内の `#[cfg(test)]` に配置
- 統合テストは `crates/*/tests/` に配置
- E2E テストは CLI のバイナリ実行を通じて検証

## 品質検査

### フォーマット

```bash
cd crates
cargo fmt              # 自動整形
cargo fmt --check      # 差分確認のみ
```

### 静的解析

```bash
cargo clippy           # lint
cargo clippy -- -D warnings   # 警告をエラーとして扱う
```

### 一括実行（Makefile）

```bash
make check             # fmt --check + clippy + test を一括実行
make fmt               # 自動整形
make lint              # clippy のみ
make test              # テストのみ
```

## mds コマンドの動作確認

開発中のコマンドをサンプルパッケージで実行する方法です。

```bash
cd crates

# 構造検査
cargo run -p mds-cli -- check --package ../examples/minimal-ts

# 生成プレビュー
cargo run -p mds-cli -- build --package ../examples/minimal-ts --dry-run

# 生成実行
cargo run -p mds-cli -- build --package ../examples/minimal-ts

# 対話型初期化
cargo run -p mds-cli -- init --package /tmp/test-project

# 環境診断
cargo run -p mds-cli -- doctor --package ../examples/minimal-ts
```

## デバッグ

### ログ出力

`--verbose` フラグで詳細な出力を得られます。

```bash
cargo run -p mds-cli -- check --package ../examples/minimal-ts --verbose
```

### デバッガの利用

VSCode の場合、`.vscode/tasks.json` に定義されたタスクを利用できます。`F5` で直接デバッグ実行することもできます。

`launch.json` の例:

```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "type": "lldb",
      "request": "launch",
      "name": "Debug mds check",
      "cargo": {
        "args": ["build", "-p", "mds-cli"],
        "filter": { "kind": "bin", "name": "mds" }
      },
      "args": ["check", "--package", "../examples/minimal-ts", "--verbose"]
    }
  ]
}
```

### テストのデバッグ

特定のテストを詳細出力付きで実行:

```bash
cargo test -p mds-core -- --nocapture test_name
```

## コード変更時のチェックリスト

1. `cargo fmt` でフォーマット
2. `cargo clippy` で警告がないことを確認
3. `cargo test` で全テストがパスすることを確認
4. 新機能の場合はテストを追加
5. ドキュメントの更新が必要な場合は合わせて更新
6. サンプルプロジェクトで動作確認

## 関連ドキュメント

- [CONTRIBUTING.md](../../../CONTRIBUTING.md) — 貢献方法の全体説明
- [アーキテクチャ](../../project/architecture.md) — 設計方針と不変条件
- [用語集](../../project/glossary/core.md) — プロジェクト共通の用語定義
- [技術スタック](../../project/tech-stack.md) — 採用技術とバージョン方針
- [コントリビューション](contributing.md) — 報告と提案の方針

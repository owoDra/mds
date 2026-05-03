# 開発ガイド

このページでは、mds 本体の開発に参加するための環境構築、ビルド、テスト、デバッグの手順を説明します。

mds を利用してプロジェクトを運用する場合は、[はじめに](getting-started.md)を参照してください。`curl -fsSL https://raw.githubusercontent.com/owo-x-project/owox-mds/main/install.sh | sh` でインストールできます。

以下はリポジトリを clone して開発する場合の手順です。

## 前提条件

| ツール | バージョン | 用途 |
| --- | --- | --- |
| Rust | 1.86 以上 | コア処理のビルドとテスト |
| Git | 最新 | バージョン管理 |

## 環境構築

### Rust 環境（必須）

```bash
# Rust ツールチェーンのインストール
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 品質ツールの追加
rustup component add rustfmt clippy

# 開発用に mds をインストール
cargo run -p mds-cli -- build --verbose
./.github/script/sync-self-hosted-rust.sh
cargo install --path .build/rust/mds-cli

# 確認
mds --version
```

## リポジトリ構造

```
mds/
├── src-md/                  # mds 自身の Markdown 正本
│   ├── index.md             # source root の設計
│   ├── mds/core/            # コアライブラリ正本
│   ├── mds/cli/             # CLI 正本
│   └── mds/lsp/             # LSP 正本
├── .build/                  # 生成物（Git 管理しない）
│   └── rust/                # 生成された Cargo ワークスペース
├── editors/vscode/          # VS Code 拡張機能
├── docs/
│   ├── project/             # 設計正本（要件・仕様・ADR）
│   └── wiki/ja/             # 利用者向けドキュメント
├── examples/                # サンプルプロジェクト
└── .vscode/tasks.json       # 開発タスク定義
```

## ビルド

### Rust ビルド

```bash
cargo run -p mds-cli -- build --verbose
./.github/script/sync-self-hosted-rust.sh
cd .build/rust
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
cargo run -p mds-cli -- build --verbose
./.github/script/sync-self-hosted-rust.sh
cd .build/rust
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
- 統合テストは `src-md/*/tests/*.rs.md` に配置し、`.build/rust/*/tests/` に同期して実行
- E2E テストは CLI のバイナリ実行を通じて検証

## 品質検査

### フォーマット

```bash
cargo run -p mds-cli -- build --verbose
./.github/script/sync-self-hosted-rust.sh
cd .build/rust
cargo fmt              # 自動整形
cargo fmt --check      # 差分確認のみ
```

### 静的解析

```bash
cargo clippy           # lint
cargo clippy -- -D warnings   # 警告をエラーとして扱う
```

### 一括実行

```bash
cargo run -p mds-cli -- build --verbose
./.github/script/sync-self-hosted-rust.sh
cd .build/rust
cargo fmt --check && cargo clippy -- -D warnings && cargo test
```

VSCode ではタスク「mds: Check All」で同じ検査を実行できます。

## mds コマンドの動作確認

開発中のコマンドをサンプルパッケージで実行する方法です。

```bash
cargo run -p mds-cli -- build --verbose
./.github/script/sync-self-hosted-rust.sh
cd .build/rust

# 構造検査
cargo run -p mds-cli -- check --package ../../examples/minimal-ts

# 生成プレビュー
cargo run -p mds-cli -- build --package ../../examples/minimal-ts --dry-run

# 生成実行
cargo run -p mds-cli -- build --package ../../examples/minimal-ts

# 対話型初期化
cargo run -p mds-cli -- init --package /tmp/test-project

# 環境診断
cargo run -p mds-cli -- doctor --package ../../examples/minimal-ts
```

## デバッグ

### ログ出力

`--verbose` フラグで詳細な出力を得られます。

```bash
cargo run -p mds-cli -- check --package ../../examples/minimal-ts --verbose
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
      "args": ["check", "--package", "../../examples/minimal-ts", "--verbose"]
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

1. `cargo run -p mds-cli -- build --verbose` で package 内の生成 `src/` / `tests/` を更新
2. `./.github/script/sync-self-hosted-rust.sh` でこの repo 用の `.build/rust/` を再生成
3. `.build/rust` で `cargo fmt` を実行
4. `.build/rust` で `cargo clippy` の警告がないことを確認
5. `.build/rust` で `cargo test` がパスすることを確認
6. 新機能の場合はテストを追加
7. ドキュメントの更新が必要な場合は合わせて更新
8. サンプルプロジェクトで動作確認

## 関連ドキュメント

- [CONTRIBUTING.md](../../../CONTRIBUTING.md) — 貢献方法の全体説明
- [アーキテクチャ](../../project/architecture.md) — 設計方針と不変条件
- [用語集](../../project/glossary/core.md) — プロジェクト共通の用語定義
- [技術スタック](../../project/tech-stack.md) — 採用技術とバージョン方針
- [コントリビューション](contributing.md) — 報告と提案の方針

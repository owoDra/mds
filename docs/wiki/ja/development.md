# 開発ガイド

このページでは、mds 本体の開発に参加するための環境構築、ビルド、テスト、デバッグの手順を説明します。

mds を利用してプロジェクトを運用する場合は、[はじめに](getting-started.md)を参照してください。GitHub Releases installer の `curl -fsSL https://raw.githubusercontent.com/owo-x-project/owox-mds/latest/install.sh | sh` でインストールできます。

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

# checked-in workspace をビルド
cargo build --workspace

# 任意: checked-in source tree から CLI をインストール
cargo install --path mds/cli

# 確認
cargo run -p mds-cli -- --version
```

## リポジトリ構造

```
owox-mds/
├── mds/
│   ├── core/
│   │   ├── src/             # mds-core の checked-in Rust source
│   │   └── tests/           # mds-core の checked-in tests
│   ├── cli/
│   │   ├── src/             # mds CLI の checked-in Rust source
│   │   └── tests/           # mds CLI の checked-in tests
│   └── lsp/
│       ├── src/             # mds-lsp の checked-in Rust source
│       └── tests/           # mds-lsp の checked-in tests
├── editors/vscode/
│   ├── src/                 # VSCode 拡張の source
│   └── package.json         # 拡張マニフェストと scripts
├── docs/
│   ├── project/             # 設計正本（要件・仕様・ADR）
│   └── wiki/                # 利用者向けドキュメント
├── examples/                # サンプルプロジェクト
└── target/                  # Cargo build artifacts
```

## ビルド

### first-party Rust workspace のビルド

```bash
cargo build --workspace
```

### 特定パッケージのビルド

```bash
cargo build -p mds-core
cargo build -p mds-cli
cargo build -p mds-lsp
```

### VSCode 拡張のビルド

```bash
cd editors/vscode
npm install
npm run compile
```

この repository の first-party package では、checked-in Rust / TypeScript source tree を直接編集します。`mds` command は sample package の smoke test や product behavior の確認に限って使ってください。

## テスト

### 全テスト実行

```bash
cargo test --workspace
```

### 特定のテストを実行

```bash
cargo test -p mds-core                           # mds-core のテストのみ
cargo test -p mds-cli                            # mds-cli のテストのみ
cargo test -p mds-lsp                            # mds-lsp のテストのみ
```

### テストの書き方

- ユニットテストは対象モジュール内の `#[cfg(test)]` に配置
- 統合テストは `mds/*/tests/*.rs` に配置
- E2E テストは CLI のバイナリ実行や sample package を通じて検証

## 品質検査

### フォーマット

```bash
cargo fmt --all --check
```

### 静的解析

```bash
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

### 一括実行

```bash
cargo fmt --all --check
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

`editors/vscode` を変更した場合は、追加で `cd editors/vscode && npm run compile` を実行してください。

## mds コマンドの動作確認

開発中のコマンドをサンプルパッケージで実行する方法です。これらは product behavior の確認用であり、この repository の first-party source を再生成するための手順ではありません。

```bash
# 構造検査
cargo run -p mds-cli -- check --package examples/minimal-ts

# 生成プレビュー
cargo run -p mds-cli -- build --package examples/minimal-ts --dry-run

# 生成実行
cargo run -p mds-cli -- build --package examples/minimal-ts

# 対話型初期化
cargo run -p mds-cli -- init --package /tmp/test-project

# 環境診断
cargo run -p mds-cli -- doctor --package examples/minimal-ts
```

## デバッグ

### ログ出力

`--verbose` フラグで詳細な出力を得られます。

```bash
cargo run -p mds-cli -- check --package examples/minimal-ts --verbose
```

### デバッガの利用

VSCode の場合、`.vscode/tasks.json` に定義されたタスクを利用できます。`F5` で直接デバッグ実行することもできます。

`launch.json` の例:

```json
{
  "configurations": [
    {
      "type": "lldb",
      "request": "launch",
      "name": "Debug mds lint",
      "cargo": {
        "args": ["build", "-p", "mds-cli"],
        "filter": { "kind": "bin", "name": "mds" }
      },
      "args": ["check", "--package", "examples/minimal-ts", "--verbose"]
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

1. `mds/` と `editors/vscode/` 配下の checked-in Rust / TypeScript source と test を直接編集
2. `cargo fmt --all --check` を実行
3. `cargo check --workspace` を実行
4. `cargo test --workspace` を実行
5. `cargo clippy --workspace --all-targets -- -D warnings` を実行
6. `editors/vscode` を変更した場合は `cd editors/vscode && npm run compile` を実行
7. 新機能の場合はテストを追加
8. ドキュメントの更新が必要な場合は合わせて更新
9. `cargo run -p mds-cli -- ...` でサンプルプロジェクトの動作確認

## 関連ドキュメント

- [CONTRIBUTING.md](../../../CONTRIBUTING.md) — 貢献方法の全体説明
- [アーキテクチャ](../../project/architecture.md) — 設計方針と不変条件
- [用語集](../../project/glossary/core.md) — プロジェクト共通の用語定義
- [技術スタック](../../project/tech-stack.md) — 採用技術とバージョン方針
- [コントリビューション](contributing.md) — 報告と提案の方針

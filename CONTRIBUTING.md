# Contributing to mds

mds への貢献に関心を持っていただきありがとうございます。このドキュメントでは、開発参加の手順と方針を説明します。

## クイックスタート

```bash
# 1. Rust のビルドと確認
cd crates
cargo build
cargo test

# 2. mds コマンドの実行確認
cargo run -p mds-cli -- check --package ../../examples/minimal-ts
```

詳しい開発環境の構築手順は [開発ガイド](docs/wiki/ja/development.md) を参照してください。

## 歓迎する貢献

- 不具合報告と再現手順
- ドキュメント改善
- 仕様の分かりにくい部分の指摘
- エラーメッセージの改善提案
- 利用例の追加
- 新しい言語や配布方法の要望
- テストケースの追加
- パフォーマンス改善

## 開発の流れ

### 1. Issue を確認する

作業を始める前に、関連する Issue がないか確認してください。新しい機能や変更の場合は、Issue を作成して方針を議論してから実装を始めると効率的です。

### 2. ブランチを作成する

```bash
git checkout -b feature/your-feature-name
```

### 3. 変更を実装する

- コーディング規約は既存コードのスタイルに合わせてください
- 新しい機能にはテストを追加してください
- ドキュメントの更新が必要な場合は合わせて更新してください

### 4. テストを実行する

```bash
cd crates
cargo test
cargo clippy
cargo fmt --check
```

一括実行する場合は Makefile も利用できます:

```bash
make check   # fmt + clippy + test を一括実行
```

### 5. Pull Request を作成する

- 変更の目的を明確に書いてください
- 関連する Issue があればリンクしてください
- テストが通ることを確認してください

## 不具合報告

不具合を報告する場合は、次の情報を含めてください:

| 項目 | 内容 |
| --- | --- |
| 実行したコマンド | `mds check --package ...` など |
| 期待した結果 | 正常終了、特定の出力など |
| 実際の結果 | エラーメッセージ、終了コードなど |
| 環境 | Rust、Node.js、Python のバージョン |
| 設定ファイル | `mds.config.toml` の内容 |
| 再現に必要な Markdown | 最小限の実装 Markdown |

## プロジェクト構造

```
crates/
  mds-core/     # コア処理ライブラリ（解析、検証、生成、init）
  mds-cli/      # CLI エントリポイントと引数解析
  mds-lang-rs/  # Rust 言語アダプター
packages/        # npm パッケージ配布用
python/          # Python パッケージ配布用
docs/
  project/       # 設計正本（要件、仕様、ADR、アーキテクチャ）
  wiki/ja/       # 利用者向けドキュメント
examples/        # サンプルプロジェクト
```

## ドキュメントの書き方

- 読み手が前提を知らなくても理解できるように書いてください
- 略語や専門用語は最初に意味を説明してください
- 利用者向けのドキュメントは `docs/wiki/ja/` に配置してください
- 設計や仕様のドキュメントは `docs/project/` に配置してください

## 関連ドキュメント

- [開発ガイド](docs/wiki/ja/development.md) — 環境構築、ビルド、テスト、デバッグの詳細
- [アーキテクチャ](docs/project/architecture.md) — 設計方針と不変条件
- [用語集](docs/project/glossary/core.md) — プロジェクト共通の用語
- [技術スタック](docs/project/tech-stack.md) — 採用技術とバージョン方針

## ライセンス

MIT License です。貢献されたコードは同じライセンスの下で配布されます。

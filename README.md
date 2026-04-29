# mds

mds は、Markdown を設計と実装の正本として扱う開発ツールチェーンです。

Markdown 文書の中に TypeScript、Python、Rust などの実際のコードをコードブロックとして記述し、`mds build` でそれらを実行可能なソースファイルとして取り出します。Markdown 内のコードがそのまま動くコードになるため、設計の意図と実装が常に一致します。

## 何ができるか

- Markdown 内の `Types`、`Source`、`Test` コードブロックに書いた実コードから `.ts`、`.py`、`.rs` ファイルを生成する
- `mds check` で Markdown の構造と整合性を検査する
- `mds lint` / `mds test` で Markdown 内のコードに対して検査・テストを実行する
- `mds init` で対話型ウィザードによるプロジェクト初期化を行う

## クイックスタート

```bash
# ビルド
cd crates && cargo build -p mds-cli

# 対話型で初期化（推奨）
cargo run -p mds-cli -- init --package ../path/to/package

# 検査 → プレビュー → 生成
cargo run -p mds-cli -- check --package ../path/to/package
cargo run -p mds-cli -- build --package ../path/to/package --dry-run
cargo run -p mds-cli -- build --package ../path/to/package
```

動作する最小構成は [examples/](examples/) を参照してください。

## 動作環境

- Rust 1.86 以上（必須）
- Node.js 24 以上（TypeScript を扱う場合）
- Python 3.13 以上（Python を扱う場合）

## ドキュメント

| 対象 | 入口 |
| --- | --- |
| **mds を使う人** | [wiki 入口](docs/wiki/ja/index.md) — はじめに、コマンド、設定、生成、トラブルシューティング |
| **mds の開発に参加する人** | [CONTRIBUTING.md](CONTRIBUTING.md) — 環境構築、開発フロー、テスト |

### 主なリンク

- [はじめに](docs/wiki/ja/getting-started.md) — 前提と最小構成
- [コマンド](docs/wiki/ja/commands.md) — 全コマンドの使い方
- [開発ガイド](docs/wiki/ja/development.md) — ビルド、テスト、デバッグ
- [AI エージェント連携](docs/wiki/ja/ai-agent-integration.md) — Claude Code、Codex、Opencode、GitHub Copilot
- [エディタ統合 (LSP)](docs/wiki/ja/editor-integration.md) — VSCode 拡張、Neovim、リアルタイム診断

## コントリビューション

不具合報告、ドキュメント改善、実装改善を歓迎します。詳しくは [CONTRIBUTING.md](CONTRIBUTING.md) を参照してください。

## ライセンス

MIT License です。詳しくは [LICENSE](LICENSE) を参照してください。

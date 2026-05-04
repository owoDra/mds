<p align="center">
	<img src=".github/assets/readme-header.png" alt="markdown source" width="720">
</p>

# mds

mds は、Markdown を設計と実装の正本として扱う開発ツールチェーンです。

Markdown 文書の中に TypeScript、Python、Rust などの実際のコードをコードブロックとして記述し、`mds build` でそれらを実行可能なソースファイルとして取り出します。Markdown 内のコードがそのまま動くコードになるため、設計の意図と実装が常に一致します。

## 何ができるか

- Markdown 内の `Types`、`Source`、`Test` コードブロックに書いた実コードから `.ts`、`.py`、`.rs` ファイルを生成する
- `mds lint` で Markdown の構造検査と code block lint を実行する
- `mds typecheck` / `mds test` で Markdown 内のコードに対して型検査・テストを実行する
- `mds init` で対話型ウィザードによるプロジェクト初期化を行う

## クイックスタート

```bash
# インストール
curl -fsSL https://raw.githubusercontent.com/owo-x-project/owox-mds/main/install.sh | sh

# 基本的な使い方
mds init --package ./path/to/package
mds lint --package ./path/to/package
mds typecheck --package ./path/to/package
mds build --package ./path/to/package
```

VSCode 拡張: `code --install-extension owo-x-project.mds`

動作する最小構成は [examples/](examples/) を参照してください。

## 動作環境

ランタイム依存なし — mds は単一の静的バイナリです。

- Rust 1.86 以上（ソースからビルドする場合のみ必要）

## ドキュメント

| 対象 | 入口 |
| --- | --- |
| **mds を使う人** | [wiki 入口](docs/wiki/ja/index.md) — はじめに、コマンド、設定、生成、トラブルシューティング |
| **mds の開発に参加する人** | [CONTRIBUTING.md](CONTRIBUTING.md) — 環境構築、開発フロー、テスト |

### 主なリンク

- [はじめに](docs/wiki/ja/getting-started.md) — 前提と最小構成
- [コマンド](docs/wiki/ja/commands.md) — 全コマンドの使い方
- [descriptor ガイド](docs/wiki/ja/descriptors.md) — 言語・quality tool・package manager TOML の完全ガイド
- [開発ガイド](docs/wiki/ja/development.md) — ビルド、テスト、デバッグ
- [AI エージェント連携](docs/wiki/ja/ai-agent-integration.md) — Claude Code、Codex、Opencode、GitHub Copilot
- [エディタ統合 (LSP)](docs/wiki/ja/editor-integration.md) — VSCode 拡張、Neovim、リアルタイム診断

## コントリビューション

不具合報告、ドキュメント改善、実装改善を歓迎します。詳しくは [CONTRIBUTING.md](CONTRIBUTING.md) を参照してください。

## ライセンス

MIT License です。詳しくは [LICENSE](LICENSE) を参照してください。

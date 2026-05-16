<p align="center">
	<img src=".github/assets/readme-header.png" alt="markdown source" width="720">
</p>

# mds

mds は、Markdown をパッケージ設計、実装コード、実行可能な検証の正本として扱う開発ツールチェーンです。

`.mds/source` と `.mds/test` に tableless な文書を書き、公開面は prose で説明し、実行するコードは `Source` または `Test` の code fence に置き、`mds build` で package 出力へ展開します。`mds-lsp` は generated file から得た diagnostics、hover、definition を元の Markdown へ差し戻せます。

## 何ができるか

- source doc は `.mds/source`、verification doc は `.mds/test` に固定する current authoring-v2
- source doc は `Purpose`、`Contract`、`API`、`Source`、`Cases`、test doc は `Purpose`、`Covers`、`Cases`、`Test` を使う tableless authoring
- `[roots]`、`[output]`、`[[output.override]]` による package output planning
- `mds init`、`mds new`、`mds init --ai` による current project/doc/agent kit の scaffold
- `mds-lsp` による diagnostics、snippets、generated-file bridge navigation

## クイックスタート

```bash
# 最新の GitHub Releases バイナリをインストール
curl -fsSL https://raw.githubusercontent.com/owo-x-project/owox-mds/latest/install.sh | sh

# package を初期化または検査
mds init --package ./path/to/package
mds lint --package ./path/to/package
mds build --package ./path/to/package --dry-run
mds build --package ./path/to/package
```

VS Code 拡張: `code --install-extension owo-x-project.mds`

インストーラーは platform 別の GitHub Releases archive を取得し、`mds` と `mds-lsp` の両方をインストールします。Marketplace 版の VS Code 拡張には対応する `mds-lsp` バイナリが同梱されるため、VS Code 利用時に LSP を別途入れる必要はありません。

## 最小パッケージ構成

```text
my-package/
├── mds.config.toml
├── package.md
├── package.json
├── .mds/
│   ├── source/
│   │   ├── overview.md
│   │   └── greet.ts.md
│   └── test/
│       ├── overview.md
│       └── greet.ts.md
├── src/
└── tests/
```

動作する最小構成は [examples/](examples/) を参照してください。

## 動作環境

ビルド済み `mds` CLI バイナリ自体にはランタイム依存はありません。言語ごとの検査では、設定した Node.js、Python、Rust などの toolchain を利用します。

## ドキュメント

| 対象 | 入口 |
| --- | --- |
| **mds を使う人** | [wiki 入口](docs/wiki/ja/index.md) - はじめに、設定、生成、エディタ連携、トラブルシューティング |
| **mds の開発に参加する人** | [CONTRIBUTING.md](CONTRIBUTING.md) - 環境構築、開発フロー、テスト |

### 主なリンク

- [はじめに](docs/wiki/ja/getting-started.md) - mds の導入と package 準備
- [設定](docs/wiki/ja/configuration.md) - canonical roots、output patterns、checks
- [Markdown 正本](docs/wiki/ja/markdown-source.md) - current source/test 文書モデル
- [生成の仕組み](docs/wiki/ja/generation.md) - output planning、overrides、manifest
- [コマンド](docs/wiki/ja/commands.md) - CLI リファレンス
- [AI エージェント連携](docs/wiki/ja/ai-agent-integration.md) - agent kit 生成と template 管理
- [エディタ統合 (LSP)](docs/wiki/ja/editor-integration.md) - VS Code 拡張、他 editor、generated-file bridge
- [開発ガイド](docs/wiki/ja/development.md) - repository の build、test、debug

## コントリビューション

不具合報告、ドキュメント改善、実装改善を歓迎します。first-party の repository 開発では `mds/` と `editors/vscode/` 配下の checked-in source / test を直接編集し、必要に応じて Cargo や npm で検証します。詳しくは [CONTRIBUTING.md](CONTRIBUTING.md) を参照してください。

## ライセンス

MIT License です。詳しくは [LICENSE](LICENSE) を参照してください。
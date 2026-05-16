# mds 日本語 wiki

この wiki は、mds の current authoring-v2 surface を説明する live docs です。

mds は canonical な source / verification roots、tableless な Markdown 文書、package output planning、generated file から Markdown へ差し戻す editor tooling を前提にしています。旧 metadata table workflow は、この wiki の live model ではありません。

## まず読むページ

1. [はじめに](getting-started.md) - mds の導入と package 準備
2. [設定](configuration.md) - canonical roots、output patterns、checks
3. [Markdown 正本](markdown-source.md) - current source/test 文書の形
4. [コマンド](commands.md) - CLI と基本フロー
5. [生成の仕組み](generation.md) - 出力計画と書き込みの流れ

## 中心トピック

- [基本概念](concepts.md) - 正本、logical module、generated-file bridge
- [品質検査](quality.md) - 構造診断、tool 実行、check policy
- [トラブルシューティング](troubleshooting.md) - よくある authoring-v2 の失敗と確認手順
- [エディタ統合 (LSP)](editor-integration.md) - VS Code 拡張と他 editor
- [AI エージェント連携](ai-agent-integration.md) - agent kit 生成と template 管理

## 補助ガイド

- [モノレポでの使い方](monorepo.md) - 大きな repository での package 単位導入
- [パッケージ情報同期](package-sync.md) - `mds package sync` の managed metadata 更新
- [配布方針](distribution.md) - CLI、installer、editor package の配布
- [コントリビューション](contributing.md) - 報告と提案の進め方
- [開発ガイド](development.md) - repository の build、test、debug
- [LSP 開発ガイド](lsp-development.md) - editor stack の内部メモ
- [ロードマップ](roadmap.md) - 現在の focus と follow-up

## 現在の live model

- source doc は `.mds/source`、verification doc は `.mds/test` に置く
- source doc は `Purpose`、`Contract`、`API`、`Source`、`Cases` を使う
- test doc は `Purpose`、`Covers`、`Cases`、`Test` を使う
- 出力 path は `[roots]`、`[output]`、`[[output.override]]` で決まる
- `mds-lsp` は generated file 由来の hover、definition、diagnostics を Markdown へ戻せる

## ページ一覧

| ページ | 内容 |
| --- | --- |
| [はじめに](getting-started.md) | install、最小 package 構成、最初の command |
| [設定](configuration.md) | `mds.config.toml`、canonical roots、output patterns、checks |
| [Markdown 正本](markdown-source.md) | source/test docs、overview docs、root module docs |
| [コマンド](commands.md) | `init`、`new`、`build`、`lint`、`typecheck`、`test`、`doctor`、`package sync` |
| [生成の仕組み](generation.md) | logical module、default outputs、overrides、manifest |
| [基本概念](concepts.md) | 正本、output planning、package boundary |
| [品質検査](quality.md) | 構造診断、selected tools、check policy |
| [トラブルシューティング](troubleshooting.md) | よくある failure と確認手順 |
| [エディタ統合 (LSP)](editor-integration.md) | bundled VS Code 拡張と他 editor 設定 |
| [AI エージェント連携](ai-agent-integration.md) | agent kit 生成と template 管理 |
| [モノレポでの使い方](monorepo.md) | package 単位の有効化と安全な出力境界 |
| [パッケージ情報同期](package-sync.md) | managed package metadata の同期 |
| [配布方針](distribution.md) | release binaries、installer、editor packages |
| [コントリビューション](contributing.md) | contribution policy |
| [開発ガイド](development.md) | repository の build、test、debug |
| [LSP 開発ガイド](lsp-development.md) | editor stack の内部メモ |
| [ロードマップ](roadmap.md) | 現在の focus area |
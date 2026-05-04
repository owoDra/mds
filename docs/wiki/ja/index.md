# mds 日本語 wiki

この wiki は、mds を利用する人、導入を検討する人、開発に参加する人に向けた日本語の説明です。

mds は、Markdown を設計、実装、テストの正本として扱い、Markdown 内のコードブロックから言語ごとの派生コードを生成する開発ツールチェーンです。

## あなたの目的は？

### mds を試してみたい

1. [はじめに](getting-started.md) — 前提条件と最小構成
2. [サンプルプロジェクト](../../../examples/) — 動作する最小構成の例
3. [コマンド](commands.md) — 基本的な使い方

### mds の考え方を理解したい

1. [基本概念](concepts.md) — 正本・派生コード・公開面
2. [Markdown 正本](markdown-source.md) — Markdown 文書の種類と役割
3. [生成の仕組み](generation.md) — コード生成の規則

### 既存プロジェクトに mds を導入したい

1. [はじめに](getting-started.md) — 最小構成の確認
2. [設定](configuration.md) — mds.config.toml の詳細
3. [モノレポでの使い方](monorepo.md) — 複数パッケージの管理

### AI エージェントと連携したい

1. [AI エージェント連携](ai-agent-integration.md) — 対応 CLI と設定生成

### エディタで mds を使いたい

1. [エディタ統合 (LSP)](editor-integration.md) — VSCode 拡張、Neovim、リアルタイム診断

### 問題を解決したい

1. [トラブルシューティング](troubleshooting.md) — よくある問題と解決策
2. [品質検査](quality.md) — 検査と診断の実行方法

### mds の開発に参加したい

1. [コントリビューション](contributing.md) — 報告と提案の方針
2. [開発ガイド](development.md) — 環境構築、ビルド、テスト、デバッグ
3. [LSP 開発ガイド](lsp-development.md) — mds-lsp の開発、デバッグ、機能追加
4. [descriptor ガイド](descriptors.md) — 言語、quality tool、package manager descriptor

## 読む順序

初めて読む場合は、次の順序がおすすめです。

1. [はじめに](getting-started.md)
2. [基本概念](concepts.md)
3. [Markdown 正本](markdown-source.md)
4. [コマンド](commands.md)
5. [設定](configuration.md)
6. [生成の仕組み](generation.md)

## ページ一覧

| ページ | 内容 |
| --- | --- |
| [はじめに](getting-started.md) | インストール前の前提、最小構成、基本的な実行手順を説明します。 |
| [基本概念](concepts.md) | 正本、派生コード、実装 Markdown、公開面、依存関係などの用語を説明します。 |
| [Markdown 正本](markdown-source.md) | mds が扱う Markdown 文書の種類と役割を説明します。 |
| [コマンド](commands.md) | mds の各コマンドの目的と使い方を説明します。 |
| [設定](configuration.md) | `mds.config.toml` の役割と主要な設定を説明します。 |
| [モノレポでの使い方](monorepo.md) | パッケージ単位の対象判定と複数言語の扱いを説明します。 |
| [生成の仕組み](generation.md) | `Types`、`Source`、`Test` から派生コードが作られる規則を説明します。 |
| [言語アダプター](language-adapters.md) | TypeScript、Python、Rust ごとの差分をどこで扱うかを説明します。 |
| [品質検査](quality.md) | 構造検査、静的検査、自動修正、テスト、環境診断を説明します。 |
| [パッケージ情報同期](package-sync.md) | パッケージ情報から `package.md` を同期する仕組みを説明します。 |
| [配布方針](distribution.md) | Cargo、npm、Python パッケージ、ネイティブ実行ファイルでの配布方針を説明します。 |
| [トラブルシューティング](troubleshooting.md) | よくある問題と確認方法を説明します。 |
| [AI エージェント連携](ai-agent-integration.md) | AI コーディングエージェント向け設定の生成と拡張方法を説明します。 |
| [エディタ統合 (LSP)](editor-integration.md) | LSP サーバーによるリアルタイム診断、ナビゲーション、補完、VSCode 拡張を説明します。 |
| [コントリビューション](contributing.md) | 開発参加時に確認することを説明します。 |
| [開発ガイド](development.md) | 開発環境の構築、ビルド、テスト、デバッグの手順を説明します。 |
| [descriptor ガイド](descriptors.md) | built-in と workspace descriptor の配置、役割、追加方法を説明します。 |
| [LSP 開発ガイド](lsp-development.md) | mds-lsp の開発、デバッグ、capability 追加の手順を説明します。 |
| [ロードマップ](roadmap.md) | 現在の中心範囲と今後の予定を説明します。 |

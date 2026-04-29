# mds 日本語 wiki

この wiki は、mds を利用する人、導入を検討する人、開発に参加する人に向けた日本語の説明です。

mds は、Markdown を設計、実装、テストの正本として扱い、Markdown 内のコードブロックから言語ごとの派生コードを生成する開発ツールチェーンです。

## 読む順序

初めて読む場合は、次の順序がおすすめです。

1. [はじめに](getting-started.md)
2. [基本概念](concepts.md)
3. [Markdown 正本](markdown-source.md)
4. [コマンド](commands.md)
5. [設定](configuration.md)
6. [生成の仕組み](generation.md)

## 目的別の入口

mds を試したい場合は、[はじめに](getting-started.md)を読んでください。

mds の考え方を理解したい場合は、[基本概念](concepts.md)と[Markdown 正本](markdown-source.md)を読んでください。

コマンドの使い分けを確認したい場合は、[コマンド](commands.md)を読んでください。

複数のパッケージを含むリポジトリで使いたい場合は、[モノレポでの使い方](monorepo.md)を読んでください。

生成されるファイルの規則を知りたい場合は、[生成の仕組み](generation.md)を読んでください。

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
| [コントリビューション](contributing.md) | 開発参加時に確認することを説明します。 |
| [ロードマップ](roadmap.md) | 現在の中心範囲と今後の予定を説明します。 |

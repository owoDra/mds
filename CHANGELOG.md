# Changelog

このファイルは mds の主要な変更を記録します。

フォーマットは [Keep a Changelog](https://keepachangelog.com/) に基づきます。

## [Unreleased]

### Added

- 対話型 `mds init` ウィザードモード（`dialoguer` ベース）
- `mds new <name.lang.md>` コマンド — 実装 Markdown のスキャフォールド生成
- CONTRIBUTING.md — コントリビューター向けの入口ドキュメント
- 開発ガイド（docs/wiki/ja/development.md）— ビルド、テスト、デバッグ手順
- .vscode/tasks.json — VSCode タスク定義
- サンプルプロジェクト（examples/minimal-ts, minimal-py, minimal-rs）
- .editorconfig — エディタ横断の統一設定
- .githooks/pre-commit — コミット前の自動品質チェック
- GitHub Issue / PR テンプレート

### Changed

- README.md を入口特化のスリム構成に刷新
- wiki index.md にユースケース別ナビゲーション追加
- CLI usage 表示を構造化（コマンド一覧 + オプション + 例）
- エラーメッセージにコンテキスト付きヒントを追加

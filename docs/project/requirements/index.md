# Requirements

## 役割

このディレクトリは、何を実現するかを管理する正本です。

## 置いてよいもの

- 目標
- 根拠
- 成功指標
- 対象範囲と対象外
- 制約や品質条件

## 置いてはいけないもの

- 実装手段の詳細
- テスト手順の詳細
- 一時的な作業メモ

## 命名規則

- `REQ-<category>-<short-title>.md`

## 参照ルール

- 要求変更時は spec / validation / ADR への影響を確認する

## 参照

- `REQ-core-markdown-source-of-truth.md`: Markdown を正本、生成コードを派生物として扱う要求
- `REQ-platform-multi-ecosystem-distribution.md`: npm / Cargo / uv を横断する配布要求
- `REQ-adapter-required-language-adapters.md`: TypeScript / Python / Rust の必須 language adapter 要求
- `REQ-config-toml-fixed-config.md`: `mds.config.toml` 固定と設定継承の要求
- `REQ-monorepo-package-boundary.md`: monorepo での package 単位の mds 対象判定要求
- `REQ-doc-model-markdown-document-types.md`: `index.md`、`package.md`、implementation md の文書種別要求
- `REQ-implementation-one-md-one-feature.md`: 1 implementation md が 1 機能を扱う要求
- `REQ-metadata-expose-uses.md`: `Expose` と `Uses` による公開面と依存の明示要求
- `REQ-generation-code-output-rules.md`: Source / Types / Test の生成コード出力要求
- `REQ-quality-md-state-validation.md`: Markdown 状態での check / lint / lint --fix / test 要求
- `REQ-cli-command-surface.md`: CLI コマンド面の要求
- `REQ-ux-obsidian-readable-markdown.md`: Obsidian で読める Markdown の要求
- `REQ-ai-agent-cli-initialization.md`: AI agent CLI 向け instruction / skill / workflow 初期化要求
- `REQ-init-development-environment-setup.md`: `mds init` による project 初期化と開発環境セットアップ要求
- `REQ-release-prepublish-quality.md`: 全配布経路の公開前品質要求

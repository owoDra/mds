# Shared Specs

> Deprecated: shared behavioral detail は段階的に `mds/core/.mds/source/overview.md`、`mds/cli/.mds/source/overview.md`、および各 implementation md へ移行する。

## 役割

このディレクトリは、複数 subproject にまたがる旧 spec を一時保持する migration surface です。

## 置いてよいもの

- 共通 API 契約
- 共通状態ルール
- 共通認可や横断要件に関する詳細仕様

## 置いてはいけないもの

- 1 つの subproject に閉じる仕様
- 設計草案や調査メモ

## 命名規則

- `SPEC-<category>-<short-title>.md`

## 参照ルール

- subproject 固有仕様が必要な場合は、対象の `<subproject>/index.md` と個票を追加する

## 参照

- `SPEC-config-toml-resolution.md`: `mds.config.toml` の設定解決 legacy spec
- `SPEC-package-boundary-detection.md`: monorepo package 境界検出 legacy spec
- `SPEC-markdown-document-model.md`: `overview.md`、implementation md の文書モデル legacy spec
- `SPEC-expose-uses-tables.md`: `Expose` / `Uses` テーブル legacy spec
- `SPEC-code-generation-output.md`: Source / Types / Test の生成出力 legacy spec
- `SPEC-adapter-rust-generation.md`: Rust adapter 生成 legacy spec
- `SPEC-adapter-typescript-generation.md`: TypeScript adapter 生成 legacy spec
- `SPEC-adapter-python-generation.md`: Python adapter 生成 legacy spec
- `SPEC-parser-generation-mvp-phase.md`: Parser + 生成 MVP legacy spec
- `SPEC-cli-commands.md`: CLI command legacy spec
- `SPEC-obsidian-readable-markdown.md`: Obsidian readable markdown legacy spec
- `SPEC-md-state-quality-operations.md`: Markdown state quality legacy spec
- `SPEC-doctor-command.md`: doctor command legacy spec
- `SPEC-package-sync.md`: package sync legacy spec
- `SPEC-distribution-and-versions.md`: distribution/version legacy spec
- `SPEC-ai-agent-cli-initialization.md`: AI init legacy spec
- `SPEC-init-development-environment-setup.md`: development setup legacy spec
- `SPEC-release-prepublish-quality.md`: release quality legacy spec
- `../../../../mds/core/.mds/source/overview.md`: Markdown model、generation、adapter、quality、doctor、package sync、distribution、release quality の移行先
- `../../../../mds/cli/.mds/source/overview.md`: CLI command surface の移行先
- `../../../../mds/core/.mds/source/init/mod.rs.md`: AI init / development setup の主要移行先

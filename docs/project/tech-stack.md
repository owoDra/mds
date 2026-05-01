# 技術スタック

## 目的

このファイルは、採用済み技術を役割単位で記録します。

## 読むべき場面

- 採用技術やバージョン方針を確認したいとき
- 新しい採用判断を記録したいとき

| 技術名 | 採用スタック | バージョン | ADR参照 |
| --- | --- | --- | --- |
| Rust | core、native CLI、LSP、現行 adapter 生成処理 | 1.86+ | `adr/active/ADR-003-multi-ecosystem-rust-core.md` |
| serde_json / toml Rust crates | package metadata と `mds.config.toml` の標準 parser | serde_json 1.x / toml 0.8.x | `adr/active/ADR-002-toml-only-config.md`, `adr/active/ADR-003-multi-ecosystem-rust-core.md` |
| TypeScript / Node.js | VS Code extension、TypeScript 生成対象 | Node.js 24+ | `adr/active/ADR-003-multi-ecosystem-rust-core.md` |
| Python | Python 生成対象 | 3.13+ | `adr/active/ADR-003-multi-ecosystem-rust-core.md` |
| Markdown | 設計書兼ソースの正本 | tbd | `adr/active/ADR-001-markdown-source-of-truth.md` |
| TOML | `mds.config.toml` 設定ファイル | tbd | `adr/active/ADR-002-toml-only-config.md` |
| cargo | Rust distribution / runner | Rust 1.86+ 同梱系列 | tbd |
| npm | Node.js distribution / runner | npm 10+ | tbd |
| uv | Python runner | 最新安定系列 | tbd |
| clippy / rustfmt / cargo-nextest | Rust lint / format / test 接続候補 | Rust 1.86+ 同梱系列 / 最新安定系列 | tbd |
| ESLint / Prettier / Biome / Vitest / Jest | TypeScript lint / format / test 接続候補 | 最新安定系列 | tbd |
| Ruff / Black / Pytest / unittest | Python lint / format / test 接続候補 | 最新安定系列 | tbd |
| Claude Code / Codex CLI / Opencode / GitHub Copilot CLI | AI agent kit 生成対象 CLI | 各 CLI の最新安定系列 | `adr/active/ADR-006-ai-agent-init-and-dev-setup.md` |
| SBOM / provenance / artifact signing | 全配布経路の公開前品質 gate | format / provider は release 実装時に固定 | `adr/active/ADR-006-ai-agent-init-and-dev-setup.md` |

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

## 配布と最低対応バージョン

- mds の現行利用者向け配布面は GitHub Releases の native binary archive と VS Code extension package の 2 系統です。
- GitHub Releases の native binary archive は `mds` と `mds-lsp` を含み、`install.sh` が OS / architecture に合う archive を取得します。
- VS Code extension package は platform-specific package として公開し、対応する `mds-lsp` binary を同封します。
- `mds-core` は単独配布せず、`mds-cli` / `mds-lsp` の内部 workspace dependency として binary に link します。
- runtime と toolchain の最低対応は Rust 1.86+、Node.js 24+、Python 3.13+ を基準にします。
- release 前品質 gate は `release.mds.toml` と `./.github/script/release-check.sh` を基準にし、checksum、signature、SBOM、provenance、install smoke test の欠落を許容しません。

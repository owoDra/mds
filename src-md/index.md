# mds Source Root

## Purpose

This directory is the source-of-truth root for developing mds with mds.

## Architecture

- `index.md` files describe hierarchy-level design, responsibility boundaries, exposed surface, and local rules.
- `package.md` files describe package-level metadata and package rules.
- `*.rs.md` and `*.ts.md` files are implementation Markdown documents. Their code blocks are synchronized into `.build/`.
- `.build/` contains generated workspace files and build artifacts; do not edit generated files directly.

## Exposes

| Kind | Name | Target | Summary |
| --- | --- | --- | --- |
| module | mds-core | mds-core/index.md | Rust core library source. |
| module | mds-cli | mds-cli/index.md | Native CLI source. |
| module | mds-lsp | mds-lsp/index.md | Language Server Protocol source. |
| module | vscode | vscode/index.md | VS Code extension source. |

## Rules

- Keep source-level design in the nearest `index.md`.
- Keep one implementation Markdown per feature or generated Rust file during the migration period.
- Run `scripts/sync-build.sh` before Cargo commands.
- Treat `.build/` as generated output.

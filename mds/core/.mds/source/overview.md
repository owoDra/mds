# mds-core

## Purpose

Rust core library for parsing, validating, generating, and initializing mds projects.

## Architecture

This package is authored under `mds/core/.mds/source/` and synchronized into package `src/` / `tests/` by `mds build`. The repo-local helper `./.github/script/sync-self-hosted-rust.sh` mirrors those generated files into `.build/rust/mds/core/` before Cargo commands. Package metadata is read from `../Cargo.toml`; mds does not use a package root `index.md`.

## Source Of Truth Scope

- Package-wide behavioral detail now lives in this overview and the implementation markdown files under `mds/core/.mds/source/`.
- `docs/project/specs/` is treated as a legacy migration surface. New behavioral detail belongs here or in the nearest implementation markdown file.
- Shared cross-cutting rules should prefer package overview summaries plus module-local `Purpose` / `Contract` sections over separate spec documents.

## Markdown Model And Generation

- Authoring roots are fixed to `.mds/source` and `.mds/test`.
- One implementation markdown file covers one feature and owns its executable `Types`, `Source`, and `Test` code blocks.
- `Uses` remains the canonical dependency input; generated imports and output filenames are derived from descriptors instead of language branches.
- `descriptor.rs.md`, `generation.rs.md`, `markdown.rs.md`, and the TOML files under `descriptors/` together define suffix matching, output file rules, lexical rules, scaffolds, and tool execution defaults.

### Migrated Markdown Model Rules

- `overview.md` explains package-level intent, `index.md` explains a directory boundary, implementation markdown explains one feature, and test markdown explains one verification unit.
- `Expose`, `Exposes`, `Uses`, package dependency tables, and related metadata stay in canonical Markdown table shapes so humans and parsers read the same source.
- Obsidian-readable authoring remains a constraint: headings, tables, and fenced code blocks stay standard Markdown first, and machine-only metadata stays subordinate to readable prose.

### Migrated Generation And Adapter Rules

- Source / Types / Test output paths are deterministic from authoring path plus descriptor file rules; arbitrary `file=` style output redirection is not part of the model.
- Built-in descriptors for TypeScript, Python, Rust, and overlay / framework variants define aliases, suffix matching, output naming, lexical rules, scaffold defaults, and tool behaviors from TOML instead of Rust language branches.
- Workspace-local `.mds/descriptors/*.toml` files extend or override descriptor behavior for generation, `mds new`, quality operations, and LSP validation without recompiling mds.
- Current built-in language defaults remain: `foo.ts.md -> src/foo.ts / src/foo.types.ts / tests/foo.test.ts`, `pkg/foo.py.md -> src/pkg/foo.py / src/pkg/foo.pyi / tests/pkg/test_foo.py`, and `foo.rs.md -> src/foo.rs / src/foo.types.rs / tests/foo.test.rs` unless a descriptor says otherwise.

## Quality And Diagnostics

- Markdown-state `lint`, `lint --fix`, and `test` target fenced code blocks only.
- Descriptor tooling rules choose whether a tool receives stdin, a temp file, or inline source, and regex capture rules map tool output back to Markdown path, line, and column.
- `lint --fix` rewrites only fenced code block contents and leaves headings, tables, narrative sections, and metadata untouched.

### Migrated Quality Operation Rules

- Missing required toolchains fail the operation and mark the environment incomplete; missing optional tools emit warnings only.
- Descriptor quality profiles define the default lint / fix / test commands and their required or optional tools for built-in languages.
- Tool output that already points at generated temp files is rewritten back to Markdown paths before surfacing diagnostics so editor and CLI users see source-of-truth locations.

## Config, Package Boundary, And Operations

- `config.rs.md` owns fixed `mds.config.toml` resolution and quality/adapters/package-sync settings.
- `package.rs.md`, `package_sync.rs.md`, and `doctor.rs.md` own package boundary detection, package sync behavior, and environment diagnostics.
- `release_quality.rs.md` owns release gate artifact validation for checksum, signature, SBOM, provenance, and smoke tests.

### Migrated Config And Boundary Rules

- `mds.config.toml` is the only supported configuration file. Alternate config formats are intentionally unsupported.
- Supported package-level configuration is limited to fixed TOML tables such as `package`, `roots`, `adapters`, `quality`, `doctor`, `package_sync`, and label overrides; unsupported keys are reported instead of silently becoming new behavior.
- An enabled package is discovered from `mds.config.toml` plus one package metadata root among `package.json`, `pyproject.toml`, or `Cargo.toml`.
- Package sync derives dependency snapshots from actual package metadata and may run an optional post-hook when explicitly enabled.
- `mds doctor` validates required and optional toolchains from configured quality profiles and package metadata expectations.

### Migrated Distribution And Release Rules

- Current supported distribution surfaces are Cargo crates, native binaries, and the VS Code extension package.
- Release validation requires artifact path, checksum, signature, SBOM, provenance, and smoke-test expectations per published artifact.
- Runtime minimum versions and supported toolchain policy are recorded in `docs/project/tech-stack.md` and enforced by release-quality expectations instead of a standalone spec.

## Init And AI Kit

- `init/mod.rs.md` and the template files under `init/templates/` own CLI bootstrap, AI agent kit generation, and development environment setup behavior.
- Changes that affect command surface, source-of-truth layout, or generated guidance must update the matching init templates in the same change.

### Migrated Init And AI Setup Rules

- `mds init` is the single entrypoint for project bootstrap, AI agent kit generation, and development-environment setup.
- Bootstrap is expected to work from `npx`, Cargo, and `uvx` entry paths.
- Built-in AI agent kit support covers Claude Code, Codex CLI, Opencode, and GitHub Copilot CLI. Target-specific differences stay inside templates and template metadata.
- Quality-tool defaults are data-driven from descriptor TOML, but interactive init still writes only the tools the user selected for the package.
- Toolchain installation, project dependency installation, and global AI CLI installation remain interactive-default operations unless explicit non-interactive options are provided.

### Package Summary

| Name | Version |
| --- | --- |
| mds-core | 0.1.0-alpha.1 |

### Dependencies

| Name | Version | Summary |
| --- | --- | --- |
| regex | 1 |  |
| serde | 1 |  |
| serde_json | 1 |  |
| toml | 0.8 |  |

### Dev Dependencies

| Name | Version | Summary |
| --- | --- | --- |

## Exposes

| Kind | Name | Target | Summary |
| --- | --- | --- | --- |
| module | mds-core | ../../.build/rust/mds-core | Generated Cargo package. |

## Rules

- Keep package-level source design in this overview.
- Keep implementation code in `*.rs.md` files.
- Do not edit generated files under `../src`, `../tests`, or `.build/rust/mds-core`.
- Do not add new project-level spec documents for behavior already covered by this package overview or implementation markdown.
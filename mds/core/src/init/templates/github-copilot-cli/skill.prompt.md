---
mode: agent
description: "Generate code from markdown sources using mds, validate document structure, and synchronize specifications with implementation"
mds-managed: true
---

You are an mds (Markdown Source) assistant. Markdown is the source of truth — code is generated from it.

## Commands

- `mds new <name.lang.md>` — Create new implementation markdown from template
- `mds new overview.md` — Create hierarchy overview markdown without Imports / Exports
- `mds new lib.rs.md` / `mds new sub/mod.rs.md` / `mds new index.ts.md` — Create language root module markdown for Imports / Exports
- `mds lint` — Validate markdown structure and references
- `mds build --dry-run` — Preview what would be generated
- `mds build` — Generate code from markdown sources
- `mds lint --fix --check` — Fix and validate formatting
- `mds test` — Run tests on generated outputs

## Workflow

1. Read `.mds/source/` files to understand the current state
2. Create new files with `mds new <name.lang.md>` (ensures correct template)
3. Record dependencies in Imports; keep spec-state docs code-free until implementation is ready
4. Run `mds lint` → `mds build --dry-run` → `mds build`

Always use `mds new` to scaffold new files. Examples: `mds new greet.ts.md`, `mds new sub/overview.md`

## mds Format

Source files: `.mds/source/name.{lang}.md` → generates `src/name.{lang}`
Test docs: `.mds/test/name.md` → generates language-specific test outputs and must declare `Covers`

- One file = one generated source file
- All code blocks are concatenated (separated by blank lines) to produce output
- Import/use/require statements are forbidden in code blocks; record dependencies in the Imports section table
- Each code block must contain exactly one logical unit by default
- Doc comments and docstrings belong in surrounding markdown text, not inside code blocks
- `Purpose` documents every source md; `Contract` documents impl-state behavior
- Source md without `Types` / `Source` code is spec state; adding generated code makes it impl state
- `Exports.Summary` must describe the public definition; do not use `-`
- Exported definitions referenced by other files need matching H5 shared definitions with prose
- Imports section table is required for dependencies

Rules: one md per feature, code fence language = file extension, top-level implementations split per fence by default

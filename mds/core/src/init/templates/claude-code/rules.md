---
paths:
  - ".mds/source/**,.mds/test/**"
  - "**/*.md"
mds-managed: true
---

# mds Project Rules

Markdown is the source of truth. Generated code must not be edited directly.

## Workflow

- Use `mds new <name.lang.md>` to create new source markdown files in `.mds/source/` (never create them manually). Create matching test markdown in `.mds/test/` when behavior needs executable verification and add `Covers` for the source module id
- Use `mds new overview.md` or `mds new sub/overview.md` to create new overview files under `.mds/source/` or `.mds/test/`, depending on whether you are documenting source or tests
- Run `mds package sync` after package metadata changes
- Run `mds check` before any generation-sensitive changes
- Run `mds build --dry-run` before writing generated outputs
- Run `mds lint --fix --check` to validate markdown quality

## mds Markdown Format

Source files live in `.mds/source/` as `name.{lang}.md` (e.g., `helper.ts.md` → generates `src/helper.ts`). Test docs live in `.mds/test/` as Markdown files with `Covers` and `Test` sections.

### Generation Rules

- One `.{lang}.md` file = one generated source file
- All code blocks are concatenated (separated by blank lines) to produce the output
- Import/use/require statements are forbidden in code blocks; record dependencies in Uses
- Each code block must contain exactly one logical unit (type, function, class, impl, etc.) by default
- Doc comments and docstrings belong in surrounding markdown text, not inside code blocks
- Sections (## headings) are optional documentation

### Code Blocks

- Fenced with language matching file extension (ts.md → ```ts)
- Multiple blocks are concatenated with blank lines
- Imports/use/require are forbidden in code blocks; record dependencies in Uses

### Uses Table

| Target | Summary |
| --- | --- |
| ./config | Configuration module |
| lodash | npm/PyPI/crates dependency |

### Constraints

- One source md per feature
- Keep executable test intent in `.mds/test/` with `Covers`
- Generated output naming follows built-in language descriptors
- Code fence language must match file extension
- Default `mds check` expects top-level implementations to be split per code fence; projects may relax selected checks in `[check]`
- Project-specific rules override mds rules when they conflict

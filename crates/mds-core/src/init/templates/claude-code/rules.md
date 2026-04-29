---
paths:
  - "src-md/**"
  - "**/*.md"
mds-managed: true
---

# mds Project Rules

Markdown is the source of truth. Generated code must not be edited directly.

## Reading Order

1. Read `docs/project/index.md` for project structure
2. Read requirements and specs before modifying generated behavior
3. Read `docs/project/architecture.md` for invariants

## Workflow

- Use `mds new <name.lang.md>` to create new implementation markdown files (never create them manually)
- Use `mds new index.md` or `mds new sub/index.md` to create new index files
- Run `mds check` before any generation-sensitive changes
- Run `mds build --dry-run` before writing generated outputs
- Run `mds lint --fix --check` to validate markdown quality

## mds Markdown Format

Implementation files live in `src-md/` as `name.{lang}.md` (e.g., `helper.ts.md` → generates `src/helper.ts`).

### Required Sections (all H2, in order)

```
## Purpose    — Feature description
## Contract   — Expected behavior and guarantees
## Types      — Type definitions + Uses table
## Source     — Implementation code + Uses table
## Cases      — Expected behavior examples (human reference)
## Test       — Test code + Uses table
```

### Uses Table (declares imports — NEVER put import/use/require in code blocks)

| From | Target | Expose | Summary |
| --- | --- | --- | --- |
| internal | foo/util | Util | same package module |
| package | lodash | debounce | npm/PyPI/crates dependency |
| builtin | node:fs | readFileSync | language built-in |
| workspace | @scope/lib | Config | monorepo package |

**Expose syntax**: `Name`, `Name as Alias`, `default: Name` (TS only), `* as ns`

### Code Blocks

- Fenced with language matching file extension (ts.md → ```ts)
- Multiple blocks per section are concatenated with blank lines
- NEVER include import/use/require/from statements

### index.md (per-directory, Exposes table)

| Kind | Name | Target | Summary |
| --- | --- | --- | --- |
| function | normalize | helper | string normalization |

**Kind values**: `type`, `value`, `function`, `class`, `module`

### Constraints

- One implementation md per feature
- No H1 in implementation md, no H5+ headings
- Target paths: no `.md` extension, no `./` prefix
- Project-specific rules override mds rules when they conflict

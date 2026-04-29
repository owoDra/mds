---
applyTo: "src-md/**,**/*.md"
mds-managed: true
---

# mds Project Rules

Markdown is the source of truth. Generated code must not be edited directly.

## Reading Order

1. Read `docs/project/index.md` for project structure
2. Read requirements and specs before modifying generated behavior

## Workflow

- Use `mds new <name.lang.md>` to create new implementation markdown files (never create them manually)
- Use `mds new index.md` to create new index files for directories
- Run `mds check` before any generation-sensitive changes
- Run `mds build --dry-run` before writing generated outputs
- Run `mds lint --fix --check` to validate markdown quality

## mds Markdown Format

Implementation files: `src-md/name.{lang}.md` → generates `src/name.{lang}`

### Required Sections (all H2, in order)

- `## Purpose` — Feature description
- `## Contract` — Behavior guarantees
- `## Types` — Type definitions + Uses table
- `## Source` — Implementation + Uses table
- `## Cases` — Example behaviors (human reference)
- `## Test` — Test code + Uses table

### Uses Table (declares imports — NEVER put import/use/require in code blocks)

| From | Target | Expose | Summary |
| --- | --- | --- | --- |
| internal | foo/util | Util | same package module |
| package | lodash | debounce | external dependency |
| builtin | node:fs | readFileSync | language built-in |
| workspace | @scope/lib | Config | monorepo package |

Expose tokens: `Name`, `Name as Alias`, `default: Name` (TS only), `* as ns`

### Constraints

- One implementation md per feature
- No H1 in implementation md; no H5+ headings
- Code fence language must match file extension
- Target paths: no `.md`, no `./` prefix
- Project-specific rules override mds rules when they conflict

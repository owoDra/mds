---
mds-managed: true
---

# mds Project Rules

Markdown is the source of truth. Generated code must not be edited directly.

## Reading Order

1. Read `docs/project/index.md` for project structure
2. Read requirements and specs before modifying generated behavior
3. Read `docs/project/architecture.md` for invariants

## Dev Environment

```sh
mds new <name.lang.md>  # Create new implementation markdown from template
mds new index.md        # Create new index markdown for a directory
mds check               # Validate markdown structure
mds build --dry-run     # Preview generation output
mds build               # Generate code from markdown
mds lint --fix --check  # Fix and validate formatting
mds test                # Run tests on generated outputs
```

Always use `mds new` to create new `src-md/` files. Never create them manually.

## mds Markdown Format

Implementation files live in `src-md/` as `name.{lang}.md` (e.g., `helper.ts.md` → generates `src/helper.ts`).

### Required Sections (all H2, in order)

- `## {{PURPOSE}}` — Feature description
- `## {{CONTRACT}}` — Behavior guarantees
- `## {{TYPES}}` — Type definitions + Uses table
- `## {{SOURCE}}` — Implementation + Uses table
- `## {{CASES}}` — Example behaviors (human reference)
- `## {{TEST}}` — Test code + Uses table

### Uses Table (declares imports — NEVER put import/use/require in code blocks)

| From | Target | {{EXPOSE}} | Summary |
| --- | --- | --- | --- |
| internal | foo/util | Util | same package module |
| package | lodash | debounce | external dependency |
| builtin | node:fs | readFileSync | language built-in |
| workspace | @scope/lib | Config | monorepo package |

Expose tokens: `Name`, `Name as Alias`, `default: Name` (TS only), `* as ns`

### index.md (per-directory, Exposes table)

| Kind | Name | Target | Summary |
| --- | --- | --- | --- |
| function | normalize | helper | string normalization |

Kind values: `type`, `value`, `function`, `class`, `module`

### Constraints

- One implementation md per feature
- No H1 in implementation md; no H5+ headings
- Code fence language must match file extension
- Target paths: no `.md`, no `./` prefix
- Multiple code blocks in one section → concatenated
- Project-specific rules override mds rules when they conflict

## Testing

- Run `mds check` to validate structure before committing
- Run `mds test` to run all generated tests
- Fix any errors before creating PRs

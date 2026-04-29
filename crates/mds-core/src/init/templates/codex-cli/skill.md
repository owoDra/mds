---
mds-managed: true
---

# mds Skill

## Purpose

Work with the mds (Markdown-Driven Specification) system where Markdown is the single source of truth for code generation.

## Commands

```sh
mds new <name.lang.md>  # Create new implementation markdown from template
mds new index.md        # Create new index markdown for a directory
mds check               # Validate markdown structure
mds build --dry-run     # Preview generation output
mds build               # Generate code from markdown
mds lint --fix --check  # Fix and validate formatting
mds test                # Run tests on generated outputs
```

## Workflow

1. Read existing `src-md/` files to understand the current state
2. Create new files with `mds new <name.lang.md>` (ensures correct template)
3. Fill in Purpose, Expose, Uses, Types, Source, Test sections
4. Run `mds check` → `mds build --dry-run` → `mds build` → `mds test`

Always use `mds new` to scaffold new files. Example: `mds new greet.ts.md`, `mds new sub/index.md`

## mds Markdown Format

Files: `src-md/name.{lang}.md` → generates `src/name.{lang}`

### Sections (all H2, in order)

- `## Purpose` — Feature description
- `## Contract` — Behavior guarantees
- `## Types` — Types + Uses table
- `## Source` — Implementation + Uses table
- `## Cases` — Example behaviors
- `## Test` — Tests + Uses table

### Uses Table (NEVER put imports in code blocks)

| From | Target | Expose | Summary |
| --- | --- | --- | --- |
| internal | foo/util | Util | same package |
| package | lodash | debounce | external dep |
| builtin | node:fs | readFileSync | std lib |
| workspace | @scope/lib | Config | monorepo |

### Rules

- One `.{lang}.md` per feature
- No H1 in implementation md; no H5+
- Code fence language = file extension
- Target paths: no `.md`, no `./`

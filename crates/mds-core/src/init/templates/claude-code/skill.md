---
name: mds
description: Generate code from markdown sources, validate document structure, and synchronize markdown-driven specifications with implementation
mds-managed: true
---

## What this skill does

This skill helps you work with the mds (Markdown-Driven Specification) system. mds treats Markdown files as the single source of truth and generates code from them.

## When to use

- When adding or modifying features defined in markdown
- When code generation output needs updating
- When validating that markdown sources and generated code are in sync

## Commands

```sh
mds new <name.lang.md>  # Create new implementation markdown from template
mds new index.md        # Create new index markdown for a directory
mds check               # Validate markdown structure and references
mds build --dry-run     # Preview what would be generated
mds build               # Generate code from markdown sources
mds lint --fix --check  # Fix and validate formatting
mds test                # Run tests on generated outputs
```

## Workflow

1. Read existing `src-md/` files to understand the current state
2. Create new markdown files with `mds new <name.lang.md>` (generates correct template with all required sections)
3. Fill in Purpose, Expose, Uses, Types, Source, Test sections
4. Run `mds check` to validate structure
5. Run `mds build --dry-run` to preview generation
6. Run `mds build` to generate code
7. Run `mds test` to verify correctness

## Creating New Files

Always use `mds new` to create implementation markdown files. This ensures the correct format:

```sh
mds new greet.ts.md                    # New TypeScript feature
mds new utils/helper.py.md             # New Python feature in subdirectory
mds new parser.rs.md                   # New Rust feature
mds new sub/index.md                   # New index for subdirectory
mds new greet.ts.md --package ./my-pkg # Specify target package
```

## mds Markdown Format

Implementation files: `src-md/name.{lang}.md` → generates `src/name.{lang}`

### Required Sections (all H2, in order)

- `## Purpose` — Feature description
- `## Contract` — Behavior guarantees
- `## Types` — Type definitions + Uses table
- `## Source` — Implementation + Uses table
- `## Cases` — Example behaviors (human reference)
- `## Test` — Test code + Uses table

### Uses Table (declares imports)

CRITICAL: Never put import/use/require statements in code blocks. Use this table instead:

| From | Target | Expose | Summary |
| --- | --- | --- | --- |
| internal | foo/util | Util, helper | same package |
| package | lodash | debounce | external dep |
| builtin | node:fs | readFileSync | std lib |
| workspace | @scope/lib | Config | monorepo |

Expose tokens: `Name`, `Name as Alias`, `default: Name` (TS), `* as ns`

### Constraints

- One `.{lang}.md` per feature (one file = one generated source)
- No H1 in implementation md; no H5+ headings
- Code fence language must match file extension
- Target paths: no `.md`, no `./` prefix
- Multiple code blocks in one section → concatenated

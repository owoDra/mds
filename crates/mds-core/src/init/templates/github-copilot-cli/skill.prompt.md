---
mode: agent
description: "Generate code from markdown sources using mds, validate document structure, and synchronize specifications with implementation"
mds-managed: true
---

You are an mds (Markdown-Driven Specification) assistant. Markdown is the source of truth — code is generated from it.

## Commands

- `mds check` — Validate markdown structure and references
- `mds build --dry-run` — Preview what would be generated
- `mds build` — Generate code from markdown sources
- `mds lint --fix --check` — Fix and validate formatting
- `mds test` — Run tests on generated outputs

## Workflow

1. Read `src-md/` files to understand the current state
2. Create/modify markdown following the format below
3. Run `mds check` → `mds build --dry-run` → `mds build` → `mds test`

## mds Format

Files: `src-md/name.{lang}.md` → generates `src/name.{lang}`

Required H2 sections in order: Purpose, Contract, Types, Source, Cases, Test

Uses table declares imports (NEVER put import/use/require in code blocks):

| From | Target | Expose | Summary |
| --- | --- | --- | --- |
| internal | foo/util | Util | same package |
| package | lodash | debounce | external dep |
| builtin | node:fs | readFileSync | std lib |
| workspace | @scope/lib | Config | monorepo |

Expose tokens: `Name`, `Name as Alias`, `default: Name` (TS), `* as ns`

Rules: one md per feature, no H1 in impl md, no H5+, fence lang = file ext

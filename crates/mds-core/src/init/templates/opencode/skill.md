---
name: mds
description: Generate code from markdown sources, validate document structure, and synchronize markdown-driven specifications with implementation
mds-managed: true
---

## What this skill does

Work with the mds (Markdown-Driven Specification) system where Markdown is the single source of truth for code generation.

## When to use

- Adding or modifying features defined in markdown
- Code generation output needs updating
- Validating markdown-to-code synchronization

## Commands

```sh
mds check           # Validate markdown structure
mds build --dry-run # Preview generation
mds build           # Generate code
mds lint --fix --check  # Fix formatting
mds test            # Run generated tests
```

## mds Format Quick Reference

Files: `src-md/name.{lang}.md` → generates `src/name.{lang}`

Required H2 sections in order: Purpose, Contract, Types, Source, Cases, Test

Uses table (declares imports — NEVER put import/use/require in code blocks):

| From | Target | Expose | Summary |
| --- | --- | --- | --- |
| internal | foo/util | Util | same package |
| package | lodash | debounce | external dep |
| builtin | node:fs | readFileSync | std lib |
| workspace | @scope/lib | Config | monorepo |

Rules: one md per feature, no H1 in impl md, no H5+, code fence = file extension

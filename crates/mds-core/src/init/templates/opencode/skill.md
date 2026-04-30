---
name: mds
description: Generate code from markdown sources, validate document structure, and synchronize markdown sources with implementation
mds-managed: true
---

## What this skill does

Work with the mds (Markdown Source) system where Markdown is the single source of truth for code generation.

## When to use

- Adding or modifying features defined in markdown
- Code generation output needs updating
- Validating markdown-to-code synchronization

## Commands

```sh
mds new <name.lang.md>  # Create new implementation markdown from template
mds new index.md        # Create new index markdown for a directory
mds check               # Validate markdown structure
mds build --dry-run     # Preview generation
mds build               # Generate code
mds lint --fix --check  # Fix formatting
mds test                # Run generated tests
```

Always use `mds new` to create new files: `mds new greet.ts.md`, `mds new sub/index.md`

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

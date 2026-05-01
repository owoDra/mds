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

- One file = one generated source file
- All code blocks are concatenated (separated by blank lines) to produce output
- Imports go directly in code blocks (first block)
- Sections (## headings) are optional, for documentation
- Dependencies table is optional documentation

Rules: one md per feature, code fence language = file extension

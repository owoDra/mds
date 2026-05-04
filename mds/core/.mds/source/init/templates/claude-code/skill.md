---
name: mds
description: Generate code from markdown sources, validate document structure, and synchronize markdown sources with implementation
mds-managed: true
---

## What this skill does

This skill helps you work with the mds (Markdown Source) system. mds treats Markdown files as the single source of truth and generates code from them.

## When to use

- When adding or modifying features defined in markdown
- When code generation output needs updating
- When validating that markdown sources and generated code are in sync

## Commands

```sh
mds new <name.lang.md>  # Create new implementation markdown from template
mds new overview.md        # Create new overview markdown for a directory
mds lint               # Validate markdown structure and references
mds build --dry-run     # Preview what would be generated
mds build               # Generate code from markdown sources
mds lint --fix --check  # Fix and validate formatting
mds test                # Run tests on generated outputs
```

## Workflow

1. Read existing `.mds/source/` files to understand the current state
2. Create new markdown files with `mds new <name.lang.md>` (generates correct template)
3. Record dependencies in Uses; write implementation-only code blocks
4. Run `mds lint` to validate structure
5. Run `mds build --dry-run` to preview generation
6. Run `mds build` to generate code
7. Run `mds test` to verify correctness

## Creating New Files

Always use `mds new` to create implementation markdown files:

```sh
mds new greet.ts.md                    # New TypeScript feature
mds new utils/helper.py.md             # New Python feature in subdirectory
mds new parser.rs.md                   # New Rust feature
mds new sub/overview.md                   # New source overview for subdirectory
mds new greet.ts.md --package ./my-pkg # Specify target package
```

## Format References

- See `.mds/reference/overview.md` for source overview format
- See `.mds/reference/index.md` for directory index format
- See `.mds/reference/impl.md` for source implementation md format
- See `.mds/reference/test.md` for test md format

## mds Markdown Format

Implementation files: `.mds/source/name.{lang}.md` → generates `src/name.{lang}`

### Generation Rules

- One `.{lang}.md` file = one generated source file
- All code blocks in the file are concatenated (separated by blank lines) to produce the output
- Import/use/require statements are forbidden in code blocks; record dependencies in the Imports section table
- Each code block must contain exactly one logical unit (type, function, class, impl, etc.) by default
- Doc comments and docstrings belong in surrounding markdown text, not inside code blocks

### Sections

All sections are optional. Recommended structure:

- `## {{PURPOSE}}` — Feature description (documentation only)
- `## {{CONTRACT}}` — Behavior guarantees (documentation only)
- `## {{SOURCE}}` — Implementation code blocks
- `## {{CASES}}` — Example behaviors (documentation only)

### {{IMPORTS}} Section

| {{FROM}} | {{TARGET}} | {{SYMBOLS}} | {{VIA}} | {{SUMMARY}} | {{REFERENCE}} |
| --- | --- | --- | --- | --- | --- |
| internal | ./config | Config | - | Configuration module | [./config.ts.md#config](./config.ts.md#config) |
| external | lodash | mapValues | - | Utility library | - |

##### Config

Add an H5 section for shared definitions that other modules or packages import.

### Constraints

- One `.{lang}.md` per feature (one file = one generated source)
- Code fence language must match file extension
- Imports/use/require are forbidden in code blocks; record dependencies in the Imports section table
- Default `mds lint` expects top-level implementations to be split per code fence; projects may relax selected checks in `[check]`
- Multiple code blocks per section → concatenated with blank lines

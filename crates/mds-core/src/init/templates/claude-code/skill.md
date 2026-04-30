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
mds new index.md        # Create new index markdown for a directory
mds check               # Validate markdown structure and references
mds build --dry-run     # Preview what would be generated
mds build               # Generate code from markdown sources
mds lint --fix --check  # Fix and validate formatting
mds test                # Run tests on generated outputs
```

## Workflow

1. Read existing `src-md/` files to understand the current state
2. Create new markdown files with `mds new <name.lang.md>` (generates correct template)
3. Write imports in the first code block, then implementation in subsequent blocks
4. Run `mds check` to validate structure
5. Run `mds build --dry-run` to preview generation
6. Run `mds build` to generate code
7. Run `mds test` to verify correctness

## Creating New Files

Always use `mds new` to create implementation markdown files:

```sh
mds new greet.ts.md                    # New TypeScript feature
mds new utils/helper.py.md             # New Python feature in subdirectory
mds new parser.rs.md                   # New Rust feature
mds new sub/index.md                   # New index for subdirectory
mds new greet.ts.md --package ./my-pkg # Specify target package
```

## mds Markdown Format

Implementation files: `src-md/name.{lang}.md` → generates `src/name.{lang}`

### Generation Rules

- One `.{lang}.md` file = one generated source file
- All code blocks in the file are concatenated (separated by blank lines) to produce the output
- Import statements go in their own code block at the top
- Each logical unit (type, function, class) should be its own code block

### Sections

All sections are optional. Recommended structure:

- `## Purpose` — Feature description (documentation only)
- `## Contract` — Behavior guarantees (documentation only)
- `## Source` — Implementation code blocks
- `## Cases` — Example behaviors (documentation only)

### Dependencies Table (optional, documentation only)

| Target | Summary |
| --- | --- |
| ./config | Configuration module |
| lodash | Utility library |

### Constraints

- One `.{lang}.md` per feature (one file = one generated source)
- Code fence language must match file extension
- Imports go directly in code blocks (first block)
- Multiple code blocks per section → concatenated with blank lines

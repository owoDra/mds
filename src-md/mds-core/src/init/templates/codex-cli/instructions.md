---
mds-managed: true
---

# mds Project Rules

Markdown is the source of truth. Generated code must not be edited directly.

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

### Generation Rules

- One `.{lang}.md` file = one generated source file
- All code blocks are concatenated (separated by blank lines) to produce the output
- Import statements go in their own code block at the top
- Each logical unit (type, function, class) should be its own code block
- Sections (## headings) are optional documentation

### Dependencies Table (optional, documentation only)

| Target | Summary |
| --- | --- |
| ./config | Configuration module |
| lodash | Utility library |

### Constraints

- One implementation md per feature
- Code fence language must match file extension
- Imports go directly in code blocks
- Project-specific rules override mds rules when they conflict

## Testing

- Run `mds check` to validate structure before committing
- Run `mds test` to run all generated tests
- Fix any errors before creating PRs

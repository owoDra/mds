---
mds-managed: true
---

# mds Project Rules

Markdown is the source of truth. Generated code must not be edited directly.

## Dev Environment

```sh
mds new <name.lang.md>  # Create new implementation markdown from template
mds new overview.md        # Create new overview markdown for a directory
mds lint               # Validate markdown structure
mds build --dry-run     # Preview generation output
mds build               # Generate code from markdown
mds lint --fix --check  # Fix and validate formatting
mds test                # Run tests on generated outputs
```

Always use `mds new` to create new `.mds/source/` files and add matching `.mds/test/` files when behavior needs executable verification. Never create managed scaffolding manually.

## mds Markdown Format

Source files live in `.mds/source/` as `name.{lang}.md` (e.g., `helper.ts.md` → generates `src/helper.ts`). Test docs live in `.mds/test/` as Markdown files with `Covers` and `Test` sections.

### Generation Rules

- One `.{lang}.md` file = one generated source file
- All code blocks are concatenated (separated by blank lines) to produce the output
- Import/use/require statements are forbidden in code blocks; record dependencies in the Imports section table
- Each code block must contain exactly one logical unit (type, function, class, impl, etc.) by default
- Doc comments and docstrings belong in surrounding markdown text, not inside code blocks
- Sections (## headings) are optional documentation

### Imports Section

| Target | Summary |
| --- | --- |
| ./config | Configuration module |
| lodash | Utility library |

### Constraints

- One source md per feature
- Keep executable test intent in `.mds/test/` with `Covers`
- Generated output naming follows built-in language descriptors
- Code fence language must match file extension
- Imports/use/require are forbidden in code blocks; record dependencies in the Imports section table
- Default `mds lint` expects top-level implementations to be split per code fence; projects may relax selected checks in `[check]`
- Project-specific rules override mds rules when they conflict

## Testing

- Run `mds lint` to validate structure before committing
- Run `mds test` to run all generated tests
- Fix any errors before creating PRs

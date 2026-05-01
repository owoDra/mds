---
applyTo: "src-md/**,**/*.md"
mds-managed: true
---

# mds Project Rules

Markdown is the source of truth. Generated code must not be edited directly.

## Workflow

- Use `mds new <name.lang.md>` to create new implementation markdown files (never create them manually)
- Use `mds new index.md` to create new index files for directories
- Run `mds check` before any generation-sensitive changes
- Run `mds build --dry-run` before writing generated outputs
- Run `mds lint --fix --check` to validate markdown quality

## mds Markdown Format

Implementation files: `src-md/name.{lang}.md` → generates `src/name.{lang}`

### Generation Rules

- One `.{lang}.md` file = one generated source file
- All code blocks are concatenated (separated by blank lines) to produce the output
- Imports go directly in code blocks (first block)
- Each logical unit should be its own code block
- Sections (## headings) are optional documentation

### Dependencies Table (optional, documentation only)

| Target | Summary |
| --- | --- |
| ./config | Configuration module |
| lodash | Utility library |

### Constraints

- One implementation md per feature
- Code fence language must match file extension
- Project-specific rules override mds rules when they conflict

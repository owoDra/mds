---
paths:
  - "src-md/**"
  - "**/*.md"
mds-managed: true
---

# mds Project Rules

Markdown is the source of truth. Generated code must not be edited directly.

## Workflow

- Use `mds new <name.lang.md>` to create new implementation markdown files (never create them manually)
- Use `mds new index.md` or `mds new sub/index.md` to create new index files
- Run `mds check` before any generation-sensitive changes
- Run `mds build --dry-run` before writing generated outputs
- Run `mds lint --fix --check` to validate markdown quality

## mds Markdown Format

Implementation files live in `src-md/` as `name.{lang}.md` (e.g., `helper.ts.md` → generates `src/helper.ts`).

### Generation Rules

- One `.{lang}.md` file = one generated source file
- All code blocks are concatenated (separated by blank lines) to produce the output
- Import statements go in their own code block at the top
- Each logical unit (type, function, class) should be its own code block
- Sections (## headings) are optional documentation

### Code Blocks

- Fenced with language matching file extension (ts.md → ```ts)
- Multiple blocks are concatenated with blank lines
- Imports go directly in code blocks (first block)

### Dependencies Table (optional, documentation only)

| Target | Summary |
| --- | --- |
| ./config | Configuration module |
| lodash | npm/PyPI/crates dependency |

### Constraints

- One implementation md per feature
- Code fence language must match file extension
- Project-specific rules override mds rules when they conflict

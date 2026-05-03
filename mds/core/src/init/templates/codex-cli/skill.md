---
mds-managed: true
---

# mds Skill

## {{PURPOSE}}

Work with the mds (Markdown Source) system where Markdown is the single source of truth for code generation.

## Commands

```sh
mds new <name.lang.md>  # Create new implementation markdown from template
mds new overview.md        # Create new overview markdown for a directory
mds lint               # Validate markdown structure
mds build --dry-run     # Preview generation output
mds build               # Generate code from markdown
mds lint --fix --check  # Fix and validate formatting
mds test                # Run tests on generated outputs
```

## Workflow

1. Read existing `.mds/source/` files to understand the current state
2. Create new files with `mds new <name.lang.md>` (ensures correct template)
3. Record dependencies in Uses and write implementation-only code blocks
4. Run `mds lint` → `mds build --dry-run` → `mds build`

Always use `mds new` to scaffold new files. Example: `mds new greet.ts.md`, `mds new sub/overview.md`

## Format References

- See `.mds/reference/overview.md` for source overview format
- See `.mds/reference/index.md` for directory index format
- See `.mds/reference/impl.md` for source implementation md format
- See `.mds/reference/test.md` for test md format

## mds Markdown Format

Source files: `.mds/source/name.{lang}.md` → generates `src/name.{lang}`
Test docs: `.mds/test/name.md` → generates language-specific test outputs and must declare `Covers`

### Structure

- One `.{lang}.md` file = one generated source file
- All code blocks in the file are concatenated (separated by blank lines) to produce the output
- Import/use/require statements are forbidden in code blocks; record dependencies in the Imports section table
- Each code block must contain exactly one logical unit (type, function, class, impl, etc.) by default
- Doc comments and docstrings belong in surrounding markdown text, not inside code blocks
- Sections (## headings) are optional and for documentation only

### Example

```markdown
## {{PURPOSE}}
Description of the feature.

## {{SOURCE}}

\`\`\`ts
export function greet(config: Config): string {
  return \`Hello, ${config.name}!\`;
}
\`\`\`

## Imports

| Phase | Target | Names | Summary |
| --- | --- | --- | --- |
| Source | ./config | Config | Configuration type. |
```

### Rules

- One `.{lang}.md` per feature
- Code fence language must match file extension
- Imports/use/require are forbidden in code blocks; record dependencies in the Imports section table
- Default `mds lint` expects top-level implementations to be split per code fence; projects may relax selected checks in `[check]`
- Imports section table is required for dependencies

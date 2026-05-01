---
mds-managed: true
---

# mds Skill

## {{PURPOSE}}

Work with the mds (Markdown Source) system where Markdown is the single source of truth for code generation.

## Commands

```sh
mds new <name.lang.md>  # Create new implementation markdown from template
mds new index.md        # Create new index markdown for a directory
mds check               # Validate markdown structure
mds build --dry-run     # Preview generation output
mds build               # Generate code from markdown
mds lint --fix --check  # Fix and validate formatting
mds test                # Run tests on generated outputs
```

## Workflow

1. Read existing `src-md/` files to understand the current state
2. Create new files with `mds new <name.lang.md>` (ensures correct template)
3. Write import statements and implementation in code blocks
4. Run `mds check` → `mds build --dry-run` → `mds build`

Always use `mds new` to scaffold new files. Example: `mds new greet.ts.md`, `mds new sub/index.md`

## mds Markdown Format

Files: `src-md/name.{lang}.md` → generates `src/name.{lang}`

### Structure

- One `.{lang}.md` file = one generated source file
- All code blocks in the file are concatenated (separated by blank lines) to produce the output
- Import statements go in their own code block at the top
- Each logical unit (type, function, class) should be its own code block
- Sections (## headings) are optional and for documentation only

### Example

```markdown
## {{PURPOSE}}
Description of the feature.

## {{SOURCE}}

\`\`\`ts
import { Config } from './config';
\`\`\`

\`\`\`ts
export function greet(config: Config): string {
  return \`Hello, ${config.name}!\`;
}
\`\`\`

### Dependencies
| Target | Summary |
| --- | --- |
| ./config | Configuration module |
```

### Rules

- One `.{lang}.md` per feature
- Code fence language must match file extension
- Imports go directly in code blocks (first block)
- Dependencies table is optional documentation

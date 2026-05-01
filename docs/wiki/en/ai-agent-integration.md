# AI Agent Integration

> *This page was translated from [Japanese](../ja/ai-agent-integration.md) by AI.*

This page explains the mechanism of mds's AI coding agent configuration files (agent kit) and how to add templates for new AI CLIs.

## Overview

`mds init --ai` generates configuration files following each AI CLI's best practices so that AI coding agents can work correctly in mds projects.

The generated files contain mds's Markdown format specifications (Uses tables, section structure, constraints), enabling AI agents to create implementation Markdown in the correct format.

## Supported AI CLIs

| AI CLI | Identifier | Output Path | Features |
| --- | --- | --- | --- |
| Claude Code | `claude-code`, `claude` | `.claude/rules/`, `.claude/skills/`, `.claude/commands/` | Path-scoped rules for auto-loading, skills for on-demand reference, slash commands |
| Codex CLI | `codex-cli`, `codex` | `.codex/instructions.md`, `.codex/skills/` | Instructions and skills complementing AGENTS.md |
| Opencode | `opencode` | `.opencode/agents/`, `.opencode/skills/` | Subagent definitions (build/check), skills with YAML frontmatter |
| GitHub Copilot | `github-copilot-cli`, `copilot` | `.github/instructions/`, `.github/prompts/` | Path-specific instructions with applyTo frontmatter, prompt files |

## Usage

```bash
# Generate all categories for all AI CLIs (plan display only)
mds init --ai --target all --categories all

# Apply the plan
mds init --ai --target all --categories all --yes

# For Claude Code only
mds init --ai --target claude-code --yes

# Specific categories only
mds init --ai --target all --categories instructions,skills --yes
```

### Categories

| Category | Description |
| --- | --- |
| `instructions` | AI CLI rule files. Documents mds workflows and Markdown format |
| `skills` | Detailed skill definitions referenced on demand |
| `commands` | Immediately executable command definitions (mds check, mds build, etc.) |

### Options

| Flag | Description |
| --- | --- |
| `--ai` | Run AI initialization only (skip project initialization) |
| `--target <list>` | Specify target AI CLIs as comma-separated list. `all` for all targets |
| `--categories <list>` | Specify generation categories as comma-separated list. `all` for all categories |
| `--yes` | Actually apply the plan |
| `--force` | Allow overwriting non-managed files |

## Design Principles

### Non-invasive to main files

**User-owned files such as CLAUDE.md, AGENTS.md, and copilot-instructions.md are never generated or modified.** Files are placed in each CLI's native reference path, and after generation, guidance on how to integrate with main files is displayed.

### Frontmatter management

Generated files include `mds-managed: true` in their YAML frontmatter. This enables:

- Safe updates when re-running `mds init`
- Clear distinction from non-managed (manually created) files
- Non-managed files are not overwritten without `--force`

### Native format for each AI CLI

Templates follow each AI CLI's best practices:

- **Claude Code**: Path-scoped rules in `.claude/rules/` (target files specified via `paths` in frontmatter)
- **Opencode**: Subagent definitions in `.opencode/agents/` (permission control via `mode` and `tools` in frontmatter)
- **GitHub Copilot**: Path-specific instructions in `.github/instructions/` (target specified via `applyTo` in frontmatter)
- **Codex CLI**: Instructions and skills in `.codex/`

## Adding a New AI CLI (for mds developers)

mds adopts a data-driven template system. To add support for a new AI CLI, follow these steps.

### 1. Create template directory

```
src-md/mds-core/src/init/templates/<target-key>/
├── manifest.toml       ← File mapping definition
├── instructions.md     ← Template for instructions category
├── skill.md            ← Template for skills category
└── command-check.md    ← Template for commands category
```

`<target-key>` must match the string returned by the `key()` method of the `AiTarget` enum.

### 2. Define manifest.toml

```toml
# Each [[file]] entry maps a template file to an output path

[[file]]
template = "instructions.md"     # Template filename
output_path = ".new-cli/rules.md"  # Relative path from project root
category = "instructions"         # One of: instructions, skills, commands

[[file]]
template = "skill.md"
output_path = ".new-cli/skills/mds.md"
category = "skills"
```

### 3. Create template files

Templates should include:

- The target AI CLI's native frontmatter format
- `mds-managed: true` (for update detection on re-run)
- mds Markdown format reference (Uses tables, section structure, constraints)
- mds command list (check, build, lint, test)

### 4. Add a variant to the AiTarget enum

In `src-md/mds-core/src/model/mod.rs.md`, add to the `AiTarget` enum:

```rust
pub enum AiTarget {
    // ...existing...
    NewCli,
}
```

Make `key()` return `"new-cli"` and define accepted aliases in `parse()`.

### 5. Build and verify

```bash
./scripts/sync-build.sh && cd .build/rust && cargo build && cargo test
```

build.rs automatically detects manifest.toml and registers it in the template registry. No changes to the init logic are needed.

## Content to Include in Templates

For AI agents to work correctly in mds projects, templates need the following information:

1. **Workflow**: `mds check` → `mds build --dry-run` → `mds build` → `mds test`
2. **File naming convention**: `src-md/name.{lang}.md` → `src/name.{lang}`
3. **Required section structure**: Purpose, Contract, Types, Source, Cases, Test (H2, fixed order)
4. **Uses table specification**: From (internal/package/builtin/workspace), Target, Expose, Summary
5. **Critical constraint**: Do not write import/use/require inside code blocks
6. **Expose token syntax**: `Name`, `Name as Alias`, `default: Name`, `* as ns`
7. **index.md Exposes table**: Kind, Name, Target, Summary
8. **Heading constraints**: No H1 in implementation md, no H5+

# AI Agent Integration

> *This page was translated from [Japanese](../ja/ai-agent-integration.md) by AI.*

This page explains the current AI agent kit flow in mds.

## Overview

`mds init --ai` generates instructions, skills, and command files for supported AI CLIs so agents can work with authoring-v2 packages without inventing their own document shape.

The generated guidance teaches:

- canonical `.mds/source` and `.mds/test` roots
- tableless source and test documents
- `mds new` for scaffolding new docs
- the normal validation flow for generated outputs

## Supported AI CLIs

| AI CLI | Identifier | Output path |
| --- | --- | --- |
| Claude Code | `claude-code`, `claude` | `.claude/rules/`, `.claude/skills/`, `.claude/commands/` |
| Codex CLI | `codex-cli`, `codex` | `.codex/instructions.md`, `.codex/skills/` |
| Opencode | `opencode` | `.opencode/agents/`, `.opencode/skills/` |
| GitHub Copilot | `github-copilot-cli`, `copilot` | `.github/instructions/`, `.github/prompts/` |

## Basic Usage

```bash
# Show the plan
mds init --ai --target all --categories all

# Apply the plan
mds init --ai --target all --categories all --yes

# Generate only for one CLI
mds init --ai --target claude-code --yes
```

## Categories

| Category | Purpose |
| --- | --- |
| `instructions` | Always-on rules and workflow guidance |
| `skills` | Detailed on-demand reference files |
| `commands` | Ready-to-run command snippets for the target CLI |

## Design Rules

- User-owned files such as `CLAUDE.md`, `AGENTS.md`, and `copilot-instructions.md` are not rewritten.
- Generated files include `mds-managed: true` so reruns can update them safely.
- Non-managed files are only overwritten when `--force` is given.

## What the Templates Should Teach

Templates should explain the current live surface:

1. Source docs live in `.mds/source` and test docs live in `.mds/test`.
2. Source docs use `Purpose`, `Contract`, `API`, `Source`, and `Cases`.
3. Test docs use `Purpose`, `Covers`, `Cases`, and `Test`.
4. New docs should be scaffolded with `mds new`.
5. Validation normally runs through `mds lint`, `mds build --dry-run`, `mds build`, `mds typecheck`, and `mds test`.

## Adding a New AI CLI

Create a template directory under `mds/core/src/init/templates/<target-key>/` with a `manifest.toml` plus category templates.

Typical files:

```text
mds/core/src/init/templates/<target-key>/
├── manifest.toml
├── instructions.md
├── skill.md
└── command-check.md
```

Then:

1. Add the target to `AiTarget`.
2. Keep template output paths in the new CLI's native location.
3. Include `mds-managed: true` in generated frontmatter.
4. Validate with `cargo check --workspace`, `cargo test --workspace`, and a focused `mds init --ai` smoke test.

`build.rs` registers template manifests automatically, so no separate sync step is needed.
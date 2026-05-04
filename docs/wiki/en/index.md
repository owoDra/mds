# mds English Wiki

> *This page was translated from [Japanese](../ja/index.md) by AI.*

This wiki is for people who want to use mds, consider adopting it, or participate in its development.

mds is a development toolchain that treats Markdown as the source of truth for design, implementation, and testing, generating language-specific derived code from code blocks within Markdown.

## What is your goal?

### I want to try mds

1. [Getting Started](getting-started.md) — Prerequisites and minimal setup
2. [Example Projects](../../../examples/) — Working minimal examples
3. [Commands](commands.md) — Basic usage

### I want to understand the philosophy of mds

1. [Core Concepts](concepts.md) — Source of truth, derived code, public surface
2. [Markdown Source](markdown-source.md) — Types and roles of Markdown documents
3. [Generation Mechanism](generation.md) — Code generation rules

### I want to introduce mds to an existing project

1. [Getting Started](getting-started.md) — Verify minimal setup
2. [Configuration](configuration.md) — Details of mds.config.toml
3. [Monorepo Usage](monorepo.md) — Managing multiple packages

### I want to integrate with AI agents

1. [AI Agent Integration](ai-agent-integration.md) — Supported CLIs and configuration generation

### I want to use mds in my editor

1. [Editor Integration (LSP)](editor-integration.md) — VSCode extension, Neovim, real-time diagnostics

### I want to solve a problem

1. [Troubleshooting](troubleshooting.md) — Common problems and solutions
2. [Quality Checks](quality.md) — Running checks and diagnostics

### I want to contribute to mds development

1. [Contributing](contributing.md) — Policies for reports and proposals
2. [Development Guide](development.md) — Environment setup, build, test, debug
3. [LSP Development Guide](lsp-development.md) — Developing, debugging, and adding features to mds-lsp
4. [Descriptor Guide](descriptors.md) — Language, quality tool, and package manager descriptors

## Reading Order

If you are reading for the first time, the following order is recommended.

1. [Getting Started](getting-started.md)
2. [Core Concepts](concepts.md)
3. [Markdown Source](markdown-source.md)
4. [Commands](commands.md)
5. [Configuration](configuration.md)
6. [Generation Mechanism](generation.md)

## Page List

| Page | Description |
| --- | --- |
| [Getting Started](getting-started.md) | Explains pre-installation prerequisites, minimal setup, and basic execution steps. |
| [Core Concepts](concepts.md) | Explains terms such as source of truth, derived code, implementation Markdown, public surface, and dependencies. |
| [Markdown Source](markdown-source.md) | Explains the types and roles of Markdown documents handled by mds. |
| [Commands](commands.md) | Explains the purpose and usage of each mds command. |
| [Configuration](configuration.md) | Explains the role and main settings of `mds.config.toml`. |
| [Monorepo Usage](monorepo.md) | Explains per-package target detection and multi-language handling. |
| [Generation Mechanism](generation.md) | Explains the rules for generating derived code from `Types`, `Source`, and `Test`. |
| [Language Adapters](language-adapters.md) | Explains where language-specific differences for TypeScript, Python, and Rust are handled. |
| [Quality Checks](quality.md) | Explains structural checks, static analysis, auto-fix, testing, and environment diagnostics. |
| [Package Info Sync](package-sync.md) | Explains the mechanism for syncing `package.md` from package information. |
| [Distribution Policy](distribution.md) | Explains distribution policies via Cargo, npm, Python packages, and native executables. |
| [Troubleshooting](troubleshooting.md) | Explains common problems and how to verify them. |
| [AI Agent Integration](ai-agent-integration.md) | Explains configuration generation and extension methods for AI coding agents. |
| [Editor Integration (LSP)](editor-integration.md) | Explains real-time diagnostics, navigation, completion via LSP server, and the VSCode extension. |
| [Contributing](contributing.md) | Explains what to check when participating in development. |
| [Development Guide](development.md) | Explains procedures for environment setup, build, test, and debug. |
| [Descriptor Guide](descriptors.md) | Explains built-in and workspace descriptor directories for languages, quality tools, and package managers. |
| [LSP Development Guide](lsp-development.md) | Explains procedures for developing, debugging, and adding capabilities to mds-lsp. |
| [Roadmap](roadmap.md) | Explains the current focus area and future plans. |

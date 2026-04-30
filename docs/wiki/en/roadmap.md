# Roadmap

> *This page was translated from [Japanese](../ja/roadmap.md) by AI.*

This page explains the current core scope and future direction of mds.

## Current Core Scope

The current core scope is Markdown parsing, structure inspection, generation planning, and derived code generation.

The target languages are TypeScript, Python, and Rust.

The main scope includes the following:

- Reading `mds.config.toml`
- Detecting mds-enabled packages
- Inspecting `package.md`
- Inspecting the public surface of `index.md`
- Inspecting implementation Markdown
- Inspecting `Expose` and `Uses`
- Generation from `Types`, `Source`, `Test`
- Adding generation headers
- Generating `.mds/manifest.toml`
- Diff display with `mds build --dry-run`

## Future Enhancement Areas

The following areas will be enhanced in the future:

- Static inspection in Markdown state
- Auto-fix in Markdown state
- Test execution in Markdown state
- Runtime environment diagnosis
- Package information sync
- Initialization command
- Pre-publish quality inspection
- Distribution channel development

## Language Adapters

Language adapters for TypeScript, Python, and Rust will be developed.

Language adapters handle dependency declarations, file naming conventions, inspection tools, test execution, and language-specific additional generated artifacts.

## Distribution

mds aims for distribution accessible from multiple environments.

Target distribution channels are:

- Cargo
- npm
- Python package
- Native executables

## Quality and Safety

mds maintains the following policies for safely handling generated code:

- Does not overwrite existing files without a managed header.
- Rejects generation if the target is outside the package.
- Does not write generated output if the manifest is corrupted.
- Inspects distribution artifacts and provenance information before publishing.

## Unchanging Policies

The following policies are maintained as the core of mds:

- Markdown is treated as the source of truth.
- Generated code is treated as derived artifacts.
- Does not infer and generate implementation code from design descriptions.
- One implementation Markdown handles only one feature.
- Language-specific differences are encapsulated in language adapters.

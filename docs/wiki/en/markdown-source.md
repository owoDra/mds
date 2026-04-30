# Markdown as Source of Truth

> *This page was translated from [Japanese](../ja/markdown-source.md) by AI.*

This page explains the types and roles of Markdown documents handled by mds.

## Basic Policy

In mds, Markdown is not merely a place for descriptive text.

Markdown contains purpose, contracts, public interface, dependencies, implementation code, and test code. Language-specific files that are generated are derivatives created from Markdown.

## Document Types

The main Markdown document types handled by mds are the following three:

| Document | Role |
| --- | --- |
| `index.md` | Describes the purpose, structure, public interface, and rules of a directory or hierarchy. |
| `package.md` | Describes package information, dependencies, and package-level rules. |
| `*.ts.md`, `*.py.md`, `*.rs.md` | Records the implementation, types, and tests for a single feature. |

## `index.md`

`index.md` is the entry point of a hierarchy.

It primarily contains:

- The purpose of the hierarchy
- The structure of the hierarchy
- What is exposed externally
- Rules to follow within the hierarchy

mds reads the public interface from `index.md` and verifies the relationship between the hierarchy's design and generation targets.

## `package.md`

`package.md` is a document that describes package-level information.

It primarily contains:

- Package name and version
- Dependencies
- Development dependencies
- Package-level rules

`mds package sync` synchronizes the managed sections of `package.md` based on language-specific package information files.

## Implementation Markdown

Implementation Markdown is a document representing a single feature.

By default, implementation Markdown is placed under `src-md`.

| Filename | Target Language |
| --- | --- |
| `src-md/**/*.ts.md` | TypeScript |
| `src-md/**/*.py.md` | Python |
| `src-md/**/*.rs.md` | Rust |

## Implementation Markdown Sections

Implementation Markdown handles the following sections:

| Section | Role |
| --- | --- |
| `Purpose` | Describes the purpose of the feature. |
| `Contract` | Describes inputs, outputs, constraints, and failure conditions. |
| `Types` | Contains type definitions and type-related code. |
| `Source` | Contains implementation code. |
| `Cases` | Describes representative usage examples and expected results. |
| `Test` | Contains test code. |

`Purpose`, `Contract`, and `Cases` are descriptions for humans to confirm intent. mds does not infer implementation code from these descriptions.

The code blocks in `Types`, `Source`, and `Test` are the direct generation sources for derived code.

## Writing Dependencies

import, use, require, and similar statements are not written directly inside code blocks as a rule.

Dependencies are written in the `Uses` table. mds generates dependency declarations appropriate for the target language through language adapters.

This rule ensures that dependencies are not buried within code and remain easy to verify in the document.

## Writing Public Interface

Functions, types, modules, and other items to be exposed are written in the `Expose` table.

By expressing the public interface as a table, it becomes clear which parts of the implementation are intended to be used externally.

## Important Constraints

- One implementation Markdown handles only one feature.
- Generated code is not the source of truth.
- Instead of directly editing generated code, edit the source Markdown.
- Implementation Markdown that contains only design descriptions is not treated as a complete implementation.
- The goal is not to interpret arbitrary free-form Markdown.

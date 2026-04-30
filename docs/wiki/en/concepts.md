# Core Concepts

> *This page was translated from [Japanese](../ja/concepts.md) by AI.*

This page explains the fundamental concepts needed to understand mds.

## mds

mds is a development toolchain that treats Markdown as the source of truth for design, implementation, and testing.

mds is not a system that automatically infers and generates code from design descriptions. It reads implementation code, test code, and metadata written within Markdown and generates language-specific files.

## Source of Truth

The source of truth is the primary information referenced by both humans and tools.

In mds, implementation Markdown is the source of truth. Generated `.ts`, `.py`, `.rs`, and other files are not the source of truth — they are derivatives created from Markdown.

This approach allows design, dependencies, public surface, implementation, and tests to be reviewed in the same place.

## Derived Code

Derived code refers to files generated from Markdown.

Derived code is not meant to be edited directly. If you want to make changes, modify the source Markdown or the generation rules.

mds adds a header to managed generated files. By not overwriting existing files that lack a management header, it protects files written by the user.

## Implementation Markdown

Implementation Markdown is a Markdown file representing a single feature.

File names include the target language.

| File Name Example | Target Language |
| --- | --- |
| `foo.ts.md` | TypeScript |
| `foo.py.md` | Python |
| `foo.rs.md` | Rust |

Each implementation Markdown is responsible for exactly one feature. Mixing multiple features in a single file makes it difficult to understand the purpose, dependencies, and test correspondence.

## `Types`

`Types` is the section for writing types, interfaces, and type definition code.

In TypeScript it is treated as a type definition file, in Python as a stub file, and in Rust as a types file. The specific output rules differ by language.

## `Source`

`Source` is the section for writing implementation code.

mds reads the code blocks in this section and generates regular source files for the target language.

## `Test`

`Test` is the section for writing test code.

mds reads the code blocks in this section and generates test files for the target language.

## `Expose`

`Expose` is metadata representing what an implementation Markdown or hierarchy exposes to the outside.

By writing the functions, types, and modules to be published in a table, it allows users and tools to verify the public surface.

## `Uses`

`Uses` is metadata representing the dependencies used by `Types`, `Source`, and `Test`.

By extracting dependency declarations such as import, use, and require outside of code blocks, language adapters can generate dependency declarations appropriate for the target language.

## Language Adapters

Language adapters are components responsible for language-specific differences.

They mainly handle the following.

- Generating import and use statements
- Determining generated file names
- Connecting to checking tools and test execution
- Managing language-specific additional generated artifacts

The core processing of mds handles Markdown reading, structural inspection, and generation planning. Language-specific differences are contained within language adapters as much as possible.

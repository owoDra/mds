# Monorepo Usage

> *This page was translated from [Japanese](../ja/monorepo.md) by AI.*

This page explains how mds fits repositories with multiple packages.

## Basic Policy

mds enables packages one package at a time. A repository can mix mds packages and non-mds packages safely.

## How Packages Are Detected

mds looks for package roots that contain:

- `mds.config.toml`
- `package.md`
- recognized package-manager metadata such as `package.json`, `pyproject.toml`, or `Cargo.toml`

Non-target packages are not rewritten.

## Per-Package Authoring Layout

Each package keeps its own canonical authoring roots and output roots.

```text
package-a/
├── mds.config.toml
├── package.md
├── .mds/source/
├── .mds/test/
├── src/
└── tests/
```

## Multiple Languages

Different packages can use different language suffixes and different `[quality.<lang>]` sections. The package boundary, output planning, and managed-file safety rules stay the same.

## Recommended Practices

- keep one `mds.config.toml` per package
- keep outputs inside that package
- use `.mds/source` and `.mds/test` everywhere for authoring roots
- verify a new package with `mds lint` and `mds build --dry-run` before the first write
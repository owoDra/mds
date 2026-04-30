# Distribution Policy

> *This page was translated from [Japanese](../ja/distribution.md) by AI.*

This page explains the distribution policy of mds.

## Basic Policy

mds aims to be usable from multiple environments, centered on core processing and commands implemented in Rust.

To allow installation according to the user's language environment, distribution via Cargo, npm, Python packages, and native executables is targeted.

## Distribution Channels

| Distribution Channel | Role |
| --- | --- |
| Cargo | A channel for installing mds from a Rust environment. |
| npm | A channel for invoking mds from a Node.js environment. |
| Python package | A channel for invoking mds from a Python environment. |
| Native executable | A channel for running the mds command without depending on a language environment. |

## Core Processing

Core processing is implemented in Rust.

Rust handles Markdown parsing, configuration resolution, package discovery, generation planning, file generation, diagnostics, and more.

## Usage from npm

The npm package is treated as an entry point for invoking native executables.

Rather than changing mds semantics on the npm side, the same mds commands are made callable from a Node.js environment.

## Usage from Python

The Python package is also treated as an entry point for invoking native executables.

Rather than creating different semantics on the Python side, the same mds commands are made callable from a Python environment.

## Pre-release Quality Verification

Before publication, the quality of distribution artifacts is verified.

Verification targets include:

- Existence of distribution artifacts
- Checksums
- Signatures
- Software bill of materials
- Provenance information
- Basic post-install operation verification
- Ability to launch native executables from wrapper packages

These verifications are to prevent distributing artifacts that cannot be installed after publication.

## Current Status

This distribution policy is a target for implementation and release preparation.

Available installation methods will change as releases progress. To try it reliably at this point, run it as a Rust command from the repository.

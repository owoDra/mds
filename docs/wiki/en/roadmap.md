# Roadmap

> *This page was translated from [Japanese](../ja/roadmap.md) by AI.*

This page summarizes the current focus of mds.

## Current Focus

- authoring-v2 with canonical `.mds/source` and `.mds/test` roots
- package output planning through `[roots]`, `[output]`, and `[[output.override]]`
- source-map-backed generated-file bridge for editor features
- current `init`, `new`, examples, and AI kit templates
- structural diagnostics and selected toolchain execution

## Near-Term Follow-Up

- continue tightening live docs, examples, and templates around authoring-v2
- broaden package-level validation and editor ergonomics
- extend output patterns and quality integration where packages need them
- improve release, distribution, and onboarding polish

## Stable Policies

- Markdown remains the source of truth.
- Generated files remain derived artifacts.
- Source docs and test docs stay split by responsibility.
- Output safety wins over convenience when the two conflict.
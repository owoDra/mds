# vscode

## Purpose

VS Code extension source and editor integration surface for mds.

## Architecture

The existing extension source remains under `editors/vscode/src` until the TypeScript self-hosting phase. Package metadata is read from `../package.json`; mds does not use a package root `index.md`. Generated extension outputs are redirected to `.build/node/vscode`.

## Exposes

| Kind | Name | Target | Summary |
| --- | --- | --- | --- |
| module | owo-x-project.mds | .. | VS Code extension package. |

## Rules

- Do not write generated JavaScript to the repository root; use `.build/node/vscode`.

# vscode

## Purpose

VS Code extension source and editor integration surface for mds.

## Architecture

The existing extension source remains under `editors/vscode/src` until the TypeScript self-hosting phase. Package metadata is read from `../package.json`; mds does not use a package root `index.md`. Generated extension outputs are redirected to `.build/node/vscode`.

### Package Summary

| Name | Version |
| --- | --- |
| mds | 0.1.0 |

### Dependencies

| Name | Version | Summary |
| --- | --- | --- |
| vscode-languageclient | ^9.0.1 |  |

### Dev Dependencies

| Name | Version | Summary |
| --- | --- | --- |
| @types/node | ^20.11.0 |  |
| @types/vscode | ^1.85.0 |  |
| typescript | ^5.3.3 |  |

## Exposes

| Kind | Name | Target | Summary |
| --- | --- | --- | --- |
| module | owo-x-project.mds | .. | VS Code extension package. |

## Rules

- Do not write generated JavaScript to the repository root; use `.build/node/vscode`.
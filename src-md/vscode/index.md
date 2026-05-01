# vscode

## Purpose

VS Code extension source and editor integration surface for mds.

## Architecture

The existing extension source remains under `editors/vscode` until the TypeScript self-hosting phase. Generated extension outputs are redirected to `.build/node/vscode`.

## Exposes

| Kind | Name | Target | Summary |
| --- | --- | --- | --- |
| extension | owo-x-project.mds | editors/vscode | VS Code extension package. |

## Rules

- Do not write generated JavaScript to the repository root; use `.build/node/vscode`.

---
id: SPEC-adapter-typescript-generation
status: 採用
related:
  - docs/project/requirements/REQ-adapter-required-language-adapters.md
  - docs/project/requirements/REQ-generation-code-output-rules.md
  - docs/project/specs/shared/SPEC-code-generation-output.md
---

# TypeScript Adapter 生成

## 概要

`packages/lang-ts` は TypeScript の生成 file pattern と import 生成を担う。

## 関連要求

- `../../requirements/REQ-adapter-required-language-adapters.md`
- `../../requirements/REQ-generation-code-output-rules.md`

## 入力

- `.ts.md` implementation md
- `Types`、`Source`、`Test` のコードブロック
- `Uses` テーブル
- 解決済みの `markdown_root`、`source_root`、`types_root`、`test_root`

## 出力

- TypeScript Source ファイル
- TypeScript Types ファイル
- TypeScript Test ファイル
- TypeScript import 文

## 挙動

- 既定 pattern は、`src-md/foo/bar.ts.md` から Source `src/foo/bar.ts`、Types `src/foo/bar.types.ts`、Test `tests/foo/bar.test.ts` を生成する。
- `Uses` の `Types` 依存は TypeScript の type-only import として生成できる。
- `Uses` の `Source` と `Test` 依存は通常 import として生成する。
- `Uses.Expose` が空の場合は module import / side-effect import 相当として扱う。
- `Uses.Expose` の `default: Foo`、`A as B`、`* as ns` を TypeScript import へ変換する。
- Markdown 状態の quality 操作では Prettier、ESLint、Vitest へ一時 TypeScript code を渡す。

## 状態遷移 / 不変条件

- TypeScript 固有の file pattern は core の Markdown model を変更しない。
- 型を `src/**/*.types.ts` に出す既定は adapter の既定であり、config で root を上書きできる。

## エラー / 例外

- `.ts.md` 以外の implementation md を TypeScript adapter の生成対象にしない。
- TypeScript import に変換できない `Uses` は adapter 診断にする。
- TypeScript toolchain が不足する場合は environment 不足診断にする。

## 横断ルール

- shared spec の生成 lifecycle、manifest、header、上書き規則に従う。
- shared spec の `Expose` / `Uses` canonical schema を変更しない。

## 検証観点

- `src-md/foo/bar.ts.md` から `src/foo/bar.ts`、`src/foo/bar.types.ts`、`tests/foo/bar.test.ts` が導出できることを確認する。
- `Types` の type-only import と `Source` / `Test` の通常 import を fixture で確認する。
- default / alias / namespace import と Prettier / ESLint / Vitest 接続を fixture で確認する。

## 関連資料

- `../shared/SPEC-code-generation-output.md`
- `../shared/SPEC-expose-uses-tables.md`
- `../shared/SPEC-md-state-quality-operations.md`
- `../../patterns/impl-adapter-boundary.md`

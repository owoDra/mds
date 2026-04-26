# TypeScript Implementation Team Guide

## 役割

TypeScript implementation team は、npm 配布、TypeScript language adapter、TypeScript 生成規則、Node.js 向け CLI wrapper の実装品質を保つ責任を持つ。

## 担当範囲

- `packages/core`: npm から利用する core API 境界。
- `packages/cli`: Node.js / npm 向け CLI entrypoint。
- `packages/lang-ts`: TypeScript の file pattern、import 生成、型 import、test file 生成。
- TypeScript fixture、Vitest / ESLint / Prettier 導線。

## ルール

- npm workspace は `packages/package.json` を入口にし、pnpm は導入しない。
- TypeScript 固有の import 形式、拡張子有無、type-only import は `packages/lang-ts` に閉じ込める。
- core の Markdown model、`Expose` / `Uses` schema、config の意味を TypeScript 側で変更しない。
- package の public API は `src/index.ts` から明示的に export し、内部 module を barrel export で無制限に公開しない。
- test は production source から分離し、unit test は repository の test convention に合わせた `tests/` または `*.test.ts` に置く。
- generated fixture の期待値は、import 順、type-only import、拡張子なし相対 import、header、末尾 LF まで固定する。
- TypeScript 実装変更では `npm` test / lint / format 導線が存在する場合に必ず実行する。導線がない場合は未実施理由を task に残す。

## 固有知識

- Parser + 生成 MVP の TypeScript 既定 pattern は、`src-md/foo/bar.ts.md` から Source `src/foo/bar.ts`、Types `src/foo/bar.types.ts`、Test `tests/foo/bar.test.ts` を生成する。
- `Types` の `Uses` は type-only import にできる。
- `Source` / `Test` の `Uses` は通常 import として生成する。
- `internal` の相対 import は拡張子なしにする。
- default import と alias は MVP 外であり、実装前に spec を追加する。

## 関連資料

- `../architecture.md`
- `../patterns/impl-adapter-boundary.md`
- `../specs/packages-lang-ts/SPEC-adapter-typescript-generation.md`
- `../specs/shared/SPEC-parser-generation-mvp-phase.md`
- `../validation.md`

# TypeScript Implementation Team Guide

## 役割

TypeScript implementation team は、VS Code extension と TypeScript 生成規則の実装品質を保つ責任を持つ。

## 担当範囲

- `editors/vscode`: VS Code extension の TypeScript 実装。
- TypeScript の file pattern、import 生成、型 import、test file 生成仕様。
- TypeScript fixture、Vitest / ESLint / Prettier 導線。

## ルール

- TypeScript 固有の import 形式、拡張子有無、type-only import は TypeScript adapter 境界に閉じ込める。
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
- `../specs/shared/SPEC-adapter-typescript-generation.md`
- `../specs/shared/SPEC-parser-generation-mvp-phase.md`
- `../validation.md`

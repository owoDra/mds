# Phase 09: docs / init / examples / snippets 更新

## 前提

- new core behavior、package output config、source map、diagnostics が実装済みまたは仕様固定済みである。
- migration は対象外である。
- descriptor-driven language adapter cleanup の範囲が決まっている。
- self-hosting 停止後の repository development flow が docs に反映できる状態である。

## やること

- `mds init` templates を tableless authoring style へ更新する。
- `mds new` templates を tableless authoring style へ更新する。
- AI agent kit templates から `Imports`、`Exports`、`Types` tables を外す。
- VS Code snippets と completion snippets を tableless source/test sections へ更新する。
- docs build で `[[module]]` / `[[module#symbol]]` を通常 Markdown links へ変換する。
- `examples/minimal-ts` の duplicated test suffix、例: `greet.test.test.ts` を解消する。
- minimal Rust / Python examples を tableless style へ寄せる。
- `examples/descriptor-samples/` と `examples/descriptor-catalog/` を削除または新方針に合わせて縮小する。
- README と wiki から `src-md`、`Types`、self-hosted build、descriptor-driven language adapter claims を削除または置換する。

## 完了条件

- new project initialization が tableless authoring documents を生成する。
- VS Code snippets が old tables や `Types` section を提案しない。
- examples が new authoring model と package output config を示している。
- docs が migration や first-party self-hosted development を約束していない。
- descriptor docs は削除済み、または package managers / quality tool manifests に必要な範囲へ縮小されている。

## 注意事項

- docs / snippets は実装済み挙動に合わせる。先に理想だけを書きすぎない。
- README、wiki、VS Code README、examples は利用者が最初に見るため、old model の断片が残ると混乱する。
- descriptor docs を完全削除する場合、package manager / quality tool manifest の説明まで失わないように分離する。
- examples は migration fixture ではない。破壊的変更後の canonical examples として最小に保つ。

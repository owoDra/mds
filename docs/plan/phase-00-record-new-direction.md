# Phase 00: 方針の記録

## 前提

- この repository は mds self-hosting を停止する。
- migration は明示的に対象外とする。
- 既存の proposal、architecture、validation、agent instructions には first-party package migration や self-hosted Markdown source の前提が残っている。
- この phase では実装の大規模削除には入らず、後続 phase が矛盾なく進められる正本整理を優先する。

## やること

- `docs/project/architecture.md` から、mds 自身を `mds/**/.mds/source` と `mds/**/.mds/test` から rebuild する不変条件を外す。
- `docs/project/validation.md` を更新し、この repository の通常検証を `cargo fmt --all --check`、`cargo check --workspace`、`cargo test --workspace`、`cargo clippy --workspace --all-targets`、`npm run compile` in `editors/vscode` へ寄せる。
- `docs/project/proposals/active/proposal-markdown-authoring-layout-v2.md` に、breaking alpha change として migration command を作らない方針を反映する。
- `.agents/project.md` を更新し、first-party code の変更・検証入口を `mds build` 前提にしない。
- old self-hosted build、descriptor-driven adapter、first-party `.mds` 正本の記述を historical / superseded / cleanup target として整理する。
- `docs/plan/index.md` と phase files を今後の実装判断の参照導線にする。

## 完了条件

- contributor と AI agent が first-party implementation を `.mds/source` や `.mds/test` から編集すべきだと読める記述が主導線から消えている。
- validation policy が通常 Rust / TypeScript project としての直接検証を標準にしている。
- active proposal と plan が migration なしで一致している。
- 後続 phase で generated Rust files を直接編集しても方針違反にならない。

## 注意事項

- この phase では `.mds` directory をまだ消さない。消す前に Phase 01 で generated code を canonical source として固定する。
- `mds build` は product fixture の検証には残り得るが、この repository 自身の source regeneration には使わない。
- docs の語彙変更だけで実装が変わったように見せない。実装削除は Phase 01 以降で扱う。
- archive の全面更新は不要。active な正本と agent 導線を優先する。

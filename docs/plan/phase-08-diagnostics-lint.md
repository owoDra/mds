# Phase 08: diagnostics / lint 厳格化

## 前提

- core は logical module ID と wiki-link を language-specific parser なしで解決できる。
- source map が diagnostics range remap に使える。
- migration support は意図的に提供しない。
- tableless source/test Markdown を標準 authoring model とする。

## やること

- unresolved `[[module]]` を error にする。
- unresolved `[[module#symbol]]` を `warning`、`error`、`allow` で configurable にする。
- test md の `## 対象` / `## Covers` 未解決を error にする。
- source md 内 test code 混在と test md 内 source code 混在を診断する。
- `legacy_tables = "warn" | "error" | "allow"` を追加する。
- `implementation_section_only` を追加し、実行対象 code を canonical implementation section に限定できるようにする。
- `split_source_and_test` を追加し、source/test 混在を拒否できるようにする。
- `Types` acceptance と `Types` migration を示唆する diagnostics を削除する。

## 完了条件

- tableless source/test md が標準形として lint/build で扱われる。
- legacy tables は標準ではなく configurable diagnostics の対象である。
- source/test mixing が一貫して検出される。
- wiki-link diagnostics が language-specific parser に依存しない。
- core と LSP diagnostics の代表 tests が揃っている。

## 注意事項

- `legacy_tables` は migration 補助ではなく、breaking change 後の policy control として設計する。
- `[[module#symbol]]` は外部 LSP bridge の完成度により誤検出リスクがあるため、configurable severity を維持する。
- source/test code mixing の判定は language-specific AST なしで行うため、doc kind、section、output kind を基準にする。
- old quick actions が legacy table rename を標準導線として見せる場合は削除または legacy-only に落とす。

# Phase 03: migration の対象外化

## 前提

- authoring-v2 は alpha 段階の破壊的変更として扱う。
- first-party Markdown を自動移行する必要はない。
- user-facing docs や CLI に migration 提供の約束が残っている可能性がある。

## やること

- `mds migrate authoring-v2` command を実装しない方針を CLI docs と plan に明記する。
- authoring-v2 migration の dry-run diff、fixture、command spec を実装 scope から削除する。
- old `Imports` / `Exports` / `Uses` tables の自動変換計画を削除する。
- unsupported legacy authoring pattern は migration suggestion ではなく direct diagnostics として扱う。
- alpha users は手動で新 authoring model に合わせる必要があることを docs に明記する。

## 完了条件

- CLI command list に `migrate authoring-v2` がない。
- tests が migration dry-run diff を期待しない。
- docs が automatic table conversion を約束しない。
- legacy authoring pattern は migration path ではなく unsupported / legacy diagnostics として扱われる。

## 注意事項

- `legacy_tables` の diagnostics は migration ではない。新 model への拒否・警告・許容の policy control として扱う。
- `mds migrate` という別目的 command が将来必要になる可能性は否定しないが、この計画では作らない。
- user が破壊的変更を受け入れる前提なので、互換維持のための複雑な fallback を増やさない。
- docs では「migration なし」を曖昧にせず、alpha breaking change として書く。

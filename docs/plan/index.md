# Plans

## Self-hosting Removal and Authoring V2

mds 自身の self-hosting を停止し、Readable Authoring Model、core 言語非依存、source map、package output config、外部 LSP 委譲へ破壊的に移行する計画です。

この計画では `mds migrate authoring-v2` は作りません。alpha 段階の破壊的変更として、first-party Markdown の自動移行ではなく、不要な mds 管理資産を削除して通常の Rust / TypeScript source を直接正本にします。

## Phase 00 での判断基準

- self-hosting removal / authoring-v2 に関する Phase 00 の実装判断は、この index と対象 phase file を一次資料として扱う。
- `docs/project/**` や active proposal に旧 self-hosting 前提が残っている場合は、Phase 00 で明示更新されるまでこの plan を優先する。

## Phase Files

- [Phase 00: 方針の記録](phase-00-record-new-direction.md)
- [Phase 01: 生成済みコードの正本化](phase-01-freeze-generated-code.md)
- [Phase 02: first-party mds 管理資産の削除](phase-02-remove-first-party-mds-assets.md)
- [Phase 03: migration の対象外化](phase-03-remove-migration-scope.md)
- [Phase 04: core 言語非依存化](phase-04-core-language-independent.md)
- [Phase 05: source map 基盤](phase-05-source-map.md)
- [Phase 06: package output config](phase-06-package-output-config.md)
- [Phase 07: LSP bridge 準備](phase-07-lsp-bridge.md)
- [Phase 08: diagnostics / lint 厳格化](phase-08-diagnostics-lint.md)
- [Phase 09: docs / init / examples / snippets 更新](phase-09-docs-init-examples.md)

## 全体前提

- この repository は `mds/core/.mds`、`mds/cli/.mds`、`mds/lsp/.mds` を source of truth として扱うことをやめる。
- `mds/**/src` と `mds/**/tests` を通常の手編集対象にする。
- first-party package の移行 command は作らない。
- `src-md`、`Types`、`Imports` table、`Exports` table、descriptor-driven language adapter との長期互換は目標にしない。
- mds product として user package の Markdown authoring は扱うが、この repository 自身は mds-generated self-hosted code に依存しない。

## 全体完了条件

- first-party Rust / TypeScript source が直接編集・直接検証できる。
- authoring-v2 migration command や migration fixture が存在しない。
- core build が language-specific import/export extraction に依存しない。
- source map が build、diagnostics、docs、LSP の共通 remap layer になる。
- output path が descriptor ではなく package config pattern で決まる。
- tableless source/test Markdown が product の標準 authoring model になる。
- old descriptor-driven language adapter examples、docs、snippets、self-hosted artifacts が削除または明確に再分類される。

## 推奨順序

- Phase 00 と Phase 01 を先に行い、今後の code edit が self-hosted regeneration で上書きされない状態にする。
- Phase 02 で重複正本と stale generated assets を減らしてから core を大きく変える。
- Phase 03 で migration を明確に対象外にしてから CLI を変更する。
- Phase 04 と Phase 06 は descriptor 依存を同時に外すため、近いタイミングで実施する。
- Phase 05 は Phase 07 の前に行う。
- Phase 08 は Phase 04 から Phase 06 の後に行う。
- Phase 09 は実装済みの挙動に docs / init / examples / snippets を合わせるため最後に行う。

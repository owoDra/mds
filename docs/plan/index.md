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

## 進捗メモ

- 2026-05-13: Phase 00 を完了し、first-party self-hosting は historical / superseded / cleanup target として扱う方針を architecture、validation、agent guidance、active proposal、team guide、ADR / proposal archive 導線へ反映した。
- 2026-05-13: Phase 01 を完了し、first-party checked-in Rust source / tests を canonical source/test として固定した。generated header を除去し、repo-facing docs / scripts / release workflow を direct source tree と root Cargo workspace 前提へ更新した。`mds/core/src/runner.rs` の self-hosted mirror 同期は削除済みで、`cargo check --workspace` と `cargo test --workspace` が通る状態を確認した。
- 2026-05-14: Phase 02 を完了し、first-party `.mds` 資産と self-hosted artifact を削除した。live docs / requirements / patterns / skills / release manifest comment の stale 参照を checked-in source / docs に張り替え、core fixture test で self-hosted mirror 非依存を確認した。
- 2026-05-14: Phase 03 を完了し、migration promise を live surface から除去した。LSP の legacy rename quick fix を削除し、proposal / diagnostics wording を direct diagnostics framing に揃えたうえで、ユーザーフィードバックに従い `Types` の semantic acceptance を build / config / init / template / LSP から前倒しで除去した。
- 2026-05-14: Phase 04 を完了し、core build / quality から synthetic import 注入と language-specific syntax lint を除去した。core extractor API と LSP の extractor 依存を外し、`Lang` を built-in variant のない opaque extension key 前提へ縮小した。
- 2026-05-14: Phase 05 を完了し、parser/model に code fence span 保持を追加した。build planning に `SourceMap` / `SourceSpan` / `GenerationPlan` を導入し、initial mapping を code fence 由来 source/test output に限定したうえで、core quality の path/line remap を SourceMap lookup へ差し替えた。`cargo test -p mds-core --test parser_generation_mvp_test` が 72 tests 成功した。
- 2026-05-16: Phase 06 を完了し、package output config を core config/model と output planning へ導入した。`[roots]` は `source_md` / `test_md` / `source_out` / `test_out` へ切り替え、`[output]` と `[[output.override]]` を追加した。output path は package config pattern と override から決定し、`build.rs` 相当の special placement も descriptor `special_files` ではなく config override で表せるようにした。`src-md` fallback は live runtime surface から除去し、`cargo test -p mds-core --test parser_generation_mvp_test`、`cargo test -p mds-lsp --test diagnostics_test`、`cd editors/vscode && npm run compile` が成功した。
- 2026-05-16: Phase 07 を完了し、mds-lsp に source map-backed bridge API と `executeCommand` surface を追加した。VS Code extension は generated-file mode を優先して hover / definition を各言語 provider へ委譲し、generated diagnostics を全 indexed Markdown 文書へ mirror するようにした。`docs/plan/phase-07-lsp-bridge.md` には bridge 判断と `mds-virtual:` URI design を追記し、`cargo test -p mds-lsp` と `cd editors/vscode && npm run compile` が成功した。
- 2026-05-16: Phase 08 を完了し、authoring-v2 diagnostics policy を core と LSP に接続した。`[check]` に `legacy_tables`、`unresolved_module_symbols`、`implementation_section_only`、`split_source_and_test` を追加し、core markdown load は canonical section only と source/test mixing rejection を正とするようにした。package-level wiki-link validation は `[[module]]` を error、`[[module#symbol]]` を policy configurable にし、LSP diagnostics / code action も tableless authoring-v2 標準へ追随した。最終再確認では `parser_generation_mvp_test` 84 pass、`diagnostics_test` 15 pass、`capabilities_test` 24 pass を確認した。
- 2026-05-16: Phase 09 を完了し、`mds init` / `mds new` templates、AI agent kit templates、VS Code snippets / README、LSP completion、minimal examples、top-level README / wiki を authoring-v2 へ揃えた。descriptor example directories と stale live wiki pages を削除し、docs build による wiki-link 変換は既存 surface が無いため follow-up として明記した。Phase 完了過程で `mds/core/src/adapter.rs` の Broken pipe race も修正し、`cargo test -p mds-core --test parser_generation_mvp_test` が default 並列で 84 pass になることを確認した。
- 2026-05-16: 追加 cleanup として first-party package-local `mds.config.toml` 3 件と empty な `mds/{core,cli,lsp}/.build/` を削除した。examples の `mds.config.toml` は product examples 用として保持し、current CLI で `mds lint --package examples/minimal-{ts,py,rs}` を再実行して syntax を確認した。
- 全フェーズ完了。再開時は plan ではなく residual follow-up の有無を確認する。

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

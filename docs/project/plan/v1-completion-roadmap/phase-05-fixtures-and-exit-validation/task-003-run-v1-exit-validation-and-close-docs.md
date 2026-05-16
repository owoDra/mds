# Task 003: Run V1 Exit Validation And Close Docs

## 目的

v1 完了判定に必要な build / test / compile / example command / docs 参照整合をまとめて確認し、残る docs 差分を閉じる。

## 前提条件

- `minimal-ts` と broken/remap fixture が揃っている
- core、CLI、LSP、VS Code の主要 spec 差分が解消されている

## 作業内容

- Rust workspace build / test、VS Code compile、examples command、package sync、diagnostic remap 確認を実施する
- `validation.md`、plan、spec、examples README の参照整合と完了状態を見直す
- v1 完了後に残るものを bug / polish / v2 項目へ切り分ける

## 完了条件

- v1 完了に必要な代表確認が一通り実施される
- docs 上の roadmap / validation / examples 参照が現実装と一致する
- 未完了事項が残る場合も、v1 blocking か post-v1 かを判定できる

## 検証方法

- `cargo test` と必要な `cargo build`
- `npm run compile` in `editors/vscode`
- `examples/minimal-ts` と broken/remap fixture を使う `mds` command 確認

## 依存関係

- `task-001-align-minimal-ts-fixture.md`
- `task-002-add-broken-remap-fixture-and-live-regressions.md`

## 成果物

- `docs/project/validation.md`
- `docs/project/plan/v1-completion-roadmap/`
- `docs/project/specs/examples/`
- `examples/README.md`

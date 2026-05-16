# V1 Completion Roadmap

## 目的

現行実装と v1 spec の差分を埋め、`mds-core` `mds-cli` `mds-lsp` `vscode-extension` `examples` を v1 完成状態へ到達させる。

## スコープ

- `mds-core` の config/schema runtime、link policy、package sync、doctor、quality policy
- `mds-cli` の init/new workflow、command surface 検証、CLI テスト
- `mds-lsp` の workspace index、navigation、guided authoring、bridge command surface
- VS Code extension の active context 表示、embedded provider bridge、diagnostic mirror
- `examples/` の fixture 整備、live regression、最終 validation と docs closure

## 非スコープ

- `requirements/v2/` に属する project-wide document governance
- VS Code 以外の editor 実装追加
- v1 spec 対象外の Python / Rust example 復活

## 前提

- v1 requirement / spec は `docs/project/requirements/v1/` と `docs/project/specs/` を正本とする
- `capability-schema-migration` は v1 完成の前提 plan として継続する
- 現状 `mds-core` は package discovery、Markdown validation、safe generation、manifest、source map、diagnostic remap の基盤を持つ

## 現状要約

- `mds-core` は生成 safety と Markdown 検証の土台を持つが、link policy、config/schema runtime、doctor/package sync policy が spec 未達である
- `mds-cli` は command surface 自体は揃っているが、wizard screen flow、`mds new <path> <kind>` 契約、CLI 実運用テストが未完了である
- `mds-lsp` は diagnostics / completion / remap を持つが、`overview.md` / test doc index、refresh trigger、structured navigation 優先が未完了である
- VS Code extension は completion / hover / definition bridge を持つが、status bar、references / rename / code action / formatting bridge、embedded diagnostic mirror が未完了である
- `examples/minimal-ts` は骨格だけ整っており、quality scripts、managed region、live regression、broken/remap fixture が未完了である

## 完了定義

- v1 spec 群で定義した必須挙動が各 subproject と fixture で観測できる
- `examples/minimal-ts` と分離された broken/remap fixture を使う回帰確認が通る
- v1 完了に必要な差分が大きい設計メモではなく、通常の bug / polish 単位まで縮小される

## 統合実行順

1. `../capability-schema-migration/phase-01-architecture-and-schema/index.md`: config/schema runtime の architecture と最小 schema surface を確定する
2. `../capability-schema-migration/phase-02-core-runtime-migration/index.md`: `mds-core` の language / quality / remap runtime を config/schema へ移す
3. `phase-01-core-runtime-and-authoring-policy/index.md`: shared/core spec と core policy 差分を埋め切る
4. 以下は `phase-01-core-runtime-and-authoring-policy` 完了後に並行着手できる
   - `phase-02-cli-authoring-entrypoints/index.md`: CLI init/new と command surface を v1 仕様へ揃える
     - `../capability-schema-migration/phase-03-cli-and-editor-adoption/task-001-update-init-and-new-for-schema-runtime.md`: schema runtime 追従をこの phase に内包して進める
   - `phase-03-lsp-workspace-and-navigation/index.md`: LSP の index、navigation、bridge command surface を v1 仕様へ揃える
     - `../capability-schema-migration/phase-03-cli-and-editor-adoption/task-002-update-lsp-and-vscode-language-discovery.md`: language discovery 追従のうち LSP 側をこの phase に内包して進める
5. `phase-04-vscode-embedded-experience/index.md`: VS Code bridge UX を v1 仕様へ揃える
   - `../capability-schema-migration/phase-03-cli-and-editor-adoption/task-002-update-lsp-and-vscode-language-discovery.md`: language discovery 追従のうち VS Code 側をこの phase に内包して進める
6. 以下は `phase-04-vscode-embedded-experience` 着地後に並行着手できる
   - `phase-05-fixtures-and-exit-validation/task-001-align-minimal-ts-fixture.md`: success-path fixture を完成させる
   - `phase-05-fixtures-and-exit-validation/task-002-add-broken-remap-fixture-and-live-regressions.md`: broken/remap fixture と live regression を追加する
   - `../capability-schema-migration/phase-04-cleanup-and-validation/task-001-remove-obsolete-builtins-and-align-examples.md`: obsolete built-in 整理と examples 整合をこの段階で進める
7. `phase-05-fixtures-and-exit-validation/task-003-run-v1-exit-validation-and-close-docs.md`: 最終 validation と docs closure を行う
   - `../capability-schema-migration/phase-04-cleanup-and-validation/task-002-run-regression-and-close-doc-gaps.md`: migration 側の回帰確認と doc gap closure をこの最終 task に内包して進める

## フェーズ一覧

1. `phase-01-core-runtime-and-authoring-policy/index.md`: shared/core spec と runtime policy を一致させる
2. 以下は `phase-01` 完了後に並行着手できる
   - `phase-02-cli-authoring-entrypoints/index.md`: init/new と CLI 検証面を v1 仕様へ揃える
   - `phase-03-lsp-workspace-and-navigation/index.md`: LSP の index、navigation、bridge command surface を v1 仕様へ揃える
3. `phase-04-vscode-embedded-experience/index.md`: VS Code bridge UX を v1 仕様へ揃える
4. `phase-05-fixtures-and-exit-validation/index.md`: examples、回帰確認、docs closure で v1 完了を検証する

## 依存関係

- `../capability-schema-migration/index.md`
- `../../specs/shared/SPEC-model-package-layout.md`
- `../../specs/shared/SPEC-authoring-markdown-format.md`
- `../../specs/shared/SPEC-generation-safety-and-derivation.md`
- `../../specs/shared/SPEC-language-extension-contract.md`
- `../../specs/shared/SPEC-ux-embedded-language-bridge.md`
- `../../specs/shared/SPEC-ux-navigation-and-traceability.md`
- `../../specs/mds-core/SPEC-core-config-and-authoring-policy.md`
- `../../specs/mds-core/SPEC-core-overview-and-package-sync.md`
- `../../specs/mds-core/SPEC-core-quality-and-fix-pipeline.md`
- `../../specs/mds-cli/SPEC-cli-command-surface-and-execution.md`
- `../../specs/mds-cli/SPEC-cli-init-and-new-workflows.md`
- `../../specs/mds-cli/SPEC-cli-init-wizard-screen-flow.md`
- `../../specs/mds-cli/SPEC-cli-doctor-and-update.md`
- `../../specs/mds-lsp/SPEC-lsp-authoring-navigation-remap.md`
- `../../specs/vscode-extension/SPEC-vscode-embedded-editor-experience.md`
- `../../specs/examples/SPEC-examples-v1-regression-fixtures.md`
- `../../specs/examples/SPEC-examples-minimal-ts-fixture.md`
- `../../validation.md`

## 検証方針

- 各 phase 完了時に spec と実装の差分が 1 つ以上明確に閉じていることを確認する
- CLI / LSP / VS Code は個別確認で終わらせず、`examples/minimal-ts` と broken/remap fixture で横断確認する
- 最終 phase では build / test / compile / example command と docs 参照整合をまとめて確認する

## 参照

1. `../capability-schema-migration/index.md`: config/schema runtime 移行 plan。本 roadmap の先行 / 内包 task を持つ
2. `phase-01-core-runtime-and-authoring-policy/index.md`: shared/core spec と runtime policy の一致
3. 以下は `phase-01` 完了後に並行着手できる
   - `phase-02-cli-authoring-entrypoints/index.md`: CLI authoring entrypoint の完成
   - `phase-03-lsp-workspace-and-navigation/index.md`: LSP workspace/navigation の完成
4. `phase-04-vscode-embedded-experience/index.md`: VS Code embedded editor UX の完成
5. `phase-05-fixtures-and-exit-validation/index.md`: fixture / regression / docs closure

# Capability Schema Migration

## 目的

built-in descriptor / tool registry 依存を package config と外部 capability schema 中心の runtime へ置き換え、spec と実装を整合させる。

## スコープ

- language identity 判定の config/schema 化
- special file / output / root module rule の config/schema 化
- quality slot command / capture rule の config/schema 化
- CLI wizard / init / new の入力面見直し
- LSP / VS Code の language discovery 契約更新

## 非スコープ

- v2 全体資料管理の実装
- すべての外部 tool を無変換で高精度統合すること
- editor 追加実装

## 前提

- source map と diagnostic remap は引き続き kernel で維持する
- impl md の file suffix と fence label で language identity を判定する
- built-in registry は段階的に縮小 / 除去する

## 完了定義

- built-in descriptor / tool registry 依存を置き換える config/schema runtime が定義・実装される
- wizard、core、LSP、VS Code の挙動が新 spec と整合する
- examples と validation が新 runtime を前提に更新される

## フェーズ一覧

- `phase-01-architecture-and-schema/index.md`: architecture と schema 契約を確定する
- `phase-02-core-runtime-migration/index.md`: mds-core を config/schema runtime へ移行する
- `phase-03-cli-and-editor-adoption/index.md`: CLI / LSP / VS Code を新 runtime へ追従させる
- `phase-04-cleanup-and-validation/index.md`: 旧 registry を整理し、examples と検証を揃える

## 依存関係

- `../../proposals/active/proposal-capability-schema-runtime.md`
- `../../architecture.md`
- `../../specs/shared/SPEC-language-extension-contract.md`
- `../../specs/mds-core/SPEC-core-config-and-authoring-policy.md`
- `../../specs/mds-core/SPEC-core-quality-and-fix-pipeline.md`

## 検証方針

- 各 phase で spec と実装の責務境界が前進していることを確認する
- diagnostic remap が source map 前提で継続成立することを確認する
- wizard と examples が新 runtime を前提に使いやすく保たれることを確認する

## 参照

- `phase-01-architecture-and-schema/index.md`: architecture と schema 契約の確定
- `phase-02-core-runtime-migration/index.md`: core runtime の移行
- `phase-03-cli-and-editor-adoption/index.md`: CLI / editor の追従
- `phase-04-cleanup-and-validation/index.md`: cleanup と回帰確認

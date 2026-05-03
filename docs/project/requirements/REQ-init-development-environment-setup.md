---
id: REQ-init-development-environment-setup
status: 採用
related:
  - docs/project/architecture.md
  - mds/core/.mds/source/init/mod.rs.md
---

# 開発環境セットアップ初期化

## 目標

mds は CLI から mds プロジェクトの作成、AI 初期化、開発環境セットアップを一貫して実行できること。

## 根拠

mds は Rust、Node.js、Python、AI CLI、各言語 toolchain を横断するため、利用開始時の環境差分を CLI が検出し、必要に応じて導入まで支援する必要があるため。

## 対象範囲

- `mds init` に project 初期化と開発環境セットアップを統合すること
- `npx`、Cargo、`uvx` の 3 経路から bootstrap できること
- project dependencies、toolchains、global AI CLI の導入を扱えること
- TypeScript / Python / Rust の lint、format、test runner は利用者が必要 / 不要と候補 tool を選択できること
- 既定では対話確認しながら外部コマンドを実行すること
- 非対話実行では明示 option がない限り変更しないこと
- 導入失敗時は部分成功を保持し、失敗項目を診断すること

## 対象外

- 利用者の承認なしに global toolchain や AI CLI を変更すること
- OS package manager の全組み合わせを初期必須範囲に含めること
- mds project ではない任意 project の環境を自動修復すること

## 成功指標

- `npx`、Cargo、`uvx` の bootstrap smoke test がある
- `mds init` の対話、非対話、部分失敗、再実行の挙動が fixture で検証できる
- setup 後に `mds doctor`、`mds check`、代表 toolchain 検証へ進める

## 制約 / 品質条件

- 外部影響が大きい操作は interactive default とし、ユーザー確認を必須にする
- 非対話実行は明示 option または明示 plan がある場合だけ変更を行う
- install 失敗は曖昧な成功扱いにせず、成功項目と失敗項目を分けて報告する
- 未選択の quality tool を暗黙必須にしない

## 関連資料

- `../architecture.md`
- `../../../mds/core/.mds/source/init/mod.rs.md`
- `../../../mds/core/.mds/source/doctor.rs.md`
- `../../../mds/cli/.mds/source/overview.md`
- `../adr/active/ADR-006-ai-agent-init-and-dev-setup.md`

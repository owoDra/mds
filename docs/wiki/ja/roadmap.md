# ロードマップ

このページでは、mds の current focus を要約します。

## 現在の focus

- canonical `.mds/source` / `.mds/test` を使う authoring-v2
- `[roots]`、`[output]`、`[[output.override]]` による package output planning
- editor feature のための source-map-backed generated-file bridge
- current な `init`、`new`、examples、AI kit template
- 構造診断と selected toolchain execution

## 近い follow-up

- authoring-v2 に沿った live docs、examples、template の整備継続
- package-level validation と editor ergonomics の改善
- package ごとに必要な output pattern と quality integration の拡張
- release、distribution、onboarding の磨き込み

## 変えない方針

- Markdown を正本として扱う
- generated file は派生物として扱う
- source doc と test doc は責務ごとに分ける
- 安全性と整合性を convenience より優先する
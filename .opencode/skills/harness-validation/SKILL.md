---
name: harness-validation
description: ハーネスとプロジェクト資料の配置、命名、参照、責務分離の整合性を確認するときに使用する
argument-hint: "範囲=<all|skills|tasks|docs> 進め方=<自走|対話>"
---

## 目的

`.agents/` と `docs/project/` の存在、配置、命名、相互参照、責務分離、project 定義との整合を確認し、修正が必要な箇所を整理する。

## 前提資料

- `.agents/project.md`
- `.agents/scripts/validate_harness.sh`
- `.opencode/skills/harness-validation/references/best-practices.md`
- `docs/project/index.md`
- `docs/project/glossary/core.md`
- `docs/project/architecture.md` が存在する場合は参照する

## やること

1. 必要なら `question` で `範囲`、`進め方`、完了条件、確認対象外を確認する
2. 必須資料の存在を確認する
3. 配置ルール、命名規則、見出し、front matter、相互参照、責務分離、index 参照リストと実体の整合を確認する
4. `.agents/scripts/validate_harness.sh` を使って機械検査を実行する
5. 問題なし、不整合、判断保留、推奨する次 task を整理する

## ルール

- 事実、推測、提案を分けて記録する
- 不整合を見つけても、勝手に正しい方を決め打ちしない
- 配置ルール確認では `Kind` を起点にする
- スコープ外の不整合は参考情報として分ける

## 確認事項

- 必須資料の存在確認を行った
- 配置、命名、参照、責務分離を確認した
- index 参照リストと実体の整合を確認した
- `validate_harness.sh` の結果を整理した
- 推奨する次 task まで整理した
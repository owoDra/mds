---
name: docs-update-adr
description: 重要な設計、実装、運用判断の記録を追加、更新、改訂するときに使用する
argument-hint: "判断=<判断内容> 対象範囲=<対象範囲> 状態=<下書き|提案中|採用|非推奨|保管>"
---

## 目的

重要な判断とその理由を後から追跡できる形で記録する。

## 前提資料

- `docs/project/index.md`
- `docs/project/glossary/core.md`
- `docs/project/adr/index.md`
- `.agents/skills/_shared/document-reference-rules.md`
- `.agents/skills/docs-update-adr/references/adr.template.md`
- `.agents/skills/docs-update-adr/references/best-practices.md`
- 関連 requirement / spec / architecture / validation

## やること

1. 必要なら `request_user_input` で判断内容、背景、候補案、関連資料を確認する
2. `docs/project/adr/active/` と `archive/` を確認し、重複や置換関係がないか調べる
3. ADR にする価値があるか判断する
4. `active/ADR-<NNN>-<short-title>.md` を作成または更新する
5. `.agents/skills/_shared/document-reference-rules.md` に従い、`docs/project/adr/index.md` を必ず更新する
6. 旧判断を置き換える場合は関連情報で後継関係を示す

## ルール

- 1 ADR = 1 判断を守る
- 判断だけでなく代替案と理由も残す
- 時系列ログではなく意思決定の要約にする
- 参照の書き方は `.agents/skills/_shared/document-reference-rules.md` に従う
- 現在有効なものは `active/`、参照優先度を落としたものは `archive/`

## 確認事項

- 判断内容と背景が明確である
- 代替案と採否理由がある
- `docs/project/adr/index.md` を更新した
- 置換関係が必要な場合は関連情報で追える
---
name: docs-update-requirement
description: プロジェクトへの要件を追加、更新、改訂するときに使用する
argument-hint: "分類=<分類> 題名=<題名> 担当チーム=<担当チーム>"
---

## 目的

実現したいこと、必要性、成功条件を requirement として一貫した形式で管理する。

## 前提資料

- `docs/project/index.md`
- `docs/project/glossary/core.md`
- `docs/project/requirements/index.md`
- `.opencode/skills/_shared/document-reference-rules.md`
- `.opencode/skills/_shared/document-update-checklist.md`
- `.opencode/skills/docs-update-requirement/references/requirement.template.md`
- `.opencode/skills/docs-update-requirement/references/best-practices.md`
- `docs/project/architecture.md` が存在する場合は参照する
- 関連 spec / ADR / validation

## やること

1. 必要なら `question` で目的、根拠、成功指標、対象範囲、対象外、制約を確認する
2. `docs/project/requirements/` を確認し、重複や矛盾がないか調べる
3. requirement として独立させるべき粒度か判断する
4. `REQ-<category>-<short-title>.md` を作成または更新する
5. `.opencode/skills/_shared/document-reference-rules.md` に従い、`docs/project/requirements/index.md` を必ず更新する
6. requirement 変更に伴う spec / validation / ADR 影響を確認する

## ルール

- requirement には実装手段ではなく目的、境界、成功条件を書く
- 成功指標は観測可能な形で書く
- 対象外を省略しない
- 1 要件 = 1 requirements 個票を守る
- 参照の書き方は `.opencode/skills/_shared/document-reference-rules.md` に従う
- 既存 requirement と重複するなら統合を検討する

## 確認事項

- 目的、根拠、成功指標がある
- 対象範囲と対象外が分かれている
- `docs/project/requirements/index.md` を更新した
- spec / validation / ADR 影響を確認した
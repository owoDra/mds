---
name: docs-update-validation
description: プロジェクトの品質担保のための検証事項を追加、更新、改訂するときに使用する
argument-hint: "確認項目=<確認項目> 変更理由=<変更理由>"
---

## 目的

品質保証と検証の期待を明文化し、変更時に必要なテストコード更新と関連資料更新を漏れなく実行する。

## 前提資料

- `docs/project/index.md`
- `docs/project/glossary/core.md`
- `docs/project/validation.md`
- `.agents/skills/_shared/document-reference-rules.md`
- `.agents/skills/_shared/document-update-checklist.md`
- `.agents/skills/docs-update-validation/references/best-practices.md`
- `.agents/skills/docs-update-validation/references/shared-check-candidates.md`
- `.agents/skills/docs-update-validation/references/validation.template.md`
- 関連 architecture / spec / code / test / ADR / tech-stack

## やること

1. 共有候補から検証項目候補を整理する
2. 必要なら `request_user_input` で採用する検証項目、検証手段、期待結果を確認する
3. `docs/project/validation.md` を更新する
4. `.agents/skills/_shared/document-reference-rules.md` に従い、`docs/project/index.md` を必ず更新する
5. validation 変更に伴う test / spec / architecture / ADR / tech-stack 影響を確認する

## ルール

- validation 変更をテスト変更と切り離さない
- 共通候補を見ずに項目を増やしすぎない
- 参照の書き方は `.agents/skills/_shared/document-reference-rules.md` に従う
- 文書上の検証項目と実際の検証手段を乖離させない

## 確認事項

- 変更理由と対象範囲が明確である
- 検証項目ごとに手段と期待結果がある
- `docs/project/index.md` を更新した
- test / spec / ADR / tech-stack 影響を確認した
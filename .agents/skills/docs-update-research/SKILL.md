---
name: docs-update-research
description: 技術調査、制約調査、実現性確認を追加、更新、正規化するときに使用する
argument-hint: "テーマ=<調査テーマ> 対象範囲=<対象範囲> 目的=<明らかにしたいこと>"
---

## 目的

正式な要求や仕様の前段階にある技術調査を、後続判断に使える形で記録する。

## 前提資料

- `docs/project/index.md`
- `docs/project/glossary/core.md`
- `docs/project/research/index.md`
- `.agents/skills/_shared/document-reference-rules.md`
- `.agents/skills/_shared/document-update-checklist.md`
- `.agents/skills/docs-update-research/references/research.template.md`
- `.agents/skills/docs-update-research/references/best-practices.md`
- 必要に応じて `docs/project/architecture.md`
- 関連する proposal / requirement / spec / ADR / integration

## やること

1. 必要なら `request_user_input` で調査テーマ、比較観点、対象範囲、完了条件を確認する
2. 既存の `docs/project/research/` を確認し、重複調査や更新対象がないか調べる
3. 技術比較、制約、実現性、未確定事項を整理する
4. `research-<topic>.md` を作成または更新する
5. `.agents/skills/_shared/document-reference-rules.md` に従い、`docs/project/research/index.md` を必ず更新する
6. 重要判断が固まった場合は proposal または ADR への昇格要否を確認する

## ルール

- research は結論を決め打ちする場ではなく、判断材料を整理する場として扱う
- 比較軸がある場合は先に明示する
- 事実と推測を混ぜない
- 参照の書き方は `.agents/skills/_shared/document-reference-rules.md` に従う
- 採用済み仕様や正式要求を research に書かない

## 確認事項

- 調査対象と比較軸が明確である
- 分かったことと未確定事項が分かれている
- 次に必要な判断が書かれている
- `docs/project/research/index.md` を更新した
- proposal / ADR への昇格要否を確認した
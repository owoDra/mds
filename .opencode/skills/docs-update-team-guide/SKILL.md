---
name: docs-update-team-guide
description: チーム別の役割、担当範囲、ルール、固有知識を追加、更新、正規化するときに使用する
argument-hint: "チーム=<team> 役割=<役割> 担当範囲=<担当範囲>"
---

## 目的

各チームの役割、担当範囲、ルール、固有知識を一貫した形で記録し、チーム境界の曖昧さを減らす。

## 前提資料

- `docs/project/index.md`
- `docs/project/glossary/core.md`
- `docs/project/teams/index.md`
- `.opencode/skills/_shared/document-reference-rules.md`
- `.opencode/skills/docs-update-team-guide/references/team-guide.template.md`
- `.opencode/skills/docs-update-team-guide/references/best-practices.md`
- 関連 requirement / spec / ADR / architecture / validation

## やること

1. 必要なら `question` で対象 team、役割、担当範囲、固有ルール、固有知識を確認する
2. 既存 guide と関連資料を確認し、名称と責務境界をそろえる
3. `<team>-guide.md` を作成または更新する
4. `.opencode/skills/_shared/document-reference-rules.md` に従い、`docs/project/teams/index.md` を必ず更新する
5. 新規 team の場合は `.agents/project.md` 更新要否を確認する

## ルール

- 全チーム共通の不変条件は team guide に書かない
- 一時的な作業メモではなく、継続的に守る内容を書く
- 参照の書き方は `.opencode/skills/_shared/document-reference-rules.md` に従う
- 他チームとの境界は曖昧語でぼかさない

## 確認事項

- 役割、担当範囲、ルール、固有知識が分かれている
- `docs/project/teams/index.md` を更新した
- `.agents/project.md` 更新要否を確認した
- architecture との境界が崩れていない
---
name: docs-update-patterns
description: プロジェクトで共通する設計、実装、運用パターンを追加、更新、改訂するときに使用する
argument-hint: "分類=<impl|data|ui|ux|api|test|ops> 題名=<題名>"
---

## 目的

複数の spec や実装で再利用すべきパターンを整理し、プロジェクト全体の一貫性を保つ。

## 前提資料

- `docs/project/index.md`
- `docs/project/glossary/core.md`
- `docs/project/patterns/index.md`
- `.agents/skills/_shared/document-reference-rules.md`
- `.agents/skills/_shared/document-update-checklist.md`
- `.agents/skills/docs-update-patterns/references/pattern.template.md`
- `.agents/skills/docs-update-patterns/references/best-practices.md`
- 関連 requirement / spec / ADR / validation / architecture

## やること

1. 必要なら `request_user_input` で目的、適用範囲、非目標を確認する
2. 既存 pattern と重複や矛盾がないか調べる
3. pattern として独立させるべき粒度か判断する
4. `<prefix><slug>.md` を作成または更新する
5. `.agents/skills/_shared/document-reference-rules.md` に従い、`docs/project/patterns/index.md` を必ず更新する
6. pattern 追加、変更に伴う spec / implementation / validation / ADR 影響を確認する

## ルール

- requirement や spec の代替にしない
- 単発のローカル事情を pattern 化しない
- 1 共通パターン = 1 patterns 個票を守る
- 参照の書き方は `.agents/skills/_shared/document-reference-rules.md` に従う
- 適用範囲と例外条件を明示する

## 確認事項

- 再利用価値がある
- 適用範囲と非適用範囲が分かる
- `docs/project/patterns/index.md` を更新した
- 影響する正本を確認した
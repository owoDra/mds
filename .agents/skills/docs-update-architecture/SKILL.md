---
name: docs-update-architecture
description: プロジェクト全体の不変条件、責務分離、設計方針を追加、更新、改訂するときに使用する
argument-hint: "変更対象=<変更対象> 理由=<理由>"
---

## 目的

共通の不変条件、責務分離、設計方針を一貫して保守し、関連資料と実装への波及を見落とさないようにする。

## 前提資料

- `docs/project/index.md`
- `docs/project/glossary/core.md`
- `docs/project/architecture.md`
- `.agents/skills/_shared/document-reference-rules.md`
- `.agents/skills/_shared/document-update-checklist.md`
- `.agents/skills/docs-update-architecture/references/best-practices.md`
- `.agents/skills/docs-update-architecture/references/architecture.template.md`
- 関連 spec / ADR / validation / tech-stack

## やること

1. 必要なら `request_user_input` で変更意図、背景、影響範囲を確認する
2. 変更対象が architecture に書くべき内容か判断する
3. `docs/project/architecture.md` を更新する
4. `.agents/skills/_shared/document-reference-rules.md` に従い、`docs/project/index.md` を必ず更新する
5. spec / code / test / validation / tech-stack への影響を確認する
6. 重要判断がある場合は ADR 更新または作成の要否を確認する

## ルール

- チーム固有ルールや一時運用は書かない
- architecture の変更を文書修正だけで終わらせない
- 参照の書き方は `.agents/skills/_shared/document-reference-rules.md` に従う
- 新しい原則が既存原則と衝突しないか確認する

## 確認事項

- 不変条件、責務分離、設計方針のどこを変えたか明確である
- `docs/project/index.md` を更新した
- spec / validation / tech-stack 影響を確認した
- 必要な ADR 更新または作成を検討した
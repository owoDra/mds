---
name: docs-update-glossary
description: プロジェクトの用語集に用語を追加、統合、正規化するときに使用する
argument-hint: "用語=<用語> 定義=<定義> 別名=<別名>"
---

## 目的

プロジェクト資料で使う用語の意味を一箇所に集約し、同義語の乱立や曖昧な命名を防ぐ。

## 前提資料

- `docs/project/index.md`
- `docs/project/glossary/index.md`
- `docs/project/glossary/core.md`
- `.agents/skills/_shared/document-reference-rules.md`
- `.agents/skills/docs-update-glossary/references/best-practices.md`
- 関連 requirement / spec / ADR / architecture

## やること

1. 必要なら `request_user_input` で追加、更新したい用語と背景を確認する
2. 既存用語、表記揺れ、別名を調べる
3. `core.md` に置くべきか、分野別ファイルに置くべきか判断する
4. 定義を追加または更新し、必要なら他資料の表記統一を行う
5. `.agents/skills/_shared/document-reference-rules.md` に従い、`docs/project/glossary/index.md` を必ず更新する

## ルール

- 用語集は一般辞書ではなく、このプロジェクトでの意味を固定する場所として扱う
- 1 用語 1 概念を基本にする
- 参照の書き方は `.agents/skills/_shared/document-reference-rules.md` に従う
- 実装詳細や一時運用メモを定義に混ぜない

## 確認事項

- 類義語と既存項目を確認した
- 先頭文だけで意味が通る定義になっている
- core と分野別ファイルの置き分けが妥当である
- `docs/project/glossary/index.md` を更新した
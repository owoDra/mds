---
name: docs-update-spec
description: 要件に対する詳細仕様を追加、更新、改訂するときに使用する
argument-hint: "分類=<flow|state|permission|interaction|api|data> 題名=<題名> 要件=<REQ-ID>"
---

## 目的

legacy spec を保守または移行する。新しい振る舞いは原則として package overview / implementation md に書き、必要な legacy spec 参照を解消する。

## 前提資料

- `docs/project/index.md`
- `docs/project/glossary/core.md`
- `mds/core/.mds/source/overview.md`
- `mds/cli/.mds/source/overview.md`
- 必要なら legacy の `docs/project/specs/index.md`
- `.agents/skills/_shared/document-reference-rules.md`
- `.agents/skills/_shared/document-update-checklist.md`
- `.agents/skills/docs-update-spec/references/spec.template.md`
- `.agents/skills/docs-update-spec/references/best-practices.md`
- 関連 requirement / ADR / validation / pattern / architecture

## やること

1. 必要なら `request_user_input` で対象 requirement、主要な入出力、状態、エラー、横断ルールを確認する
2. 共有仕様か subproject 固有仕様かを判断する
3. 既存 spec と重複、矛盾がないか調べる
4. まず migration 先となる package overview または implementation md を更新する
5. legacy spec を残す必要がある場合だけ最小差分で更新し、対応する index へ migration 状態を反映する
6. code / test / validation / ADR / harness 影響を確認し、必要なら ADR または skill を更新する

## ルール

- 新規 spec 個票は作らない
- migration 先の mds 正本を先に更新する
- 挙動は外部観測点から見える振る舞いとして書く
- 状態遷移、不変条件、エラー条件を省略しない
- 参照の書き方は `.agents/skills/_shared/document-reference-rules.md` に従う
- 共通化できる内容は pattern 化を検討する

## 確認事項

- 関連 requirement が明確である
- 配置先が shared か subproject か妥当である
- 対象 index を更新した
- code / test / validation / ADR 影響を確認した
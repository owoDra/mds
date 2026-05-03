---
name: docs-update-spec
description: 要件に対する詳細仕様を追加、更新、改訂するときに使用する
argument-hint: "分類=<flow|state|permission|interaction|api|data> 題名=<題名> 要件=<REQ-ID>"
---

## 目的

旧 spec 相当の振る舞いを package overview / implementation md / pattern / ADR に移し、以後の正本を docs/project/specs に置かないようにする。

## 前提資料

- `docs/project/index.md`
- `docs/project/glossary/core.md`
- `mds/core/.mds/source/overview.md`
- `mds/cli/.mds/source/overview.md`
- `.opencode/skills/_shared/document-reference-rules.md`
- `.opencode/skills/_shared/document-update-checklist.md`
- `.opencode/skills/docs-update-spec/references/spec.template.md`
- `.opencode/skills/docs-update-spec/references/best-practices.md`
- 関連 requirement / ADR / validation / pattern / architecture

## やること

1. 必要なら `question` で対象 requirement、主要な入出力、状態、エラー、横断ルールを確認する
2. 共有仕様か subproject 固有仕様かを判断する
3. 既存 spec と重複、矛盾がないか調べる
4. package overview または implementation md を更新する
5. 関連 requirement / ADR / pattern / task の参照先を新しい正本へ張り替える
6. code / test / validation / ADR / harness 影響を確認し、必要なら ADR または skill を更新する

## ルール

- 新規 spec 個票は作らない
- migration 先の mds 正本または docs/project の既存正本を先に更新する
- 挙動は外部観測点から見える振る舞いとして書く
- 状態遷移、不変条件、エラー条件を省略しない
- 参照の書き方は `.opencode/skills/_shared/document-reference-rules.md` に従う
- 共通化できる内容は pattern 化を検討する

## 確認事項

- 関連 requirement が明確である
- 配置先が shared か subproject か妥当である
- 対象 index を更新した
- code / test / validation / ADR 影響を確認した
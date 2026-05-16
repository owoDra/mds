---
name: task-discovery
description: 調査、現状把握、影響範囲整理が主目的の task を開始し、後続判断に使える結果へ整理するときに使用する
argument-hint: "目的=<何を明らかにしたいか> 進め方=<自走|対話>"
---

## 目的

実装や資料更新に入る前に、現状、制約、影響範囲、論点、未確定事項を整理し、次の意思決定に使える調査結果を残す。

## 前提資料

- `.agents/project.md`
- `.opencode/skills/_shared/task-template.md`
- `.opencode/skills/_shared/reference-order.md`
- `.opencode/skills/_shared/execution-modes.md`
- `.opencode/skills/_shared/request-user-input-policy.md`
- `docs/project/index.md`
- `docs/project/glossary/core.md`
- `docs/project/tech-stack.md`
- `docs/project/patterns/index.md`
- `docs/project/architecture.md` が存在する場合は参照する

## やること

1. 対象 `.agents/tasks/task-*.md` を作成または更新し、調査対象、対象外、停止条件を明文化する
2. 必要なら `question` で `目的`、`進め方`、調査しない範囲、完了条件を確認する
3. `.opencode/skills/_shared/execution-modes.md` を参照し、今回を `自走` か `対話` のどちらで進めるかと、どこで確認を挟むかを task に残す
4. 正本を優先して読み、事実として確認できたことを収集する
5. 影響を受ける資料、コード、テスト、外部依存を整理する
6. 事実、推測、未解決事項、次に必要な判断を分けて task に残す
7. 後続に推奨する skill を整理する

## ルール

- 事実と推測を混ぜない
- 調査の途中で実装へ踏み込まない
- `docs/project/tech-stack.md` と `docs/project/patterns/index.md` を読み飛ばさない
- archive は必要になった場合だけ読む
- 必要な確認は `question` を使う

## 確認事項

- 調査対象と対象外が明確である
- 目的と進め方が明確である
- 事実、推測、未解決事項が分かれている
- `docs/project/tech-stack.md` と `docs/project/patterns/index.md` を参照した
- 影響範囲が task に残っている
- 次に進む skill が提案されている
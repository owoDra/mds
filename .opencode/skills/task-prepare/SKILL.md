---
name: task-prepare
description: ユーザーからの依頼を受けて、対象範囲、禁止事項、完了条件を整理し、次に使う task 系 skill を決めるときに使用する
argument-hint: "目的=<今回の依頼> 進め方=<自走|対話>"
---

## 目的

依頼内容の認識ずれを減らし、この turn で何を達成するかを明確にしてから後続 task に渡す。

## 初期化例外

- `.agents/project.md` が未初期化、または placeholder のままなら `harness-init` を先に使う
- `harness-init` の完了後に、この skill 相当の整理へ戻る

## 前提資料

- `.agents/project.md`
- `.opencode/skills/_shared/task-template.md`
- `.opencode/skills/_shared/execution-modes.md`
- `.opencode/skills/_shared/request-user-input-policy.md`
- `docs/project/index.md`
- `docs/project/glossary/core.md`
- `docs/project/tech-stack.md`
- `docs/project/patterns/index.md`
- `docs/project/architecture.md` が存在する場合は参照する

## やること

1. 依頼内容を読み、達成したいこと、禁止事項、曖昧な点を切り分ける
2. 必要なら `question` で次を確認する
   - 今回の目的
   - 進め方を `自走` と `対話` のどちらにするか
   - 何を変えたいか
   - どこまで変えてよいか
   - 何を変えてはいけないか
   - 完了条件は何か
3. `.opencode/skills/_shared/execution-modes.md` を参照し、後続 task をどう進めるかと、どこで確認を挟むかを task に記録する
4. 対象 task に対応する skill を選ぶ
   - 調査中心なら `task-discovery`
   - 実装中心なら `task-implementation`
   - 不具合修正なら `task-fix`
   - レビューなら `task-review`
   - 検証なら `task-validation`
5. `.agents/tasks/task-<short-title>.md` を作成または更新し、後続 skill へ引き継ぐ

## ルール

- task は進捗メモではなく文脈キャッシュとして扱う
- 未確定事項を確定前提に混ぜない
- 実装や編集に入る前に対象範囲と対象外を明文化する
- `docs/project/tech-stack.md` と `docs/project/patterns/index.md` を読み飛ばさない
- 必要な確認を通常会話だけで済ませず、可能なら `question` を使う

## 確認事項

- 次に使う task 系 skill が明確である
- 目的と進め方が task に書かれている
- 対象範囲と対象外が分かれている
- 完了条件が task に書かれている
- `docs/project/tech-stack.md` と `docs/project/patterns/index.md` を参照した
- 後続が最初に読む task が更新されている
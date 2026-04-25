---
name: harness-create-skill
description: ハーネスに新しい skill を追加するときに使用する
---

## 目的

共通化したい作業を新しい skill として追加し、命名、参照、assets 配置をそろえる。

## 前提資料

- `.agents/project.md`
- `.agents/skills/harness-create-skill/references/SKILL.example.md`
- 既存の `.agents/skills/*/SKILL.md`
- `docs/project/glossary/core.md`

## やること

1. 必要なら `request_user_input` で skill 名、用途、発動条件、references の要否を確認する
2. プレフィックスを決める
   - `task-`
   - `docs-update-`
   - `harness-`
3. `.agents/skills/<skill-name>/SKILL.md` を作成する
4. skill 固有のテンプレート、サンプル、ベストプラクティスが必要なら `references/` に置く
5. 共通説明にできるものは `_shared/` へ寄せる

## ルール

- description は発動条件が分かる 1 文にする
- skill 固有でない説明を `references/` に重複させない
- ハーネス固有の説明を docs/project に置かない

## 確認事項

- `SKILL.md` に `name` と `description` がある
- skill 固有の assets だけが `references/` にある
- 共通説明を `_shared/` と重複させていない
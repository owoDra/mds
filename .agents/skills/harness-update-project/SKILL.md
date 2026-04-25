---
name: harness-update-project
description: ハーネス用の project.md を追加、更新、正規化するときに使用する
argument-hint: "項目=<name|description|language|kind|subprojects|teams|integrations> 変更内容=<内容>"
---

## 目的

`.agents/project.md` を、現在のプロジェクト正本と整合する形で一貫して保守する。

## 前提資料

- `.agents/project.md`
- `.agents/skills/harness-update-project/references/project.template.md`
- `.agents/skills/harness-update-project/references/best-practices.md`
- `docs/project/glossary/core.md`
- 関連する `docs/project/teams/`、`docs/project/integrations/`、`docs/project/architecture.md`、`docs/project/tech-stack.md`

## やること

1. 必要なら `request_user_input` で更新対象、変更理由、追加、削除、改名の別を確認する
2. `.agents/project.md` と関連正本を照合し、差分が必要か判断する
3. `Name`、`Description`、`Language`、`Kind`、`Subprojects`、`Teams`、`Integrations` を更新する
4. 新規 team を追加した場合は team guide の存在を確認する
5. 新規 integration を追加した場合は integration 資料の存在を確認する
6. `Kind` や `Subprojects` の変更がある場合は docs/project の配置影響を確認する

## ルール

- 見出し名と順序を変えない
- 名称だけでなく役割説明も更新する
- 他の正本と食い違う状態で放置しない

## 確認事項

- `.agents/project.md` と team / integration / subproject の正本が整合している
- 影響範囲がある場合は関連資料の更新要否を確認した
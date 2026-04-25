# harness-update-project のベストプラクティス

## 基本原則

- project.md はハーネスが参照する最小情報に絞る
- 見出し名と順序を固定する
- team と integration は名称だけでなく役割が読めるようにする
- kind や subprojects を変えたら docs/project の配置ルール影響を確認する

## 更新が必要な代表例

- 新規 team を追加した
- 新規 integration を追加した
- monorepo 構成や subproject 構成が変わった
- 共有言語やプロジェクト説明を更新した

## 良い project.md の条件

- ハーネスが参照する最小情報が一目で分かる
- team、integration、subproject 名称が他資料と一致している
- 役割説明が短く明確である

## project.md で落としやすい観点

- placeholder のまま運用に入ること
- 名前だけ更新して説明が古いまま残ること
- kind 変更に docs/project の配置が追従していないこと
- team や integration を追加しても正本が増えていないこと
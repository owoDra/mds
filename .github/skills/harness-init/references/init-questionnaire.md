# harness-init 固定アンケート

`harness-init` はこの順序で質問する。項目は自由に省略しない。

## 回答状態

- `unasked`: まだ質問していない
- `confirmed`: 回答が確定した
- `none`: 該当なしと確認した
- `tbd`: 必要だが未確定で、保留理由を確認した

## Phase 1: project.md 必須項目

1. `Name`
2. `Description`
3. `Language`
4. `Kind`
5. `Subprojects`
6. `Teams`
7. `Integrations`

## Phase 2: glossary / tech-stack

1. 必読用語として最初に登録すべきものは何か
2. 確定している技術スタックはあるか

## Phase 3: integrations 詳細

integration ごとに次を確認する。

1. 何のために使うか
2. どのチームが責任を持つか
3. 認証、権限、レート制限などの制約
4. 障害時の扱い

## Phase 4: architecture / validation

1. 不変条件として何を置くか
2. 責務分離をどう定義するか
3. 設計方針として何を重視するか
4. 共通で検証すべき項目は何か

## Phase 5: team guide

team ごとに次を確認する。

1. 役割
2. 担当範囲
3. 固有ルール
4. 固有知識
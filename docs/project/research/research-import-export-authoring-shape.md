---
status: draft
related:
  - docs/project/requirements/REQ-metadata-expose-uses.md
  - docs/project/patterns/data-table-metadata.md
  - docs/project/architecture.md
---

# import / export の正本表現比較

## 目的

implementation md における import / export を、全言語共通の表形式 descriptor 入力へさらに寄せる案と、現行の `Uses` + code block 方式のどちらが総合的に扱いやすいかを比較する。

## 比較対象

### A. 全言語テーブル canonical

- import だけでなく export も表形式で記述する
- code block からは import / export を極力排除し、descriptor が各言語構文へ変換する

### B. 現行方式

- 依存は `Uses` テーブルで canonical 化する
- 公開面は `Expose` テーブルで補助する
- 実際の宣言、型、制御構造は code block を正本とする

## 評価

### 文書としての分かりやすさ

- A は依存の一覧性は高いが、export まで表へ移すと「実際に何が実装されるか」を code block だけでは読めなくなる。
- A は class / trait / impl / overload / default export / re-export / named export の差を table schema に押し込む必要があり、言語差分の説明量が増える。
- B は `Uses` で依存を一覧化しつつ、code block を読むだけで実装と公開形が見える。
- B は `Expose` を索引として使えるため、公開面の一覧性も維持できる。

### 開発管理のしやすさ

- A は descriptor と table schema の設計負債が大きい。export 表現まで共通化すると、言語追加や framework overlay ごとに例外が増えやすい。
- A は rename や declaration split のたびに code block と table の二重更新が増え、差分の整合性チェックも重くなる。
- B は import だけを `Uses` に寄せるので、adapter 境界に閉じ込めやすい。宣言本体はその言語の code block に残るため、リファクタ時の追従点が少ない。
- B は parser / generator の責務分離とも整合する。共通 metadata は table、言語固有構文は code block という境界が明確である。

### AI / 人間の共同編集

- A は表の正規化が強い一方、実装変更時に AI も人間も schema を常に意識する必要がある。
- B は `Uses` と `Expose` を見れば依存と公開面の要約が取れ、深い実装は code block に集中できる。
- B の方が「説明は Markdown、依存は table、実装は code block」という役割分担が直感的で、レビューでも追いやすい。

## 結論

- 総合的には B、つまり現行の `Uses` + `Expose` + code block 方式を維持する方がよい。
- canonical table 化は import / use / require のような依存情報までに留める。
- export を完全 table 化するのではなく、公開面の索引として `Expose` を使い、実際の宣言は code block に残す。
- 今後 descriptor を拡張する場合も、優先順位は「依存の生成規則」と「診断・品質規則」の data 化であり、宣言本体まで table DSL 化しない。

## 昇格条件

- `Expose` の意味や列構成を変える場合
- export の table canonical 化を正式提案する場合
- language descriptor に declaration DSL を追加する場合
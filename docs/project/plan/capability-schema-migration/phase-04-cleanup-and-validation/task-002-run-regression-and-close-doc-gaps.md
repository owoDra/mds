# Task 002: Run Regression And Close Doc Gaps

## 目的

新 runtime 前提で回帰確認を行い、proposal / spec / validation の残差を解消する。

## 前提条件

- examples と runtime が更新済みである

## 作業内容

- build / lint / typecheck / test / doctor / package sync を確認する
- doc gap や spec drift を洗い出して正本へ反映する

## 完了条件

- regression が通り、主要 docs が実装と整合する

## 検証方法

- validation に沿って representative command を実行する

## 依存関係

- `task-001-remove-obsolete-builtins-and-align-examples.md`

## 成果物

- `docs/project/validation.md`
- 関連 spec / proposal / plan

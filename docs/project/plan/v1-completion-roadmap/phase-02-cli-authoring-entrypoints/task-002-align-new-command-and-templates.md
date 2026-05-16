# Task 002: Align New Command And Templates

## 目的

`mds new` を `mds new <path> <kind> [options]` 契約へ揃え、impl/test/overview/root module template を v1 authoring policy に従って起票できる状態にする。

## 前提条件

- core が doc kind、doc profile、label override、link policy を解釈できる
- `SPEC-cli-init-and-new-workflows` の `new` 契約が参照できる

## 作業内容

- `new` の引数構造を `path` と `kind` 中心へ見直す
- kind 未指定時の対話 fallback と、解決不能時の usage / selection 動作を整える
- impl md、test md、overview special file、root module doc の template を追加または更新する
- label override、link policy、language detection を template 出力へ反映する

## 完了条件

- `new` が kind ごとに適切な template を canonical root に起票できる
- `overview.md` と prose-only root module doc を区別して生成できる
- unmanaged file 上書きが `--force` なしで発生しない

## 検証方法

- args / new 系 test を追加する
- temp package で impl / test / overview / root module 起票を確認する

## 依存関係

- `../phase-01-core-runtime-and-authoring-policy/task-001-complete-language-and-schema-resolution.md`
- `../phase-01-core-runtime-and-authoring-policy/task-003-enforce-link-policy-and-overview-contract.md`

## 成果物

- `mds/cli/src/args.rs`
- `mds/core/src/new.rs`
- `mds/core/src/init/mod.rs`
- `mds/cli/tests/args_test.rs`
- `mds/core/tests/`

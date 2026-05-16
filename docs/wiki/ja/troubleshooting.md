# トラブルシューティング

このページでは、current な mds 利用でよく起きる失敗をまとめます。

## mds package が見つからない

次を確認してください。

- `mds.config.toml` がある
- `[package].enabled = true`
- `--package` が package root を指している
- `package.md` と package manager metadata が想定どおりにある

## authoring root が違う

current package は canonical root だけを使います。

- `source_md` は `.mds/source`
- `test_md` は `.mds/test`

出力 layout を変えたい場合は `[output]` や `[[output.override]]` を変え、Markdown root は変えません。

## 必須 section が足りない

編集中の doc kind に対応する shape を確認してください。

- source doc: `Purpose`、`Contract`、`API`、`Source`、`Cases`
- test doc: `Purpose`、`Covers`、`Cases`、`Test`
- overview doc: `Purpose`、`Architecture`、`Rules`

## source/test 混在や legacy table warning

`split_source_and_test = true` のときは、source behavior は source doc、executable test は test doc に分けます。`legacy_tables` warning が出たら、旧 metadata table pattern を消し、API intent は prose で書きます。

## wiki-style link が解決できない

- `[[module]]` は package-local logical module id に解決する必要があります。
- `[[module#symbol]]` は shared definition や prose に書いた symbol 名など Markdown-native な情報に解決する必要があります。

source/test doc の path から導かれる module id と symbol の綴りを確認してください。

## output path が想定と違う

次を確認します。

- `[roots].source_out`
- `[roots].test_out`
- `[output]`
- `[[output.override]]`

書き込む前に `mds build --dry-run` で plan を確認してください。

## file を上書きできない

mds は managed header が付いた file だけを上書きします。対象 path に handwritten file がある場合は、その file を移動するか output pattern を変えます。

## 必要な tool が足りない

次を実行してください。

```bash
mds doctor --package ./path/to/package
```

そのうえで、関連する `[quality.<lang>]` が本当に必要な tool だけを要求しているか確認します。
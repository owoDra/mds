# モノレポでの使い方

このページでは、複数 package を含む repository で mds をどう使うかを説明します。

## 基本方針

mds は package ごとに有効化します。1 つの repository に mds package と non-mds package が混在していても安全に扱えます。

## package の判定

mds は次のような情報を package root で確認します。

- `mds.config.toml`
- `package.md`
- `package.json`、`pyproject.toml`、`Cargo.toml` などの認識済み package manager metadata

対象外 package は書き換えません。

## package ごとの authoring layout

各 package は自分の canonical authoring root と output root を持ちます。

```text
package-a/
├── mds.config.toml
├── package.md
├── .mds/source/
├── .mds/test/
├── src/
└── tests/
```

## 複数言語

package ごとに使う suffix や `[quality.<lang>]` section を変えられます。package boundary、output planning、managed-file safety rule は共通です。

## 推奨運用

- package ごとに `mds.config.toml` を置く
- 出力はその package の内側に保つ
- authoring root は `.mds/source` と `.mds/test` に統一する
- 初回書き込み前に `mds lint` と `mds build --dry-run` を実行する
# Markdown 正本

このページでは、mds の current な Markdown 文書モデルを説明します。

## canonical roots

- `.mds/source` には source doc と source overview を置きます。
- `.mds/test` には verification doc と test overview を置きます。

これらの root は固定です。単なる naming preference ではなく authoring model の一部です。

## 文書種別

| path | 役割 | 想定 section |
| --- | --- | --- |
| `.mds/source/overview.md` | source 階層の overview | `Purpose`、`Architecture`、`Rules` |
| `.mds/test/overview.md` | verification 階層の overview | `Purpose`、`Architecture`、`Rules` |
| `.mds/source/**/*.lang.md` | 1 機能または root module の source doc | `Purpose`、`Contract`、`API`、必要なら `Source`、`Cases` |
| `.mds/test/**/*.md` | 1 機能または module の executable verification | `Purpose`、`Covers`、`Cases`、`Test` |

`.mds/source/index.ts.md`、`.mds/source/lib.rs.md`、`.mds/source/mod.rs.md` のような language root module doc は、通常 `Purpose` と `API` を中心に書き、root module 自身が runtime behavior を持つときだけ `Source` を追加します。

## source doc

source doc は stable behavior、public surface note、generated source code を置く場所です。

- `Purpose` は機能の理由を説明します。
- `Contract` は入力、出力、制約、失敗条件を記録します。
- `API` は public exports や entrypoint を prose で説明します。
- `Source` は source output になる executable code fence を持ちます。
- `Cases` は代表的な振る舞いを記録します。

source doc は、最初は prose だけで始めて後から `Source` fence を追加しても構いません。

## test doc

test doc は executable verification を記述します。

- `Purpose` は何を検証するかを書きます。
- `Covers` は対象 source module id を示します。
- `Cases` は期待結果を記録します。
- `Test` は executable test fence を持ちます。

既定の check policy では、source behavior と executable verification を同じ doc kind に混在させると error になります。

## code と public surface の書き方

- generated file に必要な import や `use` は executable code fence の中に通常どおり書きます。
- public surface や re-export intent は `API` prose か root module doc に書きます。
- 旧 metadata table pattern は legacy input であり、current live authoring surface ではありません。

## 文書間参照

`Covers` や prose では `[[greet]]` や `[[greet#symbol]]` のような package-local wiki-style link を使えます。core diagnostics は Markdown-native な情報だけでこれらを検証します。

## package 境界

`package.md` と package manager metadata は package 単位のルールと metadata snapshot を扱い、feature-level authoring は `.mds/source` と `.mds/test` に置きます。
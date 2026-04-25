# mds 最終要件まとめ

## 1. mds の目的

`mds` は、**Markdown を設計書兼ソースの正本として扱う開発ツールチェーン**なのだ。

人間は Markdown を読んで、設計・仕様・実装・テストを追えるのだ。
AIエージェントは同じ Markdown を読んで、実装・修正・検証を行うのだ。
生成された `.ts`、`.py`、`.rs` などは派生物であり、正本は `.md` なのだ。

---

# 2. 基本思想

## 正本

* `.md` が正本
* 生成コードは派生物
* 設計とコードを分離しすぎない

## 方針

* 自由度より規約を優先する
* 1つの実装 md は1機能だけを扱う
* import 系はコードブロック外に置く
* 依存と公開面を表で明示する
* Obsidian でそのまま読める Markdown を使う
* Node / Rust / Python 環境を横断して使えるようにする

---

# 3. 配布・実装方針

`mds` は **npm専用ツールではなく、マルチエコシステム対応の Markdown Source toolchain** として作るのだ。

## 推奨構成

```text
mds/
  crates/
    mds-core/
    mds-cli/
    mds-lang-rs/

  packages/
    core/
    cli/
    lang-ts/
    lang-py/
    lang-rs/

  python/
    mds/
    mds_lang_py/
```

## 中核

* Rust core を推奨
* CLI は native binary として動く
* npm / cargo / uv から使えるようにする

## 導入口

```bash
npm install -D @mds/cli @mds/lang-ts
```

```bash
cargo install mds
```

```bash
uv tool install mds
# または
uvx mds
```

---

# 4. 必須パッケージ・アダプター

`@mds/lang-*` は将来対応ではなく、**最初から必須構成**なのだ。
ただし、npm以外では配布名が変わってよいのだ。

## 必須概念

* `mds core`
* `mds cli`
* language adapter

## npm 側

* `@mds/core`
* `@mds/cli`
* `@mds/lang-ts`
* `@mds/lang-py`
* `@mds/lang-rs`

## Cargo / uv 側

* `mds`
* `mds-lang-rs`
* `mds-lang-py`

## language adapter の責務

* import / use / require の生成
* lint 接続
* format 接続
* test runner 接続
* ファイル名規約
* `Types` / `Source` / `Test` の出力規則
* md内コードブロックへの仮想 lint / format

---

# 5. 設定ファイル

設定ファイルは **`mds.config.toml` 固定** にするのだ。

`mds.config.ts` や `mds.config.json` は使わないのだ。
Node 以外の Cargo / uv 環境でも自然に読めるようにするためなのだ。

---

# 6. 設定の継承

`mds.config.toml` は、ルートにもサブプロジェクトにも置けるのだ。

## 優先順位

1. built-in default
2. ルート `mds.config.toml`
3. サブプロジェクト `mds.config.toml`

近い設定が勝つのだ。

---

# 7. `mds.config.toml` の役割

## できること

* package ごとの mds 有効 / 無効
* language adapter 設定
* lint / format / test 設定
* 出力ルート設定
* 除外パス設定
* セクション名の表示名 override
* テーブル列名の表示名 override

## できないこと

* セクションの意味変更
* `Uses` の意味変更
* `Expose` の意味変更
* 必須構造の破壊

つまり、**見た目の語彙は変えられるが、意味は変えられない**のだ。

---

# 8. config 例

```toml
root = "."
strict = true
link_style = "wikilink"
allow_raw_source = true
exclude = ["dist", "node_modules", "target", ".venv"]

[packages."packages/core"]
enabled = true
language = "ts"
runner = "npm"
output_root = "src"

[packages."packages/rust-core"]
enabled = true
language = "rs"
runner = "cargo"
output_root = "src"

[packages."packages/python-lib"]
enabled = true
language = "py"
runner = "uv"
output_root = "src"

[packages."apps/web"]
enabled = false

[languages.ts]
formatter = "prettier"
linter = "eslint"
test_runner = "vitest"
types_mode = "separate"
test_file_pattern = "{name}.test.ts"
types_file_pattern = "{name}.types.ts"

[languages.py]
formatter = "ruff format"
linter = "ruff"
test_runner = "pytest"
types_mode = "separate"
test_file_pattern = "test_{name}.py"

[languages.rs]
formatter = "rustfmt"
linter = "clippy"
test_runner = "cargo test"
types_mode = "separate"
test_file_pattern = "{name}_test.rs"
```

---

# 9. セクション名・列名 override

内部では canonical key を使うのだ。
Markdown 上の表示名は config で変えられるのだ。

## 例

```toml
[labels.sections.purpose]
label = "目的"
aliases = ["Purpose", "目的"]

[labels.sections.contract]
label = "契約"
aliases = ["Contract", "契約"]

[labels.sections.types]
label = "型"
aliases = ["Types", "型"]

[labels.sections.source]
label = "実装"
aliases = ["Source", "実装"]

[labels.sections.cases]
label = "検証観点"
aliases = ["Cases", "検証観点"]

[labels.sections.test]
label = "テスト"
aliases = ["Test", "テスト"]
```

## テーブル列名の例

```toml
[labels.tables.uses.columns]
from = { label = "From", aliases = ["From", "種別"] }
target = { label = "Target", aliases = ["Target", "参照先"] }
expose = { label = "Expose", aliases = ["Expose", "使用"] }
summary = { label = "Summary", aliases = ["Summary", "概要"] }

[labels.tables.expose.columns]
kind = { label = "Kind", aliases = ["Kind", "種類"] }
name = { label = "Name", aliases = ["Name", "名前"] }
summary = { label = "Summary", aliases = ["Summary", "概要"] }
```

---

# 10. モノレポ対応

`mds` はモノレポ対応を前提にするのだ。
すべてのサブプロジェクトが mds である必要はないのだ。

## 方針

* package 単位で mds を有効化できる
* mds を使わない package は直書きソースで動いてよい
* `allow_raw_source = true` で混在を許可する

## 例

```text
repo/
  mds.config.toml
  index.md

  apps/
    web/
      package.json
      src/
        main.ts

  packages/
    core/
      mds.config.toml
      package.json
      package.md
      index.md
      auth/
        index.md
        session-store.ts.md

    rust-core/
      mds.config.toml
      Cargo.toml
      package.md
      index.md
      parser/
        ast.rs.md

    python-lib/
      mds.config.toml
      pyproject.toml
      package.md
      index.md
      token_store.py.md
```

---

# 11. mds 対象 package の判定

有力な正式案はこれなのだ。

* `mds.config.toml` で `enabled = true`
* package root に `package.md` がある
* 実体の package 定義がある

  * JS/TS: `package.json`
  * Python: `pyproject.toml`
  * Rust: `Cargo.toml`

この3つで mds package とみなすのだ。

---

# 12. 文書種別

`mds` の文書は3種類なのだ。

## `index.md`

そのディレクトリ以下の説明文書なのだ。

## `package.md`

プロジェクト、またはモノレポのサブプロジェクト単位の package 説明文書なのだ。

## `*.{lang-ext}.md`

1機能1実装の実装 md なのだ。

---

# 13. `index.md`

`index.md` は、そのディレクトリ以下の overview / architecture / navigation を担当するのだ。

## ルートの `index.md`

プロジェクト全体のアーキテクチャ説明を含むのだ。

## 下位の `index.md`

その階層以下の責務、構成、公開面、ルールを書くのだ。

## 必須セクション

* `## Purpose`
* `## Architecture`
* `## Exposes`
* `## Rules`

`Structure` は作らず、`Exposes` に統合するのだ。

---

# 14. `index.md` の Exposes

B案のテーブル形式で確定なのだ。
軽い概要説明も持つのだ。

```md
## Exposes

| File | Expose | Summary |
|---|---|---|
| [[session-store.ts.md]] | SessionStore, loadSession | セッション保存と取得 |
| [[token-service.ts.md]] | TokenService, issueToken | トークン生成と検証 |
| [[login-flow.ts.md]] | login | ログイン処理全体 |
```

---

# 15. `package.md`

`package.md` は、ディレクトリ単位ではなく **package単位** なのだ。

## 対象

* 単一プロジェクトならプロジェクトルート
* モノレポならサブプロジェクトルート

## 元データ

* JS/TS: `package.json`
* Python: `pyproject.toml`
* Rust: `Cargo.toml`

## 生成方針

* package metadata から自動生成を基本にする
* `Rules` は手書き補足を許す

---

# 16. `package.md` 必須セクション

* `## Package`
* `## Dependencies`
* `## Dev Dependencies`
* `## Rules`

## Dependencies テーブル

```md
## Dependencies

| Name | Version | Summary |
|---|---|---|
| zod | ^4.0.0 | runtime validation |
| lodash-es | ^4.17.21 | utility |
```

## Dev Dependencies テーブル

```md
## Dev Dependencies

| Name | Version | Summary |
|---|---|---|
| vitest | ^3.2.0 | test runner |
| typescript | ^5.8.0 | compiler |
```

---

# 17. 実装 md の命名規則

実装 md は **`*.{lang-ext}.md`** にするのだ。

## 例

* `session-store.ts.md`
* `token-service.ts.md`
* `token_store.py.md`
* `ast.rs.md`

## 出力

末尾の `.md` を外したものが Source の出力先になるのだ。

* `session-store.ts.md` → `session-store.ts`
* `token_store.py.md` → `token_store.py`
* `ast.rs.md` → `ast.rs`

---

# 18. 1md1実装

実装 md は、**1つの機能だけ**を扱うのだ。

禁止:

* 複数機能を1つの md に混ぜる
* 無関係な責務を同居させる
* 自由な `file=` 指定で別ファイルへ飛ばす

許可:

* 同一機能の `Types`
* 同一機能の `Source`
* 同一機能の `Test`
* 同一機能の `Cases`
* 同一機能の `Contract`

---

# 19. 実装 md の必須セクション

```md
# Session Store

## Purpose

## Contract

## Types

## Source

## Cases

## Test
```

## セクションの意味

| セクション      | 意味           |
| ---------- | ------------ |
| `Purpose`  | 機能の目的        |
| `Contract` | 外から見た振る舞いの約束 |
| `Types`    | 型・データ構造      |
| `Source`   | 実装本体         |
| `Cases`    | 期待結果の要約      |
| `Test`     | 実行可能なテスト     |

---

# 20. Contract と Types の違い

## Contract

振る舞いの約束なのだ。

例:

* `load(token)` は session があれば返す
* 期限切れなら `null` を返す
* 保存方式は呼び出し側に見せない

## Types

データ構造なのだ。

例:

* `Session`
* `SessionInput`
* `SessionError`

Contract は独立 md にしないのだ。
実装 md 内の `## Contract` に置くのだ。

---

# 21. `Types` / `Source` / `Test` の内部構造

各セクションは、本文領域と末尾メタ領域を持つのだ。

## 本文領域

* 説明文
* 補足見出し
* コードブロック
* コード間の説明

## 末尾メタ領域

* `### Expose`
* `### Uses`

## コードブロック

複数に分けてよいのだ。
コードブロックの間に説明文を挟んでよいのだ。

## 連結

同一セクション内のコードブロックは、原則として出現順に連結するのだ。

---

# 22. Expose 文法

`Expose` はテーブル形式なのだ。

```md
### Expose

| Kind | Name | Summary |
|---|---|---|
| type | Session | セッション本体 |
| interface | SessionStoreOptions | 初期化オプション |
| class | SessionStore | セッション保存サービス |
| fn | loadSession | セッション取得 |
```

## 列

* `Kind`
* `Name`
* `Summary`

## Kind 候補

* `type`
* `interface`
* `class`
* `fn`
* `const`
* `enum`
* `trait`
* `struct`

正式な Kind 一覧は言語横断で決めるが、言語固有 Kind も adapter が扱えるようにするのだ。

---

# 23. Uses 文法

`Uses` もテーブル形式なのだ。

```md
### Uses

| From | Target | Expose | Summary |
|---|---|---|---|
| internal | [[storage.ts.md]] | StorageApi | 永続化API |
| internal | [[clock.ts.md]] | Clock | 現在時刻取得 |
| package | vitest | describe, it, expect | テスト関数 |
| builtin | node:crypto | randomUUID | ID生成 |
```

## 列

* `From`
* `Target`
* `Expose`
* `Summary`

## From

* `internal`
* `package`
* `builtin`

---

# 24. Uses の粒度

`Uses` は `Types` / `Source` / `Test` ごとに別々に持つのだ。

理由:

* 型依存
* 実装依存
* テスト依存

この3つは意味が違うのだ。

---

# 25. import / use / require の扱い

import 系はコードブロック内に書かないのだ。

## ルール

* 依存は `Uses` に書く
* import / use / require は language adapter が生成する
* コードブロックには本体だけを書く

これでドキュメントとして読みやすくなるのだ。

---

# 26. Types 出力

`Types` は B案で確定なのだ。
つまり、別ファイルに出すのだ。

## 例

`session-store.ts.md` から生成:

```text
session-store.ts
session-store.types.ts
session-store.test.ts
```

---

# 27. Test 出力

`Test` は A案で確定なのだ。
つまり、1実装 md につき1テストファイルなのだ。

## 例

`session-store.ts.md` から:

```text
session-store.test.ts
```

---

# 28. Cases と Test

`Cases` と `Test` は両方持つのだ。

## Cases

人間とAI向けの期待結果を書くのだ。

## Test

実行可能なテストコードを書くのだ。

「期待結果だけ書いてテストは全部自動生成」にはしないのだ。
最初は **Cases + 実テストコード** にするのだ。

---

# 29. lint / format

lint / format は生成後コードだけではなく、**mdの状態に対して適用される**のだ。

## やり方

1. md を読む
2. `Types` / `Source` / `Test` のコードブロックを抽出する
3. `Uses` から仮想 import を作る
4. 仮想コードに formatter / linter をかける
5. 結果を md のコードブロックへ戻す

## 例

`@mds/lang-ts`:

* Prettier
* ESLint
* Vitest

`@mds/lang-py`:

* Ruff format
* Ruff
* Pytest

`@mds/lang-rs`:

* rustfmt
* clippy
* cargo test

---

# 30. Obsidian との関係

Obsidian 専用パッケージは作らないのだ。

## 方針

* 標準寄り Markdown を使う
* Obsidian ではそのまま閲覧する
* リンクと Graph view を活かす

## リンク

* 標準 Markdown リンク対応
* Obsidian `[[wikilink]]` も許容

---

# 31. 生成ルールの基本

## Source

`*.{lang-ext}.md` から `.md` を外して生成するのだ。

```text
session-store.ts.md -> session-store.ts
```

## Types

language adapter の pattern に従うのだ。

```text
session-store.ts.md -> session-store.types.ts
```

## Test

language adapter の pattern に従うのだ。

```text
session-store.ts.md -> session-store.test.ts
```

## 自由な file 指定

作らないのだ。
パスと命名規約で決めるのだ。

---

# 32. `symbol` の扱い

`symbol` は採用しない方向なのだ。

理由:

* 1md1実装
* `Expose` がある
* path が強制される
* 別機能混在が禁止される

公開名は `Expose` テーブルで表すのだ。

---

# 33. CLI コマンド案

```bash
mds build
mds check
mds graph
mds lint
mds format
mds test
mds doctor
mds package sync
```

## 役割

* `build`: md からコード生成
* `check`: 構造・参照・表の検証
* `graph`: md依存グラフ表示
* `lint`: md状態で lint
* `format`: md状態で format
* `test`: 生成または仮想コードで test
* `doctor`: 環境確認
* `package sync`: package metadata から package.md を更新

---

# 34. 未確定だが次に決める項目

かなり固まったけど、次の細部はまだ詰める余地があるのだ。

## 1. `Kind` の正式一覧

共通 Kind と言語固有 Kind をどう分けるか。

## 2. `From` の正式一覧

`internal / package / builtin` でほぼ良いが、`workspace` を追加するか。

## 3. `Uses.Expose` の複数名表現

`A, B, C` でよいか、配列っぽくするか。

## 4. コードブロック連結時の改行規則

ブロック間に何行入れるか。

## 5. generated files の配置

生成物を md 横に置くか、`output_root` 配下にまとめるか。

## 6. md内補助見出しの深さ

`####` まで許すか、制限するか。

---

# 35. 一言でいうと

`mds` は、**Markdownを正本にして、多言語コード・型・テストを生成する、強規約のAI時代向け開発ツールチェーン**なのだ。

`index.md` が階層の設計を説明する。
`package.md` が package 情報を説明する。
`*.{lang-ext}.md` が1機能1実装を表す。
`Expose` が公開面を示す。
`Uses` が依存を示す。
import と lint と format と test は language adapter が担う。
設定は `mds.config.toml` 固定で、サブプロジェクトごとに override できるのだ。

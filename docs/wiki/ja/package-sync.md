# パッケージ情報同期

このページでは、`mds package sync` の目的と使い方を説明します。

## 目的

`package.md` は、パッケージ名、バージョン、依存関係、開発用依存関係を人間が読める形で説明する文書です。

一方で、実際のパッケージ情報は `package.json`、`pyproject.toml`、`Cargo.toml` などにあります。

`mds package sync` は、これらのパッケージ情報をもとに、`package.md` の管理部分を同期します。

## 対象になる情報

主に次の情報を同期します。

| 情報 | 説明 |
| --- | --- |
| パッケージ名 | 対象パッケージの名前です。 |
| バージョン | 対象パッケージのバージョンです。 |
| 依存関係 | 実行時に必要な依存関係です。 |
| 開発用依存関係 | 開発や検査に必要な依存関係です。 |

## 実行方法

`package.md` を更新する場合は、次のように実行します。

```bash
mds package sync --package path/to/package
```

差分確認だけを行う場合は、`--check` を使います。

```bash
mds package sync --package path/to/package --check
```

## 手書き領域の扱い

`mds package sync` は、`package.md` 全体を自由に作り直すものではありません。

同期対象の管理部分を更新し、その他の説明やルールは残す方針です。

ただし、管理部分に手書きの文章が混ざっている場合は、意図しない削除を避けるためにエラーになります。

## フックに紐づけた同期確認

パッケージ情報を変更したときに、`package.md` の同期漏れを早く見つけたい場合は、フックに `mds package sync --check` を紐づける運用ができます。

この運用では、フックが `package.md` を直接更新するのではなく、同期が必要かどうかを検査します。差分がある場合はエラーにして、利用者が内容を確認してから `mds package sync` を実行します。

`mds.config.toml` では、次のようにフック用の設定を有効にできます。

```toml
[package_sync]
hook_enabled = true
```

`hook_enabled = true` の場合、既定のフック用コマンドは次の内容です。

```bash
mds package sync --check
```

別のコマンド名や実行方法にしたい場合は、明示的に指定できます。

```toml
[package_sync]
hook_enabled = true
hook = "mds package sync --check"
```

設定名には、ハイフン区切りの `[package-sync]` も使えます。

```toml
[package-sync]
hook-enabled = true
hook-command = "mds package sync --check"
```

この設定は、フック運用で使うコマンドを mds 側に記録するためのものです。フック機構そのものへの登録は、利用しているパッケージ管理ツールや継続的インテグレーションの仕組みに合わせて行います。

## フック運用の例

継続的インテグレーションでは、次のように同期確認を実行できます。

```bash
mds package sync --package path/to/package --check
```

ローカルのコミット前フックに紐づける場合も、まずは `--check` を使うことを推奨します。

```bash
mds package sync --check
```

`--check` は `package.md` を書き換えません。同期が必要な場合は失敗として扱い、差分を確認してから次のコマンドで更新します。

```bash
mds package sync --package path/to/package
```

## 推奨運用

- パッケージ情報の正は、各言語のパッケージ情報ファイルに置きます。
- `package.md` には、人間が読むための説明と同期された依存関係を置きます。
- 継続的な確認では `mds package sync --check` を使います。
- 差分を確認してから `mds package sync` で更新します。
- フックに紐づける場合も、既定では `--check` にして自動更新ではなく同期漏れの検出に使います。

## 注意点

- 任意の Markdown 文書を生成するための機能ではありません。
- パッケージ情報以外の文書を同期対象にしません。
- 外部の公開処理やパッケージ公開を暗黙に実行しません。
- フックを有効にしても、mds が外部ツールの設定ファイルを自動で登録、変更するわけではありません。
- フックで `mds package sync` を直接実行して自動更新する運用は、差分確認を省略しやすいため推奨しません。

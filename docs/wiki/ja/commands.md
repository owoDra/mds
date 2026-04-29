# コマンド

このページでは、mds のコマンドの目的と使い方を説明します。

## 基本形

mds コマンドは、対象パッケージを指定して実行できます。

```bash
mds check --package path/to/package
```

`--package` を省略した場合は、現在のディレクトリ以下から mds が有効なパッケージを探します。

## `mds check`

`mds check` は、Markdown の構造、表、設定、生成先を検査します。

主に次の内容を確認します。

- 必須セクションが存在するか
- `Expose` と `Uses` の表が正しいか
- 対象言語を判断できるか
- 生成先がパッケージの外に出ていないか
- 管理対象ではない既存ファイルを上書きしようとしていないか

実行例です。

```bash
mds check --package path/to/package
```

## `mds build`

`mds build` は、Markdown から派生コードを生成します。

```bash
mds build --package path/to/package
```

生成対象は、実装 Markdown の `Types`、`Source`、`Test` に書かれたコードブロックです。

## `mds build --dry-run`

`mds build --dry-run` は、ファイルを書き込まずに生成予定と差分を表示します。

```bash
mds build --package path/to/package --dry-run
```

初めて生成する場合や、生成規則を変更した場合は、先にこのコマンドで差分を確認してください。

## `mds lint`

`mds lint` は、Markdown 内のコードブロックを対象に、言語ごとの検査ツールを実行します。

```bash
mds lint --package path/to/package
```

TypeScript、Python、Rust では利用する検査ツールが異なります。具体的な接続は言語アダプターが担当します。

## `mds lint --fix`

`mds lint --fix` は、Markdown 内のコードブロックに自動修正を適用します。

```bash
mds lint --package path/to/package --fix
```

確認だけを行いたい場合は、`--check` を追加します。

```bash
mds lint --package path/to/package --fix --check
```

自動修正は、Markdown 全体を書き換えるのではなく、対象のコードブロックの中身を更新します。

## `mds test`

`mds test` は、Markdown 内の `Test` セクションにあるテストコードを対象に、言語ごとのテスト実行を行います。

```bash
mds test --package path/to/package
```

## `mds doctor`

`mds doctor` は、実行環境と必要なツールを診断します。

```bash
mds doctor --package path/to/package
```

表示形式を JSON にしたい場合は、次のように実行します。

```bash
mds doctor --package path/to/package --format json
```

## `mds package sync`

`mds package sync` は、対象言語のパッケージ情報をもとに `package.md` の管理部分を同期します。

```bash
mds package sync --package path/to/package
```

差分の確認だけを行う場合は、`--check` を追加します。

```bash
mds package sync --package path/to/package --check
```

## `mds init`

`mds init` は、mds を使うための初期化を行います。

```bash
mds init --package path/to/package
```

初期化では、プロジェクト構成、支援ツール向けの設定、開発環境の準備を扱います。外部コマンドの実行や環境変更は、利用者が明示した場合に行う方針です。

## `mds release check`

`mds release check` は、公開前の成果物検査を行います。

```bash
mds release check --manifest release.mds.toml
```

公開前検査では、成果物、チェックサム、署名、ソフトウェア部品表、来歴情報、インストール後の動作確認などを扱います。

## 終了コード

mds は、失敗の種類を区別できるように終了コードを使い分けます。

| 終了コード | 意味 |
| --- | --- |
| `0` | 成功しました。 |
| `1` | 診断または検査エラーがあります。 |
| `2` | コマンドの使い方または設定に誤りがあります。 |
| `3` | 内部エラーが発生しました。 |
| `4` | 実行環境または必要なツールが不足しています。 |

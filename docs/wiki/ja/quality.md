# 品質検査

このページでは、mds が扱う品質検査を説明します。

## 品質検査の考え方

mds では、生成されたファイルだけではなく、生成元である Markdown の状態を検査します。

Markdown の構造、依存関係、生成先、コードブロック、対象言語の検査を合わせて確認することで、正本と派生コードのずれを減らします。

## 構造検査

`mds check` は、Markdown の構造を検査します。

主に次の内容を確認します。

- 必須セクションが存在するか
- `Expose` と `Uses` の表が正しいか
- 実装 Markdown のファイル名から対象言語を判断できるか
- 生成先がパッケージの内側に収まっているか
- 生成先に手書きファイルを上書きする危険がないか
- `package.md` とパッケージ情報に矛盾がないか

## 静的検査

`mds lint` は、Markdown 内のコードブロックを対象に、言語ごとの静的検査を実行します。

コードブロックを一時的なファイルとして扱い、`Uses` から生成した依存宣言を付けて検査します。

検査ツールは言語ごとに選択できます。TypeScript では ESLint または Biome、Python では Ruff、Rust では Cargo Clippy などを利用できます。未選択の場合、その言語の静的検査は実行されません。

## 自動修正

`mds lint --fix` は、Markdown 内のコードブロックに自動修正を適用します。

この処理は、Markdown の説明文や構造を自由に書き換えるものではありません。修正対象は、`Types`、`Source`、`Test` のコードブロックです。

差分確認だけを行う場合は、`--check` を使います。

```bash
mds lint --package path/to/package --fix --check
```

修正ツールも言語ごとに選択できます。TypeScript では Prettier または Biome、Python では Ruff format または Black、Rust では rustfmt などを利用できます。

## テスト

`mds test` は、Markdown 内の `Test` セクションを対象に、言語ごとのテスト実行を行います。

実装 Markdown にテストコードを置くことで、機能の目的、契約、実装、テストを同じ文書から追跡できます。

テスト実行ツールは、TypeScript では Vitest または Jest、Python では Pytest または unittest、Rust では Cargo test または cargo-nextest を選択できます。

## 環境診断

`mds doctor` は、実行環境と必要なツールを診断します。

```bash
mds doctor --package path/to/package
```

診断では、対象パッケージで有効な言語アダプターと `[quality.*]` の設定に応じて、必要な実行環境やツールを確認します。未選択のツールは不足扱いになりません。

## 推奨する確認順序

開発中は、次の順序で確認すると問題を切り分けやすくなります。

1. `mds check`
2. `mds build --dry-run`
3. `mds build`
4. `mds lint`
5. `mds test`
6. `mds doctor`

このリポジトリ自身の公開前には、上記に加えて `./.github/script/release-check.sh --manifest release.mds.toml` を実行します。

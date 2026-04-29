# 設定

このページでは、`mds.config.toml` の役割と主要な設定を説明します。

## 基本方針

mds の設定ファイル名は `mds.config.toml` です。

設定形式を複数用意せず、TOML に固定します。これにより、設定の解釈を単純にし、リポジトリ内のどこで mds が有効なのかを確認しやすくします。

## 最小例

```toml
[package]
enabled = true

[roots]
markdown = "src-md"
source = "src"
types = "src"
test = "tests"

[adapters.ts]
enabled = true

[adapters.py]
enabled = false

[adapters.rs]
enabled = false

[quality.ts]
linter = "eslint"
fixer = "prettier --write"
test_runner = "vitest run"
required = ["node", "eslint", "prettier", "vitest"]
optional = []
```

## `[package]`

`[package]` は、パッケージ単位の mds の有効化を設定します。

| キー | 意味 |
| --- | --- |
| `enabled` | このパッケージを mds の対象にするかどうかを指定します。 |
| `allow_raw_source` | 生成対象外の生のソースを許可するかどうかを指定します。 |

通常は、mds の対象にしたいパッケージで `enabled = true` を指定します。

## `[roots]`

`[roots]` は、Markdown と生成先の場所を指定します。

| キー | 既定値 | 意味 |
| --- | --- | --- |
| `markdown` | `src-md` | 実装 Markdown を置く場所です。 |
| `source` | `src` | `Source` から生成するファイルの出力先です。 |
| `types` | `src` | `Types` から生成するファイルの出力先です。 |
| `test` | `tests` | `Test` から生成するファイルの出力先です。 |

生成先は、対象パッケージの内側である必要があります。パッケージの外に出る出力先は拒否されます。

## `[adapters]`

`[adapters]` は、対象言語の有効化を設定します。

| セクション | 対象言語 |
| --- | --- |
| `[adapters.ts]` | TypeScript |
| `[adapters.py]` | Python |
| `[adapters.rs]` | Rust |

利用しない言語は `enabled = false` にできます。

## 品質検査の設定

言語ごとの検査、修正、テストで使うコマンドは、品質検査用の設定で扱います。`mds init` で選択した quality tool は、`[quality.ts]`、`[quality.py]`、`[quality.rs]` に明示されます。

選択できる代表的な候補は次のとおりです。

| 言語 | 検査 | 修正 | テスト |
| --- | --- | --- | --- |
| TypeScript | ESLint、Biome | Prettier、Biome | Vitest、Jest |
| Python | Ruff | Ruff、Black | Pytest、unittest |
| Rust | Cargo Clippy | rustfmt | Cargo test、cargo-nextest |

使わない機能は `false` にできます。たとえば TypeScript の品質検査を使わない場合は次のように指定します。

```toml
[quality.ts]
linter = false
fixer = false
test_runner = false
required = []
optional = []
```

実行環境に必要なツールがない場合、`mds doctor` で確認できます。未選択のツールは不足扱いになりません。

## 設定の注意点

- 設定ファイル名は `mds.config.toml` に固定します。
- セクション名やキー名で mds の意味を自由に変更することはできません。
- 未対応の設定を指定した場合、警告として扱われることがあります。
- 生成先がパッケージの外に出る設定は拒否されます。
- 設定で自由度を増やすより、規約を保つことを優先します。

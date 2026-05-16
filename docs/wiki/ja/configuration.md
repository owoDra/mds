# 設定

このページでは、current な `mds.config.toml` surface を説明します。

## 最小例

```toml
[package]
enabled = true
allow_raw_source = false

[roots]
source_md = ".mds/source"
test_md = ".mds/test"
source_out = "src"
test_out = "tests"

[output]
source = "{source_out}/{module}.{ext}"
test = "{test_out}/{module}.test.{ext}"

[check]
legacy_tables = "warn"
unresolved_module_symbols = "warn"
implementation_section_only = true
split_source_and_test = true

[quality.ts]
linter = ""
fixer = "prettier --write"
test_runner = ""
required = ["node", "prettier"]
optional = []
```

## `[package]`

| キー | 意味 |
| --- | --- |
| `enabled` | package で mds を有効化する |
| `allow_raw_source` | 生成対象外の source file を generated output と共存させる |

## `[roots]`

`source_md` と `test_md` は canonical で固定です。

| キー | 意味 |
| --- | --- |
| `source_md` | `.mds/source` 固定 |
| `test_md` | `.mds/test` 固定 |
| `source_out` | generated source output の base directory |
| `test_out` | generated test output の base directory |

Markdown root の名前は変えず、必要な path 変更は output planning で行います。

## `[output]`

`[output]` は doc kind ごとの既定 output pattern を定義します。

| キー | 既定値 |
| --- | --- |
| `source` | `{source_out}/{module}.{ext}` |
| `test` | `{test_out}/{module}.test.{ext}` |

利用できる placeholder は次のとおりです。

- `{source_out}`
- `{test_out}`
- `{module}`
- `{ext}`
- `{{` と `}}` は literal brace

未知の placeholder は error になります。

## `[[output.override]]`

一部 module だけ別の naming rule が必要なときに使います。

```toml
[[output.override]]
match = "*"
kind = "test"
path = "{test_out}/test_{module}.{ext}"
```

| field | 意味 |
| --- | --- |
| `match` | logical module id に対する glob pattern |
| `kind` | `source` または `test` |
| `path` | `[output]` と同じ placeholder を使う置換 pattern |

最初に一致した override が使われます。

## `[check]`

`[check]` は authoring-v2 diagnostics を制御します。

| キー | 既定値 | 意味 |
| --- | --- | --- |
| `legacy_tables` | `warn` | 旧 metadata table pattern への warn / error |
| `unresolved_module_symbols` | `warn` | unresolved `[[module#symbol]]` の policy |
| `implementation_section_only` | `true` | executable section だけを生成元として扱う |
| `split_source_and_test` | `true` | source doc と test doc の責務混在を拒否する |

unresolved `[[module]]` は常に error です。

## `[quality.<lang>]`

`[quality.ts]`、`[quality.py]`、`[quality.rs]` などの per-language section で外部 tool を設定します。

| キー | 意味 |
| --- | --- |
| `linter` | `mds lint` で使う command |
| `fixer` | `mds lint --fix` で使う command |
| `test_runner` | `mds test` で使う command |
| `required` | package に必須の tool 一覧 |
| `optional` | 無くてもよいがあると便利な tool 一覧 |

使わない tool は `""` や空 list にします。

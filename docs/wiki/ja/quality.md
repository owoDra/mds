# 品質検査

このページでは、mds が authoring-v2 package をどう検証するかを説明します。

## 構造検証

`mds lint` は、外部 tool を起動する前に Markdown model を検証します。

確認する内容:

- 現在の doc kind に必要な section
- canonical `.mds/source` / `.mds/test`
- output planning が package の内側に収まっているか
- managed file を安全に上書きできるか
- package-local wiki-style link と symbol reference

## check policy

`[check]` ではいくつかの authoring-v2 diagnostics を制御します。

| キー | 既定値 | 意味 |
| --- | --- | --- |
| `legacy_tables` | `warn` | 旧 metadata table pattern への warn / error |
| `unresolved_module_symbols` | `warn` | unresolved `[[module#symbol]]` の policy |
| `implementation_section_only` | `true` | executable section だけを生成元として扱う |
| `split_source_and_test` | `true` | source doc と test doc の責務混在を拒否 |

unresolved `[[module]]` は常に error です。

## tool 実行

構造検証の後で、mds は `[quality.<lang>]` に設定した command を起動できます。

例:

- `mds lint` で linter
- `mds lint --fix` で fixer
- `mds typecheck` で typecheck command
- `mds test` で test runner
- `mds doctor` で runtime と tool の有無確認

## 自動修正の範囲

`mds lint --fix` は対象の code fence を書き換えます。周囲の prose や文書構造を自由に書き換えるものではありません。

## 推奨順序

1. `mds lint`
2. `mds build --dry-run`
3. `mds build`
4. `mds typecheck`
5. `mds test`
6. `mds doctor`

この順序にすると、authoring の問題と外部 toolchain の問題を切り分けやすくなります。
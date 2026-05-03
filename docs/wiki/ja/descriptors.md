# descriptor ガイド

このページでは、mds が使う built-in descriptor と workspace override の配置を説明します。

## ディレクトリ構成

| 範囲 | パス | 役割 |
| --- | --- | --- |
| built-in 言語 descriptor | `mds/core/src/descriptors/languages/base/` | file mapping や syntax などの純言語ルール |
| built-in framework overlay | `mds/core/src/descriptors/languages/overlays/` | Flutter や Rails などの framework 差分 |
| built-in quality tool | `mds/core/src/descriptors/tools/` | lint、typecheck、test の command manifest |
| built-in package manager | `mds/core/src/descriptors/package-managers/` | metadata file、lockfile、command 推薦 |
| workspace override | `.mds/descriptors/` | repository 単位の上書き |

## 言語 descriptor

言語 descriptor は次を定義します。

- Markdown file suffix の対応
- `Source`、`Types`、`Test` の出力 file rule
- import、top-level declaration、comment、doc comment の syntax hint
- quality default command と tool profile

framework overlay も同じ schema を使いますが、`languages/overlays/` に分離して置きます。

## Quality tool descriptor

quality tool descriptor は次のディレクトリに分かれます。

- `tools/lint/`
- `tools/typecheck/`
- `tools/test/`

各 descriptor は command prefix と、次の実行情報を結び付けます。

- input mode
- output mode
- diagnostic capture regex

新しい runner や parser を足す場合は TOML を追加します。

## Imports table 形

現在の canonical Imports columns は次です。

- `Kind`
- `From`
- `Target`
- `Symbols`
- `Via`
- `Summary`
- `Code`

`Code` は descriptor ベース renderer が全言語を吸収しきるまでの fallback として残します。

## Package manager descriptor

package manager descriptor は次を定義します。

- 検出に必要な metadata file
- 優先度に使う lockfile
- install、build、typecheck、lint、test の推薦 command
- `mds init` と package sync が使う metadata reader kind

現在の built-in には npm、pnpm、yarn、bun、cargo、uv、poetry、bundler、pub、Flutter pub、dotnet、CMake、Meson、Conan、vcpkg、Zig などがあります。

## Workspace override

workspace 側の override は次に置きます。

- `.mds/descriptors/languages/`
- `.mds/descriptors/tools/`
- `.mds/descriptors/package-managers/`

workspace descriptor は id または alias 単位で built-in を上書きします。
# descriptor ガイド

このページでは、mds が使う built-in descriptor 群の配置、schema、設定項目、現在の built-in inventory をまとめます。

## ディレクトリ構成

| 範囲 | パス | 役割 |
| --- | --- | --- |
| built-in 言語 descriptor | `mds/core/src/descriptors/languages/base/` | file mapping、import style、syntax hint、quality default などの純言語ルール |
| built-in framework overlay | `mds/core/src/descriptors/languages/overlays/` | Angular、Flutter、Rails など framework 固有の差分 |
| built-in linter manifest | `mds/core/src/descriptors/linters/` | command prefix と入出力モード、diagnostic parser |
| built-in quality tool manifest | `mds/core/src/descriptors/tools/` | lint / typecheck / test runner の個別 manifest |
| built-in package manager | `mds/core/src/descriptors/package-managers/` | metadata file、lockfile、reader kind、推薦 command |
| workspace override | `.mds/descriptors/` | repository 単位の上書き |

## 共通の見方

- `id`: descriptor の canonical name。
- `aliases`: 補助名。CLI や config の別名解決に使う。
- `match_suffixes` / `match_prefixes`: file suffix や command prefix の match 条件。
- `quality_defaults.*`: 言語ごとの既定 command。
- `tool_profiles.*`: `mds init` や `mds config` が候補として提示する named profile。
- `behavior.*`: tool 実行方法。`input`、`output`、`append_file_arg`、`diagnostics` を持つ。

## Imports table 形

現在の canonical Imports columns は次です。

- `From`
- `Target`
- `Symbols`
- `Via`
- `Summary`
- `Reference`

使い分けは次のとおりです。

- `From`: `builtin` / `internal` / `external` / `workspace` など依存の出所。
- `Target`: module / package / namespace 単位。`mds_core::config` のように module までで止める。
- `Symbols`: import される function / class / type / module 名。`merge_config_file` のような最終要素を置く。
- `Via`: type-only、namespace、alias など import style の補助情報。不要なら `-`。
- `Summary`: 人間向けの短い説明。不要なら `-`。
- `Reference`: `internal` と解決可能な `external` なら Markdown location を置く。不要なら `-`。

## 言語 descriptor の schema

### 主要 field

| field | 説明 |
| --- | --- |
| `id` | 言語 id。例: `ts`, `py`, `rs` |
| `aliases[]` | 同義語。例: `typescript`, `python`, `rust` |
| `match_suffixes[]` | `foo.ts.md` の `ts` 部分に相当する suffix |
| `language.primary_ext` | primary output extension |
| `files.source.*` | `Source` section の出力 rule |
| `files.types.*` | `Types` section の出力 rule |
| `files.test.*` | `Test` section の出力 rule |
| `special_files[]` | `build.rs` や framework 固有 file の例外 rule |
| `imports.style` | descriptor 駆動の import renderer style |
| `syntax.imports[]` | import line 判定用の starts-with / contains 条件 |
| `syntax.top_level_keywords[]` | top-level declaration 判定に使う prefix |
| `syntax.comment_prefixes[]` | コメント行の prefix |
| `syntax.doc_comment_prefixes[]` | doc comment 行の prefix |
| `syntax.doc_string_delimiters[]` | Python などの doc string delimiter |
| `scaffold.fence_lang` | `mds new` が使う code fence label |
| `scaffold.source_body` | `mds new` が置く default source body |
| `tooling.typecheck/lint/fix/test` | command 固有の tool behavior override |
| `quality_defaults.typecheck/lint/fix/test` | 言語 default command |
| `tool_profiles.*` | named quality profile |

### `files.*` rule の field

| field | 説明 |
| --- | --- |
| `strip_lang_ext` | Markdown 名から `.ts` などの言語 suffix を落とすか |
| `prefix` | 出力 file 名の prefix |
| `suffix` | 出力 file 名の suffix |
| `extension` | 出力 extension |

### `imports.style` の built-in 値

| style | 主な対象 |
| --- | --- |
| `typescript` | TS / JS / JSX / TSX / Angular / Astro / Svelte / Vue |
| `python` | Python |
| `rust` | Rust |
| `go` | Go |
| `java` | Java |
| `csharp` | C# |
| `c-include` | C / C++ |
| `dart` | Dart / Flutter |
| `ruby` | Ruby / Rails |
| `scss` | SCSS |
| `zig` | Zig |
| `mojo` | Mojo |
| `none` | HTML / CSS / SQL など import section を使わない言語 |

## built-in 言語 descriptor 一覧

### base language

| id | path | 主な用途 |
| --- | --- | --- |
| `c` | `languages/base/c.toml` | C source / header 生成、`c-include` import |
| `cpp` | `languages/base/cpp.toml` | C++ source / header 生成、`c-include` import |
| `cs` | `languages/base/cs.toml` | C# source / type / test 生成 |
| `css` | `languages/base/css.toml` | CSS source 生成、import style は `none` |
| `dart` | `languages/base/dart.toml` | Dart source / test 生成 |
| `go` | `languages/base/go.toml` | Go source / test 生成 |
| `html` | `languages/base/html.toml` | HTML source 生成、import style は `none` |
| `java` | `languages/base/java.toml` | Java source / test 生成 |
| `js` | `languages/base/js.toml` | JavaScript source / test 生成 |
| `mojo` | `languages/base/mojo.toml` | Mojo source / test 生成 |
| `py` | `languages/base/py.toml` | Python `.py` / `.pyi` / `test_*.py` 生成 |
| `rs` | `languages/base/rs.toml` | Rust source / types / test 生成 |
| `ruby` | `languages/base/ruby.toml` | Ruby source / test 生成 |
| `scss` | `languages/base/scss.toml` | SCSS source 生成 |
| `sql` | `languages/base/sql.toml` | SQL source 生成、import style は `none` |
| `ts` | `languages/base/ts.toml` | TypeScript source / types / test 生成 |
| `zig` | `languages/base/zig.toml` | Zig source / test 生成 |

### framework overlay

| id | path | 差分 |
| --- | --- | --- |
| `angular` | `languages/overlays/angular.toml` | Angular 向け TypeScript overlay |
| `astro` | `languages/overlays/astro.toml` | Astro component 向け overlay |
| `flutter` | `languages/overlays/flutter.toml` | Flutter widget / test naming |
| `jsx` | `languages/overlays/jsx.toml` | JSX component 出力 |
| `rails` | `languages/overlays/rails.toml` | Rails 向け Ruby overlay |
| `svelte` | `languages/overlays/svelte.toml` | Svelte component 出力 |
| `tsx` | `languages/overlays/tsx.toml` | TSX component 出力 |
| `vue` | `languages/overlays/vue.toml` | Vue SFC 出力 |

## linter manifest の schema

| field | 説明 |
| --- | --- |
| `id` | manifest id |
| `match_prefixes[]` | command 判定に使う prefix |
| `behavior.input` | `tempfile` / `stdin` / `inline` |
| `behavior.output` | `none` / `stdout` / `tempfile` |
| `behavior.append_file_arg` | file path を positional arg へ付けるか |
| `behavior.diagnostics[]` | error parser 定義 |
| `diagnostics[].pattern` | regex |
| `diagnostics[].path_group` | path capture group 名 |
| `diagnostics[].line_group` | line capture group 名 |
| `diagnostics[].column_group` | column capture group 名 |
| `diagnostics[].message_group` | message capture group 名 |
| `diagnostics[].severity` | default severity |
| `diagnostics[].line_offset` | line offset 補正 |

### built-in linter manifest

| id | path | 用途 |
| --- | --- | --- |
| `biome-lint` | `linters/biome-lint.toml` | Biome lint parser |
| `clippy` | `linters/clippy.toml` | Rust clippy parser |
| `eslint` | `linters/eslint.toml` | ESLint parser |
| `ruff-check` | `linters/ruff-check.toml` | Ruff parser |

## quality tool manifest 一覧

### lint

| id | path | 用途 |
| --- | --- | --- |
| `clang-tidy` | `tools/lint/clang-tidy.toml` | C/C++ lint |
| `dart-analyze` | `tools/lint/dart-analyze.toml` | Dart lint |
| `dotnet-format` | `tools/lint/dotnet-format.toml` | C# lint / format |
| `rubocop` | `tools/lint/rubocop.toml` | Ruby lint |
| `sqlfluff` | `tools/lint/sqlfluff.toml` | SQL lint |

### typecheck

| id | path | 用途 |
| --- | --- | --- |
| `cargo-check` | `tools/typecheck/cargo-check.toml` | Rust typecheck |
| `dart-analyze` | `tools/typecheck/dart-analyze.toml` | Dart typecheck |
| `dotnet-build` | `tools/typecheck/dotnet-build.toml` | C# compile/typecheck |
| `flutter-analyze` | `tools/typecheck/flutter-analyze.toml` | Flutter analyze |
| `mypy` | `tools/typecheck/mypy.toml` | Python typecheck |
| `steep` | `tools/typecheck/steep.toml` | Ruby typecheck |
| `tsc` | `tools/typecheck/tsc.toml` | TypeScript typecheck |
| `typeprof` | `tools/typecheck/typeprof.toml` | Ruby type profiler |
| `zig-build` | `tools/typecheck/zig-build.toml` | Zig build/typecheck |

### test

| id | path | 用途 |
| --- | --- | --- |
| `cargo-test` | `tools/test/cargo-test.toml` | Rust library test |
| `ctest` | `tools/test/ctest.toml` | C/C++ test |
| `dart-test` | `tools/test/dart-test.toml` | Dart test |
| `dotnet-test` | `tools/test/dotnet-test.toml` | C# test |
| `flutter-test` | `tools/test/flutter-test.toml` | Flutter widget / unit test |
| `jest` | `tools/test/jest.toml` | JavaScript / TypeScript test |
| `nextest` | `tools/test/nextest.toml` | Rust nextest |
| `pytest` | `tools/test/pytest.toml` | Python test |
| `rspec` | `tools/test/rspec.toml` | Ruby test |
| `unittest` | `tools/test/unittest.toml` | Python stdlib test |
| `vitest` | `tools/test/vitest.toml` | TypeScript / JavaScript test |
| `zig-test` | `tools/test/zig-test.toml` | Zig test |

## package manager descriptor の schema

| field | 説明 |
| --- | --- |
| `id` | package manager id |
| `aliases[]` | 補助名 |
| `display_name` | 表示名 |
| `lang` | 主対象言語 |
| `metadata_files[]` | package 検出に使う file pattern |
| `lockfiles[]` | 優先順位付けに使う lockfile |
| `metadata_reader` | `package_json`、`pyproject`、`cargo_toml` などの reader kind |
| `commands.install/build/typecheck/lint/test` | 推薦 command |

### built-in package manager 一覧

| id | path | 主対象 |
| --- | --- | --- |
| `bundler` | `package-managers/bundler.toml` | Ruby |
| `bun` | `package-managers/bun.toml` | JavaScript / TypeScript |
| `cargo` | `package-managers/cargo.toml` | Rust |
| `cmake` | `package-managers/cmake.toml` | C / C++ |
| `conan` | `package-managers/conan.toml` | C / C++ |
| `dotnet` | `package-managers/dotnet.toml` | C# |
| `flutter-pub` | `package-managers/flutter-pub.toml` | Flutter |
| `hatch` | `package-managers/hatch.toml` | Python |
| `meson` | `package-managers/meson.toml` | C / C++ |
| `npm` | `package-managers/npm.toml` | JavaScript / TypeScript |
| `pdm` | `package-managers/pdm.toml` | Python |
| `pnpm` | `package-managers/pnpm.toml` | JavaScript / TypeScript |
| `poetry` | `package-managers/poetry.toml` | Python |
| `pub` | `package-managers/pub.toml` | Dart |
| `uv` | `package-managers/uv.toml` | Python |
| `vcpkg` | `package-managers/vcpkg.toml` | C / C++ |
| `yarn` | `package-managers/yarn.toml` | JavaScript / TypeScript |
| `zig-build` | `package-managers/zig-build.toml` | Zig |

## workspace override

workspace 側の override は次に置きます。

- `.mds/descriptors/languages/`
- `.mds/descriptors/tools/`
- `.mds/descriptors/package-managers/`

override の運用ルールは次です。

- built-in と同じ `id` で置くと上書きになる。
- `aliases` だけを増やしたい場合も full TOML を置く。
- `languages/overlays/` 相当の差分も `languages/` に flat に置く。
- command manifest は `match_prefixes` が衝突すると prefix の長いものを優先する設計を前提にする。
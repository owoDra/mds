# Descriptor Guide

This page explains the built-in descriptor directories used by mds, the schema for each descriptor family, and the current built-in inventory.

## Directory Layout

| Scope | Path | Purpose |
| --- | --- | --- |
| Built-in language descriptors | `mds/core/src/descriptors/languages/base/` | File mapping, import style, syntax hints, and language-level quality defaults |
| Built-in framework overlays | `mds/core/src/descriptors/languages/overlays/` | Framework-specific deltas such as Flutter or Rails |
| Built-in linter manifests | `mds/core/src/descriptors/linters/` | Command-prefix matching, IO mode, and diagnostic parsing |
| Built-in quality tool manifests | `mds/core/src/descriptors/tools/` | Lint, typecheck, and test runner manifests |
| Built-in package managers | `mds/core/src/descriptors/package-managers/` | Metadata files, lockfiles, reader kind, and recommended commands |
| Workspace overrides | `.mds/descriptors/` | Repository-local overrides |

## How To Read Descriptor Fields

- `id`: canonical descriptor name.
- `aliases`: alternative names accepted by config and CLI surfaces.
- `match_suffixes` / `match_prefixes`: file suffix or command prefix match rules.
- `quality_defaults.*`: default commands for a language.
- `tool_profiles.*`: named quality profiles exposed by `mds init` and config.
- `behavior.*`: tool execution details such as `input`, `output`, `append_file_arg`, and `diagnostics`.

## Imports Table Shape

The current canonical Imports columns are:

- `From`
- `Target`
- `Symbols`
- `Via`
- `Summary`
- `Reference`

Use them as follows.

- `From`: dependency origin such as `builtin`, `internal`, `external`, or `workspace`.
- `Target`: the module / package / namespace boundary, for example `mds_core::config`.
- `Symbols`: imported functions, classes, types, or modules, for example `merge_config_file`.
- `Via`: import-style modifier such as type-only, namespace, or alias. Use `-` when unused.
- `Summary`: short human-facing explanation. Use `-` when unnecessary.
- `Reference`: Markdown location for resolvable `internal` and `external` dependencies. Use `-` when unavailable.

## Language Descriptor Schema

### Primary fields

| Field | Meaning |
| --- | --- |
| `id` | Language id such as `ts`, `py`, or `rs` |
| `aliases[]` | Additional accepted names |
| `match_suffixes[]` | Markdown suffixes such as `ts` in `greet.ts.md` |
| `language.primary_ext` | Primary output extension |
| `files.source.*` | Output rule for `Source` |
| `files.types.*` | Output rule for `Types` |
| `files.test.*` | Output rule for `Test` |
| `special_files[]` | Exceptions such as `build.rs` |
| `imports.style` | Descriptor-driven import rendering style |
| `syntax.imports[]` | Import-line detection rules |
| `syntax.top_level_keywords[]` | Prefixes used for top-level declaration checks |
| `syntax.comment_prefixes[]` | Comment prefixes |
| `syntax.doc_comment_prefixes[]` | Doc-comment prefixes |
| `syntax.doc_string_delimiters[]` | Doc-string delimiters for languages such as Python |
| `scaffold.fence_lang` | Fence language used by `mds new` |
| `scaffold.source_body` | Default source body used by `mds new` |
| `tooling.typecheck/lint/fix/test` | Tool-behavior override for a specific operation |
| `quality_defaults.typecheck/lint/fix/test` | Language default commands |
| `tool_profiles.*` | Named quality profiles |

### `files.*` rule fields

| Field | Meaning |
| --- | --- |
| `strip_lang_ext` | Whether the language suffix should be removed from the output file name |
| `prefix` | Output file-name prefix |
| `suffix` | Output file-name suffix |
| `extension` | Output extension |

### Built-in `imports.style` values

| Style | Main targets |
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
| `none` | HTML / CSS / SQL and similar non-import-driven formats |

## Built-in Language Descriptors

### Base language descriptors

| id | Path | Main use |
| --- | --- | --- |
| `c` | `languages/base/c.toml` | C source / header generation with `c-include` imports |
| `cpp` | `languages/base/cpp.toml` | C++ source / header generation with `c-include` imports |
| `cs` | `languages/base/cs.toml` | C# source / type / test generation |
| `css` | `languages/base/css.toml` | CSS source generation with `none` import style |
| `dart` | `languages/base/dart.toml` | Dart source / test generation |
| `go` | `languages/base/go.toml` | Go source / test generation |
| `html` | `languages/base/html.toml` | HTML source generation with `none` import style |
| `java` | `languages/base/java.toml` | Java source / test generation |
| `js` | `languages/base/js.toml` | JavaScript source / test generation |
| `mojo` | `languages/base/mojo.toml` | Mojo source / test generation |
| `py` | `languages/base/py.toml` | Python `.py` / `.pyi` / `test_*.py` generation |
| `rs` | `languages/base/rs.toml` | Rust source / types / test generation |
| `ruby` | `languages/base/ruby.toml` | Ruby source / test generation |
| `scss` | `languages/base/scss.toml` | SCSS source generation |
| `sql` | `languages/base/sql.toml` | SQL source generation with `none` import style |
| `ts` | `languages/base/ts.toml` | TypeScript source / types / test generation |
| `zig` | `languages/base/zig.toml` | Zig source / test generation |

### Framework overlays

| id | Path | Delta |
| --- | --- | --- |
| `angular` | `languages/overlays/angular.toml` | Angular-specific TypeScript overlay |
| `astro` | `languages/overlays/astro.toml` | Astro component overlay |
| `flutter` | `languages/overlays/flutter.toml` | Flutter widget / test naming rules |
| `jsx` | `languages/overlays/jsx.toml` | JSX component output |
| `rails` | `languages/overlays/rails.toml` | Rails-oriented Ruby overlay |
| `svelte` | `languages/overlays/svelte.toml` | Svelte component output |
| `tsx` | `languages/overlays/tsx.toml` | TSX component output |
| `vue` | `languages/overlays/vue.toml` | Vue SFC output |

## Linter Manifest Schema

| Field | Meaning |
| --- | --- |
| `id` | Manifest id |
| `match_prefixes[]` | Command prefixes that resolve to this manifest |
| `behavior.input` | `tempfile`, `stdin`, or `inline` |
| `behavior.output` | `none`, `stdout`, or `tempfile` |
| `behavior.append_file_arg` | Whether the file path is appended as a positional argument |
| `behavior.diagnostics[]` | Diagnostic parser definitions |
| `diagnostics[].pattern` | Regex |
| `diagnostics[].path_group` | Path capture group name |
| `diagnostics[].line_group` | Line capture group name |
| `diagnostics[].column_group` | Column capture group name |
| `diagnostics[].message_group` | Message capture group name |
| `diagnostics[].severity` | Default severity |
| `diagnostics[].line_offset` | Optional line offset correction |

### Built-in linter manifests

| id | Path | Use |
| --- | --- | --- |
| `biome-lint` | `linters/biome-lint.toml` | Biome lint parsing |
| `clippy` | `linters/clippy.toml` | Rust clippy parsing |
| `eslint` | `linters/eslint.toml` | ESLint parsing |
| `ruff-check` | `linters/ruff-check.toml` | Ruff parsing |

## Quality Tool Manifest Inventory

### lint

| id | Path | Use |
| --- | --- | --- |
| `clang-tidy` | `tools/lint/clang-tidy.toml` | C / C++ lint |
| `dart-analyze` | `tools/lint/dart-analyze.toml` | Dart lint |
| `dotnet-format` | `tools/lint/dotnet-format.toml` | C# lint / format |
| `rubocop` | `tools/lint/rubocop.toml` | Ruby lint |
| `sqlfluff` | `tools/lint/sqlfluff.toml` | SQL lint |

### typecheck

| id | Path | Use |
| --- | --- | --- |
| `cargo-check` | `tools/typecheck/cargo-check.toml` | Rust typecheck |
| `dart-analyze` | `tools/typecheck/dart-analyze.toml` | Dart typecheck |
| `dotnet-build` | `tools/typecheck/dotnet-build.toml` | C# compile / typecheck |
| `flutter-analyze` | `tools/typecheck/flutter-analyze.toml` | Flutter analyze |
| `mypy` | `tools/typecheck/mypy.toml` | Python typecheck |
| `steep` | `tools/typecheck/steep.toml` | Ruby typecheck |
| `tsc` | `tools/typecheck/tsc.toml` | TypeScript typecheck |
| `typeprof` | `tools/typecheck/typeprof.toml` | Ruby type profiler |
| `zig-build` | `tools/typecheck/zig-build.toml` | Zig build / typecheck |

### test

| id | Path | Use |
| --- | --- | --- |
| `cargo-test` | `tools/test/cargo-test.toml` | Rust library test |
| `ctest` | `tools/test/ctest.toml` | C / C++ test |
| `dart-test` | `tools/test/dart-test.toml` | Dart test |
| `dotnet-test` | `tools/test/dotnet-test.toml` | C# test |
| `flutter-test` | `tools/test/flutter-test.toml` | Flutter widget / unit tests |
| `jest` | `tools/test/jest.toml` | JavaScript / TypeScript test |
| `nextest` | `tools/test/nextest.toml` | Rust nextest |
| `pytest` | `tools/test/pytest.toml` | Python test |
| `rspec` | `tools/test/rspec.toml` | Ruby test |
| `unittest` | `tools/test/unittest.toml` | Python stdlib test |
| `vitest` | `tools/test/vitest.toml` | TypeScript / JavaScript test |
| `zig-test` | `tools/test/zig-test.toml` | Zig test |

## Package Manager Descriptor Schema

| Field | Meaning |
| --- | --- |
| `id` | Package manager id |
| `aliases[]` | Alternative names |
| `display_name` | UI-facing display name |
| `lang` | Primary target language |
| `metadata_files[]` | File patterns used for package detection |
| `lockfiles[]` | Lockfiles used to rank matches |
| `metadata_reader` | Reader kind such as `package_json`, `pyproject`, or `cargo_toml` |
| `commands.install/build/typecheck/lint/test` | Recommended commands |

### Built-in package managers

| id | Path | Primary target |
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

## Workspace Overrides

Place overrides in one of the following directories.

- `.mds/descriptors/languages/`
- `.mds/descriptors/tools/`
- `.mds/descriptors/package-managers/`

Operational rules:

- Reuse the same `id` to override a built-in descriptor.
- If you only want to add aliases, still provide the full TOML file.
- Overlay-like language differences also live under `.mds/descriptors/languages/`.
- When command manifests overlap, design `match_prefixes` with specificity in mind so the intended manifest wins.
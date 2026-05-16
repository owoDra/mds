# Configuration

> *This page was translated from [Japanese](../ja/configuration.md) by AI.*

This page describes the current `mds.config.toml` surface.

## Minimal Example

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

| Key | Meaning |
| --- | --- |
| `enabled` | Enables mds for the package |
| `allow_raw_source` | Allows non-generated source files to coexist with generated outputs |

## `[roots]`

`source_md` and `test_md` are canonical and fixed.

| Key | Meaning |
| --- | --- |
| `source_md` | Must be `.mds/source` |
| `test_md` | Must be `.mds/test` |
| `source_out` | Base directory for generated source outputs |
| `test_out` | Base directory for generated test outputs |

Use output planning to change file locations. Do not rename the authoring roots.

## `[output]`

`[output]` defines the default output pattern for each doc kind.

| Key | Default |
| --- | --- |
| `source` | `{source_out}/{module}.{ext}` |
| `test` | `{test_out}/{module}.test.{ext}` |

Supported placeholders are:

- `{source_out}`
- `{test_out}`
- `{module}`
- `{ext}`
- `{{` and `}}` for literal braces

Unknown placeholders are rejected.

## `[[output.override]]`

Use overrides when a subset of modules needs a different path shape.

```toml
[[output.override]]
match = "*"
kind = "test"
path = "{test_out}/test_{module}.{ext}"
```

| Field | Meaning |
| --- | --- |
| `match` | Glob pattern matched against the logical module id |
| `kind` | `source` or `test` |
| `path` | Replacement pattern using the same placeholders as `[output]` |

The first matching override wins.

## `[check]`

`[check]` controls authoring-v2 diagnostics.

| Key | Default | Meaning |
| --- | --- | --- |
| `legacy_tables` | `warn` | Warns or errors on old metadata tables |
| `unresolved_module_symbols` | `warn` | Policy for unresolved `[[module#symbol]]` |
| `implementation_section_only` | `true` | Only generation sections are treated as executable output sources |
| `split_source_and_test` | `true` | Rejects mixing source behavior and executable tests in the same doc kind |

`[[module]]` unresolved links are always errors.

## `[quality.<lang>]`

Use per-language sections such as `[quality.ts]`, `[quality.py]`, and `[quality.rs]` to configure external tools.

| Key | Meaning |
| --- | --- |
| `linter` | Command used by `mds lint` |
| `fixer` | Command used by `mds lint --fix` |
| `test_runner` | Command used by `mds test` |
| `required` | Tools that must exist for the package |
| `optional` | Tools that may improve the workflow but are not required |

Set unused tools to `""` or an empty list as appropriate.

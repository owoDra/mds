# Troubleshooting

> *This page was translated from [Japanese](../ja/troubleshooting.md) by AI.*

This page lists the most common current failures when using mds.

## No mds package found

Check the following:

- `mds.config.toml` exists
- `[package].enabled = true`
- the `--package` path points at the package root
- `package.md` and package-manager metadata exist where expected

## Wrong authoring roots

Current packages use canonical authoring roots only.

- `source_md` must be `.mds/source`
- `test_md` must be `.mds/test`

If you need a different output layout, change `[output]` or `[[output.override]]`, not the Markdown roots.

## Missing required sections

Check the expected shape for the doc kind you are editing.

- source docs: `Purpose`, `Contract`, `API`, `Source`, `Cases`
- test docs: `Purpose`, `Covers`, `Cases`, `Test`
- overview docs: `Purpose`, `Architecture`, `Rules`

## Source/test mixing or legacy table warnings

If `split_source_and_test = true`, keep executable source behavior in source docs and executable tests in test docs. If you see `legacy_tables` warnings, remove old metadata-table patterns and describe API intent in prose instead.

## Unresolved wiki-style links

- `[[module]]` must resolve to a package-local logical module id.
- `[[module#symbol]]` must resolve to Markdown-native symbols, such as shared definitions or names described in prose.

Check the module id derived from the source/test doc path and the exact symbol spelling.

## Unexpected output path

Inspect:

- `[roots].source_out`
- `[roots].test_out`
- `[output]`
- `[[output.override]]`

Run `mds build --dry-run` to confirm the planned destination before writing files.

## Cannot overwrite a file

mds only overwrites files that already have the managed header. If the target path contains a handwritten file, move that file or change the output pattern.

## Required tools are missing

Run:

```bash
mds doctor --package ./path/to/package
```

Then confirm the relevant `[quality.<lang>]` section only names tools you actually want to require.
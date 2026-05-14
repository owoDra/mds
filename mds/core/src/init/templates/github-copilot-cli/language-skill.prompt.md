---
mode: agent
description: "Author mds {{LANG_ID}} markdown with correct Imports and Exports"
mds-managed: true
---

# mds {{LANG_ID}} Authoring

Use this when editing `.mds/source/**/*.{{LANG_SUFFIXES}}.md` for descriptor `{{LANG_ID}}`.

## Best Practices

- Put dependencies in `## {{IMPORTS}}`; do not write import/use/require/include/using statements in code fences.
- Put public API metadata in `## {{EXPORTS}}` and add H5 anchors for imported shared definitions.
- Keep source md in spec state without generated code until implementation is ready; adding `{{SOURCE}}` code makes it impl state.
- Do not use `-` for `{{EXPORTS}}.{{SUMMARY}}`; explain the definition.
- Keep one feature per implementation md and one top-level logical unit per code fence.
- Keep doc comments and docstrings in Markdown prose outside generated code fences.

## Imports

```markdown
| {{FROM}} | {{TARGET}} | {{SYMBOLS}} | {{VIA}} | {{SUMMARY}} | {{REFERENCE}} |
| --- | --- | --- | --- | --- | --- |
{{IMPORT_ROW}}
```

Generated import example:

```{{FENCE_LANG}}
{{GENERATED_IMPORT}}
```

## Exports

```markdown
| {{NAME}} | {{VISIBILITY}} | {{SUMMARY}} |
| --- | --- | --- |
| exported_name | public | Stable public entrypoint. |

##### exported-name

Link target for other mds files.
```

Run `mds lint`, `mds build --dry-run`, `mds build`, and `mds test` after edits.

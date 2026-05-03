---
mds-managed: true
---

Generate code from markdown sources. Always run `mds lint` first, then preview with `--dry-run`.

```sh
mds lint && mds build --dry-run
```

If the dry-run output looks correct, apply the generation:

```sh
mds build
```

After generation, verify with:

```sh
mds test
```

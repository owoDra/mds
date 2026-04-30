---
mds-managed: true
---

Fix markdown formatting issues and validate quality.

```sh
mds lint --fix --check
```

This command:
- Fixes auto-correctable formatting issues in code blocks
- Reports remaining violations that need manual attention
- Validates code block structure and cross-references

---
mds-managed: true
---

Validate mds markdown structure and references. Run this before making generation-sensitive changes.

```sh
mds check
```

If errors are found, read the diagnostic output and fix the referenced markdown files. Common issues:
- Missing code blocks in implementation markdown
- Broken cross-references between markdown files
- Code fence language not matching file extension

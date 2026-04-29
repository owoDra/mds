---
mds-managed: true
---

Validate mds markdown structure and references. Run this before making generation-sensitive changes.

```sh
mds check
```

If errors are found, read the diagnostic output and fix the referenced markdown files. Common issues:
- Missing `Uses` table entries for imports
- Broken cross-references between markdown files
- Schema violations in frontmatter or tables

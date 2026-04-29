---
mode: agent
description: "Validate mds markdown structure and references before making generation-sensitive changes"
mds-managed: true
---

Run `mds check` to validate the markdown structure. Report any errors found and suggest fixes.

If errors are found:
1. Read the diagnostic output
2. Identify the referenced markdown files
3. Fix missing `Uses` table entries, broken cross-references, or schema violations
4. Re-run `mds check` to confirm the fix

---
mode: agent
description: "Validate mds markdown structure and references before making generation-sensitive changes"
mds-managed: true
---

Run `mds lint` to validate the markdown structure. Report any errors found and suggest fixes.

If errors are found:
1. Read the diagnostic output
2. Identify the referenced markdown files
3. Fix missing code blocks, language mismatches, or broken references
4. Re-run `mds lint` to confirm the fix

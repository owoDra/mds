---
mode: agent
description: "Generate code from mds markdown sources with validation"
mds-managed: true
---

Generate code from markdown sources with full validation:

1. Run `mds check` to ensure markdown is valid
2. Run `mds build --dry-run` to preview output
3. If preview is correct, run `mds build` to generate
4. Run `mds test` to verify correctness

# Cursor

[Cursor](https://cursor.sh) supports project-level rules in `.cursor/rules/` using `.mdc` files. Global rules must be added via the Cursor UI.

---

## Install workspace rules

```bash
zrk install --target cursor
```

This writes `.mdc` files to `.cursor/rules/` in your project.

## Install global rules (manual)

Cursor does not have a filesystem path for global rules — they are configured in the Cursor UI.

`zrk install-global --target cursor` will print the content to copy:

```bash
zrk install-global --target cursor
# ⚠ Cursor: 'review-roles.mdc' requires manual setup (global rules are UI-only)
# ⚠ Cursor: 'review-prompting.mdc' requires manual setup (global rules are UI-only)
# ...
```

**To add global rules in Cursor:**

1. Open Cursor → Settings → Rules
2. Click "Add Rule"
3. Paste the content printed by `zrk install-global --target cursor`
4. Repeat for each file

---

## Install paths

| Scope     | Path                 |
| --------- | -------------------- |
| Global    | (manual — Cursor UI) |
| Workspace | `.cursor/rules/`     |

## File format

Cursor uses `.mdc` files with YAML frontmatter:

```markdown
---
description: zrk: review-roles
globs:
name: review-roles
---

# Review Roles

...
```

---

## Verification

```bash
ls .cursor/rules/
# prep-review.mdc  pack-materials.mdc  project-context.mdc

zrk status --target cursor
```

---

## Trigger sentences

```
create review materials for the last 3 commits
pack the auth changes for review
```

---

## Next

- [Back to agent overview](overview.md)
- [First review walkthrough](../getting-started/first-review.md)

# Windsurf

[Windsurf](https://codeium.com/windsurf) supports project rules in `.windsurf/rules/`. Global rules are UI-only.

---

## Install workspace rules

```bash
zrk install --target windsurf
```

## Install global rules (manual)

Same as Cursor — global rules require the Windsurf UI.

```bash
zrk install-global --target windsurf
# ⚠ Windsurf: 'review-roles.md' requires manual setup (global rules are UI-only)
```

Add each printed file's content in Windsurf → Settings → Rules.

---

## Install paths

| Scope     | Path                   |
| --------- | ---------------------- |
| Global    | (manual — Windsurf UI) |
| Workspace | `.windsurf/rules/`     |

## File format

Windsurf uses `.md` files with an HTML comment trigger header:

```markdown
<!-- trigger: review-roles -->

# Review Roles

...
```

---

## Verification

```bash
ls .windsurf/rules/
# prep-review.md  pack-materials.md  project-context.md

zrk status --target windsurf
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

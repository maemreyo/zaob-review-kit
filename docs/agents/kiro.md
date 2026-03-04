# Kiro

[Kiro](https://kiro.dev) uses a "steering" system: markdown files in `~/.kiro/steering/` (global) or `.kiro/steering/` (workspace) that are always included in the agent's context.

---

## Install

```bash
zrk install-all --target kiro
```

## Install paths

| Scope     | Path                |
| --------- | ------------------- |
| Global    | `~/.kiro/steering/` |
| Workspace | `.kiro/steering/`   |

## File format

Kiro files use `.md` with a YAML frontmatter block:

```markdown
---
description: zrk: review-roles
globs:
name: review-roles
---

# Review Roles

When reviewing code, adopt these perspectives...
```

zrk adds this frontmatter automatically when installing.

---

## Verification

After install, Kiro's steering panel should show the installed files. Check:

```bash
ls ~/.kiro/steering/
# review-roles.md  review-prompting.md  review-ignore.md  review-memory.md

ls .kiro/steering/
# prep-review.md  pack-materials.md  project-context.md
```

Check status:

```bash
zrk status --target kiro
```

---

## Trigger sentences

In Kiro's chat:

```
tạo materials để review 3 commit gần nhất
create review materials for the last 3 commits
pack the auth changes for review
review toàn bộ thay đổi trong PR #42
```

---

## Next

- [Back to agent overview](overview.md)
- [First review walkthrough](../getting-started/first-review.md)

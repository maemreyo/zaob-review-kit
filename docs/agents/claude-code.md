# Claude Code

[Claude Code](https://docs.anthropic.com/claude-code) is Anthropic's official CLI for Claude. It supports custom commands via markdown files in `~/.claude/commands/` (global) or `.claude/commands/` (workspace).

---

## Install

```bash
zrk install-all --target claude-code
```

## Install paths

| Scope     | Path                             |
| --------- | -------------------------------- |
| Global    | `~/.claude/commands/review-kit/` |
| Workspace | `.claude/commands/review-kit/`   |

## File format

Claude Code uses `.md` files with a YAML frontmatter `description` field:

```markdown
---
description: zrk: prep-review
globs:
name: prep-review
---

# Prep Review

Prepare review materials for uploading to Claude.ai...
```

---

## Trigger

Claude Code workflows are triggered via natural language in the agent's chat. After install:

```
tạo materials để review 3 commit gần nhất
create review materials for the last 3 commits
prep review for the auth changes
```

---

## Verification

```bash
ls ~/.claude/commands/review-kit/
# review-roles.md  review-prompting.md  review-ignore.md  review-memory.md

ls .claude/commands/review-kit/
# prep-review.md  pack-materials.md  project-context.md

zrk status --target claude-code
```

---

## Next

- [Back to agent overview](overview.md)
- [First review walkthrough](../getting-started/first-review.md)

# Multiple Agents

You can install zrk for multiple agents simultaneously — useful if your team uses different tools, or you use different agents for different projects.

---

## Install for all agents at once

```bash
zrk install-all --all-targets
```

This installs workspace and global files for all four agents.

```
→ Installing all files for Kiro
✓ Installed .kiro/steering/prep-review.md
✓ Installed ~/.kiro/steering/review-roles.md
...
→ Installing all files for Claude Code
✓ Installed .claude/commands/review-kit/prep-review.md
...
→ Installing all files for Cursor
⚠ Cursor: 'review-roles.mdc' requires manual setup (global rules are UI-only)
✓ Installed .cursor/rules/prep-review.mdc
...
→ Installing all files for Windsurf
⚠ Windsurf: 'review-roles.md' requires manual setup (global rules are UI-only)
✓ Installed .windsurf/rules/prep-review.md
```

---

## File layout after all-targets install

```
your-project/
  .kiro/steering/
    prep-review.md
    pack-materials.md
    project-context.md
  .claude/commands/review-kit/
    prep-review.md
    pack-materials.md
    project-context.md
  .cursor/rules/
    prep-review.mdc
    pack-materials.mdc
    project-context.mdc
  .windsurf/rules/
    prep-review.md
    pack-materials.md
    project-context.md
```

---

## Shared output directory

`.materials/` is agent-agnostic — any agent writes its output there. This means you can switch agents mid-review and still find the output in the same place.

---

## Updating all agents

```bash
zrk update --all-targets
```

---

## Status for all agents

```bash
zrk status --all-targets
```

---

## Next

- [Back to agent overview](overview.md)
- [CLI flags reference](../reference/cli-flags.md)

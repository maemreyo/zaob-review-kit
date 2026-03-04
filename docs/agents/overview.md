# Agent Setup Overview

`zrk` supports four AI coding agents. They differ in where they store config files and what format they use.

---

## Agent comparison

| Agent       | Global dir                       | Workspace dir                  | Format                    | Auto global? |
| ----------- | -------------------------------- | ------------------------------ | ------------------------- | ------------ |
| Kiro        | `~/.kiro/steering/`              | `.kiro/steering/`              | `.md` + YAML frontmatter  | Yes          |
| Claude Code | `~/.claude/commands/review-kit/` | `.claude/commands/review-kit/` | `.md` + YAML frontmatter  | Yes          |
| Cursor      | (none)                           | `.cursor/rules/`               | `.mdc` + YAML frontmatter | Manual       |
| Windsurf    | (none)                           | `.windsurf/rules/`             | `.md` + comment header    | Manual       |

**Auto global** means `zrk install-global` writes files directly. Cursor and Windsurf require you to paste the content into their settings UI.

---

## Choosing your agent

**Kiro or Claude Code:** Recommended for the smoothest experience. Both support automatic global install.

**Cursor:** Works well for workspace-level rules. Global rules require one manual copy-paste step in Cursor Settings.

**Windsurf:** Same as Cursor — workspace rules are automatic, global rules are manual.

**Multiple agents:** Valid use case — use `--all-targets` to install for all at once.

---

## Install for each agent

```bash
# Kiro
zrk install-all --target kiro

# Claude Code
zrk install-all --target claude-code

# Cursor
zrk install-all --target cursor

# Windsurf
zrk install-all --target windsurf

# All at once
zrk install-all --all-targets
```

---

## Agent guides

- [Kiro](kiro.md)
- [Claude Code](claude-code.md)
- [Cursor](cursor.md)
- [Windsurf](windsurf.md)
- [Multiple agents](multiple-agents.md)

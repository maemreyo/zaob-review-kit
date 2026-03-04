# zrk install-global

Install global-scoped files into the agent's machine-wide config directory.

---

## What it installs

Four global files:

| File                  | Installed to (Kiro)                    |
| --------------------- | -------------------------------------- |
| `review-roles.md`     | `~/.kiro/steering/review-roles.md`     |
| `review-prompting.md` | `~/.kiro/steering/review-prompting.md` |
| `review-ignore.md`    | `~/.kiro/steering/review-ignore.md`    |
| `review-memory.md`    | `~/.kiro/steering/review-memory.md`    |

---

## Usage

```bash
# Install global files for default agent (kiro)
zrk install-global

# Install for a specific agent
zrk install-global --target claude-code
zrk install-global --target cursor      # prints manual instructions
zrk install-global --target windsurf    # prints manual instructions

# Install for all agents
zrk install-global --all-targets

# Preview
zrk install-global --dry-run
```

---

## Agents with automatic global install

**Kiro** and **Claude Code** have filesystem-backed global config directories. Files are written directly.

**Cursor** and **Windsurf** have UI-only global config. `install-global` prints the file contents with instructions to paste them into the agent settings UI:

```
⚠ Cursor: 'review-roles.mdc' requires manual setup (global rules are UI-only)
⚠ Cursor: 'review-prompting.mdc' requires manual setup
⚠ Cursor: 'review-ignore.mdc' requires manual setup
⚠ Cursor: 'review-memory.mdc' requires manual setup
```

See [Cursor setup](../agents/cursor.md) and [Windsurf setup](../agents/windsurf.md) for the manual steps.

---

## When to run

- Once per machine when you first install an agent
- After installing a new agent on an existing machine
- When workspace-scoped install isn't enough and you want the rules globally

---

## Next

- [install — workspace files only](install.md)
- [install-all — both global and workspace](install-all.md)

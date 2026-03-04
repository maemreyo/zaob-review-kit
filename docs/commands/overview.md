# CLI Commands Reference

## Command summary

| Command | Description | When to use |
|---|---|---|
| `install` | Workspace files → current project | Adding zrk to a new project |
| `install-global` | Global files → agent's global config | First time on a machine, or after a new agent install |
| `install-all` | Both + templates | Recommended first-time command |
| `update` | Force reinstall everything | After zrk releases new content versions |
| `status` | Show drift from installed state | Health check, CI verification |
| `list` | Print available agents and content files | Discover what's available |
| `init` | Interactive wizard | First-time setup on a new machine or project |

---

## Global flags

All flags work with all commands.

| Flag | Default | Description |
|---|---|---|
| `--target <agent>` | `kiro` | Agent to install for: `kiro`, `claude-code`, `cursor`, `windsurf` |
| `--all-targets` | false | Apply to all four agents at once (overrides `--target`) |
| `--force` | false | Overwrite existing files (default: skip) |
| `--cwd <path>` | `$PWD` | Project directory (workspace install location) |
| `--dry-run` | false | Show what would be done without doing it |
| `--no-color` | false | Disable colored output (useful in CI) |
| `--quiet` | false | Suppress non-error output |

---

## Output symbols

```
✓  green   installed / success
⚠  yellow  skipped / manual instruction / warning
✗  red     error / not installed
→  cyan    info / action being taken
–  dim     neutral / not yet created
```

---

## Common patterns

### First time on a machine

```bash
zrk install-global --target kiro
```

Installs the global workflow files to `~/.kiro/steering/`. Do this once per machine.

### First time in a project

```bash
cd my-project
zrk install
```

Installs workspace files to `.kiro/steering/`.

### First time — everything

```bash
cd my-project
zrk install-all
```

Global + workspace + templates in one shot. Recommended.

### Using the wizard

```bash
zrk init
```

Interactive prompts, then installs everything for your chosen agents.

### Using a different agent

```bash
zrk install-all --target cursor
zrk install-all --target windsurf
zrk install-all --target claude-code
```

### Installing for all agents at once

```bash
zrk install-all --all-targets
```

### Preview before installing

```bash
zrk install-all --dry-run
```

No files written. Shows exactly what would happen.

### Check installation health

```bash
zrk status
zrk status --all-targets
```

### Reinstall after zrk update

```bash
zrk update
zrk update --all-targets
```

---

## Commands in detail

- [install](install.md)
- [install-global](install-global.md)
- [install-all](install-all.md)
- [update](update.md)
- [status](status.md)
- [list](list.md)
- [init](init.md)

---

## Next

- [Agent-specific setup](../agents/overview.md)
- [CLI flags reference](../reference/cli-flags.md)

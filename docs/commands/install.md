# zrk install

Install workspace-scoped files into the current project's agent config directory.

---

## What it installs

Three workspace files:

| File                 | Installed to (Kiro example)         |
| -------------------- | ----------------------------------- |
| `prep-review.md`     | `.kiro/steering/prep-review.md`     |
| `pack-materials.md`  | `.kiro/steering/pack-materials.md`  |
| `project-context.md` | `.kiro/steering/project-context.md` |

Also writes `.kiro/steering/.zrk-manifest.json`.

---

## Usage

```bash
# Install for the default agent (kiro), current directory
zrk install

# Install for a specific agent
zrk install --target cursor
zrk install --target claude-code
zrk install --target windsurf

# Install for all agents
zrk install --all-targets

# Install to a specific project directory
zrk install --cwd /path/to/project

# Preview without writing
zrk install --dry-run

# Overwrite existing files
zrk install --force
```

---

## Example output

```
→ Installing workspace files for Kiro
✓ Installed .kiro/steering/prep-review.md
✓ Installed .kiro/steering/pack-materials.md
✓ Installed .kiro/steering/project-context.md

→ Done: 3 installed, 0 skipped, 0 manual
```

If files exist (second run without --force):

```
→ Installing workspace files for Kiro
⚠ Skipped (exists) .kiro/steering/prep-review.md
⚠ Skipped (exists) .kiro/steering/pack-materials.md
⚠ Skipped (exists) .kiro/steering/project-context.md

→ Done: 0 installed, 3 skipped, 0 manual
```

---

## install vs install-all

| Command          | Workspace files | Global files | Templates |
| ---------------- | --------------- | ------------ | --------- |
| `install`        | Yes             | No           | No        |
| `install-global` | No              | Yes          | No        |
| `install-all`    | Yes             | Yes          | Yes       |

For first-time setup, use [`install-all`](install-all.md).

---

## Next

- [install-global](install-global.md)
- [install-all](install-all.md)
- [CLI flags reference](../reference/cli-flags.md)

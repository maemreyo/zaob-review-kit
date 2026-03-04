# zrk update

Force reinstall all files with the latest content. Equivalent to `install-all --force`.

---

## Usage

```bash
# Update default agent (kiro)
zrk update

# Update a specific agent
zrk update --target cursor
zrk update --target claude-code

# Update all agents
zrk update --all-targets

# Preview what would be updated
zrk update --dry-run
```

---

## What it does

Runs `install-all` with `--force=true`. Every installed file is overwritten with the latest content from the zrk binary.

```
→ Installing all files for Kiro
✓ Installed .kiro/steering/prep-review.md       ← overwrote existing
✓ Installed .kiro/steering/pack-materials.md
✓ Installed .kiro/steering/project-context.md   ← your edits are gone
✓ Installed ~/.kiro/steering/review-roles.md
...
```

---

## What update does NOT touch

- `.archignore` — you may have customized it. Update skips templates.
- `.materials/` — your review output and memory are never touched by zrk
- `.gitignore` — already has the rules; the `append_gitignore` function skips if rules exist

---

## Before running update

Check status to see what you've modified:

```bash
zrk status
# ⚠ project-context.md (modified)
```

**Modified files will be overwritten.** If you've customized `project-context.md`, back it up:

```bash
cp .kiro/steering/project-context.md project-context.md.bak
zrk update
# Manually re-apply your customizations
```

---

## When to run update

- After `cargo install zrk` upgrades to a new version with content changes
- When you want to reset customized workflow files back to defaults
- Periodic refresh to pick up any improvements to the workflow files

---

## Next

- [status — check what's been modified before updating](status.md)
- [install-all](install-all.md)

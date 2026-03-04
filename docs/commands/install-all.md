# zrk install-all

Install workspace files + global files + templates in one command. Recommended for first-time setup.

---

## What it installs

Everything:

- 3 workspace-scoped files → `.kiro/steering/` (or agent equivalent)
- 4 global-scoped files → `~/.kiro/steering/` (or agent equivalent)
- `.archignore` → project root
- Appends gitignore rules for `.materials/` → `.gitignore`

---

## Usage

```bash
# Install-all for default agent (kiro), current directory
zrk install-all

# Install for a specific agent
zrk install-all --target cursor
zrk install-all --target claude-code

# Install for all agents at once
zrk install-all --all-targets

# Preview without writing anything
zrk install-all --dry-run

# Overwrite existing files
zrk install-all --force
```

---

## Example output

```
→ Installing all files for Kiro
✓ Installed .kiro/steering/prep-review.md
✓ Installed .kiro/steering/pack-materials.md
✓ Installed .kiro/steering/project-context.md
✓ Installed ~/.kiro/steering/review-roles.md
✓ Installed ~/.kiro/steering/review-prompting.md
✓ Installed ~/.kiro/steering/review-ignore.md
✓ Installed ~/.kiro/steering/review-memory.md
✓ Installed .archignore
✓ Updated .gitignore with review material patterns

→ Done: 8 installed, 0 skipped, 0 manual
```

Cursor (global is manual-only):

```
→ Installing all files for Cursor
✓ Installed .cursor/rules/prep-review.mdc
✓ Installed .cursor/rules/pack-materials.mdc
✓ Installed .cursor/rules/project-context.mdc
⚠ Cursor: 'review-roles.mdc' requires manual setup (global rules are UI-only)
⚠ Cursor: 'review-prompting.mdc' requires manual setup
⚠ Cursor: 'review-ignore.mdc' requires manual setup
⚠ Cursor: 'review-memory.mdc' requires manual setup
✓ Installed .archignore
✓ Updated .gitignore with review material patterns

→ Done: 4 installed, 0 skipped, 4 manual
```

---

## Dry-run preview

```bash
zrk install-all --dry-run
```

```
→ Installing all files for Kiro
→ Would create directory: .kiro/steering
→ Would create file: .kiro/steering/prep-review.md
→ Would create file: .kiro/steering/pack-materials.md
→ Would create file: .kiro/steering/project-context.md
→ Would write manifest in: .kiro/steering
→ Would create directory: ~/.kiro/steering
– Would skip (exists): ~/.kiro/steering/review-roles.md
...
```

---

## Next

- [update — reinstall with latest content](update.md)
- [status — check installation health](status.md)
- [init — interactive wizard](init.md)

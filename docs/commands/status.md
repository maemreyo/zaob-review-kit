# zrk status

Show installation status and detect drift from the installed state.

---

## Usage

```bash
# Status for default agent (kiro)
zrk status

# Status for a specific agent
zrk status --target cursor

# Status for all agents
zrk status --all-targets

# Status for a different project directory
zrk status --cwd /path/to/project
```

---

## Example output

```
Kiro

  Workspace (.kiro/steering)
  ✓ prep-review.md
  ✓ pack-materials.md
  ⚠ project-context.md (modified)

  Global (~/.kiro/steering)
  ✓ review-roles.md
  ✓ review-prompting.md
  ✓ review-ignore.md
  ✓ review-memory.md

✓ .archignore
– .materials/review_memory.md (not yet created — expected after first review)
```

---

## File states

| Symbol | State      | Meaning                                       | Action                                                |
| ------ | ---------- | --------------------------------------------- | ----------------------------------------------------- |
| `✓`    | Installed  | File exists, hash matches manifest            | None needed                                           |
| `⚠`    | Modified   | File exists but content changed since install | Leave it (you customized it) or run `update` to reset |
| `✗`    | Missing    | File was in manifest but deleted              | Run `install` to restore                              |
| `–`    | NoManifest | No manifest found in directory                | Run `install-all` to set up                           |

---

## How it works

`zrk status` reads `.zrk-manifest.json` from the install directory, then compares the SHA-256 hash of each tracked file against the stored hash.

- Hash matches → Installed
- Hash differs → Modified
- File gone → Missing
- No manifest → NoManifest

See [manifest reference](../reference/manifest.md) for details.

---

## When to run status

```bash
# After fresh install — verify everything worked
zrk install-all && zrk status

# Periodic health check
zrk status --all-targets

# Before update — see what you've customized (will be overwritten)
zrk status
zrk update   # overwrites modified files
```

---

## Next

- [update — force reinstall](update.md)
- [Manifest reference](../reference/manifest.md)

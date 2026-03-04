# .zrk-manifest.json

The manifest file tracks installed content files and powers `zrk status`.

---

## What it is

After every successful `install` or `update`, zrk writes a `.zrk-manifest.json` file into the install directory. It records the SHA-256 hash of every file that was written.

`zrk status` reads this file and compares stored hashes against the current files on disk to detect drift.

---

## Location

One manifest per install directory:

```
~/.kiro/steering/.zrk-manifest.json        ← global files
.kiro/steering/.zrk-manifest.json          ← workspace files
.claude/commands/review-kit/.zrk-manifest.json
...
```

---

## Format

```json
{
  "version": "0.1.0",
  "agent": "kiro",
  "files": {
    "review-roles.md": "a3f5b8c2...",
    "review-prompting.md": "d9e1f4a7...",
    "review-ignore.md": "b2c6e8d1...",
    "review-memory.md": "f7a3b5c9..."
  }
}
```

- `version`: zrk version that wrote this manifest
- `agent`: agent name
- `files`: map of filename → SHA-256 hex hash of the file content at install time

---

## File states reported by `zrk status`

| State        | Meaning                                            |
| ------------ | -------------------------------------------------- |
| `Installed`  | File exists, hash matches manifest                 |
| `Modified`   | File exists, hash does not match (you edited it)   |
| `Missing`    | File is in the manifest but does not exist on disk |
| `NoManifest` | No manifest file found in the directory            |

---

## When to run `zrk status`

```bash
# After a fresh install — verify everything worked
zrk install-all && zrk status

# Periodic health check
zrk status --all-targets

# Before updating — see what's been modified
zrk status
zrk update    # overwrites everything including modified files
```

---

## Should I commit .zrk-manifest.json?

No. It's machine-local metadata that tracks what zrk installed on your specific machine. Committing it would cause false "Modified" states on other developers' machines.

The default `.gitignore` rules added by `zrk install-all` do not gitignore the manifest (it lives in agent config dirs that are typically gitignored entirely).

---

## Next

- [zrk status command](../commands/status.md)
- [File layout reference](file-layout.md)

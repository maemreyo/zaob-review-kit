# CLI Flags Reference

All flags are global — they work with every command.

---

## `--target <agent>`

**Default:** `kiro`
**Values:** `kiro`, `claude-code`, `cursor`, `windsurf`

Specifies which agent to install for. Ignored when `--all-targets` is set.

```bash
zrk install-all --target cursor
zrk status --target claude-code
zrk update --target windsurf
```

---

## `--all-targets`

**Default:** false

Applies the command to all four agents at once. Overrides `--target`.

```bash
zrk install-all --all-targets
zrk status --all-targets
zrk update --all-targets
```

---

## `--force`

**Default:** false

Overwrites existing files. Without `--force`, existing files are skipped.

```bash
zrk install-all --force           # same as: zrk update
zrk install --force --target kiro
```

Difference from `update`: `--force` is a modifier on any command. `update` is a command that implies `install-all --force`.

---

## `--cwd <path>`

**Default:** current working directory (`$PWD`)

Sets the project root for workspace file installation. Does not affect global file paths.

```bash
zrk install --cwd /path/to/my-project
zrk install-all --cwd ~/projects/my-app
```

Useful for scripting or when running from outside the project root.

---

## `--dry-run`

**Default:** false

Shows what would be done without writing any files. Useful for previewing before committing.

```bash
zrk install-all --dry-run
# → Would create directory: .kiro/steering
# → Would create file: .kiro/steering/prep-review.md
# → Would create file: .kiro/steering/pack-materials.md
# ...
```

No files are created, no manifests updated, no gitignore changes.

---

## `--no-color`

**Default:** false

Disables ANSI color codes in output. Use in CI environments or when piping output.

```bash
zrk install-all --no-color
zrk status --no-color | tee install-log.txt
```

---

## `--quiet`

**Default:** false

Suppresses all non-error output. Errors still print to stderr.

```bash
zrk install-all --quiet
zrk update --quiet --all-targets
```

Combine with `--no-color` for fully machine-readable output.

---

## Combining flags

```bash
# Install for all agents in CI, no color, quiet, dry run first
zrk install-all --all-targets --dry-run --no-color --quiet

# Force update all, quiet
zrk update --all-targets --quiet

# Preview what update would do for cursor
zrk update --target cursor --dry-run
```

---

## Next

- [Command overview](../commands/overview.md)
- [Architecture](../contributing/architecture.md)

# zrk list

Print all available agents and content files.

---

## Usage

```bash
zrk list
```

---

## Example output

```
Supported agents:
  Kiro (kiro)
    Global:    /Users/you/.kiro/steering
    Workspace: ./.kiro/steering
  Claude Code (claude-code)
    Global:    /Users/you/.claude/commands/review-kit
    Workspace: ./.claude/commands/review-kit
  Cursor (cursor)
    Global:    (manual only)
    Workspace: ./.cursor/rules
  Windsurf (windsurf)
    Global:    (manual only)
    Workspace: ./.windsurf/rules

Content files:
  [Global]    review-roles.md
  [Global]    review-prompting.md
  [Global]    review-ignore.md
  [Global]    review-memory.md
  [Workspace] prep-review.md
  [Workspace] pack-materials.md
  [Workspace] project-context.md
  [Template]  archignore
  [Template]  gitignore-snippet.txt
```

---

## Use cases

- Verify that a new agent is registered before using `--target`
- Check which content files exist before scripting installs
- Confirm agent paths before running `install-global`

---

## Next

- [Agent overview](../agents/overview.md)
- [Content files reference](../reference/content-files.md)

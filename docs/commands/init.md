# zrk init

Interactive first-time setup wizard. Recommended starting point for new machines or projects.

---

## Usage

```bash
zrk init
```

---

## Wizard walkthrough

```
? Which agents are you using?
  > [x] Kiro
    [ ] Claude Code
    [ ] Cursor
    [ ] Windsurf

? Install global rules? (affects all your projects) [Y/n] Y

? Generate project-context.md scaffold? [Y/n] Y

→ Setting up Kiro
✓ Installed .kiro/steering/prep-review.md
✓ Installed .kiro/steering/pack-materials.md
✓ Installed .kiro/steering/project-context.md
✓ Installed ~/.kiro/steering/review-roles.md
✓ Installed ~/.kiro/steering/review-prompting.md
✓ Installed ~/.kiro/steering/review-ignore.md
✓ Installed ~/.kiro/steering/review-memory.md
✓ Installed .archignore
✓ Updated .gitignore with review material patterns

✓ Setup complete!
→ Next: ask your agent to fill in project-context.md
```

---

## Multi-agent selection

Select multiple agents with space:

```
? Which agents are you using?
  > [x] Kiro
    [x] Claude Code
    [ ] Cursor
    [ ] Windsurf
```

Both agents are installed in sequence.

---

## What init does

1. Asks which agents to set up
2. Asks whether to install global files (yes by default)
3. Runs `install-all` for each selected agent
4. Prints a "next steps" message

It's equivalent to:

```bash
zrk install-all --target kiro
zrk install-all --target claude-code
# (for the example above)
```

---

## After init

Fill in `.kiro/steering/project-context.md` (or agent equivalent) with your project's description. Ask your agent:

```
tạo materials để review 3 commit gần nhất
```

---

## Without the wizard feature

If zrk was compiled without the `wizard` feature (uncommon for distributed binaries), `init` falls back to installing for the `--target` agent with defaults:

```bash
zrk init --target kiro
# equivalent to: zrk install-all --target kiro
```

---

## Next

- [First review walkthrough](../getting-started/first-review.md)
- [install-all reference](install-all.md)

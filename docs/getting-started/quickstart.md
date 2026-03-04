# Quickstart

Get from zero to your first review materials in 2 minutes.

---

## Step 1: Install zrk

```bash
cargo install zrk
```

## Step 2: Set up your project

Run in your project root:

```bash
zrk init
```

You'll see interactive prompts:

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

→ Done. Next: ask your agent to fill in project-context.md
```

Or skip the wizard and go direct:

```bash
# Install for Kiro (default)
zrk install-all

# Install for Cursor
zrk install-all --target cursor

# Install for all agents at once
zrk install-all --all-targets
```

## Step 3: (Optional) Fill in project-context.md

Open `.kiro/steering/project-context.md` (or the equivalent for your agent) and fill in the project overview. This gives the reviewer context about your codebase.

```markdown
## Project Overview

A Rust CLI for managing database migrations...

## Tech Stack

Rust, PostgreSQL, sqlx

## Architecture

Layered: CLI → service → repository → database
```

This file is used by the agent when generating review prompts. It's optional — reviews work without it, just with less context.

## Step 4: Trigger a review

Open your agent's chat and type:

**In Vietnamese:**

```
tạo materials để review 3 commit gần nhất
```

**In English:**

```
create review materials for the last 3 commits
pack the authentication changes for review
prepare review for the entire docs/architecture folder
```

Your agent reads the installed workflow files and generates:

```
.materials/
  20250304_a1b2c3/
    review_context.xml   ← structured diff + context
    review_prompt.md     ← structured review prompt
    review.patch         ← raw patch
```

## Step 5: Upload to Claude.ai

1. Go to [claude.ai](https://claude.ai)
2. Attach `review_context.xml` and `review_prompt.md`
3. Send the message (the prompt file already contains everything Claude needs)

You'll get a structured review with findings from five perspectives: Senior SWE, Software Architect, QA, CEO, and Devil's Advocate.

---

## What's next?

- [First review walkthrough — detailed end-to-end](first-review.md)
- [CLI commands reference](../commands/overview.md)
- [Agent-specific setup](../agents/overview.md)
- [How the workflow files work](../workflow/overview.md)

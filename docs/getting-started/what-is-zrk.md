# What is zrk?

`zrk` is a CLI that installs review workflow files into your AI coding agent — so you can get structured code review materials ready for Claude.ai with one sentence.

---

## The workflow gap

Solo developers increasingly use two distinct AI tools:

- **AI coding agents** (Kiro, Claude Code, Cursor, Windsurf) — where code is _written_, inside the IDE
- **Claude.ai web** — where code is _reviewed_, as a conversation

These tools don't talk to each other. The gap between them is manual, repetitive, and lossy.

Sending code to Claude.ai for review is harder than it should be:

| Approach                             | Problem                                                          |
| ------------------------------------ | ---------------------------------------------------------------- |
| Zip the whole codebase               | Hits context limit immediately on any real project               |
| Send a git patch                     | Missing context — changed files reference files Claude can't see |
| Manually copy-paste related files    | Slow, inconsistent, easy to miss dependencies                    |
| Write the review prompt from scratch | Repetitive, no structure, no role perspective                    |

None of these scale.

---

## The insight

What Claude.ai actually needs is not the whole codebase — it needs **smart context**:

1. **The diff** — what changed, stripped of noise
2. **Referenced context** — files imported by or importing the changed files
3. **A structured prompt** — the right roles, the right questions, calibrated to the nature of the change

Generating this manually every time is the friction. It should take one sentence.

---

## What zrk does

`zrk` installs review workflow markdown files into your AI coding agent's config directory. Once installed, you type one sentence into your agent's chat:

> _"tạo materials để review 3 commit gần nhất"_
> _"pack the auth changes for review"_
> _"tạo materials của toàn bộ docs về architect"_

The agent reads its installed workflow files and produces:

```
.materials/
  20250304_abc-def/
    review_context.xml   ← diff + referenced files, structured
    review_prompt.md     ← structured prompt with 5 reviewer roles
    review.patch         ← raw patch for reference
```

Upload both files to Claude.ai. The prompt already contains the reviewer roles (SWE, SA, QA, CEO, Devil's Advocate), calibrated to the complexity of the change.

---

## The flow

```
Your IDE agent
      │
      │ one sentence
      ▼
review workflow files (installed by zrk)
      │
      │ runs prep-review + pack-materials
      ▼
.materials/<timestamp>/
  review_context.xml
  review_prompt.md
      │
      │ upload both
      ▼
Claude.ai → structured review
```

---

## What zrk installs

Nine markdown files across three scopes:

| File                    | Scope     | Purpose                                |
| ----------------------- | --------- | -------------------------------------- |
| `review-roles.md`       | Global    | 5 reviewer role definitions            |
| `review-prompting.md`   | Global    | Output structure and calibration rules |
| `review-ignore.md`      | Global    | Default ignore patterns                |
| `review-memory.md`      | Global    | Persistent review memory format        |
| `prep-review.md`        | Workspace | Main workflow trigger                  |
| `pack-materials.md`     | Workspace | Context XML packing rules              |
| `project-context.md`    | Workspace | Project-specific context scaffold      |
| `.archignore`           | Template  | Project-specific ignore patterns       |
| `gitignore-snippet.txt` | Template  | Appended to `.gitignore`               |

**Global** files apply to all your projects. **Workspace** files are per-project.

---

## What zrk does NOT do

- Does not run the review itself — that's your agent's job
- Does not connect to the internet at runtime (single binary, no runtime dependencies)
- Does not sync content updates automatically — run `zrk update` manually
- Does not have a GUI
- Does not modify your code or git history

---

## Next

- [Install zrk](installation.md)
- [Quickstart — up and running in 2 minutes](quickstart.md)
- [Agent setup guides](../agents/overview.md)

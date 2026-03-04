# How the Review Workflow Works

The zrk workflow has two halves: **distribution** (what `zrk` the CLI does) and **execution** (what your agent does with the installed files).

---

## The complete loop

```
1. zrk install-all
        │
        ▼
   Workflow files installed into your agent's config directory

2. You type a sentence in your agent's chat
        │
        ▼
   Agent reads prep-review.md and executes the workflow

3. Agent produces:
   .materials/<timestamp>/
     review_context.xml   ← diff + context, structured
     review_prompt.md     ← 5-role review prompt
     review.patch

4. Upload both files to Claude.ai
        │
        ▼
   Claude reviews with SWE + SA + QA + CEO + Devil's Advocate perspectives
        │
        ▼
   Structured output: Summary, Risk, Findings, Recommendations, Verdict
```

---

## Installed content files

Nine files are installed, organized by scope:

### Global files (apply to all your projects)

Installed to `~/.kiro/steering/` (or agent equivalent). The agent loads these for every project.

| File                  | Purpose                                          |
| --------------------- | ------------------------------------------------ |
| `review-roles.md`     | Defines the 5 reviewer role perspectives         |
| `review-prompting.md` | Output structure and calibration rules           |
| `review-ignore.md`    | Default patterns to exclude from context packing |
| `review-memory.md`    | Format for persistent review memory              |

### Workspace files (per-project)

Installed to `.kiro/steering/` (or agent equivalent). Project-specific behavior.

| File                 | Purpose                                                                     |
| -------------------- | --------------------------------------------------------------------------- |
| `prep-review.md`     | Main workflow trigger — the agent reads this when you type a review request |
| `pack-materials.md`  | Rules for building `review_context.xml`                                     |
| `project-context.md` | Scaffold for your project description (you fill this in)                    |

### Templates (project root)

| File                 | Purpose                                              |
| -------------------- | ---------------------------------------------------- |
| `.archignore`        | Project-specific ignore patterns for context packing |
| `.gitignore` snippet | Ensures per-review output dirs are gitignored        |

---

## The `.materials/` directory

After each review run, a timestamped subdirectory is created:

```
.materials/
  review_memory.md         ← commit this (grows over time)
  REVIEW_LOG.md            ← commit this (history)
  20250304_a1b2c3/         ← gitignored (per-review output)
    review_context.xml
    review_prompt.md
    review.patch
  20250310_d4e5f6/
    ...
```

**Commit:** `review_memory.md` and `REVIEW_LOG.md` — shared project knowledge.
**Gitignore:** timestamped subdirectories — too large, regenerate as needed.

The gitignore rules are appended by `zrk install-all` automatically.

---

## Workflow file details

- [prep-review.md](prep-review.md) — What the agent does when you trigger a review
- [pack-materials.md](pack-materials.md) — How `review_context.xml` is built
- [review-roles.md](review-roles.md) — The five reviewer perspectives
- [review-prompting.md](review-prompting.md) — Output format and calibration
- [review-memory.md](review-memory.md) — Persistent memory across reviews
- [review-ignore.md](review-ignore.md) — Ignore patterns

---

## Next

- [First review walkthrough](../getting-started/first-review.md)
- [Customizing ignore patterns](../reference/archignore.md)
- [Content file reference](../reference/content-files.md)

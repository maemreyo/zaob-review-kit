# review-memory.md

Maintains a persistent memory of review patterns across sessions, so reviews improve over time.

---

## The problem it solves

Without memory, every review starts cold. Claude doesn't know:

- That you've already addressed the "no error handling in the auth module" finding
- That your project intentionally avoids certain patterns
- Which areas of the codebase are the most fragile
- What feedback you've given in previous reviews

With `review_memory.md`, the agent reads accumulated context before each new review.

---

## Where it lives

```
.materials/
  review_memory.md     ← accumulates across reviews — commit this
  REVIEW_LOG.md        ← history log — commit this
```

The first time the agent runs a review, it creates this file. Subsequent reviews append to it.

---

## What the agent adds after each review

The agent updates `review_memory.md` with:

- **Date and scope** of the review (e.g., "2025-03-04: reviewed auth middleware, 3 commits")
- **Key findings** that were found — especially recurring patterns
- **Deferred items** — issues marked "consider for future" that weren't fixed
- **Patterns confirmed** — conventions that were followed correctly (positive signal)

---

## Example content

```markdown
# Review Memory

## 2025-03-04 — Auth middleware (3 commits)

- **Finding**: Token expiry uses strict `<` — fixed in this review
- **Finding**: Session struct has raw DB handle — deferred to future refactor
- **Pattern confirmed**: All new handlers correctly use the `require_auth()` macro

## 2025-02-20 — User profile endpoint (1 commit)

- **Recurring**: Missing test for `None` case on optional fields — seen again here
- **Deferred**: Pagination not implemented yet — known gap, tracked in #42

## Known fragile areas

- `src/auth/` — multiple review findings, needs dedicated refactor
- `src/db/migrations/` — complex, always check side effects

## Project conventions confirmed

- All public functions have doc comments
- Error types use the `thiserror` derive pattern
- No unwrap() in production paths
```

---

## Using memory in a review

Before triggering a new review, you can explicitly reference the memory:

```
review the auth changes and check if the deferred items from last time were addressed
```

The agent reads `review_memory.md` and includes the prior findings in its analysis.

---

## Commit it

`review_memory.md` should be committed. It's shared project knowledge — not machine-specific output.

```bash
git add .materials/review_memory.md
git commit -m "chore: update review memory after auth review"
```

---

## Next

- [prep-review.md — the main workflow](prep-review.md)
- [review-ignore.md — what gets excluded](review-ignore.md)

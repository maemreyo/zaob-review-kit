# Review Prompting Guidelines

## Output Structure

Always structure your review with:

1. **Summary** — one paragraph describing what changed and why
2. **File Walkthrough** — per-file table (see format below)
3. **Risk Assessment** — Low / Medium / High with justification, plus Review Effort [1–5]
4. **Findings** — organized by role perspective
5. **Suggested Tests** — specific test cases the reviewer recommends adding (see format below)
6. **Recommendations** — prioritized action items
7. **Verdict** — Ship / Ship with changes / Needs rework / Needs discussion

## File Walkthrough Format

Always include a walkthrough table immediately after the Summary. This gives the
author and reviewer a navigation map before diving into findings:

| File                          | Change type | What changed                 | Why / Notes        |
| ----------------------------- | ----------- | ---------------------------- | ------------------ |
| `src/routes/students.rs`      | New         | 8 CRUD endpoints             | Core of Phase 1A   |
| `migrations/001_students.sql` | New         | students table schema        | Needs index review |
| `src/services/health.rs`      | Modified    | division-by-zero guard added | Bug H5 fix         |

Change types: `New` / `Modified` / `Deleted` / `Renamed` / `Config`

If a file is trivial (formatting only, generated, no logic), one word suffices:
`formatting` or `generated — skip`.

## Review Effort

Include a **Review Effort [x/5]** score inside the Risk Assessment section:

| Score | Meaning                                              |
| ----- | ---------------------------------------------------- |
| 1     | Trivial — typo fix, rename, comment update           |
| 2     | Small — single function, clear intent                |
| 3     | Medium — multiple files, some complexity             |
| 4     | Large — cross-cutting change, careful reading needed |
| 5     | Very large — break into smaller PRs recommended      |

A score of 4 or 5 should include a note suggesting which parts to review first.

## Suggested Tests Format

The "Suggested Tests" section must always be present — even when the review is
positive. Use this structure for each suggested test:

```
- **[test name]** (type: unit | integration | e2e)
  Scenario: <one sentence — what situation this test exercises>
  Input / condition: <specific values, state, or trigger>
  Expected: <what should happen — return value, side effect, error>
  Catches: <what bug or regression this prevents>
```

**Test types to consider for every diff:**

- Happy path not yet covered by existing tests
- Boundary values (zero, max, empty, null/None)
- Auth / permission edge cases (wrong role, missing token, cross-tenant)
- Concurrent / duplicate requests (idempotency)
- Downstream failure (dependency returns error or times out)
- Rollback / data consistency (what if step 2 of 3 fails?)

If existing tests already cover a scenario well, say so — do not invent redundant
suggestions. If the diff has no testable logic, state "No new test cases suggested."

## Calibration Rules

### By size

- Small (< 50 lines): SWE focus — correctness and style
- Medium (50–300 lines): SWE + SA
- Large (> 300 lines): full multi-role review

### By change type (overrides size rules)

| Change contains                                      | Add roles                       |
| ---------------------------------------------------- | ------------------------------- |
| `auth/`, `login`, `token`, `permission`, `session`   | Security Engineer               |
| DB queries, migrations, schema changes               | Performance Engineer + SA + OE  |
| Public API surface changes                           | SA + CEO + Devil's Advocate     |
| New dependencies                                     | Security Engineer               |
| Breaking changes                                     | CEO + Devil's Advocate (always) |
| `async`, concurrency primitives, locks               | Performance Engineer            |
| New endpoints, background jobs, cron/scheduled tasks | OE                              |
| New env vars, config keys, external service calls    | OE                              |
| Retry logic, timeouts, circuit breakers              | OE + Performance Engineer       |

### Verdict scale

- **Ship** — no blocking issues
- **Ship with changes** — minor issues, safe to fix in follow-up
- **Needs rework** — blocking issues present, do not merge
- **Needs discussion** — architectural questions must be resolved first

## Comment Severity Labels

Tag every finding so the author knows what must be fixed vs. what is optional:

| Label          | Meaning                                             | Blocks merge? |
| -------------- | --------------------------------------------------- | ------------- |
| `[BLOCKER]`    | Incorrect behavior, security hole, data loss risk   | Yes           |
| `[MAJOR]`      | Significant design or logic flaw, hard to fix later | Yes           |
| `[SUGGESTION]` | Better approach exists, but current code works      | No            |
| `[NIT]`        | Style, naming, minor readability                    | No            |
| `[QUESTION]`   | Asking for clarification, not a criticism           | No            |

Use exactly one label per finding. If in doubt between `[BLOCKER]` and `[MAJOR]`,
prefer `[MAJOR]` and explain why. Never mix labels in one comment.

## AI Review Caveat

These findings are generated by an AI and should be treated as a structured
starting point, not a definitive verdict. Before acting on any finding:

- **Verify before flagging** — confirm the issue exists in the actual code, not
  a compressed or truncated representation of it
- **False positives happen** — AI reviewers misread context, miss runtime
  invariants, and occasionally hallucinate issues that don't exist
- **Human sign-off required** — no AI finding alone should block a merge; a
  human reviewer must confirm any `[BLOCKER]` or `[MAJOR]` before it is treated
  as such

## Team Best Practices Injection

If `review-best-practices.md` exists in the workspace, read it before reviewing
and apply its rules as additional criteria. Team-specific rules override generic
best practices when they conflict.

Reference it explicitly in findings:

> `[MAJOR]` Violates team rule: "Never expose raw DB errors to API callers"
> (review-best-practices.md, section: Error Handling)

If the file doesn't exist, apply generic best practices only and suggest the
team create one if recurring patterns appear across reviews.

## Tone

- Be direct but constructive
- Cite specific line numbers when possible
- Acknowledge what was done well

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

## Multi-File Output Structure

**Default format is Markdown.** Generate separate `.md` files organized by role perspective unless the user explicitly requests another format (e.g., `.docx`).

### File Naming Convention

All review files use the pattern `<NN>-<slug>.md` where `NN` is a two-digit number ensuring correct alphabetical sort order:

| File                   | Content                                                                              |
| ---------------------- | ------------------------------------------------------------------------------------ |
| `00-summary.md`        | Overall summary, file walkthrough, risk assessment, review effort, table of contents |
| `01-swe-review.md`     | Senior Software Engineer findings (always created)                                   |
| `02-sa-review.md`      | Software Architect findings (always created)                                         |
| `03-qa-review.md`      | Quality Assurance findings (always created)                                          |
| `04-pe-review.md`      | Performance Engineer findings (created when triggered)                               |
| `05-se-review.md`      | Security Engineer findings (created when triggered)                                  |
| `06-oe-review.md`      | Operations Engineer findings (created when triggered)                                |
| `07-ceo-review.md`     | CEO perspective findings (created when triggered)                                    |
| `08-devil-advocate.md` | Devil's Advocate findings (created when triggered)                                   |
| `99-verdict.md`        | Suggested tests, recommendations, final verdict, AI caveat                           |

### Content Distribution

**Summary File (00-summary.md):**

- One paragraph describing what changed and why
- File walkthrough table (see File Walkthrough Format section)
- Risk assessment (Low/Medium/High) with justification
- Review effort score [1-5] with explanation
- Table of contents with links to all generated role files and verdict file

**Role Files (01-08):**

- Level-1 header with role name (e.g., `# Senior Software Engineer Review`)
- Findings specific to that role perspective only
- Each finding tagged with severity label: `[BLOCKER]`, `[MAJOR]`, `[SUGGESTION]`, `[NIT]`, `[QUESTION]`
- Specific file paths and line numbers cited when referencing code
- If no issues found: brief statement "No issues found from this perspective"

**Verdict File (99-verdict.md):**

- Link back to summary file at the top: `[← Back to Summary](00-summary.md)`
- Suggested tests section (see Suggested Tests Format section)
- Prioritized recommendations organized by severity
- Final verdict: Ship / Ship with changes / Needs rework / Needs discussion
- AI review caveat (see AI Review Caveat section)

### Role Activation

**Core roles** (always create files):

- Senior Software Engineer (01)
- Software Architect (02)
- Quality Assurance (03)

**Triggered roles** (create files only when calibration rules activate them):

- Performance Engineer (04) — DB queries, async code, nested loops
- Security Engineer (05) — auth code, new dependencies
- Operations Engineer (06) — new endpoints, config changes, retry logic
- CEO (07) — breaking changes, public API changes
- Devil's Advocate (08) — breaking changes, public API changes

See Calibration Rules section for complete trigger conditions.

### Navigation Requirements

**Table of Contents in Summary:**

```markdown
## Review Files

- [Senior Software Engineer Review](01-swe-review.md)
- [Software Architect Review](02-sa-review.md)
- [Quality Assurance Review](03-qa-review.md)
- [Performance Engineer Review](04-pe-review.md) _(if triggered)_
- [Security Engineer Review](05-se-review.md) _(if triggered)_
- [Final Verdict](99-verdict.md)
```

**Back-link in Verdict:**

```markdown
[← Back to Summary](00-summary.md)
```

**Cross-references between roles:**
When a finding in one role file relates to a finding in another, use relative links:

```markdown
See also [Security Engineer Review](05-se-review.md) for authentication concerns.
```

### Examples

**Minimal Review (small change, < 50 lines):**

```
.materials/20250115-143022/
├── 00-summary.md
├── 01-swe-review.md
├── 02-sa-review.md
├── 03-qa-review.md
└── 99-verdict.md
```

**Full Review (large change with auth and DB modifications):**

```
.materials/20250115-143022/
├── 00-summary.md
├── 01-swe-review.md
├── 02-sa-review.md
├── 03-qa-review.md
├── 04-pe-review.md      ← triggered by DB queries
├── 05-se-review.md      ← triggered by auth code
├── 06-oe-review.md      ← triggered by new endpoints
└── 99-verdict.md
```

### Temp Directory for Agent Working Files

Use `.materials/<timestamp>/temp/` for intermediate working files:

- File lists and caching
- Draft findings and analysis notes
- Context notes and working memory

This keeps agent working files organized within the materials directory rather than scattered in system temp locations. The temp directory may be cleaned up after review completion or left for debugging purposes.

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

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

## Role Standard Loading Protocol

Role standard files in `role-standards/` contain full checklists for each reviewer
perspective. They are **not pre-loaded** — pulling all standards into context at once
dilutes attention and degrades review quality ("lost in the middle" effect).

**Sequential read-write-proceed pattern:**

```
For each activated role (in order 01 → 15):
  1. READ  role-standards/<NN>-<role>-standard.md
  2. APPLY its checklist to the diff
  3. WRITE reports/<NN>-<role>-review.md with findings
  4. APPEND every [BLOCKER] and [MAJOR] to temp/findings.md (one line each)
  5. PROCEED to next role (do not re-read the standard)

After all role files are written:
  6. READ  temp/findings.md (compressed log — not the individual role files)
  7. WRITE reports/99-verdict.md synthesising from that log
  8. FILL IN reports/00-summary.md table of contents and risk assessment
```

This keeps each role sharp — maximum ~5K tokens of checklist active at a time
instead of 60K+ for all standards simultaneously. The `temp/findings.md` log
means verdict synthesis reads one small file rather than re-loading all role output.

**User-specified roles:**

If `review_prompt.md` contains an `## Additional Roles` section, activate those
roles in addition to the auto-triggered set. Run them after the standard triggered
roles, in the order listed.

```markdown
## Additional Roles

- mle ← user knows this PR integrates an LLM
- finops ← user knows new Lambda functions were added
```

**Role exclusions:**

If `review_prompt.md` contains `## Skip Roles`, omit those even if the trigger
conditions match.

```markdown
## Skip Roles

- ceo ← internal refactor only, no user impact
```

---

## Multi-File Output Structure

**Default format is Markdown.** Generate separate `.md` files organized by role perspective unless the user explicitly requests another format (e.g., `.docx`).

### File Naming Convention

All review files use the pattern `<NN>-<slug>.md` where `NN` is a two-digit number ensuring correct alphabetical sort order:

| File                  | Content                                                                              |
| --------------------- | ------------------------------------------------------------------------------------ |
| `00-summary.md`       | Overall summary, file walkthrough, risk assessment, review effort, table of contents |
| `01-swe-review.md`    | Senior Software Engineer findings (always created)                                   |
| `02-sa-review.md`     | Software Architect findings (always created)                                         |
| `03-qa-review.md`     | Quality Assurance findings (always created)                                          |
| `04-pe-review.md`     | Performance Engineer findings (created when triggered)                               |
| `05-se-review.md`     | Security Engineer findings (created when triggered)                                  |
| `06-oe-review.md`     | Operations Engineer findings (created when triggered)                                |
| `07-de-review.md`     | Database Engineer findings (created when triggered)                                  |
| `08-ux-review.md`     | Frontend / UX Engineer findings (created when triggered)                             |
| `09-cl-review.md`     | Compliance Engineer findings (created when triggered)                                |
| `10-ceo-review.md`    | CEO perspective findings (created when triggered)                                    |
| `11-da-review.md`     | Devil's Advocate findings (created when triggered)                                   |
| `12-mle-review.md`    | ML / AI Engineer findings (created when triggered)                                   |
| `13-api-review.md`    | API Design findings (created when triggered)                                         |
| `14-finops-review.md` | FinOps / Cloud Cost findings (created when triggered)                                |
| `15-dx-review.md`     | Developer Experience findings (created when triggered)                               |
| `99-verdict.md`       | Suggested tests, recommendations, final verdict, AI caveat                           |

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

See `review-roles.md` for the full trigger table. Summary:

- Performance Engineer (04) — DB queries, loops, async, caching
- Security Engineer (05) — auth, user input, new deps, PII, crypto
- Operations Engineer (06) — new endpoints, config changes, jobs, IaC
- Database Engineer (07) — schema migrations, ORM models, new queries
- Frontend / UX Engineer (08) — UI components, CSS, accessibility
- Compliance Engineer (09) — PII, GDPR/CCPA, cookies, licensing
- CEO (10) — breaking changes, public API changes, user-visible impact
- Devil's Advocate (11) — major features, architecture decisions
- ML / AI Engineer (12) — ML models, LLM integration, datasets
- API Design (13) — new REST/GraphQL/gRPC endpoints, versioning
- FinOps (14) — new infra, cloud resources, cost-significant changes
- Developer Experience (15) — public APIs, SDK, README, onboarding

### Navigation Requirements

**Table of Contents in Summary:**

```markdown
## Review Files

- [Senior Software Engineer Review](01-swe-review.md)
- [Software Architect Review](02-sa-review.md)
- [Quality Assurance Review](03-qa-review.md)
- [Performance Engineer Review](04-pe-review.md) _(if triggered)_
- [Security Engineer Review](05-se-review.md) _(if triggered)_
- [Operations Engineer Review](06-oe-review.md) _(if triggered)_
- [Database Engineer Review](07-de-review.md) _(if triggered)_
- [Frontend / UX Review](08-ux-review.md) _(if triggered)_
- [Compliance Review](09-cl-review.md) _(if triggered)_
- [CEO Review](10-ceo-review.md) _(if triggered)_
- [Devil's Advocate](11-da-review.md) _(if triggered)_
- [ML / AI Engineer Review](12-mle-review.md) _(if triggered)_
- [API Design Review](13-api-review.md) _(if triggered)_
- [FinOps Review](14-finops-review.md) _(if triggered)_
- [Developer Experience Review](15-dx-review.md) _(if triggered)_
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
├── review_context.xml
├── review_prompt.md
├── reports/
│   ├── 00-summary.md
│   ├── 01-swe-review.md
│   ├── 02-sa-review.md
│   ├── 03-qa-review.md
│   └── 99-verdict.md
└── temp/
    ├── role-plan.md
    ├── file-map.md
    └── findings.md
```

**Full Review (large change with auth and DB modifications):**

```
.materials/20250115-143022/
├── review_context.xml
├── review_prompt.md
├── review.patch
├── reports/
│   ├── 00-summary.md
│   ├── 01-swe-review.md
│   ├── 02-sa-review.md
│   ├── 03-qa-review.md
│   ├── 04-pe-review.md      ← triggered by DB queries
│   ├── 05-se-review.md      ← triggered by auth code
│   ├── 06-oe-review.md      ← triggered by new endpoints
│   ├── 07-de-review.md      ← triggered by migration
│   └── 99-verdict.md
└── temp/
    ├── role-plan.md
    ├── file-map.md
    └── findings.md
```

### Temp Directory Protocol

Three files in `temp/` coordinate the review workflow. They are pre-created as
stubs by `prep-review.md` — the AI fills them progressively at defined moments.

**Never skip these files.** They are how the agent externalises its working state
and avoids holding everything in context simultaneously.

---

#### `temp/role-plan.md` — Read first, fill before reading any file content

**Purpose:** Commit to a role execution plan using only file _names_, before spending
tokens on file content. Forces deliberate role selection rather than reactive drift.

**When to fill:** After running `git diff --name-only` (or equivalent). Fill
the triggered roles table and execution order BEFORE reading any diff content.

```markdown
## Triggered roles

| #   | Role | File / pattern that triggered it |
| --- | ---- | -------------------------------- |
| 05  | SE   | src/auth/ modified               |
| 07  | DE   | migrations/008_add_reports.sql   |

## Execution order

01-swe → 02-sa → 03-qa → 05-se → 07-de → 99-verdict

## Additional roles (from review_prompt.md)

- mle: user flagged LLM integration added

## Skipped roles

- ceo: internal refactor, no public API change
```

---

#### `temp/file-map.md` — Fill while reading diff content

**Purpose:** Track which roles apply to which files. Catches role triggers that
only become visible after reading content (not just names). Serves as a navigation
aid during the review itself.

**When to fill:** Append one row per file as each file's diff is read. Do not
wait until all files are read.

```markdown
| File                | Change type | Roles   | Key observation                   |
| ------------------- | ----------- | ------- | --------------------------------- |
| src/auth/handler.rs | Modified    | SWE, SE | token validation logic changed    |
| migrations/008.sql  | New         | DE, OE  | adds NOT NULL col without default |
| src/routes/mod.rs   | Modified    | SA, OE  | new endpoint registered           |
```

---

#### `temp/findings.md` — Running log, append after each role

**Purpose:** Compressed finding log that allows `99-verdict.md` to be written by
reading ONE small file instead of re-loading 5–15 role files (60K+ tokens).

**When to append:** Immediately after writing each role's output file, append
every `[BLOCKER]` and `[MAJOR]` finding in one line each. `[SUGGESTION]` and
`[NIT]` are optional — include if they inform the overall verdict.

**Format:** `[ROLE][SEVERITY] path:line — short description`

```
[SE][BLOCKER] src/auth/handler.rs:34 — IDOR: invoice fetched without tenant check
[DE][BLOCKER] migrations/008.sql:8 — NOT NULL col without default breaks old binary
[OE][MAJOR]   src/routes/mod.rs:67 — new endpoint has no timeout on outbound call
[SWE][NIT]    src/auth/handler.rs:12 — unused import std::collections::HashMap
```

**When to read:** Read `temp/findings.md` (not individual role files) when writing
`reports/99-verdict.md`. This is the sole input for verdict synthesis.

---

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

| Change contains                                      | Add roles                    |
| ---------------------------------------------------- | ---------------------------- |
| `auth/`, `login`, `token`, `permission`, `session`   | SE (05)                      |
| DB queries, migrations, schema changes               | PE (04) + DE (07) + OE (06)  |
| Public API surface changes                           | SA (02) + CEO (10) + DA (11) |
| New dependencies                                     | SE (05)                      |
| Breaking changes                                     | CEO (10) + DA (11) — always  |
| `async`, concurrency primitives, locks               | PE (04)                      |
| New endpoints, background jobs, cron/scheduled tasks | OE (06)                      |
| New env vars, config keys, external service calls    | OE (06)                      |
| Retry logic, timeouts, circuit breakers              | OE (06) + PE (04)            |
| UI components, CSS, accessibility                    | UX (08)                      |
| PII / GDPR / cookies / open-source licence           | CL (09)                      |
| ML model / LLM integration / dataset pipeline        | MLE (12)                     |
| New REST / GraphQL / gRPC endpoints                  | API (13)                     |
| New infra / cloud resources / Lambda / auto-scaling  | FinOps (14) + OE (06)        |
| README / SDK / public API docs / onboarding changes  | DX (15)                      |
| Major feature / significant architecture change      | DA (11)                      |

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

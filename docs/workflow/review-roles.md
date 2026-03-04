# review-roles.md

Defines the five reviewer perspectives Claude uses when reviewing your code.

---

## The five roles

### 1. Senior Software Engineer (SWE)

Focused on the code itself:

- Correctness: does the logic work for all inputs, including edge cases?
- Error handling: are errors caught, logged, and surfaced correctly?
- Performance: any unnecessary allocations, N+1 queries, blocking calls?
- Readability: will the next developer understand this in six months?

### 2. Software Architect (SA)

Focused on design and structure:

- Does this fit the existing architecture, or does it create inconsistency?
- Are concerns properly separated? Is this module doing too much?
- Is the dependency graph getting worse (circular deps, tight coupling)?
- Will this scale when the team or codebase grows?

### 3. Quality Assurance (QA)

Focused on testability and risk:

- What test coverage is missing? Which paths are untested?
- What could regress with this change?
- Are acceptance criteria actually met?
- What integration risks exist with other parts of the system?

### 4. CEO

Focused on business and users:

- Does this change actually move a business metric?
- What's the user impact — could this cause confusion or data loss?
- Is the risk worth the reward? Should this ship now or wait?

### 5. Devil's Advocate

Focused on challenging assumptions:

- Why was this approach chosen? What was considered and rejected?
- What breaks in production that tests don't catch?
- Is this change even necessary? What's the simplest alternative?

---

## Why five roles

A single reviewer tends to focus on their strongest domain. The five-role structure forces Claude to cover the full review surface: correctness + design + quality + business + adversarial thinking.

The roles are applied **sequentially** — each builds on the previous. The SWE finds the bug, the SA finds the architectural smell, the QA finds the missing test for the bug, the CEO weighs whether to defer the fix, and the Devil's Advocate asks whether the code path that has the bug should exist at all.

---

## Calibration by change size

From `review-prompting.md`:

| Change size     | Roles applied                             |
| --------------- | ----------------------------------------- |
| < 50 lines      | SWE + QA (focus on correctness and tests) |
| 50–300 lines    | All five roles                            |
| > 300 lines     | All five roles with emphasis on SA        |
| Breaking change | Always includes CEO and Devil's Advocate  |

---

## Next

- [review-prompting.md — output structure](review-prompting.md)
- [First review walkthrough](../getting-started/first-review.md)

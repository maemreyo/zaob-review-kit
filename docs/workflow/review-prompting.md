# review-prompting.md

Defines how Claude structures its review output and calibrates depth by change size.

---

## Output structure

Every review Claude returns follows this structure:

### 1. Summary

One paragraph: what changed, why, and the overall assessment. No bullet points — forces a coherent narrative.

### 2. Risk Assessment

**Low / Medium / High** with one sentence of justification.

- **Low**: cosmetic changes, tests only, documentation
- **Medium**: logic changes in non-critical paths, new features with fallback
- **High**: changes to auth, payment, data integrity, public API contracts

### 3. Findings

Organized by reviewer role. Each finding includes:

- Symbol: `✗` blocking · `⚠` suggestion · `✓` positive
- Location: `[filename:line]` when applicable
- Observation: what the problem is
- Recommendation: `→ Suggested fix or approach`

### 4. Recommendations

Prioritized action items, separated into:

- **Must fix before shipping** (blocking)
- **Should fix soon** (non-blocking but important)
- **Consider for future** (technical debt, improvements)

### 5. Verdict

One of: **Ship** · **Ship with changes** · **Needs rework**

---

## Calibration by change size

The agent calibrates review depth automatically based on the size of the diff:

| Change size      | Approach                                                                                        |
| ---------------- | ----------------------------------------------------------------------------------------------- |
| < 50 lines       | Focus on SWE correctness and test coverage. Skip CEO/DA unless triggered by keyword.            |
| 50–300 lines     | Full five-role review. Standard depth.                                                          |
| > 300 lines      | Full five-role with emphasis on SA (architectural review). Consider requesting a phased review. |
| Breaking changes | Always include CEO and Devil's Advocate regardless of size.                                     |

**Keywords that escalate depth:**

If the trigger sentence mentions: `auth`, `payment`, `database migration`, `public API`, `breaking change`, `security` — Claude applies the > 300 lines calibration regardless of actual diff size.

---

## Output tone

- Direct but constructive — "This will fail when X" not "This might potentially cause issues"
- Cite specific locations — `[src/auth/middleware.rs:47]` beats "somewhere in the auth module"
- Distinguish blocking from non-blocking — makes the verdict actionable
- Acknowledge good work — pure criticism makes reviewers defensive

---

## Example verdict section

```
## Verdict: Ship with changes

**Must fix:**
- [ ] Token expiry check at middleware.rs:47 (silent auth failure edge case)

**Should fix:**
- [ ] Remove Clone derive from Session (session.rs:23) — not a blocker but will cause confusion

**Consider:**
- [ ] Add integration test for the exact-expiry edge case
```

---

## Next

- [review-roles.md — the five perspectives](review-roles.md)
- [First review walkthrough](../getting-started/first-review.md)

# Review Memory

## Purpose

Persistent memory of review patterns. Agent reads `.materials/review_memory.md` BEFORE starting any new review.

## Entry Format

After each review, APPEND to `.materials/review_memory.md`:

```markdown
### [YYYY-MM-DD] <Scope: brief description>

- **Files reviewed**: <list>
- **Key findings**: <1-3 most important issues found>
- **Recurring from previous**: <yes/no — which pattern repeated>
- **Deferred**: <issues flagged but not fixed yet>
- **Patterns to watch**: <what to look for in future reviews>
```

## Usage Protocol

**Before new review:**

1. Read `.materials/review_memory.md` if it exists
2. Check "Deferred" items — were they addressed in this change?
3. Check "Patterns to watch" — did those patterns appear again?

**During review:**

- Note if a finding matches a "Recurring" pattern → flag as persistent issue

**After review:**

- Append new entry to `.materials/review_memory.md`
- If file does not exist: create it with the first entry

## Example Entry

### [2025-03-15] Last 2 commits — Add user auth endpoints

- **Files reviewed**: src/auth/handler.rs, src/models/user.rs
- **Key findings**: Missing rate limiting on /login; password compared before hashing
- **Recurring from previous**: Yes — input validation gaps (3rd occurrence)
- **Deferred**: Refactor token refresh logic (needs design discussion)
- **Patterns to watch**: Auth handlers, any new endpoint added without middleware

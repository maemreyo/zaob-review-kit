# First Review Walkthrough

A complete end-to-end walkthrough of your first code review using zrk.

**Pre-conditions:** `zrk init` has been run, your agent is open.

---

## 1. Make some commits

For this walkthrough, assume you just finished a feature:

```bash
git log --oneline -3
# a1b2c3 Add user authentication middleware
# d4e5f6 Refactor session handling
# g7h8i9 Update login endpoint validation
```

## 2. Trigger the review

In your agent's chat, type:

```
tạo materials để review 3 commit gần nhất
```

Or in English:

```
create review materials for the last 3 commits
```

The agent reads `prep-review.md` and starts working.

---

## 3. What the agent does

The agent executes these steps (defined in the installed `prep-review.md`):

1. **Identify scope** — parses "3 commit gần nhất" → last 3 commits
2. **Generate diff** — `git diff HEAD~3..HEAD`
3. **Resolve context** — finds files imported by or importing the changed files
4. **Apply ignore rules** — filters out `node_modules/`, `*.lock`, binaries, etc.
5. **Read `.archignore`** — applies your project-specific exclusions
6. **Build `review_context.xml`** — packages everything into structured XML
7. **Generate `review_prompt.md`** — creates the structured review prompt
8. **Save to `.materials/`** — writes with a timestamp prefix

---

## 4. The output files

```
.materials/
  20250304_a1b2c3/
    review_context.xml
    review_prompt.md
    review.patch
```

### review_context.xml (annotated)

```xml
<review_context>
  <metadata>
    <project>my-app</project>
    <date>2025-03-04</date>
    <scope>last 3 commits (a1b2c3..g7h8i9)</scope>
    <author>Trung Ngo</author>
  </metadata>

  <diff>
    <!-- Full diff — not truncated -->
    diff --git a/src/auth/middleware.rs b/src/auth/middleware.rs
    ...
  </diff>

  <context_files>
    <!-- Files imported by or importing the changed files -->
    <file path="src/auth/session.rs">
      // Full content of session.rs
      ...
    </file>
    <file path="src/auth/mod.rs">
      ...
    </file>
  </context_files>

  <project_context>
    <!-- Content of project-context.md, if present -->
    ## Project Overview
    A Rust web application...
  </project_context>
</review_context>
```

### review_prompt.md (annotated)

```markdown
# Code Review

## Context

Review the authentication changes across 3 commits.
Full diff and context files are attached as review_context.xml.

## Review Roles

Please review from each of these perspectives in order:

### 1. Senior Software Engineer (SWE)

- Code correctness, edge cases, error handling
- Performance implications
- Code style and readability

### 2. Software Architect (SA)

- Design patterns, architectural consistency
- Separation of concerns, dependency management
- Scalability implications

### 3. Quality Assurance (QA)

- Test coverage gaps
- Integration risks, regression potential

### 4. CEO

- Business value alignment
- User impact, risk vs. reward

### 5. Devil's Advocate

- Challenge assumptions
- What could go wrong in production?

## Output Structure

For each role, provide:

- Key findings (blocking issues vs suggestions)
- Specific line references where applicable

End with:

- **Risk level**: Low / Medium / High
- **Verdict**: Ship / Ship with changes / Needs rework
```

---

## 5. Upload to Claude.ai

1. Go to [claude.ai](https://claude.ai)
2. Start a new conversation
3. Click the paperclip icon — attach **both files**:
   - `review_context.xml`
   - `review_prompt.md`
4. Send (no extra message needed — the prompt file contains everything)

---

## 6. What Claude returns

Claude follows the prompt structure and returns:

```
## Summary
The authentication middleware adds session validation to all protected routes...

## Risk Assessment: Medium
The session token validation logic has one edge case that could cause silent auth failures...

## Findings

### Senior SWE
✗ [auth/middleware.rs:47] Token expiry check uses `<` instead of `<=`. Tokens expiring
  at exactly the current timestamp will be rejected with a confusing error.
  → Change: `if token.expires_at < now` → `if token.expires_at <= now`

⚠ [auth/session.rs:23] Session struct derives `Clone` but contains a raw database handle.
  → Consider wrapping in Arc or removing Clone derive.

### Software Architect
✓ Session management is properly separated from request handling.
⚠ The middleware assumes a single auth provider — consider an AuthProvider trait
  for future extensibility.

...

## Verdict: Ship with changes
Fix the token expiry edge case (blocking). Session Clone is a suggestion, not blocking.
```

---

## 7. After the review

**Commit the review memory:**

```bash
git add .materials/review_memory.md
git add .materials/REVIEW_LOG.md
git commit -m "chore: update review memory after auth review"
```

**Leave the per-review output gitignored** — it's too large to commit and you can regenerate it. The gitignore rules were added by `zrk install-all`.

---

## Next

- [CLI commands reference](../commands/overview.md)
- [How the review workflow files work](../workflow/overview.md)
- [Customizing review ignore patterns](../reference/archignore.md)

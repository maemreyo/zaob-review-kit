# Prep Review

Prepare review materials for uploading to Claude.ai.

## Trigger

User says something like:

- "tạo materials để review 3 commit gần nhất"
- "prep review for the last commit"
- "pack the auth changes for review"
- "tạo materials liên quan tới phase 0"
- "pack everything related to authentication"

---

## Steps

0. **Author self-review first** — Before packaging anything for an AI or peer
   reviewer, read your own diff as if you were the reviewer:

   ```bash
   git diff HEAD~<N>..HEAD   # or: git diff main...your-branch
   ```

   Check for obvious issues: forgotten debug code, missing error handling,
   TODOs that were meant to be resolved, tests that only pass trivially.
   Self-review catches the easy stuff so the actual review focuses on what matters.

1. **Create dirs and initialise temp/ working files immediately**

   Do this FIRST — before reading any file content. The temp files are stubs
   the agent fills in progressively as it works through subsequent steps.

   ```bash
   TS=$(date +%Y%m%d-%H%M%S)
   mkdir -p .materials/$TS/reports
   mkdir -p .materials/$TS/temp
   ```

   Write `.materials/$TS/temp/role-plan.md` (fill in step 2):

   ```markdown
   # Role Plan

   _Written before reading any file content. Based on diff --name-only only._

   ## Triggered roles

   | #   | Role | File / pattern that triggered it |
   | --- | ---- | -------------------------------- |

   ## Execution order

   <!-- e.g.: 01-swe → 02-sa → 03-qa → 05-se → 99-verdict -->

   ## Additional roles (from review_prompt.md)

   <!-- Any user-specified extras and why -->

   ## Skipped roles (from review_prompt.md)

   <!-- Any suppressions and why -->
   ```

   Write `.materials/$TS/temp/file-map.md` (fill in step 3):

   ```markdown
   # File → Role Map

   _Fill while scanning diff content, one row per changed file._

   | File | Change type | Roles | Key observation |
   | ---- | ----------- | ----- | --------------- |
   ```

   Write `.materials/$TS/temp/findings.md` (append during review execution):

   ```markdown
   # Running Findings Log

   _Append one entry per [BLOCKER] or [MAJOR] finding immediately after writing
   each role file. 99-verdict.md reads THIS file — not the individual role files —
   to synthesise without re-loading 60K+ tokens of role output._

   <!-- Format: [ROLE][SEVERITY] path:line — short description -->
   ```

2. **Identify scope** — Two common modes. Fill `temp/role-plan.md` as you go.

   **Mode A — X latest commits:**

   ```bash
   git log --oneline -<N>              # confirm scope
   git diff HEAD~<N>..HEAD --name-only # list changed files
   ```

   After seeing the file names: fill the **Triggered roles** and **Execution order**
   sections of `temp/role-plan.md`. Use the trigger table in `review-roles.md`.
   Do not read file content yet — role decisions from names alone are enough.

   **Mode B — Content related to Y** (feature, phase, module name):

   ```bash
   rg -l "phase.0\|Phase 0\|phase_0" --type-not sql   # example: phase 0
   rg -l "auth\|login\|jwt" src/                       # example: auth
   rg -l "student" migrations/ eduos-core/ eduos-api/  # example: module name
   ```

   `rg -l` returns only filenames — ready to pipe straight into repomix.
   Install: `brew install ripgrep` / `apt install ripgrep`.

3. **Resolve context and fill file-map** — Expand the file list with direct imports
   and caller files (one level deep). Fill `temp/file-map.md` as you read each file:

   ```bash
   # Forward: what does the changed file import?
   grep -E "^use |^mod |^import |^from |^require" <file> | head -20

   # Reverse: what imports this file? (composition layer)
   rg -l "mod students|use.*routes.*students" src/
   ```

   Common patterns:
   - Routes changed → include the router composition file (`src/lib.rs`, `app.rs`, `main.rs`)
   - Models changed → include the lib re-export file
   - Services changed → include the handler that calls them

4. **Check token budget** — Before packing, run repomix's built-in token visualizer:

   ```bash
   # Commit-based
   git diff HEAD~<N>..HEAD --name-only | repomix --stdin --token-count-tree

   # Content-based
   rg -l "phase 0" | repomix --stdin --token-count-tree
   ```

   Target: **< 100K tokens** for Claude.ai. If over, narrow the list before packing
   (drop test files or large auto-generated files first).

5. **Apply ignore rules** — Filter out files matching:
   - `.archignore`
   - `.repomixignore` (if present)
   - patterns from `review-ignore.md`

6. **Collect documentation files** — Include project documentation so reviewers
   can verify code changes align with documented architecture and design decisions.

   **README files:**

   ```bash
   find . -name "README.md" -o -name "README.txt" -o -name "README"
   ```

   **Documentation directories:** `docs/`, `documentation/`, `.github/`

   **Architecture & API specs:**
   - `DESIGN.md`, `ARCHITECTURE.md`, `ADR-*.md`
   - `openapi.yaml`, `openapi.json`, `swagger.yaml`
   - `*.mmd`, `*.puml` — Mermaid / PlantUML diagrams

7. **Generate review_context.xml** — use `--stdin` to pass the file list.
   See `pack-materials.md` for the full command reference.

   **Mode A — commits:**

   ```bash
   git diff HEAD~<N>..HEAD --name-only | repomix --stdin \
     --include-diffs \
     --include-logs-count <N> \
     --include-full-directory-structure \
     --style xml \
     --output .materials/$TS/review_context.xml
   ```

   **Mode B — content/topic:**

   ```bash
   rg -l "<keyword>" | repomix --stdin \
     --include-full-directory-structure \
     --style xml \
     --output .materials/$TS/review_context.xml
   ```

   Do **not** add `--compress` unless the user asks. See `pack-materials.md`.

8. **Save patch** (Mode A only):

   ```bash
   git diff HEAD~<N>..HEAD > .materials/$TS/review.patch
   ```

9. **Scaffold reports/ stubs** — Now that scope is known (from temp/file-map.md),
   pre-create the two bookend files. The agent fills these in during the review.

   Write `.materials/$TS/reports/00-summary.md`:

   ```markdown
   # Review Summary

   **Scope**: <N commits / topic description>
   **Date**: <today>
   **Risk**: <!-- Low / Medium / High -->
   **Effort**: <!-- [x/5] -->

   ## What Changed

   <!-- One paragraph: what changed and why -->

   ## File Walkthrough

   | File | Change type | What changed | Notes |
   | ---- | ----------- | ------------ | ----- |

   <!-- Populate from temp/file-map.md -->

   ## Risk Assessment

   **Level**: <!-- Low / Medium / High -->
   **Justification**: <!-- one sentence -->
   **Review Effort**: <!-- [x/5] and brief explanation -->

   ## Review Files

   <!-- Table of contents — fill in after all role files are written -->
   ```

   Write `.materials/$TS/reports/99-verdict.md`:

   ```markdown
   [← Back to Summary](00-summary.md)

   # Verdict

   ## Suggested Tests

   <!-- Use format from review-prompting.md §Suggested Tests Format -->

   ## Prioritised Recommendations

   <!-- Synthesised from temp/findings.md — do not re-read individual role files -->

   ### Blockers (fix before merge)

   ### Major (fix soon)

   ### Suggestions (optional)

   ## Final Verdict

   <!-- Ship / Ship with changes / Needs rework / Needs discussion -->

   ---

   _AI-generated review — human sign-off required for any [BLOCKER] or [MAJOR] finding._
   ```

10. **Generate review_prompt.md** — Write a structured prompt:

    ```markdown
    # Code Review Request

    **Scope**: last <N> commits — <brief description>
    **Date**: <today>
    **Context file**: review_context.xml
    **Output directory**: reports/
    **Working files**: temp/role-plan.md, temp/file-map.md, temp/findings.md

    ## Focus areas

    - Correctness of logic
    - Edge cases and error handling
    - Code clarity and maintainability
    - Security concerns (if applicable)

    ## Specific questions

    <any specific questions the user mentioned>

    ## Additional Roles

    <!-- Uncomment or add roles to activate beyond auto-triggered defaults     -->
    <!-- Available: pe, se, oe, de, ux, cl, ceo, da, mle, api, finops, dx     -->
    <!-- Example: - mle   (this PR integrates an LLM API)                      -->
    <!-- Example: - finops (new cloud resources provisioned)                   -->

    ## Skip Roles

    <!-- Uncomment to suppress auto-triggered roles that are not relevant here -->
    <!-- Example: - ceo   (internal refactor only, no user impact)             -->
    ```

    Save to `.materials/$TS/review_prompt.md`

11. **Report to user**

```
✓ Materials ready in .materials/<TS>/
  - review_context.xml      (<size>, ~<tokens> tokens)
  - review_prompt.md
  - review.patch            (Mode A only)
  - reports/
      00-summary.md         ← stub pre-scaffolded, fill in
      99-verdict.md         ← stub pre-scaffolded, fill in
  - temp/
      role-plan.md          ← DONE: roles decided from file names
      file-map.md           ← DONE: file → role mapping complete
      findings.md           ← append findings here during review

Upload review_context.xml + review_prompt.md to Claude.ai.
Finished review files land in reports/ — copy to project reports/ when done.
```

---

## Output

- `.materials/<timestamp>/review_context.xml` — generated by repomix
- `.materials/<timestamp>/review_prompt.md`
- `.materials/<timestamp>/review.patch` (commit mode only)
- `.materials/<timestamp>/reports/` — all review output files
- `.materials/<timestamp>/temp/` — agent working files (ephemeral)

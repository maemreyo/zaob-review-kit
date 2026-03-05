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
   and caller files (one level deep). **Fill `temp/file-map.md` as you read each file —
   do this NOW, during prep, not later during review execution.**

   ```bash
   # Forward: what does the changed file import?
   grep -E "^use |^mod |^import |^from |^require" <file> | head -20

   # Reverse: what imports this file? (composition layer)
   rg -l "mod students|use.*routes.*students" src/
   ```

   After reading each file's diff, immediately append a row to `temp/file-map.md`:

   ```
   | src/auth/handler.rs | Modified | SWE, SE | token validation logic changed |
   ```

   Do not batch this for the end — fill row-by-row as you go. The summary's
   File Walkthrough table in step 9 should be populated from `temp/file-map.md`.

   Common context patterns:
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

6. **Generate review_context.xml** — pipe directly into repomix. Never write an
   intermediate file list.

   ### ⛔ Anti-patterns — never buffer through intermediate files

<!-- agent:kiro:start -->

> **Kiro:** Both patterns below also hang Kiro's terminal tool.
> The heredoc (`cat > file << 'EOF'`) with 50+ lines freezes the terminal completely.
> Always pipe directly — never buffer file lists through temp files or heredocs.

<!-- agent:kiro:end -->

```bash
# ❌ NEVER do this — heredoc with many lines; hangs terminal-based agents
cat > .materials/$TS/temp/file-list.txt << 'EOF'
src/auth/handler.rs
migrations/001.sql
... (50 more lines)
EOF

# ❌ NEVER do this — writing the list to a file then reading it back is pointless
git diff ... --name-only > /tmp/files.txt
repomix --include "$(cat /tmp/files.txt | tr '\n' ',')" ...
```

### ✅ Always pipe directly — one command, no intermediate files

**Mode A — X commits:**

```bash
# Source files only (fastest, default)
git diff HEAD~<N>..HEAD --name-only | repomix --stdin \
  --include-diffs \
  --include-logs-count <N> \
  --style xml \
  --output .materials/$TS/review_context.xml

# Source files + architecture docs (recommended for feature PRs)
{ git diff HEAD~<N>..HEAD --name-only; \
  find docs/ -name "*.md" ! -path "*/reviews/*" 2>/dev/null; \
  test -f README.md && echo README.md; } | sort -u | \
repomix --stdin \
  --include-diffs \
  --include-logs-count <N> \
  --style xml \
  --output .materials/$TS/review_context.xml
```

**Mode B — content/topic:**

```bash
{ rg -l "<keyword>" src/; \
  find docs/ -name "*.md" ! -path "*/reviews/*" 2>/dev/null; } | sort -u | \
repomix --stdin \
  --style xml \
  --output .materials/$TS/review_context.xml
```

### Documentation inclusion rules

When adding docs to the pipeline:

- ✅ Include: `docs/architecture/`, `docs/tdd/`, `docs/prompts/`, `README.md`, `AGENTS.md`, `CLAUDE.md`
- ❌ Exclude: `docs/reviews/` — previous review outputs inflate token budget with stale analysis
- ❌ Exclude: binary files (`*.png`, `*.drawio`) unless specifically asked

The shell pattern `! -path "*/reviews/*"` handles this automatically.

Do **not** add `--compress` unless the user asks. See `pack-materials.md`.

7. **Save patch** (Mode A only):

   ```bash
   git diff HEAD~<N>..HEAD > .materials/$TS/review.patch
   ```

8. **Scaffold reports/ stubs** — Now that scope is known (from temp/file-map.md),
   pre-create the two bookend files. The agent fills these in during the review.

   Write `.materials/$TS/reports/00-summary.md`:

   ```markdown
   # Review Summary

   **Scope**: <N commits / topic description>
   **Date**: <today>
   **Risk**: <Low / Medium / High — decide from file-map change types>
   **Effort**: <[x/5] — decide from total files changed and complexity>

   ## What Changed

   <!-- One paragraph: what changed and why -->

   ## File Walkthrough

   <!-- Copy from temp/file-map.md and expand the Notes column -->

   | File | Change type | What changed | Notes |
   | ---- | ----------- | ------------ | ----- |

   ## Risk Assessment

   **Level**: <Low / Medium / High>
   **Justification**: <!-- one sentence -->
   **Review Effort**: <[x/5] — see review-prompting.md §Review Effort scale>

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

9. **Generate review_prompt.md** — Write a structured prompt following the Prompt Anatomy.
   Fill in all `<placeholder>` values from the actual scope:

   ```markdown
   # Code Review Request

   I want you to perform a structured multi-role code review so that every
   significant risk — correctness, security, architecture, performance — is
   surfaced with actionable findings before this code is merged.

   ## Context Files

   Read these files completely before responding:

   - `review_context.xml` — full source and git diff for the scope below
   - `role_standards_bundle.md` — checklists for every activated role; read one
     section at a time (sequential read-write-proceed, never all at once)
   - `review-prompting.md` — output structure, role protocol, severity labels
   - `review-roles.md` — which roles to activate and their trigger conditions
   - `temp/role-plan.md` — pre-decided execution plan (read this first)
   - `temp/file-map.md` — file-to-role mapping built during prep
   - `temp/findings.md` — append every [BLOCKER] and [MAJOR] here as you go

   ## Scope

   **Commits**: last <N> commits — <brief description of what changed>
   **Date**: <today>
   **Output directory**: `reports/`

   ## Reference: What a Good Review Looks Like

   Always:

   - Cite specific file paths and line numbers for every finding
   - Complete the sequential read-write-proceed pattern (one role at a time)
   - Append to `temp/findings.md` immediately after writing each role file
   - Write 99-verdict.md by reading `temp/findings.md` only — never re-read role files

   Never:

   - Pre-load all role standards at once — read one section from the bundle,
     write the role file, proceed to the next section
   - Skip the 00-summary.md or 99-verdict.md bookend files
   - Leave a finding without a severity label ([BLOCKER] / [MAJOR] / [SUGGESTION] / [NIT])

   ## Success Brief

   **Output**: Multiple Markdown files in `reports/` (00-summary, role files, 99-verdict)
   **Success means**: Author can act on every [BLOCKER] and [MAJOR] immediately —
   each finding has a file path, line number, explanation, and suggested fix
   **Does NOT sound like**: vague AI filler, generic advice without code references,
   or a summary-only verdict that skips individual role perspectives

   ## Rules

   `review-prompting.md` contains the complete standards, severity labels, verdict
   scale, and role-loading protocol. Read it before starting. If you are about to
   skip a required step (e.g., omit temp/findings.md, merge roles, skip the plan),
   stop and tell me instead.

   ## Specific Questions

   <paste any user-specified questions or focus areas here>

   ## Additional Roles

   <!-- Activate roles beyond auto-triggered defaults (see review-roles.md)   -->
   <!-- Available: pe, se, oe, de, ux, cl, ceo, da, mle, api, finops, dx     -->
   <!-- Example: - mle   (this PR integrates an LLM API)                      -->
   <!-- Example: - finops (new cloud resources provisioned)                   -->

   ## Skip Roles

   <!-- Suppress auto-triggered roles that are not relevant for this scope    -->
   <!-- Example: - ceo   (internal refactor only, no user-visible impact)     -->

   ## Plan Before Executing

   Before writing any review output, state:

   1. The 3 rules from `review-prompting.md` that matter most for this scope
   2. Your role execution order (e.g. 01-swe → 02-sa → 03-qa → 05-se → 99-verdict)

   Only begin executing once you have written the plan above.
   ```

   Save to `.materials/$TS/review_prompt.md`

10. **Bundle role standards for Claude.ai upload**

    Claude.ai has no access to local `.kiro/steering/role-standards/` files.
    Bundle the standards needed for this review into a single uploadable file.

    Read `temp/role-plan.md` to get the execution order, then bundle only the
    triggered roles — always include the three core roles (swe, sa, qa) plus
    every triggered role from the plan:

    ```bash
    STANDARDS_DIR=".kiro/steering/role-standards"
    OUT=".materials/$TS/role_standards_bundle.md"

    # Always-present core roles
    ROLES="00-loading-guide 01-swe-standard 02-sa-standard 03-qa-standard"

    # Add triggered roles from temp/role-plan.md
    # Example: if plan shows 05-se and 07-de are triggered:
    # ROLES="$ROLES 05-se-standard 07-de-standard"

    printf '' > "$OUT"
    for role in $ROLES; do
      FILE="$STANDARDS_DIR/${role}.md"
      if [ -f "$FILE" ]; then
        echo "---" >> "$OUT"
        cat "$FILE" >> "$OUT"
        echo "" >> "$OUT"
      fi
    done
    ```

    **Why bundle, not copy individual files?**
    Claude.ai has a per-session file upload limit. One bundled file uses one
    upload slot regardless of how many roles are active. The `---` separators
    between role sections are enough for Claude to distinguish them; headers
    inside each standard already carry the role name.

    **Fallback — bundle all standards (when unsure which will be triggered):**

    ```bash
    cat .kiro/steering/role-standards/*.md \
      | awk 'FNR==1{print "---"}1' \
      > .materials/$TS/role_standards_bundle.md
    ```

    This is heavier (~60K tokens) but guarantees nothing is missing. Use the
    targeted bundle when the triggered role set is clear from the file names.

11. **Report to user**

```
✓ Materials ready in .materials/<TS>/
  - review_context.xml          (<size>, ~<tokens> tokens)
  - role_standards_bundle.md    (<size>, ~<tokens> tokens)
  - review_prompt.md
  - review.patch                (Mode A only)
  - reports/
      00-summary.md             ← stub pre-scaffolded, fill in
      99-verdict.md             ← stub pre-scaffolded, fill in
  - temp/
      role-plan.md              ← DONE: roles decided from file names
      file-map.md               ← DONE: file → role mapping complete
      findings.md               ← append findings here during review

Upload order to Claude.ai:
  1. review_context.xml
  2. role_standards_bundle.md
  3. Paste review_prompt.md as your message

Finished review files land in reports/ — copy to project reports/ when done.
```

---

## Output

- `.materials/<timestamp>/review_context.xml` — generated by repomix
- `.materials/<timestamp>/role_standards_bundle.md` — role checklists for Claude.ai
- `.materials/<timestamp>/review_prompt.md`
- `.materials/<timestamp>/review.patch` (commit mode only)
- `.materials/<timestamp>/reports/` — all review output files
- `.materials/<timestamp>/temp/` — agent working files (ephemeral)

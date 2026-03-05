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

1. **Identify scope** — Two common modes:

   **Mode A — X latest commits:**

   ```bash
   git log --oneline -<N>              # confirm scope
   git diff HEAD~<N>..HEAD --name-only # list changed files
   ```

   **Mode B — Content related to Y** (feature, phase, module name):

   ```bash
   # Find all files that mention the topic (respects .gitignore)
   rg -l "phase.0\|Phase 0\|phase_0" --type-not sql   # example: phase 0
   rg -l "auth\|login\|jwt" src/                       # example: auth
   rg -l "student" migrations/ eduos-core/ eduos-api/  # example: module name
   ```

   `rg -l` (ripgrep `--files-with-matches`) returns only filenames, one per line —
   ready to pipe straight into repomix. Install: `brew install ripgrep` / `apt install ripgrep`.

2. **Resolve context** — Expand the file list with direct imports and caller files
   (one level deep in each direction):

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

3. **Check token budget** — Before packing, run repomix's built-in token visualizer
   on the candidate file list to see if it fits:

   ```bash
   # Commit-based — pipe git file list
   git diff HEAD~<N>..HEAD --name-only | repomix --stdin --token-count-tree

   # Content-based — pipe rg file list
   rg -l "phase 0" | repomix --stdin --token-count-tree
   ```

   Target: **< 100K tokens** for Claude.ai. If over, narrow the list before packing
   (drop test files or large auto-generated files first).

4. **Apply ignore rules** — Filter out files matching:
   - `.archignore`
   - `.repomixignore` (if present)
   - patterns from `review-ignore.md`

5. **Create timestamp dir and temp directory**

   ```bash
   TS=$(date +%Y%m%d-%H%M%S)
   mkdir -p .materials/$TS/temp
   ```

   The `temp/` subdirectory is for agent working files during the review process:
   - File list caching
   - Partial analysis results
   - Draft content and notes

   This keeps intermediate files organized within the materials folder rather than
   scattered across system temp locations.

6. **Collect documentation files** — Include project documentation in the review
   context so reviewers can verify code changes align with documented architecture,
   API contracts, and design decisions.

   Documentation files to include (respecting `.archignore` and `.repomixignore`):

   **README files:**

   ```bash
   find . -name "README.md" -o -name "README.txt" -o -name "README"
   ```

   **Documentation directories:**
   - `docs/`
   - `documentation/`
   - `.github/`

   **Architecture diagrams** (in docs/ directories):
   - `*.drawio` — Draw.io diagrams
   - `*.mmd` — Mermaid diagrams
   - `*.puml` — PlantUML diagrams
   - `*.svg`, `*.png` — Rendered diagrams (include selectively, may be large)

   **API specifications:**
   - `openapi.yaml`, `openapi.json`
   - `swagger.yaml`
   - `api-spec.md`

   **Design documents:**
   - `DESIGN.md`
   - `ARCHITECTURE.md`
   - `ADR-*.md` — Architecture Decision Records

   These files will be included in `review_context.xml` alongside source code.
   The Software Architect review should reference these when evaluating
   architectural consistency. If documentation contradicts code changes, the
   reviewer should flag the discrepancy.

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

9. **Generate review_prompt.md** — Write a structured prompt:

   ```markdown
   # Code Review Request

   **Scope**: last <N> commits — <brief description>
   **Date**: <today>
   **Context file**: review_context.xml
   **Temp directory**: temp/

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

   ## Working Files

   Use the `temp/` directory for any intermediate working files during the review:

   - File lists and caching
   - Draft findings and analysis notes
   - Partial results before final output

   This directory may be cleaned up after review or left for debugging purposes.
   ```

   Save to `.materials/$TS/review_prompt.md`

10. **Report to user**

```
✓ Materials ready in .materials/<TS>/
  - review_context.xml  (<size>, ~<tokens> tokens)
  - review_prompt.md
  - review.patch        (Mode A only)
  - temp/               (agent working directory)

Upload review_context.xml to Claude.ai, then paste review_prompt.md.
```

## Output

- `.materials/<timestamp>/review_context.xml` — generated by repomix
- `.materials/<timestamp>/review_prompt.md`
- `.materials/<timestamp>/review.patch` (commit mode only)
- `.materials/<timestamp>/temp/` — agent working directory (created empty)

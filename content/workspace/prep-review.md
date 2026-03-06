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

1. **Identify scope** — determine the right input for `zrk prep`.

   Three modes depending on what the user asked for:

   **Mode A — git range (most common):**

   ```bash
   git log --oneline -<N>   # confirm which commits are in scope
   ```

   Produces input: `HEAD~<N>..HEAD` or `feature/branch..main`

   **Mode B — non-contiguous commits:**

   ```bash
   git log --oneline   # identify specific commit hashes
   ```

   Produces input: `abc123 def456 ghi789` (space-separated hashes)

   **Mode C — content / topic search:**

   Produces input: `--topic "<keyword>"`

2. **Check token budget** — Before running prep, estimate context size to
   avoid generating a file too large for Claude.ai (target: < 100K tokens):

   ```bash
   # Mode A / B — commit-based
   git diff HEAD~<N>..HEAD --name-only | repomix --stdin --token-count-tree

   # Mode C — topic-based
   rg -l "<keyword>" | repomix --stdin --token-count-tree
   ```

   If over budget, plan to add architecture docs only if the review needs them.
   The default `zrk prep` repomix call covers source only. For advanced packing
   with custom file inclusion — see `pack-materials.md`.

3. **Run `zrk prep`** — one command creates the entire materials folder:

   ```bash
   # Mode A — git range
   zrk prep HEAD~<N>..HEAD

   # Mode A — branch diff
   zrk prep feature/auth..main

   # Mode B — non-contiguous commits
   zrk prep abc123 def456 ghi789

   # Mode C — topic search (requires ripgrep)
   zrk prep --topic "phase-0"

   # Preview without writing anything
   zrk prep HEAD~3..HEAD --dry-run
   ```

   `zrk prep` does automatically:
   - Creates `.materials/<TS>/` with `standards/`, `reports/`, `temp/`
   - Auto-detects triggered roles from changed filenames → writes `temp/role-plan.md`
   - Copies only the needed role standard files into `standards/`
   - Generates a self-contained `review_prompt.md` (inline protocol, no external refs)
   - Generates `UPLOAD_ORDER.md` with the exact zip command and file list
   - Runs repomix → `review_context.xml`
   - Saves `review.patch` (git modes only)

   After running, note the timestamp directory printed in output.

4. **Resolve context if needed** — `zrk prep` runs a basic repomix over the
   changed files. If the review needs architecture docs or composition files,
   regenerate `review_context.xml` manually:

   ```bash
   TS=<timestamp from step 3>

   # Add architecture docs alongside source files
   { git diff HEAD~<N>..HEAD --name-only; \
     find docs/ -name "*.md" ! -path "*/reviews/*" 2>/dev/null; \
     test -f README.md && echo README.md; } | sort -u | \
   repomix --stdin \
     --include-diffs \
     --include-logs-count <N> \
     --style xml \
     --output .materials/$TS/review_context.xml
   ```

   Common context patterns:
   - Routes changed → include the router composition file (`src/lib.rs`, `app.rs`, `main.rs`)
   - Models changed → include the lib re-export file
   - Services changed → include the handler that calls them

   For full repomix options — see `pack-materials.md`.

   ### ⛔ Anti-patterns — never buffer through intermediate files

<!-- agent:kiro:start -->

> **Kiro:** Both patterns below also hang Kiro's terminal tool.
> The heredoc (`cat > file << 'EOF'`) with 50+ lines freezes the terminal completely.
> Always pipe directly — never buffer file lists through temp files or heredocs.

<!-- agent:kiro:end -->

```bash
# ❌ NEVER do this
git diff ... --name-only > /tmp/files.txt
repomix --include "$(cat /tmp/files.txt | tr '\n' ',')" ...
```

5. **Fill `temp/file-map.md`** — read the diff in `review_context.xml` and
   fill one row per changed file. Do this NOW, during prep, not during review.

   ```bash
   # Forward: what does this file import?
   grep -E "^use |^mod |^import |^from |^require" <file> | head -20

   # Reverse: who imports this file?
   rg -l "mod auth|use.*routes.*auth" src/
   ```

   Append row-by-row as you read each file:

   ```
   | src/auth/handler.rs | Modified | SWE, SE | token validation logic changed |
   ```

6. **Refine `temp/role-plan.md`** — `zrk prep` pre-fills roles from filename
   patterns. Check for false positives or missing roles only apparent from file
   content, then update `review_prompt.md` accordingly:
   - Add roles: fill `## Additional Roles` in `review_prompt.md`
   - Remove roles: fill `## Skip Roles` in `review_prompt.md`

7. **Fill `## Specific Questions`** in `review_prompt.md` — paste any
   user-specified focus areas or questions. Leave blank if none.

8. **Report to user**

```
✓ Materials ready in .materials/<TS>/
  - review_context.xml          ← source + diff context
  - review_prompt.md            ← paste this as your Claude.ai message
  - review.patch                (git modes only)
  - standards/
      00-loading-guide.md
      01-swe-standard.md
      02-sa-standard.md
      03-qa-standard.md
      NN-xx-standard.md         ← triggered roles only
  - reports/
      00-summary.md             ← stub, fill LAST after all role files done
      99-verdict.md             ← stub, fill from temp/findings.md only
  - temp/
      role-plan.md              ← DONE: pre-filled, refine if needed
      file-map.md               ← DONE: fill during prep step 5
      findings.md               ← append [BLOCKER]/[MAJOR] during review

See UPLOAD_ORDER.md for exact zip command and upload sequence.
```

---

## Output

- `.materials/<timestamp>/review_context.xml` — generated by repomix
- `.materials/<timestamp>/review_prompt.md` — self-contained, paste as message
- `.materials/<timestamp>/review.patch` (commit mode only)
- `.materials/<timestamp>/standards/` — individual role standard files
- `.materials/<timestamp>/reports/` — all review output files
- `.materials/<timestamp>/temp/` — agent working files (ephemeral)
- `.materials/<timestamp>/UPLOAD_ORDER.md` — zip command + upload sequence

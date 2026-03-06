# Prep Review

Prepare review materials for uploading to Claude.ai.

## Trigger

User says something like:

- "prep review for the last 3 commits"
- "tạo materials để review commit này"
- "pack the auth changes for review"
- "review everything related to phase 0"
- "pack changes related to authentication"

---

## Steps

0. **Author self-review first** — Before packaging anything, read your own diff:

   ```bash
   git diff HEAD~<N>..HEAD   # or: git diff main...feature-branch
   ```

   Check for obvious issues: forgotten debug code, missing error handling,
   TODOs meant to be resolved, tests that only pass trivially.

1. **Identify scope + relevant docs**

   **Determine scope mode:**

   - **Mode A — git range**: `git log --oneline -<N>` to confirm commits
   - **Mode B — non-contiguous commits**: note the specific hashes
   - **Mode C — topic search**: identify the keyword

   **Find relevant docs** — check `project-context.md` first:

   ```bash
   # Does this project have a Documentation Map?
   grep -A 20 "## Documentation Map" .kiro/steering/project-context.md 2>/dev/null

   # What docs exist?
   find docs/ -name "*.md" | sort 2>/dev/null
   ls *.md 2>/dev/null   # root-level: README, AGENTS, CLAUDE, CONTRIBUTING

   # User referenced a specific file (e.g. "#1b.md", "spec for phase 2")?
   find . -iname "*<name>*" 2>/dev/null | grep -v ".materials"
   ```

   The `Documentation Map` in `project-context.md` (if filled in) maps keywords
   to doc files. `zrk prep` reads this automatically and suggests relevant docs
   in its output — check the suggestion before deciding what to include.

2. **Check token budget** — target < 150K tokens:

   ```bash
   git diff HEAD~<N>..HEAD --name-only | repomix --stdin --token-count-tree
   ```

   If over budget, drop test files or large auto-generated files first.

3. **Run `zrk prep`**

   ```bash
   # Mode A — git range
   zrk prep HEAD~<N>..HEAD

   # Mode B — non-contiguous commits
   zrk prep abc123 def456 ghi789

   # Mode C — topic search
   zrk prep --topic "<keyword>"

   # With specific extra files the user referenced
   zrk prep <scope> --include <file1> <file2>

   # With an entire docs folder (all .md, skips reviews/)
   zrk prep <scope> --docs <docs-folder>

   # Both
   zrk prep <scope> --docs <docs-folder> --include README.md AGENTS.md
   ```

   `zrk prep` will print suggested docs from the Documentation Map if found.
   If relevant, re-run with `--include` to add them.

   `zrk prep` does automatically:
   - Creates `.materials/<TS>/` with `standards/`, `reports/`, `temp/`
   - Auto-detects triggered roles from changed filenames → `temp/role-plan.md`
   - Copies only needed role standard files into `standards/`
   - Generates self-contained `review_prompt.md`
   - Generates `UPLOAD_ORDER.md` with zip command
   - Runs repomix → `review_context.xml`
   - Saves `review.patch` (git modes only)

4. **Resolve context if needed** — only if `--docs` doesn't cover it
   (e.g. need non-`.md` files or router entry-points):

   ```bash
   TS=<timestamp from step 3>

   { git show --name-only --format="" <hash1> <hash2> | sort -u; \
     find <docs-folder> -name "*.md" ! -path "*/reviews/*"; } | sort -u | \
   repomix --stdin --include-diffs --style xml \
     --output .materials/$TS/review_context.xml
   ```

<!-- agent:kiro:start -->
   > **Kiro:** Never buffer file lists through temp files or heredocs — hangs terminal.
<!-- agent:kiro:end -->

5. **Fill `temp/file-map.md`** — one row per changed file, during prep:

   ```
   | src/auth/handler.rs | Modified | SWE, SE | token validation logic changed |
   ```

6. **Refine `temp/role-plan.md`** — check for false positives or missing
   roles only apparent from file content:
   - Add: fill `## Additional Roles` in `review_prompt.md`
   - Remove: fill `## Skip Roles` in `review_prompt.md`

7. **Fill `## Specific Questions`** in `review_prompt.md` — user-specified
   focus areas. Leave blank if none.

8. **Report to user**

```
✓ Materials ready in .materials/<TS>/
  - review_context.xml
  - review_prompt.md            ← paste as Claude.ai message
  - review.patch                (git modes only)
  - standards/                  ← core + triggered role files
  - reports/                    ← 00-summary.md, 99-verdict.md stubs
  - temp/                       ← role-plan.md, file-map.md, findings.md

See UPLOAD_ORDER.md for zip command and upload sequence.
```

---

## Output

- `.materials/<timestamp>/review_context.xml` — repomix output
- `.materials/<timestamp>/review_prompt.md` — self-contained, paste as message
- `.materials/<timestamp>/review.patch` (commit mode only)
- `.materials/<timestamp>/standards/` — role standard files
- `.materials/<timestamp>/reports/` — review output files
- `.materials/<timestamp>/temp/` — working files
- `.materials/<timestamp>/UPLOAD_ORDER.md` — zip command + upload sequence
# Pack Materials

Package code context into a structured XML file for Claude.ai review using **repomix**.

## Prerequisites

repomix must be available. Check and install if needed:
```bash
npx repomix --version   # run via npx (no install needed)
# or install globally:
npm install -g repomix
```

## Command

After identifying the changed files and their direct imports (step 3 of prep-review), run:

```bash
repomix \
  --include "<file1>,<file2>,..." \
  --include-diffs \
  --include-logs-count <N> \
  --style xml \
  --compress \
  --ignore "**/*.lock,**/node_modules/**,**/target/**" \
  --output .materials/<timestamp>/review_context.xml
```

### Flag reference

| Flag | Purpose |
|------|---------|
| `--include "src/a.rs,src/b.rs"` | Limit to changed files + their direct imports |
| `--include-diffs` | Embed uncommitted git diffs in the output |
| `--include-logs-count <N>` | Include last N commits (match user's requested scope) |
| `--style xml` | Claude-optimized XML format |
| `--compress` | Tree-sitter compression — extracts signatures, reduces tokens |
| `--ignore` | Exclude noise: lockfiles, build artifacts, vendor dirs |
| `--output` | Write to the timestamped materials directory |

## Scope → include pattern mapping

| User says | `--include` | `--include-logs-count` |
|-----------|-------------|------------------------|
| "review last 3 commits" | files changed in last 3 commits | `3` |
| "review auth changes" | files matching `src/auth/**` | `5` |
| "review this PR" | files in PR diff | number of PR commits |
| "review everything" | omit `--include` (full repo) | `10` |

## Token budget

Target: stay under **100K tokens** for Claude.ai free tier.

1. Run without `--compress` first, check output size
2. If over limit, add `--compress`
3. If still over, narrow `--include` to changed files only (drop imports)
4. If still over, use `--split-output` and upload the first chunk

## Respect ignore rules

Before running repomix, merge project ignore sources into the `--ignore` flag:
- `.archignore` (project-specific exclusions)
- `.repomixignore` (if present)
- Patterns from `review-ignore.md`

Example:
```bash
IGNORE=$(cat .archignore 2>/dev/null | grep -v '^#' | tr '\n' ',' | sed 's/,$//')
repomix --include "..." --ignore "$IGNORE,**/*.lock" --style xml --compress ...
```

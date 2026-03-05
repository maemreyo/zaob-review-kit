# Pack Materials

Package code context into a structured XML file for Claude.ai review using **repomix**.

## Prerequisites

repomix must be available. Check and install if needed:

```bash
npx repomix --version   # run via npx (no install needed)
# or install globally:
npm install -g repomix
```

## Default Command (no compression)

**Do not use `--compress` unless the user explicitly asks for it.**
Compression strips SQL string literals, inline configs, and other content that
reviewers need to verify correctness. Default is always full content:

```bash
repomix \
  --include "<file1>,<file2>,..." \
  --include-diffs \
  --include-logs-count <N> \
  --style xml \
  --ignore "**/*.lock,**/node_modules/**,**/target/**" \
  --output .materials/<timestamp>/review_context.xml
```

### When to use `--compress`

Only add `--compress` when the user explicitly says something like:

- "compress the output"
- "context quá lớn, nén lại đi"
- "use compress mode"
- Token budget is over limit **and** user approves the trade-off

If you add `--compress`, warn the user:

> ⚠️ `--compress` enabled — SQL query bodies and string literals will be stripped.
> Reviewers cannot verify WHERE clauses or security-critical logic from this output.
> Consider uploading `review.patch` alongside as a fallback.

### Flag reference

| Flag                            | Purpose                                                                          |
| ------------------------------- | -------------------------------------------------------------------------------- |
| `--include "src/a.rs,src/b.rs"` | Limit to changed files + their direct imports + caller files                     |
| `--include-diffs`               | Embed uncommitted git diffs in the output                                        |
| `--include-logs-count <N>`      | Include last N commits (match user's requested scope)                            |
| `--style xml`                   | Claude-optimized XML format                                                      |
| `--compress`                    | Tree-sitter compression — **off by default**, use only when explicitly requested |
| `--ignore`                      | Exclude noise: lockfiles, build artifacts, vendor dirs                           |
| `--output`                      | Write to the timestamped materials directory                                     |

## File Selection: Two Algorithms

The `--include` glob flag works for known filenames. For dynamic scopes — commits
or topic-based searches — use `--stdin` to pipe a file list from shell tools.
`--stdin` reads file paths one per line; paths specified this way are added to
the include patterns, with normal ignore rules still applying.

### Algorithm A — X latest commits

```bash
# Get the changed file list from git, pipe into repomix
git diff HEAD~<N>..HEAD --name-only | repomix --stdin \
  --include-diffs \
  --include-logs-count <N> \
  --include-full-directory-structure \
  --style xml \
  --output .materials/$TS/review_context.xml
```

Add caller/composition files manually if needed (see `prep-review.md` Step 2).

### Algorithm B — Content related to Y (feature / phase / module)

Use **ripgrep** (`rg`) to find all files that mention the topic, then pipe
into repomix. `rg` respects `.gitignore` and is far faster than `grep -r`.

```bash
# Find files mentioning a topic, pipe to repomix
rg -l "phase.0\|Phase 0" | repomix --stdin \
  --include-full-directory-structure \
  --style xml \
  --output .materials/$TS/review_context.xml

# Scope to specific directories or file types
rg -l "auth\|jwt" src/ --type rust | repomix --stdin --style xml ...

# Multiple keywords (OR)
rg -l "student|enrollment" migrations/ eduos-core/ | repomix --stdin ...
```

Install ripgrep: `brew install ripgrep` (macOS) / `apt install ripgrep` (Linux).
Ripgrep defaults to recursive search, respects `.gitignore`, and skips binary
files automatically.

### Scope → command mapping

| User says                              | Algorithm | Key command                                                                                                 |
| -------------------------------------- | --------- | ----------------------------------------------------------------------------------------------------------- |
| "review last N commits"                | A         | `git diff HEAD~N..HEAD --name-only \| repomix --stdin`                                                      |
| "review this PR / branch"              | A         | `git diff main...HEAD --name-only \| repomix --stdin`                                                       |
| "pack everything related to phase 0"   | B         | `rg -l "phase.0" \| repomix --stdin`                                                                        |
| "pack the auth changes"                | B         | `rg -l "auth\|jwt\|login" src/ \| repomix --stdin`                                                          |
| "pack files changed by Jane this week" | A         | `git log --since="1 week ago" --author="Jane" --name-only --pretty=format:'' \| sort -u \| repomix --stdin` |

## Documentation File Inclusion

Include project documentation in review context so reviewers can verify code
changes align with documented architecture, API contracts, and design decisions.

### Documentation patterns to include

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

### Packing strategy

Documentation files are packed alongside source code in `review_context.xml`.
The Software Architect review should reference these when evaluating
architectural consistency. If documentation contradicts code changes, the
reviewer should flag the discrepancy.

**Important:** `.archignore` and `.repomixignore` patterns still apply.
Binary images (_.png, _.svg) should be included selectively — they may be
large and consume significant token budget. Use `--token-count-tree` to
preview their impact before packing.

### Example: Include documentation with source files

```bash
# Combine source files and documentation
(git diff HEAD~3..HEAD --name-only; \
 find . -name "README.md" -o -name "ARCHITECTURE.md"; \
 find docs/ -name "*.mmd" -o -name "*.drawio" 2>/dev/null) | \
repomix --stdin \
  --include-diffs \
  --include-logs-count 3 \
  --style xml \
  --output .materials/$TS/review_context.xml
```

## Token budget

Target: stay under **100K tokens** for Claude.ai free tier.

Use repomix's built-in token visualizer **before** packing to see how large
your file selection will be:

```bash
# Preview token distribution without writing the output file
git diff HEAD~<N>..HEAD --name-only | repomix --stdin --token-count-tree

# Filter to only show files with 1000+ tokens (spot heavy hitters)
rg -l "phase 0" | repomix --stdin --token-count-tree 1000
```

If over budget, narrow in this order:

1. Drop test files from the list (`grep -v test | grep -v spec`)
2. Drop generated / migration files if they're not under review
3. Ask the user before adding `--compress` — explain the trade-off (see below)
4. Use `--split-output 400kb` and upload the first part

## Security-sensitive content

When the diff contains SQL queries, auth middleware, or any logic where
reviewers must verify WHERE clauses, JOIN conditions, or filter bindings —
**never compress**, even if the file is large.

For large SQL-heavy files that push the token budget over limit, pack them
as a separate uncompressed file and tell the reviewer:

```bash
# Main context (can compress non-security files if needed)
repomix \
  --include "<non-sql-files>" \
  --style xml \
  --output .materials/$TS/review_context.xml

# Security context — always uncompressed
repomix \
  --include "<sql-and-auth-files>" \
  --style xml \
  --output .materials/$TS/security_context.xml
  # No --compress here, ever
```

Tell the reviewer in `review_prompt.md`:

> `security_context.xml` contains uncompressed SQL and auth files.
> Upload it alongside `review_context.xml`.

## Respect ignore rules

Before running repomix, merge project ignore sources into the `--ignore` flag:

- `.archignore` (project-specific exclusions)
- `.repomixignore` (if present)
- Patterns from `review-ignore.md`

Example:

```bash
IGNORE=$(cat .archignore 2>/dev/null | grep -v '^#' | tr '\n' ',' | sed 's/,$//')
repomix --include "..." --ignore "$IGNORE,**/*.lock" --style xml --output ...
```

# .archignore

Project-specific file exclusion rules for review context packing.

---

## What it is

`.archignore` works like `.gitignore` but specifically for `zrk`'s review context packing step. Files matching these patterns are excluded when building `review_context.xml`.

It lives at your project root and is installed by `zrk install-all`.

---

## Syntax

One glob pattern per line. Same syntax as `.gitignore`:

```
node_modules/
*.lock
dist/
.env
```

Lines starting with `#` are comments.

---

## Default content

zrk installs a default `.archignore` covering the most common cases:

```
# Dependencies
node_modules/
vendor/
target/
.venv/

# Build output
dist/
build/
out/
*.min.js
*.min.css
*.map

# Generated
*.lock
*.generated.*
__generated__/

# Binary / media
*.png *.jpg *.jpeg *.gif *.svg
*.ico *.woff *.woff2 *.ttf *.eot
*.pdf

# IDE / OS
.idea/
.vscode/
*.swp
.DS_Store

# Environment
.env
.env.*
```

---

## Customizing for your stack

**Go project:**

```
vendor/
*.pb.go          # generated protobuf
```

**Python project:**

```
__pycache__/
*.pyc
*.egg-info/
.venv/
htmlcov/
```

**Monorepo — exclude other apps:**

```
packages/mobile/
packages/admin/
```

**Large data files:**

```
data/raw/
fixtures/*.json
```

---

## Relationship to review-ignore.md

`review-ignore.md` is a _global_ file (installed once, applies to all projects) defining baseline ignore patterns. `.archignore` is _project-specific_ and adds to those defaults.

Both are applied when packing review materials. You don't need to duplicate the defaults in `.archignore`.

---

## Files referenced but not included

Files matching ignore patterns are still _referenced_ in the review if they appear in import statements of changed files. They are mentioned in `review_context.xml` but their full content is not included. This keeps context meaningful without bloating the package.

---

## What zrk update does

`zrk update` does **not** overwrite `.archignore` — you may have customized it. Only `zrk install-all` on a fresh project creates it.

---

## Next

- [review-ignore.md workflow doc](../workflow/review-ignore.md)
- [File layout reference](file-layout.md)

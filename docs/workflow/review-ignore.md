# review-ignore.md

Default file exclusion patterns for review context packing.

---

## Default ignore patterns

These patterns are always excluded when packing `review_context.xml`:

**Dependencies:**

```
node_modules/  vendor/  target/  .venv/
```

**Build output:**

```
dist/  build/  out/  *.min.js  *.min.css  *.map
```

**Generated files:**

```
*.lock  *.generated.*  __generated__/
```

**Binary and media:**

```
*.png  *.jpg  *.jpeg  *.gif  *.svg
*.ico  *.woff  *.woff2  *.ttf  *.eot  *.pdf
```

**IDE and OS:**

```
.idea/  .vscode/  *.swp  .DS_Store
```

**Environment:**

```
.env  .env.*
```

---

## Project-specific patterns

Add project-specific patterns to `.archignore` at the project root. These are applied on top of the defaults above.

See [.archignore reference](../reference/archignore.md) for examples by tech stack.

---

## Files referenced but not included

An ignored file that appears in an import statement of a changed file will still be _mentioned_ in `review_context.xml` (as `path` only, not full content). Claude sees the dependency exists but doesn't get the full content of the ignored file.

This prevents context bloat while keeping the dependency graph visible.

---

## Customizing

Edit `review-ignore.md` in your agent's steering directory to change the default patterns:

```bash
# Kiro example
nano ~/.kiro/steering/review-ignore.md
```

Note: `zrk update` will overwrite this. If you want persistent customization, use `.archignore` instead.

---

## Next

- [.archignore reference](../reference/archignore.md)
- [prep-review.md — how ignores are applied](prep-review.md)

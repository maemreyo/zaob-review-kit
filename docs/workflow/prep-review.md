# prep-review.md

The main workflow file. Your agent reads this when you say anything that matches a review request.

---

## Trigger sentences

```
tạo materials để review 3 commit gần nhất
tạo materials để review PR #42
pack phần authentication để review
tạo materials của toàn bộ docs về architect
create review materials for the last 3 commits
pack the auth changes for review
prepare review for everything changed this week
```

The agent interprets natural language — you don't need exact phrasing.

---

## What happens step by step

1. **Identify scope**
   Parse the request to determine what to review:
   - Last N commits: `git log --oneline -N`
   - Specific commits: by hash or range
   - Named feature: files matching the description
   - Directory: all files under a path

2. **Generate diff**
   `git diff <range>` — clean diff output, not truncated.

3. **Resolve context**
   Find files that import or are imported by changed files. This is the key step that makes the review useful — Claude sees not just what changed, but what it depends on.

4. **Apply ignore rules**
   Filter out files matching patterns from `review-ignore.md` (node_modules, lock files, binaries, etc.).

5. **Read `.archignore`**
   Apply project-specific exclusions on top of the defaults.

6. **Build `review_context.xml`**
   Package diff + context files into structured XML (see [pack-materials.md](pack-materials.md)).

7. **Generate `review_prompt.md`**
   Build the structured review prompt using roles from `review-roles.md` and formatting rules from `review-prompting.md`.

8. **Save to `.materials/`**
   Write output files with a timestamp prefix: `.materials/YYYYMMDD_<hash>/`.

---

## Output

```
.materials/20250304_a1b2c3/
  review_context.xml   ← upload this
  review_prompt.md     ← upload this
  review.patch         ← reference copy of the raw patch
```

---

## Context budget

The agent caps total context at ~100K tokens, prioritizing:

1. Changed files (full content)
2. Direct imports/exports of changed files
3. Indirect dependencies
4. Project context (`project-context.md` content)

Files are dropped from the bottom of this priority list when the budget is exceeded.

---

## Next

- [How review_context.xml is built](pack-materials.md)
- [The five reviewer roles](review-roles.md)
- [Customizing what gets ignored](../reference/archignore.md)

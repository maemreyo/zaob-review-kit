# Review Checklist

Before uploading materials to Claude.ai, verify:

## Materials

- [ ] `zrk prep <scope>` was run — `.materials/<timestamp>/` exists
- [ ] `review_context.xml` exists and is < 400KB (~100K tokens)
- [ ] `standards/` contains core files + triggered role standards
- [ ] `review_prompt.md` — specific questions filled in (or confirmed blank)
- [ ] `review_prompt.md` — additional/skip roles updated if needed
- [ ] `review.patch` generated (git modes only)

## Context Quality

- [ ] `project-context.md` has real content (not just empty scaffold)
- [ ] If `review_memory.md` exists — was it read before this review?
- [ ] Entry-point / composition file included if prompt asks about
      middleware ordering, routing, or module wiring
- [ ] If diff touches queries or auth logic: verify query/logic bodies are
      readable in the context — not stripped by compression
- [ ] `--compress` was **not** added unless user explicitly requested it

## Scope Accuracy

- [ ] Only changed files + direct imports included (not whole repo)
- [ ] Lockfiles, build artifacts excluded
- [ ] `.archignore` patterns applied

## Upload

Follow the zip command and order in `UPLOAD_ORDER.md`:

```bash
cd .materials && zip -r ../review-<TS>.zip <TS>/
```

Then upload the zip to Claude.ai and paste `review_prompt.md` as your message.

If `review_memory.md` exists: mention "See also review memory for recurring patterns"

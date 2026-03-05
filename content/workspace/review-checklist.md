# Review Checklist

Before uploading materials to Claude.ai, verify:

## Materials

- [ ] `review_context.xml` exists in `.materials/<timestamp>/`
- [ ] `role_standards_bundle.md` exists — contains triggered roles + 3 core roles
- [ ] Token estimate < 100K (file size < ~400KB is generally safe)
- [ ] `review_prompt.md` has scope and description filled in
- [ ] `review_prompt.md` has `## Plan Before Executing` section (roles + execution order)
- [ ] Change type checkboxes / additional roles section reviewed
- [ ] `review.patch` generated

## Context Quality

- [ ] `project-context.md` has real content (not just empty scaffold)
- [ ] If `review_memory.md` exists — was it read before this review?
- [ ] Specific questions added to prompt (not just generic focus areas)
- [ ] Entry-point / composition file included if prompt asks about
      middleware ordering, routing, or module wiring
- [ ] If diff touches queries or auth logic: verify query/logic bodies are
      readable in the context — not stripped by compression
- [ ] `--compress` was **not** added unless user explicitly requested it

## Scope Accuracy

- [ ] Only changed files + direct imports included (not whole repo)
- [ ] Lockfiles, build artifacts excluded
- [ ] `.archignore` patterns applied

## Upload Order to Claude.ai

1. Upload `review_context.xml` first
2. Upload `role_standards_bundle.md` second
3. Paste content of `review_prompt.md` as your message
4. If `review_memory.md` exists: mention "See also review memory for recurring patterns"

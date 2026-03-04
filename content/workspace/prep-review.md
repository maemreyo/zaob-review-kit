# Prep Review

Prepare review materials for uploading to Claude.ai.

## Trigger
User says something like:
- "tạo materials để review 3 commit gần nhất"
- "prep review for the last commit"
- "pack the auth changes for review"

## Steps

1. **Identify scope**: Determine which commits or files to review
2. **Generate diff**: Create a clean diff of the changes
3. **Resolve context**: Find files imported by or importing changed files
4. **Apply ignore rules**: Filter out files matching review-ignore patterns
5. **Read .archignore**: Apply project-specific exclusions
6. **Build review_context.xml**: Package diff + context files
7. **Generate review_prompt.md**: Create the structured review prompt
8. **Save to .materials/**: Write output files with timestamp prefix

## Output
- `.materials/<timestamp>/review_context.xml`
- `.materials/<timestamp>/review_prompt.md`
- `.materials/<timestamp>/review.patch`

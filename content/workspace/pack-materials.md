# Pack Materials

Package code context into a structured XML format for Claude.ai review.

## Context XML Structure

```xml
<review_context>
  <metadata>
    <project>project-name</project>
    <date>2025-03-04</date>
    <scope>last 3 commits</scope>
  </metadata>
  <diff>
    <!-- clean diff output -->
  </diff>
  <context_files>
    <file path="src/auth/login.rs">
      <!-- full file content for referenced files -->
    </file>
  </context_files>
  <project_context>
    <!-- from project-context.md if it exists -->
  </project_context>
</review_context>
```

## Packing Rules
- Include the full diff (not truncated)
- Include referenced files (imports/exports of changed files)
- Respect .archignore and review-ignore patterns
- Cap total context at ~100K tokens — prioritize changed files, then direct imports
- Include project-context.md content if available

# Review Ignore Patterns

When packing review materials, exclude these files and directories:

## Always Ignore
- `node_modules/`, `vendor/`, `target/`, `dist/`, `build/`
- `.git/`, `.svn/`, `.hg/`
- `*.lock` (package lock files)
- `.env`, `.env.*` (environment files)
- Binary files: images, fonts, compiled assets
- Generated files: `*.min.js`, `*.min.css`, source maps

## Project-Specific
Check `.archignore` in the project root for additional patterns.
The agent should read `.archignore` before packing materials.

## Context for Review
These files should still be _referenced_ if they appear in import statements
of changed files, but their full content should not be included in the
review materials unless specifically requested.

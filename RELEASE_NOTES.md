# Release v0.4.0 - Multi-File Review Output

## Summary

This release adds comprehensive documentation for multi-file review output, enabling AI review agents to generate structured, navigable code reviews organized by role perspective.

## What's New

### Multi-File Review Output Documentation

- **Structured output format**: Reviews are now split into separate Markdown files (00-summary.md, 01-swe-review.md, etc.) instead of a single monolithic document
- **Role-based organization**: Each role perspective (SWE, SA, QA, PE, SE, OE, CEO, Devil's Advocate) gets its own file
- **Navigation system**: Table of contents in summary file, back-links in verdict, cross-references between roles
- **File naming convention**: `<NN>-<slug>.md` pattern ensures correct alphabetical sorting

### Documentation Collection

- **Automatic documentation inclusion**: prep-review now collects README files, docs/ directories, architecture diagrams, API specs, and design documents
- **Better architectural review**: Software Architect reviews can now reference project documentation to verify code changes align with documented architecture
- **Comprehensive patterns**: Supports README.md, DESIGN.md, ARCHITECTURE.md, ADR-*.md, openapi.yaml, *.drawio, *.mmd, and more

### Temp Directory for Agent Working Files

- **Organized workspace**: Agents now use `.materials/<timestamp>/temp/` for intermediate files instead of system temp locations
- **Better debugging**: Working files (file lists, drafts, notes) are preserved in the materials directory

### Example Documentation

- **Complete examples**: New docs/reviews/multi-file-example.md shows both minimal and full review structures
- **Sample content**: Demonstrates proper severity labels, file references, suggested tests format, and navigation
- **Clear guidelines**: When to use minimal vs full reviews, file naming patterns, cross-reference format

## Files Changed

- `Cargo.toml`: Version bumped to 0.4.0
- `CHANGELOG.md`: Created with release notes
- `content/global/review-prompting.md`: Added "Multi-File Output Structure" section
- `content/workspace/prep-review.md`: Added temp directory creation and documentation collection steps
- `content/workspace/pack-materials.md`: Added documentation inclusion strategy
- `docs/reviews/multi-file-example.md`: Created with comprehensive examples

## Installation

```bash
cargo install zrk
```

Or update existing installation:

```bash
cargo install zrk --force
```

## Publishing to crates.io

To publish this release to crates.io:

```bash
# 1. Push the commit and tag to GitHub
git push origin main
git push origin v0.4.0

# 2. Publish to crates.io (requires cargo and crates.io authentication)
cargo publish

# 3. Create GitHub release (optional, via GitHub web UI)
# - Go to https://github.com/maemreyo/zaob-review-kit/releases/new
# - Select tag: v0.4.0
# - Title: Release v0.4.0: Multi-file review output documentation
# - Copy content from CHANGELOG.md for description
```

## Breaking Changes

None. This release is fully backward compatible - all changes are documentation-only.

## Migration Guide

No migration needed. Users can update to 0.4.0 and run:

```bash
zrk update --target <your-agent>
```

This will install the updated documentation files with multi-file review instructions.

## Next Steps

After publishing:

1. Update any external documentation or blog posts
2. Announce the release on relevant channels
3. Monitor for user feedback on the new multi-file review format
4. Consider adding automated tests for the documentation structure in future releases

# Implementation Plan: Multi-File Review Output

## Overview

This implementation updates the zaob-review-kit (zrk) documentation to support multi-file review output. The changes are entirely documentation-based—no code modifications are required. The work focuses on updating three key documentation files (review-prompting.md, prep-review.md, pack-materials.md) to instruct AI review agents how to structure their output as separate Markdown files organized by role perspective.

## Tasks

- [x] 1. Update review-prompting.md with multi-file output instructions
  - Add new "Multi-File Output Structure" section after "Output Structure" section
  - Document the file naming convention: `<NN>-<slug>.md` with specific filenames for each role
  - Specify content distribution: what goes in summary file (00-summary.md), role files (01-08), and verdict file (99-verdict.md)
  - Include examples showing minimal review (SWE/SA/QA only) and full review (all roles triggered)
  - Add navigation requirements: TOC in summary, back-links in verdict, cross-references between roles
  - Specify that Markdown is the default format unless user explicitly requests another format
  - Document the temp directory usage: `.materials/<timestamp>/temp/` for agent working files
  - _Requirements: 7.1, 7.2, 7.3, 7.4, 7.5, 13.6_

- [x] 2. Update prep-review.md to create temp directory and collect documentation
  - [x] 2.1 Add temp directory creation step
    - Document that prep-review should create `.materials/<timestamp>/temp/` directory
    - Specify that this directory is for agent working files (file lists, drafts, notes)
    - Update review_prompt.md generation to inform agent of temp directory location
    - _Requirements: 13.1, 13.2, 13.6_
  
  - [x] 2.2 Add documentation collection step
    - Document the documentation file patterns to search for:
      - README files: `README.md`, `README.txt`, `README` in all directories
      - Documentation directories: `docs/`, `documentation/`, `.github/`
      - Architecture diagrams: `*.drawio`, `*.mmd`, `*.puml`, `*.svg`, `*.png` in docs/
      - API specifications: `openapi.yaml`, `openapi.json`, `swagger.yaml`, `api-spec.md`
      - Design documents: `DESIGN.md`, `ARCHITECTURE.md`, `ADR-*.md`
    - Specify that documentation files should be included in review_context.xml
    - Note that .archignore and .repomixignore patterns should be respected
    - _Requirements: 14.1, 14.2, 14.6_

- [x] 3. Update pack-materials.md to include documentation files
  - Document the documentation inclusion strategy in the file packing process
  - Specify which documentation patterns to include (same as prep-review.md)
  - Clarify that documentation files are packed alongside source code in review_context.xml
  - Note that review-ignore.md patterns still apply (don't pack binary images unless needed)
  - _Requirements: 14.6_

- [x] 4. Create example output structure documentation
  - Create a new example file showing the complete multi-file review structure
  - Include sample content for each file type (summary, role files, verdict)
  - Show the table of contents format with links
  - Demonstrate severity labels and file references in role files
  - Show suggested tests format in verdict file
  - Include examples of both minimal and full reviews
  - _Requirements: 7.4, 12.1, 12.3_

- [x] 5. Checkpoint - Review documentation changes
  - Ensure all documentation updates are consistent with each other
  - Verify that the multi-file output instructions are clear and actionable
  - Verify that temp directory and documentation collection steps are well-documented
  - Ask the user if questions arise or if any clarifications are needed

## Notes

- This feature requires NO code changes—only documentation updates
- All changes are to markdown files in the content/ directory
- The implementation focuses on instructing AI agents how to structure their output
- Existing calibration rules, severity labels, and review quality standards remain unchanged
- The multi-file structure integrates with existing prep-review workflow without breaking changes

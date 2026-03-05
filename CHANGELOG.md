# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.5.1] - 2025-03-05

### Changed

- Refined prep-review.md workflow documentation
  - Restructured markdown formatting for consistency
  - Clarified file-map.md population strategy (fill row-by-row during prep, not batched at end)
  - Added explicit guidance on when and how to populate temp/file-map.md during file reading
  - Emphasized that File Walkthrough table should be sourced from temp/file-map.md
  - Updated risk and effort field formatting with angle brackets for clarity

### Added

- Implemented KiroInclusion enum for steering file loading modes
  - Categorizes files as Always, Auto, or Manual inclusion
  - Comprehensive documentation for Kiro inclusion strategy with design rationale
  - Improved agent context management by distinguishing between loading modes

## [0.5.0] - 2025-03-05

### Added

- Comprehensive role-based review standards
  - Added 15 new role standard documents with detailed review criteria
  - Roles: SWE, SA, QA, PE, SE, OE, DE (Data Engineer), UX, CL (Cloud), CEO, DA (Devil's Advocate), MLE (ML Engineer), API, FinOps, DX (Developer Experience)
  - Added loading guide (00-loading-guide.md) for role standards
  - Integrated role standards into agent modules (Kiro, TRAE)

- Restructured review workflow with temp file protocol
  - Introduced reports/ subdirectory for role review outputs (better organization)
  - Added temp/ directory protocol with three coordinating files:
    - role-plan.md: tracks which roles to execute
    - file-map.md: maps role outputs to filenames
    - findings.md: compressed log of blockers/majors for verdict synthesis
  - Updated .gitignore to exclude temp/ while keeping reports/ committed

### Changed

- Enhanced review-prompting.md with improved content and temp file protocol
- Updated review-roles.md with better role guidance
- Restructured prep-review.md with detailed steps for directory initialization
- Improved review execution flow to append findings after each role
- Verdict synthesis now reads only findings.md instead of individual role files (efficiency improvement)

### Fixed

- Cleaned up unused transitive dependencies in Cargo.lock
- Downgraded rustix and tempfile to compatible versions for stability

## [0.4.0] - 2025-03-05

### Added

- Multi-file review output documentation
  - Added comprehensive "Multi-File Output Structure" section to review-prompting.md
  - Documented file naming convention: `<NN>-<slug>.md` for organized review files
  - Added navigation requirements: TOC in summary, back-links in verdict, cross-references
  - Documented temp directory usage: `.materials/<timestamp>/temp/` for agent working files
  
- Documentation collection in review workflow
  - Added documentation file patterns to prep-review.md (README, docs/, architecture diagrams, API specs, design documents)
  - Updated pack-materials.md with documentation inclusion strategy
  - Documentation files now included in review_context.xml alongside source code
  
- Example documentation
  - Created docs/reviews/multi-file-example.md with complete examples
  - Included both minimal review (core roles only) and full review (all roles triggered) examples
  - Demonstrated proper severity labels, file references, and suggested tests format

### Changed

- Updated prep-review.md to create temp directory for agent working files
- Enhanced pack-materials.md with documentation file inclusion patterns
- Improved review-prompting.md with detailed multi-file output instructions

## [0.3.1] - Previous Release

(Previous changelog entries would go here)

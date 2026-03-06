# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.6.2] - 2025-03-06

### Changed

- Version bump for release

## [0.6.1] - 2025-03-06

### Changed

- Streamlined prep workflow documentation with zrk prep command integration
  - Replaced manual multi-step prep process with single `zrk prep` command
  - Consolidated scope identification into three modes (git range, non-contiguous commits, topic search)
  - Moved token budget checking before prep execution with repomix token counting
  - Simplified context resolution with auto-detection of triggered roles and file inclusion
  - Removed intermediate file creation steps (role-plan.md, file-map.md pre-generation)
  - Updated anti-patterns section to reflect new automated workflow
  - Reduced prep documentation from 135 to 99 lines while maintaining completeness
  - Aligned prep-review.md with automated tooling capabilities for faster review preparation

- Updated review-checklist.md to reflect streamlined workflow

## [0.6.0] - 2025-03-06

### Added

- New `prep` command for comprehensive review preparation
  - Three input modes: git ranges, commit hashes, and topic search
  - Role trigger detection system with 12 specialized review roles
  - Automatic role activation based on changed file patterns
  - Manual role selection support
  - Structured review preparation output with role-specific standards
  - File annotations showing which roles are triggered by each file
  - Git diff parsing for commit-based reviews
  - Ripgrep content search for topic-based reviews
  - Comprehensive error handling for invalid git ranges and missing files

- Role-based review scoping
  - Performance Engineer (PE) - DB queries, async code, nested loops
  - Security Engineer (SE) - Auth code, new dependencies
  - Operations Engineer (OE) - New endpoints, config changes
  - Data Engineer (DE) - Data pipelines, ETL, migrations
  - UX Designer (UX) - UI components, user flows
  - Cloud Architect (CL) - Infrastructure, cloud resources
  - CEO - Breaking changes, public API changes
  - Devil's Advocate (DA) - Challenge assumptions
  - ML Engineer (MLE) - ML models, training pipelines
  - API Designer (API) - API endpoints, contracts
  - FinOps (FinOps) - Cost optimization, resource usage
  - Developer Experience (DX) - Developer tools, workflows

### Changed

- Updated error types to support prep-specific error variants
- Enhanced main.rs with prep command integration

## [0.5.3] - 2025-03-05

### Changed

- Standardized markdown formatting across documentation
  - Normalized table formatting with consistent spacing and alignment
  - Added blank lines between sections for improved readability
  - Reformatted code blocks and nested lists with proper indentation
  - Updated emphasis styling from bold to italic for agent notes
  - Improved visual hierarchy in anti-patterns section
  - Consistent bullet formatting in documentation inclusion rules

### Fixed

- Added dead_code suppressions for dynamic trait dispatch
  - Suppressed warnings for filter_agent_sections, parse_agent_start, parse_agent_end functions
  - Suppressed warning for filter_content trait method
  - Added documentation explaining Rust cannot trace calls through dynamic trait dispatch at compile time

## [0.5.2] - 2025-03-05

### Added

- Anti-patterns guide in pack-materials.md
  - "Never write an intermediate file list" section with examples of heredoc and temp-file anti-patterns
  - Warnings about patterns that hang terminal agents
  - Documentation of direct piping approach as correct pattern

- Enhanced review_prompt.md template
  - Full Prompt Anatomy structure with context files, scope details, and reference sections
  - Clearer guidance for review preparation

### Changed

- Refined material packing workflow in prep-review.md
  - Restructured step 6 to emphasize piping directly into repomix without intermediate files
  - Added explicit anti-patterns section with warnings
  - Consolidated Mode A and Mode B examples with clearer command structure
  - Renumbered workflow steps for better flow

- Updated documentation inclusion rules
  - Include architecture/tdd/prompts docs
  - Exclude docs/reviews/ directory to prevent stale analysis from inflating token budget
  - Exclude binary files by default

### Fixed

- Binary image syntax updated from underscore to backtick notation (_.png → *.png)

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

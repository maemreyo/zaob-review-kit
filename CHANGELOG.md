# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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

# Requirements Document

## Introduction

This feature enhances the zaob-review-kit (zrk) code review workflow to produce structured, multi-file Markdown output instead of a single monolithic document. The current behavior on Claude.ai web defaults to creating `.docx` files with all role perspectives mixed together, making it difficult to navigate and review findings by role. This feature will split the review into separate Markdown files, one per role perspective, with a summary file and a verdict file.

## Glossary

- **Review_Agent**: The AI agent (Claude.ai web or other) that performs code review using zrk prompts
- **Role_Perspective**: A specific review lens (SWE, SA, QA, PE, SE, OE, CEO, Devil's Advocate) defined in review-roles.md
- **Review_Output_Directory**: The timestamped directory under `.materials/<timestamp>/` where review files are written
- **Temp_Directory**: The `.materials/<timestamp>/temp/` subdirectory for agent working files and intermediate outputs
- **Summary_File**: The `00-summary.md` file containing overall summary, file walkthrough, and risk assessment
- **Role_File**: A numbered Markdown file (e.g., `01-swe-review.md`) containing findings from one role perspective
- **Verdict_File**: The `99-verdict.md` file containing final recommendations and ship decision
- **Triggered_Role**: A role perspective that is activated based on calibration rules (e.g., Security Engineer for auth changes)
- **Documentation_Files**: Project documentation including README files, docs/ directory, architecture diagrams, API specs, and design documents

## Requirements

### Requirement 1: Default to Markdown Output Format

**User Story:** As a code reviewer, I want review reports generated as Markdown files, so that I can read them in any text editor and version control them easily.

#### Acceptance Criteria

1. THE Review_Agent SHALL generate all review output files with `.md` extension
2. THE Review_Agent SHALL use Markdown formatting for all content (headers, lists, code blocks, tables)
3. THE Review_Agent SHALL NOT create `.docx` files unless explicitly requested by the user

### Requirement 2: Create Summary File

**User Story:** As a code reviewer, I want a single summary file with overview information, so that I can quickly understand the scope and risk level before diving into detailed findings.

#### Acceptance Criteria

1. THE Review_Agent SHALL create a file named `00-summary.md` in the Review_Output_Directory
2. THE Summary_File SHALL contain the overall summary paragraph describing what changed and why
3. THE Summary_File SHALL contain the file walkthrough table as specified in review-prompting.md
4. THE Summary_File SHALL contain the risk assessment (Low/Medium/High) with justification
5. THE Summary_File SHALL contain the review effort score [1-5] with explanation
6. THE Summary_File SHALL include a table of contents linking to all generated Role_Files

### Requirement 3: Create Separate Role Perspective Files

**User Story:** As a code reviewer, I want findings organized into separate files by role perspective, so that I can focus on specific concerns (security, performance, architecture) without distraction.

#### Acceptance Criteria

1. WHEN a Role_Perspective is active, THE Review_Agent SHALL create a separate Role_File for that perspective
2. THE Review_Agent SHALL name Role_Files using the pattern `<NN>-<role-slug>-review.md` where NN is a two-digit number
3. THE Review_Agent SHALL use these specific filenames for core roles:
   - `01-swe-review.md` for Senior Software Engineer
   - `02-sa-review.md` for Software Architect
   - `03-qa-review.md` for Quality Assurance
4. WHEN a Triggered_Role is active, THE Review_Agent SHALL create additional Role_Files:
   - `04-pe-review.md` for Performance Engineer
   - `05-se-review.md` for Security Engineer
   - `06-oe-review.md` for Operations Engineer
   - `07-ceo-review.md` for CEO perspective
   - `08-devil-advocate.md` for Devil's Advocate
5. THE Review_Agent SHALL NOT create Role_Files for roles that are not triggered by calibration rules

### Requirement 4: Structure Role File Content

**User Story:** As a code reviewer, I want each role file to contain only findings relevant to that perspective, so that I can efficiently review domain-specific concerns.

#### Acceptance Criteria

1. THE Role_File SHALL begin with a level-1 header naming the role perspective
2. THE Role_File SHALL contain only findings from that specific Role_Perspective
3. THE Role_File SHALL use comment severity labels ([BLOCKER], [MAJOR], [SUGGESTION], [NIT], [QUESTION]) as specified in review-prompting.md
4. THE Role_File SHALL cite specific file paths and line numbers when referencing code
5. WHEN a Role_Perspective has no findings, THE Role_File SHALL contain a brief statement indicating no issues were found

### Requirement 5: Create Verdict File

**User Story:** As a code reviewer, I want a final verdict file with consolidated recommendations, so that I can quickly see the overall decision and action items.

#### Acceptance Criteria

1. THE Review_Agent SHALL create a file named `99-verdict.md` in the Review_Output_Directory
2. THE Verdict_File SHALL contain the suggested tests section as specified in review-prompting.md
3. THE Verdict_File SHALL contain prioritized recommendations organized by severity
4. THE Verdict_File SHALL contain the final verdict (Ship / Ship with changes / Needs rework / Needs discussion)
5. THE Verdict_File SHALL include the AI review caveat as specified in review-prompting.md

### Requirement 6: Maintain File Ordering

**User Story:** As a code reviewer, I want review files numbered in a logical order, so that I can read them sequentially when needed.

#### Acceptance Criteria

1. THE Review_Agent SHALL number files so that alphabetical sorting produces the correct reading order
2. THE Summary_File SHALL be numbered `00-` to appear first
3. THE Verdict_File SHALL be numbered `99-` to appear last
4. WHEN multiple Triggered_Roles are active, THE Review_Agent SHALL assign numbers 04-08 in the order: PE, SE, OE, CEO, Devil's Advocate

### Requirement 7: Update Review Prompting Guidelines

**User Story:** As a prompt maintainer, I want the review-prompting.md file updated with multi-file output instructions, so that Review_Agents know how to structure their output.

#### Acceptance Criteria

1. THE review-prompting.md file SHALL include a new section titled "Multi-File Output Structure"
2. THE new section SHALL specify the exact filenames and numbering scheme for all review files
3. THE new section SHALL explain which content belongs in each file type (Summary_File, Role_File, Verdict_File)
4. THE new section SHALL include examples of file naming for both minimal reviews (SWE/SA/QA only) and full reviews (all roles triggered)
5. THE new section SHALL specify that Markdown format is required unless the user explicitly requests another format

### Requirement 8: Preserve Existing Review Quality Standards

**User Story:** As a code reviewer, I want all existing review quality standards maintained, so that the multi-file format doesn't reduce review thoroughness.

#### Acceptance Criteria

1. THE Review_Agent SHALL continue to apply all calibration rules from review-prompting.md
2. THE Review_Agent SHALL continue to use all comment severity labels correctly
3. THE Review_Agent SHALL continue to include the suggested tests section with the specified format
4. THE Review_Agent SHALL continue to apply role-specific criteria from review-roles.md, review-security.md, and review-performance.md
5. THE Review_Agent SHALL continue to check for team best practices in review-best-practices.md if it exists

### Requirement 9: Integrate with Existing Workflow

**User Story:** As a developer, I want the multi-file output to work seamlessly with prep-review and pack-materials workflows, so that I don't need to change my review preparation process.

#### Acceptance Criteria

1. THE Review_Agent SHALL write all review files to the same Review_Output_Directory created by prep-review
2. THE Review_Agent SHALL use the same timestamp format for directory naming as prep-review
3. WHEN review_context.xml and review_prompt.md exist in the Review_Output_Directory, THE Review_Agent SHALL add review output files to the same directory
4. THE Review_Agent SHALL NOT modify or delete existing files (review_context.xml, review_prompt.md, review.patch) in the Review_Output_Directory

### Requirement 10: Support Review Memory Integration

**User Story:** As a code reviewer, I want review memory entries to reference specific role files, so that I can track which perspectives found recurring issues.

#### Acceptance Criteria

1. WHEN appending to review_memory.md, THE Review_Agent SHALL list the generated Role_Files in the "Files reviewed" field
2. WHEN a finding is recurring, THE Review_Agent SHALL reference the specific Role_File where it was previously found
3. THE Review_Agent SHALL include the Summary_File and Verdict_File in the review memory entry

### Requirement 11: Handle Edge Cases

**User Story:** As a code reviewer, I want the multi-file output to handle edge cases gracefully, so that reviews don't fail due to unusual scenarios.

#### Acceptance Criteria

1. WHEN a review is trivial (review effort 1/5), THE Review_Agent SHALL still create separate files but may omit Role_Files with no findings
2. WHEN a Role_Perspective is triggered but finds no issues, THE Review_Agent SHALL create the Role_File with a "No issues found" statement
3. WHEN the Review_Output_Directory does not exist, THE Review_Agent SHALL create it before writing review files
4. IF file creation fails for any Role_File, THE Review_Agent SHALL log the error and continue creating remaining files
5. WHEN a user explicitly requests a single-file review, THE Review_Agent SHALL honor that request and create one consolidated file

### Requirement 12: Provide Navigation Between Files

**User Story:** As a code reviewer, I want easy navigation between review files, so that I can jump between perspectives efficiently.

#### Acceptance Criteria

1. THE Summary_File SHALL include a table of contents with links to all generated Role_Files and the Verdict_File
2. THE Verdict_File SHALL include a link back to the Summary_File at the top
3. WHEN a Role_File references a finding from another role, THE Review_Agent SHALL include a relative link to that Role_File
4. THE Review_Agent SHALL use relative Markdown links (e.g., `[Security Review](05-se-review.md)`) for all cross-references


### Requirement 13: Provide Temp Directory for Agent Working Files

**User Story:** As a code reviewer using an AI agent, I want the agent to use a dedicated temp directory within the materials folder, so that intermediate files are organized and not scattered across random system temp locations.

#### Acceptance Criteria

1. THE Review_Agent SHALL create a subdirectory named `temp/` within the Review_Output_Directory
2. THE Temp_Directory path SHALL be `.materials/<timestamp>/temp/`
3. THE Review_Agent SHALL write all intermediate working files to the Temp_Directory
4. THE Review_Agent SHALL use the Temp_Directory for file list caching, partial analysis results, and draft content
5. WHEN the review is complete, THE Review_Agent MAY clean up the Temp_Directory or leave it for debugging purposes
6. THE prep-review workflow SHALL inform the Review_Agent of the Temp_Directory location in review_prompt.md

### Requirement 14: Include Documentation Files in Review Context

**User Story:** As a code reviewer, I want project documentation included in the review context, so that I can verify code changes align with documented architecture, API contracts, and design decisions.

#### Acceptance Criteria

1. THE prep-review workflow SHALL identify and include Documentation_Files when packing review context
2. THE prep-review workflow SHALL search for these documentation patterns:
   - `README.md`, `README.txt`, `README` in project root and subdirectories
   - Files in `docs/`, `documentation/`, `.github/` directories
   - Architecture diagrams: `*.drawio`, `*.mmd` (Mermaid), `*.puml` (PlantUML), `*.svg`, `*.png` in docs/
   - API specifications: `openapi.yaml`, `openapi.json`, `swagger.yaml`, `api-spec.md`
   - Design documents: `DESIGN.md`, `ARCHITECTURE.md`, `ADR-*.md` (Architecture Decision Records)
3. THE prep-review workflow SHALL include documentation files in review_context.xml alongside source code
4. THE Review_Agent SHALL reference Documentation_Files when evaluating architectural consistency
5. WHEN documentation contradicts code changes, THE Review_Agent SHALL flag the discrepancy in the Software Architect review
6. THE pack-materials workflow SHALL document the documentation inclusion strategy in pack-materials.md

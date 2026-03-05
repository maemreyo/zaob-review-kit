# Role Standards — Loading Guide

This directory contains full checklists for each reviewer perspective.
Files are loaded **on demand**, one at a time, immediately before writing
the corresponding review output file.

## Loading Protocol

```
READ  role-standards/<NN>-<role>-standard.md
APPLY checklist to the diff
WRITE <NN>-<role>-review.md
NEXT  role — do not re-read the previous standard
```

Do **not** load multiple standard files simultaneously.
Each standard is ~3–5K tokens. Loading all at once wastes 60K+ tokens
and degrades accuracy on the actual code under review.

## File Index

| File                  | Role                     | Always?   |
| --------------------- | ------------------------ | --------- |
| 01-swe-standard.md    | Senior Software Engineer | Yes       |
| 02-sa-standard.md     | Software Architect       | Yes       |
| 03-qa-standard.md     | Quality Assurance        | Yes       |
| 04-pe-standard.md     | Performance Engineer     | Triggered |
| 05-se-standard.md     | Security Engineer        | Triggered |
| 06-oe-standard.md     | Operations Engineer      | Triggered |
| 07-de-standard.md     | Database Engineer        | Triggered |
| 08-ux-standard.md     | Frontend / UX Engineer   | Triggered |
| 09-cl-standard.md     | Compliance Engineer      | Triggered |
| 10-ceo-standard.md    | CEO / Business           | Triggered |
| 11-da-standard.md     | Devil's Advocate         | Triggered |
| 12-mle-standard.md    | ML / AI Engineer         | Triggered |
| 13-api-standard.md    | API Design               | Triggered |
| 14-finops-standard.md | FinOps / Cloud Cost      | Triggered |
| 15-dx-standard.md     | Developer Experience     | Triggered |

## Severity Labels

Every finding must carry exactly one label:

| Label          | Meaning                                       | Blocks merge?  |
| -------------- | --------------------------------------------- | -------------- |
| `[BLOCKER]`    | Incorrect behaviour, security hole, data loss | Yes            |
| `[MAJOR]`      | Significant flaw, hard to fix later           | Yes            |
| `[SUGGESTION]` | Better approach exists; current code works    | No             |
| `[NIT]`        | Style, naming, minor readability              | No             |
| `[QUESTION]`   | Needs clarification before proceeding         | Author decides |
| `[PRAISE]`     | Acknowledge good practice                     | N/A            |

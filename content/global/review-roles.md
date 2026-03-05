# Role Registry

## How to use this file

1. Scan the trigger table below to decide which roles to activate for this diff
2. **Core roles** always run — no trigger needed
3. For each activated role, load its standard file from `role-standards/`
   **just before** writing that role's review section (see Loading Protocol in `review-prompting.md`)
4. Never load all standard files at once — one at a time, write output, move on

---

## Core Roles (always activate)

| #   | Role                               | Standard file                       | Covers                                                             |
| --- | ---------------------------------- | ----------------------------------- | ------------------------------------------------------------------ |
| 01  | **SWE** — Senior Software Engineer | `role-standards/01-swe-standard.md` | Correctness, error handling, naming, SOLID, code smells, DRY       |
| 02  | **SA** — Software Architect        | `role-standards/02-sa-standard.md`  | Architecture alignment, ADRs, coupling, scalability, API contracts |
| 03  | **QA** — Quality Assurance         | `role-standards/03-qa-standard.md`  | Test coverage, test design quality, testability, defect prevention |

---

## Triggered Roles

Activate when the diff matches the trigger condition. Multiple roles can trigger on the same diff.

| #   | Role                             | Standard file                          | Trigger                                                             |
| --- | -------------------------------- | -------------------------------------- | ------------------------------------------------------------------- |
| 04  | **PE** — Performance Engineer    | `role-standards/04-pe-standard.md`     | DB queries, nested loops, caching, async/await, migrations          |
| 05  | **SE** — Security Engineer       | `role-standards/05-se-standard.md`     | Auth/authz, user input, new dependencies, PII, crypto, file access  |
| 06  | **OE** — Operations Engineer     | `role-standards/06-oe-standard.md`     | New endpoints, config/env changes, scheduled jobs, IaC, retry logic |
| 07  | **DE** — Database Engineer       | `role-standards/07-de-standard.md`     | Schema migrations, ORM model changes, new queries, seed data        |
| 08  | **UX** — Frontend Engineer       | `role-standards/08-ux-standard.md`     | UI components, CSS/styling, accessibility, Core Web Vitals          |
| 09  | **CL** — Compliance Engineer     | `role-standards/09-cl-standard.md`     | PII handling, GDPR/CCPA, cookies/tracking, open-source licensing    |
| 10  | **CEO** — Business Perspective   | `role-standards/10-ceo-standard.md`    | Breaking changes, public API surface, user-visible behaviour        |
| 11  | **DA** — Devil's Advocate        | `role-standards/11-da-standard.md`     | Major features, architecture decisions, significant scope           |
| 12  | **MLE** — ML / AI Engineer       | `role-standards/12-mle-standard.md`    | ML models, LLM integration, dataset pipelines, model serving        |
| 13  | **API** — API Design Reviewer    | `role-standards/13-api-standard.md`    | New REST/GraphQL/gRPC endpoints, versioning changes, SDK surface    |
| 14  | **FinOps** — Cloud Cost Engineer | `role-standards/14-finops-standard.md` | New infra, auto-scaling, data transfer, compute/storage changes     |
| 15  | **DX** — Developer Experience    | `role-standards/15-dx-standard.md`     | Public/internal APIs, SDK changes, README, onboarding flow          |

---

## Quick Trigger Reference

```
auth / login / token / permission / session  →  SE (05)
DB query / migration / schema / ORM          →  PE (04) + DE (07) + OE (06)
new endpoint / background job / cron         →  OE (06)
new dependency                               →  SE (05)
async / await / lock / channel               →  PE (04)
breaking change / public API                 →  CEO (10) + DA (11)
UI component / CSS / a11y                    →  UX (08)
PII / GDPR / cookie / licence                →  CL (09)
ML model / LLM / dataset / inference         →  MLE (12)
REST / GraphQL / gRPC / OpenAPI              →  API (13)
new infra / cloud resource / Lambda          →  FinOps (14) + OE (06)
README / SDK / doc / changelog               →  DX (15)
major feature / arch change                  →  DA (11)
```

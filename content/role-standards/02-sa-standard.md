# SA Standard — Software Architect

**Perspective:** System thinker. Does this fit, scale, and stay maintainable as the codebase grows?

---

## 1. Architectural Alignment

- [ ] Does the change follow documented ADRs? Deviations must be intentional and noted.
- [ ] Does it respect established layer boundaries? (e.g., no DB access from route handlers)
- [ ] Is the module/service decomposition preserved, or does this create an inconsistency?
- [ ] If a new architectural decision is made here, is an ADR written or updated?

## 2. Coupling & Cohesion

- [ ] Are new dependencies one-directional? No circular imports introduced.
- [ ] Is the new code cohesive — does it belong in this module, or is it pulled in from elsewhere?
- [ ] Is the public surface area minimal? (expose what callers need, hide the rest)
- [ ] Can this module be tested, deployed, or replaced independently of its callers?

## 3. Scalability & Extensibility

- [ ] Will this design hold under 10× current load without a rewrite?
- [ ] Is state managed appropriately — stateless where possible, explicit state transitions where not?
- [ ] Are extension points provided for foreseeable future needs, without speculative generality?
- [ ] Does horizontal scaling remain feasible (no hard dependency on a single instance's memory)?

## 4. API & Contract Design

- [ ] Public API is stable and versioned — callers won't need changes when internals evolve
- [ ] Request/response contracts are well-typed; no untyped `Map<String, Any>` blobs
- [ ] Breaking changes are explicit — old callers will fail loudly, not silently misbehave
- [ ] Error contracts are as well-defined as success contracts

## 5. Technical Debt

- [ ] Is new technical debt introduced? Is it tracked (comment + ticket)?
- [ ] Does the change improve, worsen, or preserve the existing debt level?
- [ ] Shortcuts are labelled: `// DEBT(#123): skipping validation until auth refactor`

## 6. ADR Quality Check

If a new ADR was written, verify it includes:

```markdown
## Status
## Context       ← what forced this decision?
## Decision      ← what was decided (assertive)
## Consequences  ← positive, negative, neutral
## Alternatives Considered
```

## 7. Non-Functional Properties (ISO/IEC 25010)

- [ ] **Reliability** — can it recover from partial failure?
- [ ] **Maintainability** — will a new team member understand this in 6 months?
- [ ] **Portability** — does this hardcode environment assumptions?
- [ ] **Interoperability** — are integration contracts honoured and documented?

## Output Format

```
[MAJOR] This handler imports directly from `db/` bypassing the service layer.
All DB access must go through services/ per ADR-004. Move the query to
UserService::find_by_token().

[SUGGESTION] The new EventBus struct introduces a second pub-sub mechanism
alongside the existing MessageQueue. Consider whether these should be unified
or whether the distinction is intentional and documented.
```

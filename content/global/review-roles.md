# Review Roles

When reviewing code, adopt these perspectives sequentially:

## Senior Software Engineer (SWE)

- Code correctness, edge cases, error handling
- Algorithmic complexity and obvious performance issues
- Code style, readability, maintainability

## Software Architect (SA)

- Design patterns, architectural consistency
- Separation of concerns, dependency management
- Scalability and extensibility implications

## Quality Assurance (QA)

- Test coverage gaps, missing edge case tests
- Integration risks, regression potential
- Acceptance criteria validation

## Security Engineer (SE)

- Authentication and authorization boundaries
- Input validation and injection risks
- Secret exposure, data leakage, dependency vulnerabilities
- (Detail: see review-security.md)

## Performance Engineer (PE)

- Query efficiency, N+1 detection
- Memory allocation and leak risks
- Algorithmic complexity regression
- Blocking operations in async contexts
- (Detail: see review-performance.md)

## Operations Engineer (OE)

The question this role answers: _"If this breaks at 3am, can the on-call engineer diagnose and recover without the author?"_

- **Observability**: Are errors, latency, and key business events logged at the right level? Can a stranger read the logs and understand what happened?
- **Rollback safety**: Can this change be reverted independently? Does it include a migration with no easy down-path?
- **Failure modes**: What happens when a downstream dependency is down or slow? Are timeouts and circuit breakers in place?
- **On-call impact**: Does this add new alert surface? Does it change existing alert thresholds without updating the runbook?
- **Configuration drift**: Are new environment variables or config keys documented? Can the service start with missing config, or will it crash silently?

Trigger: new endpoints, background jobs, schema migrations, changes to retry/timeout logic, new external dependencies.

## Chief Executive Officer (CEO)

- Does this feature solve the right problem?
- Are there breaking changes for existing users?
- If this bug escapes to production, what is the blast radius?
- Does technical debt increase measurably?

## Devil's Advocate

- Challenge assumptions made in the implementation
- Identify what could go wrong in production
- Question whether the change is necessary at all

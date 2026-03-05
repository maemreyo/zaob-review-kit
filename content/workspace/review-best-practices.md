# Team Review Best Practices

<!--
  PURPOSE: This file teaches the AI reviewer your team's specific conventions.
  It is injected into every review alongside the generic role-based guidelines.

  HOW TO USE:
  - Fill in each section with your project's actual rules
  - Keep rules concrete and checkable, not vague ("always validate input" is
    too generic; "validate with the InputValidator struct before any DB call"
    is checkable)
  - Reference specific files or patterns where helpful
  - Delete sections that don't apply to your stack
  - Commit this file; update it whenever a new convention is agreed on

  AGENT: Read this file before reviewing. Apply all rules below as additional
  criteria. Flag violations with the section name, e.g.:
  "[MAJOR] Violates team rule: Error Handling — raw DB error exposed to caller"
-->

## Architecture Patterns

<!-- Which patterns are required / forbidden in this codebase? -->
<!-- Examples:
- All business logic lives in `services/`, never in route handlers
- Route handlers may only call one service function; composition happens in services
- Direct DB access from agent/worker code is forbidden — go through the API
-->

## Error Handling

<!-- How should errors be handled and surfaced? -->
<!-- Examples:
- Never expose raw database errors to API responses; map to domain errors first
- All service functions return Result<T, AppError>; never panic in production paths
- Log errors at ERROR level with structured fields before returning to caller
-->

## Data & Persistence

<!-- Conventions for DB access, migrations, data shapes -->
<!-- Examples:
- Every query must filter by center_id from JWT; never query without tenant scope
- Money fields: always NUMERIC(15,2) / rust_decimal::Decimal, never f64
- Soft delete: use deleted_at IS NULL in every query, never hard delete user data
- Migration files are append-only; never modify an existing migration after it ships
-->

## Security Rules

<!-- Project-specific security constraints beyond generic OWASP -->
<!-- Examples:
- JWT-protected routes: center_id comes from JWT claims only, never from query params
- Tool endpoints (X-Internal-Key): center_id comes from query param (OpenFang-supplied)
- Never log user PII (name, phone, email) in structured log fields
- Phone numbers must be normalized to E.164 before storage
-->

## Testing Conventions

<!-- What makes a test acceptable here? -->
<!-- Examples:
- Every new service function needs at least one unit test and one integration test
- Integration tests use sqlx::test with real migrations, never mock the DB
- Test names: test_<function>_<scenario> (snake_case), describe what the test proves
- No trivial tests: assert!(true) or asserting only that code doesn't panic are rejected
-->

## Naming & Style

<!-- Conventions not caught by the linter/formatter -->
<!-- Examples:
- Handler functions named as verbs: list_students, create_student, not get_all or students
- Boolean fields: is_active, has_guardian (not active, guardian_exists)
- TODO comments must include phase reference: TODO(P2.1): description
-->

## Known Anti-Patterns

<!-- Things that have caused bugs before — watch for recurrence -->
<!-- Examples:
- Don't use COALESCE($1, existing_value) for optional updates without checking
  that $1=NULL is the intended "no change" sentinel — see Bug M3
- Don't add .unwrap() in async handlers; use ? and map_err to StatusCode
- Don't clone AppState inside handlers; it's already Arc-wrapped
-->

## Module-Specific Notes

<!-- Per-module context that affects review decisions -->
<!-- Examples:
- eduos-core: pure library, no Axum/HTTP imports allowed
- eduos-api: all handlers must be in routes/, middleware in middleware/
- migrations/: filenames are timestamps; ordering matters for FK references
-->

## Deferred / Known Issues

<!-- Issues the team is aware of but not fixing yet -->
<!-- List here so the reviewer doesn't re-flag them as new findings -->
<!-- Examples:
- Phone uniqueness across centers is not enforced at DB level (tracked: Issue #42)
- resolve_user tool has no center_id filter by design (1-instance-1-center assumption)
-->

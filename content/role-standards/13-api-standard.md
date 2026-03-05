# API Standard — API Design Reviewer

**Perspective:** API consumer. Is this API intuitive, stable, and safe to build on?

References: Google API Design Guide (AIP), Stripe API Design Principles, OpenAPI 3.1, Postel's Law.

---

## 1. Resource Modelling

- [ ] Resources are **nouns**, not verbs: `/orders` not `/getOrders`, `/users/{id}/activate` not `/activateUser`
- [ ] Resource hierarchy is logical and reflects the domain (max 2–3 levels of nesting)
- [ ] IDs are stable and opaque — UUID preferred over sequential integers that reveal scale
- [ ] Singleton resources (`/account`) vs. collections (`/accounts`) used correctly

## 2. HTTP Semantics

- [ ] Correct HTTP methods:
  - GET: read, idempotent, no side effects
  - POST: create, or non-idempotent action
  - PUT: full replace (idempotent)
  - PATCH: partial update (idempotent)
  - DELETE: remove (idempotent)
- [ ] Correct status codes:
  - 200 OK / 201 Created / 204 No Content for success
  - 400 Bad Request for validation errors
  - 401 Unauthorised for missing/invalid auth
  - 403 Forbidden for valid auth but insufficient permission
  - 404 Not Found for missing resource
  - 409 Conflict for duplicate / state conflict
  - 422 Unprocessable Entity for semantic validation failure
  - 429 Too Many Requests for rate limiting
- [ ] `Location` header returned on 201 Created pointing to the new resource URL
- [ ] `ETag` / `Last-Modified` on cacheable resources

## 3. Request & Response Design

- [ ] Request bodies validated with schema (required fields, types, constraints documented)
- [ ] Response shape is consistent — the same resource type always returns the same fields
- [ ] Collection endpoints support pagination — cursor-based preferred over offset/limit
- [ ] Collections return metadata: `total`, `next_cursor`, `prev_cursor` (or `Link` headers for REST)
- [ ] Filtering, sorting, and searching via documented query params
- [ ] Response never exposes internal IDs, DB columns, stack traces, or implementation details
- [ ] Envelope consistent across all endpoints (`data`, `error`, `meta` — pick one and use it everywhere)

## 4. Error Design (Stripe Standard)

Every error response must include:
- HTTP status code
- Machine-readable `code` string (stable across API versions, e.g. `"validation_error"`)
- Human-readable `message` string
- `request_id` for traceability
- Field-level detail for validation errors: `[{ "field": "email", "code": "invalid_format", "message": "..." }]`

Errors must NOT leak: stack traces, SQL queries, internal file paths, server hostnames.

## 5. Versioning & Breaking Changes

**Non-breaking (safe to add without a version bump):**
- Adding new optional fields to responses
- Adding new endpoints
- Adding new optional request parameters with defaults

**Breaking (require a new major version or deprecation period):**
- Removing or renaming fields
- Changing field types
- Changing status codes on existing endpoints
- Changing the semantics of an existing field

- [ ] All changes correctly classified as breaking vs. non-breaking
- [ ] Breaking changes have a migration guide (before/after examples)
- [ ] Deprecated fields marked and sunset date communicated
- [ ] Versioning strategy consistent with the rest of the API (`/v1/` in path, or `API-Version` header)

## 6. Security

- [ ] Every endpoint is authenticated (explicit decision documented to make one public)
- [ ] Rate limiting headers present: `X-RateLimit-Limit`, `X-RateLimit-Remaining`, `Retry-After`
- [ ] Idempotency keys supported on mutating operations where retries are likely (payment-grade)
- [ ] API keys / tokens only accepted in headers — never in URL query params (they end up in logs)

## 7. Documentation & Contract

- [ ] OpenAPI 3.1 spec updated and accurate for all changed endpoints
- [ ] Every endpoint has: description, request schema, all response schemas (success + all errors)
- [ ] Example request / response provided for every endpoint
- [ ] Changelog entry written for any additive or breaking change

## Output Format

```
[BLOCKER] POST /users returns 200 OK on success. It should return 201 Created
with a Location: /users/{id} header. Clients that check for 201 to detect
creation will misinterpret this as an existing resource being returned.

[MAJOR] The error response for invalid input is {"error": "invalid"} with no
field detail. Clients cannot tell which field failed or why. Add field-level
errors: [{"field": "email", "code": "already_exists", "message": "..."}].

[SUGGESTION] The collection endpoint GET /reports returns all results with no
pagination. When the table has 100K rows, this will OOM the application server.
Add cursor-based pagination with a default page size of 50.
```

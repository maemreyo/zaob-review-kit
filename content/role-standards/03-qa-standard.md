# QA Standard — Quality Assurance

**Perspective:** Defect hunter. Is this code truly verifiable, and are the tests meaningful?

---

## 1. Test Coverage — What's Missing

- [ ] Happy path covered
- [ ] Boundary values tested: zero, one, max, empty, null/None
- [ ] At least one negative case: invalid input, missing required data, wrong type
- [ ] Auth/permission edge cases: wrong role, missing token, cross-tenant access
- [ ] Downstream failure: what happens when a dependency returns an error or times out?
- [ ] Concurrent/duplicate requests where idempotency matters
- [ ] Rollback / partial failure: what if step 2 of a 3-step operation fails?

## 2. Test Design Quality

- [ ] Tests follow **Arrange / Act / Assert** (or **Given / When / Then**)
- [ ] One logical assertion concept per test (multiple `assert_eq!` on the same result is fine; asserting unrelated things is not)
- [ ] Tests are deterministic — no `sleep()`, no dependency on wall clock, no random without seeded RNG
- [ ] Test data is isolated — tests do not share mutable state or execution order dependencies
- [ ] Test names describe the scenario: `test_create_user_returns_conflict_when_email_exists`

## 3. Coverage Thresholds

| Scope | Target |
|-------|--------|
| New / changed code — line coverage | ≥ 80% |
| Critical paths (auth, payment, data mutation) | ≥ 95% |
| Branch coverage | ≥ 70% |

Coverage must not **decrease** from the baseline without explicit justification.

## 4. Testability of the Code Under Review

- [ ] Functions can be tested in isolation — no hidden globals, no hardcoded singletons
- [ ] External dependencies are injectable (trait objects, interfaces, constructor injection)
- [ ] No hardcoded environment-specific values (`localhost:5432`) in production code paths
- [ ] Side effects are separated from pure logic where feasible

## 5. Test Anti-Patterns (flag these)

| Anti-Pattern | Description |
|---|---|
| **Tautological test** | Asserts that code doesn't panic, or `assert!(true)` |
| **Omnibus test** | One test exercises 10 different scenarios — can't tell what broke |
| **Mockery overload** | Everything is mocked; the test proves nothing about real integration |
| **Time bomb** | `assert_eq!(result.date, "2025-01-01")` — will fail next year |
| **Flaky sleep** | `thread::sleep(Duration::from_millis(100))` to wait for async work |
| **Shared fixture mutation** | Two tests write to the same DB record; order-dependent |

## 6. Regression Risk

- [ ] Has a previously reported bug been fixed? Is there a test that would have caught it originally?
- [ ] Does the change touch a frequently-broken area? (Check `review_memory.md` for patterns)
- [ ] Does any existing test become less meaningful after this change?

## 7. Suggested Tests Format

For every gap found, write a concrete test specification:

```
- **[test name]** (unit | integration | e2e)
  Scenario: <one sentence>
  Input: <specific values or state>
  Expected: <return value, side effect, or error>
  Catches: <what bug or regression this prevents>
```

## Output Format

```
[MAJOR] No test for the case where `user_id` belongs to a different tenant.
The service function queries by ID without a center_id guard — a cross-tenant
leak that existing tests would not catch.

Suggested test:
- test_get_order_rejects_cross_tenant_id (integration)
  Scenario: User from tenant A requests order owned by tenant B
  Input: valid JWT for tenant A, order_id from tenant B
  Expected: 403 Forbidden, no data returned
  Catches: IDOR / tenant isolation breach
```

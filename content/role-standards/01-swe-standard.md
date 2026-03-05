# SWE Standard — Senior Software Engineer

**Perspective:** Code craftsman. Is this code correct, readable, and maintainable?

---

## 1. Correctness & Logic

- [ ] Does the code implement the stated requirements correctly?
- [ ] Are edge cases handled? (empty input, zero, null/None, max values, duplicates)
- [ ] Are error conditions caught at the right level and surfaced meaningfully?
- [ ] Are all conditional branches intentional and covered?
- [ ] Are race conditions possible on shared mutable state?
- [ ] Does the code behave correctly on first call, repeated calls, and concurrent calls?

## 2. Error Handling

- [ ] Errors are not silently swallowed (`catch {}`, `unwrap()` in production paths, `_ =`)
- [ ] Error messages include enough context for debugging (what failed, what value caused it)
- [ ] Resources are released on both success and error paths (files, connections, locks)
- [ ] Panics / crashes are not possible on valid (if unexpected) input

## 3. Naming & Readability

- [ ] Variable, function, and type names convey intent without requiring comments to decode
- [ ] No single-letter names except conventional loop indices (`i`, `j`) and closures
- [ ] Boolean names read as assertions: `is_active`, `has_permission`, not `active`, `permission`
- [ ] Function names are verbs: `create_user`, `fetch_orders`, not `user`, `orders`
- [ ] Magic numbers replaced with named constants

## 4. SOLID & Design Principles

- [ ] **Single Responsibility** — does each function/class do one thing?
- [ ] **DRY** — is the same logic duplicated in two or more places?
- [ ] **Open/Closed** — can behaviour be extended without modifying existing code?
- [ ] **Dependency Inversion** — does the code depend on abstractions, not concretions?
- [ ] Functions are short enough to fit on one screen (~30–50 lines max)

## 5. Code Smells (flag by name)

| Smell | What to look for |
|-------|-----------------|
| **God Class / Function** | One unit doing too many unrelated things |
| **Feature Envy** | A method uses another object's data more than its own |
| **Primitive Obsession** | Using raw strings/ints where a small type would be clearer |
| **Long Parameter List** | More than 4–5 parameters; consider a config struct |
| **Shotgun Surgery** | One logical change requires edits in 5+ unrelated places |
| **Divergent Change** | One class changes for many different reasons |
| **Message Chain** | `a.b().c().d()` — brittle train wreck |
| **Speculative Generality** | Abstractions for requirements that don't exist yet |
| **Dead Code** | Unreachable branches, unused parameters, commented-out code |

## 6. Dependencies

- [ ] New libraries are justified (not reinventing existing functionality)
- [ ] No dependency added just for one small utility function
- [ ] Version is pinned or has a bounded constraint

## 7. Documentation

- [ ] Public API functions have doc comments (purpose, params, return, panics/errors)
- [ ] Complex algorithms have a "why" comment, not just "what"
- [ ] TODOs reference a ticket or issue number

## Output Format

```
[BLOCKER] src/orders.rs:42 — integer overflow possible when `quantity * price`
exceeds i32::MAX. Use i64 or saturating_mul.

[SUGGESTION] src/users.rs:18 — `get_user_by_email_from_db` could be `find_user`.
The storage layer is already implied by the module context.

[NIT] src/lib.rs:5 — unused import `std::collections::BTreeMap`.
```

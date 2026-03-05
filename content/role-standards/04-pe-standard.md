# PE Standard — Performance Engineer

**Perspective:** Efficiency optimiser. Will this regress latency, throughput, or resource usage?

---

## 1. Database & Queries

- [ ] **N+1 query** — is a DB call made inside a loop over a result set? Batch with `WHERE id IN (...)` or a JOIN.
- [ ] Missing index on columns used in WHERE, JOIN, or ORDER BY for the new query
- [ ] Unbounded query — no LIMIT on a table that will grow (will return 100K rows someday)
- [ ] `SELECT *` where only specific columns are needed
- [ ] Transaction scope wider than necessary — holds lock across non-DB work
- [ ] ORM-generated SQL inspected — no surprise eager-load JOINs or lazy-load waterfalls
- [ ] Bulk operations use batch insert/update, not row-by-row in a loop

## 2. Algorithmic Complexity

- [ ] O(n²) or worse on a collection that scales with input — flag explicitly
- [ ] Nested loops where inner collection is fetched from DB or network each iteration
- [ ] Sorting or dedup inside a hot path that runs on every request
- [ ] Regex compiled on every call instead of once at startup
- [ ] Recursive function without memoization on overlapping subproblems

## 3. Memory & Allocation

- [ ] Large objects (result sets, file contents) not held in memory longer than needed
- [ ] Unnecessary clone/copy of large data structures in hot paths
- [ ] Resource leak — file handles, DB connections, channels not released on error paths
- [ ] Object creation in tight loops allocating heap repeatedly (consider pre-allocation or pooling)

## 4. Async & Concurrency

- [ ] Blocking call (sync I/O, `thread::sleep`, mutex lock) inside an async function
- [ ] Lock held across an `await` point — other tasks starved while waiting
- [ ] Thundering herd — if a cache expires or a connection pool is exhausted simultaneously
- [ ] Unbounded channel or queue that could grow without backpressure

## 5. Caching

- [ ] Is caching applied at the right layer? (Not caching inconsistent data; not over-caching mutable data)
- [ ] Cache invalidation strategy is correct and tested
- [ ] Cache keys are collision-free (include tenant/user scope where needed)
- [ ] TTL is appropriate for the data's change frequency
- [ ] Cache stampede handled (mutex/lock on first fill, or probabilistic early expiry)

## 6. Network & I/O

- [ ] Unnecessary round-trips — multiple sequential calls that could be batched or pipelined
- [ ] Payload size appropriate — no over-fetching (returning 50 fields when 5 are needed)
- [ ] Connection pooling used; not creating a new connection per request
- [ ] Timeouts set on all outbound calls

## 7. Default Targets (override per project in `review-best-practices.md`)

| Metric | Warning threshold |
|--------|-------------------|
| API response p99 | > 500ms |
| Single DB query | > 100ms |
| Queries per request | > 5 (investigate) |
| Memory per request delta | Growing unbounded |

## Output Format

```
[BLOCKER] services/reports.rs:87 — fetching user details inside a loop over
order_ids. For 1,000 orders this makes 1,000 DB round-trips. Replace with:
  let users = UserRepo::find_by_ids(&order.user_ids).await?;
and build a HashMap for O(1) lookup.

[SUGGESTION] The compiled Regex in validate_phone() is rebuilt on every call.
Move it to a static LazyLock<Regex> or OnceLock<Regex> at module level.
```

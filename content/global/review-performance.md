# Performance Review

## Performance Engineer (PE) Role

Flag regressions before they reach production:

### Database & Queries

- N+1 query pattern — DB call inside a loop over results?
- Missing index on a column used in WHERE, JOIN, or ORDER BY?
- `SELECT *` where only specific columns are needed?
- Transaction scope wider than necessary?

### Memory & Allocation

- Unnecessary clone/copy of large data structures?
- Large objects held in memory longer than needed?
- Resource leak — file handles, connections, channels not closed/dropped?

### Async & Concurrency

- Blocking call (I/O, sleep, mutex) inside an async function?
- Lock held across an await point?
- Thundering herd if a cache expires or a connection pool is exhausted?

### Algorithmic Complexity

- Hidden O(n²) — nested loop where outer collection grows with input?
- Sort or dedup inside a hot path?
- Regex compiled on every call instead of once?
- Recursive function without memoization on repeated subproblems?

## Trigger Conditions

Apply Performance Engineer role when the diff touches:

- Code paths that handle high request volume or large data sets
- Any new nested loop
- Database layer — queries, migrations, schema changes
- `async`, `await`, threads, mutexes, channels
- PR description mentions "optimization", "performance", or "scale"

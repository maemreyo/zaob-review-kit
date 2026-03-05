# DE Standard — Database / Data Engineer

**Perspective:** Data steward. Is the schema sound, migration safe, and data integrity enforced?

---

## 1. Schema Design

- [ ] Tables and columns follow the project's naming convention (check `review-best-practices.md`)
- [ ] Appropriate data types chosen — avoid `TEXT` when `VARCHAR(n)` suffices; avoid `FLOAT` for money (use `NUMERIC`/`DECIMAL`)
- [ ] Nullable columns are intentional — every nullable column should have a documented reason
- [ ] Primary keys defined on every table; type consistent with the rest of the schema (UUID vs. serial)
- [ ] Foreign key constraints declared where referential integrity is required
- [ ] CHECK constraints used for simple domain rules (non-negative amounts, valid enum values)

## 2. Migration Safety

- [ ] Migration is **backward compatible** — old binary can run against new schema:
  - Adding a nullable column: ✓ safe
  - Adding a NOT NULL column without a default: ✗ breaking for old binary
  - Renaming a column: ✗ breaking — use expand/contract (add new, backfill, drop old)
  - Dropping a column still read by old binary: ✗ breaking
- [ ] Migration is **idempotent** where the DB engine allows it (`CREATE TABLE IF NOT EXISTS`, `ADD COLUMN IF NOT EXISTS`)
- [ ] Down-migration (rollback) script exists and is tested on a copy of production data
- [ ] Large table migrations avoid full table locks:
  - Adding an index: use `CREATE INDEX CONCURRENTLY`
  - Adding a NOT NULL column: add nullable → backfill → add constraint in separate step
- [ ] Migration tested against a realistic data volume — not just an empty test database

## 3. Query Correctness & Safety

- [ ] All queries are correctly scoped to the requesting tenant/user (no missing WHERE clauses)
- [ ] `SELECT *` avoided — explicit column lists prevent breakage when schema changes
- [ ] Aggregations on large tables justified — consider pre-aggregation or materialised views
- [ ] JOINs are on indexed columns
- [ ] Execution plan (`EXPLAIN ANALYZE`) reviewed for any new complex query
- [ ] Transactions used where multiple writes must be atomic; scope is as narrow as possible

## 4. Index Strategy

- [ ] Indexes exist for all foreign keys (many DBs don't add these automatically)
- [ ] Indexes exist for columns frequently used in WHERE, JOIN, ORDER BY
- [ ] Composite index column order matches the most selective column first
- [ ] No redundant index added (covering an existing index's prefix)
- [ ] Index does not duplicate an existing unique constraint

## 5. Data Integrity

- [ ] UNIQUE constraints on columns that must be unique (don't rely on application-level checks alone)
- [ ] NOT NULL + DEFAULT together for new required columns (safe migration pattern)
- [ ] Soft-delete pattern consistent with the rest of the codebase (`deleted_at` vs. `is_deleted`)
- [ ] Cascading deletes / updates are intentional and documented

## 6. Data Security

- [ ] PII columns identified; encryption at rest applied where required by policy
- [ ] Sensitive data (passwords, tokens) never stored in plain text
- [ ] Row-level security applied where multi-tenancy requires it

## Output Format

```
[BLOCKER] migrations/010_add_payments.sql:8 — adding NOT NULL column `currency`
without a default to an existing table. This will fail on deploy if any existing
rows exist, and the old binary will error on INSERT. Add a DEFAULT 'USD' to the
migration (or a separate backfill step) and remove the default in a follow-up.

[MAJOR] The new `find_by_email` query in UserRepo has no index on `users.email`.
On a table with 500K rows this will be a full sequential scan on every login.
Add: CREATE INDEX CONCURRENTLY idx_users_email ON users (email);

[SUGGESTION] The `reports` table has no index on `(center_id, created_at)` which
is the most common filter combination. This will become expensive as the table grows.
```

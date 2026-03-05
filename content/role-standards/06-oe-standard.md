# OE Standard — Operations Engineer / SRE

**Perspective:** On-call engineer. If this breaks at 3 AM, can I diagnose and recover it alone?

References: Google SRE Book, Four Golden Signals, DORA Metrics, 12-Factor App.

---

## 1. Observability — Logs

- [ ] Structured logging used (JSON or key=value); no bare `println!` / `console.log` in production
- [ ] Log levels used correctly: DEBUG (dev trace), INFO (business events), WARN (recoverable anomaly), ERROR (needs attention)
- [ ] Critical operations emit an INFO log: request received, job started, external call made
- [ ] Logs include a correlation/trace ID for distributed tracing
- [ ] No PII, passwords, or tokens in log fields
- [ ] No log spam — a tight loop does not emit one log per iteration

## 2. Observability — Metrics & Tracing

- [ ] New code paths expose relevant metrics (request counter, latency histogram, error counter)
- [ ] Four Golden Signals covered for new services: latency, traffic, errors, saturation
- [ ] Distributed traces propagate context (W3C Trace Context headers or B3)
- [ ] Critical code paths instrumented with spans

## 3. SLOs & Alerting

- [ ] SLIs defined or updated for new user-facing operations
- [ ] Alerts are on SLO burn rate or meaningful thresholds — not raw CPU % that pages at 2 AM for no reason
- [ ] On-call runbook created or updated for new failure modes
- [ ] Alert fatigue avoided: no alerting on events that cannot be actioned

## 4. Deployment Safety

- [ ] Change can be deployed and **rolled back** independently of other changes
- [ ] Feature flag exists for the new behaviour (can disable without a redeploy)
- [ ] Database migrations are backward compatible — old binary can run against new schema
  - Use expand/contract pattern: add column → deploy → backfill → add NOT NULL in next release
- [ ] Zero-downtime deployment is possible (no hard cut-over that drops in-flight requests)
- [ ] New environment variables / config keys are documented; service fails loudly on startup if missing (not silently at runtime)

## 5. Resilience

- [ ] Timeouts set on every outbound call (HTTP client, DB query, external API)
- [ ] Retries use exponential backoff with jitter — not a tight `for i in 0..3` loop
- [ ] Circuit breaker in place for critical dependencies (or documented reason why not)
- [ ] Graceful degradation defined: what happens to users when a dependency is down?
- [ ] Health check endpoints (`/health`, `/ready`) updated if new dependencies added

## 6. Infrastructure as Code

- [ ] IaC changes (Terraform, Helm, K8s manifests) reviewed with the same rigour as code
- [ ] No hardcoded secrets in IaC
- [ ] Resource limits and requests set for container changes (no unbounded CPU/memory)
- [ ] IaC is idempotent — applying twice produces the same result

## 7. Incident Response Readiness

- [ ] Runbook updated (new failure modes documented with diagnosis + recovery steps)
- [ ] Logs are searchable for the failure scenarios this change could produce
- [ ] MTTD (Mean Time to Detect) not worsened — the failure will be visible in dashboards

## 8. DORA Metrics Impact

Flag changes that worsen these metrics:

| Metric | Question |
|--------|---------|
| Deployment Frequency | Does this make deployments more complex or risky? |
| Lead Time | Does this increase time from commit to production? |
| Change Failure Rate | Does this increase the likelihood of a production incident? |
| MTTR | Does this make recovery harder or slower? |

## Output Format

```
[MAJOR] The new background job has no timeout on the external HTTP call.
If the vendor API hangs, the job worker will be stuck indefinitely, eventually
exhausting the worker pool. Add a 30-second timeout and a dead-letter queue
for failed jobs.

[SUGGESTION] New env var DATABASE_POOL_SIZE is read at query time and defaults
to None silently. Consider reading it at startup and failing fast with a clear
error message if it's missing or unparseable.
```

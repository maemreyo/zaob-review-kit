# FinOps Standard — Cloud Cost Engineer

**Perspective:** Finance-aware engineer. Will this create unexpected cloud costs at scale?

References: FinOps Foundation Framework, AWS/GCP/Azure Cost Optimisation pillars.

Trigger: new infrastructure, auto-scaling changes, data pipelines, large data transfers,
new cloud services, or any change with significant compute/storage/egress implications.

---

## 1. Compute Right-Sizing

- [ ] Instance / container types match actual workload — not over-provisioned "just in case"
- [ ] CPU and memory limits/requests calibrated to measured usage (not left at defaults)
- [ ] Spot / Preemptible / ARM instances evaluated for fault-tolerant workloads
- [ ] Serverless vs. always-on evaluated based on traffic pattern (serverless expensive at high steady-state load)
- [ ] Auto-scaling policy prevents over-provisioning during low-traffic periods

## 2. Storage

- [ ] Storage class matches access pattern — hot/warm/cold tiering applied
- [ ] Data retention policies enforced (old data moved to archive or deleted)
- [ ] Unneeded snapshots, AMIs, volumes, and backups cleaned up
- [ ] Database storage growth rate estimated; unbounded growth flagged
- [ ] Log retention not unbounded (cost and compliance)

## 3. Data Transfer & Egress

- [ ] Cross-region data transfer minimised (egress between regions is expensive in all clouds)
- [ ] CDN used for static assets with appropriate cache TTLs
- [ ] Large payload transfers use compression
- [ ] Service-to-service communication stays within the same region / VPC where possible
- [ ] Data transfer volume for new features estimated in GB/month

## 4. Database Cost

- [ ] Read replicas are appropriately sized (not production-tier for dev/staging)
- [ ] Connection pooling prevents idle connection waste
- [ ] Expensive queries (full table scans, large aggregations) identified and optimised
- [ ] Reserved instances / committed use discounts evaluated for steady-state workloads

## 5. Third-Party API Costs

- [ ] External API call volume estimated (LLM tokens, SMS, email, maps, payment APIs)
- [ ] Caching used to avoid redundant external API calls
- [ ] Rate limits respected to avoid overage charges
- [ ] Cost per unit operation estimated and approved by the product owner

## 6. Cost Tagging & Attribution

- [ ] All new resources tagged: team, service, environment, cost-centre
- [ ] Tags follow the agreed schema — no ad-hoc tag names
- [ ] Budget alert thresholds set for new services (alert at 80%, hard stop optional)

## 7. Cost Impact Estimate

Include this table in the review output when infra is being added or changed:

| Dimension | Current ($/mo) | Projected ($/mo) | Delta |
|---|---|---|---|
| Compute | | | |
| Storage | | | |
| Data transfer | | | |
| Third-party APIs | | | |
| **Total** | | | |

Flag for discussion if projected monthly delta > $500.

## Output Format

```
[MAJOR] The new image processing Lambda is configured with 3008 MB memory and
no concurrency limit. At the current upload rate (est. 50K/day) this will cost
~$340/month. A 1024 MB configuration is sufficient for this workload based on
the p99 duration; reduce memory and add a reserved concurrency of 20 to prevent
runaway costs if upload volume spikes.

[QUESTION] The new pipeline copies the full S3 bucket to a different region
on every run. Is this intentional? Cross-region egress at the current data
volume (~500 GB) will add ~$45/month. Consider using S3 replication rules
instead of a full copy job.
```

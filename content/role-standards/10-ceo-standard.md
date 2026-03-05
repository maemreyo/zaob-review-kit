# CEO Standard — Business / Product Perspective

**Perspective:** Company owner. Does this move the right needle, safely, for users and the business?

Trigger: breaking changes, public API changes, user-visible behaviour, significant scope.

---

## 1. Business Alignment

- [ ] Does this change deliver the stated business objective?
- [ ] Is the scope right — not over-engineered for the business need?
- [ ] Does it align with the current product roadmap?
- [ ] Are success metrics (KPIs) defined so we'll know if it worked?

## 2. User Impact

- [ ] Does this improve or degrade the user experience?
- [ ] Are existing users affected? If so, is a migration or communication plan in place?
- [ ] Is the change backward compatible for API consumers?
- [ ] Does this affect SLA commitments to customers?
- [ ] Could this cause data loss, confusion, or a support spike?

## 3. Risk Assessment

- [ ] What is the blast radius if this fails in production? (Single user / all users / data corruption)
- [ ] Is there a tested rollback plan?
- [ ] Are there compliance or legal risks? (Escalate to CL role if unsure)
- [ ] Does this introduce vendor lock-in without a clear rationale?

## 4. Simplicity vs. Investment

- [ ] Is there a simpler solution that delivers 80% of the value at 20% of the risk?
- [ ] Is the technical debt trade-off explicitly documented and accepted?
- [ ] Is the delivery timeline realistic?

## 5. Competitive & Strategic

- [ ] Does this differentiate the product, or is it undifferentiated infrastructure?
- [ ] Could this be bought / used as a managed service instead of built?
- [ ] Are there reputational or brand risks if this goes wrong publicly?

## Output Format

```
[MAJOR] This change silently changes the meaning of the `status` field in the
API response from "active/inactive" to "active/paused/cancelled". Existing
integrations that check `status !== "inactive"` will misinterpret "paused" as
active. This needs a versioning strategy or a deprecation notice.

[QUESTION] The feature is gated behind a new plan tier. Is there a migration
path for existing paid users, and has the pricing page been updated?
```

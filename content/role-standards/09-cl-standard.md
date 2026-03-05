# CL Standard — Compliance / Legal Engineer

**Perspective:** Regulatory guard. Will this expose the company to legal, privacy, or licensing risk?

---

## 1. Data Privacy (GDPR / CCPA / PDPA)

- [ ] New data collection is in scope of the existing privacy policy
- [ ] Lawful basis for processing PII is identified (consent, contract, legitimate interest)
- [ ] Data minimisation: only data strictly necessary for the feature is collected
- [ ] Consent mechanism is explicit, granular, and revocable
- [ ] Data subject rights are supported: access, correction, deletion, portability
- [ ] PII is not retained beyond the documented retention period
- [ ] Cross-border transfers have appropriate legal mechanism (SCCs, adequacy decision)

## 2. Cookie & Tracking Compliance

- [ ] Cookies categorised: strictly necessary, functional, analytics, marketing
- [ ] Non-essential cookies are blocked until consent is given
- [ ] Cookie consent banner updated if new tracking technology is added
- [ ] Consent is stored and auditable

## 3. Industry-Specific Rules

Apply the relevant section based on the project domain:

**Healthcare (HIPAA):**
- [ ] PHI is encrypted at rest and in transit
- [ ] Access to PHI is logged and audited
- [ ] BAA (Business Associate Agreement) in place for any vendor receiving PHI

**Payments (PCI-DSS):**
- [ ] Cardholder data (PAN, CVV, expiry) is not stored unnecessarily
- [ ] Tokenisation used instead of raw card data
- [ ] PCI scope not expanded without security review

**Children's Products (COPPA):**
- [ ] No collection of personal data from users under 13 without verifiable parental consent

## 4. Open-Source Licensing

- [ ] New dependencies' licences are compatible with the project licence
- [ ] GPL/AGPL dependencies do not create unexpected copyleft obligations for a proprietary product
- [ ] Licence files included where required (e.g., MIT requires copyright notice in distributions)
- [ ] If adding AGPL code to a SaaS product: legal review required

## 5. Audit & Record-Keeping

- [ ] Audit logs are immutable and retained per regulatory minimums (GDPR: as long as data is held; HIPAA: 6 years)
- [ ] Security events recorded with timestamps and actor identity
- [ ] Audit log entries are not user-deletable

## Output Format

```
[BLOCKER] The new "export my data" endpoint returns all rows from the users
table without filtering. This would expose other users' PII, violating GDPR
Article 15 (right of access is scoped to the requesting data subject only).

[MAJOR] The analytics tracking added in this PR fires on page load before the
user has interacted with the consent banner. Under GDPR this is unlawful
processing. Analytics must not fire until consent is granted.

[QUESTION] The new `session_recordings` table stores video replays indefinitely.
What is the retention policy, and is it documented in the privacy policy?
```

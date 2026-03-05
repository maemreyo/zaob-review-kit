# SE Standard — Security Engineer

**Perspective:** Adversarial thinker. Where can an attacker break, leak, or bypass?

References: OWASP Top 10 (2025), CWE Top 25, OWASP ASVS Level 2.

---

## 1. Injection (OWASP A03 / CWE-89, CWE-79)

- [ ] All SQL queries use parameterised statements — no string concatenation with user input
- [ ] No dynamic query construction from user-controlled data
- [ ] HTML output is context-encoded (not raw user strings inserted into HTML)
- [ ] Shell commands, file paths, LDAP/XPath queries: user input never interpolated directly
- [ ] File uploads: type, size, and content validated; stored outside web root; filename sanitised

## 2. Authentication & Session (OWASP A07 / CWE-287)

- [ ] Standard auth library used — no roll-your-own crypto or token generation
- [ ] Passwords hashed with bcrypt / Argon2 / scrypt — never MD5, SHA1, or plain SHA256
- [ ] Session tokens are cryptographically random (≥ 128 bits entropy)
- [ ] Sessions invalidated on logout and on privilege change
- [ ] JWT: algorithm explicitly specified; `alg: none` rejected; secret ≥ 256 bits
- [ ] Cookie flags set: `Secure`, `HttpOnly`, `SameSite=Strict` (or `Lax` with justification)

## 3. Authorisation & Access Control (OWASP A01)

- [ ] Every endpoint checks authorisation, not just authentication
- [ ] Principle of least privilege — the operation uses the narrowest permission needed
- [ ] **IDOR** — can a user access another user's resource by changing an ID in the request?
- [ ] Vertical escalation — can a low-privilege user reach a high-privilege action?
- [ ] Tenant isolation — multi-tenant systems filter by tenant ID from JWT, never from request body

## 4. Sensitive Data Exposure (OWASP A02)

- [ ] PII, passwords, tokens are never logged (not even at DEBUG level)
- [ ] Secrets not hardcoded — use environment variables or a secrets manager
- [ ] Sensitive data encrypted at rest where required (AES-256 minimum)
- [ ] TLS enforced for data in transit; no HTTP fallback for sensitive operations
- [ ] Error responses do not leak stack traces, DB schemas, or internal paths to callers

## 5. Dependency & Supply Chain (OWASP A06)

- [ ] New dependencies checked for known CVEs (`cargo audit`, `npm audit`, `pip-audit`, Snyk)
- [ ] Dependency versions are pinned or range-bounded
- [ ] No transitive vulnerability introduced at high/critical severity
- [ ] Third-party scripts loaded from versioned, integrity-checked sources (no CDN wildcards)

## 6. Security Misconfiguration (OWASP A05)

- [ ] Debug mode / verbose errors disabled in production configuration
- [ ] Default credentials not used
- [ ] Security headers present on HTTP responses: `Strict-Transport-Security`, `X-Frame-Options`, `X-Content-Type-Options`, `Content-Security-Policy`
- [ ] CORS policy is restrictive — not `*` for credentialed requests

## 7. Cryptography (CWE-327)

- [ ] Strong algorithms: AES-256, RSA-2048+, SHA-256+; no DES, RC4, MD5, or SHA1 for security
- [ ] IVs / nonces are random and unique per operation; never reused
- [ ] Key management: keys not hardcoded; rotation strategy exists

## 8. STRIDE — Mini Threat Model

For each significant new component or endpoint, answer:

| Threat | Question |
|--------|---------|
| **Spoofing** | Can an attacker impersonate another user or service? |
| **Tampering** | Can data be modified in transit or at rest? |
| **Repudiation** | Can a user deny performing an action we need to audit? |
| **Information Disclosure** | Can sensitive data be read by unauthorised parties? |
| **Denial of Service** | Can the service be made unavailable through this code path? |
| **Elevation of Privilege** | Can a user gain permissions they shouldn't have? |

## 9. LLM / AI-specific (if applicable)

- [ ] Prompt injection: user-supplied content not directly interpolated into system prompts
- [ ] Indirect prompt injection: external content (emails, web pages) reaching the LLM is sandboxed
- [ ] LLM output not passed to `eval()`, shell, SQL, or rendered as raw HTML
- [ ] PII not sent to third-party LLM APIs without consent and DPA in place

## Output Format

```
[BLOCKER] src/auth/handler.rs:34 — password compared with == instead of a
constant-time comparison function. Timing attacks can leak whether the first N
characters are correct. Use a constant-time compare (e.g., subtle::ConstantTimeEq).

[BLOCKER] migrations/008_add_reports.sql:12 — raw user input interpolated into
the ORDER BY clause via format!(). This is SQL injection. Use a whitelist of
allowed column names and reject anything else.

[MAJOR] src/routes/invoices.rs:67 — invoice fetched by ID without checking
that it belongs to the requesting user's tenant. Horizontal privilege escalation
(IDOR) is possible.
```

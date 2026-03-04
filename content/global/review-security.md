# Security Review

## Security Engineer (SE) Role

Scan every change for these vulnerability classes:

### Authentication & Authorization

- Does each endpoint/function verify auth before executing?
- Can a user escalate privileges by manipulating parameters?
- Is token/session lifecycle handled correctly (expiry, revocation, rotation)?

### Input Validation

- Is user-controlled input sanitized before use in queries, commands, or file paths?
- SQL injection, XSS, path traversal, command injection risks?
- Is deserialization of untrusted data guarded?

### Secrets & Credentials

- Hardcoded API keys, passwords, or tokens in source?
- `.env` files or credential files at risk of being committed?
- Are credentials written to logs?

### Dependencies

- Do newly added packages have known CVEs?
- Is version pinning specific enough to prevent supply-chain drift?

### Data Exposure

- Do API responses leak internal paths, stack traces, or user PII?
- Does logging capture sensitive fields?
- Are error messages safe to surface to end users?

## Trigger Conditions

Always apply Security Engineer role when the diff touches:

- `auth/`, `login`, `token`, `password`, `permission`, `role`, `session`
- New third-party dependencies
- Request/response handling or serialization
- Database queries or raw SQL
- File system access with user-supplied paths

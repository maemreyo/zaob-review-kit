# Review Prompting Guidelines

## Output Structure

Always structure your review with:

1. **Summary** — one paragraph describing what changed and why
2. **Risk Assessment** — Low / Medium / High with justification
3. **Findings** — organized by role perspective
4. **Recommendations** — prioritized action items
5. **Verdict** — Ship / Ship with changes / Needs rework / Needs discussion

## Calibration Rules

### By size

- Small (< 50 lines): SWE focus — correctness and style
- Medium (50–300 lines): SWE + SA
- Large (> 300 lines): full multi-role review

### By change type (overrides size rules)

| Change contains                                    | Add roles                       |
| -------------------------------------------------- | ------------------------------- |
| `auth/`, `login`, `token`, `permission`, `session` | Security Engineer               |
| DB queries, migrations, schema changes             | Performance Engineer + SA       |
| Public API surface changes                         | SA + CEO + Devil's Advocate     |
| New dependencies                                   | Security Engineer               |
| Breaking changes                                   | CEO + Devil's Advocate (always) |
| `async`, concurrency primitives, locks             | Performance Engineer            |

### Verdict scale

- **Ship** — no blocking issues
- **Ship with changes** — minor issues, safe to fix in follow-up
- **Needs rework** — blocking issues present, do not merge
- **Needs discussion** — architectural questions must be resolved first

## Tone

- Be direct but constructive
- Cite specific line numbers when possible
- Distinguish between blocking issues and suggestions
- Acknowledge what was done well

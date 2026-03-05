# DX Standard — Developer Experience / Documentation Engineer

**Perspective:** First developer user. Can someone new use this successfully without asking anyone?

Trigger: public or internal APIs with new endpoints, SDK changes, major feature additions,
CLI tool changes, README changes, or any change affecting how other engineers consume the system.

---

## 1. README & Getting-Started Quality

- [ ] README answers: what does this do, who is it for, and how do I run it in under 5 minutes?
- [ ] Prerequisites explicit — exact versions where they matter (Node 20+, Rust 1.78+)
- [ ] "Hello World" or quickstart is copy-paste runnable with zero prior knowledge
- [ ] Common setup errors have documented solutions (top 3 issues from Slack/issues)
- [ ] README updated to reflect new features, config changes, or removed commands

## 2. Code-Level Documentation

- [ ] Public functions / methods have doc comments: purpose, parameters, return value, errors/panics
- [ ] Complex algorithms have a "why" comment at the decision point (not just restating the code)
- [ ] Non-obvious gotchas, ordering requirements, or invariants documented at the point of use
- [ ] `TODO` and `FIXME` have ticket references — not open-ended "fix this someday"

## 3. API / SDK Ergonomics

- [ ] API follows the principle of least surprise — does what the name says, nothing hidden
- [ ] Naming conventions consistent across the entire public surface (not `getUserById` in one place and `fetchUser` in another)
- [ ] Sensible defaults — simple use case requires minimal configuration
- [ ] Error messages guide the developer to the solution:
  - Bad: `"Error: invalid config"`
  - Good: `"Error: DATABASE_URL is required. Set it in .env or pass --database-url. See docs/setup.md."`
- [ ] Breaking changes in the SDK have a migration guide with before/after code examples

## 4. Changelog & Versioning

- [ ] CHANGELOG.md updated with a human-readable summary of what changed and why
- [ ] Semantic versioning respected: MAJOR for breaking, MINOR for additive, PATCH for fixes
- [ ] Deprecated items annotated (`@deprecated`, doc comment, or migration note)
- [ ] Migration guide written for breaking changes

## 5. Onboarding Friction (Time-to-First-Success)

Target: a new engineer should be able to make their first successful API call / run the tests in **< 15 minutes** from a fresh clone.

- [ ] Local development setup is scripted and reproducible — not word-of-mouth
- [ ] `make`, `just`, or `npm run` has standard targets: `build`, `test`, `lint`, `dev`/`run`
- [ ] Environment variables documented in `.env.example` with explanatory comments
- [ ] No steps that require tribal knowledge or Slack DMs to complete
- [ ] Setup script checks for missing prerequisites and fails with a helpful message

## 6. Internal Developer Experience

- [ ] Port conflicts documented (what port does this service use? Does it conflict with others in the stack?)
- [ ] Dev dependencies clearly separated from production dependencies
- [ ] Test suite runs locally without any cloud credentials or VPN (or clearly documented if not)

## Output Format

```
[MAJOR] The new `WebhookClient` constructor takes 6 positional parameters with
no defaults. The fourth parameter `retry_strategy` is required but only matters
for production use — it should default to a sensible value (e.g., ExponentialBackoff
with 3 retries). Requiring it breaks the "simple case should be simple" principle.

[SUGGESTION] The new --format flag accepts "json", "yaml", "toml" but the error
message when an invalid format is passed is just "unknown format". Change to:
'Unknown format "xml". Valid options: json, yaml, toml.'

[NIT] .env.example is missing the three new environment variables added in this
PR. New contributors will get a cryptic error on first run.
```

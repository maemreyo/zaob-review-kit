# DA Standard — Devil's Advocate

**Perspective:** Constructive sceptic. Challenge assumptions, stress-test the design, find the dark side.

Trigger: major features, architecture decisions, significant scope, when the team is converging too fast.

---

## 1. Assumption Challenges

Ask these questions explicitly — document the answers if they reveal new risk:

- What assumptions are baked into this design? Are they validated, or just hoped?
- What happens if the predicted load is 10× higher? 10× lower?
- What if the primary dependency is unavailable for 1 hour? 1 day? Permanently?
- What if the team member who built this leaves and no one understands it?
- What if the requirement changes in 6 months — how hard is the rework?

## 2. Alternative Approaches Not Taken

- What simpler alternative was not considered?
- Why was the existing solution not extended instead of building new?
- What would this look like if built with a different trade-off (sync vs. async, push vs. pull, etc.)?
- Is there an off-the-shelf solution that was discarded without documented reasons?

## 3. Failure Mode Analysis

- What is the most **likely** way this breaks in production in the next 30 days?
- What is the worst-case failure mode (data loss, financial impact, security breach)?
- Are there silent failure paths — code that returns success while doing the wrong thing?
- Is the error handling too optimistic? (Happy-path code with exceptions swallowed)
- What does the system do during a partial outage — fail open or fail closed?

## 4. Over-Engineering Detector

- Is this solving a problem that doesn't exist yet? (YAGNI)
- Is the abstraction level appropriate, or is it premature generalisation?
- Is a performance optimisation being applied without profiling data to justify it?
- Is the complexity justified by the actual requirements, or by anticipated requirements?
- Could a junior developer maintain this in 12 months?

## 5. Distributed Systems Anti-Patterns (flag by name)

| Anti-Pattern | Flag when you see |
|---|---|
| **Distributed Monolith** | Services are "micro" but deploy together and share a DB |
| **Chatty Interface** | 10 fine-grained RPCs where 1 batched call would work |
| **Saga Without Compensation** | Multi-step transaction with no rollback for partial failure |
| **Shared Database** | Two services writing to the same tables |
| **Synchronous Chain** | A → B → C → D, all sync; one slow link blocks everything |
| **Missing Idempotency** | Mutation endpoint called via retry logic, no idempotency key |

## 6. Long-Term Concerns

- Will this be maintainable in 2 years by someone who wasn't in the original design meeting?
- Does this create a future migration cliff (a breaking change that will be expensive to fix later)?
- Are there better industry patterns for this problem that are not being used?
- What operational burden is being added to the on-call rotation?

## Output Format

The Devil's Advocate role uses **questions** more than findings.
Format: state the concern, ask the clarifying question.

```
[QUESTION] The cache is keyed only by `user_id`. If the same user has multiple
active sessions with different permission sets, will they see each other's cached
results? What is the invalidation strategy when permissions change?

[MAJOR] This implementation assumes the payment webhook arrives exactly once.
There is no idempotency check. Payment processors document that webhooks can be
delivered multiple times. A duplicate delivery will double-charge the customer.

[THOUGHT] The three-layer event pipeline (producer → queue → transformer → queue
→ consumer) is powerful but adds significant operational complexity. For the
current volume (~500 events/day), a simpler direct DB write + scheduled job would
be easier to operate and debug. Has this trade-off been explicitly accepted?
```

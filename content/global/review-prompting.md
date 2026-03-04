# Review Prompting Guidelines

## Output Structure
Always structure your review with:
1. **Summary** — one paragraph describing what changed and why
2. **Risk Assessment** — Low / Medium / High with justification
3. **Findings** — organized by role perspective
4. **Recommendations** — prioritized action items
5. **Verdict** — Ship / Ship with changes / Needs rework

## Calibration Rules
- Small changes (< 50 lines): focus on correctness and style
- Medium changes (50-300 lines): add architectural review
- Large changes (> 300 lines): full multi-role review
- Breaking changes: always include CEO and Devil's Advocate perspectives

## Tone
- Be direct but constructive
- Cite specific line numbers when possible
- Distinguish between blocking issues and suggestions
- Acknowledge what was done well

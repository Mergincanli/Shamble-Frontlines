# ADR-NNNN: <title>

**Status:** Proposed | Accepted | Rejected | Superseded by ADR-NNNN
**Date:** YYYY-MM-DD
**Author:** <agent or person> · **Reviewed by:** <reviewer agents + Advisor>
**Signed off by Mustafa:** no | yes (date)

## Context
The situation and the forces at play. Link the code (file:symbol) and docs this rests on.

## Decision
The change we propose, in a few sentences.

## Options considered
### Option A: <name>
| Dimension | Assessment |
|---|---|
| Complexity | Low / Medium / High |
| Cost (money, time, usage) | … |
| Scalability / performance | … |
| Security & anti-cheat | … |
| Effect on the art direction / feel | … |
| Effort from the current code | … |
**Pros:** … **Cons:** …

### Option B: <name>
(same format)

## Trade-off analysis
Why the chosen option wins, and the strongest argument against it.

## Consequences
- Easier: …
- Harder: …
- Revisit when: …
- Hard constraints in CLAUDE.md that change: …

## What a spike must prove
The measurable pass marks a throwaway spike must meet before this decision is accepted (for example: 48 simulated
players at a 30 Hz server tick within X ms). No spike starts before Mustafa and the Advisor have reviewed and approved
the draft; the spike's results then go into this record, and Mustafa accepts or rejects it.

## Migration plan
Phases with verifiable milestones (what proves each phase done). The game must stay playable at every step.

## Test plan
How we will know it works: gates, harnesses, network simulation (latency / jitter / loss), soak, renders, playtests.

## Rollback
How to back out if it goes wrong.

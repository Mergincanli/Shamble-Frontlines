# NPT-EDGE-SLIVER, a question for Mustafa (2026-11-25, 04:2x UTC; answer only if you want (b))

**The question.** In U6c's merge, each probe leg's first reading of dropped packets (the start-edge read your
NPT-EDGE-READ (a) moved) now comes just after the leg starts, not before it. By reading the code, the gap is normally
under a millisecond; it has not been measured. Is that close enough to your words "The 10 seconds of warm-up and the
measured window stay fully counted"?

**Why it matters.** Drops from a stall that starts inside that gap are missed only if the same stall also holds the
reading back by about 28 ms (a figure carried from the advisor agent's round-2 ruling, not measured again). Everything
after the gap is counted, the whole measured window included.

**Options.**
- (a) Keep it as merged: your "one-line move", placed as the advisor agent's round-2 ruling wrote it. Its Low finding
  ADV-M4B-1 then closes.
- (b) A small follow-up before R1 that takes the reading just before the leg starts: a few changed lines, with its own
  plan, advisor check and one counted run. Last day to pick it: the end of Sat 28 November (Amsterdam), so that it lands
  before Stop B's fill on Tue 1 December.

**Recommendation.** (a): the advisor agent's (advisor-u6c-m4b.md, its "Decisions needed"), and mine.

**Where it comes from.** The advisor agent's M4b head read of U6c's two hand edits, 2026-11-25 (APPROVE WITH CONDITIONS
C1-C6): .checkpoints/netcode-proof-test/advisor-u6c-m4b.md. U6c's merge goes on either way; nothing waits for this
answer unless you pick (b).

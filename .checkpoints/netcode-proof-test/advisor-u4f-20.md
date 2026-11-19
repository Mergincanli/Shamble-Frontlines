# Advisor ruling: U4f's judged -20 set (2026-11-19, from main's advisor.md)

Read only. Sources: `.checkpoints/netcode-proof-test/u4f-20-read.md` (the main session's read), the committed results at
`spikes/netcode-proof/results/u4f/350677f0aef2/pm-0001-20/`, the raw logs at
`.gate/netcode-proof-test/runs/u4f-20-350677f0aef2/measured/`, marks.json's PM-0001-20 entry, room.rs, input.rs,
world.rs, ADR 0001 OQ-0001-6, plan.md's option pick. -22's set finished at 21:17 UTC (coordinator's update).

## Plan
1. Verdicts and figures: runs.txt, verdict.json, verdict-bites.json against the read; marks.json :4645-4800.
2. Lost presses: run press_lost.py (`python3 -I`), then spot-check at least two ticks by hand in room.jsonl.
3. Mechanism: room.rs press accounting (:350-356, where `presses` fills), world.rs edge rule, input.rs late drop,
   ADR 0001 :1156-1167, plan.md :728, :843-848.
4. Rule R20-Q1 to Q5; findings ADV-U4F20-n; what goes to Mustafa (TEST-FIRST, history.md:2234-2282).


# verifier-fixcheck-e2 verdicts (ADR 0003, 0005, 0006, GLOSSARY)
HEAD 59d41e0 · base 4374a1c

## Claims
| # | doc:line | claim (short) | cited | verdict | where and the words | note |
|---|---|---|---|---|---|---|
| G1 | GLOSSARY.md:783 | definition kept (1 − p99 ÷ 8.33 ms) | condition 3 | confirmed | HEAD 783 = base 783's formula, unchanged | |
| G2 | GLOSSARY.md:784-788 | measured in wave 1 after the sim core spike, an early signal only; again at slice 6, the run that decides | SPIKE-ORDER (Mustafa), TICK-60-SETUP (Advisor) | confirmed | inputs: "then the sim core spike (then the 60 Hz run)"; TICK-60-SETUP quote word for word | attribution right |
| G3 | GLOSSARY.md:789-792 | 60 Hz needs, in the deciding run, margin ≥ 25 %, research marks, and the goal budget holding at rooms per VM at 60 Hz (PM-0001-63; ADR 0001 part 2) | TICK-60-HEADROOM; condition 3 | confirmed | inputs: "60 Hz also needs the goal budget to hold at the rooms per VM measured at 60 Hz"; PM-0001-63 exists (0001:1518), part 2 at 0001:133-138 | |
| G4 | GLOSSARY.md:793-795 | Depends on: confirmed by Mustafa (2026-11-03) with his addition, quoted | TICK-60-HEADROOM (Mustafa) | confirmed | quote word for word; old "Mustafa and the Advisor" corrected to Mustafa | |
| G5 | GLOSSARY.md:1-743 | bytes kept; only §8's 60 Hz entry changed | condition 3 | confirmed | write_guard.sh GUARD PASS; one hunk only (@@ -780 +780) vs 4374a1c | |

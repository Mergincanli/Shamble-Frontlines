# touchup-test-engineer fixes (F7, F6, F12 from verifier-fixcheck-d1), docs at 449356c

F7 holds (the V-S key at 0001:1358–1359 keeps LG's 30-inputs clause for PM-0001-27; the row at 0001:1409 says "not
V-S"), but the fix goes in the key, not the row: the row is the later, deliberate alignment (test-engineer A
follow-up c, "FIX alignment (PM-0001-27: validity is PM-0006-13's, not V-S …)"), and it matches PM-0006-13
("Validity is bracket B's and LG's 80 % rule, not ADR 0001's V-S … ADR 0001's PM-0001-27 is this set of runs, read
the same way", 0006:1183), PM-0006-14 ("validity as PM-0006-13 (…, never ADR 0001's V-S)", 0006:1184) and
SPIKE_PLAN:100–102 ("share one set of runs under PM-0006-13's rules"). Adding the clause to the row alone would make
one set of runs valid in one table and invalid in the other. The density runs still carry PM-0001-2's load counters
("every run of PM-0001-4 to PM-0001-27", 0001:1380) and LG's 80 % rule with its calibration run (0006:1111–1112).
Adds one line.

### FIX F7
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
  under 80 % in every second (ADR 0006's LG key) (proposed: a starved load generator lightens the load). The first two
  clauses hold for single-room marks only: the rooms-per-VM row PM-0001-27 runs under ADR 0006's PM-0006-13 rules
  instead (no pinning, its bracket B, 3 sessions at 3 times of day, steal recorded per second and reported, never a
  validity condition), since eight tick threads cannot each be alone on four vCPUs and on SH steal is what is measured
  ("repeated at 3 times of day (shared hosts vary with neighbours)": r6-infrastructure.md, R6-HOST-REC2's mark, lines
AFTER:
  under 80 % in every second (ADR 0006's LG key) (proposed: a starved load generator lightens the load). The first two
  clauses and the 30-inputs count hold for single-room marks only: the rooms-per-VM row PM-0001-27 runs under ADR 0006's
  PM-0006-13 rules instead (no pinning, its bracket B and LG's 80 % rule, 3 sessions at 3 times of day, steal recorded
  per second and reported, never a validity condition), since eight tick threads cannot each be alone on four vCPUs and
  on SH steal is what is measured ("repeated at 3 times of day (shared hosts vary with neighbours)":
  r6-infrastructure.md, R6-HOST-REC2's mark, lines
END


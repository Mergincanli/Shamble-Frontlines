# infra-engineer, fix round 3 (ADR 0006): the netcode proof test's full name (consistency critic F13; completeness critic item 8)

Scope: every short "proof test" outside "### Pass marks" (doc lines 797-965) that means the netcode proof test.
Found two: line 550 (the finding's own line) and lines 1039-1040 (Rollback, "re-run the / proof test there").
Lines 882 and 884 are in test-engineer's pass-mark rows (left to test-engineer). Line 1050's "netcode proof / test"
is already the full term, split over two lines. BEFORE texts were copied by script from the doc at 57c43c2 and each
occurs exactly once.

### FIX I3-1
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
  two rooms fill a vCPU before any [transport runtime time](../research/GLOSSARY.md#transport-runtime-time). The proof test's measurement replaces both figures.
AFTER:
  two rooms fill a vCPU before any [transport runtime time](../research/GLOSSARY.md#transport-runtime-time). The netcode proof test's measurement replaces both figures.
END

### FIX I3-2
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
- **Game servers:** if the CCX23 cannot be ordered or fails the marks, read OQ-0006-1's fallbacks and re-run the
  proof test there; every draft that cites the reference server (N34, I-15) follows the new name. If the shared CX33
  fails, the fleet stays on dedicated vCPUs (K1). VMs are billed by the hour with a monthly cap (R6-HOST-F17), so a
AFTER:
- **Game servers:** if the CCX23 cannot be ordered or fails the marks, read OQ-0006-1's fallbacks and re-run the
  netcode proof test there; every draft that cites the reference server (N34, I-15) follows the new name. If the shared
  CX33 fails, the fleet stays on dedicated vCPUs (K1). VMs are billed by the hour with a monthly cap (R6-HOST-F17), so a
END

Note on I3-2: the only change is the word "netcode" before "proof test"; "CX33" moves to the next line so both lines
stay within 120 characters (119 and 120).

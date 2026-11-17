# security-auditor: check of U4f's builder brief rev 3

Task: check rev 3 of `.checkpoints/netcode-proof-test/u4f-brief.md` (read only). Notes here only; read-only
scripts under `.checkpoints/netcode-proof-test/u4f-r3/` (prefix `sa_`). Branch head at start: 42e0912.
No runs (no cargo, check.mjs, node). No web reads. Approval-free command forms only.
(No Edit tool in this agent: the file is rewritten whole with Write at each save.)

## Plan
- B0 Read rev 3 (whole), its change list. Read my night check (34a2e55: security-auditor-u4f-check.md) for SA-1..SA-17.
- B1 SA-1..SA-17 one by one: fixed as rev 2 says, still fixed in rev 3, or open. The Highs first (SA-1, SA-2, SA-3).
- B2 Hostile input: §4.10's 11 forged kinds; clamp points; 1.5x clock; lag switch; never-acker. Each: room/server
  check, -21 counter, negative case, bite; no panic/loop/unbounded growth; wrap-aware compare with total order;
  caps; saturating counters; NA-13 (same stamp, new seq = Late, not forged); never a Kick for a forged kind
  (u4-build-plan.md:383-384).
- B3 At least 8 recheck rows bearing on hostile input checked against code at HEAD (file:line).
- B4 Build rules: loopback/L1/L2 only; no secret; approval-free command forms (history.md:2187); the guard's
  refusals; KEPT-LOGS §11's protected folders.
- B5 Missing / contradicting a ruling or his words (:2125, :2127, :2067-2075 size M, RECS-1043 C62HB-Q4 (b)) /
  scope creep; carries 46, 80, 86 and the named others; C62H-BOUND's deferred Lows for U4f.
- B6 Findings with severity and fix; return the schema.

## Blocks done
- B0 done: rev 3 read whole (1,407 lines; head read 7cfae88; sections 0-12, NA table, 45 recheck rows, Q1-Q17,
  D1-D15). Night check (34a2e55) read: SA-1..SA-17. Next: rev 2's change list (SA rows), then the code at HEAD.

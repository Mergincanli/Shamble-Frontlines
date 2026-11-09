# U4 build plan rev 2: netcode-architect notes

Task: read both critics (u4-plan-review-test.md, u4-plan-review-arch.md) in full, check each finding against the
code, rewrite u4-build-plan.md as rev 2 with a §0.1 table (finding -> where rev 2 meets it / deferral / why wrong),
keep and update the advisor questions. No code edits, no builds, no runs.

## Plan
1. Read both reviews in full; list every finding id here.
2. Read plan rev 1 in full (645 lines) and its notes.
3. Check each finding against code / marks.json / README / SERIES / ADRs; a line per finding here.
4. Draft rev 2 text: changes per section; §0.1 table.
5. Write rev 2 to u4-build-plan.md (Write tool), DONE line here.

## Findings log
- step 1 done: both critics' notes read (test T1-T23 one-liners + T1-T4 in the task text; arch A1-A26, N1-N5
  one-liners + A1-A4 in the task text). Step 2 done: plan rev 1 read in full (646 lines) and its notes M1-M25.
- T1 CONFIRMED: marks.json PM-0001-2 checks `snapshot_clients_count` min >= 48 and `soldiers_count` min >= 48 on
  part "SS runs" {scene SS, clients 48}; README:195-200 pre rule; eval.rs:851-887 Rule::Pre applies the matched part's
  checks. -22's checks: headroom min == 0, event.ack_starved.dropped count >= 1 (marks.json -22). A kick drops
  a client -> 47 -> INVALID under pre. Fix: -22 gets no pre (its select is suite never-ack, no scene).
- T2 CONFIRMED: -30 part C select {suite both-transports, profile P0, machine.role C}, no layer (marks.json -30);
  matched_parts eval.rs:921-930 ignores only role. README:31 layer values L1, L2, real.
- T3 CONFIRMED: SERIES.md:266 compares (client, tick) pairs; a split tick with one group lost has no defined client
  hash. Fix chosen: whole-tick hash only on complete ticks; a received-but-refused group logs a sentinel.
- T4 CONFIRMED: -17 checks = figure ==0 + S30 setup (marks.json -17); notes say "fails, never passes, on 0 compared".
  Fix: client_hash covers the rebuilt 32-bit tick; refusal sentinel; client hashes its HELD presentation state
  re-quantised through Quant (so client-rounding has a real path), property test Quant(deQuant(q)) == q.

# Advisor: rulings on U1's open questions (2026-11-06)

Input: `.checkpoints/netcode-proof-test/u1-questions.md` (untracked, read at branch head 5ac4042).
Plan: `.checkpoints/netcode-proof-test/plan.md` rev 3.2. Drafts: `.gate/npt-readings` (1b4094a).

## Plan of work
1. Read the questions file, the plan's relevant sections, the catalogue §8, marks README, marks.json, SERIES.md,
   test-engineer and netcode-architect notes.
2. Check each C item against the code (eval.rs, redact-check.mjs, check.mjs, Cargo.toml).
3. Rule each item; flag any that move a number or meaning (R-PR route).
4. Verdict on U1.

## Notes (as I go)
- Read: questions file, plan §4.1-4.7, §6.1-6.3, §10.2-10.6, §12, §15; catalogue (whole); marks README (whole);
  eval.rs (whole); netcode-architect and test-engineer notes.
- C2 finding: README §2.3 (line 100-101) says "An empty series makes the check fail, never pass"; eval.rs collect()
  returns Err -> Missing::Gap -> Outcome::Gap -> run INVALID (eval.rs:221-229, 369, 888-893). Contract and code
  disagree; the code matches C2. README must change. Also: any check on a count that may legitimately be 0 must not
  read an event series (no events -> gap -> INVALID). To check in marks.json.
- C5: eval.rs:1237-1263 matches README §3.5. eval_part:953 makes a part INVALID if ANY member run is invalid, even
  with min valid passing runs: an old invalid run left in the folder voids the part forever. Check ADR V-S wording.
- B6: eval.rs:1051-1067: no matched part -> checks without parts; none -> Undecided -> NOT-RUN. README §5 lacks the
  "undecided" sentence.
- C2 more: marks.json's zero-expected checks read summary events paired with a "ran" count (good). But SERIES.md has
  no rule that counters/flags are written on every record (0 included): a sparse writer would make clean runs INVALID
  (e.g. tick.v.overrun sum == 0, late-zero's tick.v.late_count). Needs a SERIES.md rule.
- C5 gap (not asked): ADR 0001:1521 "An invalid run ... its cause is found and written down before it is measured
  again". eval_part:953 keeps any invalid run blocking the part forever; the judge has no record of a written cause
  or a superseded run. Needed before the first counted run (U8), not for U1.
- C8: npt-log also exports SCHEMA; npt-log and npt-judge depend on serde_json (plan says "serde only"); check.mjs
  allows serde_json for the judge. Read "serde" as serde + serde_json; name it in plan text.
- C10/C11: check.mjs:36-91 and :99-112 match the session's readings.
- C6/C7: redact-check.mjs:31-46, 101-118 match. Prefix forms only catch prefixed text: U6 must never print a token,
  capability or bind secret in any other form (hex, bare base64).
- Prior rulings: R-15 already reads PM-0004-67's jitter from the first 36,000 measured ticks (Q22 is in R-PR).
- Reproduced: `node spikes/netcode-proof/check.mjs` at 5ac4042: all 9 steps PASS (44 cargo tests, 9 self, 10 redact;
  275 tracked files, 81 marks, rows at 1b4094a unchanged).
- C12: marks.json's 81 entries equal marks-draft.jsonl's 81 objects (0 differ); SERIES.md differs from
  series-draft.md in 4 lines (title, the 2-line provenance, the units rule): hand edits after the copy, fine.
- C2 REPRODUCED (scratch fixture, scratchpad/c2demo): a bite run whose defect FAILs one check ("queue held at the
  cap") and removes an event (count check -> gap) comes out INVALID; the mark INVALID; the bite is never shown.
  eval.rs judge_run:885-896 pushes a gap into invalid_why; eval_bite:1059 checks validity first. Same at run level:
  a measured run with a FAIL and a gap is INVALID, not FAIL (eval_part:949-953).
- C4: `params.killed` also listed for -59 and PM-0006-22 (SERIES.md:351). -59's catch should write its own footer.
- Gate at the working tree (base af8393e): checks 1-4 and 5b-5d PASS; 5a cannot start (no Chromium in this
  container), as on PR #41. Legacy files unchanged (symbols +0 -0).
- C9: files outside §4.1 beyond the session's list: .cargo/config.toml, .gitignore, crates/*/clippy.toml (6),
  crates/judge/tests/, crates/proto/tests/catalogue.rs, ops/operator/redact-check.test.mjs.
- B7: -25 lists V-S and clean-ref at mark level; its Origin parts (ML, MAC) have no RS or LG log, so V-S makes every
  Origin run INVALID today. -58 has no `reorder` rule at all (its P3 part needs it).
- B13: marks.json's -30 has one bite only (wallclock); the draft's second bite ("the validity check itself") is gone.
- Q12: -19's ML Chrome part asks count >= 1: a trap that never rejoins passes.
- Q10: -3's ends are min/max with a band of 5 % of the half jitter (0.25 ms at P1) over 10^6 real-path packets on L2:
  one outlier fails it.
- Q15: -27 "within 5 %" read one-sided moves meaning (the draft's "unchanged"): drafts first.

## Rulings
### A. The catalogue
- A1 APPROVE (a) SessionStatus. Plan §4.7 npt-proto gains it; catalogue row (room -> client, control stream, owner,
  capability holders only, on change, at most 1/s; carries profile in force, server RTT, rewind cap, cmd-timing option,
  clipped-shot count, last saved form step); the U1 audit's 14 names become 15, with its bite; a fuzz target (A6).
- A2 APPROVE. Header params `cmd_timing` (A/B) and `cc` (newreno/cubic), from the unit's configuration on L2 and the
  scenario on L1; a socket control to change them is never added.
- A3 APPROVE. Type=notify, READY=1 at Open, timed by npt-super around `systemctl start`; TimeoutStartSec well above
  2 s so a slow start is measured, not killed. No systemd in this container: npt-super's timing gets a unit test, and
  the first real check is R1's bring-up, before any -16 start counts.
- A4 APPROVE. Audit accepts `authority-only` only; ADR 0001's results at the close note the word against "none" (638).
- A5 SETTLED: agree.
- A6 APPROVE. One fuzz target for the socket messages (1,024 B limit), plus SessionStatus's.
- A7 APPROVE. Plan §10.2 "a per-run capability" -> "a capability per browser slot, valid for the run".
- A8 APPROVE for the spike. ADR 0001's results at the close name it as open for the foundation's protocol (does a full
  state carry the pending reliable events?).
- A9 APPROVE as a reading (a design limit, not a mark's number); written into ADR 0001 at the close.
- A10 done.
### B. marks.json
- B1-B5 SETTLED: agree. B6 SETTLED: agree, but README §5 must say it: "If no check applies to a bite run, the bite
  is undecided and the mark NOT-RUN."
- B7 APPROVE with: a part's `validity` adds to the mark's, never removes; a bite run takes the validity of the parts
  it matches; move -25's V-S and clean-ref to its five flood parts and add `reorder` to -58's P3 part; README §2.2/§4;
  a fixture.
- B8 APPROVE with: `pre:PM-0001-2` on the SS and ES parts at N = 48 of the marks -2's row names (-4 to -27); bite runs
  too; scale-step parts (-10's N = 24, 36) check only GLOSSARY §3's N-scaled counts against the header's `clients`
  (soldiers = N, three teams of N/3, a client on each) and the fixed counts (15, 4, 300), the shot rate reported; a
  fixture.
- B9 APPROVE. Only on checks that state a setup the row names (today's 8 are all honest setups); no bite may target a
  setup check; a fixture.
- B10 APPROVE with: the verdict prints, beside the header's bite size, the size rule B-S or B-N gives from the part's
  valid runs. Not judged.
- B11, single marks:
  - Q10 CHANGE: p0.1 and p99.9 replace min and max for the jitter's ends (targets d - a and d + a, band 5 % of a, as
    drafted); p90 stays; min and max reported. The judge adds `p001`. U3 measures ImpairSocket's release precision;
    worse than 0.25 ms (5 % of P1's half jitter) goes back to the advisor before R1.
  - Q11 APPROVE. Q18 APPROVE. Q19 APPROVE. Q20 APPROVE. Q22 APPROVE (already R-15's words). Q26-27 APPROVE (the row
    states both). Q28 APPROVE. Q29 APPROVE.
  - Q12 CHANGE: ML Chrome part: the harness logs each injected trap; one timed rejoin per trap (joins >= traps + 1),
    at least 2 traps (the row's plural). LG's 100 as drafted.
  - Q13 APPROVE with: R4's run plan creates LG with its IPv6 /64 so the part runs; if LG has none on the day, the
    advisor applies the draft's waiver at the close, recorded. No judge feature.
  - Q14 APPROVE with: measured on RS by npt-server, one monotonic clock, from TCP accept or the first QUIC packet to
    the close; the timer may fire before 2 s, never after.
  - Q15 (a) "within 5 %" one-sided: drafts first (R-PR's route) before R5's first counted run. Words: "killing one
    room's process (SIGKILL) leaves the others' p99 at most 5 % above their own p99 before it; a drop is recorded,
    never a fail (the advisor's reading, 2026-11-06)". Until then the judge keeps the two-sided literal reading.
    (b) PM-0006-13's bite serving -27: APPROVE (one set of runs serves both records).
  - Q21 APPROVE with: a check that the pong delayer itself fired in the rule-cap runs (its own flag).
  - Q23 APPROVE: R2's and R4's run plans run -11 and -25 once per WebTransport crate.
- B12 APPROVE: report-only, never a check; printed escaped, cut to 64 B.
- B13 CHANGE: keep the draft's second bite (ADR 0001 -30). Bite format gains `expect: "invalid"` with the rule name
  (`late-zero`); shown only when its run is INVALID by that rule alone; a valid run is Missed (mark FAIL). Plus the
  judge fixture. README §5.
- B14 APPROVE with: ISO 8601 UTC with seconds and Z; Amsterdam time by the EU rule (CET/CEST, last Sundays of March
  and October at 01:00 UTC), tested across a change; missing -> gap.
- B15 APPROVE.
### C. The session's readings
- C1 APPROVE with: padded_step_ns is the busy-wait's own elapsed (the same clock reads its loop compares).
- C2 CHANGE (reproduced bug): (1) `count` of an empty series is 0, a value; every other statistic of an empty series
  is a gap. (2) Order in a run: validity rules first (INVALID, checks not judged); then any FAIL -> the run fails, gaps
  listed beside it; else any gap -> INVALID. The same in bite runs: shown when any applicable check fails on a run that
  holds the validity rules. (3) README §2.3 rewritten ("An empty series makes the check fail" is wrong today).
  (4) SERIES.md rules: counters and flags written on every record of their kind, 0 included; `second.s.inputs_count`
  lists the playing clients only, never flood or hostile sessions. (5) Fixtures: a bite with one FAIL and one gap is
  shown (the -22 case); a measured run with a FAIL and a gap is FAIL; an empty count is 0.
- C3 APPROVE (with C2's inputs_count rule).
- C4 CHANGE, built at U6: `params.killed` only for a planned SIGKILL (-27's room, PM-0006-22's soak kill), and the
  exemption holds only when the run's npt-super log records that kill of that slot; -59's rooms need none (the catch
  writes the `crash` footer). Fixture at U6.
- C5 APPROVE the order. MISSING (due U8, before R1's first counted run): how a run stops blocking its part. ADR
  0001:1521 lets an invalid run be measured again once its cause is written down; today any invalid run in the folder
  blocks the part forever, and nothing records the cause. Proposal for the advisor at U8: a committed results ledger
  of every counted run with its verdict; an invalid run superseded only with its written cause; a failing run never
  superseded except by a later build that fixes its found cause (cause and fixing commit named); the judge reads the
  ledger and refuses a folder missing a listed run.
- C6 APPROVE with (U6): tokens, capabilities and bind secrets appear as text only in their prefixed form (no hex or
  bare base64 anywhere), with a test.
- C7 APPROVE.
- C8 APPROVE; plan §4.7 names npt-log's SCHEMA, Machine, Build, Phase, Ended, Footer, Log, ReadError, read_log, and
  npt-judge's library (marks, eval, ratio, figures, log_files, read_logs, summary_line); §4.1/§4.2's "serde" reads as
  serde and serde_json.
- C9 APPROVE; §4.1's layout names every file outside it (session's list plus .cargo/config.toml, .gitignore, the
  clippy.toml files, crates/judge/tests/, crates/proto/tests/catalogue.rs, ops/operator/redact-check.test.mjs).
- C10 APPROVE. C11 APPROVE; W-PR's exception names the one workflow file by path, in the same change.
- C12 APPROVE: checked; marks.json objects equal the draft's; SERIES.md's 4 changed lines are header and the units
  rule.

## Drafts route
- Only Q15 (a). Everything else is a reading or a format choice.

## Verdict on U1
- May close once the U1-side changes are in, each with its fixture, the check command green and the gate run:
  C2 (judge + README + SERIES), B6, B7, B8, B9, B13, Q10 (p001 + -3), Q12, Q21, Q22's first-N window, Q27's rule,
  A1 (catalogue row + audit 15), the README entries for B10, B12, B14, B15 (their judge code may come with their first
  writer, U4), and the plan-text changes (A1, A2, A7, C8, C9, C10, C11).
- Carried, named in the checkpoint: C4 and C6 (U6), A3's real check (R1 bring-up), ImpairSocket precision (U3), the
  ledger (U8), Q13 and Q23 (run plans), Q15 (a) (readings PR before R5).

DONE

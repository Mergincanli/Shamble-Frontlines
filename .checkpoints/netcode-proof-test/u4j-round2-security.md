# security-auditor, U4j round 2 (the fix pass on round 1's F1-F12)

## Plan
1. Read the review packet (u4j-round2-packet.md) in full: diffs, plan's U4j section, rulings FX-1..FX-14, round-1 findings.
2. F1 first: read the fixed practice.rs `find` and the factor's computation in the fixed copy; build a table of every
   path (IN TOLERANCE, BEYOND with/without chance ruling, FAR BETTER, a second practice run, a bite run) with f and
   file:line; check FX-6 (pinned bound line), FX-1 (band check), FX-8 (6-decimal cap) for any log input that raises f.
3. F2-F6, F9 first half (FX-11): closed or not, with evidence; new paths (hostile log: panics, overflow, duplicate or
   missing fields).
4. Run the judge's tests through the runner (exit code + test summary line).
5. Verdict.

Copies: pre-fix .gate/u4j-prefix; fixed .claude/worktrees/agent-a3b8dfeb01ed52b50 (never edited).

(Process note: Block 1 below was first appended with a shell heredoc by mistake; this file is rewritten whole with the
Write tool each time since, content kept. The Edit tool is not available in this run. Last rewrite: two marks.rs line
ranges corrected after a re-check, 796-823 -> 791-824 and 619-640 -> 616-640.)

## Notes (a line after each finding)

### Block 1: F1 (the cap), read in the fixed copy (paths under spikes/netcode-proof/crates/judge/src/)
Callers of `practice::find`: eval.rs:444 (in_force, moves the check: :446-450) and eval.rs:998 (the ws-practice
validity rule). Both pass the same (run, ctx.all, ctx.mark, tolerance); find is pure, so both see one result. A ws_bound
check moves only when it judges exactly one run (eval.rs:460); otherwise it is judged unmoved (f = 1): stricter.
Tolerance in: marks.rs:348-353 (ws-practice in [1, 1.10]); parse_bound practice.rs:112-115 (line's tolerance in
[1, 1.10]); marks.rs:808-819 (each ws-practice rule's tolerance == the line's, at load); practice.rs:509 (again, run time).
So tolerance <= 1.10 on every path, from marks.json only.

Common prelude to Ok (every class): spacing :451; counted runs :452-456 (bite: measured_of :406-440); practice_of
:458-470 (candidates = role practice, same binaries, started before, over `all`: :357-367; latest :368-369; lists the
mark :375; one record :387; >1 candidate needs second_ruling :394); always_rules :477; scene/profile :485-491; spacing
:492; record `bound` == mark.ws_bound_line byte for byte :497-507; parse_bound(mark_line) :508 (the LOG's line is never
parsed); bound.tolerance == tolerance :509; checks_agree :515; setup (model, config_hz, RS kernel) :516-554; heal count ==
client_bursts_count :555-564; M_p recomputed from heals :565-571; s = M_p / M_m(marks.json) :572; class_ok :573-594;
f :597-601; guard f > tolerance -> Err :602-607; record's moved numbers must equal the judge's :618-629; k :630;
IN TOLERANCE recheck (<= k over Y', none over Z') :633-647.

| Path | class_ok (:575-580) | f (:597-601) | f <= 1.10? |
|---|---|---|---|
| IN TOLERANCE | band_low <= s <= tolerance (:576); band_low <= 1 (:107-109) | s <= 1 -> 1; else s <= tolerance | yes: f in [1, tol] |
| BEYOND + chance_ruling | s <= tolerance (:577) | max(1, s) <= tolerance | yes |
| BEYOND, no ruling | false -> Err :581 | - | no Ok |
| FAR BETTER + chance_ruling | s < band_low <= 1 (:578) | 1 by class (:597), whatever s | yes: f = 1 |
| FAR BETTER, no ruling | false -> Err | - | no Ok |
| INVALID / other / missing class | false (:579, class "" from :573) | - | no Ok |
| second practice run (>1 candidate) | needs second_ruling (:394), then the rows above | same | yes |
| bite run | p = its set's measured runs' practice run (:452-470), same p, same bound, so same s and f as the counted run | same | yes |
| any of the above, guard | - | f > tolerance -> Err (:602-607), unreachable after :576-578 and :597 | belt and braces |

Log inputs and whether each can raise f:
- record `class`: selects a row; every Ok row caps f (above). No.
- heals -> M_p -> s: f = s only on IN TOLERANCE/BEYOND, both need s <= tolerance. No.
- record `bound`: compared byte for byte with marks.json (:502), never parsed; M_m, band, tolerance, guard, model, commit
  are marks.json's (:508). FX-6 holds. No.
- record moved numbers / k: compared only (:623, :630), never applied. No.
- chance_ruling / second_ruling: gates only (:574, :394); text, never numbers. No.
- band (FX-1): from marks.json, refused at load unless 0 < low <= 1 <= high (:105-111 via marks.rs:802). With
  low <= 1, IN TOLERANCE's s >= low adds no f above 1 and FAR BETTER's s < low <= 1. No.
- 6-decimal cap (FX-8, :63-71): on marks.json's line only (at load and :508). No overflow: band_low and tolerance have
  den <= 1e6, s's terms are i64, so every cross product < 1e26; moved = X|Y|Z (i64) x f (num <= 1.1 x i64) < 1.0e38 <
  i128::MAX 1.7e38. No.
Moved number = ceil_ms(bound x f) (:198-200), so at most bound x 1.10 plus under 1 ms (the advisor's R6 rounding,
ruled); for the provisional X/Y/Z, 1.10 x is a whole ms (737 ms, 2.75 s, 7.59 s): exactly 10 %.
F1 VERDICT: CLOSED. f <= tolerance <= 1.10 on every Ok path; no log input raises it.

### Block 2: tests through the runner
`bash u4j-fix/run.sh cargo test -p npt-judge`: rc=0; the runner's summary lines: lib 14 passed, main 0, cli 2 passed,
engine 43 passed, fixtures 19 passed, doc 0; 0 failed anywhere (17:05-17:06 UTC).

### Block 3: F2-F6, F8, F9 (first half), F11, read in the fixed copy
- F2 (bound line pinned, FX-6): CLOSED. practice.rs:497-507 compares the record's `bound` with mark.ws_bound_line byte for
  byte (text() only filters blank, returns the raw str :333-337); :508 parses marks.json's line, never the log's.
  marks.rs:791-824 requires the line on a mark with ws_bound checks and only there, parses it (band, tolerance, 6
  decimals at load), checks_agree, and each ws-practice tolerance == the line's. Test u4j_ws_the_record_carries_the_
  marks_bound_line (H1 M_m 620 ms -> INVALID; commit-only change -> INVALID). FX-6 (d): params.ws_tcp_model is written
  by npt-net's DelayStream from tcp_model.rs MODEL (SERIES.md:747), not copied from the line, so the provisional model
  text keeps real runs INVALID until fixes.rs's line lands.
- F3 (one practice run per build over every set, FX-3): CLOSED. practice_of :357-367 scans `all` (eval.rs:1725, every
  run read), role practice and equal binaries, no set or listing filter; started before only; the latest must list the
  mark :375; >1 candidate needs second_ruling :394. Test u4j_ws_one_practice_run_per_build_over_every_set (listed and
  unlisted earlier run in another set -> INVALID). Bite at its counted runs' practice run: measured_of :406-440 + :458-470
  (one build, one practice run, else INVALID); the bite's f is its set's measured runs' f, so it cannot be judged looser
  or tighter than a counted run of its set.
- F4 (hs sources, Arm key, FX-5): CLOSED as routed. figures.rs:224-234 + :242-247 (a name in another process's log is a
  gap: closes (i), a client log's own replay_hash); :248 key = role LG and aarch64 (L1 `client` logs never qualify unless
  they claim role LG); :406 + :423 room must be x86_64 (closes (iv)); :423 an orphan counts everywhere (closes (iii));
  :437-446 replay without an entry is a gap. My (ii) half (only where the mark has arm_rule) is FU-5, open, Low.
- F5 (twin role, FX-4): CLOSED for -21. figures.rs:198-203 refuses a twin whose role is not `twin` (gap), as
  fate_log_twin_equal :159-164. Fixture pm-0001-21/twin-is-measured + profile/bite cases. -30's form: carry U6-a.
- F6 (panic on a long decimal, FX-8): CLOSED. practice.rs:63-71 refuses > 6 digits after the point before Ratio::parse
  (itself <= 30 digits, ratio.rs:76-78); parse_bound now runs only on marks.json's line (marks.rs:802 at load,
  practice.rs:508), so no log text reaches it at all. Unit test the_bound_line_refuses_more_than_6_decimals (probe B ->
  Err). No other new arithmetic overflows (Block 1's bounds).
- F8 (coverage floor, FX-10): CLOSED as ruled. figures.rs:345-357 list lengths == room header's clients; :358-391
  snap_incomplete lists (length, 0/1); coverage() :460-536 (hash and incomplete at once; every server tick inside a
  slot's span resolved; span covers the room's measured ticks); order :542-559 (mismatch, then empty, then gap, then
  waiting). Residual (the advisor's R2.4, a U4r carry): a client log that marks every tick snap_incomplete 1 but one
  still passes on 1 compared; the L1 P0 setup check waits for U4r's evidence. Not reopened.
- F9 first half (FX-11): CLOSED. Practice::describe :204-233 prints class, M_p, M_m, s, band text, tolerance text, f and
  why ("FAR BETTER: at the bound" | "max(1, s)"), then each ruling with {:?} (Debug-escaped: a ruling text cannot forge a
  line). Test asserts "practice run ws-practice: BEYOND" and the chance ruling's text. Second half FU-2 open (Medium):
  any non-empty text still passes as a ruling (:574, :394), so PR #40 stays needs-mustafa on it.
- F11 (ws_bound shapes, FX-6 (f)): CLOSED. marks.rs:616-640 pins x p50/<=/no where, y count/where >/<=, z max/<=/no
  where; test ws_bound_checks_keep_their_shape gains the four refusals.
- Not claimed by a fix, still open as routed: F7 (FU-1), F9 later half (FU-2), F10 (FU-3), F12 (FU-4), F4 (ii) (FU-5).

### Block 4: new paths the fix opened (hostile log input)
- Panics: none found. measured_of's logs[0] (:411, :418) is safe: runs() builds each Run from >= 1 log (eval.rs:37-45).
  heal[...] (:565, :639): heal.len() == client_bursts_count > 0 (parse_bound :99), nearest_rank clamps to [1, n]
  (eval.rs:265-269). slot_hashes' merged[slot] (:268): every list of a name has the first list's length (:256).
  decoded's clients (:335-343): u32 -> usize, no index by it. Ratio: Block 1.
- Overflow: none (Block 1). Negative heal times: s < 0 only passes as BEYOND + ruling, at f = 1 (tighter).
- Duplicate fields: still last-wins in read_log (F10, FU-3, pre-existing); with the line pinned, a duplicated `bound`
  key must still equal marks.json's line in the copy the judge reads, so it can no longer move a number.
- Missing fields: each is an Err (no started_utc, burst_spacing, model, config_hz, RS kernel, class, bound, m_p_ns,
  moved numbers, k): INVALID, never a default.
- Wider reach of a malformed practice run: practice_of calls started(p)? (:363) on every same-build practice run in any
  set before the time filter, so a later practice run with no or a malformed started_utc turns a FAIL into INVALID
  (FX-2's direction, by a malformed log). Not a finding: a planted log can already make any run INVALID (an extra log
  under the counted run's id with no footer fails `complete`), and the run script always writes started_utc (B14);
  skipping such a run instead would let a stripped timestamp hide a first practice run from the one-per-build count.
- New finding N1 (Low, test evidence): FX-1's layers (b) FAR BETTER f = 1 (:597) and (c) the guard (:602-607) are
  unreachable once (a) holds at load and the line is pinned. So the advisor's third plant for round 2 ("FAR BETTER
  f = max(1, s), with the band check and the pin removed") leaves u4j_ws_far_better_past_the_cap_is_invalid INVALID:
  if "pin removed" keeps parse_bound(mark_line), class_ok refuses s = 1.9 against marks.json's band 0.965; if it parses
  the record's line again, the guard refuses f = 1.9 > 1.1. The test turns PASS only with the guard removed too. This is
  the cap holding by several layers, not a hole; the plant must drop the guard as well (or the test must assert which
  refusal it got) to prove that test catches anything. Not before merge.
- New finding N2 (Low, pre-existing, not from the fix): npt-log's check_header (log/src/lib.rs:305-321) only refuses an
  empty run/set/role/process, so a run id can hold a newline; judge/src/lib.rs:49-51 prints why.first() raw on stdout,
  and the why lines carry run ids (eval.rs:1323, :1339, :1579). FX-11's describe goes only into the escaped JSON `needs`,
  so it adds no stdout path. A log could print a forged second summary line to a person reading stdout. Fix:
  check_header refuses control characters in those four fields; ride FU-3 (the next npt-log change). Not before merge.
- Fixtures diff (u4j-round2-packet-fixtures.diff): no IPv4 address added, no token/secret/key-like text (grep counts 0).

### Block 5: coverage table (judge input -> where the fixed judge validates it -> gap)
| Input (log or record) | Validated at | Gap |
|---|---|---|
| bound line (M_m, band, guard, model, commit, X/Y/Z/k, tolerance) | marks.json only: marks.rs:791-824 at load; practice.rs:497-515 | none |
| practice class and s -> f | practice.rs:572-607 | none (cap holds; N1 is about the tests only) |
| practice run choice (one per build, before the run) | practice.rs:349-402 over every run read | none |
| bite run's numbers | practice.rs:406-470 | none |
| practice run's setup (model, CONFIG_HZ, RS kernel, heal count) | practice.rs:516-564 | none |
| chance / second ruling | non-empty text, printed escaped (:204-233) | FU-2 (Medium, open) |
| hs source per name | figures.rs:224-247 | none |
| Arm rule key | figures.rs:248, :406, :423 | FU-5 (mark flag, Low, open) |
| decoded coverage | figures.rs:345-391, :460-536 | R2.4 residual (all-but-one incomplete; U4r carry) |
| twin identity (-21) | figures.rs:195-203 | -30's form: U6-a |
| ws_bound check shape | marks.rs:616-640 | none |
| C4 bite direction | eval.rs (reading (a)) | FU-1 (Low, open) |
| duplicate JSON keys | none (last wins) | FU-3 (Low, open) |
| transport label | none (select only) | FU-4 (Low, open) |
| control characters in run/set/role/process | none (non-empty only) | N2 (Low, new note, pre-existing) |

### Durable decisions (for my memory; no memory folder exists on disk, so they live here for the session to file)
- Threat model for npt-judge: every results log is hostile input; marks.json is reviewed input. A number that moves a
  pass bar must come from marks.json or be recomputed from the run's own series, never read from a log's claim.
- A judge reading a folder cannot defend against deleted or planted logs turning a FAIL into INVALID; that direction
  is accepted (fail-closed against PASS). What it must never do is turn a FAIL or INVALID into a PASS, or move a bar.
- WS-BAR cap: f <= tolerance <= 1.10 on every path, held by four layers (the pinned line, band_low <= 1 at load,
  FAR BETTER f = 1, the guard); rounding up to the next ms is the advisor's R6 and accepted.

## Verdict
APPROVE WITH FINDINGS. F1 CLOSED (Block 1 table). F2, F3, F4 (as routed), F5, F6, F8 (as ruled), F9 first half, F11
CLOSED. New: N1 Low (test evidence for the plant), N2 Low (pre-existing). Neither opens a round 3. Still open security
follow-ups FU-1 to FU-5 (F7, F9's later half, F10, F12, F4 (ii)) keep PR #40 needs-mustafa, as it already is. DONE.

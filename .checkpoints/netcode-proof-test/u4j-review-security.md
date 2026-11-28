# U4j review, round 1: security-auditor (validity rules)

Reviewer: security-auditor (opus). Read only: no edit, build or run in the worktree
/home/user/Shamble-Frontlines/.claude/worktrees/agent-a3b8dfeb01ed52b50 (base 411eeeed50d8c71e7e1f74366f0e1d5c27388444).
Started 2026-11-11. No earlier notes file (fresh run). No agent memory folder found (memory: local; none on disk).
Paths below are under spikes/netcode-proof/ in that worktree. Line numbers are the worktree's.

## Plan
1. Read the diff (git diff base, git status untracked) and the files: log/src/lib.rs (hs), judge marks.rs, eval.rs,
   figures.rs, practice.rs, lib.rs; tests engine.rs and fixtures.rs only as needed; marks.json diff; README/SERIES diff.
2. Read the plan's rules: u4-build-plan.md §5 U4j (:626-646), §4.6, §4.12, §2 rows (:147-154); advisor-u4.md
   conditions 2/4, nits c/f, Q20, Q22, :365-371; advisor-u4-wave1.md W1-N2; advisor-ws-docs-r2.md :373-383 (ws-practice).
3. Focus items, one block each, a notes line per finding:
   (1) C4 rules and order; mixed-build hiding a bites log behind an honest one (reading a).
   (2) hs format: writer and reader, 16 lowercase hex or null, sentinel, duplicate / out-of-order ticks, uneven lengths.
   (3) Outcome::Empty and reading (d).
   (4) ws-practice: f bound by tolerance? WS-BAR cap; second practice run without a ruling; can a log supply a ruling.
   (5) reading (e): aarch64 claim dodging a check.
   (6) hostile input on parse_bound, practice record, bound line, hs: panics, overflow, NaN/inf, duplicate keys,
       trailing data.
   (7) redact check and marks drift test unweakened.
4. Anti-cheat-style coverage table for this unit: input -> where the judge validates -> gap.
5. Verdict and findings to StructuredOutput.

## Progress
- Read: git diff (11 files + 22 untracked), log/src/lib.rs, judge marks.rs, eval.rs, figures.rs, practice.rs, ratio.rs;
  u4j-build.md; u4j-brief.md; u4-build-plan §5 U4j, §2 C4 row, §4.6, §4.12; advisor-u2 C4 (:101-103);
  advisor-ws-docs-r2 :375-378, R7 :397, R8 :398; ADR 0001 -18 row :1608; marks.json -18, -21, -30; README diff;
  SERIES rows :266, :294 (head) and the base rows; redact-check.mjs header; marks.json base-vs-head per field.
- (Process note: the F1 line below was first appended with a shell heredoc by mistake; this file is now rewritten whole
  with the Write tool, content unchanged.)

## Findings
- F1 HIGH practice.rs:384-391 (class_ok), :406 (f), :87-90 (band check in parse_bound). FAR BETTER caps nothing.
  class_ok "FAR BETTER" needs only a non-empty chance_ruling and s < band_low; parse_bound accepts any band_low in
  (0, band_high] (no band_low <= 1 <= band_high), then f = max(1, s). Exploit: bound line band_low "2.0" / band_high
  "2.0", class FAR BETTER, chance_ruling "x", M_p = 1.9 x M_m -> f = 1.9: X, Y, Z move 90 % (X 0.67 s -> 1.273 s),
  past Mustafa's 10 % and R8 (v) ("FAR BETTER: at the bound"). Fix: FAR BETTER forces f = 1; parse_bound refuses
  band_low > 1 or band_high < 1; refuse any f > tolerance before Ok. Proof: engine test from
  u4j_ws_practice_puts_the_practice_runs_numbers_in_force with that line and class: must be INVALID (today PASS).
  Before merge.
- F2 MEDIUM practice.rs:331-338, :383 (M_m read from the record's copy); README §2.5 (bound_file, bound_sha256 read by
  nothing); marks.json -18 holds no M_m, band, guard or commit. The judge pins X, Y, Z, k and the tolerance to
  marks.json but takes M_m, the band, guard_share, model and commit from the log's verbatim copy. Exploit: a record
  carrying a stale or edited bound line (M_m 600 ms for a true 654 ms) turns a practice run at true s = 0.98 (f should
  be 1) into s = 1.07 IN TOLERANCE -> f = 1.07: numbers looser than "a slowdown measured within the tolerance"; a band
  with a very low band_low skips FAR BETTER's investigation. Fix: marks.json -18 carries the bound line verbatim (a
  string; no hash dependency needed) and the rule requires the record's `bound` to equal it byte for byte (the
  provisional line now; fixes.rs's later). Proof: engine test with m_m_ns changed in the record only -> INVALID.
  Before merge (it fixes the marks.json and record format fixes.rs and U4r build on).
- F3 MEDIUM practice.rs:259-268 (candidates from `set` only, `p.serves(&mark.id)`), eval.rs:1176. "One practice run
  counts per build" is checked inside the counted run's set only, and only among practice runs that list -18.
  Exploit: practice P1 (same build) BEYOND in set r3-vs4; practice P2 IN TOLERANCE at f = 1.08 in set r4-vs2 with the
  counted run: the judge sees one candidate, needs no second_ruling, applies f = 1.08 ("nobody can shop for a better
  factor", R7). Same with a first practice run whose header does not list the mark. Fix: count candidates over every
  run the judge was given (all sets), role practice with the same binaries, listing the mark or not. Proof: fixture
  with the two practice runs in two sets -> INVALID without second_ruling. Before merge (wrong verdict).
- F4 MEDIUM figures.rs:215-219 (arm = the log's own machine.arch), :271-282 (every name read from every log),
  :320-331 (orphan with no replay entry = agreement). Base SERIES contract was "with arm_rule on L2, only those the
  x86-64 re-decode (process `replay`) also misses"; reading (e) widens it. Exploits: (i) one client log with arch
  aarch64 that also carries hs.replay_hash equal to the server's turns every mismatch into cross_arch -> PASS where
  FAIL; (ii) -58 (arm_rule false, L1 only) gets the Arm rule if a log says aarch64; (iii) an aarch64 orphan (client
  hash, no server hash: a wrong tick label) with no replay entry counts as cross-arch, not a mismatch; (iv) a run whose
  room is aarch64 too still gets the leniency. Fix: read server_hash only from process room logs, replay_hash only
  from process replay logs on x86_64 (gap otherwise), decoded_hash only from client processes; apply the Arm rule only
  where the mark has arm_rule, the room log is x86_64 and the client log aarch64; an aarch64 mismatch is cross-arch
  only when the replay has an entry equal to the server's Some(hash) (absent entry: gap). Proof: fixture
  pm-0001-17/differ with the client log's arch aarch64 and an hs.replay_hash list in the same log -> expected FAIL,
  today PASS with "aarch64 only" 1. Before merge (wrong verdict; figure input rule later units build on).
- F5 MEDIUM figures.rs:195-209 (twin_hash_mismatch_count: twin_of with profile, seed, bite; any role). The twin's
  identity is not checked: SERIES :294 says "-21's twins are role twin, -30's each other's measured runs", the judge
  enforces neither. Exploit: two forged measured runs, same seed, naming each other in params.twin: both carry the same
  forged stream, so equal hashes hold even if a forged command leaked -> "forged: 0 effect" PASS where FAIL. -30: a
  WebTransport run naming another WebTransport replay of the same recording -> PASS without testing the other
  transport. Fix: the twin must be role twin, or the two runs name each other and differ in params.transport (the
  -30 form); gap otherwise. Proof: fixture pm-0001-21/twin-is-forged (two measured forged runs naming each other,
  equal h) -> gap/INVALID, today PASS. Before merge (wrong verdict).
- F6 MEDIUM practice.rs:56-68, :87-90, :387 with ratio.rs:25-28, :135-141. parse_bound panics on log data: two
  30-digit decimals (band_low "0.12345678901234567890123456789", band_high "0.99999999999999999999999999999") reach
  `band_low > band_high`, whose cross product (~1e57) overflows i128 and hits checked()'s expect: the judge crashes
  instead of INVALID. Same for `s >= band_low` with a long band_low. Fix: README §2.5 caps each fraction at 6 decimal
  places and parse_bound refuses more (or comparisons return Err). Proof: unit test in practice::tests with that line
  -> Err (today a panic). Before merge (format fixed now, nit f).
- F7 LOW eval.rs:549-576 (C4, reading (a)). Measured direction is sound: any bites log in a run naming no bite is
  INVALID, so no bites component hides behind an honest log. Bite direction is weaker: "at least one log" lets a bite
  run whose bite-carrying process (RS's room) ran the measured build pass C4 when only LG's bots are bites-built; such
  a run is usually Missed (mark FAIL, safe) and only a chance failure would show the bite. Accept reading (a) as a
  recorded risk; tighten later by naming each bite's process in marks.json. Follow-up.
- F8 MEDIUM figures.rs:298-343 (compared counts any non-null entry, every phase); marks.json -17 (compared and
  snap_incomplete only reported). No coverage floor: a client log with one matching hash and null everywhere else
  (or hashes only in warmup) gives compared 1, mismatches 0 -> PASS; nulls are never reconciled with
  tick.s.snap_incomplete or L1's fate log, so a refused group written as "lost" (null) hides. Fix (beyond the approved
  plan: the advisor's call): on P0 parts every server (slot, tick) is compared; elsewhere compared + incomplete =
  server pairs in the measured phase, as a check. Proof: fixture pm-0001-17/sparse (one entry, rest null) -> today
  PASS. Advisor's ruling, not a builder fix.
- F9 MEDIUM practice.rs:300 (second_ruling), :385 (chance_ruling), :144-155 (describe). Both rulings are any non-empty
  text in the log; the judge cannot tell the advisor's ruling from "x", and the verdict prints neither the class nor
  the ruling, so a run judged at f = s under a BEYOND + chance record looks like IN TOLERANCE (R8 (v): "the ruling is
  in the run's report for Mustafa"). Fix now: describe() prints the class and the ruling texts; later (advisor): rulings
  named by id in a committed file. Proof: engine test asserts the needs line contains the chance_ruling text. Before
  merge (cheap).
- F10 LOW log/src/lib.rs:470-471 (pre-existing). read_log parses each line into serde_json::Value first, so a
  duplicate key (two "h", two "hs", two "class" or "bound" in an event's f) is silently last-wins: the judge reads one
  value, a grep or a person another. Fix: a duplicate-key refusing pass in read_log. Follow-up.
- F11 LOW marks.rs:587-621. A ws_bound check's op and stat are not pinned (x: p50 with <=; y: count, where op >, op
  <=; z: max with <=): a reviewed marks.json is trusted, but a ">=" op would turn f into a tightening and a "max" on x
  would break the IN TOLERANCE recheck's assumptions. Fix: compile_check refuses any other shape. Follow-up.
- F12 LOW marks.json -18 parts "L2 B1 wt"/"L2 B1 ws" select on header params.transport alone. A WebTransport run
  labelled ws (with a practice run labelled ws) is judged at the WebSocket numbers. Fix: a setup check on the ws part
  that the WsServer-only series exist (event.burst_return.budget_bytes count >= 480). Follow-up (advisor).

## Checked and found sound
- hs: writer (log/src/lib.rs:368-369) and reader (:492) refuse anything but 16 lowercase hex digits or null; numbers
  and upper case refused (tests :706-729). Ticks strictly increasing per log (reader :493-499); across logs a second
  non-null (slot, tick) is a gap (figures.rs:239-246); uneven list lengths per name and across names are gaps
  (:229-236, :284-290); server 0 is a gap, client 0 always mismatches (:291-298). Empty lists give 0 compared -> empty.
- Empty (reading (d)): fails() counts it as a failure everywhere a Fail counted (eval.rs judge_run, eval_part,
  fail_lines); setup and pre treat it as INVALID; a bite run with only empty failures is INVALID, never shown
  (eval.rs:1459-1472). Agree with reading (d): it can only lower a verdict.
- ws-practice f: IN TOLERANCE and BEYOND need s <= tolerance; the tolerance is parsed in [1, 1.10] (marks.rs) and the
  bound's must equal it (practice.rs:333); X, Y, Z, k are pinned to marks.json (:339-371); M_p recomputed from the
  practice run's heals (:372-382); moved numbers recomputed (:414-428); C4 and the always rules hold on the practice
  run (:307-314). Only F1 breaks the cap; F2, F3, F9 loosen within it.
- parse_bound: exact tag, deny_unknown_fields, serde derive refuses duplicate fields, from_str refuses trailing data,
  i64/u64 types refuse floats, NaN/inf and overflow; Ratio::parse refuses exponents, '+', spaces. Only F6 panics.
- Drift test: marks.json's top level and every mark's draft row unchanged (base vs head per field); only -17, -18,
  -20, -21, -22, -58's parts, checks, bites, report, notes changed; -17/-20/-21 tightened (scene, clients, pre).
- Redact: check.mjs and redact-check untouched; new fixtures use 203.0.113.0/24 (allow-listed documentation range);
  machine_named needs the VM address, consistent with "redact all but the run's VMs".

## Coverage table (input -> where the judge validates -> gap)
| Input (log or record) | Validated at | Gap |
|---|---|---|
| hs entries | log/src/lib.rs check_hs (writer, reader) | none |
| hs source (which log gives which name) | nowhere | F4 |
| client arch for the Arm rule | figures.rs:219 (log's own claim) | F4 |
| decoded coverage (nulls) | nowhere | F8 |
| run's build vs bite (C4) | eval.rs:549-576 | F7 (bite direction) |
| twin identity | figures.rs twin_of (profile, seed, bite, always rules) | F5 |
| bound line numbers X, Y, Z, k, tolerance | practice.rs:333-371 vs marks.json | none |
| bound line M_m, band, guard, commit | parse_bound shape only | F1, F2, F6 |
| practice class | practice.rs:384-405 | F1 |
| chance / second ruling | non-empty text only | F9 |
| one practice run per build | within the set, listed runs only | F3 |
| transport label | none (select only) | F12 |
| duplicate JSON keys | none (last wins) | F10 |

## Verdict
REVISE: F1 (High) and the before-merge Mediums F2, F3, F4, F5, F6, F9. F7, F10, F11, F12 follow-ups; F8 to the advisor.
DONE.

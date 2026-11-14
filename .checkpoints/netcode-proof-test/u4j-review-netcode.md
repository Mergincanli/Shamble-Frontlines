# U4j review, round 1: netcode-architect (read only)

Worktree /home/user/Shamble-Frontlines/.claude/worktrees/agent-a3b8dfeb01ed52b50, base 411eeeed50d8c71e7e1f74366f0e1d5c27388444.
Started 2026-11-11. No memory found; no earlier notes file.

## Plan
1. Read the sources: u4-build-plan.md §4.6, §4.12, §5 U4j (:626-646), §2 rows (:147-154); advisor-u4.md conditions
   2/4, nits c/f, Q20, Q22, U4S-BUILDER; advisor-u4-wave1.md W1-N2; plan.md PM rows for -17, -18, -20, -21, -22, -58;
   advisor-ws-docs-r2.md :373-383 (bf9d248's -18 numbers, ws-practice); .claude/rules/netcode.md; base README/SERIES.
2. Read the diff: log/src/lib.rs (hs), judge figures.rs (five figures), eval.rs (Empty, C4, ws_bound), marks.rs,
   practice.rs, marks.json changes, README/SERIES deltas, fixtures.
3. Check focus (1): the five figures against §4.6 and SERIES (pairing, complete ticks, sentinel, null, orphan, empty,
   server 0, twin per tick, cross-arch hash kind, machine_named).
4. Check focus (2): marks.json changes against PM rows and §4.12 (selects, -18 split, ws_bound numbers, WS-BAR cap,
   -22 exception, -58 W1-N2 text); no bar loosened.
5. Check focus (3): readings (a), (b), (d), (e), (f).
6. Check focus (4): writer rules for U4r and U4c L-c8 re-judge: complete and unambiguous; any rule a runner can meet
   while the figure misjudges.
7. Findings, one line each below as found; then the verdict.

## Findings (one line each, as found)
- Sources read: u4-build-plan.md §2 (:141-160), §4.6 (:296-318), §4.12 (:396-412), U4j (:626-645), U4r (:717-772),
  L-c8 (:574-576), Q11/Q20/Q22 (§10); advisor-u4.md verdict + conditions 1-13; advisor-ws-docs-r2.md R3-R11 and the
  -18 spec (:374-379); advisor-u4-wave1.md W1-N2 (:305-351) and NETEM-BAR (:534-547, :607-620); history.md:292-296
  (Mustafa's NETEM-BAR (a) words); plan.md:359 (Arm rule: "LG's aarch64 clients"); bar-fix1-bf9d248.txt (X 0.67 s,
  5 over 2.50 s, Z 6.90 s: matches marks.json).
- Diff read: log/src/lib.rs, judge figures.rs, eval.rs, marks.rs, practice.rs (whole), marks.json (-17, -18, -20, -21,
  -22, -58), README.md, SERIES.md; fixtures pm-0001-17/*; engine.rs decoded/twin tests.
- F1 (Medium, before merge) practice.rs:406 f = max(1, s) also for FAR BETTER; parse_bound (:87-90) never checks
  band_low <= 1 <= band_high, and nothing checks f <= tolerance. A bound line with band_low > tolerance (e.g. 1.2) plus a
  chance ruling on FAR BETTER at s = 1.15 gives f = 1.15 > 1.10: the only path where the judge moves numbers past
  Mustafa's cap. R8 (v) says FAR BETTER runs "at the bound" (f = 1). Fix: f = 1 for FAR BETTER, band_low <= 1 <=
  band_high in parse_bound, assert f <= tolerance. Proof: a unit test with that line.
- F2 (Medium, before merge) practice.rs:259-286,325: the latest practice run of the build is taken over ALL candidates,
  then refused if it started after the counted run. A practice run added to the set after the counted run (R8's
  investigation, same build) turns that counted run's FAIL into INVALID (or "2 practice runs" INVALID): R9 says a failed
  counted run stands as FAIL; a gap/invalid must never hide a failure (C2). Fix: only practice runs started before the
  run count (and the one-per-build rule over those). Proof: engine test, counted run FAILs, add a later practice.
- F3 (Medium, before merge, README text) README §1 hs "index = slot" never says whose slot: no admit field carries it,
  and hs is the first list paired across two logs (room vs client). U4r's runner could index the client log by spawn
  ordinal and the room by admission order: every pair mismatches (a correct build FAILs), or -19's slot reuse misaligns.
  Fix: one sentence: index = the room's slot for that client (the one the room's tick.s lists use; a client takes it from
  JoinAccepted), same in every log. Proof: a fixture with permuted client slots must FAIL (shows the rule matters).
- (process note: F1-F3 were appended with a shell heredoc by mistake; the Edit tool is not available here, so this
  file is rewritten whole with the Write tool from now on, F1-F3 kept word for word.)
- F4 (Medium, before merge) figures.rs:195-209 twin_hash_mismatch_count calls twin_of with no role check ("any role",
  for -30, which is NOT-RUN until U6 and whose pre-check is never judged). For -21 a forged run that names another
  forged run of the same seed/profile/bite (a re-run, a duplicate) compares forged with forged: equal hashes, PASS of
  "0 effect beyond the valid part" without ever meeting the valid-parts twin. README §1 and -21's notes say role twin;
  fate_log_twin_equal keeps the check (:159). Fix: require role twin now; -30's form at U6. Proof: engine case, twin
  role measured -> INVALID.
- F5 (Medium, before merge, README text; the log format U4r writes) README:43-56's hs writer rules leave gaps a runner
  can meet while the figure misjudges: (i) nothing says every completed tick carries its hash (hs is optional per
  record): a runner that samples every 10th tick passes -17's "100 % of decoded snapshots" on a 10 % sample; (ii) the
  figure reads every phase (reading f), so the room must log server_hash from each client's admission in every phase
  the clients log, else warm-up orphans FAIL a correct build; (iii) "two hashes for one tick writes the sentinel"
  (:54) should say two different hashes (an L2 duplicate datagram re-completing a tick would FAIL a correct build);
  (iv) a hash that arrives for a tick already written must never be dropped (buffer the whole run, or end `crash`).
- F6 (Low, follow-up; reading e) figures.rs:219 the Arm rule keys on machine.arch == aarch64 alone; plan.md:359 says
  "LG's aarch64 clients". A non-LG aarch64 client (ML if it is an ARM laptop: -19's ML Chrome part) would come out
  NOT-RUN ("no x86-64 re-decode") instead of FAIL on a real mismatch; an arm_rule=false mark (-58) would excuse one if a
  run ever ran on an aarch64 host. -58 is C-only today, so no live exposure. Fix: key on machine.role LG and aarch64
  (or pass the mark's arm_rule). Owner: before -19's ML part (U7).
- F7 (Medium, follow-up acceptable, fold in if cheap) practice.rs:372-376 the practice run's heal count is never checked
  against the bound's client_bursts_count (parsed at :23, unused). R7: "Valid only with 480 heal records". A practice
  run missing records gives M_p from a subset, so s and f move (either way, inside the cap) and a broken setup can
  pass as IN TOLERANCE (its count/max re-check is easier with fewer records). Fix: heal.len() == client_bursts_count.
  Proof: ws case with 479 heals -> INVALID.
- F8 (Low, follow-up) figures.rs:282 the replay log's arch is never checked (slot_hashes reads replay_hash from any
  log); the Arm rule is "replayed on x86-64". An aarch64 replay agreeing with the server would excuse a live-path
  mismatch as cross-arch. Fix: replay_hash only from x86_64 logs, else a gap.
- F9 (Low, follow-up, the fixes.rs unit) the record's bound line is tied to marks.json only by X, Y, Z, k and the
  tolerance; M_m and the band come from the line unchecked (bound_file/bound_sha256 are not read), so a stale bound
  file's M_m moves f anywhere in [1, 1.10]. Pin the bound's sha256 (or M_m) in marks.json when the real bound lands.
- F10 (Low, follow-up, for the advisor) -17: a client that logs null for a refused group (or for a group whose header,
  hence tick, it cannot read) hides the refusal; on L1 P0 (Profile P0 = 0 loss, l1/src/lib.rs:116) the incomplete
  count must be 0, so a P0-part check `tick.s.snap_incomplete sum == 0` would close it. Stricter, not in the plan.
- F11 (Low) twin hash: "a tick hashed in one only counts" (figures.rs:192) needs the writer rule that a run and its
  twin hash the same tick range; README does not say it (a twin stopped one tick early FAILs a correct build).
- F12 (Low) cross_arch_mismatch_count: U4j's Builds asks for fixtures ("with fixtures only since no U4 run feeds
  them"); only engine tests exist (tests/fixtures.rs has no aarch64 or replay case). Test-engineer's call.
- F13 (Low, U6 forward) C4's role list (eval.rs:555: bite, twin) will refuse PM-0006-20's logging-off twin of its
  planted-format bite run, while twin_of demands the same bite: U6's alloc_twin_diff must settle it.
- F14 (Low) -18's reports lack the practice outcome's band and tolerance (the ruling's list: M_p, M_m, s, the band, f,
  the tolerance); describe() (practice.rs:144-155) prints four of six. SERIES's header.params.started_utc row (:306)
  lacks -18 in its Marks column.
- Focus (2) checked: -17 L1 P0/P2/P3/B1 gain SS, 48, pre (S30 and L2 parts unchanged); -18 three parts with exactly
  the ruling's selects, checks (a)-(e), bf9d248's 670000000 / 5 over 2500000000 / 6900000000, "provisional" in every
  name and the notes, ws-practice:1.10 capped in marks.rs and parse_bound; -20 two L1 parts; -21 eight; -22 scene and
  clients only, README §4 exception written; -58 W1-N2 words verbatim. No bar loosened by the marks text.
- Readings: (a) right (L2's bite lives in RS's binary; "at least one" fails toward Missed, never a false show); (b)
  right (R11: the bite is judged at the counted run's numbers; its own practice would be BEYOND); (d) right (like a
  setup failure: the run did not test the bite; INVALID never passes); (e) see F6; (f) right (-58's notes say window
  all; stricter for -17) but it makes F5 (ii) a writer rule. None needs the advisor except F10 (a new check).
- Focus (1) holds otherwise: pairing by (slot, tick) over every non-null client entry; sentinel counts; null skipped;
  orphan counts; 0 compared -> Empty (fails, no bite shown); server 0 -> gap; twin per tick incl. one-sided ticks;
  cross_arch = aarch64 mismatches the replay does not reproduce (both absent = agreement, consistent with the rule);
  machine_named covers every Machine field (log/src/lib.rs:24-42).

## Verdict
REVISE (round 1): F1, F2, F3, F4 and F5 before merge (each cheap: guards in practice.rs, a role check, README text);
F7 recommended with them; F6, F8-F14 follow-ups. DONE.

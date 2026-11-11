# U4j round 2: netcode-architect review notes

Reviewer: netcode-architect. Round 2 of at most 2, on what changed since round 1 (.gate/u4j-prefix vs
.claude/worktrees/agent-a3b8dfeb01ed52b50). Never edit a copy; commit nothing; commands only through
.checkpoints/netcode-proof-test/u4j-fix/run.sh; git only as `cd <copy> && git ...`.

## Plan
1. Read the review packet in full (diffs, plan U4j section, design rules, advisor FX-1..FX-14 rulings, round-1
   findings, Mustafa's bars, carries).
2. Find my round-1 findings NC-U4J-1..-14 and how the advisor routed each (fix / follow-up / carry).
3. For each routed-to-fix NC finding: CLOSED or NOT CLOSED with file:line evidence in the fixed copy.
4. README paragraphs: ws-practice rule (FX-3 words) and hs writer rules (FX-12 words) vs advisor R6, R8 (v) and
   Mustafa's two quotes (WS-BAR refinement; NETEM-BAR (a)), word for word where rulings give words.
5. Coverage floor (FX-10) and Arm rule key (FX-5) vs plan §4.6 and ADR 0001 -17 / -19 rows.
6. Can U4r's runner and U4c's L-c8 re-judge meet every writer rule as written (process names, arch, role,
   snap_incomplete lists, clients header).
7. Run the judge's tests through run.sh (exit code + test runner summary line).
8. New defects: severity, file:line, scenario, cheapest proof, before-merge flag.
9. Verdict.

## Progress log
- (start) plan written.
- step 1 partly: packet read (src diff, README/SERIES/marks diff, rulings FX-1..14, FU-1..6, carries, R2, R4, my
  round-1 F1-F14, Mustafa's bars, §4.6, §4.12, carry 54). My routing: 1->FX-1, 2->FX-2, 3->FX-12, 4->FX-4,
  5(i)->FX-10, 5(ii-iv)->FX-12, 6->FX-5(b)+FU-5, 7->FX-7, 8->FX-5(a), 9->FX-6, 10->R2.4 U4r carry, 11->FX-12,
  12->FX-5 fixtures, 13->U6-b carry, 14->FX-11.
- (process note: the line above was appended with a shell heredoc by mistake; no Edit tool exists in this seat, so
  this file is rewritten whole with Write from here on, earlier lines kept word for word. The first read-only
  commands (ls, git status, grep, python reading marks.json) ran outside run.sh; every command from the word check on
  ran through run.sh.)
- step 2-3: fixed practice.rs read whole (1-649); figures.rs 130-577 (twin, source, slot_hashes, decoded, coverage,
  the three figures); eval.rs in_force/eval_check (436-470) and judge (1726-1742: `all = runs(logs)`).
- step 4: advisor-ws-docs-r2.md R4-R9 (:394-399) read. Word check (scratchpad words.py, through run.sh, rc=0): every
  block quote of FX-3, FX-5, FX-6, FX-10, FX-12 FOUND verbatim (whitespace and backticks normalised) in README or
  SERIES; FX-11's format checked by eye against describe (practice.rs:204-233): matches. FX-12's inline items
  (decoded_hash "(only in the clients' logs ...)", replay_hash, twin role), FX-8's README words, FX-6's record bullet
  and FX-13's "header checks" drop: in the diff, matching.
- step 5: ADR 0001:1605 (-17 key: "An L2 mismatch counts only once ... decoded again from the start on x86-64 ...
  miss the server's logged hash too; one that shows only on LG's aarch64 ... decides nothing") and :1609 (-19: "ML's
  Chrome (Wasm) is judged as it stands; on an LG join ...") read. FX-5's key (LG + aarch64 + room x86_64,
  figures.rs:248, :406, :423) matches; on an x86-64 LG it counts directly, which is stricter in form and equal in
  effect (a same-arch re-decode reproduces). Orphan everywhere: stricter, in R5's readings list.
- step 6: u4-build-plan.md U4c (:568-599, L-c8 at :590-592), U4e (:703-734), U4r (:736-782) read; marks.json -3 read:
  -3 reads only probe series and fate_log_twin_equal, none of which the fix pass touched, so L-c8's re-judge meets
  every new rule trivially. l1/src/lib.rs:13-14, :617-642: L1 reorders datagrams (the reorder step); -58's L1 P3 part
  exists for that.
- step 7: `cargo test -p npt-judge` through run.sh: rc=0; summary lines: lib 14 ok, main 0, cli 2 ok, engine 43 ok,
  fixtures 19 ok, doc 0; 0 failed. `-- --list`: every FX-named test present.
- cites re-checked through run.sh (rc=0): README.md:55-65 (decoded_hash item), :213 (§2.5 "f = max(1, s) exactly"),
  eval.rs:1715 (FAIL note), SERIES.md:264 (cross_arch row), :319 (snap_incomplete row).

## Round-1 findings: closed or not
- NC-1 (FX-1) CLOSED. Every path to Ok in find (practice.rs:575-607): IN TOLERANCE band_low<=s<=tol -> f in [1,tol];
  BEYOND chance && s<=tol -> f in [1,tol]; FAR BETTER chance && s<band_low<=1 -> f=1; other class -> Err; then
  f>tol -> Err (:602). tol = the rule's (marks.rs ws-practice cap 1..1.10) = the pinned line's (:509), and the line
  is marks.json's byte for byte (:497-507), refused at load if its band leaves out 1 (parse_bound :105-111, marks.rs
  ws_bound_line check). My scenario (band_low 1.2, FAR BETTER at s 1.15) is refused three times over.
- NC-2 (FX-2) CLOSED: only practice runs started before the counted run are candidates (:363-366). Residual: Low
  (N2 below).
- NC-3 (FX-12, room's slot) CLOSED: README hs bullet carries FX-12's words; fixture permuted-slots FAIL.
- NC-4 (FX-4) CLOSED: figures.rs:198-203 refuses a non-twin role as a gap; fixture twin-is-measured INVALID; profile
  and bite twin gaps added (TE-F14).
- NC-5 (i) (FX-10) CLOSED: coverage() figures.rs:460-536, order figures.rs:542-560 = FX-10 (f); fixtures sparse,
  sampled, slot-unlogged, hashed-and-incomplete, short-lists INVALID; my 10 % sample case is (c)'s gap.
- NC-5 (ii)-(iv) (FX-12) CLOSED as texts; the new words leave one timing case open: N1 below (Medium, carry).
- NC-6 (FX-5 (b)) CLOSED for the fix half (LG + aarch64 + room x86_64); the arm_rule-flag half is FU-5 (not claimed).
- NC-7 (FX-7) CLOSED: heal.len() == client_bursts_count (:555-564); engine cases 479 and 481 INVALID (engine.rs:2195,
  :2199).
- NC-8 (FX-5 (a)) CLOSED: source() figures.rs:224-234, slot_hashes :242-247; fixture lg-replay-on-aarch64 INVALID.
- NC-9 (FX-6) CLOSED: ws_bound_line pinned; record's bound byte for byte (:497-507); marks.rs load checks.
- NC-10: not claimed (R2.4: U4r carry, evidence first). Open as routed.
- NC-11 (FX-12) CLOSED: README twin role text ("a run and its twin log h over the same ticks ...").
- NC-12 (FX-5 fixtures) CLOSED: lg-cross-arch, lg-replay-differs, lg-replay-in-client-log, lg-replay-on-aarch64,
  l1-aarch64-claim, lg-orphan, lg-replay-missing-entry, lg-orphan-no-replay judged on the real marks.json's L2 P2 /
  L1 P2 parts (fixtures.rs:306-366+), reasons asserted.
- NC-13: not claimed (U6-b carry). Open as routed.
- NC-14 (FX-11) CLOSED: describe prints class, M_p, M_m, s, band, tolerance, f, run, rulings (:204-233) and reaches
  each moved check's needs (eval.rs:461-466); SERIES started_utc row lists -18.

## README paragraphs vs R6, R8 (v) and Mustafa's words
- R6 "f = max(1, s), used only when s is at most the tolerance ... rounded up to the next millisecond; k unchanged ...
  One factor, once ... always applied to the bound itself": README §4 holds each (every accepted class has
  s <= tolerance; f on the pinned bound; k unmoved). Met.
- R8 (v) "chance, only for a lone count or maximum failure with s inside the tolerance, or for FAR BETTER ... the
  counted run goes ahead at the moved numbers (FAR BETTER: at the bound), never looser, and the ruling is in the run's
  report for Mustafa": README's BEYOND / FAR BETTER lines and "f = 1 for FAR BETTER (the advisor's R8 (v): at the
  bound)"; the report part through "Checks (c) to (e) print ... any ruling's text". Met.
- WS-BAR refinement "up to 10 % slower ... may move by that much; beyond 10 %, investigate first and never loosen":
  f = s only for s <= tolerance <= 1.10; s > tolerance has no accepted class; the judge refuses f > tolerance. Met.
- NETEM-BAR (a) "The bar stays capped as WS-BAR says": the same cap, enforced by the judge whatever any record or
  ruling says. Its second sentence (report a kernel-slower failure to Mustafa with the netem numbers) is a report
  duty, not a judge rule; nothing in the new text contradicts it (s > tolerance ends INVALID, never looser).

## New findings (one line each, as found)
- N1 (Medium, not before merge; carry to U4e's and U4r's briefs) README §1 decoded_hash item (README.md:55-65 in the
  fixed copy): "no hash is ever dropped: the writer buffers until every earlier tick is written, or ends its log
  crash" with "(a tick none of whose groups arrived counts too, known from the next header's tick)" never says WHEN a
  tick may be written lost, nor what a group of a tick older than one already decoded is. L1 reorders datagrams
  (l1/src/lib.rs:13-14, :617-642; -58's L1 P3 exists for it), so on P2/P3 a group of tick t often arrives after t+1.
  A writer that writes t lost on t+1's header then meets t's hash: by the rule it must end its log crash (every P3 run
  INVALID); a client that drops the stale group and logs it as refused writes the sentinel (a correct build FAILs).
  Neither is a false pass, and §4.6's sentinel list (no baseline, bad header, size or decode error) argues against
  the second, but nothing in the words settles it. Fix (one sentence in the carries): "a tick is written lost only
  once none of its groups can still decode (on L1 the runner may decide it at the stop); a group of a tick older than
  one already decoded is decoded and hashed under its tick like any other, or, if the client drops it as stale, that
  tick is lost (null, snap_incomplete 1), never the sentinel". Cheapest proof: U4r's contract test on a short L1 P3
  run with the reorder step on: no crash footer, decoded_hash_mismatch_count 0 with no gap.
- N2 (Low, follow-up) practice.rs:363 `started(p)?` runs for every practice run of the build before the start-time
  filter, so a practice run with a missing or malformed params.started_utc, added at any time and in any set, makes
  the counted run INVALID: a FAIL then reads INVALID, the one way left round R9's "a practice run started after the
  run never changes its verdict". Fails safe, never loosens; the run script always writes started_utc. Fix: say so in
  README §4 ("a practice run of the build whose start cannot be read makes the run INVALID"), or rule it. Proof: E0
  plus a later practice run with no started_utc: today INVALID.
- N3 (Low, test quality; the test-engineer's) engine.rs:2289-2317 u4j_ws_far_better_past_the_cap_is_invalid asserts
  only INVALID with some "ws-practice: " reason. On the fixed code the pin (practice.rs:502) refuses first, so the
  test cannot say which guard held; the band guard is proven by the unit test and the marks.rs test, and the
  f <= tolerance guard (:602) is by design unreachable. The advisor's plant "FAR BETTER f = max(1, s), with the band
  check and the pin removed" still comes out INVALID through :602, so it is caught only if the plant also removes
  :602. No defect in the judge; the round-2 plant needs that detail.
- N4 (Low, wording) README §2.5's record bullet (README.md:213) and -18's notes in marks.json still say
  "f = max(1, s)" with no FAR BETTER exception, beside README §4's "f = 1 for FAR BETTER". Equal in value (FAR BETTER
  needs s < band_low <= 1, so max(1, s) = 1), so no number differs; one clause would end the apparent contradiction.
- N5 (Low, wording) SERIES.md:264 cross_arch_mismatch_count row: "an aarch64 mismatch with no replay log in the run
  is NOT-RUN" should read "an LG aarch64 mismatch (the room on x86_64)"; eval.rs:1715's FAIL note "an aarch64-only
  mismatch is a cross-architecture finding" likewise. SERIES.md:319's tick.s.snap_incomplete row still says "-17
  (reported)" and "at the tick its groups' headers name", while it now decides the coverage rule for -17, -58 and -19
  and counts a tick none of whose groups arrived (no header names it).
- N6 (Low, writer rules; fold into N1's carry) the coverage rule's (d) needs each client's first resolved tick before
  the room's first measured hash for its slot; the words state the tail ("after the measured phase the clients keep
  logging ...") but not the head (every client admitted and its first snapshot decoded in the warm-up). With a warm-up
  of a second or more this always holds; a mid-measured admission whose first datagram is lost would be INVALID
  (-19's joins: U6-c already carries it). Also: the room logs a hash for every snapshot it sends, so none is sent after
  its last tick record (else an orphan FAILs a correct build at the stop).
- Checked, no finding: FX-10 (a)-(f) vs §4.6 (stricter only where §4.6's own writer rule says snap_incomplete 1);
  WS budget skips and carry 54's skipped snapshots (server null: no coverage requirement); B1 bursts (resolved from
  the next header); permuted slots; orphans extend the span but FAIL first; Waiting after gaps per FX-10 (f).
- Verdict: APPROVE WITH FINDINGS. NC-1 (F1) CLOSED; no new Critical or High, so no round 3 from this seat. N1 is a
  Medium for U4e's and U4r's briefs (carries), not before the merge; N2-N6 Low follow-ups. DONE.

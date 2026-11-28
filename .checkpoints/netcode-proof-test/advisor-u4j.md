# Advisor: U4j (judge and marks change), round 1 rulings for the one fix pass

Date: 2026-11-11. Seat: day session, advisor agent (rules read from origin/main:.claude/agents/advisor.md).
Scope: U4j in worktree /home/user/Shamble-Frontlines/.claude/worktrees/agent-a3b8dfeb01ed52b50, base 411eeeed50d8c71e7e1f74366f0e1d5c27388444.
I rule; I edit no code, build nothing, run nothing, commit nothing.

## Plan
1. Read the main checkpoint's resume card, u4j-brief.md, u4j-build.md (readings (a)-(f)).
2. Read u4-build-plan.md §5 U4j, §4.6, §4.12; advisor-u4.md (conditions 2, 4; nits c, f; Q20, Q22, U4S-BUILDER);
   advisor-u4-wave1.md (W1-N2); advisor-ws-docs-r2.md (:375-378, R7-R11); history.md "Decisions taken" (WS-BAR, its
   refinement, NETEM-BAR (a)).
3. Read the diff in the worktree and the untracked files.
4. Read the three reviews whole (netcode, security, test) and the test probe outputs.
5. Write rulings: R1 the fix list (dedup, fix choice, proof by name); R2 the calls; R3 readings (a)-(f); R4 severity;
   R5 Decisions needed; then "What I checked, and what I did not".

## Progress
- [x] step 1 (the readings (a)-(f) are not in u4j-build.md; they are in the builder's handback, read from its
  transcript agent-a3b8dfeb01ed52b50.jsonl, "Readings for the advisor or reviewers", quoted in R3 below)
- [x] step 2 (plan §2, §4.6, §4.12, §5 U4j, Q11/Q20/Q22; advisor-u4.md conditions 1-13; W1-N2; ws-docs-r2 :375-400
  R3-R12; history.md :292-294 NETEM-BAR (a), :381-386 WS-BAR refinement, :387-391 WS-BAR; plan.md:359 the Arm rule)
- [x] step 3 (diff read: practice.rs whole; figures.rs :100-360; eval.rs :1-80, :420-470, :535-600, :980-1010,
  :1161-1200, :1386-1446, :1614-1650, :1700-1730; marks.rs :575-630; README diff; marks.json -17, -18, -21 parsed;
  fixtures pm-0001-17/base headers; the test-engineer's probe-run1/2 and plant-t outputs)
- [x] step 4 (three reviews read whole)  - [x] step 5 (rulings below, block by block). DONE.

## Verdict on round 1
**REVISE, as the three reviewers say.** One fix pass, by a fixer agent in U4j's worktree, closes FX-1 to FX-14 below
before the merge. Then round 2 runs on what changed (netcode-architect, security-auditor, test-engineer). F1 stays
High, so a round 3 is open to it alone if round 2 finds it still open. Nothing needs Mustafa now.

Ids: NC-n = u4j-review-netcode.md's Fn; SEC-Fn = u4j-review-security.md; TE-Fn and TE-Rn = u4j-review-test.md. Paths
are under spikes/netcode-proof/ in the worktree. "Fails today" means: the fixer writes the test or fixture first, runs
it on the unfixed code, and records its failure by name (the failing assert line) in its notes before the fix.

## R1. The fix pass's list (before the merge)

**How the fixer works** (u4j-brief.md's rules, unchanged): shared target and offline; touch before each build; files
only crates/judge, crates/log and marks/; no manifest, Cargo.lock or check.mjs edit; the Edit and Write tools; plants
in place, one at a time, restored and sha256-checked; commit nothing. At the end: check.mjs all 10 steps twice in a
row (counts as deltas on 304 / 327 / 48), `npt-judge --check-marks`, clippy -D warnings host and wasm32 in both
builds, `cargo +1.99.1 fmt --all --check`. Notes: u4j-fix.md (plan first, a line per item). It lists every new public
item by crate (A19) and every marks.json change as before and after. FX-1 first.

### FX-1 (High): FAR BETTER moves nothing, and no path moves the numbers past the tolerance
NC-1 = SEC-F1 = TE-F1. Probe A: band 2.0 to 2.0, s = 1.9 gives X' = 1.273 s, and a 1.2 s median PASSES.
- **The fix** (all three, as the reviewers agree):
  - (a) `parse_bound` refuses a band that leaves out 1: band_low <= 1 <= band_high. M_m is the median of the run
    medians, so a correct build's 1st to 99th percentile band holds 1 by definition.
  - (b) FAR BETTER gives f = 1. My R8 (v) says "FAR BETTER: at the bound". IN TOLERANCE and BEYOND keep f = max(1, s).
  - (c) Before `Ok`, `find` refuses any f above the tolerance. After (a) and (b) this guard cannot be reached; it stays
    anyway.
- **README §4's ws-practice paragraph:** FX-3 gives the whole paragraph.
- **Proof:**
  - `practice::tests::the_bound_line_refuses_a_band_that_leaves_out_1`: band 2.0 to 2.0, 1.01 to 1.2, and 0.5 to 0.9
    are each refused. Fails today.
  - engine.rs `u4j_ws_far_better_past_the_cap_is_invalid`: probe A committed. The record's bound line has band 2.0 to
    2.0, class FAR BETTER with a chance ruling, s = 1.9: the ws part is INVALID. Today it PASSES at 1.273 s.

### FX-2: a practice run started after the counted run changes nothing
NC-2 = TE-F2 (probes E1, E2: a FAIL becomes INVALID). My R9: "a counted WebSocket run that fails stands as FAIL".
- **The fix:** the candidates are only the practice runs that started before the run. The one-per-build rule counts
  over those. A run with no such candidate is INVALID ("no practice run of its build started before it").
- **Proof:** engine.rs `u4j_ws_a_practice_run_after_the_counted_run_changes_nothing`. Probe E0 FAILs. Adding a later
  practice run, with a second_ruling (E1) or without one (E2), leaves it FAIL. Fails today.

### FX-3: one practice run per build, over every run the judge reads; a bite run at its counted run's numbers
SEC-F3 = TE-F3 (probe F). TE-F15 is folded in, because the bite rule has to change with it. Reading (b) changes (R3).
- **The fix:**
  - (a) `find` takes every run the judge reads (`all`), not the counted run's set. My R7's "the results set holds its
    practice run" means the results the judge reads.
  - (b) A practice run of the build counts toward the rule whatever its set, and whether or not it lists the mark.
    The latest one counts; if it does not list PM-0001-18, the run is INVALID.
  - (c) A bite run takes the practice run in force for the measured runs of its own set: those that serve the mark
    with the bite run's layer, profile and `params.transport`.
    - More than one build among those measured runs: INVALID. None: INVALID.
    - The practice run is then found for that build exactly as for a measured run (FX-2, FX-3 (a), (b)).
    - So the bite run is judged at the same f as its counted run. A smaller f would show the bite falsely; a larger
      one would hide it.
- **The text: README §4's `ws-practice:<tolerance>` bullet, from "the run is INVALID unless" to "between the practice
  run and the counted run.", becomes:**
  > the run is INVALID unless the runs the judge reads hold its practice run and that run's record (§2.5) puts the
  > numbers in force. The practice run: role `practice`, listing the mark, built from the same binaries
  > (`build.binary_sha256` over its logs), with the same `scene`, `profile`, `params.burst_spacing_ns`,
  > `params.ws_tcp_model` (equal to the bound's `model`), `params.config_hz` and RS's `machine.kernel` (each present in
  > both runs), started (`params.started_utc`) before the run, holding the four rules above, with exactly the bound's
  > `client_bursts_count` measured heal records and exactly one `ws_practice` record. A bite run takes the practice
  > run of its set's measured runs that serve the mark with its layer, profile and `params.transport` (one build among
  > them, else INVALID), so it is judged at its counted run's numbers. One practice run counts per build: of that
  > build's practice runs started before the run (in any set, listing the mark or not), the latest counts, and with
  > more than one only if its record has a `second_ruling`; a practice run started after the run never changes its
  > verdict. The record: its `bound` is the mark's `ws_bound_line` byte for byte; `m_p_ns` is the practice run's own
  > median heal time; with s = M_p / M_m:
  > - `IN TOLERANCE` needs band_low <= s <= tolerance, and the practice run's heals at most k over Y' and none over
  >   Z' (the moved numbers);
  > - `BEYOND` needs a `chance_ruling` and s <= tolerance (a lone count or maximum failure);
  > - `FAR BETTER` needs a `chance_ruling` and s < band_low;
  > - `INVALID` never counts.
  >
  > f = max(1, s) for IN TOLERANCE and BEYOND, and f = 1 for FAR BETTER (the advisor's R8 (v): at the bound). The
  > judge refuses any f above the tolerance. The record's moved numbers equal the judge's: the bound's times f,
  > rounded up to the next millisecond, with k unmoved. Checks (c) to (e) print the class, M_p, M_m, s, the band, f,
  > the tolerance, the practice run and any ruling's text beside the numbers in force. So no number is set by hand,
  > and nothing in marks.json changes between the practice run and the counted run.
- **Proof:**
  - engine.rs `u4j_ws_one_practice_run_per_build_over_every_set`: probe F committed. A second practice run of the
    build, class BEYOND, sits in another set, with no second_ruling: INVALID. Fails today. A third case: the earlier
    run does not list the mark, still INVALID.
  - engine.rs `u4j_ws_a_bite_run_takes_its_counted_runs_practice_run`, four cases:
    - (1) the bite run's checks print the same f as its set's measured run;
    - (2) the set's measured runs come from two builds: INVALID. Fails today, which is valid;
    - (3) TE-F15: the set holds practice runs of two builds and measured runs of one. The bite takes the measured
      build's practice run and is valid. Fails today, which is INVALID ("cannot tell whose numbers");
    - (4) a practice run of an older build in another set leaves the bite run valid.
    - (1) and (4) pass today. They guard FX-3 (c): with FX-3 (a) alone, (4) would come out INVALID.

### FX-4: the twin of -21's twin-hash checks must have role twin
NC-4 = SEC-F5 = TE-F4 (probe D: two forged measured runs that name each other PASS). TE-F14 rides it.
- **The fix:** `twin_hash_mismatch_count` refuses a twin whose role is not `twin` as a gap, the way
  `fate_log_twin_equal` does. Its doc comment drops "any role (-30's twins are each other's measured runs)". -30's
  form (each other's measured runs, differing in `params.transport`) is added by U6 with -30: carry U6-a (the Carries list).
- **Proof:**
  - fixture `pm-0001-21/twin-is-measured`: two forged measured runs naming each other, with equal `h`. Judged in
    tests/fixtures.rs `pm_0001_21_the_twin_must_be_role_twin`: INVALID, and the reason names the role. Fails today.
  - TE-F14: `u4j_the_twin_hash_compares_every_tick_and_its_twin_whole` gains a twin with another profile and a twin
    with another bite, each a gap.

### FX-5: each hash name from its own process, and the Arm rule's key
SEC-F4 = TE-F5 + TE-F11 + TE-F12 + NC-6 + NC-8 + NC-12. Probes C1, C2 and O; C3, the mark's own flag, is FU-5.
Reading (e) changes (R3).
- **The fix (my call: R2.7):**
  - (a) The sources:
    - `hs.server_hash` is read only from logs of process `room`;
    - `hs.decoded_hash` only from logs of process `client` (L1) or `bots` (L2);
    - `hs.replay_hash` only from logs of process `replay` with machine.arch `x86_64`.
    - A name found in any other log is a gap.
  - (b) The Arm rule applies to a client hash only when all three hold:
    - its log's `machine.arch` is `aarch64`;
    - its log's `machine.role` is `LG` (ADR 0001:1605, -17: "one that shows only on LG's aarch64"; :1609, -19,
      judges ML's Chrome "as it stands");
    - the run's room log is on `x86_64`.
    - Every other mismatch counts.
  - (c) A client hash at a (slot, tick) where the server logged none (an orphan) counts on every machine. The key's
    excuse ("miss the server's logged hash too") needs a server hash for the replay to reproduce. This is a reading
    of ADR 0001:1605, and it is stricter. It joins the readings list (R5).
  - (d) An aarch64 mismatch against a server hash, in a run that has a replay log but no replay entry at that
    (slot, tick), is a gap: the re-decode is incomplete. Today it counts as a mismatch. A run with no replay log at
    all stays NOT-RUN (Waiting), as now.
- **The text:**
  - SERIES `decoded_hash_mismatch_count`: "On an aarch64 client's log (machine.arch), a mismatch counts only when
    `hs.replay_hash` at that (slot, tick) differs from the server's too (the Arm rule; else
    cross_arch_mismatch_count)." becomes:
    > On a client log written on LG (`machine.role` LG) on aarch64, in a run whose room log is on x86_64, a mismatch
    > against a server hash counts only when `hs.replay_hash` (a `replay` log on x86_64) at that (slot, tick) differs
    > from the server's too (the Arm rule; else cross_arch_mismatch_count); a replay log with no entry there is a gap.
    > A client hash with no server hash counts on every machine.
  - The same row's Gaps gain: "a name in a log of another process (`server_hash` outside `room`, `decoded_hash`
    outside `client` and `bots`, `replay_hash` outside `replay` on x86_64)".
  - SERIES `cross_arch_mismatch_count`: drop "(or has no server hash there)" and "(both absent is agreement)". Its
    count is the LG aarch64 mismatches against a server hash whose x86-64 replay entry equals that hash.
  - README §1's three `hs` items name their process as in (a) (FX-12 gives the words).
- **Proof:**
  - TE-F12: new fixtures under `fixtures/pm-0001-17/` on the real marks.json's L2 P2 part. Each holds an LG `bots`
    log, a room log of machine.role RS on x86_64 and, where named, a `replay` log:
    - `lg-cross-arch`: the LG hash differs and the x86-64 replay equals the server. The part's run is valid, the
      value is 0 and "aarch64-only" is 1.
    - `lg-replay-differs`: FAIL.
  - Each case below fails today (with today's result):
    - `lg-replay-in-client-log` (C1): FAIL or gap; today PASS.
    - `lg-replay-on-aarch64` (C2): gap; today PASS.
    - `l1-aarch64-claim` (probe O's L1 form, a role C client claiming aarch64): FAIL; today PASS.
    - `lg-orphan` (an aarch64 orphan, with the replay present): FAIL; today PASS.
    - `lg-replay-missing-entry`: INVALID; today FAIL.
  - Judged in tests/fixtures.rs `pm_0001_17_the_arm_rule_holds_on_lg_only`, and added to the folder-reader test's list.

### FX-6: marks.json pins the bound line; the record must carry it byte for byte
SEC-F2 = TE-F6 = NC-9. Probe H: a record saying M_m 620 ms turns a FAIL into a PASS. SEC-F11 rides it (same code).
- **The form (my call: R2.3):**
  - (a) A new optional mark field `ws_bound_line` (a string, the line verbatim). A mark with `ws_bound` checks must
    have it, and only such a mark may.
  - (b) `--check-marks` (marks.rs at load) parses it with `parse_bound`. It refuses the mark unless the line's X, k,
    Y and Z equal the `ws_bound` checks' numbers and its `tolerance` equals the `ws-practice` rule's. These checks
    move from run time to marks time, and `find` keeps them too.
  - (c) The ws-practice rule refuses a record whose `bound` is not that string byte for byte. M_m, the band, the
    guard, the model and the commit then come from reviewed marks.json, never from a log.
  - (d) The provisional line, so that no real counted run can be judged on placeholders. Its model text never equals
    a real run's `params.ws_tcp_model` (FX-13), so such a run comes out INVALID until fixes.rs's bound replaces the
    line. In marks.json, escaped as JSON needs:
    `npt-ws-bound v1 {"model":"provisional: the single-burst bound of bf9d248 (fixes.rs ws_bar_fix1), not the run-structured bound","commit":"bf9d24807106c4524d1aa0849409d219259c3943","runs_count":2000,"client_bursts_count":480,"x_ns":670000000,"y_ns":2500000000,"k_count":5,"z_ns":6900000000,"m_m_ns":654000000,"band_low":"0.965","band_high":"1.035","tolerance":"1.10","guard_share":"0.0000"}`
  - (e) -18's notes gain: "The bound's line is held verbatim in `ws_bound_line`; until fixes.rs's run-structured bound
    replaces it, its model says provisional, so no counted run can match it."
  - (f) SEC-F11: `compile_check` also pins each letter's shape:
    - x: stat p50, op `<=`;
    - y: stat count, a `where` with op `>`, op `<=`;
    - z: stat max, op `<=`.
    - Any other shape is refused.
  - (g) A new public item (A19): the field on `Mark`. List it for §4.1 at the merge.
- **The text (README §2.5):**
  - Replace "marks.json's PM-0001-18 holds the same X, k, Y, Z (its `ws_bound` checks) and tolerance
    (`ws-practice:<t>`), named in its notes by the file's path and commit." with:
    > marks.json's PM-0001-18 holds this line verbatim in its `ws_bound_line` (provisional until fixes.rs's bound
    > lands: its `model` says so, so no counted run's header matches it), and `--check-marks` refuses the mark unless
    > the line's X, k, Y and Z equal its `ws_bound` checks' and its `tolerance` the `ws-practice` rule's.
  - In the record bullet, "`bound` (the bound's line, verbatim)" becomes "`bound` (the bound's line, byte for byte
    the mark's `ws_bound_line`)".
  - README §2.1's mark-field table gains a `ws_bound_line` row in the same words.
- **Proof:**
  - engine.rs `u4j_ws_the_record_carries_the_marks_bound_line`: probe H committed. The record's line says M_m 620 ms,
    with its moved numbers recomputed to match: INVALID. Today it PASSES. A second case changes only the line's
    commit: INVALID.
  - marks.rs unit `ws_bound_line_agrees_with_its_checks`, each case refused:
    - the line's X differs from check (c);
    - its tolerance differs from the rule's;
    - no line on a mark with ws_bound checks;
    - a line on a mark without them;
    - band 2.0 to 2.0.
  - SEC-F11's shapes go in `ws_bound_checks_keep_their_shape`: x with op `>=`, x with stat max, z with p50, and y with
    a `where` op `>=`, each refused. Fails today.

### FX-7: the practice run needs exactly the bound's heal count
NC-7 = TE-F7 (probe G: 479 heals PASS). My R7: "Valid only with 480 heal records".
- **The fix:** `heal.len()` must equal the bound's `client_bursts_count`, else INVALID.
- **Proof:** `u4j_ws_practice_refuses_every_number_not_set_by_its_rules` gains 479 and 481 heals, each INVALID.
  Fails today.

### FX-8: no panic on a long decimal
SEC-F6 = TE-F8 (probe B: the judge panics at ratio.rs:27).
- **The fix:** `parse_bound` refuses a fraction with more than 6 digits after the point (an Err, never a panic).
- **The text:** README §2.5, "the four fractions are decimal strings (`"0.965"`), read exactly" becomes "the four
  fractions are decimal strings with at most 6 digits after the point (`"0.965"`), read exactly; band_low <= 1 <=
  band_high".
- **Proof:** `practice::tests::the_bound_line_refuses_more_than_6_decimals`: probe B's line is an Err. Fails today
  with a panic.

### FX-9: the tolerance and band boundaries are pinned by tests
TE-F9 (plant T: cap 1.10 to 1.12, band_low dropped and the Z' recheck dropped together survive all 65 tests).
TE-R1 rides it. My ruling (advisor-ws-docs-r2.md:378) asks for "the boundaries s = 1.00, 1.10 and 1.1001".
- **Proof:** engine.rs `u4j_ws_the_tolerance_and_band_boundaries`, five cases:
  - s = 1.00: PASS at the bound, f = 1;
  - s = 1.10 (probe T1a): PASS at 737 ms, 2.75 s and 7.59 s;
  - s = 1.1001 (T1b): INVALID;
  - IN TOLERANCE at s = 0.95, below band_low (T2): INVALID;
  - one practice heal over Z' only (T3): INVALID.
- These pass on today's code, which is right here. Their proof is plant T: the fixer re-runs it in place, and T1b, T2
  and T3 each fail by name.

### FX-10: the coverage floor, which checks README's own `null` rule
TE-F10 = SEC-F8 = NC-5 (i), with TE-R2. Probe S: one hash of six passes "100 % of decoded snapshots".
- **The form (my call: R2.1):** in `decoded`, over the logs FX-5 (a) admits:
  - (a) Every `hs` list and every `tick.s.snap_incomplete` list holds exactly the run's `clients` entries (the room
    log's header). Any other length is a gap, checked with the other structural gaps.
  - (b) Resolved: a (slot, tick) is resolved when a client log gives it a hash (compared) or a
    `tick.s.snap_incomplete` of 1 (lost, not compared). Both at once is a gap.
  - (c) Per slot, the span runs from the slot's first resolved tick to its last. Every tick inside the span at which
    the room logged a server hash for that slot must be resolved. If not, it is a gap: "slot s, tick t: the room
    logged a hash and no client log resolves it". That catches a null with snap_incomplete 0, and a missing tick
    record (a sampled log).
  - (d) Every slot the room logged at a measured tick has a span that covers the room's first and last measured
    server hash for it. A slot with no span, or a short span, is a gap.
  - (e) Ticks outside a slot's span are neither compared nor counted: before the client's first snapshot, and ticks
    still in flight at the stop.
  - (f) The order, so a gap never hides a failure (my C2 ruling):
    - First, (a) and the structural gaps (malformed entries, slot counts, two logs at one (slot, tick), a server 0,
      a name in the wrong process).
    - Then, any counted mismatch gives its count: FAIL.
    - Then, 0 compared gives `empty`.
    - Then, a coverage gap (b) to (d) or FX-5 (d) is a gap.
    - Then the value 0.
    - The Waiting case (an LG mismatch in a run with no replay log) also comes after the counted mismatches. Today it
      returns at once, so it can hide an orphan's or another log's FAIL as NOT-RUN. A further FX-5 fixture,
      `lg-orphan-no-replay` (an LG orphan plus an LG mismatch against a server hash, with no replay log), must FAIL.
      Today it is NOT-RUN.
- **The text:** the SERIES `decoded_hash_mismatch_count` row's Gaps gain:
  > the coverage rule (README §1's `null` rule, checked): every `hs` list and `tick.s.snap_incomplete` list holds the
  > run's `clients` entries; a (slot, tick) is resolved by a client hash (compared) or by `tick.s.snap_incomplete` 1
  > (lost, not compared), never both; per slot, every tick from its first resolved tick to its last at which the room
  > logged that slot's hash is resolved, and that span covers the room's measured ticks for the slot; a mismatch still
  > fails the run, and 0 compared still comes out `empty`.
  - README §1's writer side is FX-12's.
- **Proof:** new fixtures under `fixtures/pm-0001-17/`, each INVALID and each failing today:
  - `sparse` (probe S): one hash of six, nulls with snap_incomplete 0;
  - `sampled`: a client tick record missing inside the span;
  - `slot-unlogged`: a slot with measured server hashes and no client entry;
  - `hashed-and-incomplete`;
  - `short-lists`: 2 entries under `clients` 48.
  - They are judged in tests/fixtures.rs `pm_0001_17_every_server_hash_is_resolved_on_the_client_side`.
  - The six existing cases move to 48-entry lists. The extra slots may be null in both logs (clients not connected),
    so their results and counts stay as they are. `none-compared` still FAILs (empty), with its gaps beside it.

### FX-11: the verdict prints the practice outcome whole
SEC-F9 (the fix pass's half; the rest is follow-up FU-2) = NC-14.
- **The fix:** `Practice::describe` prints, in this form:
  > {n:?} {bound} x f {f} = {moved}, rounded up to the ms; practice run {id}: {class}, s = M_p {m_p} / M_m {m_m} =
  > {s}, band {low} to {high}, tolerance {tol}, f = {f} ({max(1, s) | FAR BETTER: at the bound})
  - Then "; chance ruling: {text:?}" and "; second ruling: {text:?}" when the record has them.
  - So a run judged under a ruling never reads as IN TOLERANCE.
- **The text:** SERIES's `header.params.started_utc` row gains -18 in its Marks column (NC-14).
- **Proof:** `u4j_ws_practice_puts_the_practice_runs_numbers_in_force` asserts that check (c)'s needs line holds the
  class, "band 0.965 to 1.035", "tolerance 1.10" and, in its BEYOND case, the chance ruling's text. Fails today.

### FX-12: README's `hs` writer rules, exact
NC-3 and NC-5 (ii) to (iv), with NC-11. The words below replace README §1's; my call: R2.6.
- **The `hs` bullet's first sentence:** "name → one entry per client slot of the run (index = slot, the same length
  in every record of the run)" becomes:
  > name → one entry per client slot of the run: exactly the header's `clients` entries in every record of every log
  > of the run, at index = the room's slot for that client (the slot the room's `tick.s` lists use; a client takes it
  > from `JoinAccepted`), the same in every log
- **The `server_hash` item:** after "never 0;" insert:
  > written only in the room's log (process `room`), for each connected client from its admission on, in every phase
  > the clients log; `null` for a client not connected and for a tick whose snapshot the room did not send that client
  > (one the WebSocket send budget skipped);
- **The `decoded_hash` item:**
  - "(the clients' log, at the tick t the client rebuilt)" becomes "(only in the clients' logs, process `client` on L1
    and `bots` on L2, at the tick t the client rebuilt)".
  - "a client with two hashes for one tick writes the sentinel there;" becomes:
    > a client with two different hashes for one tick writes the sentinel there (one hash met twice, as a duplicated
    > datagram gives, is written once), and no hash is ever dropped: the writer buffers until every earlier tick is
    > written, or ends its log `crash`. Every tick from a client's first snapshot to the end of its log is resolved for
    > that client: a hash, or `null` with `tick.s.snap_incomplete` 1 (a tick none of whose groups arrived counts too,
    > known from the next header's tick); after the measured phase the clients keep logging until every measured tick
    > is resolved, and a tick still in flight at the stop is not written;
- **The `replay_hash` item:** "(process `replay`, the x86-64 re-decode" becomes "(only in a `replay` log on x86_64,
  the x86-64 re-decode".
- **The `twin` role:** "named in the other's `params.twin`: PM-0001-3, -21)" becomes "named in the other's
  `params.twin`: PM-0001-3, -21; a run and its twin log `h` over the same ticks, since a tick hashed in one only counts
  as a mismatch)".
- **The SERIES rows** for `hs.server_hash`, `hs.decoded_hash` and `hs.replay_hash`: the same process words.
- **Proof:** fixture `pm-0001-17/permuted-slots` (two connected clients' entries swapped in the client log): FAIL. It
  passes today too. It documents that pairing is by the room's slot, and that the rule matters.

### FX-13: the practice run ran on the counted run's setup
TE-F16 (Low). It rides FX-3's change to `find`. My R7: "Valid only with ... a header naming the model, RS's kernel
and CONFIG_HZ".
- **The fix:** the practice run and the run (counted, or a bite run) must each have the same three things, else
  INVALID (absent or different):
  - `params.ws_tcp_model`, equal to the bound's `model`;
  - `params.config_hz`;
  - the room log's (machine.role RS) `machine.kernel`.
- **The text:** FX-3's paragraph already carries it. README §2.5's record bullet drops "the header checks" from the
  fields "for people".
- **Proof:** `u4j_ws_practice_refuses_every_number_not_set_by_its_rules` gains these cases, each INVALID and each
  failing today:
  - another kernel;
  - another config_hz;
  - another ws_tcp_model;
  - a ws_tcp_model that is not the bound's model;
  - each of the three absent.
  - The ws tests' headers gain the three params, with the provisional model's text (FX-6 (d)).

### FX-14: machine_named's remaining conditions, and a -1 fixture
TE-F13 (Low). It rides the push, because §5 U4j asks for machine_named "with fixtures" on -1's parts.
- **The fix:** tests only.
  - `u4j_machine_named_needs_every_field_and_a_vm_its_address`'s FAIL list gains role, name, plan and arch emptied,
    and mem_mb and threads_per_core at 0.
  - A fixture `pm-0001-1/laptop` (an ML or MAC log: every field, no address) PASSes its part's figure, in
    tests/fixtures.rs `pm_0001_1_a_laptop_needs_no_address`.
- **Proof:** the fixer plants "role not checked" once in figures.rs, in place and restored, and the new case fails by
  name.

### The fix pass's end, for the main session
- Every FX names its tests in the fixer's notes, with the failing line on the unfixed code (or the plant).
- fixtures/README.md's table is updated.
- The new public items are listed by crate (at least `Mark`'s `ws_bound_line`).
- marks.json's changes are stated as before and after (-18: `ws_bound_line`, the notes sentence).
- The checks in R1's preamble pass twice.

### Follow-ups (not before the merge; each goes in PR #40's report, with its owner)
- **FU-1 (SEC-F7, Low):** C4's bite direction, "at least one log" (reading (a), accepted). Once the run plans know
  which process carries each bite, marks.json names it per bite and C4 checks that log. Owner: R3's run-plan unit
  (the first L2 bite runs). Today it fails safe: such a run is usually Missed, so the mark FAILs.
- **FU-2 (SEC-F9, its later half, Medium):** rulings by id. Before any counted WebSocket run can rest on a ruling:
  - the advisor's rulings of chance and of a found cause live in a committed file, each with an id, the practice
    run's id and its text;
  - the record names the id;
  - the judge refuses an id that is not in the file, or one that names another run.
  - Owner: the unit that builds the run script's practice record, before R3's run plan goes to Mustafa.
- **FU-3 (SEC-F10, Low; it was there before U4j):** npt-log reads a duplicate JSON key silently, the last one
  winning. A refusing pass in `read_log`. Owner: the next change to npt-log; U4r's brief names it.
- **FU-4 (SEC-F12, Low):** a run of the WebTransport part labelled `ws`. Once npt-server writes `burst_return`, the
  `L2 B1 ws` part gains a setup check that the WsServer-only series exist (`event.burst_return.budget_bytes` count
  >= 480). Owner: R3's run-plan unit.
- **FU-5 (TE-F5's C3 and NC-6's second option, Low):** the mark's own `arm_rule`. `--check-marks` refuses a mark with
  `arm_rule` false whose check reads `decoded_hash_mismatch_count` on a part that can select an LG log. No mark does
  so today: -58 is L1 only. Owner: the first unit that adds such a part.
- **FU-6 (mine, A-U4J-1, Medium):** R7's message-size condition goes unchecked by the judge: "a mean WebSocket
  snapshot message size within 5 % of the size the bound used". The line has no size field, and the practice run has
  no size series. The run script's class enforces it today.
  - The fixes.rs bound unit adds the mean message size to the line. If v1 lines are in use by then, the tag becomes
    v2.
  - The judge checks the practice run's mean against it once U4r logs per-message sizes (§2's "Sizes for fixes.rs"
    row).
  - Owner: the fixes.rs bound unit, before R3's run plan.

### Carries
- **U4e (the room and the client hash):**
  - the room logs `server_hash` for each client from its admission, in every phase, and null for a snapshot it did
    not send;
  - the client files each hash under its rebuilt tick, at the room's slot from `JoinAccepted`;
  - two different hashes give the sentinel; no hash is dropped;
  - a tick none of whose groups arrived counts in `snap_incomplete`.
  - All of these are FX-12's words.
- **U4r (the lab's logs and the contract test):**
  - the writer rules of FX-5 (a), FX-10 and FX-12: each name in its own process; lists of `clients` entries; the
    after phase until every measured tick is resolved; a run and its twin hash the same ticks;
  - the contract test shows `decoded_hash_mismatch_count` resolving with no gap on every -17 suite;
  - NC-10 (R2.4).
- **U6:**
  - U6-a: -30's twin form (each other's measured runs, differing in `params.transport`) beside FX-4's role check
    (SEC-F5's second half);
  - U6-b (NC-13): C4's role list refuses PM-0006-20's logging-off twin of its planted-format bite run, while
    `twin_of` asks for the same bite; U6's `alloc_twin_diff` settles it;
  - U6-c: -19's join hash against FX-10's coverage rule. A trapped and rejoining client must leave no unresolved
    server tick, and ML's `browser` process joins FX-5 (a)'s list when -19's figure is built.

## R2. The calls you asked for
- **R2.1, the coverage floor and the null reconciliation (TE-F10 = SEC-F8 = NC-5 (i)): FX-10's form, before the
  merge.**
  - Why before:
    - It checks the README's own rule ("`null` for a tick with a group lost and none refused (counted in
      `tick.s.snap_incomplete`)"), so it adds no new bar.
    - Without it, "100 % of decoded snapshots" passes on 1 hash of 6.
    - It is the input contract U4r writes to, so fixing it later means writing the logs twice.
  - Why this form: it is per slot and per tick against the room's own hashes. The span keeps an honest log's head and
    tail (before the first snapshot; in flight at the stop) from being gaps. The measured-phase cover stops a log that
    starts late or skips a slot. The order (f) keeps C2: a gap never hides a mismatch or an empty comparison.
  - Not taken: a count-only floor (compared + incomplete = server pairs). It cannot say which tick is missing, and a
    sampled log can still balance it.
- **R2.2, rulings as any non-empty text (SEC-F9).**
  - Now: FX-11, the verdict prints the class and each ruling's text with M_p, M_m, s, the band, f and the tolerance.
    That meets my R8 (v), "the ruling is in the run's report for Mustafa".
  - Later: FU-2, rulings by id in a committed file, before R3's run plan goes to Mustafa.
  - It stays a security finding left open until FU-2, so the PR stays needs-mustafa. It is that already.
- **R2.3, pinning the bound line (SEC-F2 = TE-F6): FX-6's form.** The line goes verbatim in marks.json
  (`ws_bound_line`), and the record must equal it byte for byte.
  - Why not a sha256 or a few numbers: the verbatim line is readable in review, pins every field the judge reads
    (M_m, the band, the guard, the model, the commit), and needs no hash code.
  - The provisional model text makes the placeholders unusable for a real run (FX-6 (d)).
- **R2.4, NC-10 (a P0 setup check that `tick.s.snap_incomplete` sums to 0): not now; a U4r carry, evidence first.**
  - With FX-10 in, a refused group can hide only as "lost and counted". On L1 P0 (no loss: l1/src/lib.rs:116) that
    count must be 0.
  - U4r's contract test reports the count on each short L1 P0 run. If it is 0 on every one, U4r adds to -17's L1 P0
    part, with a bite (a client that writes null and incomplete 1 for a refused group):
    `{"name": "setup: L1 P0 loses no group (no client-tick left incomplete)", "series": "tick.s.snap_incomplete",
    "stat": "sum", "window": "all", "op": "==", "value": 0, "parts": ["L1 P0"], "setup": true}`.
    It goes in before -17's judged runs.
  - If the count is not 0, the cause comes to me first.
  - Why not now: a check whose writer is not built could make every correct L1 P0 run INVALID (admission, the stop)
    before anyone has seen one.
- **R2.5, TE-F16 (the kernel, config_hz and ws_tcp_model compared between the practice and the counted run): yes,
  now (FX-13).** It is my R7's own validity condition, it rides FX-3's change to `find`, and it is a few lines.
- **R2.6, README's `hs` writer rules (NC-3, NC-5 (ii) to (iv)): accepted whole, before the merge, in FX-12's words.**
  - NC-5 (i) is FX-10.
  - NC-11, the twin's same tick range, rides with them.
- **R2.7, the Arm rule's key (SEC-F4 = TE-F5, F11, F12; reading (e)): FX-5.**
  - The process per hash name.
  - LG and aarch64, with the room on x86_64.
  - An orphan counts everywhere.
  - A missing replay entry is a gap.
  - Fixtures on the real marks.json's L2 part.
  - The mark's own `arm_rule` is FU-5.
  - Why LG: ADR 0001's -17 and -19 rows say "LG's aarch64", and -19 judges ML's Chrome "as it stands", so the
    machine's role, not only its arch, is the key.

### Every finding, mapped (so nothing is dropped)
| Reviewer | Finding → where it goes |
|---|---|
| netcode | 1 → FX-1 · 2 → FX-2 · 3 → FX-12 · 4 → FX-4 · 5 (i) → FX-10 · 5 (ii) to (iv) → FX-12 · 6 → FX-5 (b), FU-5 · 7 → FX-7 · 8 → FX-5 (a) · 9 → FX-6 · 10 → R2.4 (U4r) · 11 → FX-12 · 12 → FX-5 fixtures · 13 → U6-b · 14 → FX-11 |
| security | F1 → FX-1 · F2 → FX-6 · F3 → FX-3 · F4 → FX-5 · F5 → FX-4, U6-a · F6 → FX-8 · F7 → FU-1 · F8 → FX-10 · F9 → FX-11, FU-2 · F10 → FU-3 · F11 → FX-6 (f) · F12 → FU-4 |
| test | F1 → FX-1 · F2 → FX-2 · F3 → FX-3 · F4 → FX-4 · F5 → FX-5 (C3: FU-5) · F6 → FX-6 · F7 → FX-7 · F8 → FX-8 · F9 → FX-9 · F10 → FX-10 · F11 → FX-5 (c) · F12 → FX-5 fixtures · F13 → FX-14 · F14 → FX-4 · F15 → FX-3 · F16 → FX-13 · R1 → FX-9 · R2 → FX-10 (a) |
| advisor | A-U4J-1 → FU-6 |

## R3. The builder's readings (a) to (f)
Quoted from the builder's handback ("Readings for the advisor or reviewers"). They are not in u4j-build.md, so the
main session should copy them into its notes at the merge.
- **(a) "C4's 'at least one log' form", because -4's U1 fixture has RS's room on the bites build while LG's bots run
  the measured build: ACCEPTED.**
  - netcode: right. security F7: sound in the measured direction, and weaker in the bite direction, where it fails
    safe.
  - Recorded as a risk; tightened by FU-1.
- **(b) "A bite run under ws-practice takes its set's measured-build practice run": CHANGED (FX-3 (c)).**
  - netcode: right, under R11.
  - Kept: the bite run is judged at the counted run's numbers.
  - Changed:
    - the practice run is the one in force for its set's measured runs on the same layer, profile and transport
      (one build), found over every run the judge reads, with FX-2's start rule;
    - so a practice run of an older build elsewhere cannot make it INVALID;
    - and the bite run cannot take an f other than its counted run's.
- **(c) "The ruling's `machine.config_hz` is written as `params.config_hz`", since a new Machine field would break
  npt-lab's machine.rs: ACCEPTED.**
  - No reviewer objected.
  - My ruling's text (advisor-ws-docs-r2.md:377, SERIES (5)) reads as `params.config_hz` from now on.
  - FX-13 compares it.
- **(d) "A bite run that compared nothing is INVALID": ACCEPTED.** All three agree. It can only lower a verdict. It
  follows README §2.3's rule for setup checks: a run that did not test the mark shows no bite.
- **(e) "The Arm rule is keyed on the client log's `machine.arch` = aarch64, not on `mark.arm_rule`": CHANGED
  (FX-5).**
  - netcode F6 (partly), security F4, test F5 and F11 (wrong).
  - The key is now:
    - an LG log on aarch64, with the room on x86_64;
    - each name from its own process;
    - an orphan counts everywhere;
    - a missing replay entry is a gap.
  - The mark's flag is FU-5.
- **(f) "The decoded-hash figures read every phase", because -58's wraps fall in its first seconds: ACCEPTED.**
  netcode: right, and it makes NC-5 (ii) a writer rule. That rule is in FX-12's `server_hash` words (from admission,
  in every phase the clients log).

## R4. Severity
- **F1 (NC-1 = SEC-F1 = TE-F1): High, confirmed.** netcode labelled it Medium; I set it High, in writing.
  - The reason: a rule loosened beyond Mustafa's words. A log can make the judge move X, Y and Z by 90 %, while his
    words cap the move:
    - WS-BAR's refinement (2026-11-07): "If the practice run is up to 10 % slower than the model, the numbers may move
      by that much; beyond 10 %, investigate first and never loosen".
    - NETEM-BAR (a) (2026-11-10): "The bar stays capped as WS-BAR says".
  - Probe A shows the move (X' 1.273 s), and nothing but a non-empty ruling text stands in the way.
- **How round 2 proves it closed:**
  1. The fixer's notes show `the_bound_line_refuses_a_band_that_leaves_out_1` and
     `u4j_ws_far_better_past_the_cap_is_invalid` failing on the unfixed code, then passing.
  2. The test-engineer re-runs probe A on the head: INVALID. It then runs three plants, one at a time, each caught by
     name:
     - plant T (cap 1.12, band_low dropped, Z' recheck dropped): caught by FX-9's test;
     - "parse_bound's band check removed": caught by FX-1's unit test and FX-6's marks test;
     - "FAR BETTER f = max(1, s), with the band check and the pin removed": caught by
       `u4j_ws_far_better_past_the_cap_is_invalid`.
  3. The security-auditor writes a table of every path to `Ok` in `find` (class × s range → f) showing f <= the
     tolerance on each, with the line pinned by FX-6.
  4. The netcode-architect checks README §4's new paragraph against my R6 and R8 (v) and Mustafa's two quotes above.
  - If any of these is open after round 2, F1 alone gets a round 3. If it is still open after round 3, the
    change-set stops for Mustafa.
- **The others:** the reviewers' labels stand. FX-2, FX-3, FX-4, FX-5, FX-6, FX-7, FX-8, FX-9 and FX-10 are Medium;
  FX-13 and FX-14 are Low riding the push; FX-12 and FX-11 are Medium (the text U4r writes to; the report for
  Mustafa).

## R5. Decisions needed
**None.** Every call above is technical, inside the approved plan and my rulings on it, and none loosens a bar: each
fix is stricter or restores a ruled rule. Two notes for the main session (no decision):
- **The readings list** (u4-build-plan.md §10's preamble and §11; the readings PR before R5) gains one entry, exact
  words: "PM-0001-17's Arm rule key ('counts only once that client's logged bytes, decoded again ... on x86-64, miss
  the server's logged hash too': ADR 0001:1605) read as applying only where the server logged a hash: a client hash at
  a tick the server never logged counts on every machine (the U4j fix pass, the advisor's FX-5 (c)). Stricter."
- **Open security findings** at the merge: FU-1 to FU-5 (SEC-F7, the later half of F9, F10, F12, and C3's flag). They
  keep PR #40 needs-mustafa, which it is already, and each is named in its report.

## For Mustafa
Nothing now.

## What I checked, and what I did not
**Checked (read only; nothing edited, built or run in the worktree):**
- My role, from origin/main:.claude/agents/advisor.md, word for word.
- The worktree is at 411eeeed50d8c71e7e1f74366f0e1d5c27388444: 11 files modified, 22 untracked (21 fixtures and
  practice.rs), plus u4j-build.md.
- The code:
  - practice.rs, whole: parse_bound :50-104, class_ok :384-391, f :406;
  - figures.rs :100-360: twin_of, twin_hash_mismatch_count with no role check :195-209, slot_hashes reading every
    log :215-252, decoded with the orphan-as-agreement compare and the early Waiting :257-336;
  - eval.rs: Run and serves; in_force; always_rules (C4) :541-576; the ws-practice rule :998-1002; set_of
    :1161-1163; judge_run; eval_bite; judge_mark; judge, which shows `all` holds every run the judge reads;
  - marks.rs's ws_bound shape check :587-621 (op not pinned: SEC-F11);
  - the README diff, whole;
  - marks.json -1, -17, -18, -21, parsed.
- The test-engineer's outputs probe-run1.txt, probe-run2.txt and plant-t.txt. Their results (A, B, C1-C3, D, E0-E2,
  F, G, H0-H1, T1a-T3, S, O) match the reviews' claims.
- The binding texts:
  - u4-build-plan.md: §2, §4.6, §4.12, §5 U4j, and Q11, Q20 and Q22's options;
  - advisor-u4.md: the verdict and conditions 1-13;
  - advisor-u4-wave1.md W1-N2;
  - advisor-ws-docs-r2.md :375-400 (R3-R12, and the -18, ws-practice and record lines);
  - history.md :292-294 (NETEM-BAR (a), verbatim) and :381-391 (WS-BAR and its refinement, verbatim);
  - plan.md:359 (the Arm rule);
  - ADR 0001 :1399, :1605 (-17's key) and :1609 (-19's ML "as it stands").
- Cites checked:
  - l1/src/lib.rs:114-116 (P0: "0 ms, no jitter, no loss");
  - the full SHA of bf9d248 (bf9d24807106c4524d1aa0849409d219259c3943, by `git rev-parse` in the worktree);
  - Machine has `kernel` (log/src/lib.rs:24-41);
  - L1's client process is `client` (fixture headers);
  - SERIES :306's started_utc row lacks -18 (NC-14).
- The builder's readings (a)-(f): from its handback in the session's agent transcript, since u4j-build.md has none.

**Not checked:**
- No probe or plant re-run, and no check.mjs run: I build and run nothing. The counts 304 / 327 / 48 and the probe
  results are the builder's and the test-engineer's, read from their notes.
- I did not read engine.rs or fixtures.rs whole. The test names I cite are from grep. The test-engineer's claim that
  65 committed tests survive plant T is theirs.
- Whether FX-10's coverage rule fits U4r's real writer at admission and at the stop. No such log exists yet. U4r's
  contract test is where that shows, and R2.4 keeps the one check with no evidence out until it does.
- u4-build-plan.md's W1 text batch (sha256 now a81b230f…, per the checkpoint) against U4j's section: I read the
  section as it stands, not the diff from 0d69332c.
- Whether `Mark`'s new field can be `pub String` without exposing practice.rs's pub(crate) `Bound`. That is the
  fixer's to settle, and to list under A19.






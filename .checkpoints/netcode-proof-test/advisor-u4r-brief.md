# The advisor's check of the U4r builder brief (rev 3.1), 2026-11-13

Seat: a day session's advisor (workflow agent, opus, effort xhigh), on main's `.claude/agents/advisor.md` (read with
`git show origin/main:.claude/agents/advisor.md`). Read only: no build, no edit of a source or of the brief, no commit.
Branch `claude/netcode-proof-test-plan-hcky5g`, head ad248a2 when this was written (checkpoint commits only since
c49773f). The brief: u4r-brief.md rev 3.1, sha256 bcaef38322375a74c902f998eef079b630e9525ae98550d84f7e318596e36e2e
(1,625 lines). Notes: advisor-u4r-brief-notes.md (this run was cut by the spend limit at 13:06 after B9 and resumed at
14:12; the notes hold every block).

## Verdict
**APPROVE WITH CONDITIONS.** The build may launch once conditions L1 to L5 are met; L6 is the launch.
- Every item of plan §5 U4r (its Builds, Tests, judged runs, the -30 pre-check, the sizes, the Evidence and the
  reviewers) and every carry of §U4r is in the brief, testable as worded (check 1). The placeholders P1 to P15 each find
  their anchor once, with the first and last words named (check 2). My rulings since the night's rev 2 are applied, with
  two gaps (check 3: A1, A8). netcode-architect's High (F-j) and four Mediums are answered, and the revision's own
  addition (U4R-Q14 (b), the duplicate stamps) holds under my arithmetic.
- My findings: **two Medium** (A1: the TE-M3 line and P12 miss my TE-M3 rulings; A3: U4R-Q14 (a)'s lead has no margin
  at P0) and **seven Low** (A2, A4 to A9). None High; no security defect.
- All twelve open questions are ruled below, none escalated: each is a technical or test choice inside the plan, or
  the text of my own conditions and carries. No rule or pass bar is loosened, and no word or decision of Mustafa's
  changes.
- The lane runner u4r-run.sh: **approved**. INF2-4's runner follow-up (my M9 Block 2): **met**.
- Risk tier 1 (spike code on PR #40's branch; check.mjs is the spike's own check, not one of tools/' gate files). This
  ruling is a notes-folder text.

## What I read, and the state it rests on
- The brief whole; its history (rev 2 at ed973c9; the rev 3 check u4r-brief-r3-check.md: REVISE, F-j High, F-a, F-k,
  F-l, F-m Medium, 13 Lows; the drafter's and reviser's notes u4r-brief-r3-notes.md, read as their account, not as
  evidence).
- The sources: u4-build-plan.md §5 U4r (:750-803) and the parts the brief cites; plan.md; u4s-carries.md §U4r
  (:255-432) and entries 6, 14, 20, 49, 63, 74; my rulings advisor-u4.md, -u4-wave1, -u4d-close, -u4j, -u4j-r2,
  -u4df, -u4e-r1, -u4e-r2, -u4d2-r1, -u4d2-r2, -u4d2-m9, -te-m3 and -te-m3-check; the code at the head under
  spikes/netcode-proof/ (crates client, room, server, l1, link, sim, lab, bots, log, net; check.mjs).
- **Stamps at the head** (equal to section 0's): plan 01ede63d…, plan.md 3ca4e12a…, carries 8b7f165e…, history
  88bc195f… (2,011 lines), u4r-run.sh 349b51ad…; all 13 rulings' sha256 equal section 0. New since: advisor-te-m3.md
  5ecc3e23… and advisor-te-m3-check.md 1ffd0294… (A1).
- **spikes/ moved once since the brief's head read:** `git diff --stat cc8595d HEAD -- spikes/` names only
  crates/net/src/l2/delay_tests.rs (+10 −3, c49773f, TE-M3's fix); `git diff --quiet c49773f HEAD --
  spikes/netcode-proof` exits 0 (A2).

## Check 1: coverage of §5 U4r, testable as worded
Every Builds item (`SimClock`, `L1Scenario` in RQ8's order, the header params, the `--bite` dispatch, the logs with
`admit` events and footers, `scenario --record` and `replay`, the runner recordings, -57's pairing, the brawl and the
open-loop scripts, the c-u4r scripts), every Test (r1 twin with its bite; r2 contract with PM-0001-2 passing and
condition 5's suites by U4R-Q4; r3 replay with the floored-grid bite; r4 class audit with owner-to-all; r5 ticks per
second and the set's total), every judged run (-17 ×4 with tick16 at 63,736 and client-rounding; -57 with its three
bites; -18 B1 with 10 bursts, no-fallback and `burst_eviction`), the -30 pre-check whole (suite, legs, depth, comparer,
`late_count`, the setup condition, the RTT rule with its logs, both bites, no mark, the records, NOT-RUN), the sizes for
fixes.rs with their framing, the Evidence (7.4) and the reviewers (section 12 item 7) are in the brief, each ending in
an exit code, a test's own summary line or a judge's line. The plan cites I spot-checked hold (:750, :756, :758,
:766-772, :791-792, :796-803, :806, :835, :837, :881-888, :909-930). The additions beyond the plan are marked as the
brief's (r6, r7, 7.2's receipt field, 8.4's switch, precheck-recordings.json, the `rewinds_refused` reports, -18's heal
bite check, 2.2's step rule, D13), but one (A7).

## Check 2: the placeholders
Print-only, against the files now: P1 carries "## U4r (the L1 runner and its judged runs)" once (:255; span :255-432;
entry numbers exactly the 17); P2 history :292 once and its continuation :293 ("before it changes anything."), the line
before it "  USAGE-SAVINGS: (a), all three, with these limits." (two leading spaces: the brief's whitespace rule reads
it as one, so its assert holds); P3 df-ruling :462-464; P4 df-ruling :465-468 (two `<crate>`: the first replaced,
"<crate>'s files" by "their files"); P5 j-r2 :168-171; P8 j-r2 :298; P9 m9 :252-256; P10 m9 :259-264; P11 d2-r2
:304-308; P12 d2-r1 :308-310; P13 to P15 carries :72-75, :116-119, :135-136. Each once, first and last words as named;
every closed file's sha256 equals section 0. P12's words now need my TE-Q2 beside them (A1), and TE-Q3's words need a
placeholder of their own (P16, A1).

## Check 3: my rulings since rev 2
Applied as ruled: W1-N2 and U4E-Q2 (-58 out: C2), U4E-Q3 to Q6 and Q13 (C6, carry 58, owner-to-all), W1-TARGET item 3
as amended (P9), the close ruling's F3 and carries 58-60, the U4j ruling's carry 62 with R2.4, FU-3 and FU-6, j-r2's
B2.1, B2.3, B6 and B3, e-ruling's U4E-Q13 and F2-ROUTE (carry 66 met by U4d2), df-ruling R3 (69), R4.1 (P4), R6 (the
slip line) and T4 items 1 to 3, e-r1's R1 (73), A5, R3 (74 to 76), R4 with NC-L1 (C7) and NC-L4, R5 (77) and M9,
e-r2's merge note, d2-ruling A13 (crates/bots/clippy.toml), d2-r1 Blocks 4 and 5 (82, 83, TE-M3), INF-1's det-u4.mjs
question, d2-r2 Blocks 1, 2, 5 (the interim rule) and 6 (P11), m9 Blocks 1 and 2. Two gaps: my TE-M3 rulings of today
(A1) and d2-r1's RR-L4 (A8).

## Findings, each with its exact fix
Old phrases are matched with each line break and the next line's indent read as one space. I counted each one in the
flattened brief: once each (50 phrases, print-only `python3 -I` from stdin).

**A1, Medium: the TE-M3 line and P12 do not carry my TE-M3 rulings (TE-Q2, TE-Q3), so TE2-5's end reads wrong.** The
line still says "its cause is the main session's planned change before U4r's merge", and P12 ends the rule "until
TE-M3's cause is found and fixed", which c49773f now meets. My TE-Q2 (advisor-te-m3.md:119-121) moved the end to U4r's
merge, and only if no check.mjs test-step failure happens in P5 or in U4r's counted and judged runs. My check of the
commit (advisor-te-m3-check.md, "What the main session does next", item 2) asks for exactly these words. The test's
lines also moved (:2551-2603 is now its doc :2544-2553 and the test :2554-2614). Fix (made only now that this check has
returned, as that item says):
- Replace the span from "and TE-M3's npt-net test," to "(TE2-5 (c), d2-r1 :303-304)." with: "and TE-M3's npt-net test,
  `l2::delay::tests::a_shutdown_s_never_completes_ends_at_its_deadline_without_spinning` (delay_tests.rs:2544-2614,
  its doc :2544-2553), which failed under a counted run's load (d2-r1 :287-293). Its cause is found and fixed at
  c49773f (TE2-5 (c); the advisor's TE-M3 ruling, advisor-te-m3.md, and its check of the commit,
  advisor-te-m3-check.md: APPROVE): a pump run already under way when the shutdown began calls S's `poll_shutdown`
  once but started before the test's count, so the assertion is `s_calls <= polls + 1`. "<<P16>>" (the advisor's TE-M3
  ruling, TE-Q3, word for word). The test is in no crate U4r changes and reads no U4r file, so a failure of it (polls =
  4 included) is recorded under TE2-5 with its name and panic (FX-10's file of the failed step kept beside the log
  under .gate/u4r/), does not stop your step group, and comes to the advisor whole; never a retry."
- Replace "(the advisor's U4d2 round-1 ruling, Block 5). **Expected failures, named in advance:**" with "(the
  advisor's U4d2 round-1 ruling, Block 5); since TE-M3's fix (c49773f) the rule ends at U4r's merge, and only if no
  failure of check.mjs's test step (any test) happens in U4r's counted and judged runs; if one does, the rule continues
  and that failure comes to the advisor (TE-Q2, advisor-te-m3.md:119-121; TE-M3's P5 had none). **Expected failures,
  named in advance:**".
- Section 12 item 5: replace the span from "5. **TE2-5 (c), TE-M3's planned change**" to "so the builder's TE2-5
  entries read it." with "5. **TE2-5 (c), TE-M3's change: made at c49773f** (advisor-te-m3.md; its check
  advisor-te-m3-check.md, APPROVE; CI green on c49773f). Section 4's TE-M3 line carries it, with P16 and TE-Q2's end."
- Section 12 item 3 gains: "`<<P16>>` advisor-te-m3.md :136-139: the words inside TE-Q3's quotation marks, from
  \"U3c's (u3c-fix2-c2b.md:74)\" to \"run watch every assertion.\" (each found once; sha256 5ecc3e23… asserted
  first)." Section 0's stamps gain `te-m3` advisor-te-m3.md (5ecc3e23…), `te-m3-check` advisor-te-m3-check.md
  (1ffd0294…) and `u4r-check`, this file (its sha256 at the fold).

**A2, Low: the "spikes/ is cc8595d's" tests no longer hold as worded.** Since c49773f, `git diff --quiet cc8595d <base>
-- spikes/netcode-proof` exits 1, so section 12 item 1 would send the launch to a counted run it does not need and item
9 to a re-walk of section 9; and the ancestry checks would accept a base without TE-M3's fix, which the TE-M3 line now
assumes. The counts are unchanged at c49773f (TE-M3's P5: 11 of 11 steps, 491 / 561, self 71 of 71, marks 81, redact
0: advisor-te-m3-check.md, item 6). Fix, each old phrase by its new one:
- "Its spikes/ is cc8595d's (`git diff --quiet cc8595d HEAD -- spikes/netcode-proof` exits 0), U4d2's merge, which
  holds" → "Its spikes/ was cc8595d's, U4d2's merge, and is now c49773f's (TE-M3's fix, delay_tests.rs alone: `git diff
  --stat cc8595d c49773f -- spikes/` names that one file; `git diff --quiet c49773f HEAD -- spikes/netcode-proof`
  exits 0); cc8595d holds".
- "`git merge-base --is-ancestor cc8595d HEAD` exits 0. Record both" → "`git merge-base --is-ancestor c49773f HEAD`
  exits 0 (TE-M3's fix, on U4d2's merge cc8595d). Record both".
- "at cc8595d, U4d2's merge, the same: D1)" → "at cc8595d, U4d2's merge, and at c49773f, TE-M3's fix, the same: D1)".
- "the `git merge-base --is-ancestor cc8595d HEAD` result" → "the `git merge-base --is-ancestor c49773f HEAD` result".
- "read at the head fa8e335 (spikes/ = cc8595d)" → "read at the head fa8e335 (spikes/ = cc8595d; c49773f since then
  changes one npt-net test, which no row names)".
- "spikes/ at `<base>` differs from cc8595d (section 12 item 8)." → "spikes/ at `<base>` differs from c49773f's
  (section 12 item 9)." (this also fixes A5's third cite).
- D1: "They are `<base>`'s counts if `cd /home/user/Shamble-Frontlines && git diff --quiet cc8595d <base> --
  spikes/netcode-proof` exits 0 (it exits 0 at fa8e335)" → "The same at c49773f, TE-M3's fix (its P5: 11 of 11 steps,
  491 / 561, self 71 of 71, marks 81, redact 0: advisor-te-m3-check.md). They are `<base>`'s counts if `cd
  /home/user/Shamble-Frontlines && git diff --quiet c49773f <base> -- spikes/netcode-proof` exits 0 (it exits 0 at
  ad248a2)".
- Section 12 item 1: "If `cd /home/user/Shamble-Frontlines && git diff --quiet cc8595d <base> -- spikes/netcode-proof`
  exits 0, the counts are D1's" → the same with c49773f for cc8595d.
- Section 12 item 9: "if spikes/ at `<base>` differs from cc8595d," → "if spikes/ at `<base>` differs from c49773f's,".

**A3, Medium: U4R-Q14 (a)'s lead has no margin at P0.** The round-trip form gives P0 a lead of 1. With no delay, a
client step's newest n means only s ≤ t_{n+1}: the snapshot of n + 1 leaves at t_{n+1} (RQ8 step 3) and is never
delivered in its send instant (l1/src/lib.rs:1672, `gate_ns < now_ns`), while the client steps come after the tick
(step 5). So a step at a tick's own due instant reads n, stamps n + 1, the tick just run, and its `Input` is delivered
only after that instant: late (room/src/input.rs:107-113). A client's step instants are fixed from its first `input`
instant at the tick period (client/src/lib.rs:386, :452-453), so a client whose first input falls on a tick instant
keeps falling on them (wholly when it is a whole 100 ms from the base, every third tick), and every such command is
late. Lead 2 removes it. Fix: U4R-Q14's ruling below (the lead `npt_sim::ticks_for_ms(rtt_ms + jitter_ms).max(1) +
1`, so P0 2), with its words in 2.1, carry 74 and D13.

**A4, Low: U4R-Q14 (b1) needs a press rule.** The room refuses a command whose press counter moved by more edges than
the ticks it covers since the last accepted command (input.rs:91-99, `PressRefused`, counted apart from `late`:
room.rs:796). Under (b1) a bot that counts its press edges per step can give, after a skipped step, a command covering
one tick with two edges: refused, silently, outside carry 74's late count. Fix: in U4R-Q14's ruling below (press edges
counted per command given), with 2.2's sentence and D13's `press_refused` count.

**A5, Low: three cross-references send the launch re-check to section 12 item 8; it is item 9.** Fix: "the re-check at
launch compares (section 12 item 8)." → "the re-check at launch compares (section 12 item 9)."; "(section 12 items 3
and 8)" → "(section 12 items 3 and 9)"; the third is A2's section 9 line.

**A6, Low: one wrong cite.** 2.1 step (2)'s `Decoder::accept` and `reject` are pump.rs:276-287 (:263-274 are
`is_closed` and `is_joined`). Fix: "(pump.rs:265-275; without `accept`" → "(pump.rs:276-287; without `accept`".

**A7, Low: carry 49's unit test is an addition the brief does not mark.** Carry 49 asks the refusal, not a test of it.
I keep the test (a refusal never shown to fire is not shown to work). Fix: "A unit test shows the refusal can fire:" →
"A unit test (this brief's: carry 49 asks none) shows the refusal can fire:"; and in section 2's preamble, "it adds
nothing it does not mark as its own (2.4 r6 and r7, 7.2's added records, 8.4's switch)," → "it adds nothing it does
not mark as its own (2.1's test of carry 49's refusal, 2.4 r6 and r7, 7.2's added records, 8.4's switch),".

**A8, Low: d2-r1's RR-L4 is not named where the `wasm` step's time is judged.** RR-L4 (d2-r1 :209-212) says that if
U4r's Evidence decision brings the step's time into question, the first remedy is to feed `replay_to` by chunks and
cache the hash per `step` in hash-wasm's State, before the step's runs move out of check.mjs. Fix: "Over 300 s, it comes
to the advisor as U4r's det-u4.mjs question" → "Over 300 s, it comes to the advisor as U4r's det-u4.mjs question, with
RR-L4's remedy (d2-r1 :209-212: `replay_to` fed by chunks and the hash cached per `step`, crates/hash-wasm, not your
lane) weighed first".

**A9, Low: the drafter's scratch copy by shell redirect (task item 8).** The drafter wrote plan-df.md into the
session's scratchpad by a shell redirect (its notes, B3a). I checked: it is outside the repository, its sha256 equals
`git show df8975b:.checkpoints/netcode-proof-test/u4-build-plan.md`'s (0d69332c…), it was only read, and the notes say
so at their top (df-ruling R6's form). It broke the write rule's letter (a `>` into a file, in any folder), changed
nothing in the repository and nothing the brief rests on. Recorded; nothing redone, no push. The brief's section 4
already carries the lesson ("in any folder, the session scratchpad included"); the builder's limits stand.

## Rulings on the open questions
Each ruling's words go into section 10's "Answered" table (a row each, its question kept as asked below it, for the
record) and into the places it names; where the brief says "by U4R-Qn", the ruling's words replace it.

**U4R-Q1 (npt-lab to npt-bots, npt-bots to npt-sim): (b).** The main session adds both edges in its own commit on
committed state before the launch (condition 10 (d)): `npt-bots.workspace = true` in crates/lab/Cargo.toml and
`npt-sim.workspace = true` (no features) in crates/bots/Cargo.toml, and nothing else. Its own counted run (main-run.sh,
`-x`) shows: Cargo.lock's diff holds only those two edges (no new package); check.mjs's 11 steps pass with the base's
counts; the deps step's npt-sim feature sets stay one set in SIM_BUILDS (check.mjs:1636-1640); and the `wasm` step's
seconds beside D3's 33.1 s warm, since npt-lab's two release builds (WASM_BUILDS) now carry npt-net's tree
(bots/Cargo.toml:24). If that tree puts the step's time in question, it comes to me first (one route then: npt-net
optional in npt-bots behind a feature its two binaries require). The builder's lane holds no manifest (section 4).

**U4R-Q2 (what is left): (c).** U4r's lane gains marks/SERIES.md for new rows only, one per name U4r writes that has
none (R26's list), each naming its writer; no existing row changes. Carry 62 (h)'s check in marks.json is the main
session's (the carry's "else" branch), after r2's evidence: if r2's measured-phase sum of `tick.s.snap_incomplete` is 0
on every short L1 P0 run, the main session adds the check and its bite entry to -17's L1 P0 part in its own commit on
the spike branch before the counted run of U4r's merge step, and shows the check can fail by running the judge (`--only
PM-0001-17`) on the builder's bitten short P0 logs (setup check FAIL by name); not 0 sends the cause to me first. The
bite's site is U4r's (npt-lab's client writer, bites build only), its id in the form `PM-0001-17/<name>`, named in the
report; marks.json's entry uses that exact spelling.

**U4R-Q4 (r2's scope): (a).** r2 covers -17's four profiles, -18's B1, -20's P3 under option B and -21's option-B
suites with PM-0001-2 passing and the shot band, the open-loop scripts standing in for the cheats' command patterns
(those suites' own checks not resolved), plus the carries' suites. This narrows my condition 5 at U4r to what can run
there and moves its option-A half, not the bar: -21's option-A suites and -20's option A must pass PM-0001-2 in U4f's
contract test before their marks count. The main session adds a carry line to u4s-carries.md §U4f (:491): "**U4R-Q4,
condition 5's option-A half (the advisor's U4r brief check):** U4f extends U4r's contract test (r2) to -21's option-A
suites and -20's P3 option A, with every PM-0001-2 check passing, not only resolving, and the shot band, before -20's
and -21's marks count; a suite that fails comes to the advisor first."

**U4R-Q6 (the wallclock bite's site): (a),** with one rule for the field. U4r's lane gains in npt-server, bites builds
only: `Decoder::with_bite(self, &str) -> Option<Decoder>` (a pub item beyond §4.1, named in the report under this
ruling); the site in `pump`, on that instance's decoded `Input`s; `#[allow(clippy::disallowed_methods,
clippy::disallowed_types)]` on the site alone (server/clippy.toml:6-17); `npt_server::BITES` widened to four, with
crates/server/tests/schedule.rs:347's pin updated. **The field rule:** the wall-clock read changes, on every command
the pump passes on for that leg, a field the World's hashed state takes at that tick (the command's move or aim), to a
value never equal to the received one (for example XOR with 1 | the clock's low bits), and never its tick, sequence,
slot, buttons, press counters or view time, which would give a VOID, a repeat or a refusal and show nothing. The bite
is shown on a pair that is not VOID, the comparer naming a tick (7.2). P4's replacement gains ", or in npt-server".
.claude/rules/netcode.md binds these edits. rust-reviewer and engine-architect read them.

**U4R-Q7 (-17's clean runs' `tick_start`): (a).** Every -17 L1 run, clean and bitten, starts at 63,736; -57's and
-18's shared runs with them. Replace "the clean runs' `tick_start`: U4R-Q7;" with "every -17 L1 run, clean and bitten,
at `tick_start` = 63,736 (U4R-Q7 (a));".

**U4R-Q8 (-18's runs): (a), shared.** -18's measured runs are -17's B1 runs, with -18's 10 bursts (offsets 0 to 1
tick) and headers listing -17, -57 and -18; each bite run lists its own mark only. Replace "its bursts: U4R-Q8" with
"its bursts are -18's (U4R-Q8 (a): -18's measured runs are these, their headers listing -17, -57 and -18)"; 7.3's row
"| -18 measured (B1 × seeds 1-3), if not shared with -17's B1 runs (U4R-Q8) | 3 × 600 s | 1,800 |" is removed (the
totals: U4R-Q9 below).

**U4R-Q9 (the TC B1 recording): (b).** U4r records SS P2 seed 1, ES P3 seed 2 and SS P0 seed 4; TC B1 seed 3 is U4h's,
made with its TC shooter. The real reason is the Rewinds: a TC recording without TC's shooter holds Rewinds the
shooter would not make, and U4h builds both the shooter and the rules they meet (plan :835-837). It also takes one
`scenario --record` out of the `wasm` step. Brief words:
- "**The four runner recordings**" → "**The three runner recordings at U4r**"; "`scenario --record` at SS P2 seed 1,
  ES P3 seed 2, TC B1 seed 3 (U4R-Q9), and SS P0 seed 4" → "`scenario --record` at SS P2 seed 1 and ES P3 seed 2, and
  SS P0 seed 4 (TC B1 seed 3 is U4h's, with its TC shooter: U4R-Q9 (b))".
- "(SS P2 seed 1, ES P3 seed 2, TC B1 seed 3, or two by U4R-Q9)" → "(SS P2 seed 1 and ES P3 seed 2 at U4r; U4h adds
  TC B1 seed 3)".
- "Its cost is about 3 × 9,000 / R s on top of the step's 33.1 s warm (R the ticks per second of 7.3: 300 s at R = 90,
  the whole 5-minute budget; 90 s at R = 300)" → "Its cost is about 2 × 9,000 / R s on top of the step's 33.1 s warm
  (R the ticks per second of 7.3: 200 s at R = 90; 60 s at R = 300)".
- "so the three about 3 × 9,000 / R s (300 s at R = 90; 90 s at R = 300), plus their replays;" → "so the two about 2 ×
  9,000 / R s (200 s at R = 90; 60 s at R = 300), plus their replays;".
- "the four recordings first (the pre-check needs SS P0 seed 4)" → "the three recordings first (the pre-check needs SS
  P0 seed 4)"; "The four recordings (or three, by U4R-Q9):" → "The three recordings:".
- 7.3: "| the four runner recordings | 4 × 300 s | 1,200 |" → "| the three runner recordings | 3 × 300 s | 900 |";
  "**15,600 s = 4.33 h = 468,000 ticks** (13,800 s = 414,000 ticks if -18 shares -17's B1 runs)" → "**13,500 s = 3.75 h
  = 405,000 ticks** (-18 sharing -17's B1 runs: U4R-Q8 (a))"; "**Wall time** = 468,000 ticks ÷ R" → "**Wall time** =
  405,000 ticks ÷ R"; the R table's rows → "| 300 | 10× | 1,350 s ≈ 23 min |", "| 90 | 3× | 4,500 s ≈ 1.3 h |", "| 30 |
  1× | 13,500 s ≈ 3.8 h |", "| 10 | 0.33× | 40,500 s ≈ 11.3 h |"; "the warm-ups add at most about 31 runs × 30 s = 930
  s" → "the warm-ups add at most about 27 runs × 30 s = 810 s".
- The plan text (the main session's commit, as my earlier rulings' plan edits; each old phrase found once): §4.15's
  "U4r's three runner recordings (`scenario --record`: SS P2 seed 1, ES P3 seed 2, TC B1 seed 3)" → "U4r's two
  runner recordings (`scenario --record`: SS P2 seed 1, ES P3 seed 2) and U4h's TC B1 seed 3, made with its TC shooter
  (the advisor's U4r brief check, U4R-Q9)"; §4.15's "The determinism step keeps its three; the fourth serves the
  pre-check only." → "The determinism step keeps the runner recordings (two at U4r, the third at U4h); SS P0 seed 4
  serves the pre-check only."; §5 U4r's "the four runner recordings (§4.15; the fourth, SS P0, for the -30
  pre-check: the advisor's condition 3)" → "three runner recordings (§4.15: SS P2 seed 1, ES P3 seed 2, and SS P0 seed
  4 for the -30 pre-check: the advisor's condition 3; TC B1 seed 3 is U4h's: U4R-Q9)"; §5 U4h's Builds: after "the tolerance-edge
  stamper and the 150 ms pong delayer;" add " the TC B1 seed 3 runner recording with the TC shooter, joining the
  recordings manifest (U4R-Q9);". Carry line for §U4h
  (:548): "**U4R-Q9, the TC B1 seed 3 recording (the advisor's U4r brief check):** U4h records TC B1 seed 3 with
  `scenario --record` and its TC shooter, adds it to crates/lab/recordings.json under carry 82's schema, and states
  the `wasm` step's time with it against the 5-minute budget."

**U4R-Q10 (showing the RTT rule can fire): (b').** r7 at U4r; (a') only in the re-run after U4h, and only if U4h's
brief states a timing in which it can fire.

**U4R-Q11 (negative cases for r2): (a).** Unit tests on doctored copies of a short run's logs, each failing r2 by name:
one `admit` event gone, one footer gone, one written name's SERIES.md row gone, one `late_count` series gone where
`late-zero` applies.

**U4R-Q12 (the re-run after U4h): (a), reading (i).** My "6 runs" (advisor :346) meant one recording and three pairs:
reading (i), with 8.3's replay-first rule for the recording, results in results/u4h-precheck-30/<commit>/, read in
U4h's review. -30's three-recording shape is U6's C part. Carry line for §U4h (:548): "**O1's re-run (carry 40; the
advisor's U4r brief check, U4R-Q12 (a)):** after U4h merges, the main session replays U4r's SS P0 seed 4 recording on
the U4h build and counts both legs' replacements; at 0 that is the clean pair, else SS P0 seed 4 is re-recorded on the
U4h build (its SHA-256 beside the old one's); then the three pairs (clean, wallclock, late-command), six runs, by U4r's
pre-check script, results in results/u4h-precheck-30/<commit>/, read by the advisor in U4h's review; -30 stays NOT-RUN
until U6. U4h's brief states what its estimate does with an L1 session whose `rtt_ns` is still `None`; the re-run
compares each client's first-sample instant with the instant U4h's estimate first reads that client's RTT (its receipt
or its run, both logged at U4r); the Loopback leg's sessions keep `RttSource::Link`."

**U4R-Q13 (carry 63): (a), bounded.** U4r's lane gains crates/log/src/lib.rs for carry 63's two refusals and their
tests in its test module (log/src/lib.rs:549) only: `read_log` refusing a JSON object with a duplicate key, and
`check_header` refusing a header text field holding a character for which `char::is_control` holds; each test written
to fail before its refusal and shown failing by name then (the builder's notes give the failing line), nothing else in
crates/log changed, and check.mjs's test step's judge fixtures still passing. security-auditor reads them (the seat
that raised FU-3 and FU-7). P4's replacement gains ", or in npt-log".

**U4R-Q14 (carry 74's lead and the duplicate stamps): (a1) amended by A3, and (b1) with A4's rule.** The arithmetic,
from the merged code: the endpoint stamps each command newest + lead when the page gives it (client/src/lib.rs:394) and
sends it in that step's `Input` (:429); the room counts late a command whose tick it has run or already buffered
(room/src/input.rs:107-113), after the forward compare (:81-89) and the press compare (:91-104); L1's delay each way
is d − a + U[0, 2a], with d half the round trip and a half the jitter (l1/src/lib.rs:215-221, :789-814); the pump
(RQ8 step 2) runs before the tick (step 3) at one instant. Let tick k run at t_k (P = 33.3 ms at 30 Hz; its snapshot
leaves then):
- **The lead.** With a delay above 0, a step at s whose newest is n has not received n + 1, so s < t_{n+1} + d + a;
  its command, stamped n + L, arrives before t_{n+1} + rtt + jitter, and is in time if that is at or before t_{n+L}
  (an arrival at t_{n+L} itself is pumped before the tick). So (L − 1)·P ≥ rtt + jitter holds every command at every
  phase and every draw: L = ⌈(rtt + jitter) / P⌉ + 1, the brief's `ticks_for_ms(rtt_ms + jitter_ms) + 1`: P1 3, P2
  and B1 5 (120 / 33.3 = 3.6), P3 7 (180 / 33.3 = 5.4). It is the worst case, not the least lead the check's model
  found clean (P3 at 6); I keep it because it has no tail. At P0, A3: lead 2. **The ruled lead:**
  `npt_sim::ticks_for_ms(rtt_ms + jitter_ms).max(1) + 1`: P0 2, P1 3, P2 and B1 5, P3 7, each within `CMD_CATCHUP` = 8.
- **Carry 74's own formula contradicts its premise.** At lead 1 a command is in time only if 2d plus the step's wait
  fits one tick period, so "every command late" sets in when the round trip passes a tick period, which is the carry's
  own words ("one-way delay above about half a tick period"). The arrival offset 2d + the step's wait ± the jitter
  spans about 80 to 153 ms at P2: lead 3 (100 ms) leaves most commands late, lead 4 (133 ms) a tail, lead 5 (167 ms)
  none, as the check's model found (97.7 %, 2.3 %, 0).
- **Losses and bursts at the ruled lead** (arithmetic; r2 reports them, nothing judged here): a command whose first
  `Input` is lost goes again in the next step's `Input` (the endpoint keeps the unacknowledged ones up to the
  redundancy cap: client/src/lib.rs:407-408, :429), one period later, and is late only in the tail where its arrival
  offset passes (L − 1)·P: about 2 % of resends, so about 0.05 % of P2's commands (2 % loss) and 0.1 % of P3's (5 %).
  A B1 burst (1 s of total loss both ways) under (b1) loses only the commands given while the snapshots sent before it
  still arrive, about (d + a) / P ≈ 2 per client, resent after it and late: about 20 a client over 10 bursts, about
  0.1 % of a 10-minute run. So carry 74's 1 % rule can hold on every U4r profile, and D13 and r2 show whether it does.
- **The duplicates (b) hold as the revision says.** A step inside a snapshot's arrival window (as wide as `jitter_ns`)
  sees that snapshot arrived with probability p, by a fresh draw each step, so two steps in a row read one newest with
  probability p(1 − p); over phases spread evenly, `jitter_ns` / P × 1/6: about 10 % at P2 and B1, 15 % at P3, 5 % at
  P1; at P0 only through a seeded clock rate other than 1. Such a second command is late (input.rs:110), or, if a press
  counter moved, refused (the press compare runs first and covers 0 ticks: :91-99), so (b2) would mix two counts, and
  under a B1 burst every step's command would carry one frozen stamp. (b1) leaves the stamps strictly rising, so a
  command is late only by its arrival, and the late count stays the link-timing signal carry 74 means.
- **A4's rule with (b1):** the bots count press edges per command given, never per step; a press wanted at a step
  without a command waits for the next command, at most one new edge per counter per command (input.rs:91-99).
- **Carry 74, amended by this ruling** (the main session's commit to u4s-carries.md; the carry is my R3's, and P1 then
  copies the amended words): "(its largest one-way delay, half the round trip plus half the jitter, in ticks rounded
  up, plus 1)" → "(the round trip plus the jitter, in ticks rounded up, at least 1, plus 1:
  `npt_sim::ticks_for_ms(rtt_ms + jitter_ms).max(1) + 1`, so P0 2, P1 3, P2 and B1 5, P3 7, within `CMD_CATCHUP` = 8;
  its bots give a command only at a client step whose newest snapshot tick moved since their last command, and move
  each press counter by at most one edge per command; the advisor's U4r brief check, U4R-Q14)". Its 1 % rule and its
  report keep their words. This tightens nothing and loosens nothing: the bar stays 1 % of the commands sent.
- **Brief words.** 2.1: replace the span from "- **The lead** (carry 74, by U4R-Q14):" to "Either way r2 reports each
  short run's late count beside the commands sent (carry 74)." with "- **The lead** (carry 74 as U4R-Q14 amended it):
  each endpoint's `with_lead` (client/src/lib.rs:291-295; 1 by default, :262) from the run's profile,
  `npt_sim::ticks_for_ms((rtt_ns + jitter_ns) / 1,000,000).max(1) + 1` (sim/src/ticks.rs:35, rounding up: carry 39;
  every profile is in whole ms, l1/src/lib.rs:166-175): P0 2, P2 and B1 5, P3 7, each within `CMD_CATCHUP` = 8
  (room/src/input.rs:59-61), recorded beside `cmd_timing` in the header. It covers the round trip, the jitter and one
  client step at every phase, and at P0, where a packet is never delivered in its send instant, a step on a tick's own
  instant (U4R-Q14). The bots give no command at a client step whose newest tick did not move (2.2). r2 reports each
  short run's late count beside the commands sent and the steps without a command (carry 74)." 2.2: after "and r2
  reports per run the steps without a command beside the commands sent." add " A command's press counters move by at
  most one edge each since the bot's last command (the room refuses more edges than the ticks a command covers:
  room/src/input.rs:91-99), so a press wanted at a step without a command waits for the next command (U4R-Q14)." 3.2's
  row 74: "U4R-Q14 asks the advisor before launch, and the brief rewords no rule" → "U4R-Q14 ruled: the carry's lead
  amended to the round trip (P0 2), with the bots' step and press rules; its 1 % rule unchanged".
- **D13, widened:** replace the span from "- **D13, the lead and the duplicate stamps on a short P2 run" to "1 % at
  the ruled lead stops the build's recordings and comes to the advisor (carry 74)." with "- **D13, the lead and the
  duplicate stamps on short runs (stays; the builder's, before any `scenario --record`):** the brawl (SS, 48 clients,
  seed 1; one length for all, in the notes), each run under the runner (`-x`): (1) P2 at lead 5 under 2.2's rules;
  (2) P2 at lead 5 with a command at every step (a test-only setting, never a judged one: the duplicates' share, about
  10 % by U4R-Q14's arithmetic); (3) P2 at lead 3 (carry 74's old formula) under 2.2's rules; (4) P3 at lead 7 under
  2.2's rules; (5) P0 at lead 2 under 2.2's rules. The notes give per run the commands sent, the room's `late`,
  `press_refused` and `buffer_full` counts (`ClientCounters`, room.rs:118-122) with the late share, the commands stamped
  for a tick already stamped (`newest_tick()` + lead after each call), the steps without a command and
  `rewinds_refused`. Expected: (1), (4) and (5) at most 1 % late with 0 refused; (3) most commands late; (2) reported
  against the arithmetic. Its exact command lines cannot be written before the build (as D4). A late share above 1 %,
  or any press refusal, in (1), (4) or (5) stops the build's recordings and comes to the advisor (carry 74)."
- Wherever else the brief says "P0 1" for the lead (2.1's lead bullet, U4R-Q14's text, kept as asked), the fold reads
  the ruled numbers.

**Section 4's lane, as these rulings leave it.** Replace the bullet from "- **Waiting on a ruling, so not yours until it
comes:**" to "A part that needs one stops, goes in your report, and the rest is built." with "- **Opened by the
advisor's rulings (advisor-u4r-brief.md):** marks/SERIES.md, new rows only, for the names U4r writes (U4R-Q2 (c));
crates/log/src/lib.rs for carry 63's two refusals and their tests only (U4R-Q13 (a)); in npt-server, the wallclock
bite's items only (U4R-Q6 (a): the site in pump.rs, `Decoder::with_bite`, `npt_server::BITES` widened to four, and its
pin in crates/server/tests/schedule.rs:347; loopback.rs:347's one-home test reads the list whole). **Not yours:**
crates/lab/Cargo.toml, crates/bots/Cargo.toml and Cargo.lock (the main session's commit before launch: U4R-Q1 (b));
marks.json (carry 62 (h)'s check is the main session's: U4R-Q2 (c)). A part that needs anything more stops, goes in
your report, and the rest is built." In "Never": "crates/server (but U4R-Q6 (a)'s items, if ruled)," → "crates/server
(but U4R-Q6 (a)'s items),"; "crates/log (but U4R-Q13's), crates/judge," → "crates/log (but carry 63's two refusals:
U4R-Q13 (a)), crates/judge,"; "crates/tcpinfo; marks/ (but U4R-Q2's); docs/;" → "crates/tcpinfo; marks/ (but
SERIES.md's new rows: U4R-Q2 (c)); docs/;". P4's replacement: "npt-lab or npt-bots, or in npt-log, or in npt-server, in
check.mjs's deps, wasm or marks step, or in check.test.mjs" (both of section 12 item 3's optional clauses ruled in; the
marks step because SERIES.md's new rows are U4r's: U4R-Q2 (c)); section 12 item 3's P4 line is reworded to that one
string, and its assert checks it. Section 6
item 12's out-of-lane needs: "(U4R-Q1, Q2, Q6, Q13)" stays as the report's list of what it met through those rulings.

**The review panel (section 12 item 7): confirmed.** netcode-architect, test-engineer, engine-architect (the public
items, `Decoder::with_bite` among them), rust-reviewer (the npt-server and npt-log edits included), security-auditor
(carry 76's session boundary, carry 85's refusals, carry 63's two refusals, the wall-clock read in server code) and
infra-engineer (carry 84 and the `wasm` step's time against its budget), at xhigh, at most three agents at a time with
the builder counted, so two batches of three, one round. **The judged runs' target (item 8): confirmed** as written.

## The lane runner u4r-run.sh: approved
My own `diff u4d2-run.sh u4r-run.sh`: 6 lines, only the lane's names (the header :2-3, the log folder :9, the usage
line :16, the log case :41-42). The CACHEDIR.TAG check (:46-49), `exec 9>` and `flock "$mode" 9` (:52-53) and the
provenance block are byte-equal to u4d2-run.sh's. u4r-run.sh.diff equals my diff; u4r_runner.py asserts exactly that
(its keep list, one-line finds, lines of at most 120). sha256 349b51ad…, `bash -n` exit 0. The brief's cites
u4r-run.sh:12-15 and :18-22 hold. No finding. At launch it needs its sha256 and `bash -n` only (section 12 item 2).

## INF2-4's runner follow-up (my M9 Block 2): met
- **The two diffs against Block 2's lines.** main-run.sh.inf2-4.diff is Block 2 :157-171 exactly (S, T, LOCK=$T, the
  exports, the log, the shift, the tag check with its `fi`, mkdir, cd, `exec`, `flock -x 9`), the header sentence of
  :184-186 after "...exits with that command's code.", and MR9-L1's provenance form (:179-182).
  u4d2-run.sh.inf2-4.diff is :188-198 (T, LOCK, the export; the tag check between `[ -d ]` and mkdir; `exec` and
  `flock "$mode" 9`) and the header sentence after "...first non-zero code."; its provenance untouched (already
  anchored). No other line. The blobs (d9ce0e1, 0a5a587) are the diffs' post-images, committed at 90f5f61.
- **P1:** I re-ran `bash -n` on all nine scripts: exit 0 each.
- **P3:** .gate/u4d2-merge/inf2-4-p3a.log (`-x`) runs 11:01:18 to 11:01:26, exit 0, its provenance block empty
  (MR9-L1 fixed); .gate/u4d2/inf2-4-p3b.log (shared) was held until 11:01:26 (its mtime 0.45 s after p3a's), exit 0.
  The criterion holds. Not shown by the logs: p3b's start instant (the 6 s wait is the card's account).
- **P4:** the six one-line copies and their .diff files read: one line each, as Block 2's "small script" allows
  (inf2_4_proof.py asserts one line). I re-ran main-run.sh's (a) and (b) with their logs under .gate/u4r-brief-check/:
  both exit 5 with the STOP line, no marker and no log, the missing folders still absent ((b) made the empty
  .gate/u4r-brief-check/ by its `mkdir -p`). (c) not re-run: it opens the live lock file, which a check may not touch
  while other lanes could hold it.
- **The card's record** ("INF2-4's runner follow-up: DONE") matches the files and logs.

## TE2-5 (c): what the brief's TE-M3 line must carry
A1's words: the commit c49773f; the cause in one line; TE-Q3's quoted words (P16, advisor-te-m3.md:136-139) word for
word; TE-Q2's end (at U4r's merge, and only if no check.mjs test-step failure in U4r's counted and judged runs; else
the rule continues and the failure comes to me); every failure of the test, polls = 4 included, whole to me, never a
retry; the test's lines at the head. The brief is edited only now that this check has returned
(advisor-te-m3-check.md, item 2's last sentence).

## Conditions before the launch (the main session's)
- **L1. Fold this ruling into the brief by script (rev 3.2):** A1, A2 and A5 to A8 as written; the twelve rulings with
  their brief words (Q7, Q8, Q9, Q14 and section 4's lane as written; the others as section 10's table rows); section
  10's "Answered" table gains the twelve; the fold recorded under the brief's title. Each old phrase is asserted found
  once with its line breaks read as one space (I counted all of them: once each). If a phrase fails to match, or the
  fold must change any other word, it comes back to me; otherwise there is no second check.
- **L2. Carries and plan text, in the main session's own commit, before the launch:** carry 74's amended words
  (U4R-Q14); the §U4f line (U4R-Q4); the two §U4h lines (U4R-Q9, U4R-Q12); the plan's §4.15, §5 U4r and §5 U4h edits
  (U4R-Q9). Each old phrase found once. Then section 12 item 9 runs: section 0's stamps re-set for the plan and
  carries, and every cite of the brief into them whose line moved re-pointed by script, each asserted.
- **L3. The manifests (U4R-Q1 (b)),** on committed state, with its counted run's results in the card: Cargo.lock's two
  edges only, 11 steps with the base's counts, SIM_BUILDS one set, the `wasm` step's seconds. Then `<base>` is that
  commit or later, and section 12 item 1 takes its counts from that counted run.
- **L4. The placeholders and values (section 12 items 1 to 3, with P16):** each anchor found once, every closed file's
  sha256 asserted first, and P4 with both clauses.
- **L5. The card:** "Rules in force" keeps TE2-5's rule until U4r's merge (TE-Q2), and the U4r brief's entry names this
  ruling and rev 3.2.
- **L6. Launch as section 1 says:** one builder (general-purpose, opus, xhigh) in its own worktree from `<base>`, under
  W1-TARGET's lock and u4r-run.sh; at most three agents at a time, reviewers and the builder counted.

## What the main session does next, in order
1. Read this ruling; record it on the card (L5).
2. L3: the manifest commit and its counted run.
3. L2: the carries and plan text commit; re-stamp section 0's plan and carries.
4. L1: fold this ruling into the brief (rev 3.2), then section 12 items 1 to 3 and 9 (L4) on that text.
5. L6: launch the builder. D13 runs first in the build, before any `scenario --record`.
6. After the build: the panel above in two batches; my merge conditions follow U4d2's M1 to M9 form; then Q2 (c)'s
   marks.json commit if r2's (h) sums are 0, before the merge step's counted run.

## What I checked, and what I did not
- **Checked, by reading:** the brief whole and its change lists; plan §5 U4r and the cited lines; carries §U4r, 6, 14,
  20, 49, 63 and 74; my rulings named above; the code at the head for every cite in check 1, the -57 route, the -18
  record, the view time, the RTT rule and Q14's arithmetic (client :379-456, input.rs :20-150, room.rs :60-140 and
  :785-800, pump.rs :224-303, l1 :55-70, :160-225, :789-814, :1667-1681, sim ticks.rs, check.mjs :879-945, :1625-1660,
  :2015-2045); the placeholders' anchors and every fold phrase (print-only scripts from stdin).
- **Checked, by running (read only):** sha256 of every stamped file; git ancestry and diffs; `bash -n` on the nine
  scripts; main-run.sh's INF2-4 copies (a) and (b), logs under .gate/u4r-brief-check/.
- **Not checked:** I built, tested and timed nothing. Q14's numbers are arithmetic from the code under the model's
  assumptions (one datagram per snapshot, steps at a fixed phase, uniform draws), not a run: D13 measures them before
  any recording. The cost of npt-net's tree in npt-lab's builds is L3's counted run's to show. I did not re-run INF2-4's
  (c) (the live lock file).

## Decisions needed (Mustafa)
None. Every point is a technical or test choice inside the plan, a lane between units, or the text of my own conditions
and carries (condition 5's option-A half moved to U4f with its bar; carry 74's lead and its bots' rules). No rule or
pass bar is loosened beyond his words, and nothing he sees, hears or feels is touched.

## For Mustafa
Nothing. (PR #40 stays needs-mustafa for its existing reasons; this ruling adds no action for you.)

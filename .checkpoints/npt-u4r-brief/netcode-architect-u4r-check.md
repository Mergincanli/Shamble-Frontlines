# netcode-architect check of the U4r builder brief, rev 1

Night item npt-u4r-brief, step 2 (plan: 2dc1f31's night-2026-11-10.md, "Item 4"). Scope, in the plan's words: "the -30
pre-check part, O1, the RTT argument and -18's fresh-delta reading". Draft: u4r-brief-draft.md rev 1 (785 lines). Every
source file:line is at df8975b, under .gate/npt-spike/ (`plan` = u4-build-plan.md, `advisor` = advisor-u4.md,
`carries` = u4s-carries.md, crate paths under spikes/netcode-proof/crates/). Notes only: no build, no run, no web.

Severity scale: Critical, High, Medium, Low (CLAUDE.md). The night's rule 5: a Medium or Low here is for rev 2 (the
drafter's single revision); none is carried yet (`u4s-carries.md` holds none of them).

---

## 1. The -30 pre-check (draft §7.2 :405-445, and 2.1 :87-88, :108-113, :126, 2.5 :199-200, 7.1 :398, 7.4 :492-494)

Checked against plan :740-764, §4.15 (:462-487), §4.6 (:296-318), §4.9 (:333-347), advisor conditions 2 and 3
(:90-111), Q10 (:208), marks.json -30 (:6325-6390), ADR 0001:1156-1162 (option B) and :1312-1315, and the judge's
expect-invalid rule (judge/src/eval.rs:1283).

**Carried exactly (confirmed):** suite `precheck-30`; the two legs, layer `loopback` (README §1 via U4j, plan :633-635)
and layer `L1` at P0; replay clients re-encoding the SS P0 recording's commands as `Input` with the recording's stamped
ticks, view times, sequences and press counters and the live ack (plan :741-743; §4.15 :472-475); the room computing
its own Rewinds; option B with the buffer deep enough for P0 and both legs' depth recorded (plan :742-743; condition 3,
advisor :111); the comparer is U4d2's `HashCompare` over the two legs' per-tick server hashes, naming the first
differing tick (plan :744-745, :652-654); `tick.v.late_count` summing to 0 on both legs (condition 2, advisor :93-94);
the setup condition with per-shot logging and its VOID rule, "never a pass or a fail" (plan :746-748); the two bites
and their outcomes (plan :759-760); the headers listing no mark, the result folder results/u4r-precheck-30/<commit>/
with runs.txt and the comparer's output, read by the advisor in U4r's review, nothing into marks.json (plan :761-763;
advisor :96-98); "-30 stays NOT-RUN until U6: never reported as -30's part, never in U6's folder" (plan :764, draft
:445). Seed 4 only, with the reason (one recording; "3 seeds per part" is for parts: draft :398) is right: the pre-check
is not a part (Q10, advisor :208). The late-command outcome rule (VOID by `late_count` alone, PASS or FAIL = missed:
draft :435-438) follows marks.json -30's bite (`"expect": "invalid"`, `"rule": "late-zero"`, :6380-6383) and the
judge's rule (eval.rs:1283, quote checked); it sharpens the plan's "never a pass" correctly (a FAIL means the VOID rule
did not fire), so it is not scope creep. Recording the depth under marks.json's name `buffer_depth_ticks_count`
(marks.json:6368-6372) while the plan writes `buffer_depth_ticks` (:743), with recheck row R27, is a sound choice:
U6's -30 C part reads marks.json's name.

**Additions beyond the plan (scope creep, listed):**
- "Both legs run one room configuration (the same depth, the same HitHistory span, the same seed), recorded in both
  headers" (:413-414): implied by the comparison, harmless; keep, and add the room's tick base (N6).
- The out-of-ring count logged per leg and reported (:424-426): not in the plan; harmless (a log, not a rule), and its
  reasoning is sound (with `late_count` 0 both legs run each command at its stamped tick against rings built from equal
  states, so `rewind_at`'s out-of-ring answer cannot differ). Mark it as an addition (N16).
- 8.4's switch (the script reads the replacement and clip series by name, VOID if a room log lacks them after U4h):
  labelled "this brief's proposal"; fine as a proposal for the advisor.

### Findings

**N6 (Medium): the replay clients' pacing and the room's tick base are unstated, so `late_count` 0 is left to chance.**
- Draft :407-414 (and 2.1 :126) say what the replay clients send, never when; plan :741-743 is silent too; the recording
  holds per-tick commands and no send instants (§4.15 :463-466). Option B runs exactly one command per client per tick,
  stamped tick, late commands dropped (ADR 0001:1160-1162), and a pair counts only if every command ran at the tick
  stamped on it with the late counter at 0 (ADR 0001:1312-1315). The replay clients have no `ClockSync` and no
  tick-lead controller (both U4f: plan :774).
- Scenario: the builder sends each recorded command on arrival of its tick's snapshot, or starts the replay room at
  tick 0 while the recording's first stamped tick is the recording run's `tick_start` plus the join phase; on the L1
  leg (P0: next-instant delivery, l1/src/lib.rs:30-31) a share of commands lands after its tick, `late_count` > 0, and
  every pair, clean and bitten, comes out VOID: no evidence, and the wallclock bite never shows.
- Fix for rev 2: under "plan silent", ask the builder to state and test (a) the room's tick base on both legs (the
  recording header's start tick, recorded in both headers beside the depth), and (b) the pacing rule: when a replay
  client sends the command stamped T (for example once its decoded server tick reaches T minus the depth plus one, or
  at the SimClock instant tick T minus the depth is due), why it lands before tick T on both legs at P0, and how
  redundancy covers it; and a short-run unit test that a clean replay gives `late_count` 0 on both legs.

**N7 (Medium): "the per-tick server hashes" is not pinned to the sim's state hash.**
- Draft :417-418 and 8.2 item 3 (:534-535) keep the plan's words; but 2.1 :100-101 has the room log also write "the
  server's mirror" hash per (client, tick) (§4.6 :300-302), which depends on baseline B, the window's remainder and the
  priority accumulator (an entity skipped at B's value), all driven by the leg's acks and bytes (§4.5 :283-293).
  marks.json -30's check is "identical server state hashes on every tick" (:6361).
- Scenario: the builder feeds `HashCompare` the mirror hashes (the per-tick hash list the room log already holds per
  client); an ack arriving one instant later on L1 changes one client's priority skip, its mirror hash differs from the
  Loopback leg's, and a correct pair reads FAIL.
- Fix: "the two legs' per-tick server state hashes: npt-sim's canonical per-tick hash of the room's `World` (U4d's
  FNV-1a-64, the hash D-2 compares), one per tick in each leg's room log; never the mirror hash per (client, tick)."

**N8 (Medium): the wallclock bite's read must reach a sim input by a path that is neither the RTT nor the late test,
on every run; a bitten pair that comes out PASS is not named.**
- Draft :432-434 (and 2.5 :200, 8.2 item 3 :537) say the bite is shown "only on a pair that is not VOID, with the
  comparer naming a first differing tick" and that "a wallclock bite that comes out VOID shows nothing"; U4R-Q6
  (:696-704) asks only who plants it. Plan :759: "a wall-clock read in the pump for one leg's sessions ... must give a
  mismatch". At U4r the pump's outputs that reach the sim are only the commands it lets through (the Input bucket, the
  Join deadline, decode); its RTT reaches nothing (draft 8.2 item 2) and the window feeds the encoder only.
- Scenario: the builder puts the read in the pump's RTT or ping path (the natural "clock" sites): nothing reads it at
  U4r, the bitten pair is PASS, and the record shows a missed bite where the bite never reached the sim. Or it puts it
  in the bucket's refill: whether the bucket starves depends on the machine's ticks per second R (7.3), so the bite
  shows on one machine and not on another.
- Fix: in 7.2 and U4R-Q6, "the read must change a sim input on that leg on every run, whatever R is (for example a
  command field or the accepted-command set derived from the wall-clock value), never through the RTT (nothing reads it
  at U4r) or through lateness (that makes the pair VOID, which shows nothing); a bitten pair that comes out PASS is a
  missed bite, reported, never re-planted into a fail without saying so."

**N11 (Low):** draft :415-416 "which it acknowledges at once" reads as a replay-client message; plan :753-755 means L1's
transport acknowledgement of JoinAccepted's packet (U4c's immediate acks), and Loopback has none. Reword: "whose packet
L1 acknowledges at once (U4c's immediate acks; nothing the replay client sends)".

**N13 (Low):** the late-command bite (draft :435, 2.5 :199) must hold the delayed command out of every `Input` that
carries it, redundant copies included, until its tick has run on that leg; otherwise the next `Input`'s redundancy
delivers it in time, `late_count` stays 0, and the bite is not exercised. Add that, and that the bite run's records
state the delayed command's tick and the leg's `late_count` (expected exactly 1).

**N16 (Low):** list the out-of-ring logging (:424-426) as an addition beyond the plan, with its reason (above).

Block 1 done.

---

## 2. O1 and carry 40 (draft §3 :229-244, §8 :504-577, 7.2 :439-441, U4R-Q12 :751-758, N7 :780-781)

Checked against carries :176-182, advisor :343-347, u4-conditions-check.md:42-45, plan :690-691 (U4e) and :799-800
(U4h), and marks.json -30 (:6336-6389).

**Confirmed correct:**
- Carry 40 is quoted word for word (draft :234-240 against carries :176-182, heading included), and it is the only
  entry in the file's U4r section (carries :176-182; the U4h section starts at :184).
- The statement holds at df8975b by the plan's own lines: U4e builds "`rewind_at` (the integer half; the tolerance and
  cap rules land in U4h)" (plan :690-691) and U4h "`HitHistory`'s sizing and `rewind_at`'s rules (§4.9); the view-time
  estimate with the pump's RTT; the switchable cap; the per-shot log of its parts" (plan :799-800). The draft's quotes
  (:508-510) are exact.
- The stale cites are reported correctly: advisor :344 cites "u4-build-plan.md:678-679, :787"; at df8975b :678-679 are
  U4d2's Bites (the std `sin` ... the hash-map) and :787 is U4g's `Predictor` line; u4-conditions-check.md:42-45 cites
  "U4e :678-679" and "U4h :787"; advisor :343's "u4-conditions-check.md:39-42" is :42-45 at df8975b (:37-41 are step
  5's edits-file line). Both plan cites moved by exactly 12 lines (678 to 690, 787 to 799), which fits the draft's
  unverified guess (the U4s ruling's 12 condition-1 edits, advisor :386-388, D-4's new text sitting in §5 U4d above
  U4e); it is labelled as a guess, which is right. Carry 40 cites by section, so its words need no change (correct).
- 8.2 item 1 (a check that cannot fail is evidence of nothing: advisor :352-353) and the records' wording "not built: 0
  by construction at U4r" (draft :244, :423, :527) are faithful to O1 ("holds by construction") without claiming the
  setup held.
- The re-run's case (8.3) is sound: U4h builds exactly the estimate, the tolerance, the cap and the per-shot log
  (plan :799-800); U4h depends on U4g, which depends on U4f (plan :773, :785, :798), so a re-recording on U4h's build has
  clients with `ClockSync` and `Interp`. The replay-first step (draft :557-562) is sound and, if anything, understated:
  U4r's clients stamp view times without an announced interpolation delay, while U4h's estimate subtracts one
  (§4.9 :340-341), so replacements on the old recording are likely, not merely possible. Alternatives (b) and (c)
  (:565-568) are argued correctly (condition 2 makes the pre-check U4r's: advisor :97).

### Findings

**N4 (Medium): "6 runs" has a second reading the draft does not weigh.**
- The draft reads the advisor's "re-run the pre-check pair after U4h merges (6 runs)" (advisor :346; carries :181) as
  three pairs on the one recording: clean, wallclock, late-command (7.2 :439-441; 8.3 :554; U4R-Q12 :751-753). But
  -30's own part has the shape "3 recordings of 5 minutes ... each replayed over [both transports] at P0 ... so all 6
  runs are measured" (marks.json:6389; parts `min` 6: :6344, :6353). "The pre-check pair ... (6 runs)" reads as that
  shape too: three clean pairs, six measured runs, the bites on top. The plan gives the pre-check one P0 recording
  (§4.15 :479-484), so that reading needs two more P0 recordings.
- Scenario: the advisor meant three clean pairs; the brief's re-run gives one; U4h's review finds the evidence short and
  two `scenario --record` P0 recordings plus their pairs are added after the fact.
- Fix: U4R-Q12 names both readings with their cost: (i) the draft's, one recording, three pairs, 6 runs, about 1,800 s
  simulated; (ii) -30's shape, three P0 recordings (two more recorded on the U4h build), three clean pairs plus the two
  bite pairs, 10 runs and 2 recordings, about 3,600 s simulated; with a recommendation and its reason (for example (i):
  the pre-check separates the room's command path from the link, which one P0 recording tests; the three-recording
  shape is -30's C part at U6). 7.2 :439-441 says "the advisor's '6 runs' is ambiguous; U4R-Q12" rather than adopting
  one reading.

**N5 (Medium): O1's "no reader of the RTT, no replace or clip" rests on the plan only; nothing checks it against U4e's
merged code.**
- Draft 7.2 :422 ("At U4r the room has no code that replaces or clips") and 8.2 item 2 :531 ("nothing in the room reads
  the RTT") cite the plan's split of work. At df8975b there is nothing to check: crates/room/src/lib.rs:1-5 and
  crates/server/src/lib.rs:1-5 are module docs only. U4e's pump computes the per-client RTT (plan :693-694), and the
  recheck table asks only whether the lab can read it (R15) and how the out-of-ring answer is counted (R19).
- Scenario: U4e's merged code reads the RTT somewhere (an event resend or Resync timer, a Join-deadline margin), or its
  integer `rewind_at` answers a view time outside the ring by substituting a tick instead of giving no Rewind; the
  statement carry 40 sends to the advisor is then false at U4r, and the "0 by construction" records are wrong.
- Fix: a recheck row R33, "every reader of the pump's per-client RTT and every replace, clip or substitute path in
  npt-room and npt-server at `<base>`: none outside the pump's own estimate and logs", with its command in §11:
  `git grep -n -i -e rtt -e round_trip -e srtt <base> -- spikes/netcode-proof/crates/room/src spikes/netcode-proof/crates/server/src`
  and `git grep -n -i -e replace -e clip -e tolerance -e estimate <base> -- spikes/netcode-proof/crates/room/src`, each
  hit read; any reader takes O1's statement back to the advisor before U4r's build.

**N14 (Low):** 8.3's cost (:563-564: "six runs ... plus 300 s if re-recorded") leaves out that the replay-first step is
itself the clean pair (N7 :780-781). If it shows replacements, that pair is VOID and is run again after the
re-recording: 2 + 1 + 6 = 9 runs, about 2,700 s simulated, not 2,100 s. State both branches (6 runs if no replacement;
9 runs otherwise).

The early-input bite that would make the re-run 8 runs (U4R-Q10) is checked in section 3 (N2).

Block 2 done.

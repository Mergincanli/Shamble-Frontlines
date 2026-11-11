# netcode-architect check of the U4r builder brief, rev 1

Night item npt-u4r-brief, step 2 (plan: 2dc1f31's night-2026-11-10.md, "Item 4"). Scope, in the plan's words: "the -30
pre-check part, O1, the RTT argument and -18's fresh-delta reading". Draft: u4r-brief-draft.md rev 1 (785 lines). Every
source file:line is at df8975b, under .gate/npt-spike/ (`plan` = u4-build-plan.md, `advisor` = advisor-u4.md,
`carries` = u4s-carries.md, crate paths under spikes/netcode-proof/crates/). Notes only: no build, no run, no web.

Severity scale: Critical, High, Medium, Low (CLAUDE.md). The night's rule 5: a Medium or Low here is for rev 2 (the
drafter's single revision); none is carried yet (`u4s-carries.md` holds none of them).

**Verdict: REVISE: 0 Critical, 0 High, 8 Medium (N1, N2, N4, N5, N6, N7, N8, N9), 8 Low (N3, N10-N16).** The four parts
are carried faithfully in substance; the Mediums are places where the words leave a builder free to produce a pair or
a bite that shows nothing, plus one ambiguous count. Summary of what is confirmed: section 5.

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
  clients with `ClockSync` and `Interp`. The replay-first step (draft :557-562) is sound: U4r's clients have no `Interp`
  and no ClientSettings clamp (U4g: plan :789), while U4h's estimate includes the announced interpolation delay (§4.9
  :340-341), so replacements on the old recording are plausible, and the step decides it by a count. Alternatives (b)
  and (c) (:565-568) are argued correctly (condition 2 makes the pre-check U4r's: advisor :97).

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

---

## 3. The RTT argument (plan :749-758; draft 7.2 :427-431, 8.2 items 2-3 :529-541, 8.3 :549-553, U4R-Q10 :732-741)

Checked against link/src/lib.rs (the trait :79-81, Loopback :129-214), l1/src/lib.rs (the crate doc :30-38, `Rtt`
:348-368, `launch` and `advance_acks` :512-532, `rtt_ns` :784-785, profiles :115-139), plan §4.9 (:333-347), U4c
(:552-583), U4e (:685-712) and the RQ8 order as the draft gives it (2.1 :74-78).

**Confirmed correct:**
- The facts: Loopback's `rtt_ns` is `Some(0)` always (link/src/lib.rs:203-205; its test :282); U4c's starting state is
  333 ms and 166.5 ms variance before the first sample, which the first sample sets (plan :555-557). At df8975b, which
  predates U4c, L1's `rtt_ns` is `None` before the first sample (`Rtt::smoothed` starts `None`: l1/src/lib.rs:349-353,
  :358-361; returned at :784-785), so R2's question (Some(333 ms) or None at `<base>`) is the right one.
- The argument itself is sound. `rewind_at` has three outcomes, accept (a Rewind from the view time), replace by the
  estimate, or clip to the cap, the last two counted (§4.9 :337-339); only the estimate and the cap's test read the RTT
  (:340-345). With 0 replacements and 0 clips, each Rewind is a function of the view time and the ring, so the RTT can
  split the legs only through a replacement or a clip, which the setup condition makes VOID. The draft states it
  faithfully (8.2 item 2 :529-533).
- The ordering premise holds at P0 with the replay clients' wait: JoinAccepted is the server's first packet to that
  client; L1 starts its implicit ack when it launches the packet (l1/src/lib.rs:512-521), at P0 both ways draw 0 delay
  (:115-117), and an end reads an ack once its clock reaches it (:33-34, :526-532). A client that waits for JoinAccepted
  sends its first `Input` at or after JoinAccepted's arrival instant, so the `Input` reaches the server at a later
  instant, when the server end has read the sample (L-c6 orders same-instant internal events first: plan :571), and
  under option B the command runs later still. The draft's 8.2 item 2 ("holds trivially and is untested at U4r",
  nothing in the room reading the RTT before U4h) is right, subject to N5's recheck at `<base>`.
- The VOID-per-leg rule, its binding on the L1 leg only, and the re-run's account of why the argument only means
  something after U4h (8.3 :547-553) are faithful to plan :749-758.

### Findings

**N1 (Medium): the RTT rule drops "the server's" from the sample it times.**
- Draft 7.2 :427-429: "the runner logs per client, on each leg, the SimClock instant of its link's first RTT sample".
  Plan :755-757: "so the server's L1 link holds its first sample ... The runner logs per client the instant of that
  first sample". Each L1 link has two ends with their own estimates (l1/src/lib.rs:458, :529: `rtt[end]`), and the room
  reads the server end's (§4.9 :341-342).
- Scenario: the builder logs the client end's `rtt_ns` (the client's own link object, the natural "its link"). The
  client end samples from its `Join`'s ack, which is ready by the instant the server receives the `Join` and sends
  JoinAccepted, so it always precedes the first command: "first command ran before first sample" can never be true, and
  the rule passes vacuously at U4r and again in the re-run after U4h, where it is the argument's only run-level guard.
- Fix: "the SimClock instant at which the server end of that client's L1 link first holds an RTT sample, as the pump
  reads it (N12)".

**N2 (Medium): the proposed early-input bite (U4R-Q10) cannot make the rule fire as worded; 8.2 item 3's "can already
fire" overstates it.**
- Draft 8.2 item 3 :538-541 says the "first command before first sample" VOID "can already fire"; U4R-Q10 :732-741
  proposes "one L1-leg replay client sends its first `Input` before it decodes `JoinAccepted`", which "must make the L1
  leg VOID by the RTT rule", at U4r and in the re-run (8 runs each).
- Why it does not fire: the pump sends JoinAccepted at some instant t1 (RQ8 step 2); at P0 its ack's sample is ready
  when JoinAccepted arrives, at the next instant t2 (l1/src/lib.rs:30-31, :512-521; 0 delay each way). An `Input` sent
  by the client at any step after its `Join` and before decoding JoinAccepted reaches the pump at t2 or later, and the
  server end reads the sample by then (above), so its command runs after the sample: the leg is not VOID. The rule can
  fire only if the command runs at t1 itself: the client sends `Join` and its first `Input` in one step, both reach the
  pump in one call at t1 (the `Join` first by fate index, else the `Input` is "a datagram before Join dropped and
  counted", plan :703), and the `Input` is stamped for the tick that is due at t1 (RQ8 step 3 follows step 2).
- Scenario: the advisor takes (a); the builder plants the natural reading (send the first `Input` one step early); the
  L1 leg comes out valid; the records show a missed bite on a guard that works, at U4r and again after U4h, two extra
  300 s pairs each time.
- Fix: in U4R-Q10, either (a') the bite's exact timing above, with the expected outcome "the L1 leg VOID by the RTT rule
  alone, `late_count` 0, the Loopback leg valid", and "if U4e's room refuses an `Input` in the pump call that admits
  its `Join`, the bite is not exercised"; or (b') show the rule's code by a unit test on doctored per-client records
  (first-sample instant after the first command's run instant: VOID; equal instants: not VOID, N12), U4R-Q11's
  pattern, with no runs. Recommend (b') at U4r, (a') only in the re-run if the advisor wants a run-level proof. Reword
  8.2 item 3: "can fire only when a command runs in the instant JoinAccepted is sent, a timing no waiting replay client
  reaches".

**N3 (Low): two premises of the re-run's rule belong in the U4h carry (U4R-Q12), and 8.4's "needs no new log" depends
on the first.**
- The comparand: plan :755-756 has the sample in place "before the room reads that client's RTT for any command", but
  the logged comparand is the first command's run (plan :756-758; draft :427-429). The two agree only if U4h's estimate
  reads the RTT when the command runs (`rewind_at` at its tick: §4.9 :337). If U4h reads it at receipt, in the pump, a
  defect that checks the first command before the sample but runs it after passes the rule. 8.4 :576 ("the re-run needs
  no new log") holds only in the first case. Fix: log the first command's receipt instant beside its run tick at U4r
  (one field), or have the U4h carry say the re-run compares the sample with the instant the room first reads that
  client's RTT.
- The Loopback leg's RTT source: §4.9 :341-344 gives the link's `rtt_ns` to `wt` and L1 sessions and the server's own
  ping samples to `ws`; "Loopback's `rtt_ns` is always Some(0)" binds only if the pump classes a Loopback session with
  `wt` and L1. Name it in the U4h carry and in R15's check.

**N10 (Low):** 8.2 item 3 :536, "every room decision but the RTT's being independent of the link", is too broad: the
encoder's baseline, the window's remainder, the priority skips, event resends and Resync all follow each leg's acks
(§4.5 :283-293; §4.7 :321-324). They stay out of the sim's state hash (N7), so the argument stands; reword to "the
sim's inputs (which command runs at each tick, and its Rewind) are independent of the link once `late_count` is 0 and
no view time is replaced or clipped", and have the records list per leg the admits (48) and any drop or kick, so a FAIL
caused by a client dropped on one leg (Kick(overflow): §4.7 :321-324) is read as that.

**N12 (Low): the rule's measurement details.** (a) The first-sample instant is the instant the server end's `rtt_ns`
first reflects a sample as the pump reads it (an end reads an ack once its clock reaches it: l1/src/lib.rs:33-34,
:526-532), not the ack's own time. (b) The first command's run instant is its tick's due instant (base + ⌊k × 10⁹ /
TICK_HZ⌋: draft 2.1 :76); VOID only if strictly earlier than the first-sample instant; at one instant the pump (step 2)
precedes the tick (step 3), so equal instants are not VOID. (c) The Loopback leg writes "Some(0) from the start" in
that column, so it is never an empty gap. (d) 8.2 item 2's "333 ms before its first sample" should read "U4c's 333 ms
(at df8975b: `None`, l1/src/lib.rs:349-353)", until R2 settles it at `<base>`.

Needs a run (day), only where code must settle it: R2 and R4 as the draft has them (N2 in its §11), plus N5's two
`git grep` lines at `<base>`.

Block 3 done.

---

## 4. -18's fresh-delta reading (draft 2.1 :117-120, 7.1 :397, 7.3 :462-463, R16 :605, R21 :610, U4R-Q8 :714-720)

Checked against plan :738-739, §4.5 (:282-294), §4.12 :408, §4.13 :423-425, advisor-u3q.md:180-186, advisor condition 6
(:125-129), marks.json -18 (:4236-4308), the catalogue (docs/design/netcode-proof-messages.md: its only -18 line is the
`PeerBurst` row, :209, and the `Resync` row, :90), and l1/src/lib.rs's bursts (:14-21, :135-139, :163-167).

**Confirmed correct:**
- Plan :738-739 is carried whole: L1 B1, 10 bursts, each start offset 0 to 1 tick, the fresh-delta reading, bite
  no-fallback, `burst_eviction` logged. SS and 48 clients (§4.12 :408), 10 bursts in 600 s (§4.13 :423-425), the offsets
  in the header's `burst_starts_ns` as U2's probe writes them (lab/src/main.rs:132-135) are right.
- The ruling is quoted exactly ("a delta counts once it was encoded at or after the link's return": advisor-u3q.md:184-185)
  and applied rightly: the heal is the SimClock time from the link's return to the first decode of a delta encoded at
  or after it, per client and burst (2.1 :117-119); R16 rightly asks U4e's `StepOut` for each decoded delta's encode
  tick, which the reading needs. On L1 the burst is half-open in send time (`in_burst`: t ≥ start and t − start < len,
  l1/src/lib.rs:163-167), so "encoded at or after the return" is exactly the first snapshot the burst does not take.
  On L1 the reading changes nothing in practice: packets sent before a burst arrive inside it at B1's delays (P2's
  100 ms ± 20, :135-136), and the reorder step holds none past the return, since the packet that would overtake it is
  sent inside the burst and lost (:14-16, :21); so the record keeps one definition for L1 and L2 at no cost on L1,
  which is the ruling's point (stale deltas are an L2 effect: advisor-u3q.md:180-184).
- marks.json is not contradicted: the checks (`event.burst_heal.after_link_ns` max ≤ 500,000,000 ns and count ≥ 480,
  :4268-4281) are what the draft's row cites; the note's plain wording ("when it first decodes a delta after the link
  returns", :4307) is the text the ruling tightens "inside the row being rewritten" (advisor-u3q.md:184), i.e. U4j's
  split (plan :632), which R21 checks at `<base>`. The catalogue's `PeerBurst` row (100 % loss both ways, 10 bursts a
  run) agrees.
- Condition 6's route (the bite's baseline one the client still holds: -18's L1 part to the advisor before it counts)
  is quoted correctly (advisor :128-129).

### Findings

**N9 (Medium): "a delta" is not separated from a FullState, and a heal through a Resync is not recorded.**
- The record counts "the first decode of a delta" (2.1 :117-119; 7.1 :397), and neither the draft nor marks.json says
  that a `FullState` (the join's or a Resync's) is not one. After a 1 s burst a resync can happen: the room forces one
  on event-queue overflow (§4.7 :321-322), and "a cap below about 400 would resync all 48 clients on every burst"
  (§4.7 :325-326); once that FullState is acknowledged, the no-fallback bite's "last acknowledged snapshot"
  (marks.json:4298) is one the client holds, and deltas decode again.
- Scenario: in the bite run the client heals through a resync inside 500 ms; the runner counts the FullState's decode,
  or the delta after it, as the heal; no-fallback reads missed though the defect is there (or the clean runs' heal
  times are the resync's, not the fallback's).
- Fix: "the heal is the first decode of a `Snapshot` delta, against a held baseline or the per-type defaults, encoded at
  or after the return; a `FullState` never counts. Each burst's record also notes whether a FullState reached that
  client between the burst's start and its heal; a bite run whose heals came through one goes to the advisor with
  condition 6's report, never counted as caught or missed on its own." (The event cap's value is U4e's; R18 or R20 can
  name it at `<base>`.)

**N15 (Low): the -18 row's details.** (a) Name marks.json:4307's plain wording and say the record follows the ruling
whichever words U4j's split carries, the report stating which (R21 already asks). (b) The plan fixes the offset range,
not how the 10 offsets are chosen nor the bursts' spacing in 600 s: the builder states both (for example a seeded draw
in [0, 1 tick) ns per burst; bursts every 60 s from 30 s), recorded in `burst_starts_ns`; "read here as from a tick
boundary" is a sound reading (at 30 Hz every whole second is a tick boundary). (c) `burst_eviction` is a per-client
fact ("1 if the last acknowledged baseline had left the ring", §4.5 :289-290): per client and burst, as R18 asks.

Block 4 done.

---

## 5. Outside scope, and what is confirmed

**Outside scope:** no Critical or High seen in the parts read in passing (7.3's arithmetic re-added: 15,744 s =
472,320 ticks, and the R table, are right).

**Confirmed correct (in scope):** carry 40 word for word and its statement at df8975b (plan :690-691, :799-800); the
stale cites (advisor :344's :678-679 and :787; advisor :343's :39-42, now :42-45); every leg, the comparer, `late_count`,
the setup condition, the VOID rules, both bites' outcome rules, the depth's name, the result folder, no mark, and
"-30 stays NOT-RUN until U6"; the RTT facts (link/src/lib.rs:203-205; plan :555-557; L1 at df8975b `None` before the
first sample) and the argument itself (sound at P0 with the replay clients' wait); the re-run's case and its
replay-first step; -18's row and its fresh-delta record, with no contradiction in marks.json or the catalogue.

**Not settled here (needs the code at `<base>`, day):** what `rtt_ns` returns before the first sample after U4c (R2);
how the lab learns the server end's first-sample instant (R4); N5's two greps; whether U4e's client sends `Resync` on an
undecodable delta (the catalogue gives Resync only for a client that "has lost its state", :90).

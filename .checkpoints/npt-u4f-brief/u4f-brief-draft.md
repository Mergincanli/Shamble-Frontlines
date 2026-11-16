# U4f builder brief: clock and commands (rev 2, night draft)

rev 2, 2026-11-16 (night item 7, npt-u4f-brief), revised by the netcode-architect once for both checks of rev 1 (the
test-engineer's `test-engineer-u4f-check.md`, TE-1 to TE-24; the security-auditor's `security-auditor-u4f-check.md`,
SA-1 to SA-17) and the night session's spot-check SC-1 (`.checkpoints/npt-u4f-brief.md`, "Open findings"). Read from the
spike branch at 11fd190e9536702f0dcfc72d584b637ebc6184be, read only. rev 1 is at 828e2f4 (same path). Next: the day
session brings it up to date (section 12's list) and takes it to the advisor before U4f's build
(N/u4-build-plan.md:1054-1056; N/advisor-u4.md:154-158). Nothing here is ruled: every choice the plan or a ruling leaves
open is a U4F-Q question in section 10, and the brief follows its recommendation only where it says "by U4F-Qn". The form
is U4r's brief, rev 3.2 (N/u4r-brief.md), kept short. The findings (NA-1 to NA-13) are listed after section 12.

## Change list (rev 1 to rev 2)

Status: **fixed** (where), **question** (U4F-Qn: only a ruling settles it; the text shows the recommendation "by
U4F-Qn"), **needs a run** (Dn), **declined** (why). Nothing was declined.

| Id | Sev. | Status | Where in rev 2 |
|---|---|---|---|
| TE-1 | High | question U4F-Q9 | 2.4's order rewritten to the recommendation: the uncounted repeat filter first; f11, f12 |
| TE-2 | Med | question U4F-Q9 | 2.4: an honest reordered command reaches the repeat filter or the outside-window rule (room.rs:791; client lib.rs:442-443, :464); f12 (iii), (iv) |
| TE-3 | Med | fixed | 5.2's -20 option-B row; 5.3 adds -20 P3 option B with U4f's suite and controller |
| TE-4 | Med | fixed | 2.2 and 5.3: under option A "late" read as arrived after its stamped tick ran; the room's `late` on 5.3's option-B runs; D15 |
| TE-5 | Med | question U4F-Q11 (v) | 2.6's fast clock stamps by its own rule, by U4F-Q11 (v); D12 |
| TE-6 | Med | fixed | 5.3 gains 7 short bitten runs; D4 gains the bitten arrivals; D12 |
| TE-7 | Med | question U4F-Q15 | 2.6's lag-switch row; D13 |
| TE-8 | Med | question U4F-Q11 (iii) | restated: the loosening and the readings-PR route (plan :971-974) |
| TE-9 | Med | fixed | 2.10: measured +100, bites +115 (f30 bites-only); conditional deltas listed |
| TE-10 | Med | fixed | f31; section 3's row 39 |
| TE-11 | Med | question U4F-Q16 | 2.9 (b): the newest-tick bound, apart from C62H-BOUND |
| TE-12 | Low | fixed | f25 names the failing input and its counter; 2.13's accessor |
| TE-13 | Med | fixed | 5.3's length per suite (fast clock 70 s measured); f28 |
| TE-14 | Low | fixed | f6 restated, in npt-lab |
| TE-15 | Med | fixed | 2.2's strictly-rising rule; f5's pass rule; f32 |
| TE-16 | Med | fixed | 5.5 and U4F-Q2 in MiB throughout, the total from the rows, "no margin"; NA-5 |
| TE-17 | Low | fixed | f21 Δ 1; r9 named as kept |
| TE-18 | Low | fixed | 2.12: kind 10 in one row |
| TE-19 | Low | fixed | 2.3 and section 8 row 1: -58's two sites, names and unit proofs stay |
| TE-20 | Low | fixed | 2.7: `lead_ticks_count` only where a run fixes the lead (U4F-Q7) |
| TE-21 | Low | fixed | 2.3: the sort takes a total key; f8 |
| TE-22 | Low | fixed | 5.2's -22 row: the main session's -22 set report; section 12 item 10 |
| TE-23 | Med | fixed | f33; 2.4's unit per kind |
| TE-24 | Low | needs a run D11 | 5.3's repeated forged run |
| SA-1 | High | fixed (carried, entry 80 (a)) | 2.3: the total key; f8 Δ 3; D14 |
| SA-2 | High | question U4F-Q9 | = TE-1 |
| SA-3 | High | question U4F-Q13 | 2.6 (every copy forged), 2.12 (twins by U4F-Q13); f16 per kind |
| SA-4 | Med | fixed | 2.4's unit per kind; f33 |
| SA-5 | Med | question U4F-Q14 | 2.5 writes every refusal counter per session (names by U4F-Q1); f14 |
| SA-6 | Med | question U4F-Q11 (v) | = TE-5; f19 asserts both refusal counts |
| SA-7 | Med | question U4F-Q15 | = TE-7 |
| SA-8 | Med | fixed | 2.2; f5, f32 |
| SA-9 | Med | question U4F-Q9 | the kept window and the outside-window rule; f12 |
| SA-10 | Med | fixed | 2.6's clamp row (reruns reported); f18's option-B case |
| SA-11 | Med | question U4F-Q8 | restated: both allowances per value; (ii) a carry to U4h |
| SA-12 | Low | fixed | 2.6 (kind 9 on datagrams, both forms); f15 |
| SA-13 | Low | fixed | 2.6's bucket arithmetic; `input_limited` per cheat reported |
| SA-14 | Low | fixed | 2.4 kind 6: z left to the codec and the clamp, by design |
| SA-15 | Low | fixed | 2.6 and f15: kind 7 sends cooldown and reload edges |
| SA-16 | Low | fixed | 5.2's -22 row reports `overflow_resyncs_count`; section 9's follow-up for U6 |
| SA-17 | Low | fixed | section 4: the catalogue's logging rule |
| SC-1 | - | fixed (accepted) | 2.1's ping row; U4F-Q6's why; NA-4; D4 amended |
| NA-3 | Med | changed | holds for margins up to about 0.73 s (TE's check) |
| NA-4 | High | changed | SC-1: the pings meet the tick grid at one phase per connect |
| NA-5 | Med | changed | TE-16: "no margin", in MiB |
| NA-7 | Med | changed | the security-auditor's correction; TE-1's order |
| NA-13 | Med | new | input.rs:110: an equal-to-buffered stamp is `Late` under B; 2.4 kind 2; f13 |

**Tally:** TE 24: 17 fixed, 6 questions, 1 needs a run, 0 declined. SA 17: 10 fixed, 7 questions, 0, 0. SC 1: fixed.
NA: 4 changed, 1 new.

**Counts (rev 2):**

| Item | rev 1 | rev 2 |
|---|---|---|
| Tests, measured build (Δ on `<base>`) | +60 (in fact +59: TE-9) | **+100** |
| Tests, bites build (Δ) | +74 | **+115** (100 + 14 bites-only unit proofs + f30) |
| Conditional, both builds | T-6 +5 | T-6 +5; U4F-Q6 (b) +3; U4F-Q9 (a) +1; U4F-Q11 (iii) (c) +1; U4F-Q13 (a) +1 |
| Negative cases (measured) | 44 | **60** |
| Bites | 17 | 17 (+1 run by U4F-Q15 (b)) |
| Judged runs | 65 (36 + 12 twins + 17 bites) | 65 (+1 by U4F-Q15 (b); +13 by U4F-Q10 (c)) |
| Short pre-runs (5.3) | 9 + 5 (or 9) | 14 (+4 by U4F-Q11 (iv)) |
| Carry placeholders | 14 | 14 |
| `@@WORDS@@` placeholders | 15 | 15 |
| Recheck rows (section 8) | 29 | 33 |
| Open items (section 9) | 15 | 17 (SA-16's follow-up; SA-11's carry by U4F-Q8 (ii)) |
| Questions | 12 | 16 |
| Needs a run (day) | 10 | 15 |
| Disk, every set and pre-run (estimate) | "about 3,750 MB" | about 3,596 to 3,656 MiB against 3,593 MiB above the floor (5.5) |

The arithmetic behind each count is in its section (2.10, 5.1, 5.5).

---

## 0. How to read the citations

- **Read at:** 11fd190 (the spike branch's head at the night's fetch). Every file:line below is at 11fd190. `<base>`
  is the day session's (section 12 item 1): the spike branch's head at launch. The card was edited while the scout ran
  (N/night-u4f-scout-full.md:5 names 0ad41a6), so card items are named by their text; their 11fd190 line numbers are
  given only for section 8's recheck.
- **Short names:** `N/` = .checkpoints/netcode-proof-test/; `plan` = N/u4-build-plan.md; `plan.md` = N/plan.md (the
  test plan, rev 3.3); `carries` = N/u4s-carries.md; `card` = .checkpoints/netcode-proof-test.md; `history` =
  N/history.md; `catalogue` = docs/design/netcode-proof-messages.md; `ADR 0001` = docs/adr/0001-multiplayer-netcode.md;
  `marks.json`, `SERIES.md`, `README` = spikes/netcode-proof/marks/'s; a crate path (`room/src/input.rs`) is under
  spikes/netcode-proof/crates/; `check.mjs` is spikes/netcode-proof/check.mjs; run scripts are under
  spikes/netcode-proof/ops/runs/.
- **The rulings** (closed records, all in N/): `advisor` advisor-u4.md; `wave1` advisor-u4-wave1.md; `e-ruling`
  advisor-u4e-brief.md; `e-r1` advisor-u4e-r1.md; `e-r2` advisor-u4e-r2.md; `r-check` advisor-u4r-brief.md; `r-r1`
  advisor-u4r-r1-full.md; `r-b2` advisor-u4r-build2-full.md; `c62h` advisor-u4r-c62h-full.md; `c62h-review`
  advisor-c62h-review-full.md; `te-m3` advisor-te-m3.md; `df-ruling` advisor-u4df.md; `j-r2` advisor-u4j-r2.md; `m9`
  advisor-u4d2-m9.md; `d2-r2` advisor-u4d2-r2.md; `b1-p3` advisor-brawl1-sets-p3-full.md.
- **Placeholders** (filled by the day session's script, never by hand: the night plan's rule 6 and item 7's shape 3
  and 9): `@@CARRY-<entry>@@` is carries' entry at the line range section 3 gives, word for word;
  `@@WORDS-<file>:<a>-<b>@@` is the text at that range (the quoted span inside it where section 12 item 3 says so).
  Mustafa's words appear only as cites into history by line range, or as a WORDS placeholder there; none is retyped
  or paraphrased as his.
- **Units:** disk and memory in MiB (2^20 B) throughout; a source figure in MB (10^6 B, as c-u4r-runs.mjs:138-153
  states its rates) is converted and its MB figure given in brackets (TE-16).
- **Stamps:** the sha256 and line count of plan, plan.md, carries, history and the card at `<base>` are the day
  session's (section 12 item 2); a file whose sha256 moved since 11fd190 has its cites re-walked (section 8).
- **Mustafa's standing rules, by range only:** the shared target, history :411-420; LEFTOVER-DELETE, :404-410; the
  cleanup after each merge, :859-861; RESUME-CARD, WIP-CLEAN, TCP-FREEZE, SQUASH-40, PROMOTE, :379-403; SIZE-CHECK,
  :398-399 and :2039-2087; USAGE-SAVINGS, :291-299; NIGHT-BUDGET, the 9.5 floor and "a Low never pushes", :335-352;
  his words move by script and line ranges, :353-356; SPEND-LIMIT and review_packet's ALLOWED, :1927-1975; CHECK-IN,
  :1990-2012; KEPT-LOGS (c), :2088-2111; C62H-REVIEW (a) and the read rule, :2113-2129 (the rule's line :2127). The
  card's "Rules in force" lists them (card :3013-3088 at 11fd190).

---

## 1. Start

1. **Base:** `<base>` (section 12 item 1). U4f depends on U4r (plan :805), merged; U4f, U4g and U4h run one at a time
   (plan :882-883; advisor :186-187). In your worktree: `git merge --ff-only <base>`; if that cannot fast-forward,
   `git switch -c u4f-build <base>`; then `git log -1 --format=%H` shows `<base>`. Record it in your notes first.
2. **Who and where:** one builder, general-purpose, opus, effort xhigh, in an isolated worktree under
   `/home/user/Shamble-Frontlines/.claude/worktrees/` (`isolation: worktree`; plan §7.2 :925-927), under the shared
   target rule and W1-TARGET's lock (section 4). The judged runs are not yours: the main session runs them after the
   merge, in the main folder (plan :928-929; section 5).
3. **Your notes:** `.checkpoints/netcode-proof-test/u4f-build.md` in your worktree. First line: this task. Then your
   plan as blocks (one block: one type with its tests, one check with its counter, one cheat, one bite and its proof,
   one run script); each finished block written at once, a notes line per finding, one line after each block (done,
   what next). On a restart: your notes and `git diff` first, then go on after the last block marked done.
4. **The runner:** `u4f-run.sh` (section 12 item 4: made by the main session from u4r-run.sh, only the lane's names
   changed, with its .diff; sha256 and `bash -n` at launch). Its use as u4r-run.sh's (N/u4r-brief.md:265-274): `-x`
   for every counted run, bite run and plant; a run's result is its exit code and the test runner's own summary line,
   never a grep (section 4).
5. **What stops you** (each written in your notes and your report, then build the rest): a file outside section 4's
   lane; an item another unit merged changed beyond what this brief names; a manifest or Cargo.lock change (the main
   session's); a new external dependency; disk below section 4's floors; a part a U4F-Q leaves open at launch; a tick
   count or rate-derived constant you need (carry 39's rule: record name, definition, values at 30 and 60 Hz and site,
   build the rest); a wire change beyond U4F-Q6's ruling (the catalogue and its tests move with it:
   .claude/rules/netcode.md:15).
6. **Read before you build** (and nothing you do not need):
   - plan: §5 U4f (:805-816); §4.1's rows for npt-client, npt-room, npt-server and npt-bots (:193-197); §4.2's refusal
     rule (:239-249); §4.3 (:251-265); §4.4 (:267-290); §4.5's window (:301-303); §4.7 (:330-336); §4.8 (:338-341);
     §4.9's tolerance (:351-356, context only: U4h's); §4.10 (:360-390); §4.11 (:392-407); §4.12 (:409-425); §4.13's
     shot band and SS's scripts (:427-447); §4.14 (:453-476); §5's preamble (:507-512); §5 U4e (:714-748) and U4r
     (:750-803); Lanes (:851-889); §7 (:909-932); §9 (:958-968); §10's readings rule (:971-974); Q16, Q21, Q22
     (:1042-1045, :1054-1060);
   - plan.md: E3, E10, E12 (:461, :468, :470); E7's endpoint (:485-491); §4.4's table (:506-525); §4.6 (:553-566);
     §4.7's npt-room, npt-client and npt-lab entries with RQ8's order and the host clocks (:596-598, :609, :617-628);
   - ADR 0001: Part 5's checks, acknowledgements and Time (:224-255); OQ-0001-6 (:1156-1167); rows PM-0001-20, -21, -22
     (:1610-1612) and -47 (:1644);
   - marks.json: -20 (:4618-4778), -21 (:4779-5119), -22 (:5120-5202), -47 (:9046-9109), PM-0001-2 (:151-625);
     SERIES.md's rows of 2.7; README §2.5 (:187-192), §4 (:247-290), §5 (:317-330);
   - catalogue: where each check runs (:52-59); the Input, Ping, Pong and Kick rows (:87, :91-92, :99); the authority
     note and the logging rule (:120-123);
   - u4-scout-netcode.md: F104 (:122), F117 (:135), F59 (:77), F79 (:97), F108 (:126), F119 (:137), F121 (:139);
   - the rulings sections 3 and 10 name, and advisor conditions 1, 4, 5 and 10 (:78-89 amended at :477-479,
     :112-124, :143-149), the F2 ruling's point (6) (:463-465), wave1 W1-U4D N-M2 (:197-239), U4E-Q6 and U4E-Q11
     (:473-479, :492-495), e-ruling U4E-Q12 (:217-244, :340-341), r-check U4R-Q4 and U4R-Q14 (:198-205, :289-358);
   - .claude/rules/netcode.md (:1-17): binding on proto, room, client and server code (plan §7.1 item 8, :921-922;
     advisor :192-193). U4f writes room and client code and feeds the server's pump: every rule there binds it;
   - the code at `<base>`, by section 8's rows.

---

## 2. What to build

U4f's row (plan :806-809): ClockSync; the client's tick-lead controller; `InputBuffer` option A; every other room-side
check with its -21 counter; the server-side checks' -21 counters and judged runs on U4e's pump (U4E-Q11); the cheats'
`Behaviour`s. The marks it moves (plan :810-815): -47, -20, -21 and -22 (section 5). Nothing here adds an engine
feature beyond those rows (CLAUDE.md's "for later" rule); an item a table marks "proposed" is the builder's to build
only once section 10's ruling names it.

### 2.1 ClockSync, the command clock (npt-client; plan §4.8 :338-341; ADR 0001:239-251; net F117 :135)

| Field | What U4f builds | Source |
|---|---|---|
| Home | `ClockSync` in npt-client, owned by `ClientEndpoint` | plan.md:609 (the symbol); plan §4.8 |
| What it estimates | the offset from the client's host clock to the server's clock, signed integer ns: the command clock only. The interpolation clock is `Interp`'s (U4g: plan :822); ClockSync exposes nothing for it now | plan §4.8; net F117 |
| Its samples | the client's own `Ping` (id, the host time of the step that sends it) and the room's `Pong` (id, echoed time, tick, hold time), each timed by its datagram's arrival stamp from `ClientEndpoint::input`'s `received` (client/src/lib.rs:414-424), never by the step's own `now` | ADR 0001:242-245; catalogue :91-92; plan.md:486 |
| The server's stamp | as U4F-Q6 rules. As built the room answers a `Ping` in the I/O step of the instant it arrives (plan.md:618-622; hold about 0) with the next tick to run (lab/src/scenario.rs:459; server/src/pump.rs:458-467), so each sample's server instant is known only to one tick period (NA-4) | U4F-Q6 |
| The filter | ADR 0001:240-242's rules (drift at most 1-2 ms per frame, halve errors over 100 ms, reset over 500 ms), over the lowest-RTT samples; every threshold in ns, data, pinned by a test | ADR 0001:239-242; net F117 |
| Ping rate and phase | data, at most the room's limit (10 a second per session: catalogue :91; the pump's bucket, pump.rs:458-471). The client sends a `Ping` only at a step, and its steps fall every 1/TICK_HZ of its host clock (client/src/lib.rs:485-488), so every ping period is a whole number of ticks: on L1, with an honest host rate, one connect's samples meet the server's tick grid at one phase, moved only by the delay jitter (P2: ±10 ms each way, l1/src/lib.rs:149-156, :214-218, against a 33.3 ms period). The filter may not count on random phases (SC-1; U4F-Q6). `ping_limited` 0 on a clean run, read from 2.5's per-session counters | SC-1; net F117 |
| Bounds | outstanding pings and kept samples each capped (as the pump's `OUTSTANDING` 8 and `SAMPLES` 8: pump.rs:24, :26); a `Pong` naming none of its pings is dropped and counted (`unknown_pongs`, client/src/lib.rs:403-410, today crate-private: public only if the lab logs it) | .claude/rules/netcode.md:13-14 |
| Clock reads | none: the endpoint reads no clock (ADR 0001:245-247; client/src/lib.rs:6) | ADR 0001 |
| Carry 39 | the tick period in ns comes from the rate passed in (production callers pass `npt_sim::TICK_HZ`), pinned at 30 and 60 Hz (f31); no local tick constant | carries :169-179 |
| What -47 reads | npt-lab writes `event.connect_clock.offset_err_ns` (SERIES.md:149) per connect: abs(estimate - true) at the first client step at or after 2 s from the connect's first sample (the builder states this instant in its doc and report); the true offset is the lab's seeded host offset (lab/src/scenario.rs:18-19; plan.md:625) | marks.json:9069-9094 |
| Bites | `PM-0001-47/newest-sample` (the estimator takes the newest raw sample) and `PM-0001-47/tick-quantised` (each sample timed at the step that reads it, not at its arrival), both in `npt_client::BITES` (client/src/lib.rs:28-32, growing to 6 with 2.11's -20 bite), through `ClientEndpoint::with_bite` (:314-326); the ids spelled as marks.json's (:9097, :9101), not as the plan's words at :810 (NA-11). A lowest-RTT filter keeps the samples the tick-quantised bite delays least, so its bite may not fail: D4 models it and 5.3 runs it short first (TE-6) | marks.json; plan §4.14's rule (:454-455) |

### 2.2 The client's tick-lead controller (npt-client; net F104 :122; carry 74; r-check U4R-Q14 :289-358)

| Field | What U4f builds | Source |
|---|---|---|
| What it replaces | the endpoint's fixed lead (`lead`, client/src/lib.rs:214, 1 by default :267; `with_lead` :296-301, whose doc names U4f's controller as its replacement); whether `with_lead` stays as a lab setting is U4F-Q7 | carry 74 |
| What it sets | each command's stamped tick (today `newest + lead`, :429) | ADR 0001:250-251 |
| Its form | U4F-Q7: (a) the newest snapshot tick plus a lead from the client's own RTT and jitter samples, by U4R-Q14's arithmetic (L = ceil((rtt + jitter) / P) + 1, r-check :296-302); or (b) ClockSync's estimate of the server's tick at the command's arrival plus a margin and the buffer target (net F104 :122) | U4F-Q7 |
| Stamps rise strictly | each stamp is strictly above the previous command's (the room refuses any other as kind 2: ADR 0001:231-233); the controller lowers its lead only by holding (no command at that step), never by stamping back; a ClockSync reset moves the estimate, never a stamp backwards (TE-15, SA-8; f5, f32) | ADR 0001:231-233 |
| Same under both options | one controller under option A and option B (net F104); the endpoint never reads `cmd_timing` (README :191-192 makes it the room's and the scenario's); the lab builds both options' endpoints with one controller setting (f6) | net F104 |
| Inputs | ClockSync's samples only (the client's own measure), never the profile (the lab's knowledge: lab/src/scenario.rs:126-131 is U4r's stand-in) | ADR 0001 part 1 |
| Bounds | at least 1; at most the room's lead bound less a margin (2.4 kind 1); the endpoint keeps at most `REDUNDANCY_CAP` = 8 unacknowledged commands (client/src/lib.rs:442-443) | carry 27; ADR 0001:231-233 |
| Pass marks it serves | -20's 99.9 % on time at P3 under both options (marks.json:4684-4697); carry 74's 1 % late rule on 5.3's runs, read under option B as the room's `late` count and under option A as the commands that arrived after their stamped tick ran (1 - `tick.s.cmd_on_time`'s share, SERIES.md:61), both reported on every short run (TE-4); the lead it settles on per profile reported beside U4R-Q14's numbers (P0 2, P1 3, P2 and B1 5, P3 7; P4 9 by the same arithmetic: l1/src/lib.rs:193-196) | carry 74 |
| The bots' rules | unchanged for honest bots: one command per client step, only when the newest tick moved; at most one new edge per press counter per command (bots/src/lib.rs:14-21; U4R-Q14's (b1) and A4) | r-check :322-323 |

### 2.3 `InputBuffer` option A beside U4e's option B (npt-room; OQ-0001-6, ADR 0001:1156-1167)

| Field | Option A (U4f builds) | Option B (U4e's, kept) |
|---|---|---|
| When a command runs | at the first room tick after it arrives (pumped before the tick: plan.md:621-622), several in one tick within `CMD_CATCHUP` = 8 per client, each one's tick strictly newer than the last run | exactly one per client per tick, the one stamped for it (room/src/input.rs:129-151) |
| A tick with none | nothing runs for that client (plan §4.13 :439-441) | the last command runs again (:135-141) |
| Late | no late rule: a strictly newer command runs whenever it arrives; on time is still read against its stamp (SERIES.md:61) | stamped for a tick already run: dropped and counted `late` (:107-109). Today a stamp equal to a buffered command's is `Late` too (:110): under 2.4's order kind 2 refuses it first (NA-13) |
| The per-tick cap | at most 8 run per tick; the excess refused and counted under the catch-up kind (2.4 kind 10) | the buffer holds at most 8 (`Full`, :114-116), counted under the catch-up kind |
| The setting | `params.cmd_timing` "A" or "B", a start-up setting, never changed in a run, recorded in every header (README :191-192; plan.md:461); today fixed to "B" (lab/src/scenario.rs:1286) | the same |
| Depth recorded | U4F-Q3 (the card's DEPTH ruling and its option-A note, card :1930-1933 and :1962-1964 at 11fd190) | `buffer_depth_ticks` (input.rs:63-66); the measured L - 1 reading (card DEPTH ruling) |
| -58's two sites | through U4f's rewrite of `offer`, the sequence-forward compare (input.rs:82; its name `CMD_SEQ`, :11) and the press-edge compare (:98; `PRESS`, :13) keep their sites, names and unit proofs: PM-0001-58/lt-cmd-seq and lt-press (plan :468-469), judged at U4g (plan :829-831) (TE-19) | the same |
| **Carry 80 (a) first** | before option A runs several commands of one slot in a tick, `canonical` (room/src/room.rs:363-369) takes a total key: within a slot, the tick's signed distance from the tick being run (`tick.wrapping_sub(t) as i32`), which orders any set of u32 ticks and equals today's wrap-aware order for ticks within 2^31 of `t`. Carry 80 (a) allows the lead-bound route instead; this brief takes the total key, because the bound alone leaves a non-total triple reachable from a fresh slot (SA-1: ticks T-2^31+1, T-2^30 and T+1, none 2^31 from T, with no last run to refuse against) and in the no-lead-bound bite run, where a panic makes the run INVALID (README :250-251; TE-21). Tests f8 and f9 (2.10) | carry 80 (a), carries :513-518 |

### 2.4 The command checks, each with its -21 counter (npt-room unless named; plan.md:512-514; catalogue :87)

**The counting rule and the order (by U4F-Q9; TE-1 = SA-2, TE-2, SA-9, SA-4, NA-13):**
1. **Kind 8 first**, on the whole datagram (room.rs:756-788); kinds 9 and 11 at the pump, before the room sees it.
2. **Per command, oldest first** as now (room.rs:791). A sequence not after the newest received (input.rs:81-84) goes to
   the **repeat filter**, never counted as forged: content equal to the command kept under that sequence is a redundant
   copy (each `Input` re-sends every unacknowledged command: client/src/lib.rs:442-443, :464), dropped and counted
   `commands_repeat` as now (room.rs:795); content that differs, inside the kept window, is **kind 3**; a sequence older
   than the kept window is dropped and counted under its own non-forged counter (`seq_outside_window`, the builder's
   name), never as forged. The kept window: the last K commands per client received under a new sequence, accepted or
   refused, K = `REDUNDANCY_CAP` plus a stated reorder allowance, bounded (.claude/rules/netcode.md:13-14). An honest
   reordered datagram's commands were all received in the later datagram that overtook it, so they land in the filter,
   or, past K, in the outside-window counter (TE-2).
3. **A new sequence** then meets kinds 1, 2, 4, 5 and 6 in that order, then kind 10 (option A: at the tick's run, past
   `CMD_CATCHUP`; option B: `Full`), with option B's late rule after kind 2.
4. **Kind 2** compares the stamp with the newest accepted command's (run or buffered: NA-13), so an honest late
   command (stamps rise strictly: 2.2) passes it and is counted `late`, and a forged equal or older stamp is kind 2,
   never late.
5. **One kind per refusal**, the first check that refuses; npt-room reports per tick each refused command as (slot,
   seq, kind), as npt-sim's `reports()` does for its own (plan §4.2 :246-247).
6. **The unit per kind (SA-4):** one forged command (slot, seq) for kinds 1-7 and 10; one forged datagram for kinds 8
   and 9; one forged message for kind 11. npt-lab counts a forged item "rejected and counted" only when a refusal
   record names it with its own kind and no copy of it ran (checked against the tick's `TickInput`); `unrejected_count`
   (SERIES.md:159) counts the items failing that and is never negative; kind 8 is read from `ack_refused` (datagrams:
   room.rs:785), never `ack_refused_commands`; kinds 9 and 11 are matched by count per session (the pump keeps counts,
   not records), each forger's datagrams or messages carrying only its own kind (f15, f33).

Each kind is one check of ADR 0001:231-239 (plus the wrong channel, plan.md:489), 11 in all (marks.json:4920-4931,
:5118).

| # | Kind (forged command) | Home, place in the order | Today at 11fd190 | U4f builds | Counter (builder names it; SERIES row by U4F-Q1) | Bite (2.11) |
|---|---|---|---|---|---|---|
| 8 | acknowledgement of a snapshot never sent | room, 1st, whole datagram | the datagram's commands refused (room.rs:783-787), `ack_refused`, `ack_refused_commands` | kept; counted as this kind, by datagram | `ack_refused` | PM-0001-21/no-ack-sent |
| 3 | sequence not forward | room, in the repeat filter | `Repeat`, counted with honest copies (input.rs:81-88; room.rs:795) | a refusal apart from honest repeats (NA-6) | sequence refusals | PM-0001-21/no-seq-forward |
| 1 | tick too far ahead (the lead bound) | room, 1st for a new sequence | none (NA-7: 8 far-ahead commands fill option B's buffer, then run at their stamps) | the check; its form and value by U4F-Q8 | lead-bound refusals | PM-0001-21/no-lead-bound |
| 2 | tick equal to or older than the newest accepted | room, 2nd | caught only as `late` under B (input.rs:107-113), an equal-to-buffered stamp included (:110: NA-13) | the check, apart from `late` | not-newer refusals | PM-0001-21/no-newer-tick; PM-0001-21/not-older-than (`>=` for `>`, plan :462) |
| 4 | move length above 1 | room, 3rd | none (the sim scales a long vector) | x² + y² ≤ 127², integers (bots/src/lib.rs:30-31) | move refusals | PM-0001-21/no-move-length |
| 5 | more press edges than ticks covered | room, 4th | `PressRefused` (input.rs:90-106), counted `press_refused` | kept; the counter is this kind's | `press_refused` | PM-0001-21/no-press-edge |
| 6 | aim outside the map | room, 5th | none | x and y inside the greybox's extent (±12,800 cm: sim/src/greybox.rs:62, :151; the room's `extent`, room.rs:441); z is left to the codec's range and the clamp by design (the extent holds x and y only: SA-14); beyond-reach points pass on to the sim's clamp | aim refusals | PM-0001-21/no-aim-bounds |
| 10 | more commands in a tick than the catch-up cap | room, last | option B's `Full` (`buffer_full`) | option A's per-tick cap; both counted as this kind | catch-up refusals | PM-0001-21/no-catchup-cap |
| 7 | fire or reload edge against the sim's state (cooldown, reload; ammunition cannot occur in SS: plan :434-435, world.rs:1411-1419, so it stays with U4d's unit test per kind, advisor :87-88: SA-15) | npt-sim, in the step | the sim refuses and records (slot, seq, kind) in `reports()` (plan §4.2 :239-249) | the lab's attribution from `reports()`; the bite's site is npt-sim's (NA-9, U4F-Q1) | the sim's refusal counters | PM-0001-21/no-cooldown |
| 9 | trailing bytes or an unknown field (a set reserved or pad bit: proto quant.rs:115-122) | npt-server's pump (plan :287-290) | `trailing_bytes`, `unknown_field` (pump.rs:125-128) | the -21 series only (U4E-Q11); forged on `Input` datagrams only, both forms (SA-12) | those two | PM-0001-21/no-trailing (npt-proto, quant.rs:40) |
| 11 | a known type on the wrong channel | npt-server's pump | `wrong_channel` (pump.rs:122-123); refused and counted, never closing a joined session (pump.rs:414-421; catalogue :87 against :99) | the -21 series only | `wrong_channel` | PM-0001-21/no-channel (npt-proto, quant.rs:41) |
| - | beyond the weapon's reach | npt-sim's clamp (plan §4.2 :227-231) | clamped, reported, never refused (`event.clamp_kind.counted_count` counts refusals only: SERIES.md:144) | the clamp cheat (2.6) | none by design | PM-0001-21/refuse-not-clamp (npt-sim, NA-9) |

The plan's other room-column checks stay with their units: the ClientSettings clamp (U4g: plan :822); the view-time
tolerance and the rewind cap (U4h: plan :835-837); Resync, roster, token and the event cap (U4e, built).

### 2.5 The server-side checks' -21 counters on U4e's pump (U4E-Q11, wave1 :492-495)

U4e built the checks and their unit tests; U4f writes their -21 series. The counters exist per session (`Counters`,
pump.rs:110-161; plan §4.1 :195): among them `too_large`, `trailing_bytes`, `unknown_field`, `wrong_channel`,
`input_limited` (the bucket of 8, refilled at 2 x TICK_HZ a second: catalogue :87), `ping_limited`, `pong_unknown`,
`join_again`, `before_join`. **Every refusal counter reaches a log (SA-5):** today none of the pump's does (no pump
counter name in lab/src: the security-auditor's grep), and `client_counts` writes six of the room's (scenario.rs:1165-
1186). npt-lab writes every refusal field of each session's `Counters` and of its `ClientCounters` (room.rs:63-136) at
the session's end (a kick, a replacement, a leave or a close: before its `Decoder` is dropped, scenario.rs:441-455) and
at the stop, one event per session (names and SERIES rows by U4F-Q1), so the forged suite reports per client every
counter other than the forger's own kind's (0 expected on honest clients and on each forger outside its kind), and a
kicked never-acker's counts survive its kick. How U4E-Q11's "judged runs" read for the size and bucket checks (no forged
kind of the 11 is theirs: plan :385-388), and whether the hold-time clamp (pump.rs:510, uncounted; plan.md:514 names it)
gains a counter, are U4F-Q14. U4f changes no pump code by this brief; if U4F-Q14 or U4F-Q6 needs any, the lane opens
npt-server (U4F-Q1) and section 6's `--all-features` item applies.

### 2.6 The cheats' `Behaviour`s (npt-bots; plan.md E12 :470; plan :808-809; bots/src/lib.rs:10-11)

Every cheat is a measured-build behaviour (the suites are measured runs), clock-injected and transport-free like U4r's
(bots/src/lib.rs:5-13). A forged datagram is the honest endpoint's own datagram, decoded and re-encoded with only the
kind's change (npt-bots depends on npt-proto: bots/Cargo.toml:23), so forged content rides only inside datagrams the
honest script sends anyway, at the same instants, on the same flows (plan :376); the forger forges the command it
stores, so every redundant copy carries the same forgery (SA-3); the twins of 2.12 follow the same rule. Each cheat
client's `input_limited` is reported per run, 0 expected (SA-13; 2.5).

| Cheat | Suite (marks.json select) | What it does | Count per run | Its twin |
|---|---|---|---|---|
| 11 forgers, one kind each (2.4) | `forged`, A and B | an open-loop seeded script (commands from seed, slot and tick only: plan :380-381) with every k-th command forged in its kind; kind 9 on `Input` datagrams only, trailing bytes and a set reserved or pad bit in turn (SA-12); kind 7 cooldown and reload edges (SA-15); 37 honest open-loop clients | 1,000 forged items per kind, in 2.4's unit (marks.json:4909-4919; plan :442) | 2.12 |
| the catch-up kind (kind 10) | `forged` | under option A, 16 new commands in two datagrams in one tick (plan :385-388); under option B, more commands than the buffer has room for, all inside the lead bound so none is kind 1 (SA-11; U4F-Q8); inside the bucket in both | as kind 10's 1,000 | 2.12, by U4F-Q13 |
| the clamp cheat | `clamp`, A and B | open-loop, aim points beyond the 6,000 cm reach (sim/src/scene.rs:66) yet inside the map (2.4 kind 6): U4r's stand-in's 32,000 cm (bots/src/lib.rs:74-76) lies outside the map and would be refused (NA-8). Reruns of a clamp-cheat command under option B (a tick with none stamped: input.rs:135-141) are reported per run, 0 expected at U4F-Q11 (ii)'s recommended P0; a rerun re-clamps from a moved origin (sim/src/world.rs:1285-1312), so a lossy profile needs a per-tick twin rule first (SA-10; f18) | 1,000 points beyond reach per weapon kind the sim has (hitscan: marks.json:4954-4965) | the forged run's clamp reports' points (SERIES.md:322) at the same instants (plan :363-366) |
| the 1.5x clock | `fast-clock`, A and B | a client whose host clock runs at rate 1.5 over each 60 s episode (plan.md:625; the host clock stays continuous), a command at every one of its steps (45 a second, inside the bucket's 60 a second: SA-13), stamped by its own rule by U4F-Q11 (v) (recommended: each stamp one tick newer than its previous, held at its newest tick plus the lead bound, and every 10th command one tick past the bound, so the clean build refuses both kinds 1 and 2 and the not-older-than bite lets the held stamps run: TE-5, SA-6); each episode reports its lead-bound and not-newer refusals beside its excess | 10 episodes of 60 s (marks.json:4988-4999) | none (the check compares with the bound: SERIES.md:155-156; the honest reference by U4F-Q11 (iii)) |
| the lag switch | `lag-switch`, A and B, P1 | holds its `Input` datagrams 200 ms, then sends them in one burst (about 6 datagrams, inside the bucket's 8: SA-13). At one command per moved newest tick a hold carries about 6 new commands, under `CMD_CATCHUP` = 8, so its check cannot fail at 200 ms (TE-7, SA-7): U4F-Q15 | 100 bursts (marks.json:5024-5035) | none |
| the never-acker | `never-ack` | a brawl client whose `Input`s never acknowledge a snapshot or a full state (ack groups 0); every other client brawls; its `overflow_resyncs_count` reported (scenario.rs:1186; SA-16) | one among 48 (ADR 0001:1612) | none |

### 2.7 npt-lab: suites, settings and series (lab/src/scenario.rs; lab/src/logs.rs)

- **Settings:** `--cmd-timing A|B` into the header (scenario.rs:1286 today writes "B"); the suites `forged`, `clamp`,
  `fast-clock`, `lag-switch`, `never-ack` and `clock` (marks.json's selects; SERIES.md:413's suite param) and their
  behaviours by U4F-Q11; `--twin <run>` with role `twin` (README :22-23; scenario.rs:1404); the 1.5 host-clock rate
  per client and episode (scenario.rs:18-19 names it U4f's); the lead bound's value in the header (U4F-Q8).
- **`header.params.lead_ticks_count`** (SERIES.md:342: the one lead every client of the run stamps with) is written
  only by a run whose lead `with_lead` fixes (U4F-Q7 (i)); the contract test asserts it absent from a controller run's
  header, and the settled lead per client goes in its own series (U4F-Q1) (TE-20).
- **The -47 suite:** 1,000 connects at P2 with seeded random true offsets, each connect's estimate compared at 2 s
  (2.1); 48 slots give about 21 rounds of fresh endpoints. Rejoins on one slot meet carry 62 (d)'s head rule and the
  slot-keyed logs of NC-U4R-5, so the suite's logging is by U4F-Q4.
- **The series U4f writes** (each name's writer as SERIES.md gives it; a name with no row gets one only by ruling, as
  U4R-Q2 (c): U4F-Q1):

| Mark | Series (SERIES.md row) | Writer |
|---|---|---|
| -20 | `tick.s.cmd_on_time` (:61), `tick.s.input_missing` (:63), `tick.v.press_count` (:40), `tick.v.press_lost_count` (:42), `tick.v.press_doubled_count` (:41), `second.s.buffer_depth_count` (:101); `header.params.reorder_permille` (:245, U4r's) | npt-room's counts, npt-lab's room log |
| -21 | `event.forged_kind.sent_count`, `.unrejected_count` (:158-159, in 2.4's unit); `event.clamp_kind.sent_count`, `.counted_count` (:144-145); `event.clamp_report.x_um`, `.y_um`, `.z_um` (:322); `event.fast_clock.excess_cmds_count`, `.excess_move_um` (:155-156); `event.lag_burst.held_ns` (:170); `tick.v.lagswitch_excess_move_um` (:34); `tick.v.lagswitch_cmds_count` (:33); the per-session refusal counters (2.5) and the fast clock's per-episode refusals (2.6): names and rows by U4F-Q1 | npt-lab, from the room's and the pump's records |
| -22 | `tick.v.ack_starved_headroom_count` (:21, from `ClientSlot::headroom`, room.rs:282-287); `event.ack_starved.dropped` (:128); `second.v.rss_bytes` (:93, whose writer is npt-server: on L1 by U4F-Q1, NA-2); `header.params.event_queue_cap_count` (:442) with `event_queue_cap_provisional` (:344) | npt-room, npt-lab |
| -47 | `event.connect_clock.offset_err_ns` (:149) | npt-lab |
| all four | `header.params.cmd_timing` (:412), `params.suite` (:413), `params.twin` (:426) | npt-lab |

- The -20 and -58 press series are the same names (SERIES.md:41-42 list both marks): U4g's -58 reads U4f's writers.
- **logs.rs's `room_tick`** (logs.rs:495) gains these series: RR-B1-7 (1) rides that edit (section 9).

### 2.8 The event cap's measurement before -22's runs (carry 57; EVQ-LOAD; U4E-Q6, wave1 :473-479)

1. The room's cap is data, provisional at 1,024 (room/src/events.rs:38-40; room.rs:460-461), recorded as provisional
   in every header (SERIES.md:344). U4f measures and records plan §4.7's value before any -22 run (carry 57).
2. **EVQ-LOAD**, carried word for word: @@WORDS-N/advisor-brawl1-sets-p3-full.md:158-158@@ (the quoted carry line;
   also N/u4r-report.md:1048-1052).
3. **The measurement** (the builder's short runs, SS, 48 clients, the brawl, under the runner, `-x`): P2, P3 and B1
   (B1 with at least 2 bursts), 3 seeds each; per client the relevant gameplay-event rate and the queue's high-water
   mark per burst and per 30 s bin (`ClientSlot::queue`, room.rs:277-280; carry 58's figures).
4. **The value, by U4F-Q12:** plan §4.7's formula (:335) lands below the observed need for any margin up to about
   0.73 s (NA-3: 560 a second x (1 s + RTT + margin) reaches about 1,008 at a 0.7 s margin, while 1,024 already
   overflowed at P3 without a burst; the margin is free, so the formula reaches the need only with a larger one), and
   carry 50 bounds the cap from above for -22 (NA-12: a never-acker's events expire at 255 ticks, room/src/events.rs:
   19-20, so a cap above its rate x 255 / 30 s, about 3,900 at 460 a second, is never reached and -22's headroom check
   fails a correct build).
5. The chosen value, its derivation and the high-water figures go into plan §4.7 (the main session's commit: plan risk
   11, :953-954) and into the room's default with `event_queue_cap_provisional` false; the judged -20, -21 and -22
   runs run at it.

### 2.9 The client carries 80 (b)-(e) (npt-client; carries :518-534)

| Part | Site at 11fd190 | Test (2.10) | Bearing on Mustafa's open answers |
|---|---|---|---|
| (b) a refused far-ahead snapshot header moves the newest tick and resolves the span lost | `receive`'s refused-datagram sentinel (client/src/lib.rs:508-522) into `sentinel` (:631-641) and `see` (:583-615) | f24, asserting the bound U4F-Q16 rules: how far a refused header may move the newest tick, which is carry 80 (b)'s own bound (TE-11) | that bound is **not** C62H-BOUND's lost-tick bound, which is open with Mustafa (the card's "C62H-REVIEW ANSWERED" block, :479-492 at 11fd190; c62h-review :75-80): U4f designs no lost-tick bound and changes no -17 check; its report gives each judged run's lost count beside the share the profile's loss predicts, as c62h-review :55 records for the open answer |
| (c) a failing command drops the whole `Input`, acks included | `step` (:461-469) | f25: a page command whose aim lies outside the `Input` codec's range (the builder names the field and its bound in proto's codec) is dropped and counted (`unencodable_commands`, the builder's name; accessor in 2.13), the acks kept (TE-12) | none |
| (d) the client's two too-old cases | `too_old`, `old_header`, `bad_header` (:528-565) | f26 (carry 80 (d)'s endpoint test as worded) | **touches how the client files too-old ticks** (the sentinel and `stale`): C62H-KEEP and C62H-BOUND are open with Mustafa (c62h-review :68-80); no bound designed |
| (e) a full state after a snapshot of another hash files nothing | `full_state` (the scout's :810-839) and `file` (:645-658) | f27 | none |

KL-YES (the KEPT-LOGS kit lines) is open with Mustafa too; it bears on section 5's disk, not on these files.

### 2.10 Tests and negative cases (counts as deltas on `<base>`'s: plan §5 :507-512)

Each row adds its Δ tests in both builds unless marked "bites only"; "neg" marks a negative case (a refusal or a
hostile input). Minimum deltas: **measured +100; bites build +115** (the measured 100, f30, and 2.11's 14 bites-only
unit proofs); **60 negative cases** (the Δ of the rows marked neg). Conditional, in both builds: +3 by U4F-Q6 (b) (the
Pong field's codec round trip, the Pong's 32 B limit, the catalogue row's test); +1 by U4F-Q9 (a) (the kept window never
holds more than K under a flood of new sequences); +1 by U4F-Q11 (iii) (c) (the fast clock's fixed-input episode: the
arithmetic reference equals an honest client's movement); +1 by U4F-Q13 (a) (a refused command is resent, refused again
and counted once per (slot, seq)); +5 by T-6 if contract.rs is edited (section 9). The builder states the exact counts
and the `--list` lines that name them.

| Id | Test | Crate | Neg | Δ |
|---|---|---|---|---|
| f1 | ClockSync on a fixed P2 arrival trace at one phase per connect, as L1 gives it (SC-1): error at 2 s within the stated bound | client | | 1 |
| f2 | the three filter rules of ADR 0001:240-242, one test each | client | | 3 |
| f3 | outstanding pings and samples capped; an unknown pong id dropped and counted | client | neg | 2 |
| f4 | the ping rate never exceeds the room's limit (a faster rate is limited and counted by the pump) | client | neg | 1 |
| f5 | the controller's settled lead per profile (P0-P4, B1) on a seeded trace: at least 1, at most U4R-Q14's lead for that profile, and at most 1 % of commands arriving after their stamped tick (TE-15) | client | | 1 |
| f6 | the lab builds both options' endpoints with one controller setting, and npt-client reads no `cmd_timing` (TE-14) | lab | | 1 |
| f7 | option A runs several commands of one slot in a tick; the 9th is refused and counted (kind 10) | room | neg | 1 |
| f8 | carry 80 (a): one slot's commands 2^31 ticks apart; SA-1's triple on a fresh slot under option A; a property test over random u32 ticks: no panic and one order for every arrival order | room | neg | 3 |
| f9 | carry 80 (a): carry 21's room test (tests/room.rs:235) fails with the sort removed (a plant, restored; the test itself changes, so +0) | room | | 0 |
| f10 | kinds 1-6, 8 and 10 each refused and counted under its own kind, under both options | room | neg | 8 |
| f11 | one kind per refusal: a command breaking two checks is counted once, under the first; an honest late command is counted `late`, never kind 2 | room | neg | 2 |
| f12 | (i) an honest redundant copy is refused under no kind; (ii) a backward sequence with other content inside the window is refused under kind 3; (iii) an honest P3-like trace with redundancy 8 and reordered datagrams gives 0 refusals of any kind; (iv) a sequence older than the window is dropped and counted `seq_outside_window`, never as forged (TE-1, SA-2, TE-2, SA-9) | room | neg | 4 |
| f13 | (i) 8 far-ahead commands, then an honest command: the honest one runs (NA-7); (ii) under option B a command stamped equal to a buffered one is kind 2, never `late` (NA-13) | room | neg | 2 |
| f14 | every refusal field of `Counters` and of `ClientCounters` written per session (one test per struct, each field asserted), and a kicked session's written at its kick (SA-5) | lab | neg | 3 |
| f15 | each forger produces exactly its kind (its datagram decoded: only that check broken), 11 cases; kind 9's on datagrams only, both forms; kind 7's cooldown and reload edges | bots | neg | 11 |
| f16 | per kind (11) and the clamp, under A and under B: a short forged run and its twin give equal fate logs on every flow, equal snapshot group counts and equal state hashes on every tick (plan :389-390; SA-3) | lab | | 24 |
| f17 | carry 46: a forged press stream and its twin give equal state hashes on every tick, at the room's level | room | | 1 |
| f18 | the clamp cheat's points lie beyond reach and inside the map; the twin's points from the clamp reports give equal hashes on a short run; under option B a clamp command rerun after a move gives two reports for one sequence (SA-10) | bots, lab | | 3 |
| f19 | the 1.5x clock: excess commands at most 0 in the clean build, with lead-bound and not-newer refusals each above 0 per episode (SA-6) | lab | neg | 1 |
| f20 | the lag switch under P1: at most `CMD_CATCHUP` commands run per tick; movement within the catch-up cap's | lab | neg | 1 |
| f21 | the never-acker: headroom reaches 0, never below; one resync, then `Kick(overflow)` within `FULL_STATE_ACK_TICKS` (its later datagrams reaching no room call is U4r's r9, scenario.rs:2175, kept: TE-17) | lab | neg | 1 |
| f22 | the catch-up kind under option A and under option B, inside the bucket: the catch-up check refuses the excess, never the bucket or the lead bound | lab | neg | 2 |
| f23 | a known type on the control stream: refused and counted, the session kept (catalogue :87, :99) | lab | neg | 1 |
| f24 | carry 80 (b), asserting U4F-Q16's bound and the lost count | client | neg | 1 |
| f25 | carry 80 (c): the unencodable command dropped and counted, the acks kept | client | neg | 1 |
| f26 | carry 80 (d)'s endpoint test, each half failing under its plant | client | neg | 1 |
| f27 | carry 80 (e), failing under its plant | client | neg | 1 |
| f28 | every series of 2.7 written, with its SERIES.md row, on a short run of each suite at 5.3's lengths (the contract test's own check: plan §4.12 :412-415); `lead_ticks_count` absent under the controller (TE-20); each cheat's `input_limited` written | lab | | 1 |
| f29 | the -47 suite: 1,000 connects, one event each at 2 s, true offsets seeded | lab | | 1 |
| f30 | §4.11's bites per instance for U4f's new ids: a bitten and a clean endpoint and room on parallel threads (plan :399-401); bites only, since `with_bite` exists only in bites builds (plan :193-194) (TE-9) | client, room | | 1 (bites only) |
| f31 | carry 39: each rate-derived value U4f adds, pinned at 30 and 60 Hz: ClockSync's tick period in ns; the lead bound's ticks (U4F-Q8 (b): `ticks_for_ms_at` with the rate passed; under (a), npt-sim's own pin by carry 18's route). The lab keeps the 60 s episode and the 200 ms hold in ns on simulated time: no tick count (TE-10) | client, room | | 2 |
| f32 | honest stamps rise strictly and stay within the lead bound: a property test over seeded P0-P4 and B1 sample traces, and a case through a lead step-down and a ClockSync reset (TE-15, SA-8) | client | | 2 |
| f33 | npt-lab's attribution: per kind (11), a forged item counted rejected only under its own kind; a forged command refused under another kind counts unrejected; a planted copy that ran counts unrejected 1 (TE-23, SA-4) | lab | neg | 13 |

Kept, not new: the Send asserts (plan :402-403) cover `ClientEndpoint` with ClockSync and `Room<World>` with option
A; the runner's twin test (plan :406-407); -58's lt-cmd-seq and lt-press unit proofs (2.3); U4r's r8 and r9
(scenario.rs:2120, :2175).

### 2.11 The bites (17; plan §4.14 :453-476; marks.json)

Each planted where its code lives, shown by a unit test here, and run judged in its suite and option (plan :453-455,
:811-812). Bites-only, chosen per instance (plan :399-401); one home per id (check.mjs's `biteHomeProblems`).

| Bite | Site (crate) | Plant | Unit proof | Judged run (section 5) |
|---|---|---|---|---|
| PM-0001-20/redundancy-1 | the endpoint's redundancy (client/src/lib.rs:442-443) | repeats truncated to 1 | an `Input` holds one command (new) | -20 P3, option by U4F-Q10 (recommended B) |
| PM-0001-21/no-lead-bound | room, kind 1 | check removed | a far-ahead command buffered or run (new) | forged, option by U4F-Q10 |
| PM-0001-21/no-newer-tick | room, kind 2 | check removed | an equal-tick command runs (new) | forged, U4F-Q10 |
| PM-0001-21/no-seq-forward | room, kind 3 | check removed | a backward sequence with other content runs (new) | forged, U4F-Q10 |
| PM-0001-21/no-move-length | room, kind 4 | check removed | a long move runs (new) | forged, U4F-Q10 |
| PM-0001-21/no-press-edge | room, kind 5 | check removed | two edges in one covered tick run (new) | forged, U4F-Q10 |
| PM-0001-21/no-aim-bounds | room, kind 6 | check removed | an off-map aim runs (new) | forged, U4F-Q10 |
| PM-0001-21/no-cooldown | npt-sim's shot phase (NA-9; U4F-Q1) | check removed | a cooldown-breaking edge fires (new) | forged, U4F-Q10 |
| PM-0001-21/no-ack-sent | room, kind 8 (room.rs:783-787) | check removed | a never-sent ack's commands run (new) | forged, U4F-Q10 |
| PM-0001-21/no-trailing | npt-proto (quant.rs:40) | U4a's | U4a's and U4e's (proto/tests/bites.rs:211; server/tests/pump.rs:477) | forged, U4F-Q10 |
| PM-0001-21/no-catchup-cap | room, kind 10 | the per-tick cap removed | 16 commands run in one tick (plan :463) (new) | forged, option A (plan :463); in the lag-switch suite too only by U4F-Q15 (b) |
| PM-0001-21/no-channel | npt-proto (quant.rs:41) | U4a's | U4a's and U4e's (proto/tests/bites.rs:238; server/tests/pump.rs:483) | forged, U4F-Q10 |
| PM-0001-21/not-older-than | room, kind 2 | `>=` in place of `>` | an equal-tick command runs (plan :462) (new) | fast-clock, option A (plan :462; marks.json:5107-5108, :5118) |
| PM-0001-21/refuse-not-clamp | npt-sim's clamp (NA-9; U4F-Q1) | a refusal in place of the clamp | a beyond-reach point refused and counted (new) | clamp, U4F-Q10 |
| PM-0001-22/unbounded | the room's event queue (events.rs:22-24; room lib.rs:44) | U4e's | U4e's (room/tests/room.rs:1302, :1367) | never-ack over a full S30 (marks.json:5195; e-ruling :340-341) |
| PM-0001-47/newest-sample | ClockSync (2.1) | the newest raw sample | the estimate equals the newest sample (new) | clock, P2 |
| PM-0001-47/tick-quantised | the endpoint's arrival stamps (2.1) | the step's time in place of arrival | a sample's time moves to its step (new) | clock, P2 |

New unit proofs: 14 (redundancy-1, 11 of -21's 13, both of -47's); U4a's and U4e's three stand. A unit proof shows the
state changes, not that a suite reaches the site, so before each judged set 5.3 runs one bite per suite short and shows
it failing its check by name (TE-6). A bite run needs no twin: each fails another check of its part (README §5
:317-326), and keeps its part's validity rules, `pre:PM-0001-2` included (README :323-324), so each -20 and -21 bite run
must also pass PM-0001-2 (U4F-Q10's reason for -20's option).

### 2.12 -21's twins (plan §4.10 :360-390; Q16 (a) :1042-1045; advisor condition 1 :78-89 as amended at :477-479; carry 46)

| Kind | The twin's datagram in the forged one's slot | Source |
|---|---|---|
| refused whole by the room (kinds 1-6) | by U4F-Q13: (b, recommended) the twin's script never makes the refused command (its sequence index skipped), so neither run carries it after its refusal; or (a) the same datagram with the forged command, and every later copy of it, left out | condition 1 (only the refused part removed); SA-3 |
| the catch-up kind (kind 10), in one row (TE-18) | the twin's first datagram of that tick carries the commands the forged run's room accepted; every further datagram of that tick carries repeats only; the refused commands are never made (U4F-Q13 (b)), so the ticks they named run nothing (A) or the last command again (B) in both runs | plan :385-388; SA-3 |
| a datagram voided whole (kind 9) or its commands refused (kind 8) | the header only, no commands, its ack equal to the previous datagram's | plan :377-379 |
| a wrong-channel message on the control stream (kind 11) | a valid no-effect message (a ClientSettings repeating the current delay) | plan :382-384 |
| a sim-checked edge (kind 7) | the forged command with only the refused press removed: that button's counter one lower from the refused command on, every other field equal, exactly the presses `reports()` lists (carry 46) | plan :367-371; wave1 :197-235 |
| the clamp | the forged run's clamp reports' points at the same instants (one per command: 2.6's rerun rule) | plan :363-366; SERIES.md:322 |

Every client of the forged and clamp suites runs an open-loop seeded script (plan :380-381). f16 proves each row, per
kind and under both options, before the judged sets; f17 proves carry 46.

### 2.13 The seam rule and the public items, by crate (A19: plan §7.2 :930-931; §5 :512)

- **The seam:** U4f changes no seam type (`Command`, `Rewind`, `TickInput`, `SimCore`'s methods: plan §4.2). The two
  npt-sim bites (NA-9) add ids to `World::BITES` (sim/src/world.rs:937-938) and their sites, bites builds only, if
  U4F-Q1 opens crates/sim for them; any other sim change is a seam change for the advisor (carry 26's route).
- **Proposed public items** (engine-architect checks them in review; each named in the report against §4.1 and plan.md
  §4.7; anything else named as beyond them):

| Crate | Added or changed | Why |
|---|---|---|
| npt-client | `ClockSync` (plan.md:609; not in plan §4.1's row :193); a `ClientEndpoint` accessor for the clock estimate, which the lab reads for -47; the unencodable-command count's accessor (2.9 (c)); the controller's setting if U4F-Q7 keeps one; a sequence skip for the twin's script if U4F-Q13 (b); `BITES` to 6 ids | 2.1, 2.2, 2.9, 2.11, 2.12 |
| npt-room | option A behind the `cmd_timing` setting (a `Room` constructor or setting); the per-tick refusal records (slot, seq, kind) and the kept window, by U4F-Q9; the new counters in `ClientCounters` (room.rs:63-136), `seq_outside_window` among them; the lead bound's setting if U4F-Q8 makes it data; `BITES` widened by 9 ids | 2.3, 2.4 |
| npt-bots | the cheats' and the never-acker's constructors on `Behaviour` (plan §4.1 :197 names them) | 2.6 |
| npt-sim | none, or two ids in `World::BITES` (bites builds only) by U4F-Q1; a lead-bound tick constant only by U4F-Q8 (a), through carry 18's route | 2.11, 2.4 |
| npt-server | none (2.5), unless U4F-Q14 or U4F-Q6 opens it | |
| npt-lab | binary settings only (2.7); no new library item | |

---

(Sections 3-12 and the findings: being rewritten for rev 2; until this line is gone, take them from rev 1 at 828e2f4.)

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

(Sections 2-12 and the findings: being rewritten for rev 2; until this line is gone, take them from rev 1 at 828e2f4.)

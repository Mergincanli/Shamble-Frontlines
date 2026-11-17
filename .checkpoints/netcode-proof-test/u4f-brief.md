# U4f builder brief: clock and commands (rev 3.2, day draft)

rev 3.2, 2026-11-17, by the netcode-architect: the review budget's last fix pass, for round 2's two checks of rev 3.1
and the main session's D2 and D4 runs (the change list's first block); no third round unless a Critical or High is
open (none is). rev 3.1, by the same agent (workflow wf_e78b24a2-736, its revision step): rev 3 revised for the
test-engineer's and the security-auditor's checks (the change list's second block). rev 3, by the same agent (its draft
step), from rev 2 (the night's:
`git show 34a2e55:.checkpoints/npt-u4f-brief/u4f-brief-draft.md`, every cite at 11fd190; rev 1 at 828e2f4) and the
record since. Read only, at the head of section 0. Next (the card's U4f line): the advisor's check with the questions
of section 10 (N/u4-build-plan.md:1054-1056, U4-BRIEFS (a)); D2 and D4, the two runs before it, are filled (section 11). Nothing here is ruled: every choice the plan or a ruling leaves open is a
U4F-Q question, and the brief follows its recommendation only where it says "by U4F-Qn". The form is U4r's brief, rev
3.2 as filled at launch (N/u4r-brief.md), with C62H-BOUND's brief (N/c62h-bound-brief.md) for the stops and the texts
for his yes. Findings NA-1 to NA-16 are after section 12.

## Change list (rev 3.1 to rev 3.2: round 2, D2, D4)

Round 2's checks of rev 3.1, both PASS WITH FINDINGS with no Critical or High: the test-engineer's
(N/u4f-r3/r2-te-r31-full.json, TE31-1 to TE31-11; notes N/u4f-brief-r31-check-te.md) and the security-auditor's
(N/u4f-r3/r2-sa-r31-full.json, SA31-1 to SA31-4; notes N/u4f-brief-r31-check-sa.md); TE31-4 and SA31-2 are one finding.
Each finding done or rejected, with its reason, in N/u4f-brief-r3-notes.md ("Revision for round 2"). Nothing of rev
3.1's list below is undone but R3's wording (TE31-7). Every cite is at the head: spikes/ unchanged since 0ad9661.

| # | Finding or source | What changed | Where |
|---|---|---|---|
| R11 | TE31-1 (Medium) | by U4F-Q7 (iii) (c): a target more than 2 ahead is reached 2 a step, the owed stamps carried (none unused) while it lies at most `REDUNDANCY_CAP` ahead of the previous stamp; only beyond that (a ClockSync reset) are stamps skipped and counted, expected 0; the controller's lead moves at most 1 a step; rev 3.1's skip kept as option (e), sound under Q7 (b) only (under (a) about 1.2 % of P3's ticks unstamped: the test-engineer's model); f36 asserts `stamps_skipped` at most 0.1 % | 2.2; 2.10 f36; Q7 |
| R12 | TE31-2 (Medium) | 5.3 runs no twin, so the forged and clamp twin checks read a Gap (judge/src/figures.rs:99-109; eval.rs:346-347): listed beside the length-failing checks; the reading rule per run (a clean partner: every check but those passes; a bitten run: its named check fails, and the checks its bite reaches may fail too, p9 and p12); p1's twin bar dropped; p2-p6 spelled out; p13 named as the one bitten run expected to pass; D12 the same | 5.3; 11 D12 |
| R13 | TE31-3 (Medium) | p19 reports apart the commands dropped late by W, the `input_limited` datagrams, the kind-10 refusals and `lagswitch_cmds_count`'s max (at most 8), with P1's arithmetic: the Input bucket of 8 (server/src/pump.rs:20, :84) and W act before the cap, so an honest hold never puts more than 8 new commands into a tick; the cap is shown acting by the forged suite's kind 10; Q15 (c)'s why restated | 5.3 p19; Q15 |
| R14 | TE31-4 = SA31-2 (Medium) | the first stamp: no command is due, and none stamped, before the endpoint holds a snapshot (under U4F-Q7 (b) also an estimate) and a target above its `JoinAccepted` tick, which the endpoint keeps (today it keeps only the slot: client/src/lib.rs:496); the first stamp is the controller's target, so `newest.map_or(0, ...)` (:429) is never reached under the controller; f32 (iii) new (+1); f13 (iv) the anchor's boundary (+1, neg) | 2.2; 2.10; 2.13 |
| R15 | SA31-1 (Medium) | by U4F-Q18 (ii) (d), new: the room keeps per slot index the newest accepted stamp across join, replace, leave and kick (a fixed array of its slots), and a join's kind-2 reference is the later of `JoinAccepted`'s tick and that stamp, by signed distance; a command between them dropped under `rejoin_overlap` (report-only), never kind 2; (e) the exposure written up instead; f39 conditional (+1, neg); LEAD-BOUND's "per session" clause by Q18 (ii) | 2.3; 2.4 item 4; 2.10; 2.13; Q18; 12 item 6 |
| R16 | TE31-5 (Medium) | f16's pairs each assert their forger's item inside the measured window and the twin's matching datagram; the forging interval a lab setting (`--forge-every`, a new header row); f29 under U4F-Q4 (a) or (c) asserts the events per round against carry 75's bound, and the 1,000 connects run once in D7 | 2.6; 2.7; 2.10 f16, f29; 5.6 B2 |
| R17 | the Lows: TE31-6, TE31-7, TE31-8, TE31-9, TE31-10, TE31-11 = SA31-4 (a) and (b), SA31-3, SA31-4 (c) | each applied: p20 read as every command dropped late, so "f38 shows it"; the after-idle wording (the room's limit the bound plus W, the checks' allowance the bound alone); 5.5, Q2 and KL-U4F in MiB from D2, -20's and -21's disk figures corrected, C62H-BOUND's two closed sets named; Q18 in 12 item 4; f36's model with L1's reorder step, the wall-time rule on every new test, a stop at 1.5 times `<base>`'s test step, f37 on the lab's own episode function; kind 1's anchor, kind 3's copy changed in place; every tick compare a signed distance, cases at room tick 1, Q17's reachable triple; kind 10's view tick | 1 item 5; 2.3; 2.4; 2.6; 2.10; 2.12; 5.3; 5.5; 5.6 B3; Q2; Q8; Q17; Q18; 12 items 4, 6; R3 |
| R18 | D2 (the main session, 18:54 UTC) | filled: 5,468 MiB free (1,372 above the floor) and each folder's size; 5.5's room about 1,752 MiB after the u4r-P3 pack, before the part-clean (the shared target holds 6,346 MiB) | 11 D2; 5.5; Q2; 12 item 6; NA-5 |
| R19 | D4 (two models and a noise run) | filled: under U4F-Q6 (a) and (e) no estimator passes 5 ms; with (b)'s exact stamp the mean passes (2.35 ms at 99,000 connects) and both bites show, while the lowest-RTT pick sits on the bar (4.856 ms); by U4F-Q6 (b) the filter averages and f1 pins 4.0 ms; NA-4 confirmed; Q6's status and figures (not ruled) | 2.1; 2.10 f1; 5.3 p11; Q6; 11 D4; NA-4 |
| R20 | the recheck | rows 51-54 new: the judge's twin Gap; the join's fresh slot and the room's first tick; the endpoint's pre-snapshot stamp and view clamp; L1's reorder step and P0's delivery, the Input bucket, `after32` | 8 |

**Counts at rev 3.2** (rev 3.1's below): tests, measured build **+109** (f13 (iv), f32 (iii)); bites build **+124**;
negative cases **64** (f13 (iv)); conditional: **+1** by U4F-Q18 (ii) (d) (f39), the rest as rev 3.1; short pre-runs
**15** (conditionals as rev 3.1); questions **18** (Q7 (iii) gains (e), Q18 gains (ii)); recheck rows **54**; open items
in 9.1 **19**; disk about **3,607 MiB (up to about 3,727)** and the build phase, against **1,372 MiB** above the floor at
D2 plus about 380 from the u4r-P3 pack plus the part-cleans (the shared target holds 6,346 MiB; its part-clean's return
is not measured); bars for him as rev 3.1: B3 recommended (LEAD-BOUND, reworded), B5-B7 only if ruled, KL-U4F only if
D2 at launch shows the room short after 5.5's items 1 and 2; planned size about **4,850** added lines (3,900 to 5,900;
rev 3.2 adds about 100), the stop past 6,000 as before.

## Change list (rev 3 to rev 3.1: the two checks)

The test-engineer's check (N/u4f-brief-r3-check-te.md: TE3-1 to TE3-15) and the security-auditor's
(N/u4f-brief-r3-check-sa.md: SA3-1 to SA3-8), each finding done or rejected, with its reason, in
N/u4f-brief-r3-notes.md ("Revision for the checks"). Nothing of rev 3's list below is undone. Every cite is at the same
head as rev 3's (0.1): spikes/ unchanged.

| # | Finding | What changed | Where |
|---|---|---|---|
| R1 | TE3-1 (High) | the honest client's command rule under the controller: a command for every stamp (stamps contiguous), the old "only when the newest moved" rule for `with_lead` runs only, by U4F-Q7 (iii); f5's bounds restated (the lead bound less a margin; 1 % late by carry 74); f36 new (-20's two shares on a seeded P3 trace); p5 and p6 read -20's two share checks by name, a miss to the advisor | 2.2; 2.3; 2.10; 5.3; 11 D15; Q7 |
| R2 | TE3-2 (High), SA3-2 (Medium), TE3-4 (Medium) | the `Input` codec's two rules stated; per kind: how it is forged, what the forger's endpoint stores, what "rejected and counted" means, and its twin; kind 3 an added copy of an accepted sequence with changed content; kinds 1, 2, 4, 5 and 6 forge the newest copy only; a later copy of a refused sequence is a repeat whatever its content; the twin keeps the forged run's sequence numbers (U4F-Q13 (b): index and stamp skipped) and leaves out of an `Input` any command older than a skip (none at P0); kind 3's own twin row; f15, f16 assert every forged and twin `Input` encodes; f33's kind-3 and kind-7 cases; the fast clock's stamps never decrease. **Rejected:** SA3-2's twin "with no command at that step, its sequences consecutive", since the state hash carries command sequences (sim/src/world.rs:1979, :1995-2009) | 2.4 items 2, 4, 6; 2.6; 2.12; 2.13; 2.10; Q11 (v); Q13 |
| R3 | SA3-1 (High) | the join anchor (a fresh slot's kind-2 reference is its join's tick) under both options; new U4F-Q18: option A's past window W = `CMD_CATCHUP` (recommended (a)); f13 (iii); f38 and p20 by Q18 (a); under option A, after an idle, the room's own limit is the bound plus W, while -21's fast-clock checks still allow the bound alone (reworded at rev 3.2, TE31-7; Q8, section 12 item 6) | 2.3; 2.4; 2.10; 5.3; 7; Q8; Q17; Q18; 12 |
| R4 | TE3-3 (Medium) | each short bitten run names the one check that must fail, read by name from its checks, and its clean partner passing that check; the count checks that fail by length on every short run listed; p21, a clean clock run, new; each bar in c-u4f-r2.mjs's expect line; D12 rewritten | 5.3; 11 D12 |
| R5 | TE3-5 (Medium) | f16 compares the fate logs entry by entry without the datagram's size; `fate_log_sha256` only for same-run identity; the reading of plan :389-390 to the advisor | 2.10 f16; 9.1 |
| R6 | TE3-6 (Medium) | the fast clock's schedule (one client, 1.5x from the measured start, ten back-to-back 60 s episodes); its suite's measured window 610 s, in the header; f37 asserts the schedule at the judged settings; 5.1's -21 cite corrected | 2.6; 2.7; 2.10 f37; 5.1; 5.5 |
| R7 | TE3-7 (Medium) | each lab test's scale in P2_SHORT's form; the episode length and the connect count lab settings; a wall-time stop | 2.10 |
| R8 | TE3-8, SA3-5 (Medium), TE3-10 (Low) | 5.5 gains the build phase; at launch, before the builder starts, the u4r-P3 pack and the shared target's part-cleans; packing only u4r-P3 (8.1's last folder, PACK-READ's second); a U4f set's own folder only after his yes to KL-U4F; net figures | 5.5; Q2; 12 items 6, 13 |
| R9 | TE3-9 (Medium) | P24's start key made unique (found once) | 0.5; 12 item 3 |
| R10 | the Lows: TE3-11, TE3-12 = SA3-6 (2), TE3-13 = SA3-4, TE3-14, TE3-15, SA3-3, SA3-6 (1), (3), (4), SA3-7, SA3-8 | each applied: 2.10's generator rule; D3 as a Grep-tool search; the gap case (f12 (v)); the plants in the return list; :1148's wording; kind 4's width; the no-socket rule; Q4 (a)'s key; Q6 (b)'s client-sent field (+1); the fast clock's `ping_limited`; option A's cap on arrival | 2.4; 2.6; 2.10; 4; 7; 11 D3; Q4; Q6; Q8 |

**Counts at rev 3.1** (rev 3's in the table below): tests, measured build **+107** (f12 (v), f13 (iii), f36's two,
f37); bites build **+122**; negative cases **63** (f12 (v), f13 (iii)); conditional: U4F-Q6 (b) **+4** (SA3-6 (4)),
**+1** by U4F-Q18 (a) (f38), the rest as rev 3; short pre-runs **15** (p21) (+4 by U4F-Q11 (iv), +1 by U4F-Q15 (c),
+1 by U4F-Q18 (a)); questions **18** (Q18); recheck rows **50** (46-50 new: the codec, the hash's sequences, the fate
log, the join tick, U4r's run lines); open items in 9.1 **19** (3 new); disk about **3,608 MiB (up to about 3,728)**
and the build phase (no figure) against 1,418 MiB of room plus about 380 MiB from the u4r-P3 pack plus the
part-cleans (D2); bar changes for him: as rev 3 (B3 recommended; B5-B7 only if ruled); one more entry for him only if
D2 shows the room short: **KL-U4F** (12 item 6); planned size about **4,750** added lines (3,800 to 5,800; rev 3.1
adds about 350), the stop past 6,000 as before.

## Change list (rev 2 to rev 3)

Status: **applied** (a ruling's or his words' effect, by placeholder), **re-pointed**, **answered**, **open** (a
U4F-Q), **new**, **filled** (a D-item). Rev 2's own change list (TE-1 to TE-24, SA-1 to SA-17, SC-1) stays as it was:
nothing of it is undone here.

| # | What changed | Source | Where in rev 3 |
|---|---|---|---|
| C1 | Every cite at the head read; rev 2's 11fd190 cites that moved or went listed | `git diff --stat 11fd190 <head>` (0.1) | 0.4; sections 1-11 |
| C2 | Placeholders `<<P1>>` to `<<P36>>` in u4r-brief.md's form replace rev 2's 14 `@@CARRY-n@@` and 15 `@@WORDS-…@@`; each anchor shown found once | N/u4f-r3/stamps_anchors.py (its output: 0.5) | 0.5; 12 item 3 |
| C3 | Answered: C62H-KEEP (a), C62H-BOUND (a) and C62H-BOUND-YES (a), KL-YES with its change, KL-CHANGE-YES, KL-MERGE (a); C62H-BOUND built, merged and judged | history :2131-2175 (`<<P19>>`, `<<P20>>`, `<<P21>>`); N/u4r-report.md from :1201 | 2.9; 9 (rev 2's three "open with Mustafa" rows closed); Q16 |
| C4 | What C62H-BOUND changes where carry 80 (b) and (d) touch the client's filing: its two -17 checks read that filing, the lab's short P2 test guards it, his stop rule applies; U4f designs no lost-tick bound | marks.json:3983-4005; lab/src/scenario.rs:2904; N/c62h-bound-brief.md:599-607 under `<<P21>>` | 2.9 |
| C5 | C62HB-Q4 (b): -58's two lost-tick checks and ADV-CBR-1 are U4g's | history :2204-2227 (`<<P23>>`); N/advisor-c62h-bound-report-head-full.json (R1, ADV-CBR-1); card, Open findings "From C62H-BOUND" | 2.3's -58 row; 9 |
| C6 | His 13:35 rule: every marks.json, SERIES.md and judge change U4f could make listed, each a new or changed bar or not; every bar to him under Decisions needed before it is built, in c62h-bound-paste.md's form; the reviewer's read named | history :2125 (`<<P15>>`), :2127 (`<<P16>>`); N/advisor-c62h-review-full.md:82-93 | 5.6 (new); 12 item 6 |
| C7 | His command-forms rule in the build rules | history :2187 (`<<P17>>`) | 4 |
| C8 | KEPT-LOGS in U4F-Q2's disk answer: §11's rule, PACK-READ (a), the protected folders never packed without his word, packing by brief 8.1 | .claude/CHECKPOINTS.md §11 (:231-249); history :2212 (`<<P22>>`); N/c62h-bound-brief.md:485-501; card, RECS-1043 and R4 | 5.5; Q2 |
| C9 | Every deferred Low since 11fd190 placed: taken where U4f opens the file anyway, else left with its taker | card Open findings; merge-stop (deferred Lows); N/u3c-held-1/build.md:116-122 | 9.2 (new) |
| C10 | JDG-ONLY-1's interim rule and ADV-CBH-1's lesson on every judge call U4f plans | N/advisor-u4r-c62h-full.md:20 (`<<P31>>`); head-check ADV-CBH-1, ADV-CBH-2, H2 | 5.4 |
| C11 | -21's measured runs and their twins judged in one call | judge/src/figures.rs:191-214; ops/runs/c-u4r-runs.mjs:42-43 | 5.4 |
| C12 | Build rules brought to the rules in force: the lane runner from u4r-run.sh (e9437ae2…), logs under .gate/u4f/; SIZE-CHECK for an M unit with a planned size and the stop | N/u4r-brief.md section 4; history :2048-2051 (`<<P18>>`), :2067-2075 | 1 item 4; 4 |
| C13 | New U4F-Q17: carry 80 (a)'s two routes (the night card's Next step 4) | carries :515-520 (`<<P1>>`); the night card | 2.3; Q17 |
| C14 | U4F-Q6 gains option (e): the room's own pings carry the server's ns | server/src/pump.rs:387-404; catalogue :91 | 2.1; Q6; D4 |
| C15 | U4F-Q8 gains (iii): whether the lead bound's value goes to him as the fast-clock checks' allowance | marks.json:5026-5049; `<<P15>>` | Q8; 5.6 |
| C16 | U4F-Q1: (a) drops `rss_bytes`'s L1 writer (the report reads "not available"); gains (d), RR-UH1-2's SERIES.md half | judge/src/eval.rs:1504-1556; N/u3c-held-1/build.md:116-122 | Q1; 2.7; 9.2 |
| C17 | U4F-Q4's why gains carry 75's admission bound (NA-16) | lab/src/scenario.rs:70, :995-1006; carries :357-365 (75) | Q4; 2.7 |
| C18 | New NA-14: option A's `tick.s.input_missing` read by stamp; f34 | SERIES.md:63 | 2.3; 2.10 |
| C19 | New NA-15: `Room::leave` drops a slot's counters whole; f35 | room/src/room.rs:689-695, :739 | 2.5; 2.10 |
| C20 | New NA-16: the -47 suite meets carry 75's admission bound | as C17 | 2.7; Q4 |
| C21 | NA-2 changed: -22's `rss_bytes` report reads "not available", never INVALID | eval.rs:1504-1556; marks.json:5208-5216 | NA table; 2.7 |
| C22 | NA-5 changed: the disk at the last reading (5,514 MiB after U3C-HELD-1's E6) and where room comes from | card (U3C-HELD-1 E6); N/c62h-bound-brief.md:489-501 | 5.5; Q2 |
| C23 | NA-3's proof: no log writes the queue's length; EVQ-LOAD's 27 resyncs are the proof (D5 filled) | lab/src (no `event_queue_count` writer); `<<P32>>` | 2.8; 11 |
| C24 | Recheck walked: 33 rows (25 hold, 8 changed), 12 rows added | the code at the head | 8 |
| C25 | D1-D15 walked: 4 filled, 11 left, each with its command, pass bar and when | section 11 | 11 |
| C26 | f16's twin rule adds the same tick set in both runs; f29 asserts no admission refused | figures.rs:191-214; scenario.rs:995-1006 | 2.10 |
| C27 | U3C-HELD-1 closed before U4f's merge, as the merge-stop ruling's C3 placed it | merge-stop C3; N/u3c-held-1/u3c-held-1-report.md | 9 |
| C28 | Section 12 in u4r-brief.md's form; `<base>` stays a placeholder | task | 12 |

**Counts:**

| Item | rev 1 | rev 2 | rev 3 |
|---|---|---|---|
| Tests, measured build (Δ on `<base>`) | +60 | +100 | **+102** (f34, f35) |
| Tests, bites build (Δ) | +74 | +115 | **+117** |
| Conditional, both builds | T-6 +5 | T-6 +5; Q6 (b) +3; Q9 (a) +1; Q11 (iii) (c) +1; Q13 (a) +1 | the same |
| Negative cases (measured) | 44 | 60 | **61** (f35) |
| Bites | 17 | 17 (+1 by Q15 (b)) | 17 (+1 by Q15 (b)) |
| Judged runs | 65 | 65 (+1 by Q15 (b); +13 by Q10 (c)) | the same |
| Short pre-runs (5.3) | 9 + 5 (or 9) | 14 (+4 by Q11 (iv); +1 by Q15 (c)) | the same |
| Placeholders | 14 + 15 | 14 + 15 | **36** `<<P>>` (P1 holds three carries), plus `<base>` and `<<PV1>>` |
| Recheck rows (section 8) | 29 | 33 | **45** (33 walked: 25 hold, 8 changed; 12 added) |
| Open items (section 9) | 15 | 17 | **16** rows in 9.1 (the 3 answered in one row, 1 closed, 2 new, 12 from rev 2; rev 2's TE-B1-R2-1 and RR-B1-7 (1) rows move to 9.2); **23** deferred findings in 10 rows of 9.2 (4 taken) |
| Questions | 12 | 16 | **17** (Q17 new; none of Q1-Q16 fully answered: 10's status lines) |
| Needs a run (day) | 10 | 15 | 15: **4 filled** (D1, D5, D6, D9's premise), 11 left |
| Bar changes for Mustafa | - | - | **0** by the recommendations; **1** recommended for his yes (the lead bound's value, -21); **3** only if a ruling picks them (-21: Q11 (iii) (a), Q14 (c), Q15 (b)) |
| Disk, every set and pre-run (estimate) | about 3,750 MB | about 3,596-3,667 MiB vs 3,593 MiB of room | the same estimate vs **1,418 MiB** of room at the last reading (5.5) |
| Planned size (SIZE-CHECK) | - | - | about **4,400** added lines (3,500 to 5,500); the stop past **6,000** (4) |

---

## 0. How to read the citations

### 0.1 The head read

- **The head read:** 7cfae885ca58beba55a08ead2cdb31942f881f15 (`git rev-parse HEAD` in the main folder, 2026-11-17 17:35
  UTC, the saver's checkpoint commit). The draft began at 8afcc11 (DOCS-54's close); every commit between them
  touches .checkpoints/ only (`git diff --stat 8afcc11 7cfae88 -- spikes docs .claude` and the four stamped files
  print nothing), and so do the saver's later commits while it was written (892081a, 42e0912: the same diff from
  7cfae88 prints nothing). Its spikes/netcode-proof is 0ad9661's, U3C-HELD-1's fix: `git diff --stat 0ad9661 HEAD --
  spikes/netcode-proof` prints nothing.
- **Since 11fd190** (rev 2's head; `git log 11fd190..HEAD -- spikes/`): 914798a REFS-KIT (check.mjs's refs step's one
  exception, for the KEPT-LOGS kit files); C62H-BOUND's eb2bebf (npt-l1's `server_datagrams_since`, npt-lab's link
  record `tick.s.snap_link_lost_count`), b06991e (npt-judge's two figures and their shared reader), 15102d1
  (PM-0001-17's two checks and one planted bug; SERIES.md, marks/README.md), 561bf42 (the set runner
  c-u4r-runs.mjs); its judged results cb37f56 (P0 re-judge), daec788 (P2), 8ca774a (P3); the kept-logs lists b5cb253,
  6c8c7cf, 978202d; U3C-HELD-1's 0ad9661 (crates/net/src/l2/delay.rs and delay_tests.rs, test code only, and
  SERIES.md's two ws_*_held_bytes rows). Main taken in at 2bd44a5 (PRs #52, #53) and 3f2fa21 (PR #54). Not taken in:
  main c94c81b (PR #55, docs only; the card's "From DOCS-54" note applies at the next take-in).
- **What did not change since 11fd190:** crates/room, client, bots, server, sim, proto, link, log, hash-wasm;
  lab/src/contract.rs; ops/runs/c-u4r-r2.mjs; docs/adr/0001-multiplayer-netcode.md; the catalogue;
  .claude/rules/netcode.md; N/u4-build-plan.md. So every rev 2 cite into them holds unless section 8 says otherwise.
  What changed: check.mjs (+17), check.test.mjs, crates/judge, crates/l1 (+33, after :411), crates/lab (logs.rs +54,
  scenario.rs +311), crates/net, marks/ (marks.json +26 inside -17, SERIES.md, README.md), c-u4r-runs.mjs (+47),
  results/.

### 0.2 Stamps at the head (sha256, lines; N/u4f-r3/stamps_anchors.py, read only)

| Short name | File | sha256 | Lines | Since 11fd190 |
|---|---|---|---|---|
| `plan` | N/u4-build-plan.md | 43ff74f2a3ce903f1061c67f53b416ae2498f3890f2116426feeab4373656385 | 1,100 | unchanged: every plan cite of rev 2 holds |
| `plan.md` | N/plan.md (rev 3.3) | 3eecd28196a529969e9cce1e6eee9dd35100b796414242b835903557e28bc09b | 1,525 | REFS-KIT: +1 at :453, +6 at :662-667, so rev 2's cites :453-:661 are +1 and those after :667 +7 |
| `carries` | N/u4s-carries.md | 57600c9c79bdccfe521dc1181ee9ab9d86087e11c38b788b9b223072c9c01dce | 682 | +2 at :322-323 (C62H-KEEP's answer), so entries 63 on are +2 |
| `history` | N/history.md | 3bfe1aa2d5a3f781267956e011079c42b6cc12e61a058af1af9e395076246854 | 2,232 | appended at its end only |

Closed records (each placeholder into one asserts its sha256 first): `df-ruling` advisor-u4df.md 842ec092…; `j-r2`
advisor-u4j-r2.md 7efa9410…; `m9` advisor-u4d2-m9.md f31f7876…; `d2-r2` advisor-u4d2-r2.md e776f0ba…; `te-m3`
advisor-te-m3.md 5ecc3e23…; `c62h` advisor-u4r-c62h-full.md 18dd3d20…; `b1-p3` advisor-brawl1-sets-p3-full.md
1bc5889c…; `r-b2` advisor-u4r-build2-full.md f4f5ac92…; `r-r1` advisor-u4r-r1-full.md 7b552e8c…; `head-check`
advisor-c62h-bound-head-full.json a64259f5…; `merge-stop` advisor-c62h-bound-merge-stop-full.json 55ad3561…;
`c62h-review` advisor-c62h-review-full.md 7d2dd26b…. Living (copied by text anchor): carries, history, the card,
N/u4r-report.md (c5dee3fa…, 1,764 lines). The runners: u4r-run.sh e9437ae200d234725b392bc5090e2b802d9938bb8ba5b12c23123c3d607e97d7
(117 lines; TE-B1-R2-2's fix, as N/c62h-bound-brief.md:60 has it); main-run.sh
8df2dee311a6f00d147b876be787313e54900309625cfbabd12059aac86d0ae6 (88). The marks at the head: marks.json
c1935a5b996fb0aaff82b0b3c1c604255e1534f6ca7e8d2e55a9f472cd3b22a9 (C62H-BOUND's "after"), SERIES.md
1dd67e2f2db8e3c569a23fa0580dd2fc47c240d94526d6440348df7dcc41725c (U3C-HELD-1's), c-u4r-runs.mjs f2002890…, c-u4r-r2.mjs
4e007500….

### 0.3 Short names

`N/` = .checkpoints/netcode-proof-test/; `card` = .checkpoints/netcode-proof-test.md, cited by block or item, never by
line; `catalogue` = docs/design/netcode-proof-messages.md; `ADR 0001` = docs/adr/0001-multiplayer-netcode.md;
`marks.json`, `SERIES.md`, `README` = spikes/netcode-proof/marks/'s; a crate path (`room/src/input.rs`) is under
spikes/netcode-proof/crates/; `check.mjs` is spikes/netcode-proof/check.mjs; run scripts are under
spikes/netcode-proof/ops/runs/. Rev 2's ruling names stand (`advisor` advisor-u4.md, `wave1`, `e-ruling`, `e-r1`,
`e-r2`, `r-check` advisor-u4r-brief.md, `r-r1`, `r-b2`, `c62h`, `c62h-review`, `te-m3`, `df-ruling`, `j-r2`, `m9`,
`d2-r2`, `b1-p3`), plus: `c62hb-brief` N/c62h-bound-brief.md; `c62hb-paste` N/c62h-bound-paste.md; `merge-stop`;
`head-check`; `report-head` N/advisor-c62h-bound-report-head-full.json; `held-review`
N/u3c-held-1/advisor-plan-review-full.json; `held-build` N/u3c-held-1/build.md; the C62H-BOUND review records
`cbr-test`, `cbr-net`, `cbr-rust` (N/c62h-bound-review-{test,netcode,rust}-full.json).

### 0.4 Rev 2's cites at 11fd190 that moved or went (every other one holds; section 8 walks the code rows)

| Rev 2's cite | What it named | At the head |
|---|---|---|
| carries :506-509, :510-534 ((a) :513-518, (b) :518-520, (c) :520-522, (d) :522-529, (e) :529-534), :535-537 | 46, 80 (a)-(e), 86 | :508-511; :512-536 ((a) :515-520, (b) :520-522, (c) :522-524, (d) :524-531, (e) :531-536); :537-539 |
| carries :343-354, :364-371, :372-381 | 74, 76, 77 | :345-356, :366-373, :374-383 |
| carries :305-321, :322-327, :447-473, :476-493 | 62 (h), 63, 64, 67 | :305-323 (C62H-KEEP answered at :322-323), :324-329, :450-475, :478-495 |
| plan.md :461, :468, :470, :485-491, :486, :489, :506-525, :510-514, :524-525, :553-566, :596-598, :609, :617-628, :618-622, :625 | E3, E10, E12, E7's endpoint, arrival timing, wrong channel, §4.4's table, the checks' counters, simulated time, §4.6, §4.7's entries, ClockSync, npt-lab's entry, RQ8, the host clocks | each +1: :462, :469, :471, :486-492, :487, :490, :507-526, :511-515, :525-526, :554-567, :597-599, :610, :618-629, :619-623, :626 |
| plan.md :765-775 | seeds 1 to 3 | +7: :772-782 |
| marks.json -20 :4618-4778 (parts :4632, :4646; checks :4684-4721; report :4743-4759), -21 :4779-5119 (parts :4792-4890; checks :4909-5035; bites :5086-5112; notes :5118), -22 :5120-5202 (:5133, :5145-5155, :5182-5190, :5195), -47 :9046-9109 (:9059, :9069-9094, :9097, :9101) | the four marks | +26 (C62H-BOUND's insert in -17): -20 :4644-4804 (A :4658, B :4672; checks :4709-4767; report :4769-4791); -21 :4805-5145 (parts :4816-4931; checks :4933-5074; bites :5087-5140; notes :5144); -22 :5146-5228 (:5158, :5170-5193, :5208-5216, :5218-5223); -47 :9072-9135 (:9084, :9095-9120, :9123, :9127) |
| SERIES.md :322, :342, :344, :358, :412, :413, :426, :442 | clamp report, lead_ticks, cap provisional, client_counts, the cmd_timing, suite, twin and cap params | :324, :344, :346, :360, :415, :416, :429, :445 (rows before :258 hold) |
| lab/src/scenario.rs :126-131, :441-455, :458-509 (:459), :846-861, :1158, :1165-1186, :1282-1299 (:1286), :1404-1405, :204, :212-216, :2120, :2175 | lead_for, session ends, io and the Pong's tick, the endpoint's build, the fate hash, client_counts, params, twin, LAB_BITES, Bites::of, r8, r9 | :132-137; :458-472 and :522-555; :475-562 (:476); :897-913; :1242, :1276; :1259-1271 (:1270); :1366-1418 (:1370); :1411, :1488; :215-219 (3 ids now); :227; :2212; :2267 |
| lab/src/logs.rs:495 | `room_tick` | :524 (its lint allow :520-523) |
| check.mjs:1957 | WASM_CRATES | :1970 (LEAF :39 holds) |
| c-u4r-runs.mjs :131-132, :138-153, :163-166, :171, :176-178, :324-332 | floor, byte rates, split, run limit, judge fit, `planJudged` | :133, :138-154, :167 (SPLIT_LINE) and :36-45, :172, :177-179, :312-327 (`bitesOf` :312, `planJudged` :327) |
| room/tests/room.rs:1302, :1367 | -22/unbounded's unit proofs | :1303 (`unbounded_case`), :1363 (the bites test), :1444 (the clean headroom test): rev 2's two lines were off by a few, the file unchanged |
| N/u4r-report.md:1048-1052, :1137-1141, :1162 | EVQ-LOAD, TE-B1-R2-1 with RR2-B1-1, RR-B1-7 (1) | :1049-1053, :1138-1142, :1163 |
| card :479-492, :483-484, :419-461, :1467-1474, :1527-1528, :1930-1933, :1960-1964, :3013-3088 | rev 2's card lines | the card is cited by block from rev 3 on (living) |
| rev 2's section 9 rows C62H-BOUND, C62H-KEEP, KL-YES | open with Mustafa | answered (C3) |

### 0.5 Placeholders and units

- Every place where the brief holds someone's words (the carries, Mustafa's words, the advisor's sentences the card
  makes binding) is a placeholder `<<Pn>>` naming its file and its anchor; the main session copies each by script at
  launch (section 12 item 3), asserting the anchor found once and the copy's first and last words. Mustafa's words
  appear only so; none is retyped or paraphrased as his. The anchors' counts at the head (stamps_anchors.py): P1 to
  P36 each found once (section 12 item 3 gives each span's lines); P24 by its start key, whose bare first words stand
  twice in df-ruling (rev 3.1, TE3-9; N/u4f-r3/r31_check.py counts the key once). Section 12 item 3's anchors (each span's first and
  last words, u4r-brief.md §12's form) are the only other place any of his words stand, as the fill script's keys:
  N/u4f-r3/brief_check.py finds no run of 7 of his words anywhere else in this brief, and at rev 3.1
  N/u4f-r3/r31_check.py (punctuation ignored, section 12 item 3 found by its own text) finds none either, after
  TE3-15's rewording of U4F-Q8's why. Values: `<base>` (the spike
  branch's head at launch) and `<<PV1>>` (u4f-run.sh's sha256).
- **Units:** disk and memory in MiB (2^20 B); a source figure in MB (10^6 B, as c-u4r-runs.mjs:138-154 states its
  byte rates) is converted, its MB figure in brackets.
- **Mustafa's standing rules, by range only:** rev 2's list (0, last bullet) stands; plus SIZE-CHECK, his words
  :2048-2051 (`<<P18>>`) and the main session's reading :2067-2075; KEPT-LOGS (c) :2088-2111 and §11; the 13:35 rule
  :2125, :2127 (`<<P15>>`, `<<P16>>`); C62H-KEEP, C62H-BOUND and KL-YES :2131-2146; KL-CHANGE-YES :2148-2161; KL-MERGE
  and C62H-BOUND-YES :2163-2175 (`<<P21>>`); NS-STATUS and the command forms :2177-2187 (`<<P17>>`); NS-LINES
  :2189-2197; MERGE-54 :2199-2202; RECS-1043 :2204-2227 (`<<P22>>`, `<<P23>>`); MERGE-55 :2229-2232. The card's
  "Rules in force" lists them.

---

## 1. Start

1. **Base:** `<base>` (section 12 item 1). U4f depends on U4r (plan :805), merged; C62H-BOUND goes before U4f's build
   (c62hb-brief :79-81, C11) and is merged; U3C-HELD-1 goes before U4f's merge (merge-stop C3) and is closed (card).
   U4f, U4g and U4h run one at a time (plan :882-883). In your worktree: `git merge --ff-only <base>`; if that cannot
   fast-forward, `git switch -c u4f-build <base>`; then `git log -1 --format=%H` shows `<base>` and `git merge-base
   --is-ancestor 0ad9661 HEAD` exits 0. Record both in your notes first.
2. **Who and where:** one builder, general-purpose, opus, effort xhigh, in an isolated worktree under
   `/home/user/Shamble-Frontlines/.claude/worktrees/` (`isolation: worktree`; plan §7.2 :925-927), under the shared
   target rule and W1-TARGET's lock (section 4). The judged runs are not yours: the main session runs them after the
   merge, in the main folder (plan :928-929; section 5).
3. **Your notes:** `.checkpoints/netcode-proof-test/u4f-build.md` in your worktree. First line: this task. Then your
   plan as blocks (one type with its tests, one check with its counter, one cheat, one bite and its proof, one run
   script); each finished block written at once, a notes line per finding, one line after each block (done, what
   next, `git diff --stat <base>`'s added lines). On a restart: your notes and `git diff` first, then go on after the
   last block marked done (section 4's builders' limits).
4. **The runner:** `/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4f-run.sh` (`<<PV1>>`: made by the
   main session at launch from u4r-run.sh, e9437ae2…, by script, only the lane's names changed: the header, the log
   folder .gate/u4f/, the usage line; its .diff beside it; its sha256 and `bash -n` exit 0 at launch; section 12 item
   2). Its use is u4r-run.sh's (N/u4r-brief.md:265-274): `bash <runner> [-x] [-k] <worktree> <log> <command...>`, the
   lane lock for that one call (`-x` exclusive), 4 GB free checked, the touch, the shared target and
   `CARGO_NET_OFFLINE=true`, each exit code and the provenance lines appended; every log under
   `/home/user/Shamble-Frontlines/.gate/u4f/`. How a run's result is read: `<<P14>>`.
5. **What stops you** (each written in your notes and your report, then build the rest): a file outside section 4's
   lane; an item another unit merged changed beyond what this brief names; a manifest or Cargo.lock change (the main
   session's); a new external dependency; disk below section 4's floors; a part a U4F-Q leaves open at launch; a tick
   count or rate-derived constant you need (carry 39's rule, `<<P8>>`: record name, definition, values at 30 and 60 Hz
   and site, build the rest); a wire change beyond U4F-Q6's ruling (the catalogue and its tests move with it:
   .claude/rules/netcode.md:15); **any edit to marks.json, an existing SERIES.md row or crates/judge** (5.6: a bar
   comes to Mustafa first, `<<P15>>`); **the lab's short P2 C62H-BOUND test failing** on your build
   (`c62hb_p2_short_every_measured_lost_tick_is_explained`, lab/src/scenario.rs:2904: under the C62H-BOUND stop
   rules his yes adopted, `<<P21>>` with c62hb-paste :120-124, it stops the work for him); **your added lines past 6,000**
   (SIZE-CHECK, section 4); rev 3.1: no controller lead within 2.2's bounds reaching -20's two shares on f36's P3 trace
   (TE3-1); a new lab test slower than 2.10's scale rule allows (TE3-7); a forged or twin `Input` that does not encode, or
   a twin that leaves a command out at P0 (2.6, 2.12); rev 3.2: any new test past 2.10's time rule, or the test step past
   1.5 times `<base>`'s (TE31-10); f1's 4.0 ms not met, by U4F-Q6 (b) (D4).
6. **Read before you build** (and nothing you do not need):
   - plan: §5 U4f (:805-816); §4.1's rows for npt-client, npt-room, npt-server and npt-bots (:193-197); §4.2's refusal
     rule (:239-249); §4.3 (:251-265); §4.4 (:267-290); §4.5's window (:301-303); §4.7 (:330-336); §4.8 (:338-341);
     §4.9's tolerance (:351-356, context only: U4h's); §4.10 (:360-390); §4.11 (:392-407); §4.12 (:409-425); §4.13's
     shot band and SS's scripts (:427-447); §4.14 (:453-476); §5's preamble (:507-512); §5 U4e (:714-748) and U4r
     (:750-803); Lanes (:851-889); §7 (:909-932); §9 (:958-968); §10's readings rule (:971-974); Q16, Q21, Q22
     (:1042-1045, :1054-1060);
   - plan.md: E3, E10, E12 (:462, :469, :471); E7's endpoint (:486-492); §4.4's table (:507-526); §4.6 (:554-567);
     §4.7's npt-room, npt-client and npt-lab entries with RQ8's order and the host clocks (:597-599, :610, :618-629);
   - ADR 0001: Part 5's checks, acknowledgements and Time (:224-255); OQ-0001-6 (:1156-1167); rows PM-0001-20, -21,
     -22 (:1610-1612) and -47 (:1644);
   - marks.json: -20 (:4644-4804), -21 (:4805-5145), -22 (:5146-5228), -47 (:9072-9135), PM-0001-2 (:151-625), and
     -17's C62H-BOUND checks (:3972-4005) and bites (:4030-4039) as the guard on carry 80's filing; SERIES.md's rows of
     2.7 and C62H-BOUND's four frozen rows (:291, :292, :322, :378); README §2.5 (:187-192), §4 (:247-290), §5
     (:317-330);
   - catalogue: where each check runs (:52-59); the Input, Ping, Pong and Kick rows (:87, :91-92, :99); the authority
     note and the logging rule (:120-123);
   - u4-scout-netcode.md: F104 (:122), F117 (:135), F59 (:77), F79 (:97), F108 (:126), F119 (:137), F121 (:139);
   - the rulings sections 3 and 10 name; advisor conditions 1, 4, 5 and 10 (:78-89 amended at :477-479, :112-124,
     :143-149), the F2 ruling's point (6) (:463-465), wave1 W1-U4D N-M2 (:197-239), U4E-Q6 and U4E-Q11 (:473-479,
     :492-495), e-ruling U4E-Q12 (:217-244, :340-341), r-check U4R-Q4 and U4R-Q14 (:198-205, :289-358); since rev 2:
     merge-stop (C1-C8 and its deferred-Lows ruling), head-check (ADV-CBH-1, ADV-CBH-2, H2), c62h-review :82-93;
   - .claude/rules/netcode.md (:1-17): binding on proto, room, client and server code (plan §7.1 item 8, :921-922;
     advisor :192-193). U4f writes room and client code and feeds the server's pump: every rule there binds it;
   - the code at `<base>`, by section 8's rows.

---

## 2. What to build

U4f's row (plan :806-809): ClockSync; the client's tick-lead controller; `InputBuffer` option A; every other room-side
check with its -21 counter; the server-side checks' -21 counters and judged runs on U4e's pump (U4E-Q11); the cheats'
`Behaviour`s. The marks it moves (plan :810-815): -47, -20, -21 and -22 (section 5). Nothing here adds an engine
feature beyond those rows (CLAUDE.md's "for later" rule); an item a table marks "proposed" is built only once section
10's ruling names it.

### 2.1 ClockSync, the command clock (npt-client; plan §4.8 :338-341; ADR 0001:239-251; net F117 :135)

| Field | What U4f builds | Source |
|---|---|---|
| Home | `ClockSync` in npt-client, owned by `ClientEndpoint` | plan.md:610; plan §4.8 |
| What it estimates | the offset from the client's host clock to the server's clock, signed integer ns: the command clock only. The interpolation clock is `Interp`'s (U4g: plan :822); ClockSync exposes nothing for it now | plan §4.8; net F117 |
| Its samples | by U4F-Q6: the client's own `Ping` (id, the host time of the step that sends it) and the room's `Pong` (id, echoed time, tick, hold time), each timed by its datagram's arrival stamp from `ClientEndpoint::input`'s `received` (client/src/lib.rs:414-424), never by the step's own `now`; under U4F-Q6 (e) also the room's own `Ping` (id and the server's clock in ns at its send: pump.rs:387-404), timed the same way | ADR 0001:242-245; catalogue :91-92; plan.md:487 |
| The server's stamp | as U4F-Q6 rules. As built the room answers a `Ping` in the I/O step of the instant it arrives (plan.md:619-623; hold about 0) with the next tick to run (lab/src/scenario.rs:476, :480; server/src/pump.rs:458-467), so each sample's server instant is known only to one tick period (NA-4); the room's own pings carry its exact ns (pump.rs:396-399) | U4F-Q6 |
| The filter | ADR 0001:240-242's rules (drift at most 1-2 ms per frame, halve errors over 100 ms, reset over 500 ms); every threshold in ns, data, pinned by a test. **By U4F-Q6 (b) (rev 3.2, D4):** the estimate averages its samples (the mean of the 2 s of samples, or a filter at least as good on f1's trace), never the lowest-RTT pick alone: with an exact stamp D4 puts the mean's p99 at 2.35 ms and the lowest-RTT pick's at 4.856 ms, 0.14 ms under the bar (99,000 connects: N/u4f-r3/d4/d4-compare.md), so a judged run of the latter passes or fails by chance; f1 pins 4.0 ms. ADR 0001:239-242 names no pick ("from its own ping samples"), so this needs no reading | ADR 0001:239-242; net F117; D4 |
| Ping rate and phase | data, at most the room's limit (10 a second per session: catalogue :91; the pump's bucket, pump.rs:458-471). The client sends a `Ping` only at a step, and its steps fall every 1/TICK_HZ of its host clock (client/src/lib.rs:485-488), so **every ping period is a whole number of ticks**: on L1, with an honest host rate, one connect's samples meet the server's tick grid at one phase, moved only by the delay jitter (P2: ±10 ms each way, l1/src/lib.rs:149-163, :214-221, against a 33.3 ms period). The filter may not count on random phases, and the builder may not move a ping off the step grid to get them (the endpoint reads no clock): SC-1 is met here and in D4's phase-locked model. `ping_limited` 0 on a clean run, read from 2.5's per-session counters | SC-1; net F117 |
| Bounds | outstanding pings and kept samples each capped (as the pump's `OUTSTANDING` 8 and `SAMPLES` 8: pump.rs:24, :26); a `Pong` naming none of its pings is dropped and counted (`unknown_pongs`, client/src/lib.rs:403-410, today crate-private: public only if the lab logs it) | .claude/rules/netcode.md:13-14 |
| Clock reads | none: the endpoint reads no clock (ADR 0001:245-247; client/src/lib.rs:5-6) | ADR 0001 |
| Carry 39 (`<<P8>>`) | the tick period in ns comes from the rate passed in (production callers pass `npt_sim::TICK_HZ`), pinned at 30 and 60 Hz (f31); no local tick constant | carries :169-179 |
| What -47 reads | npt-lab writes `event.connect_clock.offset_err_ns` (SERIES.md:149) per connect: abs(estimate - true) at the first client step at or after 2 s from the connect's first sample (the builder states this instant in its doc and report); the true offset is the lab's seeded host offset (lab/src/scenario.rs:18-19, :930; plan.md:626) | marks.json:9095-9111 |
| Bites | `PM-0001-47/newest-sample` and `PM-0001-47/tick-quantised`, in `npt_client::BITES` (client/src/lib.rs:28-32, growing to 6 with 2.11's -20 bite), through `ClientEndpoint::with_bite` (:314-326); the ids spelled as marks.json's (:9123, :9127), not as the plan's words at :810 (NA-11). A lowest-RTT filter keeps the samples the tick-quantised bite delays least, so its bite may not fail: D4 models it and 5.3 runs it short first (TE-6). D4 (rev 3.2): by U4F-Q6 (b), both bites show against the mean (tick-quantised 11.9 ms; newest-sample 8.9 ms, against 2.35 ms clean); under (a) and (e) every clean estimate already fails, so neither bite can show | marks.json; plan §4.14 (:454-455) |

### 2.2 The client's tick-lead controller (npt-client; net F104 :122; carry 74; r-check U4R-Q14 :289-358)

| Field | What U4f builds | Source |
|---|---|---|
| What it replaces | the endpoint's fixed lead (`lead`, client/src/lib.rs:214, 1 by default :267; `with_lead` :296-301, whose doc names U4f's controller as its replacement; the lab sets it from `lead_for`, lab/src/scenario.rs:132-137, :900); whether `with_lead` stays as a lab setting is U4F-Q7 | carry 74 (`<<P4>>`) |
| What it sets | each command's stamped tick (today `newest + lead`, :429) | ADR 0001:250-251 |
| Its form | U4F-Q7: (a) the newest snapshot tick plus a lead from the client's own RTT and jitter samples, by U4R-Q14's arithmetic (L = ceil((rtt + jitter) / P) + 1, r-check :296-302); or (b) ClockSync's estimate of the server's tick at the command's arrival plus a margin and the buffer target (net F104 :122) | U4F-Q7 |
| Stamps rise strictly | each stamp is strictly above the previous command's (the room refuses any other as kind 2: ADR 0001:231-233); the controller lowers its lead only by holding (no command at that step), never by stamping back; a ClockSync reset moves the estimate, never a stamp backwards (TE-15, SA-8; f5, f32) | ADR 0001:231-233 |
| **Stamps contiguous (rev 3.1, TE3-1; by U4F-Q7 (iii))** | under the controller each step gives the commands for every stamp from the previous stamp + 1 up to the controller's target, at most 2 a step (a count, no rate: carry 39); none when the target is not above the previous stamp (a hold, which lowers the lead and leaves no tick without a command); **rev 3.2 (TE31-1), by U4F-Q7 (iii) (c):** a target more than 2 ahead is reached 2 a step, the owed stamps carried (none unused) while it lies at most `REDUNDANCY_CAP` ahead of the previous stamp, and the controller moves its lead by at most 1 a step; only a target beyond that (a ClockSync reset) leaves the stamps between unused, counted per client (`stamps_skipped`, the builder's name; report-only, 5.6 B2), expected 0 in every measured window of an honest run (under Q7 (a) the newest tick jumps 3 or more in about 1.16 % of P3's steps, so leaving those stamps unused would leave about 1.2 % of P3's ticks unstamped; carried, the backlog stays at most 3: the test-engineer's model, N/u4f-brief-r31-check-te.md B3). **The first stamp (rev 3.2, TE31-4 = SA31-2):** no command is due, and the endpoint stamps none, until it holds a snapshot (under Q7 (b) also a ClockSync estimate) and a target above its `JoinAccepted` tick, which the endpoint keeps (today it keeps only the slot: client/src/lib.rs:496); the first command is stamped at the controller's target, never counted up from 0 or from the join tick, so the endpoint's `newest.map_or(0, ...)` stamp (:429) is never reached under the controller (f32 (iii)). The endpoint tells its caller how many are due (a `ClientEndpoint` accessor, 0 before the first stamp is due: 2.13). Why: as built a client gives one command only when its newest tick moved (bots/src/lib.rs:14-21, :224-227), so at P3 about 20 % of its ticks get no stamped command (U4r's P3 runs: 700,948 commands and 178,595 steps without one, results/u4r/0f066338dd8d/P3/runs.txt:18; at P0 0: results/u4r/051995db9cea/P0/runs.txt:22), against -20's "missing inputs at most 0.1 % of player-ticks" (marks.json:4724-4737) | TE3-1; SERIES.md:63 |
| Same under both options | one controller under option A and option B (net F104); the endpoint never reads `cmd_timing` (README :191-192 makes it the room's and the scenario's); the lab builds both options' endpoints with one controller setting (f6) | net F104 |
| Inputs | ClockSync's samples only (the client's own measure), never the profile (the lab's knowledge: `lead_for` is U4r's stand-in) | ADR 0001 part 1 |
| Bounds | at least 1; at most the room's lead bound less a margin (2.4 kind 1; the builder states the margin); under option B also at most 9, so the buffer's 8 ahead hold every command (input.rs:114-116); the endpoint keeps at most `REDUNDANCY_CAP` = 8 unacknowledged commands (client/src/lib.rs:442-443) | carry 27 (`<<P6>>`); ADR 0001:231-233 |
| Pass marks it serves | -20's two shares at P3 under both options: at least 99.9 % of commands before their tick runs and at most 0.1 % of player-ticks missing (marks.json:4710-4737; f36; p5, p6). U4R-Q14's fixed lead 7 gave 1,112 late of 700,948 at P3 (0.16 %: P3 runs.txt:18), so the controller may settle above it; if no lead within its bounds reaches 0.1 % on f36's trace, the builder stops and reports (section 1 item 5). Carry 74's 1 % late rule on 5.3's other runs, read under option B as the room's `late` count and under option A as the commands that arrived after their stamped tick ran (1 - `tick.s.cmd_on_time`'s share, SERIES.md:61), both reported on every short run (TE-4); the lead it settles on per profile reported beside U4R-Q14's numbers (P0 2, P1 3, P2 and B1 5, P3 7; P4 9 by the same arithmetic: l1/src/lib.rs:193-196) | carry 74; TE3-1 |
| The bots' rules | **under the controller** (rev 3.1, by U4F-Q7 (iii)): each step gives the commands the endpoint says are due (above: none, one or two; none before the first stamp is due, as bots/src/lib.rs:14-16's snapshot floor today), so no tick goes without a stamped command but by loss; **under `with_lead`** (U4r's suites, the -30 pre-check: U4F-Q7 (i)) unchanged: one command per client step, only when the newest tick moved (bots/src/lib.rs:14-21). Both: at most one new edge per press counter per command (U4R-Q14's (b1) and A4) | r-check :322-323; TE3-1 |

### 2.3 `InputBuffer` option A beside U4e's option B (npt-room; OQ-0001-6, ADR 0001:1156-1167)

| Field | Option A (U4f builds) | Option B (U4e's, kept) |
|---|---|---|
| When a command runs | at the first room tick after it arrives (pumped before the tick: plan.md:622-623), several in one tick within `CMD_CATCHUP` = 8 per client, each one's tick strictly newer than the last run | exactly one per client per tick, the one stamped for it (room/src/input.rs:127-151) |
| A tick with none | nothing runs for that client (plan §4.13 :439-441); with contiguous stamps (2.2) only a lost or late command leaves a tick without one | the last command runs again (:135-141) |
| Late | by U4F-Q18 (a) (rev 3.1, SA3-1): a command stamped more than W = `CMD_CATCHUP` ticks before the tick it would run at is dropped and counted `late` (never forged; it never runs); else a strictly newer command runs whenever it arrives; on time is still read against its stamp (SERIES.md:61) | stamped for a tick already run: dropped and counted `late` (:107-113). Today a stamp equal to a buffered command's is `Late` too (:110): under 2.4's order kind 2 refuses it first (NA-13) |
| **The join anchor** (rev 3.1, SA3-1) | a fresh slot's kind-2 reference is the tick its `JoinAccepted` names (the room's next tick at the join: room.rs:681), so a first command stamped at or before it is kind 2 (2.4 item 4). **Across joins (rev 3.2, SA31-1), by U4F-Q18 (ii) (d):** every join builds a fresh `InputBuffer` (room.rs:648-653) while the slot's soldier lives on (a replacement kicks the old session, no respawn: lab/src/scenario.rs:458-472), so the room keeps per slot index the newest accepted stamp across join, replace, leave and kick (a fixed array of its slots, never keyed by client data), and a join's kind-2 reference is the later, by signed distance, of its `JoinAccepted` tick and that kept stamp; a command after the join's tick but at or before the kept stamp is dropped under a non-forged counter (`rejoin_overlap`, the builder's name; report-only, 5.6 B2), never kind 2, so an honest rejoin counts nothing as forged | the same, both rows (its `late` check needs a first `take`: input.rs:23, :131) |
| **Missing, by stamp (NA-14)** | `tick.s.input_missing` (SERIES.md:63: "one per player this tick: 1 when no command for this tick had arrived") is written by its stamp: 1 when no command stamped for this tick had arrived before it ran, never "no command ran this tick" (option A runs commands on arrival, so arrivals bunch: at P3's ±15 ms one-way jitter against a 33.3 ms tick, ticks with none run are common, and the run-reading would fail -20's 0.1 % on a correct build); f34 | the same words, the buffer's stamp |
| The per-tick cap | at most 8 run per tick; the excess refused on arrival, once 8 new commands are held for the coming tick, and counted under the catch-up kind (2.4 kind 10), so the room's list never holds more than `CMD_CATCHUP` whatever the pump lets through (SA3-8) | the buffer holds at most 8 (`Full`, :114-116), counted under the catch-up kind |
| The setting | `params.cmd_timing` "A" or "B", a start-up setting, never changed in a run, recorded in every header (README :191-192; plan.md:462); today fixed to "B" (lab/src/scenario.rs:1370) | the same |
| Depth recorded | U4F-Q3 (the card's DEPTH ruling and its option-A note) | `buffer_depth_ticks` (input.rs:63-66); the measured L - 1 reading (the card's DEPTH ruling) |
| -58's two sites | through U4f's rewrite of `offer`, the sequence-forward compare (input.rs:82; its name `CMD_SEQ`, :11) and the press-edge compare (:98; `PRESS`, :13) keep their sites, names and unit proofs: PM-0001-58/lt-cmd-seq and lt-press (plan :468-469), judged at U4g (plan :829-831) (TE-19). -58's two new lost-tick checks are U4g's (C62HB-Q4 (b): `<<P23>>` and RECS-1043, history :2204-2227), with ADV-CBR-1's caveat in U4g's brief carry (report-head, R1; card, Open findings): **not U4f's**. U4f writes the press series -58 shares with -20 (2.7) | the same |
| **Carry 80 (a) first** (`<<P1>>`, carries :515-520) | before option A runs several commands of one slot in a tick, `canonical` (room/src/room.rs:361-369) takes a total key, by U4F-Q17: within a slot, the tick's signed distance from the tick being run (`tick.wrapping_sub(t) as i32`), which orders any set of u32 ticks and equals today's wrap-aware order for ticks within 2^31 of `t`. The carry also allows the lead-bound route; rev 2 took the total key as a requirement, which rev 3 asks as U4F-Q17 (the bound alone leaves a non-total triple reachable from a fresh slot, SA-1: ticks T-2^31+1, T-2^30 and T+1, none 2^31 from T, with no last run to refuse against, and in the no-lead-bound bite run, where a panic makes the run INVALID: README :250-251; TE-21). Rev 3.1's join anchor (U4F-Q18) closes the fresh-slot route; the bite run's stays (rev 3.2, SA31-3: in the no-lead-bound bite run under option A, t-5, t+2^30 and t+2^31-2 in one tick, each newer than the last and none late, whose pairwise wrap-aware order cycles). Tests f8 and f9 (2.10) | carry 80 (a) |

### 2.4 The command checks, each with its -21 counter (npt-room unless named; plan.md:513-515; catalogue :87)

**The counting rule and the order (by U4F-Q9; TE-1 = SA-2, TE-2, SA-9, SA-4, NA-13):**
1. **Kind 8 first**, on the whole datagram (room.rs:756-788); kinds 9 and 11 at the pump, before the room sees it.
2. **Per command, oldest first** as now (room.rs:791). A sequence not after the newest received (input.rs:81-84) goes to
   the **repeat filter**, never counted as forged: content equal to the command kept under that sequence is a redundant
   copy (each `Input` re-sends every unacknowledged command: client/src/lib.rs:442-443, :464), dropped and counted
   `commands_repeat` as now (room.rs:795); content that differs from an **accepted** command kept under that sequence,
   inside the kept window, is **kind 3**; a later copy of a **refused** sequence is a repeat whatever its content (it
   can never run: SA3-2); a sequence older than the kept window, or one inside it with no command kept under it (a gap:
   a hostile skip, or an honest command after a long loss), is dropped and counted under its own non-forged counter
   (`seq_outside_window`, the builder's name), never as forged (TE3-13 = SA3-4). The kept window: the last K commands
   per client received under a new sequence, accepted or refused (each kept with that mark), K = `REDUNDANCY_CAP` plus
   a stated reorder allowance, bounded (.claude/rules/netcode.md:13-14). An honest reordered datagram's commands were
   all received in the later datagram that overtook it, so they land in the filter, or, past K, in the outside-window
   counter (TE-2).
3. **A new sequence** becomes the newest received (input.rs:89, kept by U4F-Q13 (b)), then meets kinds 1, 2, 4, 5 and
   6 in that order, then kind 10 (option A: on arrival, once 8 new commands are held for the coming tick, SA3-8; option
   B: `Full`), with option B's late rule and option A's past window (U4F-Q18) after kind 2.
4. **Kind 2** compares the stamp with the newest accepted command's (run or buffered: NA-13), or on a fresh slot with
   the join's tick (the join anchor: 2.3; U4F-Q18; across joins by its (ii) (d)), so an honest late command (stamps
   rise strictly: 2.2) passes it and is counted `late`, and a forged equal or older stamp, or a first stamp at or before
   the join, is kind 2, never late. **Every tick compare** (rev 3.2, SA31-3) of kind 1, kind 2, the join anchor and W is
   a signed distance (`stamp.wrapping_sub(t) as i32` against the bound or `-(W as i32)`, or `Wrap::after32`,
   proto/src/wrap.rs:60-65), never `t + bound` or `t - W` in u32: the room's ticks start at its sim's tick + 1
   (room.rs:439), so a plain `t - W` underflows in the first ticks: a debug or test build panics on network input, and
   a release room wraps and drops honest commands as late (release overflow checks are on for npt-sim and npt-proto
   only: Cargo.toml:116-120). f13 (iii) and f38 each hold a case at room tick 1.
5. **One kind per refusal**, the first check that refuses; npt-room reports per tick each refused command as (slot,
   seq, kind), as npt-sim's `reports()` does for its own (plan §4.2 :246-247).
6. **The unit per kind (SA-4):** one forged command (slot, seq) for kinds 1-7 and 10; one forged datagram for kinds 8
   and 9; one forged message for kind 11. npt-lab counts a forged item "rejected and counted" only as its kind's rule
   says (rev 3.1, TE3-2), each checked against every tick's `TickInput`:
   - kinds 1, 2, 4, 5, 6 and 10: a refusal record of that kind names (slot, seq), and no command of that (slot, seq)
     runs in any `TickInput`;
   - kind 3 (a resent copy, with changed content, of a command the room accepted: 2.6): a kind-3 refusal names (slot,
     seq), and no command with the forged copy's content runs in any `TickInput`; the honest original of that (slot,
     seq) ran, as expected;
   - kind 7: the sim's refusal record (slot, seq, Cooldown or Reload: sim/src/world.rs:1401-1420, :1445-1452) for the
     forged edge, so the press was not applied; the command itself runs (expected), and the twin's equal hashes show
     the refused press changed nothing (2.12);
   - kinds 8, 9 and 11: below.

   `unrejected_count` (SERIES.md:159) counts the items failing their rule and is never negative; kind 8 is read from
   `ack_refused` (datagrams: room.rs:785), never `ack_refused_commands`; kinds 9 and 11 are matched by count per session
   (the pump keeps counts, not records), each forger's datagrams or messages carrying only its own kind (f15, f33).

Each kind is one check of ADR 0001:231-239 (plus the wrong channel, plan.md:490), 11 in all (marks.json:4946-4957,
:5144).

| # | Kind (forged command) | Home, place in the order | At the head (unchanged since 11fd190) | U4f builds | Counter (builder names it; SERIES row by U4F-Q1) | Bite (2.11) |
|---|---|---|---|---|---|---|
| 8 | acknowledgement of a snapshot never sent | room, 1st, whole datagram | the datagram's commands refused (room.rs:783-788), `ack_refused`, `ack_refused_commands` | kept; counted as this kind, by datagram | `ack_refused` | PM-0001-21/no-ack-sent |
| 3 | sequence not forward | room, in the repeat filter | `Repeat`, counted with honest copies (input.rs:81-88; room.rs:795) | a refusal apart from honest repeats (NA-6) | sequence refusals | PM-0001-21/no-seq-forward |
| 1 | tick too far ahead (the lead bound) | room, 1st for a new sequence | none (NA-7) | the check; its form and value by U4F-Q8 | lead-bound refusals | PM-0001-21/no-lead-bound |
| 2 | tick equal to or older than the newest accepted (on a fresh slot: at or before the join's tick, the join anchor) | room, 2nd | caught only as `late` under B (input.rs:107-113), an equal-to-buffered stamp included (:110: NA-13) | the check, apart from `late`, with the join anchor (2.3) | not-newer refusals | PM-0001-21/no-newer-tick; PM-0001-21/not-older-than (`>=` for `>`, plan :462) |
| 4 | move length above 1 | room, 3rd | none (the sim scales a long vector) | x² + y² ≤ 127², integers, each part widened to i32 before it is squared and no `abs()` on a wire value (move_xy is [i8; 2]: sim/src/seam.rs:35; release overflow checks are on for npt-sim and npt-proto only: Cargo.toml:116-120) (SA3-3; bots/src/lib.rs:30-31) | move refusals | PM-0001-21/no-move-length |
| 5 | more press edges than ticks covered | room, 4th | `PressRefused` (input.rs:90-106), counted `press_refused` | kept; the counter is this kind's | `press_refused` | PM-0001-21/no-press-edge |
| 6 | aim outside the map | room, 5th | none | x and y inside the greybox's extent (±12,800 cm: sim/src/greybox.rs:62, :151; the room's `extent`, room.rs:441); z left to the codec's range and the clamp by design (SA-14); beyond-reach points pass on to the sim's clamp | aim refusals | PM-0001-21/no-aim-bounds |
| 10 | more commands in a tick than the catch-up cap | room, last | option B's `Full` (`buffer_full`) | option A's per-tick cap, refused on arrival (SA3-8); both counted as this kind | catch-up refusals | PM-0001-21/no-catchup-cap |
| 7 | fire or reload edge against the sim's state (cooldown, reload; ammunition cannot occur in SS: plan :434-435, sim/src/world.rs:1411-1419, so it stays with U4d's unit test per kind, advisor :87-88: SA-15) | npt-sim, in the step | the sim refuses and records (slot, seq, kind) in `reports()` (world.rs:1401-1420, :1445-1452; plan §4.2 :239-249) | the lab's attribution from `reports()`; the bite's site is npt-sim's (NA-9, U4F-Q1) | the sim's refusal counters | PM-0001-21/no-cooldown |
| 9 | trailing bytes or an unknown field (a set reserved or pad bit: proto/src/quant.rs:115-122) | npt-server's pump (plan :287-290) | `trailing_bytes`, `unknown_field` (pump.rs:126-128, :172-173) | the -21 series only (U4E-Q11); forged on `Input` datagrams only, both forms (SA-12) | those two | PM-0001-21/no-trailing (npt-proto, quant.rs:40) |
| 11 | a known type on the wrong channel | npt-server's pump | `wrong_channel` (pump.rs:124, :171); refused and counted, never closing a joined session (pump.rs:414-421; catalogue :87 against :99) | the -21 series only | `wrong_channel` | PM-0001-21/no-channel (npt-proto, quant.rs:41) |
| - | beyond the weapon's reach | npt-sim's clamp (plan §4.2 :227-231; world.rs:1283-1312) | clamped, reported, never refused (`event.clamp_kind.counted_count` counts refusals only: SERIES.md:144) | the clamp cheat (2.6) | none by design | PM-0001-21/refuse-not-clamp (npt-sim, NA-9) |

The plan's other room-column checks stay with their units: the ClientSettings clamp (U4g: plan :822); the view-time
tolerance and the rewind cap (U4h: plan :835-837); Resync, roster, token and the event cap (U4e, built).

### 2.5 The server-side checks' -21 counters on U4e's pump (U4E-Q11, wave1 :492-495)

U4e built the checks and their unit tests; U4f writes their -21 series. The counters exist per session (`Counters`,
pump.rs:110-161; plan §4.1 :195): among them `too_large`, `trailing_bytes`, `unknown_field`, `wrong_channel`,
`input_limited` (the bucket of 8, refilled at 2 x TICK_HZ a second: catalogue :87), `ping_limited`, `pong_unknown`,
`join_again`, `before_join`. **Every refusal counter reaches a log (SA-5):** today none of the pump's does, and the
room's `client_counts` footer writes some (lab/src/scenario.rs:1259-1271; SERIES.md:360). npt-lab writes every refusal
field of each session's `Counters` and of its `ClientCounters` (room.rs:63-136) once per session at its end and at the
stop (names and SERIES rows by U4F-Q1), so the forged suite reports per client every counter other than the forger's
own kind's (0 expected on honest clients and on each forger outside its kind). **Where each end happens (NA-15):** the
pump's `Decoder` dies with its session (a replacement, scenario.rs:458-472; a `Leave`, :513-534; a closed link,
:544-555; a kick, :592-599); the room's `ClientCounters` die when the room frees the slot, `Room::leave` setting it
`None` whole (room.rs:689-695), which a `Leave` does inside `Room::receive` (:739) and a closed link through the lab
(scenario.rs:549). So npt-lab reads a session's two sets of counters before it routes that session's `Leave` to the room,
before it calls `Room::leave`, and before it drops a `Decoder`; a kicked slot keeps its `ClientSlot` (kicked, until a
rejoin replaces it: room.rs:1057-1063, :637-686), so a kicked never-acker's room counts survive its kick (f35). How
U4E-Q11's "judged runs" read for the size and bucket checks (no forged kind of the 11 is theirs: plan :385-388), and
whether the hold-time clamp (pump.rs:510, uncounted; plan.md:515 names it) gains a counter, are U4F-Q14. U4f changes no
pump code by this brief; if U4F-Q14 or U4F-Q6 needs any, the lane opens npt-server (U4F-Q1) and section 6's
`--all-features` item applies.

### 2.6 The cheats' `Behaviour`s (npt-bots; plan.md E12 :471; plan :808-809; bots/src/lib.rs:10-11)

Every cheat is a measured-build behaviour (the suites are measured runs), clock-injected and transport-free like U4r's
(bots/src/lib.rs:5-13). A forged datagram is the honest endpoint's own datagram, decoded and re-encoded with only the
kind's change (npt-bots depends on npt-proto: bots/Cargo.toml:23), so forged content rides only inside datagrams the
honest script sends anyway, at the same instants, on the same flows (plan :376). Each cheat client's `input_limited`
is reported per run, 0 expected (SA-13; 2.5).

**The `Input` codec's two rules** (rev 3.1, TE3-4, SA3-2), which every forged and twin datagram keeps, since an `Input`
that breaks one does not encode and the endpoint then sends nothing in that slot (client/src/lib.rs:466-469), so the
two runs' flows part: (1) one `Input`'s commands have consecutive sequences, newest first, an older one's never sent
(proto/src/input.rs:19-21; the encoder's `NotCanonical`, :155-157); (2) each older command's tick is 0 to 255 ticks at
or below its newer neighbour's (`OutOfRange`, :158-159); the view offset fits 16 bits (:92-95). At P0 (U4F-Q11 (ii)
(a)) each acknowledgement returns within a step (U4r's P0 runs: no step without a command,
results/u4r/051995db9cea/P0/runs.txt:22), so an honest `Input` mostly holds the new command alone; at a lossy profile
older copies ride with it. **Per kind, at the suites' profile** (replacing rev 3's "the forger forges the command it
stores", SA-3, for every kind but 7 and the clamp):

| Kind | The forged datagram | What the forger's endpoint stores | Encodes because | Its twin (2.12) |
|---|---|---|---|---|
| 1 | the honest datagram, its newest command's tick set to the forger's newest tick + 2 + the lead bound + k, k from 1 to 100 (rev 3.2, SA31-4 (a) = TE31-11: above the highest tick the room can be at on its arrival at P0, where nothing arrives in the instant it was sent, l1/src/lib.rs:69-70) | the honest command, so every later copy is honest (a repeat of a refused sequence: 2.4 item 2) | the forged command is the newest; any older copy lies at most 102 + the bound + its own lead below it (under 255) | the honest datagram without that command (U4F-Q13 (b)) |
| 2 | its newest command's tick set equal to the previous command's (a tick step of 0); older ticks are f10's unit cases | the honest command | step 0 is allowed | as kind 1 |
| 4 | its newest command's move set long ([127, 127], [-128, -128] in turn) | the honest command | any i8 pair encodes | as kind 1 |
| 5 | its newest command's fire press counter moved by more edges than the ticks it covers | the honest command | counters are 8-bit fields | as kind 1 |
| 6 | its newest command's aim x or y set outside ±12,800 cm, inside the grid's ±32,767 | the honest command | inside `POSITION_XY` | as kind 1 |
| 3 | the honest datagram with its copy of the last command the room accepted (its sequence one below the newest's, its tick unchanged) given changed move or aim, that copy changed in place when the datagram already carries it, added when the datagram holds the new command alone (rev 3.2, SA31-4 (b) = TE31-11: a second copy of one sequence is `NotCanonical`, proto/src/input.rs:155-157) | nothing of the copy | consecutive, tick step as the honest pair's | the same datagram with the copy's honest content (a redundant copy, counted `commands_repeat`): nothing skipped |
| 7 | its newest command with a fire or reload press edge where the sim refuses it | **the forged command itself** (the room accepts it, so every copy must be equal: SA-3's rule) | as an honest command | the forged command with only the refused press removed (carry 46) |
| 10 | under A, after holding 8 steps (no command), 16 new commands in two datagrams in one tick, stamped from its last stamp + 1 on, all inside [t - W, t + the bound] (U4F-Q18; at P0's lead of 2, about t - 5 to t + 10); under B, more commands than the buffer has room for, inside the bound | all of them (honest contents) | 8 consecutive commands per datagram, step 1; each command's view tick set to at most its own tick, as the endpoint clamps it (client/src/lib.rs:436-441; rev 3.2, SA31-4 (c): a stamp below the newest tick with `view_tick` = newest, bots/src/lib.rs:22-24, has a negative view offset, `OutOfRange`, proto/src/input.rs:92-95) | 2.12's row |
| 8, 9 | the ack replaced by one never sent (8); trailing bytes or a set reserved bit (9) | the honest command (resent in the next `Input`, since the room took none: room.rs:783-788) | 8: as an honest `Input`; 9 is meant not to decode | the header only, its ack the previous datagram's |
| 11 | a known type on the control stream | - | a valid message | a ClientSettings repeating the current delay |
| the clamp | the page's own command with an aim beyond reach | the command itself (accepted: every copy equal) | as an honest command | the clamp reports' points |

| Cheat | Suite (marks.json select) | What it does | Count per run | Its twin |
|---|---|---|---|---|
| 11 forgers, one kind each (2.4) | `forged`, A and B | an open-loop seeded script (commands from seed, slot and tick only: plan :380-381) with every k-th command forged in its kind (k a lab setting, `--forge-every`, set in the judged runs to give 1,000 per kind and smaller in the lab tests: rev 3.2, TE31-5); kind 9 on `Input` datagrams only, trailing bytes and a set reserved or pad bit in turn (SA-12); kind 7 cooldown and reload edges (SA-15); 37 honest open-loop clients | 1,000 forged items per kind, in 2.4's unit (marks.json:4934-4945; plan :442) | 2.12 |
| the catch-up kind (kind 10) | `forged` | under option A, 16 new commands in two datagrams in one tick, stamped for ticks the forger held back (plan :385-388), inside option A's past window and the lead bound (the table above; U4F-Q18), so none is `late` or kind 1; under option B, more commands than the buffer has room for, all inside the lead bound so none is kind 1 (SA-11; U4F-Q8); inside the bucket in both | as kind 10's 1,000 | 2.12, by U4F-Q13 |
| the clamp cheat | `clamp`, A and B | open-loop, aim points beyond the 6,000 cm reach (sim/src/scene.rs:66) yet inside the map (2.4 kind 6): U4r's stand-in's 32,000 cm (bots/src/lib.rs:74-76) lies outside the map and would be refused (NA-8). Reruns of a clamp-cheat command under option B (input.rs:135-141) are reported per run, 0 expected at U4F-Q11 (ii)'s recommended P0; a rerun re-clamps from a moved origin (world.rs:1283-1312), so a lossy profile needs a per-tick twin rule first (SA-10; f18) | 1,000 points beyond reach per weapon kind the sim has (hitscan: marks.json:4980-4991) | the forged run's clamp reports' points (SERIES.md:324) at the same instants (plan :363-366) |
| the 1.5x clock | `fast-clock`, A and B | one client per run whose host clock runs at rate 1.0 through the warm-up and 1.5 from the measured window's start to the run's end (plan.md:626; the host clock stays continuous; the lab's doc names the rate U4f's: scenario.rs:18-19). **Its schedule** (rev 3.1, TE3-6): the episodes are the measured window's back-to-back 60 s spans of the server's simulated time, the first from the measured start, each one's records written at its end; no settling between them (a client that never idles starts each episode at most the bound ahead, so each episode's excess is at most 0 from any start); the suite's measured window is **610 s** (ten whole episodes and 10 s, so the tenth's end lies inside it), in the header as every run's is (`params.measured_ticks_count`, scenario.rs:1390) with the episode length `params.fast_clock_episode_ns` (60 s; a new SERIES.md row, 5.6 B2). A command at every one of its steps (45 a second, inside the bucket's 60 a second: SA-13), stamped by its own rule by U4F-Q11 (v), its stamps never decreasing (the codec's rule above); each episode reports its lead-bound and not-newer refusals beside its excess. Its pings come at its steps, so 1.5 times an honest client's: `ping_limited` above 0 is expected and reported for this client, never a failure (SA3-7; 2.1's 0 is the honest clients') | 10 episodes of 60 s per run (marks.json:5014-5025) | none (the check compares with the bound: SERIES.md:155-156; the honest reference by U4F-Q11 (iii)) |
| the lag switch | `lag-switch`, A and B, P1 | holds its `Input` datagrams 200 ms, then sends them in one burst (about 6 datagrams, inside the bucket's 8: SA-13). At one command per moved newest tick a hold carries about 6 new commands, under `CMD_CATCHUP` = 8, so its check cannot fail at 200 ms (TE-7, SA-7): U4F-Q15 | 100 bursts (marks.json:5050-5061) | none |
| the never-acker | `never-ack` | a brawl client whose `Input`s never acknowledge a snapshot or a full state (ack groups 0); every other client brawls; its `overflow_resyncs_count` reported (scenario.rs:1270; SA-16) | one among 48 (ADR 0001:1612) | none |

### 2.7 npt-lab: suites, settings and series (lab/src/scenario.rs; lab/src/logs.rs)

- **Settings:** `--cmd-timing A|B` into the header (scenario.rs:1370 today writes "B"); the suites `forged`, `clamp`,
  `fast-clock`, `lag-switch`, `never-ack` and `clock` (marks.json's selects; SERIES.md:416's suite param) and their
  behaviours by U4F-Q11; `--twin <run>` with role `twin` (README :22-23; scenario.rs:1411, :1488); the 1.5 host-clock
  rate per client and episode (scenario.rs:18-19); the lead bound's value in the header (U4F-Q8); rev 3.1 (TE3-6,
  TE3-7): the fast clock's episode length (`--fast-clock-episode-s`, 60 in the judged runs, `params.fast_clock_episode_ns`
  in the header) and the -47 suite's connect count (`--connects`, 1,000 in the judged runs, in the header as
  `params.connects_count`), each a new SERIES.md row (5.6 B2), so the lab tests run them short (2.10); rev 3.2 (TE31-5):
  the forgers' interval (`--forge-every`, in the header under the builder's name, a new SERIES.md row), so a short
  lab pair carries at least one forged item of its kind.
- **`header.params.lead_ticks_count`** (SERIES.md:344: the one lead every client of the run stamps with) is written
  only by a run whose lead `with_lead` fixes (U4F-Q7 (i)); the contract test asserts it absent from a controller run's
  header, and the settled lead per client goes in its own series (U4F-Q1) (TE-20).
- **The -47 suite:** 1,000 connects per run at P2 with seeded random true offsets, each connect's estimate compared at
  2 s (2.1); 48 slots give about 21 rounds of fresh endpoints. Rejoins on one slot meet carry 62 (d)'s head rule and
  the slot-keyed logs of NC-U4R-5, and **carry 75's admission bound** (NA-16: npt-lab refuses an admission once the
  room has stamped 32,768 events past the event start, scenario.rs:70, :995-1006; carry 75, carries :357-365, puts that
  at about 30 s of SS's events, while 21 rounds of at least 2.5 s take about 52 s or more): the suite's logging and
  its behaviour are U4F-Q4's; f29 asserts no admission refused, and under U4F-Q4 (b) no event stamped, at its short
  setting, so no connect count meets the bound.
- **The series U4f writes** (each name's writer as SERIES.md gives it; a name with no row gets one only by ruling, as
  U4R-Q2 (c): U4F-Q1; the contract test refuses a written name with no row, lab/src/contract.rs:6-9):

| Mark | Series (SERIES.md row) | Writer |
|---|---|---|
| -20 | `tick.s.cmd_on_time` (:61), `tick.s.input_missing` (:63, by stamp: NA-14), `tick.v.press_count` (:40), `tick.v.press_lost_count` (:42), `tick.v.press_doubled_count` (:41), `second.s.buffer_depth_count` (:101); `header.params.reorder_permille` (:245, U4r's) | npt-room's counts, npt-lab's room log (no writer of any of them today: `git grep` of crates/*/src finds none) |
| -21 | `event.forged_kind.sent_count`, `.unrejected_count` (:158-159, in 2.4's unit); `event.clamp_kind.sent_count`, `.counted_count` (:144-145); `event.clamp_report.x_um`, `.y_um`, `.z_um` (:324); `event.fast_clock.excess_cmds_count`, `.excess_move_um` (:155-156); `event.lag_burst.held_ns` (:170); `tick.v.lagswitch_excess_move_um` (:34); `tick.v.lagswitch_cmds_count` (:33); the per-session refusal counters (2.5) and the fast clock's per-episode refusals (2.6): names and rows by U4F-Q1 | npt-lab, from the room's and the pump's records |
| -22 | `tick.v.ack_starved_headroom_count` (:21, from `ClientSlot::headroom`, room.rs:282-287); `event.ack_starved.dropped` (:128); `header.params.event_queue_cap_count` (:445) with `event_queue_cap_provisional` (:346; written today: scenario.rs:882-886); `second.v.rss_bytes` (:93): its writer is npt-server, which runs no process on L1, and -22's report reads it from process `room` (marks.json:5208-5216): on L1 the judge prints "not available" for a report with no data and never makes the run INVALID for it (judge/src/eval.rs:1504-1556), so U4f writes no L1 `rss_bytes` and its -22 set report says so (NA-2) | npt-room, npt-lab |
| -47 | `event.connect_clock.offset_err_ns` (:149) | npt-lab |
| all four | `header.params.cmd_timing` (:415), `params.suite` (:416), `params.twin` (:429) | npt-lab |

- The -20 and -58 press series are the same names (SERIES.md:41-42 list both marks): U4g's -58 reads U4f's writers.
- **logs.rs's `room_tick`** (logs.rs:524, its lint allow :520-523, which C62H-BOUND's review accepted: cbr-rust, point
  (a)) gains these series: RR-B1-7 (1)'s loop rewrite rides that edit (`<<P36>>`; 9.2), and RR-CB-2's three docs ride
  scenario.rs's and logs.rs's edits (9.2).
- **C62H-BOUND's link record runs on every U4f run** (scenario.rs:1045-1060; the join's `Err` fails the run,
  scenario.rs:604-631): each slot's send must push exactly one fate per snapshot group. U4f's suites keep that true: a
  kicked slot codes no group (room.rs:1047-1049, :1061-1062), a left slot has no `ClientSlot` (room.rs:689-695), and
  the cheats change only client-to-server datagrams. A suite that sends the room's datagrams any other way stops and
  goes to the advisor.

### 2.8 The event cap's measurement before -22's runs (carry 57; EVQ-LOAD; U4E-Q6, wave1 :473-479)

1. The room's cap is data, provisional at 1,024 (room/src/events.rs:40; room.rs:460-461), recorded as provisional
   in every header (SERIES.md:346; scenario.rs:882-886). U4f measures and records plan §4.7's value before any -22
   run (carry 57, `<<P3>>`).
2. **EVQ-LOAD**, carried word for word: `<<P32>>` (also N/u4r-report.md:1049-1053).
3. **The measurement** (the builder's short runs, SS, 48 clients, the brawl, under the runner, `-x`): P2, P3 and B1
   (B1 with at least 2 bursts), 3 seeds each; per client the relevant gameplay-event rate and the queue's high-water
   mark per burst and per 30 s bin (`ClientSlot::queue`, room.rs:277-280; carry 58's figures, `<<P11>>`). No log
   writes the queue's length today (no npt-lab writer of `second.s.event_queue_count`, SERIES.md:385), so these
   figures are new: the builder writes them into its short runs' records (the series by U4F-Q1), never by a grep of
   a kept log.
4. **The value, by U4F-Q12:** plan §4.7's formula (:335) lands below the observed need for any margin up to about
   0.73 s (NA-3: 560 a second x (1 s + RTT + margin) reaches about 1,008 at a 0.7 s margin at P3, while 1,024 already
   overflowed at P3 without a burst, `<<P32>>`'s 27 forced resyncs; the margin is free, so the formula reaches the need
   only with a larger one), and carry 50 (`<<P9>>`) bounds the cap from above for -22 (NA-12: a never-acker's events
   expire once older than 255 ticks, room/src/events.rs:19-20, so a cap above its rate x 255 / 30 s, about 3,900 at
   460 a second, is never reached and -22's headroom check fails a correct build).
5. The chosen value, its derivation and the high-water figures go into plan §4.7 (the main session's commit: plan risk
   11, :953-954) and into the room's default with `event_queue_cap_provisional` false (`Room::with_event_cap`,
   room.rs:481-484); the judged -20, -21 and -22 runs run at it. The cap is a design constant the -22 check reads,
   not a bar's text (5.6).

### 2.9 The client carries 80 (b)-(e), and what C62H-BOUND changes for them (npt-client; `<<P1>>`, carries :520-536)

**Answered since rev 2** (C3): C62H-KEEP (a) (`<<P19>>`; carries :322-323), C62H-BOUND (a) with its wording to him
first (`<<P20>>`) and C62H-BOUND-YES (a) (history :2163-2175, its last bullet `<<P21>>`); C62H-BOUND is built (eb2bebf,
b06991e, 15102d1, 561bf42) and its P2 and P3 sets judged (daec788, 8ca774a). So where carry 80 (b) and (d) touch how the
client files lost and too-old ticks:
- **Two judged checks now read that filing:** -17's `snap_lost_unexplained_count` and
  `snap_whole_despite_link_loss_count` on its L1 P2, P3 and B1 parts (marks.json:3983-4005; SERIES.md:291-292, :322,
  :378), and C62H-KEEP's L1 P0 check (marks.json:3972-3982). A client that files a measured tick lost with no room or
  link reason, or files whole a tick the link lost a datagram of, fails them.
- **The guard in U4f's build:** the lab's short P2 test `c62hb_p2_short_every_measured_lost_tick_is_explained`
  (lab/src/scenario.rs:2904, with the join tests :2795-2843) runs in every check.mjs test step. If it fails on U4f's
  build, the work stops and goes to Mustafa: the stop rules of the C62H-BOUND post, which his yes adopted (`<<P21>>`;
  the rules themselves at c62hb-paste :120-124, quoted in c62hb-brief :603-607, name the lab's short P2 test), with no
  rewording and no retry.
- **On honest L1 runs** a refused datagram and a stale drop do not occur (the P2 and P3 sets: sentinels 0, stale 0,
  advisor-c62h-bound.md:20), so (b)'s bound and (d)'s test change no honest run's filing; f24 and f26 assert it by
  each case's `lost` and `hashes` output, beside the short P2 test.
- **U4f designs no lost-tick bound and changes no -17 check, row or figure:** C62H-BOUND's texts are frozen and any
  change after his yes comes back to him (history :2173). U4F-Q16's bound is how far a refused header may move the
  newest tick, a different bound.
- **-17's counted P2 and P3 sets** were judged on the client before U4f's edits; whether they stand after U4f's merge
  is the advisor's call at U4f's close (9.1), on f24, f26 and the short P2 test's evidence.

| Part | Site at the head (client/src/lib.rs, unchanged since 11fd190) | Test (2.10) |
|---|---|---|
| (b) a refused far-ahead snapshot header moves the newest tick and resolves the span lost | `receive`'s refused-datagram sentinel (:508-522) into `sentinel` (:631-641) and `see` (:583-615) | f24, asserting the bound U4F-Q16 rules (TE-11) |
| (c) a failing command drops the whole `Input`, acks included | `step` (:458-470) | f25: a page command whose aim lies outside the `Input` codec's range (the builder names the field and its bound in proto's codec) is dropped and counted (`unencodable_commands`, the builder's name; accessor in 2.13), the acks kept (TE-12) |
| (d) the client's two too-old cases | `too_old`, `old_header`, `bad_header` (:528-565) | f26 (carry 80 (d)'s endpoint test as worded) |
| (e) a full state after a snapshot of another hash files nothing | `full_state` (:810-839: `got != 0` counts a duplicate, :817-820) and `file` (:645-658) | f27 |

### 2.10 Tests and negative cases (counts as deltas on `<base>`'s: plan §5 :507-512)

Each row adds its Δ tests in both builds unless marked "bites only"; "neg" marks a negative case. Minimum deltas
(rev 3.2): **measured +109; bites build +124** (the measured 109, f30, and 2.11's 14 bites-only unit proofs); **64
negative cases** (rev 3.2: f13 (iv); f32 (iii) is an honest case, +1 test). Conditional, in both builds: +1 by U4F-Q18 (ii) (d) (f39, a negative
case; rev 3.2, SA31-1); +4 by U4F-Q6 (b) (the Pong field's codec round trip, the Pong's 32 B
limit, the catalogue row's test, and the room ignoring or refusing the field in a client's `Pong`, a negative case:
SA3-6 (4)); +1 by U4F-Q9 (a) (the kept window never holds more than K under a flood of new sequences); +1 by U4F-Q11
(iii) (c) (the fast clock's fixed-input episode: the arithmetic reference equals an honest client's movement); +1 by
U4F-Q13 (a) (a refused command is resent, refused again and counted once per (slot, seq)); +1 by U4F-Q18 (a) (f38); +5
by T-6 if contract.rs is edited (`<<P34>>`). The builder states the exact counts and the `--list` lines that name them.

**Generators and traces (rev 3.1, TE3-11):** every property test and seeded trace uses a seeded generator written in
the test (the project's splitmix64 step, as bots/src/lib.rs:114-121), no dependency (a manifest change is a stop:
section 1 item 5), and names its seeds and case count in its doc: at least 1,000 cases each; f8's property test 10,000
random tick triples, each in all six arrival orders.

**The lab tests' scale (rev 3.1, TE3-7):** check.mjs's test step runs every test in the debug profile, in both builds,
twice per end check (check.mjs:2052-2057), so each lab test of this table runs at the smallest scale that proves its
claim, in P2_SHORT's form (lab/src/scenario.rs:2842-2843: 16 clients, a 6 s warm-up, 4 s measured) unless its row names
another: f16 and f18 at 16 clients (one forger or clamp client, 15 honest), a 2 s warm-up and 4 s measured per pair; f19
at a 2 s episode (the episode length a lab setting, 2.7); f20 at P1 with a hold every 2 s; f28 the contract checker's
run (lab/src/contract.rs) on one short scenario per suite at those settings, never at 5.3's lengths (those are
c-u4f-r2.mjs's, after the merge); f29 at 16 slots and 32 connects (two rounds; the count a lab setting, 2.7). The builder
runs each new test once alone through the runner and writes its wall time from the runner's summary line (rev 3.2,
TE31-10: every new test, the client and bots tests f36 and the property tests f8 and f32 included, not the lab's only);
a new test that takes more than twice `c62hb_p2_short_every_measured_lost_tick_is_explained`'s time, read the same way
in the same build, stops for the advisor; the test step's time goes in each end check's line (check.mjs prints it on
the step's line, check.mjs:2678-2681), and a test step past 1.5 times `<base>`'s, read the same way (from D1's counted
run, or from E6's run log if it holds that line, else from one run of `<base>`'s test step at launch), stops for the
advisor.

| Id | Test | Crate | Neg | Δ |
|---|---|---|---|---|
| f1 | ClockSync on a fixed P2 arrival trace at one phase per connect, as L1 gives it (SC-1): error at 2 s within the stated bound; **by U4F-Q6 (b) (rev 3.2, D4):** at least 1,000 seeded connects with D4's parameters (section 11), the p99 of abs(error) at 2 s at most 4.0 ms: the bar less 1 ms for what D4 did not model (the filter's drift, halving and reset rules, L1's reorder step, the read instant), above D4's mean (2.35 ms) and below its lowest-RTT pick (4.856 ms), so a lowest-RTT-only filter fails it | client | | 1 |
| f2 | the three filter rules of ADR 0001:240-242, one test each | client | | 3 |
| f3 | outstanding pings and samples capped; an unknown pong id dropped and counted | client | neg | 2 |
| f4 | the ping rate never exceeds the room's limit (a faster rate is limited and counted by the pump) | client | neg | 1 |
| f5 | the controller's settled lead per profile (P0-P4, B1) on a seeded trace: at least 1, at most the lead bound less 2.2's margin (and at most 9 under option B), reported beside U4R-Q14's lead for that profile, and at most 1 % of commands arriving after their stamped tick (carry 74; TE-15; rev 3.1: TE3-1 replaced "at most U4R-Q14's lead"; P3's own bar is f36's) | client | | 1 |
| f6 | the lab builds both options' endpoints with one controller setting, and npt-client reads no `cmd_timing` (TE-14) | lab | | 1 |
| f7 | option A runs several commands of one slot in a tick; the 9th is refused on arrival and counted (kind 10), and the room's list for the coming tick never holds more than 8 under a flood of 3 datagrams of 8 (SA3-8) | room | neg | 1 |
| f8 | carry 80 (a): one slot's commands 2^31 ticks apart; SA-1's triple on a fresh slot under option A; a property test over random u32 ticks: no panic and one order for every arrival order | room | neg | 3 |
| f9 | carry 80 (a): carry 21's room test (room/tests/room.rs:235) fails with the sort removed (a plant, restored; the test itself changes, so +0) | room | | 0 |
| f10 | kinds 1-6, 8 and 10 each refused and counted under its own kind, under both options; kind 4's case includes the moves [-128, -128] and [-128, 0], both refused, with no panic in a debug or test build (SA3-3) | room | neg | 8 |
| f11 | one kind per refusal: a command breaking two checks is counted once, under the first; an honest late command is counted `late`, never kind 2 | room | neg | 2 |
| f12 | (i) an honest redundant copy is refused under no kind; (ii) a backward sequence with other content than the accepted command kept under it, inside the window, is refused under kind 3, and a later copy of a refused sequence with other content is a repeat; (iii) an honest P3-like trace with redundancy 8 and reordered datagrams gives 0 refusals of any kind; (iv) a sequence older than the window is dropped and counted `seq_outside_window`, never as forged; (v) a sequence inside the window with no command kept under it (an overtaken datagram carrying the one sequence the overtaking one lacked) is counted `seq_outside_window`, never kind 3 (TE3-13 = SA3-4) | room | neg | 5 |
| f13 | (i) 8 far-ahead commands, then an honest command: the honest one runs (NA-7); (ii) under option B a command stamped equal to a buffered one is kind 2, never `late` (NA-13); (iii) the join anchor, under both options: a fresh slot whose first stamp is the join's tick - 1,000, and one at the join's tick - 2^31 + 1, each refused as kind 2, nothing run (SA3-1), with a case at room tick 1 (rev 3.2, SA31-3: no underflow, no panic); (iv) rev 3.2 (TE31-4 = SA31-2), under both options: a first stamp equal to the join's tick refused as kind 2, and a first stamp of the join's tick + 1 accepted and run, so an honest client stamped by 2.2's first-stamp rule counts 0 as kind 2 | room | neg | 4 |
| f14 | every refusal field of `Counters` and of `ClientCounters` written per session (one test per struct, each field asserted), and a kicked session's written at its kick (SA-5) | lab | neg | 3 |
| f15 | each forger produces exactly its kind (its datagram decoded: only that check broken), 11 cases, as 2.6's per-kind table builds it; every forged `Input` encodes under both options (no `EncodeError`; the forger's `unencodable_commands` 0: TE3-4, SA3-2); kind 9's on datagrams only, both forms; kind 7's cooldown and reload edges | bots | neg | 11 |
| f16 | per kind (11) and the clamp, under A and under B, at P0 (2.10's scale): a short forged run and its twin give equal fate logs on every flow, **compared entry by entry on link, direction, flow, outcome, overtaken flag, index in its flow, send time and arrival, the size and resends reported but left out** (the canonical form carries each datagram's size, l1/src/lib.rs:353-362, :1146, and the twins differ in size by design, plan :377-379, while the fates are drawn per flow index, :1125-1138; `fate_log_sha256` is never compared for -21's twins, only for same-run identity: D11, r8) (TE3-5); equal snapshot group counts; **the same set of ticks hashed** (a tick hashed in one run only counts as a mismatch: judge/src/figures.rs:191-214); equal state hashes on every tick (plan :389-390; SA-3); every forged and twin `Input` encodes, and the twin left out no command (2.12's count 0); **each pair carries its kind** (rev 3.2, TE31-5): its forger's `sent_count` for its kind at least 1 inside the measured window (kind 10: one whole episode; the clamp: one beyond-reach point), at a `--forge-every` set for the short scale (2.7), and the twin's matching datagram present | lab | | 24 |
| f17 | carry 46 (`<<P1>>`): a forged press stream and its twin give equal state hashes on every tick, at the room's level | room | | 1 |
| f18 | the clamp cheat's points lie beyond reach and inside the map; the twin's points from the clamp reports give equal hashes on a short run; under option B a clamp command rerun after a move gives two reports for one sequence (SA-10) | bots, lab | | 3 |
| f19 | the 1.5x clock: excess commands at most 0 in the clean build, with lead-bound and not-newer refusals each above 0 per episode (SA-6), at a 2 s episode | lab | neg | 1 |
| f20 | the lag switch under P1: at most `CMD_CATCHUP` commands run per tick; movement within the catch-up cap's | lab | neg | 1 |
| f21 | the never-acker: headroom reaches 0, never below; one resync, then `Kick(overflow)` within `FULL_STATE_ACK_TICKS` (its later datagrams reaching no room call is U4r's r9, scenario.rs:2267, kept: TE-17) | lab | neg | 1 |
| f22 | the catch-up kind under option A and under option B, inside the bucket: the catch-up check refuses the excess, never the bucket or the lead bound | lab | neg | 2 |
| f23 | a known type on the control stream: refused and counted, the session kept (catalogue :87, :99) | lab | neg | 1 |
| f24 | carry 80 (b), asserting U4F-Q16's bound and the lost count; an honest stream's `lost` output unchanged (2.9) | client | neg | 1 |
| f25 | carry 80 (c): the unencodable command dropped and counted, the acks kept | client | neg | 1 |
| f26 | carry 80 (d)'s endpoint test, each half failing under its plant; no `lost` run from either too-old case (2.9) | client | neg | 1 |
| f27 | carry 80 (e), failing under its plant | client | neg | 1 |
| f28 | every series of 2.7 written, with its SERIES.md row, on a short run of each suite at 2.10's scale (the contract checker's own check, run on those short scenarios: plan §4.12 :412-415; contract.rs:6-9); `lead_ticks_count` absent under the controller (TE-20); each cheat's `input_limited` written | lab | | 1 |
| f29 | the -47 suite at 16 slots and 32 connects (two rounds): one event each at 2 s, true offsets seeded, **no admission refused by carry 75's bound** and, under U4F-Q4 (b), no event stamped, so no connect count meets it (NA-16); under U4F-Q4 (a) or (c) (rev 3.2, TE31-5), its events per round asserted at most 1,560 (32,768 over the judged runs' 21 rounds), and the 1,000-connect run made once among the builder's D7 runs | lab | | 1 |
| f30 | §4.11's bites per instance for U4f's new ids: a bitten and a clean endpoint and room on parallel threads (plan :399-401); bites only, since `with_bite` exists only in bites builds (plan :193-194) (TE-9) | client, room | | 1 (bites only) |
| f31 | carry 39: each rate-derived value U4f adds, pinned at 30 and 60 Hz: ClockSync's tick period in ns; the lead bound's ticks (U4F-Q8 (b): `ticks_for_ms_at` with the rate passed, sim/src/ticks.rs:25; under (a), npt-sim's own pin by carry 18's route, `<<P7>>`). The lab keeps the 60 s episode and the 200 ms hold in ns on simulated time: no tick count (TE-10) | client, room | | 2 |
| f32 | honest stamps rise strictly and stay within the lead bound: a property test over seeded P0-P4 and B1 sample traces, and a case through a lead step-down and a ClockSync reset (TE-15, SA-8); (iii) rev 3.2 (TE31-4 = SA31-2), under both U4F-Q7 forms: a join at room tick > 0 with a `Pong` arriving before the first snapshot: the endpoint stamps no command and its commands-due count is 0 until it holds a snapshot and a target above the join's tick, then its first stamp is the controller's target, above the join's tick | client | | 3 |
| f33 | npt-lab's attribution, by 2.4 item 6's per-kind rules: per kind (11), a forged item counted rejected only under its own kind, the kind-3 case with the honest original run and its forged copy counted rejected, the kind-7 case with the command run, its press refused and counted rejected (TE3-2); a forged command refused under another kind counts unrejected; a planted copy that ran counts unrejected 1 (TE-23, SA-4) | lab | neg | 13 |
| f34 | NA-14: under option A, a P3-like trace whose commands arrive bunched (two in one tick, none the next) writes `tick.s.input_missing` 0 for every tick whose stamped command arrived before it ran, and 1 for a tick whose stamped command never arrived | room | | 1 |
| f35 | NA-15: a session that sends `Leave`, one whose link closes, and one replaced: each one's `Counters` and `ClientCounters` are written once, with the values before the room freed the slot or the `Decoder` was dropped | lab | neg | 1 |
| f36 | rev 3.1, TE3-1: (i) -20's two shares on a seeded P3 trace of at least 100,000 ticks under both options (P3's one-way 75 ms ± 15 ms and 5 % loss each way, l1/src/lib.rs:190-192, :214-221, and, rev 3.2 (TE31-10), the 1 % adjacent-pair swaps every profile carries, `ReorderStep::PART_13`, :146, :173, modelled in the test; snapshots each tick; the endpoint's pings, controller and redundancy as built, in the form U4F-Q7 rules): at least 99.9 % of commands arrive before their stamped tick runs and at most 0.1 % of player-ticks have no stamped command arrived before they run (marks.json:4710-4737's two checks), and `stamps_skipped` at most 0.1 % of ticks (rev 3.2, TE31-1); (ii) npt-bots: a controller run's behaviour gives a command for every stamp the endpoint names (no tick between two given), and a `with_lead` run keeps bots/src/lib.rs:14-21's rule | client, bots | | 2 |
| f37 | rev 3.1, TE3-6: the fast clock's episode schedule (2.6): rev 3.2 (TE31-10), the lab's own episode function, called with the judged settings (a 10 s warm-up, 610 s measured), returns 10 episodes of 60 s each wholly inside the measured window, and with 5.3's (10 s, 70 s) returns 1 | lab | | 1 |
| f38 | rev 3.1, SA3-1, **only by U4F-Q18 (a)**: under option A a client that idles N ticks (N = 60 s of ticks, and N = 100,000 from a stamp of the join's tick + 1), then sends 8 commands a tick stamped one apart from its last stamp + 1: over any span, at most W + the lead bound more commands run than ticks pass, and every command the window drops is counted `late`; with a case at room tick 1 (rev 3.2, SA31-3) | room | | 1 (conditional) |
| f39 | rev 3.2, SA31-1, **only by U4F-Q18 (ii) (d)**, under both options: a client stamps up to the tick being run + the lead bound, rejoins its slot (and, in turn, a replaced session and a kicked slot that rejoins), then stamps from its new join's tick + 1: no command at or before the slot's kept stamp runs, each is counted `rejoin_overlap`, and an honest rejoin's forged counts are 0 | room | neg | 1 (conditional) |

Kept, not new: the Send asserts (plan :402-403) cover `ClientEndpoint` with ClockSync (client/tests/endpoint.rs:92-95)
and `Room<World>` with option A (room/tests/room.rs:228); the runner's twin test (plan :406-407); -58's lt-cmd-seq and lt-press unit proofs (2.3);
U4r's r8 and r9 (scenario.rs:2212, :2267); C62H-BOUND's lab tests (scenario.rs:2795-2990) and l1 test
(l1/src/tests.rs:398).

### 2.11 The bites (17; plan §4.14 :453-476; marks.json)

Each planted where its code lives, shown by a unit test here, and run judged in its suite and option (plan :453-455,
:811-812). Bites-only, chosen per instance (plan :399-401); one home per id (check.mjs's `biteHomeProblems`, :936).

| Bite | Site (crate) | Plant | Unit proof | Judged run (section 5) |
|---|---|---|---|---|
| PM-0001-20/redundancy-1 | the endpoint's redundancy (client/src/lib.rs:442-443) | repeats truncated to 1 | an `Input` holds one command (new) | -20 P3, option by U4F-Q10 (recommended B) |
| PM-0001-21/no-lead-bound | room, kind 1 | check removed | a far-ahead command buffered or run (new) | forged, option by U4F-Q10 |
| PM-0001-21/no-newer-tick | room, kind 2 | check removed | an equal-tick command runs (new) | forged, U4F-Q10 |
| PM-0001-21/no-seq-forward | room, kind 3 | check removed | a backward sequence with other content runs (new) | forged, U4F-Q10 |
| PM-0001-21/no-move-length | room, kind 4 | check removed | a long move runs (new) | forged, U4F-Q10 |
| PM-0001-21/no-press-edge | room, kind 5 | check removed | two edges in one covered tick run (new) | forged, U4F-Q10 |
| PM-0001-21/no-aim-bounds | room, kind 6 | check removed | an off-map aim runs (new) | forged, U4F-Q10 |
| PM-0001-21/no-cooldown | npt-sim's shot phase (world.rs:1411-1419; NA-9; U4F-Q1) | check removed | a cooldown-breaking edge fires (new) | forged, U4F-Q10 |
| PM-0001-21/no-ack-sent | room, kind 8 (room.rs:783-788) | check removed | a never-sent ack's commands run (new) | forged, U4F-Q10 |
| PM-0001-21/no-trailing | npt-proto (quant.rs:40) | U4a's | U4a's and U4e's (proto/tests/bites.rs:211; server/tests/pump.rs:477) | forged, U4F-Q10 |
| PM-0001-21/no-catchup-cap | room, kind 10 | the per-tick cap removed | 16 commands run in one tick (plan :463) (new) | forged, option A (plan :463); in the lag-switch suite too only by U4F-Q15 (b) |
| PM-0001-21/no-channel | npt-proto (quant.rs:41) | U4a's | U4a's and U4e's (proto/tests/bites.rs:238; server/tests/pump.rs:483) | forged, U4F-Q10 |
| PM-0001-21/not-older-than | room, kind 2 | `>=` in place of `>` | an equal-tick command runs (plan :462) (new) | fast-clock, option A (plan :462; marks.json:5133-5135, :5144) |
| PM-0001-21/refuse-not-clamp | npt-sim's clamp (world.rs:1283-1312; NA-9; U4F-Q1) | a refusal in place of the clamp | a beyond-reach point refused and counted (new) | clamp, U4F-Q10 |
| PM-0001-22/unbounded | the room's event queue (events.rs:22-26; room lib.rs:44) | U4e's | U4e's (room/tests/room.rs:1303, :1363, :1444) | never-ack over a full S30 (marks.json:5218-5223; e-ruling :340-341) |
| PM-0001-47/newest-sample | ClockSync (2.1) | the newest raw sample | the estimate equals the newest sample (new) | clock, P2 |
| PM-0001-47/tick-quantised | the endpoint's arrival stamps (2.1) | the step's time in place of arrival | a sample's time moves to its step (new) | clock, P2 |

New unit proofs: 14 (redundancy-1, 11 of -21's 13, both of -47's); U4a's and U4e's three stand. A unit proof shows the
state changes, not that a suite reaches the site, so before each judged set 5.3 runs one bite per suite short and shows
it failing its check by name (TE-6). A bite run needs no twin: each fails another check of its part (README §5
:317-326), and keeps its part's validity rules, `pre:PM-0001-2` included (README :323-324), so each -20 and -21 bite run
must also pass PM-0001-2 (U4F-Q10's reason for -20's option). U4f adds no lab bite: `LAB_BITES` keeps its 3 ids in
their order (scenario.rs:215-219; precheck.rs reads index 1).

### 2.12 -21's twins (plan §4.10 :360-390; Q16 (a) :1042-1045; advisor condition 1 :78-89 as amended at :477-479; carry 46)

| Kind | The twin's datagram in the forged one's slot | Source |
|---|---|---|
| refused whole by the room (kinds 1, 2, 4, 5 and 6) | by U4F-Q13: (b, recommended) the twin's endpoint never makes the refused command: its sequence index and its stamp are used and nothing is sent for them (a lab-side endpoint setting, 2.13), so the two runs' commands that run keep the same sequence numbers, which the state hash reads (the Shot and Hit events and each projectile carry the command's sequence: sim/src/world.rs:1979, :1995-2009), and the twin's datagram in that slot is the honest one without that command (its unacknowledged commands and acknowledgement only); each later `Input` leaves out any command older than a skipped index, so its commands stay consecutive (2.6's codec rule), counted per client (`twin_left_out`, the builder's name, report-only: 5.6 B2), 0 expected at P0, where each acknowledgement returns within a step; or (a) the same datagram with the forged command, and every later copy of it, left out | condition 1 (only the refused part removed); rev 3.1: TE3-2, TE3-4, SA3-2 |
| a resent copy with changed content (kind 3) | the same datagram with the copy's honest content: a redundant copy, counted `commands_repeat`, nothing skipped (its size may differ by the fields its mask names: f16 leaves sizes out) | TE3-2 |
| the catch-up kind (kind 10), in one row (TE-18) | the twin's first datagram of that tick carries the commands the forged run's room accepted, each view tick at most its own tick as in the forged datagram (rev 3.2, SA31-4 (c)); every further datagram of that tick carries repeats only; the refused commands are never made (U4F-Q13 (b)), so the ticks they named run nothing (A) or the last command again (B) in both runs | plan :385-388; SA-3 |
| a datagram voided whole (kind 9) or its commands refused (kind 8) | the header only, no commands, its ack equal to the previous datagram's | plan :377-379 |
| a wrong-channel message on the control stream (kind 11) | a valid no-effect message (a ClientSettings repeating the current delay) | plan :382-384 |
| a sim-checked edge (kind 7) | the forged command with only the refused press removed: that button's counter one lower from the refused command on, every other field equal, exactly the presses `reports()` lists (carry 46) | plan :367-371; wave1 :197-235 |
| the clamp | the forged run's clamp reports' points at the same instants (one per command: 2.6's rerun rule) | plan :363-366; SERIES.md:324 |

Every client of the forged and clamp suites runs an open-loop seeded script (plan :380-381): each command's content a
function of seed, slot and the client's step since its first snapshot, never of the count of commands given (as U4r's
scripts are: bots/src/lib.rs:278), so a twin's skipped command leaves every later command's content equal. The twin
rows hold at P0 (U4F-Q11 (ii) (a)); at a lossy profile a command a twin's `Input` leaves out can be lost where the
forged run's copy got through, so a suite moved off P0 brings its twin rule to the advisor first. f16 proves each row,
per kind and under both options, before the judged sets; f17 proves carry 46. The judge's twin figure needs the twin
in the same judge call and the same profile, seed and bite (figures.rs:191-214; 5.4).

### 2.13 The seam rule and the public items, by crate (A19: plan §7.2 :930-931; §5 :512)

- **The seam:** U4f changes no seam type (`Command`, `Rewind`, `TickInput`, `SimCore`'s methods: plan §4.2). The two
  npt-sim bites (NA-9) add ids to `World::BITES` (sim/src/world.rs:937-938) and their sites, bites builds only, if
  U4F-Q1 opens crates/sim for them; any other sim change is a seam change for the advisor (carry 26's route).
- **Proposed public items** (engine-architect checks them in review; each named in the report against §4.1 and plan.md
  §4.7; anything else named as beyond them):

| Crate | Added or changed | Why |
|---|---|---|
| npt-client | `ClockSync` (plan.md:610; not in plan §4.1's row :193); a `ClientEndpoint` accessor for the clock estimate, which the lab reads for -47; the unencodable-command count's accessor (2.9 (c)); the controller's setting if U4F-Q7 keeps one; the commands-due accessor and the skipped-stamps count (2.2, rev 3.1; rev 3.2: the accessor returns 0 until the first stamp is due, and the endpoint keeps its `JoinAccepted` tick, no new public item); a skip for the twin's endpoint (its sequence index and stamp used, nothing sent) and its left-out count if U4F-Q13 (b) (2.12); `BITES` to 6 ids | 2.1, 2.2, 2.9, 2.11, 2.12 |
| npt-room | option A behind the `cmd_timing` setting (a `Room` constructor or setting); the per-tick refusal records (slot, seq, kind) and the kept window, by U4F-Q9; the new counters in `ClientCounters` (room.rs:63-136), `seq_outside_window` among them; the lead bound's setting if U4F-Q8 makes it data; the join anchor and option A's past window (U4F-Q18; no new public item unless W becomes a setting); the per-slot kept stamp and `rejoin_overlap` by U4F-Q18 (ii) (d) (rev 3.2); `BITES` widened by 9 ids | 2.3, 2.4 |
| npt-bots | the cheats' and the never-acker's constructors on `Behaviour` (plan §4.1 :197 names them); a no-fire clock-suite script if U4F-Q4 rules it; the controller runs' command rule (2.2), no new public item | 2.6, 2.7 |
| npt-sim | none, or two ids in `World::BITES` (bites builds only) by U4F-Q1; a lead-bound tick constant only by U4F-Q8 (a), through carry 18's route | 2.11, 2.4 |
| npt-server | none (2.5), unless U4F-Q14 or U4F-Q6 opens it | |
| npt-lab | binary settings only (2.7, the episode length and connect count among them); no new library item; RR-CB-2's three doc lines (9.2) | |

---

## 3. The carries

**The rule** (word for word): `<<P2>>`. U4f's report lists each carry below as met (file:line, test), not met (why) or
not applicable (why), by number (advisor :334-335; U4r's wording, N/u4r-brief.md:695-696).

| Entry | Placeholder (filled word for word) | carries lines at the head | Where U4f meets it |
|---|---|---|---|
| §U4f: 46, 80 (a)-(e), 86 | `<<P1>>` (the section whole; carry 86 has no quotation marks, NA-1, so the section is copied whole) | :507-540 (46 :508-511; 80 :512-536, (a) :515-520, (b) :520-522, (c) :522-524, (d) :524-531, (e) :531-536; 86 :537-539) | 46: 2.12, f17. 80 (a): 2.3 by U4F-Q17, f8, f9; (b): 2.9 by U4F-Q16, f24; (c)-(e): 2.9, f25-f27. 86: 5.3 |
| 57 | `<<P3>>` | :223-228 | 2.8 |
| 74 | `<<P4>>` | :345-356 | 2.2; 5.3's late share, both readings (TE-4) |
| 21 | `<<P5>>` | :148-149 | 2.3 (option A's several commands per slot); f9 |
| 27 | `<<P6>>` | :166-168 | 2.3 (`CMD_CATCHUP` and its const assert, input.rs:59-61, :155, kept; option A's per-tick cap reads it) |
| 18 | `<<P7>>` | :129-132 | only by U4F-Q8 (a): the lead bound's tick constant by carry 18's route (advisor :463-465) |
| 39 | `<<P8>>` | :169-179 | every rate-derived value U4f adds takes the rate as an argument and is pinned at 30 and 60 Hz (f31); the lab's 60 s and 200 ms stay in ns; no local tick constant; check.mjs's tick-name rule `TICK_NAMED` (:760) refuses a `*_TICKS` or `*CATCHUP*` const not so built |
| 50 | `<<P9>>` | :180-183 | 2.8 item 4 (the cap's upper bound, NA-12); no code change |
| 61 | `<<P10>>` | :229-247 | 2.9 ((b) is the case carry 80 (e) completes) |
| 58 | `<<P11>>` | :269-273 | 2.8 item 3 (high-water marks); no -17 or -18 run is U4f's; U4f's runs report their overflow resyncs (`overflow_resyncs_count`, scenario.rs:1270) |
| 76 | `<<P12>>` | :366-373 | 2.5 (where each session ends), 2.6's never-acker (the Kick path), f21 and U4r's r9 (kept); no forged kind closes a session (2.4 kinds 9 and 11) |
| 77 | `<<P13>>` | :374-383 | (a) wraps per counter name in U4f's runs; (c) `stale()` sums reported on 5.3's short runs |

Not U4f's, each by its own condition: 63 (carries :324-329) only if U4f opens crates/log; 64 (:450-475) only if it
opens crates/judge (it adds round 2's judge follow-ups first); 67 (:478-495) only if U4f moves ES's synthetic recording
or the World's outcome on it. U4f opens none of these by section 4's proposed lane; a ruling that opens one adds its
placeholder here. Carry 62 (h) (:305-323) is not U4f's: C62H-KEEP is answered (a) (`<<P19>>`; carries :322-323).
Carry 75 (:357-365) is U4r's, but its admission bound binds U4f's -47 suite (2.7; NA-16). The night plan's list of
carries (46, 80, 86, 57, 74, 21, 27, 18, 39, 50, 61, 58, 76, 77) is met whole; none is added.

---

## 4. Build rules

**The shared target** (Mustafa's rule: history :411-420; plan §7.1 :909-922; the advisor's W1-TARGET), binding on
every cargo, rustc and rustdoc call in your worktree, check.mjs's and the `wasm` step's included, as U4r's section 4
rules 1-9 give them (N/u4r-brief.md:938-992):
1. `CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/shared-target` and `CARGO_NET_OFFLINE=true` on every call
   (the runner sets both); checked once after the first build: no `target/` in your worktree.
2. **The lock:** every such call through the runner, which takes `flock` on the shared target's `.lane.lock` for that
   one call and stops with exit 5 if it cannot: shared for a plain build; exclusive (`-x`) for a counted run
   (check.mjs, `--list` and clippy confirmations), every `wasm` step or `npt-lab hash` run, each bite run, each plant
   (B6) and every run with a scratch plant in place. A plant's commands go in one call (`-x -k`), restored and its
   sha256 checked before your next call.
3. **Provenance** from the builds run before check.mjs inside the same exclusive runner call (N/u4r-brief.md:951-957's
   form): `cargo test --locked --workspace --no-run` and the same with `--all-features`, and the `wasm` step's four
   release builds; the runner's "Compiling npt-room/npt-client/npt-bots/npt-lab v… (<your worktree>/…)" lines are the
   provenance. A missing line means "not counted".
4. Touch before each build (plan :913); the runner does it.
5. **Counts are deltas** (plan :914-915, :884-885) on check.mjs's measure (`testBothBuilds`, check.mjs:2052;
   `cargoCounts`, :1943): `<base>`'s counts (section 12 item 1: at 0ad9661, measured 552 and bites 632, 0 failed, 0
   skipped; self 78 of 78; 11 steps; marks 81; refs 1382; redact 10 of 10, 306 items, 0 findings) plus the tests you
   added (2.10). A mismatch means another copy's artifact ran: touch, rebuild, never report that run; `--list` only
   names.
6. **Disk:** `df -h /home/user` before each build, in your notes; at least 4 GB free before any wasm32 or release build
   and every check.mjs run, 2 GB before every other build; the runner's floor is 4 GiB; below it, stop and report;
   never clean, pack or delete anything (plan :920; packing is the main session's alone, never while a build runs:
   .claude/CHECKPOINTS.md §11, :236-238).
7. **Cleans and cargo probes are not yours.** The main session's cleans follow W1-TARGET item 3 as amended: `<<P27>>`
   (m9 Block 2, :252-256). The two lessons: `<<P28>>` (:259-264). The probe lesson: `<<P29>>` (d2-r2 Block 6,
   :304-308).
8. After the last bite or plant, touch and rebuild before any further claim (plan :916).
9. Cargo's own lock serializes builds: expected (plan :917). The main folder's cache is not yours (plan :918-919).

**§4.11's bans and A13's** (plan :392-407; the advisor's U4d2 ruling A13, d2-ruling :568-592), binding on ClockSync, the
1.5x host clock and the lag switch, all on simulated time (plan.md:525-526, :618-629): no HashMap or HashSet in room,
client, server, lab or bots (each crate's clippy.toml holds the ban: room, client, server, lab, bots and sim each have
one at the head); no clock read in server, lab or bots (`std::time::Instant`, `std::time::SystemTime`,
`tokio::time::Instant` and their `now`: plan :393-396); the client reads no clock (ADR 0001:245-247); npt-room
float-free (plan :397-398; check.mjs's grep; room lib.rs's `deny(clippy::float_arithmetic)`, carry 25), so every room
check is in integers; bites per instance, never a static (plan :399-401).

**Your files: the proposed lane (U4F-Q1; the plan names none: plan :805-816 and Lanes :851-889 give only "they share
the room's and the client's files", :882-883):**
- crates/room/src/ (input.rs, room.rs, lib.rs's `BITES`) and crates/room/tests/;
- crates/client/src/ and crates/client/tests/;
- crates/bots/src/lib.rs (but src/bin/) and crates/bots/tests/;
- crates/lab/src/ (scenario.rs, logs.rs, contract.rs if 5.3 needs it, their tests) and crates/lab/tests/; C62H-BOUND's
  hold on lab/src/scenario.rs and logs.rs ended when its P3 results were committed (c62hb-brief :79-81; 8ca774a);
- ops/runs/c-u4f-*.mjs (new: the r2 extension, the cap measurement's short runs and the judged-set runner, on
  c-u4r-r2.mjs's and c-u4r-runs.mjs's models); c-u4r-runs.mjs and c-u4r-r2.mjs themselves are not yours;
- by U4F-Q1 only: crates/sim/src/world.rs for the two bites' sites and ids (bites builds only: NA-9);
  marks/SERIES.md, **new rows only** (U4R-Q2 (c)'s precedent: r-check :189-196; never an existing row, never
  C62H-BOUND's four frozen rows :291, :292, :322, :378); crates/proto and the catalogue if U4F-Q6 (b) rules a wire
  change; crates/sim's carry 18 places if U4F-Q8 (a); crates/server if U4F-Q14 (a) or U4F-Q6 (d) needs a pump change.
- **Not yours:** every Cargo.toml and Cargo.lock (the main session's commit); **marks.json, every existing SERIES.md
  row, marks/README.md and crates/judge** (5.6: any change there is a bar for Mustafa first); crates/log, crates/l1,
  crates/link, crates/net, crates/tcpinfo, crates/hash-wasm; check.mjs and check.test.mjs (but by U4F-Q8 (a)); docs/
  (but by U4F-Q6 (b)); results/ (the main session's runs write it: plan :928-929). A part that needs anything more
  stops, goes in your report, and the rest is built.

**Files:** `<<P24>>` (df-ruling T4 item 1's sentence, word for word). **Plants:** `<<P26>>` (j-r2 B6, the words after
"From now on: "), then restored and the sha256 checked against the clean file. A `cp`, `mv` or `tee`, and a `>` or `2>`
into a file, are shell writes too, in any folder (N/u4r-brief.md:1016-1019). An agent that slips says so at the top of
its notes (df-ruling :317-319).

**Commands** (Mustafa's rule of 2026-11-16: `<<P17>>`): what that means for you: one command per call, the runner
call or a plain `git ...` from your worktree as your working directory; no `cd ... &&`, no `;`, `&&` or pipes, never
`git -C`, `git -c` or `git -P`; read and search files with the Read, Grep and Glob tools. A call that may still ask is
named in your notes before you send it. **Git** only in your worktree; commit nothing, push
nothing, never switch, checkout, restore, stash or reset the main folder; the main session merges (plan :925-927).
**Bite ids:** no crate's sources spell another crate's bite id as a string literal (check.mjs's `biteHomeProblems`,
:936; N/u4r-brief.md:1023-1026).

**The stop clause.** TE2-5's rule ended at U4r's merge (the card's M6 line; TE-Q2, te-m3 :119-126). Its after-clause
stands: `<<P30>>`. Whether the U4d-F ruling's stop clause (df-ruling :465-468) still goes into this brief is U4F-Q5; by
its recommendation it reads `<<P25>>` with its first `<crate>` replaced by "npt-room, npt-client, npt-bots or npt-lab
(and each crate U4F-Q1 opens)" and "<crate>'s files" by "their files" (section 12 item 3), its "TE2-5" read as your
notes' failure record. A failure in a crate outside your lane (an npt-net test, as U3C-HELD-1's was) is named by its test
and panic, kept whole under .gate/u4f/, and comes to the advisor whole: never retried into green (the merge-stop
ruling's pattern: merge-stop, process notes, applying B2.3 after TE2-5 ended).

**No retry for the `wasm` step** (N/u4r-brief.md:1055-1060): a mismatch, a trap, a SHA-256 failure or a call stopped by
the limit is never re-run into green; its first tick, field diff and digests go into your notes with the run's whole
log under .gate/u4f/.

**The builders' limits** (Mustafa's words, by script): `<<P14>>`.

**SIZE-CHECK** (Mustafa's words: `<<P18>>`; the main session's reading, history :2067-2075): U4f is size M (plan :805),
and the plan gives only the letter, so its planned size is what earlier M units came to, about 3,000 lines, and it is
past its plan at more than 6,000 lines, counted as his U4r size was (history :2064-2066: lines added by the unit's
patch, into existing files plus new files' lines). This brief's estimate, by component, for the advisor and the card
(the main session writes it beside the plan's 3,000 before the first snapshot):

| Component | Added lines (estimate) | What |
|---|---|---|
| npt-client | about 700 | ClockSync and its filter (2.1), the controller (2.2), carry 80 (b)-(e) (2.9), f1-f5, f24-f27, f31, f32 |
| npt-room | about 950 | option A, the order and the kept window, kinds 1-6 and 10 with their counters and records, the total key, 9 bites, f7-f13, f17, f34 |
| npt-bots | about 650 | the 11 forgers, the clamp cheat, the 1.5x clock, the lag switch, the never-acker, f15, f18 |
| npt-lab | about 1,400 | the suites and settings, every series of 2.7, the per-session counters, the attribution, the -47 suite, f6, f14, f16, f19-f23, f28, f29, f33, f35; RR-CB-2's docs |
| npt-sim (by U4F-Q1) | about 40 | two bite sites and ids |
| SERIES.md (by U4F-Q1) | about 25 | new rows only |
| ops/runs/c-u4f-*.mjs | about 600 | the r2 extension, the cap runs, the judged-set runner |
| rev 3.1's additions | about 350 | the contiguous-stamp rule and the commands-due accessor (client, bots), the join anchor and the past window (room), the per-kind forging and the twin's skip (bots, client), the two lab settings, f36-f38 |
| rev 3.2's additions | about 100 | the catch-up and first-stamp rules (client), the kept stamp per slot by U4F-Q18 (ii) (d) and the signed-distance compares (room), the forging interval setting (lab), f13 (iv), f32 (iii), f39 and f16's per-pair asserts |
| **Total** | **about 4,850 (3,900 to 5,900)** | about 1.6 times M's 3,000 by this estimate; the stop stays his: past 6,000 |

The builder puts `git diff --stat <base>`'s added lines in each block's notes line; past 6,000 the build stops and the
main session tells Mustafa in three lines (what grew, why, the cheaper option), as his rule says.

**His 13:35 rule** (`<<P15>>`, `<<P16>>`): you change no marks.json, no existing SERIES.md row, no marks/README.md and
no judge file (above); if the build needs one, it stops and reports (5.6).

**.claude/rules/netcode.md** binds the proto, room, client and server code U4f writes or feeds (plan :921-922): inputs
only, never state; every inbound value validated; no unbounded buffer keyed by client data (ClockSync's samples, the
room's refusal records and kept window, the cheats' held datagrams all capped); the catalogue updated with any protocol
change. **Hostile input in the logs** (catalogue :120-121): U4f's logs of forged input hold kinds, counts, sequences
and the clamp points SERIES.md rows name, never a datagram's bytes, debugging included (results/ is committed)
(SA-17). **No sockets** (rev 3.1, SA3-6 (1)): every U4f run, test and script is in-process L1: no listener, no L2 or LG
run, no fetch (the runner sets `CARGO_NET_OFFLINE`); a part that needs one stops.

---

## 5. The judged runs (the main session's, after the merge, in the main folder: plan :766, :928-929)

### 5.1 The run list, with counts (plan §9 :958-968; marks.json)

| Mark | Part (marks.json) | Select beyond layer L1 | Measured | Twins | Bites (one run each) |
|---|---|---|---|---|---|
| -20 | L1 P3 option A (:4658) | SS, P3, 48 clients, `cmd_timing` A | 3 | | redundancy-1 (option by U4F-Q10) |
| -20 | L1 P3 option B (:4672) | the same, B | 3 | | |
| -21 | forged, option A / B (:4818, :4832) | SS, 48, suite `forged` | 3 + 3 | 3 + 3 | the 11 no-* bites (option by U4F-Q10; no-catchup-cap A) |
| -21 | clamp, option A / B (:4846, :4860) | SS, 48, suite `clamp` | 3 + 3 | 3 + 3 | refuse-not-clamp |
| -21 | fast-clock, option A / B (:4874, :4888) | SS, 48, suite `fast-clock` | 3 + 3 | | not-older-than (option A) |
| -21 | lag-switch, option A / B (:4902, :4917) | SS, 48, suite `lag-switch`, P1 | 3 + 3 | | (no-catchup-cap only by U4F-Q15 (b)) |
| -22 | L1, one never-acker among 48 (:5158) | SS, 48, suite `never-ack` | 3 | | unbounded (a full S30) |
| -47 | L1 P2, 1,000 connects (:9084) | P2, suite `clock` | 3 | | newest-sample, tick-quantised |
| | **Totals** | | **36** | **12** | **17** (65 runs) |

Against plan §9 (:962-963): -20 6, -21 24 plus 12 twins, -22 3, -47 3, plus 17 bite runs. Conditional: +1 bite run by
U4F-Q15 (b); +13 by U4F-Q10 (c). Each measured run of -20 and -22 is 10 minutes (their notes: marks.json:4803, :5227);
-21's notes name no length (:5144), so its runs take the same 10 minutes, but the fast-clock suite's 610 s measured
window (2.6, ten whole episodes: TE3-6); seeds 1 to 3 (plan.md:772-782); each bite run seed 1 (c-u4r-runs.mjs's
pattern, its header :25-31). -20's runs need 10,000 scripted
presses (marks.json:4738-4747: the brawl makes none, bots/src/lib.rs:279-281, so the suite's behaviour is U4F-Q11's)
and a reorder share above 0 (marks.json:4706-4708; README :276-277; D3). The forged, clamp, fast-clock and never-ack
parts select no profile: U4F-Q11.

### 5.2 The precondition table (plan §4.12 :422-424; advisor conditions 4 and 5, :112-124; README :285-290)

| Mark | `pre:PM-0001-2` | Shown passing first by | Note |
|---|---|---|---|
| -20, option B | yes | U4r's r2 showed it with the brawl at a fixed lead of 7 (c-u4r-r2.mjs:80; U4R-Q4, r-check :198-205); U4f changes both the suite (U4F-Q11 (i)) and the lead (the controller), so 5.3's p6 shows it again with them (TE-3) | |
| -20, option A | yes | 5.3's p5 (carry 86) | |
| -21, option B (4 parts) | yes | U4r's r2 with open-loop stand-ins (c-u4r-r2.mjs:82-85), their own checks not resolved (r-check :198-201) | whether 5.3 re-runs them with the real cheats: U4F-Q11 (iv) (NA-8 shows the stand-ins differ) |
| -21, option A (4 parts) | yes | 5.3's p1-p4 (carry 86) | |
| -22 | **no** (README :285-290's exception) | none needed | its runs still log PM-0001-2's series; the load each run carried is stated in the main session's -22 set report (advisor condition 4, :116-117; TE-22), read from PM-0001-2's series as a report (a separate `--only PM-0001-2` call over the -22 folder, its verdict never counted: the designed drop leaves 47 clients, marks.json:5227), with each run's `overflow_resyncs_count` per client (scenario.rs:1270; SA-16) |
| -47 | no (its part selects no scene) | none | |

A bite run keeps its part's validity rules, the precondition included (README :323-324).

### 5.3 The short pre-runs (carry 86; U4R-Q4, r-check :198-205; TE-3, TE-4, TE-6, TE-13, TE-24)

Run by c-u4f-r2.mjs on c-u4r-r2.mjs's model (SUITES :77-92; its short bitten runs as :88-91), from the merge commit
(r2's record rule, c-u4r-r2.mjs:2-4), seed 1, before -20's and -21's judged sets. Lengths: r2's 10 s warm-up and 20 s
measured (c-u4r-r2.mjs:8-9), except the fast clock's 70 s measured, which holds one whole 60 s episode from the
measured start and 10 s (2.6's schedule; TE-13, TE3-6); a lag-switch run at the suite's spacing (100 bursts in 600 s, one
per 6 s) holds 3 bursts; a clock run holds a round of 48 connects (about 2.5 s). Every 48-client run's warm-up is 10 s
(head-check ADV-CBH-2: PM-0001-2's load checks are shown at the sets' 10 s warm-up, c-u4r-runs.mjs's settings).

**How a short run is read (rev 3.1, TE3-3):** on these lengths the count checks fail by length on clean and bitten runs
alike: -21's "forged: 1,000 commands of every kind", "clamp: 1,000 points beyond reach per weapon kind the sim has",
"fast clock: 10 runs of 60 s" and "lag switch: 100 bursts (inputs held 200 ms under P1)" (marks.json:4934-4945,
:4980-4991, :5014-5025, :5050-5061), and -47's "1,000 connects with random true offsets" (:9096-9103). The judge marks a
bite shown when any plain check of its part fails (judge/src/eval.rs:1475-1478), so a short bitten run's bite status
says nothing. Each run is read instead from its verdict's checks **by name**: the named check in the table below fails
on the bitten run and passes on its clean partner; every check but the length-failing ones above passes on both. Each
bar goes into c-u4f-r2.mjs's `expect` line, as c-u4r-r2.mjs:88 and :91 state theirs.

| # | Suite | Option, profile | Bite | Shows (the check named is read by name) | Why |
|---|---|---|---|---|---|
| p1 | forged | A (profile by U4F-Q11 (ii)) | | every PM-0001-2 check passing and the shot band; "forged: every forged command rejected and counted" and the twin check passing | carry 86 |
| p2 | clamp | A | | the same; "clamp: never refused or counted" passing; the clamp-cheat reruns (0 expected) | carry 86; SA-10 |
| p3 | fast-clock | A | | the same; "fast clock: at most the server's ticks plus the lead bound of commands" passing; the episode's excess, lead-bound and not-newer refusals | carry 86; SA-6 |
| p4 | lag-switch | A, P1 | | the same | carry 86 |
| p5 | -20 | A, P3 | | the same; **-20's two share checks by name passing: "at least 99.9 % of commands reach the server before their tick runs" and "missing inputs at most 0.1 % of player-ticks"** (`tick.s.input_missing` by stamp: NA-14) | carry 86; TE3-1 |
| p6 | -20 | B, P3 | | the same, with U4f's suite and controller | TE-3; TE3-1 |
| p7 | forged | A | no-lead-bound | "forged: every forged command rejected and counted" fails; it passes on p1 | TE-6; TE3-3 |
| p8 | clamp | A | refuse-not-clamp | "clamp: never refused or counted" fails; it passes on p2 | TE-6; TE3-3 |
| p9 | fast-clock | A | not-older-than | "fast clock: at most the server's ticks plus the lead bound of commands" fails (excess above 0); it passes on p3 | TE-5, TE-6; D12 |
| p10 | clock | P2 | newest-sample | "offset error at most 5 ms after 2 s of samples (p99 of \|estimate - true\|)" fails; it passes on p21 | TE-6; D12 |
| p11 | clock | P2 | tick-quantised | the same check fails, or it is reported as not failing (D4); it passes on p21 | TE-6; D12 |
| p12 | -20 | U4F-Q10's option, P3 | redundancy-1 | "at least 99.9 % of commands reach the server before their tick runs" fails, PM-0001-2 passing; it passes on p6 (option B) or p5 (option A) | TE-6; D12 |
| p13 | lag-switch | A, P1 | no-catchup-cap | "lag switch: no tick moves further than the catch-up cap allows" still passes (expected: TE-7), `tick.v.lagswitch_cmds_count`'s max | TE-7, SA-7; D13 |
| p14 | forged | as p1 | | p1 again at seed 1: fate-log and per-tick hashes equal to p1's | TE-24; D11 |
| p15-p18 | forged, clamp, fast-clock, lag-switch | B | | as p1-p4 | only by U4F-Q11 (iv) |
| p19 | lag-switch | A, P1 | | an unjudged 400 ms hold (about 12 commands): the catch-up cap refuses the excess, `tick.v.lagswitch_cmds_count`'s max 8 | only by U4F-Q15 (c) |
| p20 | fast-clock | A, as p3, 130 s measured | | an unjudged idle before an episode: the fast client sends nothing for the measured window's first 60 s, then runs 1.5x for one 60 s episode: that episode's excess at most W, reported, never judged (judging it would be a bar: 5.6) | SA3-1; only by U4F-Q18 (a) |
| p21 | clock | P2 | | the clean partner of p10 and p11: the p99 check passing | TE3-3 |

Each run also reports carry 74's late share in both readings (option B: the room's `late` count against the commands
sent; option A: the commands that arrived after their stamped tick ran, 1 - `tick.s.cmd_on_time`'s share, SERIES.md:61)
(TE-4; D15); carry 77 (c)'s `stale()` sum; the per-session refusal counters (2.5); each cheat's `input_limited` (0
expected) and the fast clock's `ping_limited` (above 0 expected: 2.6); the client's skipped stamps (2.2) and each twin's
left-out count (2.12); the summary line's C62H-BOUND tally (lost ticks by reason: scenario.rs's Summary and
logs.rs:179-191), read as the -17 sets' were. A suite that fails a check outside the length-failing list, p5's or p6's
share check by name missed, a late share above 1 % on another run, a bitten run whose named check does not fail, or a
clean partner whose named check fails (README :320-321) comes to the advisor before its set (carry 86). One no-* bite of
the forged suite's 11 runs short (p7); the other 10 rest on their unit proofs until the judged set.

### 5.4 Each judge call (JDG-ONLY-1's interim rule; ADV-CBH-1's lesson; twins in one call)

- **The rule, word for word:** `<<P31>>` (c62h :20; the card's JDG-ONLY-1 block). -20's and -21's parts carry
  `pre:PM-0001-2` (marks.json:4667-4669, :4681-4683, and every -21 part), so: `npt-judge --only PM-0001-2,PM-0001-20` and
  `--only PM-0001-2,PM-0001-21`; `--only PM-0001-22` and `--only PM-0001-47` (neither carries it). Each -20 and -21
  run's header, measured, twin and bite, lists PM-0001-2 and its own mark (c-u4r-runs.mjs's header :26-31); a -22 or
  -47 run lists its own mark (and -22's PM-0001-2 for 5.2's load report).
- **ADV-CBH-1's lesson** (head-check, ADV-CBH-1 and H2): `--only` drops every mark it does not name from the marks file
  the judge reads its preconditions from (judge/src/main.rs:85-93; eval.rs:958-962, :1186-1188), so a call that names
  a mark with `pre:PM-0001-2` and leaves PM-0001-2 out reads no check of that mark, every time. This binds every call
  U4f plans: the judged sets, 5.3's short bitten runs judged by hand (D12) and any scratch or pre-flight call. Each
  by-hand call also takes ADV-CBH-2's hardenings: the npt-lab binary built in the same main-run.sh call; the npt-judge
  and marks.json sha256s checked against the set's, with a stop on any difference; `--commit` the full `git rev-parse
  HEAD` after `git diff --quiet <checked head> HEAD -- spikes/netcode-proof ':(exclude)spikes/netcode-proof/results'`
  exits 0; `--date` the day it runs; a 10 s warm-up for 48 clients. Its commands are written in the main session's
  notes before it runs, for the advisor's read.
- **Twins in one call** (C11): the twin figure reads the run its `params.twin` names, from the same judge call's set,
  with the same profile, seed and bite and role `twin` (judge/src/figures.rs:191-214; c-u4r-runs.mjs:42-43). So
  -21's 24 measured runs and 12 twins are judged in one call; its 13 bite runs may go in a second (U4r-F2's split,
  SPLIT_LINE c-u4r-runs.mjs:167, its words adapted by U4f's runner: "measured runs and their twins").
- **Memory** by c-u4r-runs.mjs's fit (302.2 + 2.3403 x MiB of logs, :177) against the 10,240 MiB line (:179): -21's
  36 measured and twin runs at about 1,922.4 MiB of logs (36 x 53.4: 56 MB a run, c-u4r-runs.mjs:154) give about
  4,801 MiB; its 13 bites about 694.2 MiB, about 1,927 MiB; one call over all 49 about 6,426 MiB (estimates), each
  under the line.
- **U4f's set runner's stop note** names every set judged in two calls (the lesson of TE-CBH-5 = NA-CBB-1, whose
  stale "(P0)" stays in c-u4r-runs.mjs:14 for that file's next change: 9.2).

### 5.5 Time and disk per set (each an estimate, in MiB; the run that measures it named)

| Set | Runs | Disk (MiB, estimate) | Wall time (estimate) | Measured by |
|---|---|---|---|---|
| -20 | 7 | 373.9 (7 x 53.4: 56 MB a run with no -57 series, c-u4r-runs.mjs:149-154; -20's per-command series may add) | about 39 min (18,335 ticks at P3's 54.7 ticks/s: about 335 s a run) | the first -20 run |
| -21 | 49 | 2,623.2 (49 x 53.4, and the fast clock's 7 runs at 610 s measured, about 6.2 more) | 3.6 to 4.8 h (263 s a run at P0's 69.6 ticks/s, up to 355.4 s, the longest judged run) | the first run of each suite, and 5.3 |
| -22 | 4 | 320.4 (3 x 53.4, plus the S30 bite, three 10-minute runs' worth, 160.2) | about 34 min (the S30 bite about 1,015 s) | the bite run |
| -47 | 5 | no figure (1,000 connects in about 21 rounds of about 2.5 s: about 1 min of simulated time a run) | no figure | f29's short run; p10, p11, p21 |
| pre-runs (5.3) | 15 (19 by U4F-Q11 (iv); 20 with U4F-Q15 (c); 21 with U4F-Q18 (a)) | about 195 (15), 255 (19), 266 (20) or 315 (21): r2's 13 suites took about 138.3 (145 MB: c-u4r-r2.mjs:40-43), about 10.6 a run; a 70 s fast-clock run about 2.7 times that, p20's 130 s about 4.7 times | about 1 to 1.5 h | the runs themselves |
| the cap measurement (2.8) | 9 | about 95 (9 x 10.6) | under 1 h | the runs themselves |
| **the build phase** (rev 3.1, TE3-8) | the builder's builds and its D7 and D8 runs | no figure: the shared target's growth from one debug build of both builds, the release builds and the wasm builds, read by the builder (`du -sm` of .gate/shared-target and of .gate/u4f/, each in its own call) after its first build of each kind and written on the card. For scale only: free fell about 1,505 MiB between the P2 pack (6,455) and the B1 pack (4,950), C62H-BOUND's sets and both lanes' builds together (the card, KEPT-LOGS' second and third packs) | | the builder, at its first builds |
| **Total** | **65 + 15 + 9 (+ 6)** | **about 3,608 (up to about 3,728)**, the build phase apart | **about 6 to 8 h** | |

**Room (NA-5 as changed; rev 3.1, TE3-8, SA3-5, TE3-10):** the last reading is 5,514 MiB free after U3C-HELD-1's E6
(2026-11-17 12:34 UTC; the card), 1,418 MiB above the 4,096 MiB floor, against about 3,608 to 3,728 MiB of logs plus
the build phase. Packing keeps the archive (§11 :249), and the two packs of the u4r sets gave back about 73 % of their
folders (P2: 572 of 779 MiB; B1: 603 of 820.5 MiB; the card), so u4r-P3-0f066338dd8d (519 MiB) gives back about 380 MiB:
about **1,798 MiB** above the floor after it, before any part-clean. Where room comes from, in this order, never during
a build or a set (.claude/CHECKPOINTS.md §11 :236-238):
1. **At launch, before the builder starts, with no build and no set running** (U3C-HELD-1's plan ruling places this
   pack before U4f's counted run: held-review :37, :89): pack u4r-P3-0f066338dd8d **by brief 8.1's procedure** (c62hb-brief
   :494-499: the ruling named, the list committed alone and pushed, `kept-logs.mjs pack`, free before and after on the
   card), as PACK-READ (a) allows (`<<P22>>`; RECS-1043: one or two closed, older folders; B1 was the first) under the
   advisor's R4 (report-head; the card: 8.1's list ends with it); then the main session's part-clean of the shared target
   (W1-TARGET item 3 as amended: `<<P27>>`), with D2's readings before and after each. That is the last folder 8.1 and
   PACK-READ cover.
2. **After U4f's merge and before its first set,** with no lane running, the part-clean again (the builder's artefacts):
   the judged sets build in the main folder's own cache (`CARGO_TARGET_DIR` unset: plan :918-919; c-u4r-runs.mjs:6-9),
   so emptying the shared target does not touch them.
3. **A U4f set's own raw-log folder** is packed only after Mustafa's yes to KL-U4F (section 12 item 6): his PACK-READ
   yes covered one or two older folders, and R4 limits packing to 8.1's list, so no ruling may widen it alone. Until
   that yes, the sets are ordered and split (U4F-Q2 (c)) to fit what items 1 and 2 give, and a set that does not fit is
   not started.
4. Each set's runner checks its own projection against the floor before it starts (c-u4r-runs.mjs:225-229).

**Never** either P0 folder (u4r-P0-93509dfd7fd1, the kept set, and u4r-P0-051995db9cea, U4r's), fc411116c1f1's or
fd4612050cd6's (C62H-BOUND's reference sets) or the c-u2 folders, without his word (c62hb-brief :500-501; R4). A hash
mismatch or a folder that no longer matches its list: the folder stays, tell him, pack nothing more (§11 :243-245). The
builder never cleans or packs (section 4 rule 6): a floor stop mid-build waits for the lane to pause. D2 re-reads the
free space and the folders' sizes before the advisor's check, at launch before and after item 1, and after item 2.

### 5.6 marks.json, SERIES.md and the judge: the bars (his 13:35 rule, `<<P15>>`, `<<P16>>`)

Every change U4f could make to marks.json, SERIES.md or the judge (check, part, bite, figure, fixture, threshold), and
whether it is a new or a changed bar:

| # | Change | File | Kind | By the recommendations | To Mustafa |
|---|---|---|---|---|---|
| B1 | none to -20's, -21's, -22's or -47's parts, checks, bites, thresholds or notes: U4f's runs fit marks.json as it stands (5.1), and every figure they read exists (generic stats; `twin_hash_mismatch_count`, figures.rs:195, :830) | marks.json, crates/judge | - | no change | nothing |
| B2 | new SERIES.md rows for report-only series (2.5's per-session counters, the settled lead, `seq_outside_window`, `unencodable_commands`, the fast clock's per-episode refusals, each cheat's `input_limited`, the clamp reruns, the cap measurement's high-water series; rev 3.1: the client's `stamps_skipped`, the twin's `twin_left_out`, and the header params `fast_clock_episode_ns` and `connects_count`); no check reads any of them | SERIES.md (U4F-Q1) | not a bar | built | nothing; the read is recommended (c62h-review :93) and named in the report |
| B3 | the lead bound's value (U4F-Q8: 400 ms = 12 ticks recommended): not a text in marks.json, but it sets how much the fast-clock checks allow (marks.json:5026-5049: "the server's ticks plus the lead bound"); under option A, after an idle, a client can run up to the bound plus W more (U4F-Q18 (a); rev 3.1, SA3-1), which the draft for him states | the room's setting, the header | a bar's allowance, set for the first time | built at the ruled value | **recommended for his yes** (U4F-Q8 (iii)): the value and the allowance it gives, before the build |
| B4 | the event cap's value (U4F-Q12): a design constant -22's headroom check reads (marks.json:5227: "a design constant ... reported"); not a threshold | the room's default, plan §4.7 | not a bar | measured, then set | nothing (the advisor sees the figures first: carry 57) |
| B5 | U4F-Q11 (iii) (a): the fast clock's honest movement as the bound's arithmetic, a reading of SERIES.md:156 that changes what `event.fast_clock.excess_move_um` computes | SERIES.md:156 | a changed bar | not taken ((c) changes no text) | only if ruled |
| B6 | U4F-Q14 (c): a forged size kind and a forged rate kind, each with a check | marks.json -21, SERIES.md | new bars | not taken | only if ruled |
| B7 | U4F-Q15 (b): the lag switch's hold lengthened to at least 300 ms, and no-catchup-cap's bite run in the lag-switch suite | marks.json -21 (its part notes, :5051's check name, its bites' notes :5144) | a changed bar and a new bite run | not taken ((c) reports beside it) | only if ruled |

**So:** by this brief's recommendations U4f changes no marks.json text and no judge file, and adds only report-only
SERIES.md rows; one bar input goes to Mustafa for his yes (B3); three more only if a ruling picks them (B5-B7, all
-21's). **Before any of B3, B5, B6 or B7 is built** (`<<P15>>`; his C62H-BOUND form, history :2144, :2172-2175): the
main session drafts the exact texts (the marks.json objects and SERIES.md rows whole, or the value with the allowance it
gives) and the planned size with its stop; the advisor and the test-engineer read them; they go to him in
c62hb-paste's form (plain words; the texts in fences; the planned size table; how it differs; what stays open; the
runs; the freeze) under Decisions needed; nothing is built before his yes in his own words; at his yes each text's
sha256 is recorded, and before the judged runs a script shows the built files hold exactly those texts; any change
after it goes back to him; no bar's text is changed to match a run's result (`<<P21>>`). **The read at U4f's close**
(`<<P16>>`; c62h-review :84): `git diff --stat <base>..<head> -- spikes/netcode-proof/marks/marks.json
spikes/netcode-proof/crates/judge` (D10); if it is not empty, a reviewer's read is owed before U4f closes (the
test-engineer's review can be it, if its notes name the lines read and marks.json's sha256 before and after), and the
report to him carries the read as `<<P16>>` asks; the same read is recommended for SERIES.md's new rows and is named
the same way.

---

## 6. End checks (each a counted run under section 4's rules: `-x`, `df` before each)

1. check.mjs twice, every step passing both times: 11 steps (fmt, clippy, clippy-wasm32, doc, test, deps, wasm, refs,
   marks, self, redact), in N/u4r-brief.md:1073-1079's form: each run's last line, the test step's two counts as deltas
   on `<base>`'s (at least +109 measured and +124 bites, plus each conditional delta a ruling brings), the self count,
   the provenance lines, and the test step's wall time per build (2.10's scale rule). The test step's run includes C62H-BOUND's lab tests (scenario.rs:2795-2990) and U3C-HELD-1's
   held-bound test (crates/net): a failure of the short P2 test stops for Mustafa (section 1 item 5); a failure of a
   test outside your lane stops for the advisor (section 4's stop clause).
2. clippy on the host and on wasm32, in both builds, warnings as errors: check.mjs's two clippy steps; wasm32 covers
   LEAF plus npt-tcpinfo (check.mjs:39, :1970), so npt-room and npt-client (ClockSync, the controller, option A) are
   linted for the Worker's target too; npt-bots and npt-lab on the host only.
3. fmt: check.mjs's fmt step; a diff applied by Edit (section 4's write rule).
4. **Only if a ruling opens npt-server before U4h** (U4F-Q1, U4F-Q14 (a) or U4F-Q6 (d)): `cargo test -p npt-server
   --all-features` compiling, with that command's run as the evidence: the follow-up's words `<<P33>>` (r-b2 :28).
   Today it stops at E0425 in server/tests/loopback.rs (r-b2 :27; server/Cargo.toml and tests/loopback.rs unchanged
   since 2fbec77: D9).
5. The run scripts (c-u4f-*.mjs): each npt-lab command line they issue, run by hand on a short setting, with its line;
   each judge call they issue in 5.4's form.
6. `node tools/gate.mjs --no-smoke` is the merge step's, in the main folder (plan :510): not yours.

---

## 7. Return list (your report, in this order)

1. `<base>`; your final diff's file list (`git status --porcelain`, `git diff --stat <base>`), each file inside section
   4's lane or a lane a ruling opened (named with it); `git diff --stat <base> -- spikes/netcode-proof/crates/sim`.
2. Each build item of 2.1-2.9: built (file:line) or not (why); 2.4's order and units as built, the kept window's K;
   the strictly-rising stamp rule (2.2) as built; the contiguous-stamp rule and the controller runs' command rule
   (2.2), with each run's skipped stamps; the first-stamp rule (2.2, rev 3.2); `canonical`'s key (2.3); the join anchor,
   option A's past window W and, by U4F-Q18 (ii), the kept stamp across joins as built (2.3, 2.4), or why not; the
   signed-distance form of each tick compare (2.4 item 4); `tick.s.input_missing`'s reading under A (NA-14); where 2.5's counters are read
   before each session end (NA-15); each forger's and twin's construction per kind as built (2.6, 2.12).
3. The tests f1-f39 and any you added: name, file, what each shows; the counts in both builds as deltas (at least +109
   and +124, plus the conditional deltas the rulings bring), with the `--list` lines; the negative cases counted (at
   least 64); each new test's wall time alone and the test step's against `<base>`'s (2.10).
4. Each bite of 2.11: id, site (file:line), the test that fails by name in the bites build, that test passing clean.
   **Each plant** (f9's sort removed, f26's two halves, f27's, D14's old comparator; rev 3.1, TE3-14): its file:line,
   the test failing by name with the plant in place (the runner's log), the restore and the restored file's sha256.
5. The cap measurement (2.8): per profile and seed the rates and high-water marks; the value proposed and its
   derivation, against carry 50's upper bound.
6. f16's per-kind twin results under both options; a kind whose twin mismatched, with its first differing tick.
7. ClockSync's error on its short P2 runs (phase per connect as L1 gives it) and the controller's settled lead per
   profile beside U4R-Q14's numbers.
8. Every carry of section 3 by number: met, not met or not applicable, each with why.
9. Every pub item by crate against §4.1 and plan.md §4.7 (2.13); any beyond them named as such.
10. What stopped and goes to the main session: lane needs, tick counts or rate-derived constants (carry 39's form),
    seam or wire changes, any marks.json, SERIES.md-row or judge need (5.6).
11. The end checks (section 6): each line, each `df` reading, each counted run's provenance and its `-x` use; the
    short P2 C62H-BOUND test's line.
12. How each U4F-Q ruling (Q1-Q18) was applied.
13. The items that rode (9.2: RR-CB-2, TE-B1-R2-1 with RR2-B1-1, RR-B1-7 (1); T-6 if contract.rs opened) and how.
14. The bars line: `git diff --stat <base>..<head> -- spikes/netcode-proof/marks/marks.json
    spikes/netcode-proof/crates/judge` and the SERIES.md rows added (5.6).
15. Every stop-clause entry (by U4F-Q5) and every test failure, with its cause; no retry into green.
16. The diff's size (SIZE-CHECK: added lines, against the plan's 3,000 and this brief's 4,850); what you did not verify;
    what is left; who built it. (5.3's pre-runs and the judged sets are the main session's, after the merge.)

---

## 8. Recheck: every item from an earlier unit this brief cites, walked at the head

Rows 1-33 are rev 2's (cited at 11fd190), each "holds" or "changed: what the code has, and what the brief now says";
rows 34-45 are new, for what merged since 11fd190 that the brief relies on. At launch, section 12 item 10 re-walks a
row only if its file moved between the head read and `<base>`.

| # | Item | Built by | Its use in U4f | At the head |
|---|---|---|---|---|
| 1 | `InputBuffer`, `CMD_CATCHUP`, `buffer_depth_ticks`, `newest_received`, `offer` (its order, the newest-sequence write :89), `take`, the const assert; -58's `CMD_SEQ` and `PRESS` | U4e | option B kept; option A beside it; 2.4's order; kinds 2, 3, 5, 10; TE-19 | holds: room/src/input.rs:10-13, :17-30, :61, :64-66, :68-71, :80-125 (:82, :89, :98, :110), :127-151, :155 |
| 2 | `canonical` and its unit test | U4e | carry 80 (a)'s total key | holds: room.rs:361-369 (comparator :367), test :1399-1402 |
| 3 | `ClientCounters` | U4e | the new kinds' counters; 2.5 | holds: room.rs:63-136 (:103 overflow resyncs, :116-126 the command counters) |
| 4 | `ClientSlot`'s `queue`, `headroom`, `buffer_depth_ticks`, `kicked` | U4e | 2.8, -22's series, U4F-Q3 | holds: room.rs:277-297 |
| 5 | the event cap default and flag; `EventQueue`, `PROVISIONAL_CAP`, `MAX_EVENT_AGE`, push and expiry | U4e | 2.8; NA-12 | holds: room.rs:460-461, :481-484, :558-560; events.rs:19-20, :22-31, :40, :52-59, :87-92 |
| 6 | `receive`, `receive_input`, the ack check, the oldest-first offer | U4e | kind 8; 2.4's order | holds: room.rs:713-742, :744-801 (:783-788, :791, :795) |
| 7 | `fill_input` | U4e | option A's run path | holds: room.rs:831-878 |
| 8 | `npt_room::BITES` | U4e | 9 new ids | holds: room/src/lib.rs:38-46 (7 ids) |
| 9 | carry 21's room test | U4e | carry 80 (a), f9 | holds: room/tests/room.rs:235 |
| 10 | -22/unbounded's tests | U4e | the bite's unit proof | **changed (cite only; the file is unchanged):** `unbounded_case` :1303, the bites test :1363, the clean headroom test :1444; 2.11 cites them |
| 11 | `Counters`, `RttSource`, `Decoder`, `pump` (the room's ping, the Input bucket, the ping answer), `pong` and its hold clamp, `PING_INTERVAL_NS`, `OUTSTANDING`, `SAMPLES` | U4e | 2.5; 2.1's samples, U4F-Q6 (e); U4F-Q14 | holds: server/src/pump.rs:110-161, :183-190, :204, :372-480 (:387-404, :449-457, :458-472), :500-516 (:510), :16, :24, :26 |
| 12 | `npt_server::BITES` | U4b, U4r | unchanged | holds: server/src/schedule.rs:252-257 |
| 13 | no-trailing and no-channel and their tests; `strict` | U4a, U4e | kinds 9 and 11's bites; SA-12 | holds: proto/src/quant.rs:34-42, :108-109, :115-122; proto/tests/bites.rs:211, :238; server/tests/pump.rs:477, :483 |
| 14 | `ClientEndpoint` (fields, `new`, `with_lead`, accessors, `input`, `step`, `receive`, the too-old cases, `see`, `file`, `rebuild_sequences`) | U4e (BRAWL-1's `own_soldier`) | 2.1, 2.2, 2.9; SC-1; U4F-Q13 | holds: client/src/lib.rs:208-243, :260-294, :296-301, :343-410, :414-445, :450-490 (:485-488), :492-525, :528-565, :583-615, :645-658, :671-701 (rev 2's :671-688 ends early) |
| 15 | `full_state` | U4e | carry 80 (e) | holds, now read: client/src/lib.rs:810-839 (the duplicate at :817-820) |
| 16 | `npt_client::BITES`, `grid_with_bite`, `with_bite` | U4e | 3 new ids | holds: client/src/lib.rs:28-40, :314-326 |
| 17 | `Behaviour`, `Kind`, `script`, `step`, `command`, `FAR_CM`, `MOVES`; the honest command rule | U4r, BRAWL-1 | 2.6's cheats; NA-8 | holds: bots/src/lib.rs:78-112 (`Kind` :80-90 holds an open-loop `Presses` kind, :85), :180, :213, :272, :74-76, :30-49, :14-21 |
| 18 | `toward_centre`'s doc and the dead-input test (TE-B1-R2-1's home) | BRAWL-1 | rides (9.2) | holds: bots/src/lib.rs:128-132; the test at :362 (its dead input at :404: N/u4r-report.md:1139) |
| 19 | `lead_for`, `io` (the pong's tick), the session ends, the endpoint's build, the behaviour dispatch, `params` (`cmd_timing` "B"), the suite, `LAB_BITES`, `Bites::of` | U4r | 2.2, 2.3, 2.5, 2.7, NA-4 | **changed (C62H-BOUND's +311):** `lead_for` :132-137; `io` :475-562 (tick :476, pump :480); `drop_slot` :458-472; `send` :574-601; the build :897-913 (`with_lead` :900; behaviours :903-913; the host offset :930); `params` :1366-1418 (suite :1369, `cmd_timing` :1370, lead :1371, twin :1411); `parse` :1447 (:1488-1489); `LAB_BITES` :215-219 (3 ids: whole-as-lost, late-command, link-lost-overstated); `Bites::of` :227. The brief cites these |
| 20 | `room_tick` | U4r, BRAWL-1, C62H-BOUND | 2.7's series; RR-B1-7 (1) | **changed:** lab/src/logs.rs:524 (8 inputs now, the link list among them; `#[allow(clippy::too_many_arguments)]` :520-523, accepted by cbr-rust point (a)); U4f's series widen it (RR-B1-7 (1) rides) |
| 21 | the contract checker | U4r | 5.3; T-6; f28 | holds: lab/src/contract.rs:1-25 (its (b): every written name has its SERIES.md row, :6-9) |
| 22 | r2's suites, lengths and record rule; the set runner's floor, byte rates, limits, judge fit, the split, `planJudged` | U4r, U4r-F2/F3, C62H-BOUND | 5.3-5.5 | **changed (c-u4r-runs.mjs +47):** c-u4r-r2.mjs holds (:2-4, :8-9, :40-43, :77-92); c-u4r-runs.mjs (f2002890…): header :1-60 (the stop note :13-17, the split :36-45, twins in one call :42-43), `FLOOR` :133, byte rates :138-154, `SPLIT_LINE` :167, `RUN_LIMIT_MS` :172, `JUDGE_FIT` :177, `JUDGE_LINE_MIB` :179, the floor check :225-229, `bitesOf` :312 (-17's two bites on P2 now), `planJudged` :327, `--only` :569 |
| 23 | `World::BITES`, `World::with_bite` | U4d | NA-9's two sites | holds: sim/src/world.rs:937-938, :944 |
| 24 | the map's extent; the weapon's reach | U4d | kind 6; the clamp cheat | holds: sim/src/greybox.rs:62, :151; sim/src/scene.rs:66 |
| 25 | `TICK_HZ`, `ticks_for_ms_at`, `ticks_for_ms`; `ACK_RING_TICKS`, `FULL_STATE_ACK_TICKS` | U4s | 2.1, 2.2, f21, f31, U4F-Q16 | holds: sim/src/ticks.rs:12, :25-37, :49-70 (:56, :70) |
| 26 | the L1 profiles and their per-direction halves | U4c | 2.1, 2.2's leads; 5.1; D4 | holds: l1/src/lib.rs:149-163, :177-202, :214-221 (C62H-BOUND's insert is after :411) |
| 27 | `LEAF`, `WASM_CRATES` | U4d2 | section 6 item 2 | **changed:** `LEAF` holds at check.mjs:39; `WASM_CRATES` is :1970 (REFS-KIT's +13 before it); section 6 cites :1970 |
| 28 | marks.json's -20, -21, -22, -47 and SERIES.md's rows (`lead_ticks_count` among them) | U4j, C62H-BOUND | 2.7, 5 | **changed:** marks.json +26 inside -17 (so every -20/-21/-22/-47 line is +26; 0.4); SERIES.md's rows after :258 +2, after :370 +3 (`lead_ticks_count` :344); the brief cites the head's lines |
| 29 | u4r-run.sh (u4f-run.sh's model) | U4r | section 1 item 4 | **changed:** sha256 e9437ae200d234725b392bc5090e2b802d9938bb8ba5b12c23123c3d607e97d7 (117 lines) since TE-B1-R2-2's fix 8df63d3 (rev 2 named U4r's launch copy, 349b51ad…) |
| 30 | the header's two command acknowledgements: `newest_received` sent | U4e | U4F-Q13 (SA-3) | holds: room.rs:1322 |
| 31 | `client_counts`' footer; U4r's r8 and r9 | U4r | 2.5, f21, SA-16 | **changed:** scenario.rs:1259-1271 (`overflow_resyncs_count` :1270), r8 :2212, r9 :2267; SERIES.md's row :360 |
| 32 | the sim's clamp and its refusals (Reload, Ammunition, Cooldown) with their (slot, seq, kind) records | U4d | kind 7, the clamp cheat; SA-10, SA-15 | holds, now read: world.rs:1283-1312, :1401-1420, :1445-1452 |
| 33 | the overflow resync and the Kick path | U4e | -22, f21, SA-16 | holds: room.rs:318-323, :979-1038 (:1023-1034), :1057-1063 |
| 34 | `DatagramCounts`, `FateLog::server_datagrams_since`, `L1Net::with_fate_log` | C62H-BOUND (eb2bebf) | none directly: the lab's link record runs at every send of U4f's runs (2.7) | new: l1/src/lib.rs:332, :412-431, :1918; its test l1/src/tests.rs:398 |
| 35 | the link record: `link_lost`, its call around each send, the tally, the Summary's link totals, `Bites::link_lost_overstated` | C62H-BOUND (eb2bebf) | every U4f run passes the join (2.7); RR-CB-2's docs ride (9.2) | new: scenario.rs:604-631, :1045-1060, :198-201, :800-805; logs.rs:179-191 (the tally), :518-524, :646 (`tick.s.snap_link_lost_count`) |
| 36 | npt-judge's shared room-side reader and its two figures | C62H-BOUND (b06991e) | they read the filing carry 80 (b), (d) touch (2.9); U4f changes no judge file | new: judge/src/figures.rs:538-675 (the reader and its gaps), :677, :689, :818-826 |
| 37 | PM-0001-17's two checks and its planted bug link-lost-overstated; C62H-KEEP's L1 P0 check | C62H-BOUND (15102d1), 5e563b9 | 2.9's guard | new: marks.json:3972-3982, :3983-4005, :4036-4039 |
| 38 | the four frozen SERIES.md rows (two figures, `tick.s.snap_incomplete`, `tick.s.snap_link_lost_count`) | C62H-BOUND (15102d1, eb2bebf) | never edited by U4f (section 4; history :2173) | new: SERIES.md:291, :292, :322, :378 |
| 39 | the set runner's per-profile bites and split (`bitesOf`, `planJudged`, `SPLIT_LINE`) | C62H-BOUND (561bf42) | U4f's set runner's model (5.4) | new: c-u4r-runs.mjs:312-327, :167, :13-17 |
| 40 | the lab's short P2 test and the join tests | C62H-BOUND | the guard on carry 80 (b), (d); his stop rule (section 1 item 5) | new: scenario.rs:2904 (`c62hb_p2_short_every_measured_lost_tick_is_explained`), :2795-2843, :2954, :2970 |
| 41 | U3C-HELD-1's held-bound check | U3C-HELD-1 (0ad9661) | none: a test outside U4f's lane that runs in every check.mjs (section 4's stop clause) | new: crates/net/src/l2/delay.rs (the bound at every write) and delay_tests.rs (G-NC2); its counted run E6 at 0ad9661 gives `<base>`'s counts (section 12 item 1) |
| 42 | REFS-KIT: the refs step's one exception | 914798a | the refs step's count; U4f names spikes/ from no file outside it | new: check.mjs:1841-1862 (`KEPT_LOGS_FILES`); plan.md:453, :662-667 |
| 43 | the kept-logs lists under results/ | the main session (b5cb253, 6c8c7cf, 978202d) | refs and redact count them; a new pack list adds one each (5.5) | new: results/u4r-precheck-30/faf42aeb034c/, results/u4r/64f3307d5c7f/B1/, results/u4r/ab10f058a828/P2/ (raw-logs.sha256 each) |
| 44 | `Room::leave` (and `Leave` in `receive`) | U4e | 2.5's read before a session's end (NA-15) | new: room.rs:689-695, :739 |
| 45 | carry 75's admission bound in the runner | U4r | the -47 suite (NA-16) | new: scenario.rs:70 (`ADMIT_EVENTS_BOUND` 32,768), :995-1006 |
| 46 | the `Input` codec's two rules and the view offset | U4a | 2.6's per-kind forging and twins (TE3-4, SA3-2) | new at rev 3.1: proto/src/input.rs:19-21, :92-95, :140-198 (`NotCanonical` :155-157, `OutOfRange` :158-159); the endpoint sends nothing when an `Input` fails to encode: client/src/lib.rs:466-469 |
| 47 | the state hash's command sequences | U4d | 2.12's twin rule; U4F-Q13 (SA3-2's variant rejected) | new at rev 3.1: sim/src/world.rs:1922-1925 (only the press counters left out), :1979 (projectile seq), :1995-2009 (Shot and Hit carry seq) |
| 48 | the fate log's canonical form and its draws | U2, U4c | f16's comparison (TE3-5) | new at rev 3.1: l1/src/lib.rs:353-362 (size in each entry), :1125-1150 (the fate by flow index; size only for `TooLarge` and the entry) |
| 49 | `JoinAccepted`'s tick; `take` for every joined slot each tick; `ran_through` | U4e | the join anchor; U4F-Q18 (SA3-1) | new at rev 3.1: room.rs:681, :841-846; input.rs:23, :129-131 |
| 50 | U4r's committed run lines at P3 and P0 | U4r's sets | 2.2's contiguous stamps (TE3-1) | new at rev 3.1: results/u4r/0f066338dd8d/P3/runs.txt:18-20; results/u4r/051995db9cea/P0/runs.txt:22 |

**Tally:** rows 1-33: 25 hold, 8 changed (10, 19, 20, 22, 27, 28, 29, 31; row 10 in its cite only); rows 34-45 added;
rows 46-50 added at rev 3.1.

---

## 9. Open items and follow-ups

### 9.1 Open items (each routed or left)

| Id | Its home | In this brief |
|---|---|---|
| C62H-BOUND, C62H-KEEP, KL-YES | rev 2's three "open with Mustafa" rows | **answered** (C3): 2.9; 5.5 |
| U3C-HELD-1 | merge-stop C3; the card's Open findings | **closed** 2026-11-17 on aab3712 (fix 0ad9661), before U4f's merge as C3 placed it; its deferred Lows in 9.2 |
| C62HB-Q4 (b) and ADV-CBR-1 | `<<P23>>`; RECS-1043; report-head R1; the card | **new: U4g's**, not U4f's (2.3's -58 row) |
| -17's counted P2 and P3 sets after U4f's client edits | 2.9 | **new:** the advisor's call at U4f's close, on f24, f26 and the short P2 test |
| JDG-ONLY-1 and its interim rule | c62h :18-20; c62h-review :86; c62hb-brief :75-78 (never on the judge crate at the same time as another lane) | 5.4 applies the interim rule (`<<P31>>`); the fix is its own change-set, npt-judge's next change, not U4f's |
| the check.mjs follow-up, with CHECK-NFF-1 | N/u4r-report.md:265-274; merge-stop C4; the card | left: check.mjs's next change; U4f opens check.mjs only by U4F-Q8 (a), and then they ride |
| npt-server `--all-features` compile | r-b2 :26-28 (`<<P33>>`) | section 6 item 4 if a ruling opens npt-server; else U4h |
| option A's depth note | the card's DEPTH ruling and its option-A note | U4F-Q3 |
| NC-U4R-5 / EA-U4R-7, the logs half | r-r1 :78-81, :200-204; N/u4r-report.md:281-282 | U4F-Q4 (-47's rejoins) |
| T-6, the contract half | r-r1 :119-123 (`<<P34>>`); N/u4r-report.md:283 | rides if 5.3 edits contract.rs (+5 tests, 2.10) |
| EA-U4R-4, EA-B1-1's later half, NC-U4R-7 | N/u4r-report.md:282-283, :1164-1165 | left by U4F-Q1's recommendation (crates/sim opened for two bite sites only, bites builds); they ride if the ruling opens crates/sim wider |
| EVQ-LOAD | b1-p3 :156-158 (`<<P32>>`); N/u4r-report.md:1049-1053, :1156-1161; carries :223-228 | 2.8 and U4F-Q12 |
| RR-5 room bench | e-r1 :211-217 | left: U4f edits no encode path by this brief; if it does, the bench before and after |
| the readings PR before R5 | advisor :150-153; wave1 :236-240; plan :970-983 | U4f's runs rest on readings already listed; a ruling of U4F-Q3 (ADR 0001:1610's depth words), U4F-Q6 (d) (ADR 0001:247-248) or (e) (ADR 0001:240's "its own ping samples"), U4F-Q11 (iii) (a) or (c) (ADR 0001:1611's honest client), U4F-Q13 (a) (ADR 0001:228-229's "received") or U4F-Q15 (b) (ADR 0001:1611's 200 ms) joins the list (plan :971-974) |
| SA-16's variant: a client that acknowledges only forced full states cycles resyncs and is never kicked | room.rs:1023-1034, :1057-1063; the night's security-auditor check, SA-16 | a STATUS follow-up for U6's limits (section 12 item 11); U4f's -22 runs report `overflow_resyncs_count` per client (5.2) |
| SA-11's carry to U4h | plan :351-352; the night's check, SA-11 | U4F-Q8 (ii); if ruled, the main session adds it to carries' U4h section (section 12 item 11) |
| plan §4.10's "fate logs equal on every flow" (:389-390) for -21's twins | rev 3.1, TE3-5 | **new, for the advisor's check:** read as entry-by-entry equality without each datagram's size and resends (f16), since the twins differ in size by design (plan :377-379) while their fates are drawn per flow index (l1/src/lib.rs:1125-1138); a plan reading, not a bar |
| SA3-1's option-A bank and ADR 0001:231-233's "never more commands than the server has ticked plus that lead bound" | rev 3.1, SA3-1 | U4F-Q18; under its (b) the reading "from join" joins the readings PR (plan :971-974) and the bank goes to OQ-0001-6's write-up |
| KL-U4F (packing a U4f set's own raw-log folder) | rev 3.1, SA3-5; 5.5 item 3 | **new, for him only if D2 shows the room short** after 5.5 items 1 and 2: section 12 item 6's draft |

### 9.2 Deferred Lows since 11fd190: taken (only where U4f opens the file anyway) or left

| Finding(s) | Where (at the head) | U4f | Why |
|---|---|---|---|
| RR-CB-2 (two docs on the first of three fields; a missing "is") | lab/src/scenario.rs:800-805, :198-201; lab/src/logs.rs:181-187 | **takes it** (about 5 doc lines, in 2.7's edits) | U4f opens scenario.rs and logs.rs and edits the Summary and the Counts beside these lines; docs only; cbr-rust's fix field gives the change |
| TE-B1-R2-1 with RR2-B1-1 | bots/src/lib.rs:404 (the test's dead input), :132 (the doc) | **takes it** (`<<P35>>`) | U4f edits npt-bots' lib.rs (2.6), its named home; test and doc only |
| RR-B1-7 (1), the `room_tick` loop rewrite | lab/src/logs.rs:524 on | **takes it** (`<<P36>>`) | U4f's 2.7 series land in `room_tick`, its named home |
| TE-CBH-3, TE-CBH-4, TE-CBH-6, TE-CBH-7, RR-CB-1 | judge/tests/engine.rs, judge/src/figures.rs, judge/tests/fixtures.rs, judge/fixtures/README.md (cbr-test, cbr-rust) | leaves them: npt-judge's next change (JDG-ONLY-1) | U4f opens no judge file (section 4); a judge edit is a 13:35 read (5.6) |
| NA-CBB-3 (the rows' "process client" against the reader's client or bots) | figures.rs:583, :623 | leaves it: none needed | the merge-stop ruling made it the C62H-BOUND report's one-line note (done) |
| NA-CBB-2 (no npt-lab log judged before the P2 set) | - | none: done | ruled in as the pre-flight and run (merge-stop; head-check H2) |
| TE-CBH-5 = NA-CBB-1 (the stale "(P0)" stop note) | ops/runs/c-u4r-runs.mjs:13-15 | leaves it: c-u4r-runs.mjs's next change | U4f writes its own c-u4f-*.mjs and never edits c-u4r-runs.mjs; its own runner's stop note names every two-call set (5.4) |
| NA-UH1-3, NA-UH1-4, RR-UH1-1, RR-UH1-2, TE-UH1-1's strict variant, TE-UH1-6, TE-UH1-7, the loop script's failure-list print | crates/net/src/l2/delay.rs, delay_tests.rs, SERIES.md:613 and :642, the U3C-HELD-1 loop script (held-build :116-122) | leaves them, by U4F-Q1 (d)'s recommendation: npt-net's next change | the taker is "the next change to delay.rs, delay_tests.rs or SERIES.md"; U4f opens SERIES.md for new rows only and never npt-net, and NA-UH1-4 and RR-UH1-2 are one wording across delay.rs's docs and the two rows, which taking SERIES.md's half alone would split |
| CHECK-NFF-1 (`--no-fail-fast` for the bites build) | check.mjs | leaves it: check.mjs's next owner | U4f opens check.mjs only by U4F-Q8 (a), and then it rides (9.1) |
| ADV-CBR-1 | report-head R1 | not U4f's: U4g's brief carry | C62HB-Q4 (b) |

---

## 10. Questions for the advisor (each where the plan or a ruling leaves a choice open)

Status since rev 2: none of Q1-Q16 is fully answered by a ruling or by his words since 11fd190; each line says what
changed. Q17 is new at rev 3, Q18 at rev 3.1 (SA3-1); rev 3.1 also changed Q2, Q4 (a), Q6 (b), Q7, Q8, Q11 (v), Q13
and Q17.

**U4F-Q1, the lane's files.** *Status:* open; (a) changed (no `rss_bytes` L1 writer: 2.7, NA-2), (d) new.
*Question:* which files does U4f's lane hold? *Why it matters:* the plan names none (plan :805-816, :882-883); two of
-21's bites have their sites in npt-sim (NA-9); the new series need SERIES.md rows (the contract test refuses a name with
none: contract.rs:6-9); U4F-Q6 and U4F-Q14 may need crates/proto or crates/server; U3C-HELD-1's deferred Lows name
"the next change to ... SERIES.md" as their taker (held-build :116-122). *Options:* (a) section 4's proposed list, plus
crates/sim/src/world.rs for the two bites' ids and sites only (bites builds), plus marks/SERIES.md new rows only
(U4R-Q2 (c)'s precedent), plus crates/proto and the catalogue only by U4F-Q6 (b) and crates/server only by U4F-Q14 (a)
or U4F-Q6 (d); (b) as (a) but the two npt-sim sites by the main session's own commit before launch; (c) as (a) with
crates/sim opened wider, so EA-U4R-4, EA-B1-1's later half and NC-U4R-7 ride; and for SERIES.md: (d) U4f's new rows do
not make it RR-UH1-2's and NA-UH1-4's taker (they stay with npt-net's next change), or (e) U4f rewords SERIES.md:613 and
:642's up part too. *Recommendation:* (a) with (d): the narrowest lane in which every bite has its site; the riders
need public npt-sim items that are not U4f's; and the ws rows' wording stays one text with delay.rs's docs.

**U4F-Q2, the run budget where no figure exists.** *Status:* open; its premise changed: KEPT-LOGS is in force (§11),
PACK-READ (a) opens packing by brief 8.1 under R4 (`<<P22>>`; RECS-1043), so rev 2's option (c) ("no set before
KL-YES") is gone. *Question:* how are U4f's judged sets budgeted and ordered, given that no time or size figure exists
for -47, -21's suites, option A or a never-acker run, and the estimated total (5.5: about 3,596 to 3,667 MiB) is more
than twice the 1,418 MiB above the floor at the last reading (5,514 MiB free after E6)? *Why it matters:* the runner's
floor stops a set part way (c-u4r-runs.mjs:225-229), and §11 allows only packing between runs, never during a set or a
build (:236-238), never the protected folders without his word. Rev 3.1 (TE3-8, SA3-5, TE3-10): the build phase
also takes room (5.5's new row), U3C-HELD-1's plan ruling places the u4r-P3 pack before U4f's counted run (held-review
:37, :89), the pack gives back about 380 MiB net (not its 519), and neither PACK-READ (a) (one or two closed, older
folders: `<<P22>>`; B1 was the first) nor R4 (8.1's list, which ends with u4r-P3) covers a U4f set's own folder.
*Options:* (a) the builder measures ticks per second and bytes per suite on its short runs (f28, f29) and the build
phase's growth at its first builds (5.5); **at launch, before the builder starts and with no build running,** the main
session packs u4r-P3-0f066338dd8d by 8.1 and part-cleans the shared target (W1-TARGET item 3, `<<P27>>`), with D2's
readings before and after each; after the merge it part-cleans again (the sets build in the main folder's cache),
restates 5.5 before the first set, and runs the sets one at a time (-47, -22, -20, -21) with the floor projection, no
U4f set's own folder packed; (b) a fixed budget now from 5.5's estimates; (c) as (a) but -21's set split by suite into
four sets (forged, clamp, fast-clock, lag-switch), each with its twins in its own judge call, so each projection is
about a quarter; (d) as (a), and the advisor asks to widen R4 to each closed U4f set's own folder under §11's item 1,
which goes beyond the folders his PACK-READ yes covered, so KL-U4F goes to him under Decisions needed (section 12 item 6)
and nothing of it is packed before his yes in his own words. *Recommendation:* (a) if D2 shows the room after 5.5's
items 1 and 2 above the sets' total (every set's raw logs stay on disk until packed, so splitting alone does not lower
the total); else (d) with (c), each set's folder packed, after his yes, before the next starts; evidence is never
shrunk to fit (plan :940-941).

**U4F-Q3, option A's reading of `buffer_depth_ticks_count`.** *Status:* open, unchanged. *Question:* what do option A's
runs record as the input buffer's depth? *Why it matters:* the card's DEPTH ruling reads the header's
`buffer_depth_ticks_count` as the depth set (option B's pacing lead L) with the room's measure L - 1, while option A runs
commands on arrival, which ADR 0001:1166 counts as no buffer; -20 reports the depth (marks.json:4769-4785) and -48's
bite reads it under option B (plan :467). *Options:* (a) under A, `second.s.buffer_depth_count` 0 per client per second
and the header param 0; (b) under A, the controller's lead as the depth set and the measured lead below it, as B's; (c)
under A, no `buffer_depth_ticks_count` (it is -30's param: SERIES.md:240), `second.s.buffer_depth_count` 0, and the
ticks each command ran before its stamp reported beside it. *Recommendation:* (c); if it reads ADR 0001:1610's "the input
buffer depth the server settles on", it joins the readings PR.

**U4F-Q4, the logs' keying for a replaced session, and the -47 suite's rejoins (NC-U4R-5, EA-U4R-7, NA-16).** *Status:*
open; its why gains carry 75's bound. *Question:* does U4f key the lab's logs by session, and how do -47's 1,000 connects
rejoin? *Why it matters:* r-r1 :78-81 and :200-204 send the logs half to the first unit whose runs log a replacement; no
-20, -21 or -22 suite replaces a session (the never-acker is kicked, not replaced), but -47's 1,000 connects rejoin 48
slots about 21 times each per run, carry 62 (d)'s head rule allows no later admission before -19's join design
(C-U6-11), and carry 75's bound (scenario.rs:70, :995-1006) refuses an admission once the room has stamped 32,768
events past its start, about 30 s of SS's fighting (carries :357-365) against the suite's 52 s or more. *Options:* (a)
U4f keys `ended` and `gone` by session and resets the slot in `Logs::admit`, with a design for a later join's ticks;
the session key is a lab counter, never the join token or bind value (catalogue :120-121; rev 3.1, SA3-6 (3)); (b)
-47's suite writes no per-tick client hash records (its logs hold its connect events and the room's), so no slot files a
second session's hashes, and NC-U4R-5 stays for U6; its clients run an open-loop script that never fires (an npt-bots
behaviour: 2.13), so the room stamps no events and carry 75's bound is never met; (c) -47 measures 1,000 fresh ClockSync
starts on live sessions, no rejoin (a reading of -47's "connects"). *Recommendation:* (b): -47 reads no hash and no
event, (a) is the join design U6 owns, and f29 shows no admission refused.

**U4F-Q5, the U4d-F ruling's TE2-5 stop clause.** *Status:* open; one precedent since: the merge-stop ruling applied
TE2-5's pattern (named, no retry, the advisor's ruling) after the rule ended (merge-stop, process notes). *Question:*
does the stop clause (df-ruling :465-468, `<<P25>>`) still go into U4f's brief now that TE2-5's rule ended at U4r's
merge (TE-Q2, te-m3 :119-126)? *Why it matters:* the clause records expected failures "under TE2-5", a rule no longer in
force; without it, a builder's failing step has only TE-Q2's after-clause (`<<P30>>`). *Options:* (a) keep the clause as
the builder's step-group stop, its "TE2-5" read as the notes' failure record; (b) drop it with the rule; (c) the advisor
rewords it. *Recommendation:* (a): it costs nothing, keeps a failure from being retried unnamed, and matches the
merge-stop ruling's pattern.

**U4F-Q6, ClockSync's server-time stamp (NA-4, SC-1).** *Status:* open; option (e) new. *Question:* what server time
does a ClockSync sample carry? *Why it matters:* the room's `Pong` carries only the next tick to run
(pump.rs:458-467; scenario.rs:476), answered in the instant it arrives (hold about 0: plan.md:619-623), so each sample's
server instant is known only to 33.3 ms. The client pings at its steps, which fall at whole tick periods of its host
clock (client/src/lib.rs:485-488), so on L1 one connect's samples meet the tick grid at one phase, moved only by the
jitter (P2: ±10 ms each way): the error leaves a per-connect bias, up to about ±(33.3 - 20) / 2 ≈ ±6.7 ms where the
jitter window falls inside one tick period (estimate), against -47's p99 of 5 ms (marks.json:9104-9111). A filter tuned
past it might also mask the tick-quantised bite. *Options:* (a) the Pong as built (D4 shows whether any estimator passes
at one phase per connect); (b) the Pong gains a sub-tick server time (for example µs since its tick's due instant, 2 B,
inside the Pong's 32 B limit, catalogue :92): a wire change with the catalogue's row and the codec's tests (+4 tests,
one a negative case: the `Pong` travels both ways, pump.rs:461-466 and client/src/lib.rs:473-478, so the room ignores
the field in a client's `Pong`, or refuses it as an unknown field, never reads it: rev 3.1, SA3-6 (4); the lane opens
crates/proto and the catalogue); (c) snapshot arrivals (each leaves at its tick's due instant on L1,
plan.md:623) with the ping RTT's floor: exact on L1 only; (d) the room answers each `Ping` at its next tick's due
instant, with `hold_us` (pump.rs:460-465) from the ping's arrival to that instant: every ping gains up to one tick of
hold (subtracted, capped at one tick: pump.rs:510), the lane opens crates/server, and ADR 0001:247-248's "answers pings
on its I/O runtime as they arrive" needs a reading; (e) the room's own pings: each carries the server's clock in ns at
its send (pump.rs:394-401; catalogue :91's "8 B sender's time"), every 250 ms (PING_INTERVAL_NS, pump.rs:16), so its
arrival stamp less one estimated one-way delay (half the lowest RTT of the client's own pings) gives the server's time
with no quantisation and no wire or server change; but only 8 such samples come in 2 s, each carries one direction's
jitter uncancelled (±10 ms at P2), and ADR 0001:240's "from its own ping samples" needs a reading for the readings PR.
*Recommendation:* (b): it keeps ADR 0001:240's words, is exact off L1 too and needs no reading; D4 first, before the
advisor's check, so the advisor sees (a)'s and (e)'s figures under both phase models.

**U4F-Q7, the lead controller's form, `with_lead`, and the honest command rule.** *Status:* open; (iii) new (rev 3.1,
TE3-1). *Question:* how does the controller set a command's tick, does `with_lead` stay, and when does an honest client
give a command under the controller? *Why it matters:* U4R-Q14's lead is newest-relative and profile-set (r-check
:296-302); net F104 puts the controller on ClockSync's server-time estimate (u4-scout-netcode.md:122); in either form the
stamps rise strictly (2.2), and `header.params.lead_ticks_count` (SERIES.md:344) names one lead per run, which a
controller has not (TE-20). The bots' rule as built gives a command only when the newest tick moved (bots/src/lib.rs:14-21,
:224-227), which left about 20 % of P3's player-ticks with no command stamped for them (P3 runs.txt:18: 178,595 steps
without one beside 700,948 commands), against -20's 0.1 % missing; and U4R-Q14's lead 7 left 0.16 % late, against -20's
99.9 % on time. The other route, reading SERIES.md:63 to skip ticks no command was stamped for, changes what a bar's
series computes, a bar for him first (`<<P15>>`), and is not taken. *Options:* (a) newest + L, L from the client's own
RTT and jitter samples by U4R-Q14's arithmetic; (b) ClockSync's estimate of the server tick at arrival plus a jitter
margin and the buffer target (ADR 0001:250-251); for `with_lead`: (i) kept as a lab-only fixed setting (recorded as
`lead_ticks_count`) for U4r's suites and the -30 pre-check, (ii) removed; (iii) the honest command rule under the
controller: (c) contiguous stamps, a command for every stamp up to the controller's target, at most 2 a step, a hold
when the target is not above the last stamp (2.2), the old rule kept for `with_lead` runs only; (d) the old rule for
both. *Recommendation:* (b) with (i) and (c): the settled lead per client reported (a new series by U4F-Q1),
`lead_ticks_count` written only where (i) fixes the lead, and f36 shows -20's two shares on a P3 trace before any run.

**U4F-Q8, the lead bound's form and value, a carry to U4h (SA-11), and his yes.** *Status:* open; (iii) new. *Question:*
(i) is the lead bound a tick constant or a room setting, and what is it? (ii) does a carry go to U4h that the
input-buffer wait in §4.9's view-time estimate (plan :351-352) is capped by the honest lead, not by the lead bound? (iii)
does the value go to Mustafa for his yes before the build? *Why it matters:* the F2 ruling (advisor :463-465) names it
the first case of carry 18's route; ADR 0001:231 makes every limit data; the value must exceed every honest lead (P4's 9
by U4R-Q14's arithmetic; the controller's P3 lead may settle above U4R-Q14's 7: 2.2) and let the catch-up kind be built
under option B; under option B a client may stamp up to the bound ahead and §4.9's tolerance then adds that wait to the
view time it accepts; and the fast clock's two -21 checks allow "the server's ticks plus the lead bound"
(marks.json:5026-5049), so the value sets how loose they are, though it is no marks.json text: the first value of that
allowance is the kind of change his 13:35 rule (`<<P15>>`) brings to him (TE3-15's rewording). The allowances per
value: 400 ms = 12 ticks (up to 400 ms of rewind bought, 12 ticks of top-speed movement per episode); 600 ms = 18
ticks; 1,000 ms = 30 ticks; under option A, after an idle, the bound plus W more (U4F-Q18 (a); without (a), no bound
after an idle: SA3-1). Under option B the catch-up kind needs only more commands than the buffer has room for
(8 - (L - 1)), all inside the bound, so about 10 ticks suffice at P0's lead of 2 (input.rs:114-116).
*Options:* (a) `lead_bound_ticks_at(TICK_HZ)` in npt-sim by carry 18's route; (b) a room setting in ms turned into ticks
at the room by `ticks_for_ms_at` with the rate passed (carry 39's pattern), recorded in the header: 400, 600 or 1,000 ms;
for (ii): (c) yes, a carry to U4h; (d) no, U4h's brief decides; for (iii): (e) yes, the value and its allowance to him
in c62hb-paste's form (5.6, B3); (f) no, the advisor's ruling sets it. *Recommendation:* (b) at 400 ms (12 ticks), (c)
and (e): no new constant, the smallest round value above P4's lead with a margin and above the catch-up kind's need,
and a bar's allowance he has not seen goes to him first.

**U4F-Q9, the check order, per-kind attribution and the sequence check (TE-1 = SA-2, TE-2, SA-9, NA-6, NA-7, NA-13).**
*Status:* open, unchanged. *Question:* in what order do the room's checks run, how is each refusal counted under one
kind, and how is a forged backward sequence told from an honest repeat, inside and outside the kept window? *Why it
matters:* as built the sequence compare runs first and returns `Repeat` for every honest redundant copy (input.rs:81-84;
client/src/lib.rs:442-443): a tick check placed before it would count thousands of honest copies a run as forged kind 2;
a backward sequence is counted with the honest copies (NA-6), so kind 3 cannot be "rejected and counted"
(marks.json:4958-4969); under option B an old tick, or a stamp equal to a buffered one, is `late` like an honest late
command (NA-7, NA-13); and an overtaken honest datagram at P3 carries sequences older than a window of 8 (SA-9).
*Options:* (a) 2.4's rule, with K = `REDUNDANCY_CAP` plus a stated reorder allowance; (b) kind 3 as a sequence more than
the redundancy window behind the newest, refused and counted, a backward one inside the window dropped as a repeat; (c)
any refusal counts for any kind. *Recommendation:* (a) with K = 16 (8 + 8): (b) counts honest overtaken copies at P3 as
forged and lets a forgery inside the window go uncounted; (c) hides NA-7. The outside-window counter is reported, never
judged: a command dropped there never runs.

**U4F-Q10, each bite run's option.** *Status:* open; every bite run is judged under 5.4's rules. *Question:* under
which option does each bite run whose option the plan does not name? *Why it matters:* plan :462-463 names only
not-older-than (fast clock, A) and no-catchup-cap (forged, A); marks.json:5144 puts the no-* bites in the forged suite
and refuse-not-clamp in the clamp suite, either option; each bite run also needs `pre:PM-0001-2` to pass (README
:323-324). *Options:* (a) -21's other bites under option A, -20's redundancy-1 under option B; (b) all under B; (c) all
under both (13 more runs). *Recommendation:* (a): under option A with no repeats about 5 % of P3's commands never run,
so the shots fall toward PM-0001-2's floor (about 700 x 0.95 = 665: plan :437-445) and the bite run risks INVALID, while
under B the last command's repeat keeps the shots and the 99.9 % check fails (about 95 %).

**U4F-Q11, the suites' behaviours, profiles, references and the fast clock's stamps.** *Status:* open; (iii) (a) is now
known to be a changed bar (5.6, B5). *Question:* (i) which behaviour gives -20 its 10,000 scripted presses; (ii) which
profile do the forged, clamp, fast-clock and never-ack suites use; (iii) what is the fast clock's "honest client on the
same inputs"; (iv) does 5.3 also re-run -21's four option-B suites with the real cheats; (v) how does the fast-clock
client stamp its commands? *Why it matters:* the brawl makes no press edges (bots/src/lib.rs:279-281; an open-loop
`Presses` kind exists, :85); U4r's stand-ins ran forged, clamp and fast-clock at P0 (c-u4r-r2.mjs:82-84); SERIES.md:156
compares with an honest movement it does not define, and ADR 0001:1611 says "an honest client on the same inputs": a
reference from the bound's arithmetic is looser unless every input moves at full speed (TE-8), and rewording the row
changes what the check computes (a bar for him: 5.6); the stand-ins differ from the cheats (NA-8); the not-older-than
bite fails -21's fast-clock check only if the fast client sends equal stamps (marks.json:5133-5135) (TE-5, SA-6).
*Options:* (i) (a) the brawl with a fire press every second command, (b) the open-loop presses script; (ii) (a) P0, (b)
P2; (iii) (a) the bound's arithmetic, a reworded row, (b) an honest client in the same run on the same open-loop inputs
(a second soldier, which collides differently), (c) (a)'s arithmetic with the fast clock's inputs fixed at full move
length on a path the builder shows collision-free, where a unit test shows (a) and (b) equal (+1 test), so the row's
words hold as written; (iv) (a) yes, four more short runs (p15-p18), (b) no; (v) (a) the honest controller on the 1.5x
host clock, (b) the cheat's own rule (2.6: each stamp one tick newer than its previous, held at its newest tick plus the
lead bound, every 10th command one tick past it), its stamps never decreasing: a stamp is the larger of the previous
stamp and min(previous + 1, newest + the bound), and every 10th the previous + 1, so a command one tick past the bound
is followed only by stamps at or above it and every `Input` encodes (rev 3.1, TE3-4: a held stamp below a past-bound
one would be a tick step of -1, proto/src/input.rs:158-159). *Recommendation:* (i) (a), the real load r2's P3 suite showed; (ii)
(a), with no loss to confound the twins and no clamp reruns (SA-10); (iii) (c), no text changed once its test holds,
else (b); (iv) (a); (v) (b), with p9's short bitten run (D12) before the set.

**U4F-Q12, the event cap's value.** *Status:* open; NA-3's proof filled (D5). *Question:* how is plan §4.7's cap set
from 2.8's measurement? *Why it matters:* the formula (plan :335) gives about 1,008 at most for EVQ-LOAD's rate at a
margin up to about 0.73 s (NA-3), already overflowed at P3 (`<<P32>>`), and carry 50 caps it from above for -22 at about
3,900 (NA-12). *Options:* (a) the formula as written; (b) the largest per-client high-water mark seen on P2, P3 and B1
times a stated margin, checked below the never-acker's rate x 255 / 30 s; (c) a fixed 2,048. *Recommendation:* (b), its
figures to the advisor before -22's runs (carry 57), plan §4.7's text amended to state the measure and both bounds (the
main session's commit). The cap is no bar (5.6, B4).

**U4F-Q13, a refused command and the room's acknowledgement; the twins (SA-3).** *Status:* open; (b)'s construction
made encodable at rev 3.1 (TE3-2, TE3-4, SA3-2), one variant rejected (below).
*Question:* may a command the room refuses advance its sequence reference and the snapshot header's "newest received"
acknowledgement, and how is each kind's twin built to match? *Why it matters:* today `offer` sets the reference before
its press, late and full refusals (input.rs:89, :90-116), the header sends it (room.rs:1322), and the client then drops
every unacknowledged command at or below it (client/src/lib.rs:678-687), so a refused command is never resent; ADR
0001:228-229 acknowledges "the newest one the server received". A twin that still made the refused commands would run
them where the forged run ran nothing. *Options:* (a) only a command the buffer accepted moves the reference and the
acknowledgement: a refused command is resent and refused again, counted once per (slot, seq) (+1 test); the twin leaves
the command and every copy out; a reading of ADR 0001:228-229's "received"; (b) today's rule kept: the twin's endpoint
never makes a command the forged run's room refused whole (its sequence index and stamp used, nothing sent for them, a
lab-side endpoint setting: 2.13), so the forged run's sequence numbers, which the state hash reads (each Shot and Hit
event and each projectile carries its command's sequence: sim/src/world.rs:1979, :1995-2009), stay the same in both
runs; each later `Input` leaves out any command older than a skipped index, so its commands stay consecutive
(proto/src/input.rs:19-21, :155-157), counted, 0 at P0 (2.6, 2.12); the catch-up kind's twin sends the accepted
commands and then repeats only (2.12); kind 3's twin sends the honest copy; no reading. Rejected at rev 3.1: SA3-2's
variant, a twin with no command at that step and its sequences renumbered to stay consecutive: every later shot of
the forger would carry a sequence one lower and the hashes would part at the first one. *Recommendation:* (b): no wire
or rule change, one refusal per command by construction; f16 shows every kind's twin under both options, every
`Input` encoding and none left out at P0, before the judged sets.

**U4F-Q14, the refusal counters in the logs; U4E-Q11's size and bucket checks; the hold clamp (SA-5).** *Status:*
open; (c) is now known to be new bars (5.6, B6). *Question:* which refusal counters do U4f's runs log, how do U4E-Q11's
"judged runs" read for the size and bucket checks, and does the hold-time clamp gain a counter? *Why it matters:* today
none of the pump's counters reaches a log and the room's only partly (scenario.rs:1259-1271); plan.md:511-515 gives
every check a -21 counter, the hold-time cap included, but the clamp at pump.rs:510 has none; no forged kind of the 11
is the size's or the bucket's (plan :385-388). *Options:* (a) 2.5's per-session events, the size and bucket counters
reported in the judged -21 runs (0 expected on honest clients), and a hold-clamp counter in npt-server (the lane opens
it; section 6 item 4 applies); (b) as (a) but the hold clamp stays uncounted, with why: on L1 the RTT comes from the link
(pump.rs:339-344), so it bears on no L1 mark, and its counter becomes a follow-up for the first unit that runs `ws`
sessions; (c) a forged size and a forged rate behaviour, each with a check: new bars, to him first in c62hb-paste's form
(`<<P15>>`). *Recommendation:* (b): everything reported, no npt-server change, no bar; U4f's report names plan.md:515's
hold-clamp counter as not met, with why.

**U4F-Q15, the lag switch's check at 200 ms (TE-7 = SA-7).** *Status:* open; (b) is now known to be a changed bar (5.6,
B7). *Question:* how does U4f treat the lag switch's check, which cannot fail at 200 ms? *Why it matters:* at one
command per moved newest tick a 200 ms hold carries about 6 new commands, under `CMD_CATCHUP` = 8 (input.rs:61), so
`tick.v.lagswitch_excess_move_um` (SERIES.md:34) stays at or below 0 with no catch-up cap at all, and the lag-switch
parts have no bite (marks.json:5144); the 200 ms is ADR 0001:1611's "(proposed)". *Options:* (a) the judged parts run as
marks.json has them, and U4f's report states the arithmetic and p13's outcome beside them, as a check that cannot fail
at 200 ms; (b) the hold lengthened to at least 9 commands (300 ms or more) and no-catchup-cap's bite run in the
lag-switch suite too (+1 bite run): a reading of ADR 0001:1611 and a changed bar, to him first in c62hb-paste's form
(`<<P15>>`); (c) as (a), plus an unjudged 400 ms hold (about 12 commands) reported beside the judged parts (p19).
*Recommendation:* (c): nothing loosened or shrunk, the cap shown acting on a hold that can exceed it, no bar changed.

**U4F-Q16, carry 80 (b)'s bound on the newest tick (TE-11).** *Status:* open; its "open with Mustafa" context answered
(C3, 2.9). *Question:* how far may a refused snapshot datagram's header move the client's newest tick ("without moving
its newest tick past a bound": carries :520-522)? *Why it matters:* today the refused datagram's header tick moves the
newest tick to any value and resolves the whole span lost (client/src/lib.rs:508-522 into :631-641 and :583-615), a
span C62H-BOUND's first figure would read as unexplained lost ticks (2.9); the carry asks a bound and a test but names no
value, and f24 needs one to fail. This bound is how far the newest tick may move; C62H-BOUND's built checks, frozen
under his yes, are untouched by any option here. *Options:* (a) a refused header never moves the newest tick: its
sentinel is filed as a bad header's is (no ring slot: :556-565) and counted; (b) it moves the newest tick by at most
`ACK_RING_TICKS` (32 at 30 Hz: sim/src/ticks.rs:58-70); a header further ahead is dropped and counted in `refused`, with
no sentinel; (c) at most the ticks a burst can skip (B1's 1 s: `ticks_for_ms_at` with the rate passed).
*Recommendation:* (b): an honest refused header lies within the ring of the newest tick (B1's 1 s gap is 30 ticks at 30
Hz), the bound is an existing rate-derived constant (carry 39), and no honest run's filing changes (2.9).

**U4F-Q17, carry 80 (a)'s route (new: the night card's Next step 4).** *Status:* new. *Question:* before option A runs
several commands of one slot in a tick, which of carry 80 (a)'s two routes does U4f take (`<<P1>>`, carries :515-520:
the lead bound refuses such a command first, or the sort takes a total key)? *Why it matters:* the comparator at
room.rs:367 orders one slot's ticks wrap-aware, no total order for ticks 2^31 or more apart, and the carry rests on Rust's
sorts possibly panicking on a non-total order (D14 shows it on the head's toolchain, 1.99.1: rust-toolchain.toml:3); a
panic in the room's tick stops the room for every client and makes a run INVALID (README :250-251). Rev 2 took the total
key as a requirement; the carry lets the advisor choose. *Options:* (a) the total key (2.3: the signed distance from the
tick being run), with the lead bound also built as kind 1; (b) the lead-bound route alone: kind 1 refuses a far-ahead
command before it is buffered, and `canonical` is unchanged; (c) both, the total key and a debug assertion that no slot's
ticks span 2^31. *Recommendation:* (a): the bound alone leaves SA-1's triple (ticks T-2^31+1, T-2^30 and T+1, none
2^31 from T) reachable from a fresh slot with no last run to refuse against, and reachable in the no-lead-bound bite run;
the total key costs one comparator line and f8's three tests. Rev 3.1: U4F-Q18's join anchor closes the fresh-slot
route, and with its (a) every slot's ticks lie within W + the bound of the tick being run, so (b) would hold outside the
bite run; the bite run's far-ahead stamps, strictly rising, can still span 2^31, so (a) stays the recommendation.

**U4F-Q18, option A's lower bound: the join anchor and a past window (new at rev 3.1: SA3-1).** *Status:* new.
*Question:* under option A, what ties a command's stamp to the room's tick from below? *Why it matters:* option A runs a
strictly newer command whenever it arrives, up to 8 a tick (2.3), and kind 1 bounds a stamp from above only; option B's
only lower anchor is `ran_through` (input.rs:23, :107-113), which every tick's `take` sets for every joined slot
(room.rs:841-846), and option A has none. A client whose first stamp is far in the past (the join's tick - 100,000, or
- 2^31 + 1) passes kind 1 (not ahead) and kind 2 (a fresh slot has no reference), then sends two datagrams of 8 a tick
(the bucket: catalogue :87), each stamp one above the last: 8 commands run a tick, each one tick of movement, weapon
clock and reload (sim/src/world.rs:1333-1345, :1389-1395), for as long as the bank lasts. A client that joined honestly
gets the same after idling: 60 s of nothing, then 8 a tick for about 257 ticks. ADR 0001:231-233's claim that a fast
clock never runs more commands than the server has ticked plus the lead bound holds only from an anchored stamp, and
-21's cheats never build a bank (the fast clock starts at its newest tick plus the lead: U4F-Q11 (v); the lag switch
holds 200 ms), so the judged checks would pass a build with the hole. *Options:* (a) both: the join anchor under both
options (a fresh slot's kind-2 reference is the tick its `JoinAccepted` names, room.rs:681: 2.3, 2.4 item 4; f13 (iii))
and, under option A, a past window W = `CMD_CATCHUP` (8; carry 27's constant, a count of commands, which at one stamp
per tick is a count of ticks, with no rate in it), a command stamped more than W ticks before the tick it would run at
dropped and counted `late` (not forged; it never runs): the bank is then at most W, an honest client's resent commands
after a burst (at most `REDUNDANCY_CAP` = 8, the newest about the lead ahead: plan :439-441) fit, and so do the catch-up
kind's 16 (2.6); f38 and p20 show it; (b) the anchor only, the idle bank written up as option A's speed-hack exposure
for OQ-0001-6 and ADR 0001:231-233 read as "from join" (the readings PR: 9.1); (c) neither (rev 3). *Recommendation:*
(a): it closes both banks with no new constant and no wire change; under it the fast-clock allowance after an idle is
the bound plus W, which Q8 (iii)'s draft for him states (section 12 item 6); carry 39's tick-name rule (check.mjs's
`TICK_NAMED`, :760) reads W as `CMD_CATCHUP` itself, so no new `*_TICKS` constant is made.

---

## 11. Needs a run (day)

Each item: what is known now (its source), or the command and pass bar left for the main session, and when: **[before
the advisor]** (it decides a question), **[at launch]**, **[after the build]** (the builder's) or **[after the merge]**
(the main session's, 5.3). Commands run from the main folder `<M>` = /home/user/Shamble-Frontlines, one per call by
`<<P17>>`, through main-run.sh or the lane runner as named.

| # | Claim | Now | Left: command and pass bar | When |
|---|---|---|---|---|
| D1 | `<base>`'s counts | **filled:** at 0ad9661, U3C-HELD-1's counted run E6 through main-run.sh (held-build; the card): check.mjs all 11 steps pass; test 552 measured, 632 bites build, 0 failed, 0 skipped; refs 1382; marks 81; self 78 of 78; redact 10 of 10, 306 items, 0 findings; deps 137 crate files; wasm 27000 ticks, 0 mismatching both ways; free 5,514 MiB after it. The head's spikes/netcode-proof is 0ad9661's (0.1) | they are `<base>`'s if `git diff --quiet 0ad9661 <base> -- spikes/netcode-proof` exits 0. A new kept-logs list under results/ breaks that exit 0 while the code holds: then `git diff --quiet 0ad9661 <base> -- spikes/netcode-proof ':(exclude)spikes/netcode-proof/results'` exiting 0, with every other change a raw-logs.sha256 file, keeps every count but refs and redact's items (each +1 per new list: read them from a light run, `node check.mjs --only refs,marks,self,redact`, as MAIN-54's C4); else a counted run: `bash <M>/.checkpoints/netcode-proof-test/main-run.sh <M>/.gate/u4f-launch/base-counted.log node check.mjs`, reading the test step's two counts, self's "K of K" and the step count | at launch |
| D2 | the disk the sets need against what is free (NA-5; 5.5) | last reading 5,514 MiB free (E6, 12:34 UTC 2026-11-17); no reading since | `df -BM /home/user`, then `du -sm` of each folder under /home/user/Shamble-Frontlines/.gate/netcode-proof-test/runs/, of .gate/shared-target and of .gate/netcode-proof-test/target, each in its own call; 5.5 redone in MiB with the packs and part-cleans it plans, net of the archives (rev 3.1: about 73 % of a folder comes back); from it, whether KL-U4F goes to him (5.5 item 3) | before the advisor; at launch, before and after 5.5 item 1; after item 2 |
| D3 | L1 P3's reorder share is above 0, so -20's runs meet the `reorder` rule | not in any committed record (`git grep -i reorder` under results/u4r/ finds none) | a Grep-tool search (rev 3.1, TE3-12 = SA3-6 (2): no shell grep): pattern `"reorder_permille":[0-9]+`, path /home/user/Shamble-Frontlines/.gate/netcode-proof-test/runs/u4r-P3-0f066338dd8d, output mode content with only the matches (`-o`), the values read by hand: above 0 on every run | before u4r-P3-0f066338dd8d is packed (5.5 item 1, at launch), else from -20's own first run |
| D4 | NA-4 under both phase models (SC-1), U4F-Q6 (e)'s room-ping estimate, and the tick-quantised bite's reach (TE-6) | no script exists; nothing run | the main session's model, parameters fixed here: 1,000 connects x 3 seeds; per direction a one-way delay of 50 ms + U[-10, 10] ms (P2: l1/src/lib.rs:186-188, :214-221); 20 client samples in 2 s, one ping every 3 client steps; each sample's server stamp the next 33.333 ms tick; phase (i) uniform per sample, (ii) one seeded phase per connect, each send a whole number of tick periods from it (client/src/lib.rs:485-488); arrivals (1) as built, (2) bitten: each moved to the next client step; estimators: the mean of samples, the lowest-RTT, and (e) 8 room pings (every 250 ms, exact server ns) less half the lowest own-ping RTT; report the p99 of abs(error) for each, against 5 ms (marks.json:9104-9111). A read-only script under the notes folder, its output saved | **before the advisor** (U4F-Q6) |
| D5 | NA-3: BRAWL-1's P3 load exceeds plan §4.7's formula | **filled:** `<<P32>>` records 27 forced resyncs on 25 clients in s1 at the provisional 1,024 (rate 554-561 a second), above the formula's about 1,008; no log writes the queue's length (no npt-lab writer of `second.s.event_queue_count`: `git grep` of crates/lab/src), so per-burst high-water marks exist only from 2.8's new runs (D8) | none | done |
| D6 | every cite of this brief still points where it says (section 8) | **filled:** walked at the head (section 8; 0.4) | at launch: `git diff --stat 7cfae88 <base> -- spikes/netcode-proof docs/adr/0001-multiplayer-netcode.md docs/design/netcode-proof-messages.md .claude/rules/netcode.md .checkpoints/netcode-proof-test/u4-build-plan.md .checkpoints/netcode-proof-test/plan.md .checkpoints/netcode-proof-test/u4s-carries.md`; if it prints anything, each moved file's rows re-walked (section 12 item 10) | at launch |
| D7 | each U4f suite's ticks per second and bytes per 10 minutes, under both options; ClockSync's p99 at P2; the controller's settled lead per profile | - | the builder's D-steps, each command line in its notes (as U4r's D4 and D13: N/u4r-brief.md:1792-1841) | after the build |
| D8 | the event cap's value (2.8) | - | 2.8 item 3's nine short runs under the runner (`-x`), commands in the builder's notes; per profile and seed the rate and the high-water marks per burst and per 30 s bin | after the build |
| D9 | `cargo test -p npt-server --all-features` compiles (only if a ruling opens npt-server) | **premise filled:** it still stops at E0425: server/Cargo.toml and tests/loopback.rs are unchanged since 2fbec77 (`git diff --stat 2fbec77 <head>` on them prints nothing; r-b2 :26-27) | `bash <u4f-run.sh> -x <worktree> <M>/.gate/u4f/server-all-features.log cargo test --locked -p npt-server --all-features`: exit 0 and the runner's summary line | after the build, only by the ruling |
| D10 | marks.json and the judge unchanged at U4f's close (5.6) | - | `git diff --stat <base>..<head> -- spikes/netcode-proof/marks/marks.json spikes/netcode-proof/crates/judge`: empty, or the read owed (5.6) | at the close |
| D11 | a new suite's honest streams are the same run to run, before its twin pairs (TE-24) | - | 5.3's p1 and p14 (forged, option A, seed 1, twice) through c-u4f-r2.mjs, then the two runs' `fate_log_sha256` (scenario.rs:1242, :1276) and per-tick hash series compared: equal | after the merge |
| D12 | each suite's bite fails its check when run (TE-5, TE-6, SA-6) | - | 5.3's p7-p12 with their clean partners (p1, p2, p3, p21, p5 or p6), each judged in 5.4's form (`--only PM-0001-2,<mark>` for -20 and -21; `--only PM-0001-47`): on each bitten run the check 5.3's table names fails, read by name from the verdict's checks (never from the bite's status: judge/src/eval.rs:1475-1478), and passes on its clean partner; every check but 5.3's length-failing list passes on both; each bar in c-u4f-r2.mjs's `expect` line (rev 3.1, TE3-3) | after the merge |
| D13 | the lag switch's check cannot fail at 200 ms (TE-7, SA-7) | - | 5.3's p13 (lag switch, option A, P1, bite PM-0001-21/no-catchup-cap): `tick.v.lagswitch_cmds_count`'s max (at most 7 expected) and whether `tick.v.lagswitch_excess_move_um` stays at or below 0 | after the merge |
| D14 | whether today's `canonical` panics or only misorders on SA-1's triple (toolchain 1.99.1) | not read (no std source on the machine was found to read) | f8's triple case against the old comparator as a plant: `bash <u4f-run.sh> -x -k <worktree> <M>/.gate/u4f/d14.log cargo test --locked -p npt-room --all-features <f8's name>`, then restored and its sha256 checked; the outcome (panic or order) recorded either way | after the build |
| D15 | carry 74's late share under both readings (TE-3, TE-4) | - | 5.3's p5 and p6 (-20 P3, A and B), each reporting the room's `late` count against the commands sent and 1 - `tick.s.cmd_on_time`'s share: at most 0.1 % in both, and -20's two share checks passing by name (rev 3.1, TE3-1: at P3 the bar the controller serves is -20's, carry 74's 1 % on the other runs); a miss goes to the advisor before -20's set | after the merge |

---

## 12. Left for the main session at launch (on N/u4r-brief.md §12's model)

1. **`<base>` and its counts:** the spike branch's head at launch; its counts by D1 (the known numbers hold under D1's
   rule, else the counted run); filled in section 1 item 1 and section 4 rule 5.
2. **The runner `u4f-run.sh` (`<<PV1>>`):** made from u4r-run.sh (e9437ae2…, 117 lines) by a script in the notes folder,
   only the lane's names changed (the header, the log folder .gate/u4f/, the usage line), its .diff beside it holding
   only those; its sha256 and `bash -n` exit 0; `<<PV1>>` filled in section 1 item 4.
3. **The placeholders, by script, each checked after the copy** (N/u4r-brief.md:1857-1861's rules: the anchor found
   once, the copy's first and last words as named, the placeholder gone from its every place in the brief, no other
   byte changed; a span that wraps is copied with each line break and the next line's indent read as one space and
   rewrapped to 120; the assert compares copy and source with every run of white space read as one space; a closed
   file's sha256 asserted equal to 0.2's stamp first). At the head (stamps_anchors.py) every start anchor below is found
   once:
   - `<<P1>>` carries: from the line "## U4f (clock and commands)" (:507) to the line before "## U4g (prediction,
     interpolation and events)" (:540); the copy's entry numbers at line start exactly 46, 80 and 86.
   - `<<P2>>` carries :9-10: from "Each entry: the finding (review, id), then" to "never the rule.".
   - `<<P3>>` to `<<P13>>` carries, each from its line beginning to the line before the next line that begins with an
     entry number or "## ": P3 "57. **U4E-Q6, the room's half" (:223-228); P4 "74. **NC-M6, the endpoint's lead"
     (:345-356); P5 "21. **netcode F2 (the room's half):**" (:148-149); P6 "27. **this pass (CMD_CATCHUP):**"
     (:166-168); P7 "18. **this pass (the tick grep):**" (:129-132); P8 "39. **rust fix review nit
     (u4s-fix-review-rust.md:114-115" (:169-179); P9 "50. **the event age" (:180-183); P10 "61. **U4j's hash rules, the
     room and the client" (:229-247); P11 "58. **U4E-Q6's two conditions" (:269-273); P12 "76. **A6 and SEC-U4E-5,
     ending a session" (:366-373); P13 "77. **U4r's readings of U4e's room and client" (:374-383); each copy's entry
     number asserted.
   - history (living, by text): `<<P14>>` the line beginning "  - Builders: pass or fail comes from the exit code" and
     its one continuation line (:292-293; the line before it asserted "  USAGE-SAVINGS: (a), all three, with these
     limits."); `<<P15>>` from "A new bar, or a change to one, comes to me under Decisions needed" to "on the ruling
     alone." (:2125); `<<P16>>` the line beginning "  From now on: every change to marks.json or to the judge" (:2127);
     `<<P17>>` from "write commands in the simple forms that need no approval" to "before you send it." (:2187);
     `<<P18>>` from "  2. SIZE-CHECK from now on." to "as the rule says." (:2048-2051); `<<P19>>` the line "  C62H-KEEP:
     (a). The check stays as built." (:2141); `<<P20>>` from "  C62H-BOUND: (a), the exact limit." to "stay in until it
     is built." (:2143-2146); `<<P21>>` the line "  - The stop rules of your post stand. The check is never reworded to
     fit." (:2175); `<<P22>>` the line beginning "      - (a) Yes. I may pack one or two closed" (:2212: the
     recommended option he agreed to, RECS-1043, quoted as the session's text he answered, never as his words);
     `<<P23>>` the line beginning "  - **C62HB-Q4:** should the counter-wrap test (PM-0001-58)" (:2216: the same).
   - closed records: `<<P24>>` df-ruling :462-464, its start key `word for word: "Write files with the` (found once, at
     :462; the bare first words stand twice, :313 inside a quoted rule and :462: rev 3.1, TE3-9), the copy from that
     key's "Write files with the" to "whole with Write."; `<<P25>>`
     df-ruling :465-468, from "a failure in <crate>" to "and go on.", with the first `<crate>` replaced by "npt-room,
     npt-client, npt-bots or npt-lab (and each crate U4F-Q1 opens)" and "<crate>'s files" by "their files", the script
     asserting both and no `<crate>` left, only if U4F-Q5 keeps it; `<<P26>>` j-r2 :298, the words after "From now
     on: " to the line's end; `<<P27>>` m9 :252-256, from "Between waves, with no lane running" to "never `--target`
     without `-p`."; `<<P28>>` m9 :259-264, from "(1) never pass `--target`" to "no session removes it by hand.";
     `<<P29>>` d2-r2 :304-308, from "never run cargo" to "may use it as one."; `<<P30>>` te-m3 :127-128, from "After it
     ends, FX-10's saved output" to "says so." (NA-10); `<<P31>>` c62h :20, from "Until the fix lands, the rule is:" to
     "as both run scripts already do."; `<<P32>>` b1-p3 :158, the quoted line whole; `<<P33>>` r-b2 :28, from "Adds a
     card follow-up" to "No push for it."; `<<P34>>` r-r1 :122, from "Contract half:" to "a usage error (exit 2).".
   - u4r-report.md (living, by text): `<<P35>>` from the line beginning "- **TE-B1-R2-1 with RR2-B1-1**" to the line
     before "- **TE-B1-R2-2**" (:1138-1142); `<<P36>>` the line beginning "  - RR-B1-7 (1), the room_tick loop rewrite"
     (:1163).
   - `<<PV1>>` (item 2) and `<base>` (item 1): values, filled wherever they stand.
4. **The advisor's rulings on U4F-Q1 to Q17** (section 10), folded into the places they name (each "by U4F-Qn" read as
   the ruling's words), section 4's lane widened or narrowed to match, 2.10's conditional deltas and 5.3's conditional
   runs kept or struck; any plan text a ruling changes (plan §4.7's cap, §4.1's rows) in the main session's own commit,
   as U4r's rulings were (r-check :416-424).
5. **A constant, manifest or wire change a ruling puts before launch** (U4F-Q8 (a)'s carry 18 commit; U4F-Q6 (b) if the
   main session makes it): its own commit on committed state with its counted run (r-check :425-427).
6. **His yes on every bar text before it is built** (5.6): B3 by U4F-Q8 (iii)'s recommendation, and B5, B6 or B7 only if
   a ruling picks them; each to him in c62hb-paste's form under Decisions needed, with the exact text or value and the
   planned size; nothing of it built before his yes; each text's sha256 recorded at it. Draft entry for the main
   session's report: **LEAD-BOUND:** may U4f's room refuse a command stamped more than 400 ms (12 ticks at 30 Hz) ahead
   of the server, which also lets a 1.5x-clock client run at most 12 more commands and 12 ticks more of movement per 60 s
   than the server's ticks allow before -21's fast-clock checks fail (under option A, after the client idles, up to 8
   more again: the past window of U4F-Q18 (a), rev 3.1)? Why it matters: the value sets how loose those checks are and
   how much rewind a stamp-ahead buys under option B. Options: (a) yes, 400 ms; (b) 600 ms; (c) 1,000 ms; (d) another
   value. Recommendation: (a), the advisor's if U4F-Q8 rules so. Second draft (rev 3.1, SA3-5), **only if D2 shows the
   room short** after 5.5's items 1 and 2: **KL-U4F:** may the main session pack each of U4f's judged sets' own raw-log
   folders by the kept-logs rule, once that set's results are committed and its ruling closed, before the next U4f set
   starts? Why it matters: U4f's sets need about 3.6 to 3.7 GB of logs against about 1.8 GB above the 4 GB floor after
   the planned pack, and your PACK-READ yes covered one or two older folders. Options: (a) yes, each closed U4f set's own
   folder, never the P0 folders, the reference sets or the c-u2 folders; (b) no: the sets wait for room from elsewhere.
   Recommendation: (a). Nothing of it is packed before his yes in his own words.
7. **This brief's checks:** the test-engineer's and the security-auditor's checks of rev 3 and the revision, rev 3.1
   (this workflow: done), then the advisor's check with U4F-Q1 to Q18, after D2 and D4 (the items marked before the
   advisor).
8. **The review panel** (plan :816): netcode-architect, security-auditor (the hostile input: 2.4's kinds and order, the
   clamp points, the 1.5x clock, the lag switch, the never-acker and carry 76's Kick path, 2.5's counters),
   test-engineer, rust-reviewer; plus engine-architect on the public items (A19; plan :930-931; 2.13). Each at effort
   xhigh, at most three agents at a time, reviewers and the builder counted (advisor :147, condition 10 (c); plan
   :886-887): two batches. The test-engineer's review carries the 13:35 read if 5.6's test is not empty, and the
   recommended read of SERIES.md's new rows, each named in the report's own line.
9. **The judged runs' target:** the main folder's own cache, `CARGO_TARGET_DIR` unset (plan :918-919;
   c-u4r-runs.mjs:6-9), as U4r's; the disk by 5.5 and U4F-Q2's ruling.
10. **Re-stamp and re-check at launch:** re-stamp 0.2's sha256 of the plan, plan.md, carries and history; if one moved,
    re-check every cite of this brief into it (the carries gain entries at merges; history gains decisions at its end)
    and fold the new lines in the main session's commit; run D6's diff and re-walk section 8's rows for each file it
    names; re-run stamps_anchors.py and r31_check.py and confirm each anchor (P24 by its key) still found once.
11. **The two follow-ups of 9.1's last rows:** SA-16's variant to STATUS for U6's limits; SA-11's carry to U4h's section
    of carries if U4F-Q8 (ii) rules it.
12. **The -22 set report** (the main session's, after its judged runs): the load each -22 run carried (advisor
    condition 4, :116-117; TE-22; 5.2's report call), each run's `overflow_resyncs_count` per client (SA-16), and
    `rss_bytes` read as "not available" on L1 by design (NA-2).
13. **The disk at launch, before the builder starts** (rev 3.1, TE3-8, SA3-5; 5.5 items 1 and 2): D3 read first; then,
    with no build and no set running, pack u4r-P3-0f066338dd8d by 8.1's procedure and part-clean the shared target
    (`<<P27>>`), D2's readings before and after each on the card; the builder starts only after them. A U4f set's own
    folder waits for KL-U4F (item 6).

---

## Findings met while drafting and revising (NA-1 to NA-16; none is carried in carries)

| Id | Severity | Where (the head) | What goes wrong | Cheapest proof (day) |
|---|---|---|---|---|
| NA-4 | High (an estimate) | server/src/pump.rs:458-467; lab/src/scenario.rs:476; client/src/lib.rs:485-488 | the room's Pong gives only a tick, answered at arrival, so ClockSync's server stamps are quantised to 33.3 ms; and since the client pings at whole tick periods, on L1 one connect's samples share one phase (SC-1), so the quantisation leaves a per-connect bias of up to about ±6.7 ms at P2: -47's p99 likely lands above 5 ms, or a filter tuned past it hides the tick-quantised bite | D4, before the advisor |
| NA-14 | Medium (new) | SERIES.md:63; room/src/input.rs (option A's run path is new) | under option A commands run on arrival, so arrivals bunch; written as "nothing ran this tick", `tick.s.input_missing` would read many honest ticks as missing and fail -20's 0.1 % on a correct build; the row's words ("no command for this tick had arrived") read by stamp | f34 |
| NA-15 | Medium (new) | room/src/room.rs:689-695, :739; lab/src/scenario.rs:513-534, :544-555, :592-599 | a `Leave` routed to the room, or `Room::leave` on a closed link, sets the slot `None` whole, and the session's `Decoder` dies with it, so counters read after the end are gone: a per-session refusal record written then shows 0, and the forged suite's attribution of a departed client's refusals is lost | f35 |
| NA-16 | Medium (new) | lab/src/scenario.rs:70, :995-1006; carries :357-365 | -47's suite rejoins 48 slots about 21 times per run over 52 s or more, past carry 75's admission bound (about 30 s of SS's events): with a fighting behaviour the lab refuses the next admission and the run stops | f29 (U4F-Q4) |
| NA-3 | Medium | plan :335; `<<P32>>` | plan §4.7's cap formula gives at most about 1,008 for EVQ-LOAD's rate at any margin up to about 0.73 s, while 1,024 already overflowed at P3 without a burst: a formula cap resyncs clients in judged runs | D5 (filled) |
| NA-12 | Medium | plan :335; carries :180-183, :223-228; room/src/events.rs:19-20 | nothing bounds the cap from above, yet a never-acker's events expire once older than 255 ticks: a cap above about 3,900 is never reached and -22's headroom check (marks.json:5171-5181) fails a correct build | arithmetic; room/tests/room.rs:1444 at the chosen cap |
| NA-5 | Medium (an estimate; changed) | c-u4r-runs.mjs:133, :138-154, :225-229; the card (E6) | U4f's sets and pre-runs need about 3,596 to 3,667 MiB against 1,418 MiB above the floor at the last reading: packing by 8.1 and the shared target's part-cleans must free room first (5.5) | D2 |
| NA-6 | Medium | room/src/input.rs:80-89; room/src/room.rs:795 | a backward sequence is a counted `Repeat` like every honest copy: -21's kind 3 cannot be rejected and counted apart | f12 |
| NA-7 | Medium | room/src/input.rs:107-116 | under option B a forged old tick counts as `late`, and no lead bound exists: 8 far-ahead commands fill the buffer and later run at their stamps (pre-loaded input); the remedy needs TE-1's order (2.4) | f13 |
| NA-13 | Medium | room/src/input.rs:110 | under option B a command whose stamp equals a buffered command's is `Late`, so a forged equal-tick command with a new sequence is counted as an honest late command, never kind 2 | f13 (ii) |
| NA-8 | Medium | bots/src/lib.rs:74-76; sim/src/greybox.rs:62, :151 | U4r's clamp script aims 32,000 cm out, beyond the ±12,800 cm map: with the aim check its points are refused, failing "clamp: never refused or counted" (marks.json:4992-5003) | f18 |
| NA-9 | Medium | marks.json:5112-5115, :5136-5139; sim/src/world.rs:937-938 | no-cooldown's and refuse-not-clamp's sites are npt-sim's, outside U4f's row (plan :806-809): without a lane ruling they cannot bite | `git grep -n -e no-cooldown -e refuse-not-clamp <head> -- spikes/netcode-proof/crates` (none today) |
| NA-1 | Low | carries :537-539 | carry 86 has no quoted text, so a fill script cannot find where its rule begins: P1 copies the §U4f section whole | the fill's assert (12 item 3) |
| NA-2 | Low (changed) | SERIES.md:93; marks.json:5208-5216; judge/src/eval.rs:1504-1556 | `second.v.rss_bytes`'s writer is npt-server, which runs no process on L1; -22's report reads it from process `room` and prints "not available", never making the run INVALID: a report line with no L1 value, by design | none (read) |
| NA-10 | Low | te-m3 :126-128 | the night plan cites TE-Q2's after-clause at :126-127; it is :127-128 | P30's first-word assert |
| NA-11 | Low | plan :810; marks.json:9123, :9127 | the plan's bite words differ from marks.json's ids: a builder spelling the plan's words mints ids no bite entry matches | grep the ids in `npt_client::BITES` against marks.json at the merge |

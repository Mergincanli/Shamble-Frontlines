# U4f builder brief: clock and commands (rev 3, day draft)

rev 3, 2026-11-17, by the netcode-architect (workflow wf_e78b24a2-736, its draft step), from rev 2 (the night's:
`git show 34a2e55:.checkpoints/npt-u4f-brief/u4f-brief-draft.md`, every cite at 11fd190; rev 1 at 828e2f4) and the
record since. Read only, at the head of section 0. Next (the card's U4f line): the test-engineer's and the
security-auditor's checks, one revision (rev 3.1), then the advisor's check with the questions of section 10
(N/u4-build-plan.md:1054-1056, U4-BRIEFS (a)). Nothing here is ruled: every choice the plan or a ruling leaves open is a
U4F-Q question, and the brief follows its recommendation only where it says "by U4F-Qn". The form is U4r's brief, rev
3.2 as filled at launch (N/u4r-brief.md), with C62H-BOUND's brief (N/c62h-bound-brief.md) for the stops and the texts
for his yes. Findings NA-1 to NA-16 are after section 12.

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
| C24 | Recheck walked: 33 rows (24 hold, 9 changed), 12 rows added | the code at the head | 8 |
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
| Recheck rows (section 8) | 29 | 33 | **45** (33 walked: 24 hold, 9 changed; 12 added) |
| Open items (section 9) | 15 | 17 | **15** in 9.1 (3 answered, 1 closed, 2 new); **19** deferred Lows placed in 9.2 |
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
  print nothing). Its spikes/netcode-proof is 0ad9661's, U3C-HELD-1's fix: `git diff --stat 0ad9661 HEAD --
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
  P36 each found once (section 12 item 3 gives each span's lines). Values: `<base>` (the spike branch's head at
  launch) and `<<PV1>>` (u4f-run.sh's sha256).
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
   `/home/user/Shamble-Frontlines/.gate/u4f/`. A run's result is its exit code and the test runner's own summary
   line, never a grep (`<<P14>>`).
5. **What stops you** (each written in your notes and your report, then build the rest): a file outside section 4's
   lane; an item another unit merged changed beyond what this brief names; a manifest or Cargo.lock change (the main
   session's); a new external dependency; disk below section 4's floors; a part a U4F-Q leaves open at launch; a tick
   count or rate-derived constant you need (carry 39's rule, `<<P8>>`: record name, definition, values at 30 and 60 Hz
   and site, build the rest); a wire change beyond U4F-Q6's ruling (the catalogue and its tests move with it:
   .claude/rules/netcode.md:15); **any edit to marks.json, an existing SERIES.md row or crates/judge** (5.6: a bar
   comes to Mustafa first, `<<P15>>`); **the lab's short P2 C62H-BOUND test failing** on your build
   (`c62hb_p2_short_every_measured_lost_tick_is_explained`, lab/src/scenario.rs:2904: under the C62H-BOUND stop
   rules his yes adopted, `<<P21>>` with c62hb-paste :120-124, it stops the work for him); **your added lines past 6,000**
   (SIZE-CHECK, section 4).
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
| The filter | ADR 0001:240-242's rules (drift at most 1-2 ms per frame, halve errors over 100 ms, reset over 500 ms), over the lowest-RTT samples; every threshold in ns, data, pinned by a test | ADR 0001:239-242; net F117 |
| Ping rate and phase | data, at most the room's limit (10 a second per session: catalogue :91; the pump's bucket, pump.rs:458-471). The client sends a `Ping` only at a step, and its steps fall every 1/TICK_HZ of its host clock (client/src/lib.rs:485-488), so **every ping period is a whole number of ticks**: on L1, with an honest host rate, one connect's samples meet the server's tick grid at one phase, moved only by the delay jitter (P2: ±10 ms each way, l1/src/lib.rs:149-163, :214-221, against a 33.3 ms period). The filter may not count on random phases, and the builder may not move a ping off the step grid to get them (the endpoint reads no clock): SC-1 is met here and in D4's phase-locked model. `ping_limited` 0 on a clean run, read from 2.5's per-session counters | SC-1; net F117 |
| Bounds | outstanding pings and kept samples each capped (as the pump's `OUTSTANDING` 8 and `SAMPLES` 8: pump.rs:24, :26); a `Pong` naming none of its pings is dropped and counted (`unknown_pongs`, client/src/lib.rs:403-410, today crate-private: public only if the lab logs it) | .claude/rules/netcode.md:13-14 |
| Clock reads | none: the endpoint reads no clock (ADR 0001:245-247; client/src/lib.rs:5-6) | ADR 0001 |
| Carry 39 (`<<P8>>`) | the tick period in ns comes from the rate passed in (production callers pass `npt_sim::TICK_HZ`), pinned at 30 and 60 Hz (f31); no local tick constant | carries :169-179 |
| What -47 reads | npt-lab writes `event.connect_clock.offset_err_ns` (SERIES.md:149) per connect: abs(estimate - true) at the first client step at or after 2 s from the connect's first sample (the builder states this instant in its doc and report); the true offset is the lab's seeded host offset (lab/src/scenario.rs:18-19, :930; plan.md:626) | marks.json:9095-9111 |
| Bites | `PM-0001-47/newest-sample` and `PM-0001-47/tick-quantised`, in `npt_client::BITES` (client/src/lib.rs:28-32, growing to 6 with 2.11's -20 bite), through `ClientEndpoint::with_bite` (:314-326); the ids spelled as marks.json's (:9123, :9127), not as the plan's words at :810 (NA-11). A lowest-RTT filter keeps the samples the tick-quantised bite delays least, so its bite may not fail: D4 models it and 5.3 runs it short first (TE-6) | marks.json; plan §4.14 (:454-455) |

### 2.2 The client's tick-lead controller (npt-client; net F104 :122; carry 74; r-check U4R-Q14 :289-358)

| Field | What U4f builds | Source |
|---|---|---|
| What it replaces | the endpoint's fixed lead (`lead`, client/src/lib.rs:214, 1 by default :267; `with_lead` :296-301, whose doc names U4f's controller as its replacement; the lab sets it from `lead_for`, lab/src/scenario.rs:132-137, :900); whether `with_lead` stays as a lab setting is U4F-Q7 | carry 74 (`<<P4>>`) |
| What it sets | each command's stamped tick (today `newest + lead`, :429) | ADR 0001:250-251 |
| Its form | U4F-Q7: (a) the newest snapshot tick plus a lead from the client's own RTT and jitter samples, by U4R-Q14's arithmetic (L = ceil((rtt + jitter) / P) + 1, r-check :296-302); or (b) ClockSync's estimate of the server's tick at the command's arrival plus a margin and the buffer target (net F104 :122) | U4F-Q7 |
| Stamps rise strictly | each stamp is strictly above the previous command's (the room refuses any other as kind 2: ADR 0001:231-233); the controller lowers its lead only by holding (no command at that step), never by stamping back; a ClockSync reset moves the estimate, never a stamp backwards (TE-15, SA-8; f5, f32) | ADR 0001:231-233 |
| Same under both options | one controller under option A and option B (net F104); the endpoint never reads `cmd_timing` (README :191-192 makes it the room's and the scenario's); the lab builds both options' endpoints with one controller setting (f6) | net F104 |
| Inputs | ClockSync's samples only (the client's own measure), never the profile (the lab's knowledge: `lead_for` is U4r's stand-in) | ADR 0001 part 1 |
| Bounds | at least 1; at most the room's lead bound less a margin (2.4 kind 1); the endpoint keeps at most `REDUNDANCY_CAP` = 8 unacknowledged commands (client/src/lib.rs:442-443) | carry 27 (`<<P6>>`); ADR 0001:231-233 |
| Pass marks it serves | -20's 99.9 % on time at P3 under both options (marks.json:4710-4723); carry 74's 1 % late rule on 5.3's runs, read under option B as the room's `late` count and under option A as the commands that arrived after their stamped tick ran (1 - `tick.s.cmd_on_time`'s share, SERIES.md:61), both reported on every short run (TE-4); the lead it settles on per profile reported beside U4R-Q14's numbers (P0 2, P1 3, P2 and B1 5, P3 7; P4 9 by the same arithmetic: l1/src/lib.rs:193-196) | carry 74 |
| The bots' rules | unchanged for honest bots: one command per client step, only when the newest tick moved; at most one new edge per press counter per command (bots/src/lib.rs:14-21; U4R-Q14's (b1) and A4) | r-check :322-323 |

### 2.3 `InputBuffer` option A beside U4e's option B (npt-room; OQ-0001-6, ADR 0001:1156-1167)

| Field | Option A (U4f builds) | Option B (U4e's, kept) |
|---|---|---|
| When a command runs | at the first room tick after it arrives (pumped before the tick: plan.md:622-623), several in one tick within `CMD_CATCHUP` = 8 per client, each one's tick strictly newer than the last run | exactly one per client per tick, the one stamped for it (room/src/input.rs:127-151) |
| A tick with none | nothing runs for that client (plan §4.13 :439-441) | the last command runs again (:135-141) |
| Late | no late rule: a strictly newer command runs whenever it arrives; on time is still read against its stamp (SERIES.md:61) | stamped for a tick already run: dropped and counted `late` (:107-113). Today a stamp equal to a buffered command's is `Late` too (:110): under 2.4's order kind 2 refuses it first (NA-13) |
| **Missing, by stamp (NA-14)** | `tick.s.input_missing` (SERIES.md:63: "one per player this tick: 1 when no command for this tick had arrived") is written by its stamp: 1 when no command stamped for this tick had arrived before it ran, never "no command ran this tick" (option A runs commands on arrival, so arrivals bunch: at P3's ±15 ms one-way jitter against a 33.3 ms tick, ticks with none run are common, and the run-reading would fail -20's 0.1 % on a correct build); f34 | the same words, the buffer's stamp |
| The per-tick cap | at most 8 run per tick; the excess refused and counted under the catch-up kind (2.4 kind 10) | the buffer holds at most 8 (`Full`, :114-116), counted under the catch-up kind |
| The setting | `params.cmd_timing` "A" or "B", a start-up setting, never changed in a run, recorded in every header (README :191-192; plan.md:462); today fixed to "B" (lab/src/scenario.rs:1370) | the same |
| Depth recorded | U4F-Q3 (the card's DEPTH ruling and its option-A note) | `buffer_depth_ticks` (input.rs:63-66); the measured L - 1 reading (the card's DEPTH ruling) |
| -58's two sites | through U4f's rewrite of `offer`, the sequence-forward compare (input.rs:82; its name `CMD_SEQ`, :11) and the press-edge compare (:98; `PRESS`, :13) keep their sites, names and unit proofs: PM-0001-58/lt-cmd-seq and lt-press (plan :468-469), judged at U4g (plan :829-831) (TE-19). -58's two new lost-tick checks are U4g's (C62HB-Q4 (b): `<<P23>>` and RECS-1043, history :2204-2227), with ADV-CBR-1's caveat in U4g's brief carry (report-head, R1; card, Open findings): **not U4f's**. U4f writes the press series -58 shares with -20 (2.7) | the same |
| **Carry 80 (a) first** (`<<P1>>`, carries :515-520) | before option A runs several commands of one slot in a tick, `canonical` (room/src/room.rs:361-369) takes a total key, by U4F-Q17: within a slot, the tick's signed distance from the tick being run (`tick.wrapping_sub(t) as i32`), which orders any set of u32 ticks and equals today's wrap-aware order for ticks within 2^31 of `t`. The carry also allows the lead-bound route; rev 2 took the total key as a requirement, which rev 3 asks as U4F-Q17 (the bound alone leaves a non-total triple reachable from a fresh slot, SA-1: ticks T-2^31+1, T-2^30 and T+1, none 2^31 from T, with no last run to refuse against, and in the no-lead-bound bite run, where a panic makes the run INVALID: README :250-251; TE-21). Tests f8 and f9 (2.10) | carry 80 (a) |

### 2.4 The command checks, each with its -21 counter (npt-room unless named; plan.md:513-515; catalogue :87)

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

Each kind is one check of ADR 0001:231-239 (plus the wrong channel, plan.md:490), 11 in all (marks.json:4946-4957,
:5144).

| # | Kind (forged command) | Home, place in the order | At the head (unchanged since 11fd190) | U4f builds | Counter (builder names it; SERIES row by U4F-Q1) | Bite (2.11) |
|---|---|---|---|---|---|---|
| 8 | acknowledgement of a snapshot never sent | room, 1st, whole datagram | the datagram's commands refused (room.rs:783-788), `ack_refused`, `ack_refused_commands` | kept; counted as this kind, by datagram | `ack_refused` | PM-0001-21/no-ack-sent |
| 3 | sequence not forward | room, in the repeat filter | `Repeat`, counted with honest copies (input.rs:81-88; room.rs:795) | a refusal apart from honest repeats (NA-6) | sequence refusals | PM-0001-21/no-seq-forward |
| 1 | tick too far ahead (the lead bound) | room, 1st for a new sequence | none (NA-7) | the check; its form and value by U4F-Q8 | lead-bound refusals | PM-0001-21/no-lead-bound |
| 2 | tick equal to or older than the newest accepted | room, 2nd | caught only as `late` under B (input.rs:107-113), an equal-to-buffered stamp included (:110: NA-13) | the check, apart from `late` | not-newer refusals | PM-0001-21/no-newer-tick; PM-0001-21/not-older-than (`>=` for `>`, plan :462) |
| 4 | move length above 1 | room, 3rd | none (the sim scales a long vector) | x² + y² ≤ 127², integers (bots/src/lib.rs:30-31) | move refusals | PM-0001-21/no-move-length |
| 5 | more press edges than ticks covered | room, 4th | `PressRefused` (input.rs:90-106), counted `press_refused` | kept; the counter is this kind's | `press_refused` | PM-0001-21/no-press-edge |
| 6 | aim outside the map | room, 5th | none | x and y inside the greybox's extent (±12,800 cm: sim/src/greybox.rs:62, :151; the room's `extent`, room.rs:441); z left to the codec's range and the clamp by design (SA-14); beyond-reach points pass on to the sim's clamp | aim refusals | PM-0001-21/no-aim-bounds |
| 10 | more commands in a tick than the catch-up cap | room, last | option B's `Full` (`buffer_full`) | option A's per-tick cap; both counted as this kind | catch-up refusals | PM-0001-21/no-catchup-cap |
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
honest script sends anyway, at the same instants, on the same flows (plan :376); the forger forges the command it
stores, so every redundant copy carries the same forgery (SA-3); the twins of 2.12 follow the same rule. Each cheat
client's `input_limited` is reported per run, 0 expected (SA-13; 2.5).

| Cheat | Suite (marks.json select) | What it does | Count per run | Its twin |
|---|---|---|---|---|
| 11 forgers, one kind each (2.4) | `forged`, A and B | an open-loop seeded script (commands from seed, slot and tick only: plan :380-381) with every k-th command forged in its kind; kind 9 on `Input` datagrams only, trailing bytes and a set reserved or pad bit in turn (SA-12); kind 7 cooldown and reload edges (SA-15); 37 honest open-loop clients | 1,000 forged items per kind, in 2.4's unit (marks.json:4934-4945; plan :442) | 2.12 |
| the catch-up kind (kind 10) | `forged` | under option A, 16 new commands in two datagrams in one tick, stamped for ticks the forger held back (plan :385-388); under option B, more commands than the buffer has room for, all inside the lead bound so none is kind 1 (SA-11; U4F-Q8); inside the bucket in both | as kind 10's 1,000 | 2.12, by U4F-Q13 |
| the clamp cheat | `clamp`, A and B | open-loop, aim points beyond the 6,000 cm reach (sim/src/scene.rs:66) yet inside the map (2.4 kind 6): U4r's stand-in's 32,000 cm (bots/src/lib.rs:74-76) lies outside the map and would be refused (NA-8). Reruns of a clamp-cheat command under option B (input.rs:135-141) are reported per run, 0 expected at U4F-Q11 (ii)'s recommended P0; a rerun re-clamps from a moved origin (world.rs:1283-1312), so a lossy profile needs a per-tick twin rule first (SA-10; f18) | 1,000 points beyond reach per weapon kind the sim has (hitscan: marks.json:4980-4991) | the forged run's clamp reports' points (SERIES.md:324) at the same instants (plan :363-366) |
| the 1.5x clock | `fast-clock`, A and B | a client whose host clock runs at rate 1.5 over each 60 s episode (plan.md:626; the host clock stays continuous; the lab's doc names the rate U4f's: scenario.rs:18-19), a command at every one of its steps (45 a second, inside the bucket's 60 a second: SA-13), stamped by its own rule by U4F-Q11 (v); each episode reports its lead-bound and not-newer refusals beside its excess | 10 episodes of 60 s (marks.json:5014-5025) | none (the check compares with the bound: SERIES.md:155-156; the honest reference by U4F-Q11 (iii)) |
| the lag switch | `lag-switch`, A and B, P1 | holds its `Input` datagrams 200 ms, then sends them in one burst (about 6 datagrams, inside the bucket's 8: SA-13). At one command per moved newest tick a hold carries about 6 new commands, under `CMD_CATCHUP` = 8, so its check cannot fail at 200 ms (TE-7, SA-7): U4F-Q15 | 100 bursts (marks.json:5050-5061) | none |
| the never-acker | `never-ack` | a brawl client whose `Input`s never acknowledge a snapshot or a full state (ack groups 0); every other client brawls; its `overflow_resyncs_count` reported (scenario.rs:1270; SA-16) | one among 48 (ADR 0001:1612) | none |

### 2.7 npt-lab: suites, settings and series (lab/src/scenario.rs; lab/src/logs.rs)

- **Settings:** `--cmd-timing A|B` into the header (scenario.rs:1370 today writes "B"); the suites `forged`, `clamp`,
  `fast-clock`, `lag-switch`, `never-ack` and `clock` (marks.json's selects; SERIES.md:416's suite param) and their
  behaviours by U4F-Q11; `--twin <run>` with role `twin` (README :22-23; scenario.rs:1411, :1488); the 1.5 host-clock
  rate per client and episode (scenario.rs:18-19); the lead bound's value in the header (U4F-Q8).
- **`header.params.lead_ticks_count`** (SERIES.md:344: the one lead every client of the run stamps with) is written
  only by a run whose lead `with_lead` fixes (U4F-Q7 (i)); the contract test asserts it absent from a controller run's
  header, and the settled lead per client goes in its own series (U4F-Q1) (TE-20).
- **The -47 suite:** 1,000 connects per run at P2 with seeded random true offsets, each connect's estimate compared at
  2 s (2.1); 48 slots give about 21 rounds of fresh endpoints. Rejoins on one slot meet carry 62 (d)'s head rule and
  the slot-keyed logs of NC-U4R-5, and **carry 75's admission bound** (NA-16: npt-lab refuses an admission once the
  room has stamped 32,768 events past the event start, scenario.rs:70, :995-1006; carry 75, carries :357-365, puts that
  at about 30 s of SS's events, while 21 rounds of at least 2.5 s take about 52 s or more): the suite's logging and
  its behaviour are U4F-Q4's; f29 asserts no admission refused over its 1,000 connects.
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
  the rules themselves at c62hb-paste :120-124, quoted in c62hb-brief :603-607, name the lab's short P2 test), never
  reworded or retried around.
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

Each row adds its Δ tests in both builds unless marked "bites only"; "neg" marks a negative case. Minimum deltas:
**measured +102; bites build +117** (the measured 102, f30, and 2.11's 14 bites-only unit proofs); **61 negative
cases**. Conditional, in both builds: +3 by U4F-Q6 (b) (the Pong field's codec round trip, the Pong's 32 B limit, the
catalogue row's test); +1 by U4F-Q9 (a) (the kept window never holds more than K under a flood of new sequences); +1 by
U4F-Q11 (iii) (c) (the fast clock's fixed-input episode: the arithmetic reference equals an honest client's movement);
+1 by U4F-Q13 (a) (a refused command is resent, refused again and counted once per (slot, seq)); +5 by T-6 if
contract.rs is edited (`<<P34>>`). The builder states the exact counts and the `--list` lines that name them.

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
| f9 | carry 80 (a): carry 21's room test (room/tests/room.rs:235) fails with the sort removed (a plant, restored; the test itself changes, so +0) | room | | 0 |
| f10 | kinds 1-6, 8 and 10 each refused and counted under its own kind, under both options | room | neg | 8 |
| f11 | one kind per refusal: a command breaking two checks is counted once, under the first; an honest late command is counted `late`, never kind 2 | room | neg | 2 |
| f12 | (i) an honest redundant copy is refused under no kind; (ii) a backward sequence with other content inside the window is refused under kind 3; (iii) an honest P3-like trace with redundancy 8 and reordered datagrams gives 0 refusals of any kind; (iv) a sequence older than the window is dropped and counted `seq_outside_window`, never as forged | room | neg | 4 |
| f13 | (i) 8 far-ahead commands, then an honest command: the honest one runs (NA-7); (ii) under option B a command stamped equal to a buffered one is kind 2, never `late` (NA-13) | room | neg | 2 |
| f14 | every refusal field of `Counters` and of `ClientCounters` written per session (one test per struct, each field asserted), and a kicked session's written at its kick (SA-5) | lab | neg | 3 |
| f15 | each forger produces exactly its kind (its datagram decoded: only that check broken), 11 cases; kind 9's on datagrams only, both forms; kind 7's cooldown and reload edges | bots | neg | 11 |
| f16 | per kind (11) and the clamp, under A and under B: a short forged run and its twin give equal fate logs on every flow, equal snapshot group counts, **the same set of ticks hashed** (a tick hashed in one run only counts as a mismatch: judge/src/figures.rs:191-214) and equal state hashes on every tick (plan :389-390; SA-3) | lab | | 24 |
| f17 | carry 46 (`<<P1>>`): a forged press stream and its twin give equal state hashes on every tick, at the room's level | room | | 1 |
| f18 | the clamp cheat's points lie beyond reach and inside the map; the twin's points from the clamp reports give equal hashes on a short run; under option B a clamp command rerun after a move gives two reports for one sequence (SA-10) | bots, lab | | 3 |
| f19 | the 1.5x clock: excess commands at most 0 in the clean build, with lead-bound and not-newer refusals each above 0 per episode (SA-6) | lab | neg | 1 |
| f20 | the lag switch under P1: at most `CMD_CATCHUP` commands run per tick; movement within the catch-up cap's | lab | neg | 1 |
| f21 | the never-acker: headroom reaches 0, never below; one resync, then `Kick(overflow)` within `FULL_STATE_ACK_TICKS` (its later datagrams reaching no room call is U4r's r9, scenario.rs:2267, kept: TE-17) | lab | neg | 1 |
| f22 | the catch-up kind under option A and under option B, inside the bucket: the catch-up check refuses the excess, never the bucket or the lead bound | lab | neg | 2 |
| f23 | a known type on the control stream: refused and counted, the session kept (catalogue :87, :99) | lab | neg | 1 |
| f24 | carry 80 (b), asserting U4F-Q16's bound and the lost count; an honest stream's `lost` output unchanged (2.9) | client | neg | 1 |
| f25 | carry 80 (c): the unencodable command dropped and counted, the acks kept | client | neg | 1 |
| f26 | carry 80 (d)'s endpoint test, each half failing under its plant; no `lost` run from either too-old case (2.9) | client | neg | 1 |
| f27 | carry 80 (e), failing under its plant | client | neg | 1 |
| f28 | every series of 2.7 written, with its SERIES.md row, on a short run of each suite at 5.3's lengths (the contract test's own check: plan §4.12 :412-415; contract.rs:6-9); `lead_ticks_count` absent under the controller (TE-20); each cheat's `input_limited` written | lab | | 1 |
| f29 | the -47 suite: 1,000 connects, one event each at 2 s, true offsets seeded, **no admission refused by carry 75's bound** (NA-16) | lab | | 1 |
| f30 | §4.11's bites per instance for U4f's new ids: a bitten and a clean endpoint and room on parallel threads (plan :399-401); bites only, since `with_bite` exists only in bites builds (plan :193-194) (TE-9) | client, room | | 1 (bites only) |
| f31 | carry 39: each rate-derived value U4f adds, pinned at 30 and 60 Hz: ClockSync's tick period in ns; the lead bound's ticks (U4F-Q8 (b): `ticks_for_ms_at` with the rate passed, sim/src/ticks.rs:25; under (a), npt-sim's own pin by carry 18's route, `<<P7>>`). The lab keeps the 60 s episode and the 200 ms hold in ns on simulated time: no tick count (TE-10) | client, room | | 2 |
| f32 | honest stamps rise strictly and stay within the lead bound: a property test over seeded P0-P4 and B1 sample traces, and a case through a lead step-down and a ClockSync reset (TE-15, SA-8) | client | | 2 |
| f33 | npt-lab's attribution: per kind (11), a forged item counted rejected only under its own kind; a forged command refused under another kind counts unrejected; a planted copy that ran counts unrejected 1 (TE-23, SA-4) | lab | neg | 13 |
| f34 | NA-14: under option A, a P3-like trace whose commands arrive bunched (two in one tick, none the next) writes `tick.s.input_missing` 0 for every tick whose stamped command arrived before it ran, and 1 for a tick whose stamped command never arrived | room | | 1 |
| f35 | NA-15: a session that sends `Leave`, one whose link closes, and one replaced: each one's `Counters` and `ClientCounters` are written once, with the values before the room freed the slot or the `Decoder` was dropped | lab | neg | 1 |

Kept, not new: the Send asserts (plan :402-403) cover `ClientEndpoint` with ClockSync and `Room<World>` with option A
(room/tests/room.rs:228); the runner's twin test (plan :406-407); -58's lt-cmd-seq and lt-press unit proofs (2.3);
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
| refused whole by the room (kinds 1-6) | by U4F-Q13: (b, recommended) the twin's script never makes the refused command (its sequence index skipped), so neither run carries it after its refusal; or (a) the same datagram with the forged command, and every later copy of it, left out | condition 1 (only the refused part removed); SA-3 |
| the catch-up kind (kind 10), in one row (TE-18) | the twin's first datagram of that tick carries the commands the forged run's room accepted; every further datagram of that tick carries repeats only; the refused commands are never made (U4F-Q13 (b)), so the ticks they named run nothing (A) or the last command again (B) in both runs | plan :385-388; SA-3 |
| a datagram voided whole (kind 9) or its commands refused (kind 8) | the header only, no commands, its ack equal to the previous datagram's | plan :377-379 |
| a wrong-channel message on the control stream (kind 11) | a valid no-effect message (a ClientSettings repeating the current delay) | plan :382-384 |
| a sim-checked edge (kind 7) | the forged command with only the refused press removed: that button's counter one lower from the refused command on, every other field equal, exactly the presses `reports()` lists (carry 46) | plan :367-371; wave1 :197-235 |
| the clamp | the forged run's clamp reports' points at the same instants (one per command: 2.6's rerun rule) | plan :363-366; SERIES.md:324 |

Every client of the forged and clamp suites runs an open-loop seeded script (plan :380-381). f16 proves each row, per
kind and under both options, before the judged sets; f17 proves carry 46. The judge's twin figure needs the twin in the
same judge call and the same profile, seed and bite (figures.rs:191-214; 5.4).

### 2.13 The seam rule and the public items, by crate (A19: plan §7.2 :930-931; §5 :512)

- **The seam:** U4f changes no seam type (`Command`, `Rewind`, `TickInput`, `SimCore`'s methods: plan §4.2). The two
  npt-sim bites (NA-9) add ids to `World::BITES` (sim/src/world.rs:937-938) and their sites, bites builds only, if
  U4F-Q1 opens crates/sim for them; any other sim change is a seam change for the advisor (carry 26's route).
- **Proposed public items** (engine-architect checks them in review; each named in the report against §4.1 and plan.md
  §4.7; anything else named as beyond them):

| Crate | Added or changed | Why |
|---|---|---|
| npt-client | `ClockSync` (plan.md:610; not in plan §4.1's row :193); a `ClientEndpoint` accessor for the clock estimate, which the lab reads for -47; the unencodable-command count's accessor (2.9 (c)); the controller's setting if U4F-Q7 keeps one; a sequence skip for the twin's script if U4F-Q13 (b); `BITES` to 6 ids | 2.1, 2.2, 2.9, 2.11, 2.12 |
| npt-room | option A behind the `cmd_timing` setting (a `Room` constructor or setting); the per-tick refusal records (slot, seq, kind) and the kept window, by U4F-Q9; the new counters in `ClientCounters` (room.rs:63-136), `seq_outside_window` among them; the lead bound's setting if U4F-Q8 makes it data; `BITES` widened by 9 ids | 2.3, 2.4 |
| npt-bots | the cheats' and the never-acker's constructors on `Behaviour` (plan §4.1 :197 names them); a no-fire clock-suite script if U4F-Q4 rules it | 2.6, 2.7 |
| npt-sim | none, or two ids in `World::BITES` (bites builds only) by U4F-Q1; a lead-bound tick constant only by U4F-Q8 (a), through carry 18's route | 2.11, 2.4 |
| npt-server | none (2.5), unless U4F-Q14 or U4F-Q6 opens it | |
| npt-lab | binary settings only (2.7); no new library item; RR-CB-2's three doc lines (9.2) | |

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
placeholder here. Carry 62 (h) (:305-323) is not U4f's: C62H-KEEP answered (a), the check stays as built (:322-323).
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

**Commands** (Mustafa's rule of 2026-11-16: `<<P17>>`): every command in the simple forms that need no approval, one
per call: the runner call, or a plain `git ...` from your worktree as your working directory; no `cd ... &&`, no `;`,
`&&` or pipes, never `git -C`, `git -c` or `git -P`; read and search files with the Read, Grep and Glob tools. A call
that may still ask is named in your notes before you send it. **Git** only in your worktree; commit nothing, push
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
| **Total** | **about 4,400 (3,500 to 5,500)** | 1.5 times M's 3,000 by this estimate; the stop stays his: past 6,000 |

The builder puts `git diff --stat <base>`'s added lines in each block's notes line; past 6,000 the build stops and the
main session tells Mustafa in three lines (what grew, why, the cheaper option), as his rule says.

**His 13:35 rule** (`<<P15>>`, `<<P16>>`): you change no marks.json, no existing SERIES.md row, no marks/README.md and
no judge file (above); if the build needs one, it stops and reports (5.6).

**.claude/rules/netcode.md** binds the proto, room, client and server code U4f writes or feeds (plan :921-922): inputs
only, never state; every inbound value validated; no unbounded buffer keyed by client data (ClockSync's samples, the
room's refusal records and kept window, the cheats' held datagrams all capped); the catalogue updated with any protocol
change. **Hostile input in the logs** (catalogue :120-121): U4f's logs of forged input hold kinds, counts, sequences
and the clamp points SERIES.md rows name, never a datagram's bytes, debugging included (results/ is committed)
(SA-17).

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
U4F-Q15 (b); +13 by U4F-Q10 (c). Each measured run of -20, -21 and -22 is 10 minutes (marks.json's notes), seeds 1 to 3
(plan.md:772-782); each bite run seed 1 (c-u4r-runs.mjs's pattern, its header :25-31). -20's runs need 10,000 scripted
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
measured (c-u4r-r2.mjs:8-9), except the fast clock's 70 s measured, which holds one whole 60 s episode and its settling
(TE-13); a lag-switch run at the suite's spacing (100 bursts in 600 s, one per 6 s) holds 3 bursts; a clock run holds a
round of 48 connects (about 2.5 s). Every 48-client run's warm-up is 10 s (head-check ADV-CBH-2: PM-0001-2's load checks
are shown at the sets' 10 s warm-up, c-u4r-runs.mjs's settings).

| # | Suite | Option, profile | Bite | Shows | Why |
|---|---|---|---|---|---|
| p1 | forged | A (profile by U4F-Q11 (ii)) | | every PM-0001-2 check passing and the shot band | carry 86 |
| p2 | clamp | A | | the same; the clamp-cheat reruns (0 expected) | carry 86; SA-10 |
| p3 | fast-clock | A | | the same; the episode's excess, lead-bound and not-newer refusals | carry 86; SA-6 |
| p4 | lag-switch | A, P1 | | the same | carry 86 |
| p5 | -20 | A, P3 | | the same; `tick.s.input_missing`'s share by stamp (NA-14) | carry 86 |
| p6 | -20 | B, P3 | | the same, with U4f's suite and controller | TE-3 |
| p7 | forged | A | no-lead-bound | the forged check fails by name | TE-6 |
| p8 | clamp | A | refuse-not-clamp | the clamp check fails by name | TE-6 |
| p9 | fast-clock | A | not-older-than | excess above 0 | TE-5, TE-6; D12 |
| p10 | clock | P2 | newest-sample | -47's check fails | TE-6; D12 |
| p11 | clock | P2 | tick-quantised | -47's check fails, or it is reported as not failing (D4) | TE-6; D12 |
| p12 | -20 | U4F-Q10's option, P3 | redundancy-1 | -20's check fails, PM-0001-2 passing | TE-6; D12 |
| p13 | lag-switch | A, P1 | no-catchup-cap | the lag-switch checks still pass (expected: TE-7), `tick.v.lagswitch_cmds_count`'s max | TE-7, SA-7; D13 |
| p14 | forged | as p1 | | p1 again at seed 1: fate-log and per-tick hashes equal to p1's | TE-24; D11 |
| p15-p18 | forged, clamp, fast-clock, lag-switch | B | | as p1-p4 | only by U4F-Q11 (iv) |
| p19 | lag-switch | A, P1 | | an unjudged 400 ms hold (about 12 commands): the catch-up cap refuses the excess, `tick.v.lagswitch_cmds_count`'s max 8 | only by U4F-Q15 (c) |

Each run also reports carry 74's late share in both readings (option B: the room's `late` count against the commands
sent; option A: the commands that arrived after their stamped tick ran, 1 - `tick.s.cmd_on_time`'s share, SERIES.md:61)
(TE-4; D15); carry 77 (c)'s `stale()` sum; the per-session refusal counters (2.5); each cheat's `input_limited` (0
expected); the summary line's C62H-BOUND tally (lost ticks by reason: scenario.rs's Summary and logs.rs:179-191), read
as the -17 sets' were. A suite that fails, a late share above 1 %, or a bitten run whose check does not fail (README
:320-321) comes to the advisor before its set (carry 86). One no-* bite of the forged suite's 11 runs short (p7); the
other 10 rest on their unit proofs until the judged set.

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
| -21 | 49 | 2,617.0 (49 x 53.4) | 3.6 to 4.8 h (263 s a run at P0's 69.6 ticks/s, up to 355.4 s, the longest judged run) | the first run of each suite, and 5.3 |
| -22 | 4 | 320.4 (3 x 53.4, plus the S30 bite, three 10-minute runs' worth, 160.2) | about 34 min (the S30 bite about 1,015 s) | the bite run |
| -47 | 5 | no figure (1,000 connects in about 21 rounds of about 2.5 s: about 1 min of simulated time a run) | no figure | f29's short run; p10, p11 |
| pre-runs (5.3) | 14 (18 by U4F-Q11 (iv); 19 with U4F-Q15 (c)) | about 190 (14), 250 (18) or 261 (19): r2's 13 suites took about 138.3 (145 MB: c-u4r-r2.mjs:40-43), about 10.6 a run; the two 70 s fast-clock runs about 2.7 times that | about 1 to 1.5 h | the runs themselves |
| the cap measurement (2.8) | 9 | about 95 (9 x 10.6) | under 1 h | the runs themselves |
| **Total** | **65 + 14 + 9 (+ 5)** | **about 3,596 (up to about 3,667)** | **about 6 to 8 h** | |

**Room (NA-5 as changed):** the last reading is 5,514 MiB free after U3C-HELD-1's E6 (2026-11-17 12:34 UTC; the card),
1,418 MiB above the 4,096 MiB floor, against about 3,596 to 3,667 MiB of logs, before U4f's build and the release
builds. Where room comes from, in this order, never during a build or a set (.claude/CHECKPOINTS.md §11 :236-238):
1. **Packing, by brief 8.1's procedure** (c62hb-brief :494-499: the ruling named, the list committed alone and pushed,
   `kept-logs.mjs pack`, free before and after on the card), as PACK-READ (a) allows (`<<P22>>`; RECS-1043) under the
   advisor's R4 (report-head; the card): 8.1's list has one folder left, u4r-P3-0f066338dd8d (519 MiB; B1 and P2 are
   packed); then, as each U4f set's results are committed and its ruling closed, that set's own raw-log folder
   (§11's item 1). **Never** either P0 folder (u4r-P0-93509dfd7fd1, the kept set, and u4r-P0-051995db9cea, U4r's),
   fc411116c1f1's or fd4612050cd6's (C62H-BOUND's reference sets) or the c-u2 folders, without his word (c62hb-brief
   :500-501; R4). A hash mismatch or a folder that no longer matches its list: the folder stays, tell him, pack nothing
   more (§11 :243-245).
2. **The main session's part-cleans of the shared target** after U4f's merge and before its first set, with no lane
   running (W1-TARGET item 3 as amended: `<<P27>>`): the judged sets build in the main folder's own cache
   (`CARGO_TARGET_DIR` unset: plan :918-919; c-u4r-runs.mjs:6-9), so emptying the shared target does not touch them.
3. Each set's runner checks its own projection against the floor before it starts (c-u4r-runs.mjs:225-229); a set that
   does not fit is not started (U4F-Q2).

D2 re-reads the free space and the folders' sizes before the advisor's check and at launch.

### 5.6 marks.json, SERIES.md and the judge: the bars (his 13:35 rule, `<<P15>>`, `<<P16>>`)

Every change U4f could make to marks.json, SERIES.md or the judge (check, part, bite, figure, fixture, threshold), and
whether it is a new or a changed bar:

| # | Change | File | Kind | By the recommendations | To Mustafa |
|---|---|---|---|---|---|
| B1 | none to -20's, -21's, -22's or -47's parts, checks, bites, thresholds or notes: U4f's runs fit marks.json as it stands (5.1), and every figure they read exists (generic stats; `twin_hash_mismatch_count`, figures.rs:195, :830) | marks.json, crates/judge | - | no change | nothing |
| B2 | new SERIES.md rows for report-only series (2.5's per-session counters, the settled lead, `seq_outside_window`, `unencodable_commands`, the fast clock's per-episode refusals, each cheat's `input_limited`, the clamp reruns, the cap measurement's high-water series); no check reads any of them | SERIES.md (U4F-Q1) | not a bar | built | nothing; the read is recommended (c62h-review :93) and named in the report |
| B3 | the lead bound's value (U4F-Q8: 400 ms = 12 ticks recommended): not a text in marks.json, but it sets how much the fast-clock checks allow (marks.json:5026-5049: "the server's ticks plus the lead bound") | the room's setting, the header | a bar's allowance, set for the first time | built at the ruled value | **recommended for his yes** (U4F-Q8 (iii)): the value and the allowance it gives, before the build |
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
after it goes back to him; the bar is never reworded to fit a run (`<<P21>>`). **The read at U4f's close**
(`<<P16>>`; c62h-review :84): `git diff --stat <base>..<head> -- spikes/netcode-proof/marks/marks.json
spikes/netcode-proof/crates/judge` (D10); if it is not empty, a reviewer's read is owed before U4f closes (the
test-engineer's review can be it, if its notes name the lines read and marks.json's sha256 before and after), and the
report to him names it in its own line with what covered it; the same read is recommended for SERIES.md's new rows and
named likewise.

---

## 6. End checks (each a counted run under section 4's rules: `-x`, `df` before each)

1. check.mjs twice, every step passing both times: 11 steps (fmt, clippy, clippy-wasm32, doc, test, deps, wasm, refs,
   marks, self, redact), in N/u4r-brief.md:1073-1079's form: each run's last line, the test step's two counts as deltas
   on `<base>`'s (at least +102 measured and +117 bites, plus each conditional delta a ruling brings), the self count,
   the provenance lines. The test step's run includes C62H-BOUND's lab tests (scenario.rs:2795-2990) and U3C-HELD-1's
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
   the strictly-rising stamp rule (2.2) as built; `canonical`'s key (2.3); `tick.s.input_missing`'s reading under A
   (NA-14); where 2.5's counters are read before each session end (NA-15).
3. The tests f1-f35 and any you added: name, file, what each shows; the counts in both builds as deltas (at least +102
   and +117, plus the conditional deltas the rulings bring), with the `--list` lines; the negative cases counted (at
   least 61).
4. Each bite of 2.11: id, site (file:line), the test that fails by name in the bites build, that test passing clean.
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
12. How each U4F-Q ruling (Q1-Q17) was applied.
13. The items that rode (9.2: RR-CB-2, TE-B1-R2-1 with RR2-B1-1, RR-B1-7 (1); T-6 if contract.rs opened) and how.
14. The bars line: `git diff --stat <base>..<head> -- spikes/netcode-proof/marks/marks.json
    spikes/netcode-proof/crates/judge` and the SERIES.md rows added (5.6).
15. Every stop-clause entry (by U4F-Q5) and every test failure, with its cause; no retry into green.
16. The diff's size (SIZE-CHECK: added lines, against the plan's 3,000 and this brief's 4,400); what you did not verify;
    what is left; who built it. (5.3's pre-runs and the judged sets are the main session's, after the merge.)

<!-- r3: next part -->

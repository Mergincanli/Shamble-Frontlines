# test-engineer: check of U4f's builder brief rev 3 (2026-11-17)

Brief: N/u4f-brief.md (N = .checkpoints/netcode-proof-test), read only, at branch head 42e0912 (spikes/ = 0ad9661's).
Scripts (read-only, print only): N/u4f-r3/te_anchors.py (stamps and the 36 anchors), N/u4f-r3/te_words.py (7-word runs
of his words outside section 12 item 3). Nothing built or run.
(No Edit tool in this agent: this file is rewritten whole with Write at each save.)

## Plan (blocks)
B1 brief whole · B2 plan §5 U4f and counts · B3 TE-1..TE-24 · B4 placeholders · B5 recheck rows · B6 judged runs ·
B7 13:35 rule · B8 disk, memory, D-items · B9 missing, untestable, scope. All DONE.

## Progress (one line per block)
- B1 done: 1,408 lines, sections 0-12, NA-1..NA-16.
- B2 done: 2.10 rows sum to measured +102 (f30 excluded), bites +117 (102 + f30 + 14 unit proofs); neg rows 61; 5.1: 36 measured
  (6 + 24 + 3 + 3) + 12 twins + 17 bites = 65 (= plan §9 :962-963); 5.4's 4,801 / 1,927 / 6,426 MiB re-derived from
  c-u4r-runs.mjs:177; 5.5's 3,596 / 3,667 MiB re-derived; room 5,514 - 4,096 = 1,418. Every §5 U4f item (:806-815), each
  §4.10/§4.11/§4.14 test and bite, carries 46/80 (a)-(e)/86 and the -22 load report are in the brief.
- B3 done: walk below. TE-1 fixed (repeat filter first; honest copies counted `commands_repeat`, never forged).
- B4 done: all 14 stamps equal 0.2's; P1-P23, P25-P36 found once with the stated spans, entry numbers (P1: 46, 80, 86) and P14's
  line-before; P24's "Write files with the" found twice (advisor-u4df.md:313 and :462): TE3-9.
- B5 done: rows 1, 2, 6, 11, 14, 16, 19, 22, 27, 31, 35 (part), 36, 37, 38, 39, 40, 41, 44, 45 hold at the head (lines below).
- B6 done: marks.json -20 :4644-4804, -21 :4805-5145, -22 :5146-5228, -47 :9072-9135; judge eval.rs:1386-1502 (bite Shown on any
  failing plain check of its parts, gaps aside :1085, :1460-1478); figures.rs:191-214 (twin: profile, seed, bite, role twin,
  same set); default window measured (marks.rs:510-511). JDG-ONLY-1 (c62h :20) and ADV-CBH-1/-2 applied in 5.4.
- B7 done: rev 2 cited only :2127 (34a2e55 draft :124, :695); rev 3 carries :2125 (P15) and :2127 (P16) at 1 item 5, 4, 5.6,
  12 items 6 and 8; 5.6 B1-B7 list every change the options can make.
- B8 done: TE3-6, TE3-8, TE3-10; D1 (check.mjs `--only`, :2669), D2, D4 (P2 = 100 ms ±20, 2 %: l1/src/lib.rs:186-188; ping
  bucket 10: pump.rs:21-22), D6, D9, D14 runnable; D12: TE3-3; D3: TE3-12.
- B9 done: no scope creep found (every item traces to plan :806-815, §4.10-§4.14, carries or a U4F-Q).

## TE-1..TE-24 (rev 2's fix; still in rev 3?)
TE-1 fixed (2.4 items 1-4; f12 (i), (iii); holds against input.rs:80-125, room.rs:791-799). TE-2 fixed in part (f12 (iii), (iv));
the never-received-sequence case is open (TE3-13). TE-3 fixed (p6). TE-4 fixed (2.2, 5.3, D15). TE-5 fixed as U4F-Q11 (v) + p9.
TE-6 fixed (p7-p12; D4 (2)). TE-7 fixed as U4F-Q15 + p13, p19. TE-8 fixed as U4F-Q11 (iii) + 5.6 B5. TE-9 fixed (+102/+117,
conditionals). TE-10 fixed (f31). TE-11 fixed as U4F-Q16 + f24. TE-12 fixed (f25: aim past POSITION_XY's 16 bits,
proto/src/input.rs:10). TE-13 fixed (5.3 lengths). TE-14 fixed (f6). TE-15 fixed, but f5's bounds are too loose (TE3-1).
TE-16 fixed (MiB). TE-17 fixed (f21 Δ1). TE-18 fixed. TE-19 fixed. TE-20 fixed. TE-21 fixed as U4F-Q17. TE-22 fixed (5.2,
12 item 12). TE-23 fixed as f33, but its rule fails for kinds 3 and 7 (TE3-2). TE-24 fixed (p14, D11).

## Recheck rows walked at the head (all hold)
1 input.rs:10-13, :61, :64-71, :80-125 (:82, :89, :98, :110), :127-151, :155 · 2 room.rs:361-369 (:367) · 6 room.rs:744-801
(:783-788, :791, :795) · 11 pump.rs:16, :24, :26, :372-480, :387-404, :449-472, :500-516 (:510) · 14/16 client lib.rs:28, :214,
:267, :298, :315, :408, :414, :443, :450, :485-488, :492, :528, :583, :631, :645, :810 · 19 scenario.rs:70, :134, :215, :227,
:458, :1370, :1447 · 22/39 c-u4r-runs.mjs:133, :153-154, :167, :172, :177, :179, :224-231, :305, :312, :327, :569 · 27
check.mjs:39, :1970 · 31 scenario.rs:1262, :1270, :2212, :2267 · 36 figures.rs:538, :579, :677, :689, :818-831 · 37
marks.json:3972-3982, :3983-4005, :4036-4039 · 38 SERIES.md:291, :292, :322, :378 · 40 scenario.rs:2795-2843, :2904, :2954,
:2970 · 41 net/src/l2/delay.rs:538 (check_held_bound at every write), delay_tests.rs:2274 (G-NC2) · 44 room.rs:689-695, :739 ·
45 scenario.rs:70, :995-1006. SERIES rows of 2.7 at the cited lines.

## VERDICT: REVISE (schema: FAIL): 2 High, 7 Medium, 6 Low

## Claim · proof · gap
| Claim | Proof in the brief | Gap |
|---|---|---|
| -20 passes at P3 under both options | f5, f34, p5/p6, judged set | honest bots leave about 20 % of ticks with no stamped command (TE3-1) |
| -21 forged: every kind rejected and counted | f10, f33, f16, judged set | attribution rule fails kinds 3 and 7; kind 3 twin wrong (TE3-2) |
| short bitten runs show each bite | p7-p12, D12 | count checks fail by length; "every other check passing" unmeetable (TE3-3) |
| forgeries and twins ride the honest datagrams | f15, f16 | Input codec rules unstated (TE3-4) |
| twins fate-aligned | f16 "equal fate logs" | canonical fate log includes size (TE3-5) |
| fast clock: 10 episodes per run | 2.6, p3 | no schedule; 10 x 60 s + settling vs 610 s (TE3-6) |
| lab tests run in check.mjs | f16, f18, f28, f29 | no scale or time budget in debug (TE3-7) |
| disk fits | 5.5 | build phase unbudgeted; pack timing (TE3-8) |
| placeholders found once | 0.5, 12 item 3 | P24 found twice (TE3-9) |

## Findings, ranked (one line each; full text and fixes in the returned schema)
- TE3-1 High: -20 P3: bots' "only when the newest moved" rule kept (2.2 :269; bots/src/lib.rs:14-18); U4r P3: 700,948 commands, 178,595 steps without one (20.3 %) per run (results/u4r/0f066338dd8d/P3/runs.txt:18-20) -> about 20 % of ticks unstamped vs missing <= 0.1 % (marks.json:4724-4737); late 0.16 % at lead 7 vs 99.9 %; f5's 1 % and "at most U4R-Q14's lead" looser than the bar; p5/p6 read no -20 check by name.
- TE3-2 High: 2.4 item 6 "no copy of it ran" fails kind 7 (command runs, press refused: world.rs:1411-1419, :1446-1451) and kind 3 (resent copy of an honest command that ran); 2.12 row 1 skips kind 3's sequence; SA-3 cannot make a kind 3.
- TE3-3 Medium: 5.3 p7-p12 and D12: count checks (marks.json:4934-4945, :4980-4991, :5014-5025, :5050-5061, :9096-9103) fail by length; bite "shown" from them alone (eval.rs:1475-1478).
- TE3-4 Medium: Input codec rules (proto/src/input.rs:19-21, :155-159) unstated: twins with skipped sequences, kind 2 older ticks, kind 1 copies, fast clock (v) (b).
- TE3-5 Medium: f16 "equal fate logs": size in the canonical form (l1/src/lib.rs:353-362, :1146); twins differ in size by design.
- TE3-6 Medium: fast clock 10 episodes per run (marks.json:5014-5025) with no schedule; 70 s per episode in 5.3; -21's notes name no 10 minutes.
- TE3-7 Medium: f16/f18/f28/f29 in debug cargo test (check.mjs:2052-2057) at 48 clients, no scale or time budget.
- TE3-8 Medium: build-phase disk unbudgeted (free fell about 1,500 MiB in U3C-HELD-1's builds, card :4196-4209); pack before U4f's counted run (u3c-held-1/advisor-plan-review-full.json:37, :89).
- TE3-9 Medium: P24's anchor found twice (advisor-u4df.md:313, :462).
- TE3-10 Low: u4r-P3's 519 MiB is the folder; packing frees about 73 %.
- TE3-11 Low: property tests: no generator, seeds or case count; no RNG dependency in room/client.
- TE3-12 Low: D3 is a shell grep against section 4's Read/Grep/Glob rule.
- TE3-13 Low: the kept window's never-received sequence (gap) case undefined.
- TE3-14 Low: f9/f26/f27/D14 plant outcomes not in the return list.
- TE3-15 Low: brief :1148 retypes eight of his words (history :2125).

DONE

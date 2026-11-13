# Advisor: U4r brief rev 3.1 check — notes

Started 2026-11-13 (session time). Effort xhigh. Instructions read from origin/main:.claude/agents/advisor.md.

## Plan (blocks)
- B0: read advisor.md (done), the two earlier brief checks (form), the brief rev 3.1 whole.
- B1: read u4-build-plan.md §5 U4r, plan.md's U4r-relevant parts, u4s-carries.md §U4r; list every test, negative
  case, bite, judged run and carry; map each to the brief (check 1).
- B2: placeholders P1-P15: each anchor found once in the code at the branch head, first and last words (check 2).
- B3: my rulings since the night's rev 2 (advisor-*.md newer than ed973c9's date): applied as ruled? (check 3).
- B4: the r3 check (F-a..F-m) and notes: what rev 3.1 did with each; judge from sources.
- B5: open questions Q1, Q2 rest, Q4, Q6-Q14; Q14 with the arithmetic (check 4).
- B6: u4r-run.sh vs u4d2-run.sh (diff, the tag check, the two lock lines) (check 5).
- B7: INF2-4 runner follow-up: the two diffs vs M9 Block 2; P3 logs; P4 six copies + diffs; card's record (check 6).
- B8: TE2-5 (c): TE-M3 at c49773f under advisor-te-m3.md; what the brief's TE-M3 line must carry (check 7).
- B9: the process slip (drafter's scratch copy by shell redirect, notes B3a) (check 8).
- B10: write the ruling advisor-u4r-brief.md; return summary.

## Progress
- B0 done: advisor.md (main) read; d2 brief check's form (block list, verdict + L-conditions, checked/not, Decisions,
  For Mustafa); brief rev 3.1 read whole (1,625 lines: C27-C33, C1-C26, sections 0-12, R1-R44, Q1-Q14, D1-D13, 12's
  items 1-9); the r3 check (61 lines: F-j High, F-a/k/l/m Medium, 13 Lows) and the notes (314 lines; SLIP at top, B3a).
- B1 started: plan §5 U4r read (u4-build-plan.md:750-803).
- Head now 87d80dd (checkpoint commits after c49773f). Stamps equal section 0's: plan 01ede63d, plan.md 3ca4e12a,
  carries 8b7f165e, history 88bc195f (2,011 lines), u4r-run.sh 349b51ad. Brief sha256 bcaef383… (1,625 lines).
  `git diff --quiet cc8595d HEAD -- spikes/netcode-proof` now exits 1: c49773f's delay_tests.rs (+10 -3; one assert, its
  doc; no test added). FINDING: section 0's "spikes/ is cc8595d's", 9's and 12 items 1 and 9's cc8595d tests no longer
  hold as worded: they must name c49773f (counts unchanged: TE-M3's P5 491/561/71 at c49773f).
- B1 done (coverage, check 1): every Builds item (SimClock, L1Scenario, header params, --bite dispatch, logs, scenario
  and replay, four recordings, -57 pairing, Behaviour brawl + open-loop, c-u4r scripts), every Test (twin r1 + bite,
  contract r2 + PM-0001-2 pass + condition 5's suites via Q4, replay r3 + floored grid bite, class audit r4 + owner-to-all,
  r5 ticks/s + total), every judged run (-17 x4 + tick16 at 63,736 + client-rounding; -57 + 3 bites; -18 B1 10 bursts +
  no-fallback + burst_eviction; -30 pre-check: suite, legs, depth, comparer, late_count, setup, RTT rule + logs, both
  bites, no mark, records, NOT-RUN), sizes for fixes.rs + framing, Evidence (7.4), reviewers (12 item 7, +2 marked) are
  in the brief. Plan cites spot-checked: :750, :756, :758, :766-772, :791-792, :796-803, :806, :835, :837, :881-888,
  :909-930 hold. Marked additions: r6, r7, 7.2's receipt field, 8.4's switch, precheck-recordings.json, rewinds_refused
  reports, -18 heal bite check, the bots' step rule (Q14 b), D13. Unmarked: carry 49's unit test (F-b; carry 49 asks no
  test) -> Low, mark it as the brief's.
- -21's eight suites = forged/clamp/fast-clock/lag-switch x option A/B (lag-switch at P1); -20 P3 A and B (validity
  reorder). Q4 (a) runs the option-B halves now; mine to rule (condition 5 is my words).
- Q14 arithmetic re-done from the code (client :379-410 stamps newest+lead at input; step :415-455 one Input per step;
  room input.rs offer: forward compare, press compare, then Late on tick <= ran_through or a buffered tick; l1 delay
  d - a + U[0,2a], P2 100/20, P3 150/30, P1 50/10, B1 = P2; ticks_for_ms rounds up): need s + d_up <= t_{n+lead} with
  s < t_{n+1} + d_down, so (lead - 1) P >= rtt + jitter suffices: P0 1, P1 3, P2/B1 5, P3 7. Carry 74's form: 1, 2, 3,
  4. Duplicates: p(1-p), mean jitter/P/6: P2 10 %, P3 15 % (P1 5 %). A duplicate with a moved press counter is
  PressRefused (press compare precedes the late check): another reason (b1) beats (b2). NEW: at P0 lead 1 has no
  margin at equality: a client step at a tick's own due instant reads newest = the tick before (the snapshot is not
  delivered in its send instant) and stamps the tick just run -> every command of that client late; step times are
  fixed by the first `input` instant (client :386, :452), so a first input on a tick instant (offset 0, ms-grained
  phase on 100 ms multiples) trips it. Finding with exact fix.
- B2 done (placeholders, print-only python3 -I): P1 carries heading once at :255, span :255-432, entries exactly the
  17; P2 history :292 once + :293 ("before it changes anything."), line before :291 is "  USAGE-SAVINGS: (a), all
  three, with these limits." (two leading spaces: the whitespace rule reads it as one: holds); P3 df :462-464, P4 df
  :465-468 (two `<crate>`: first replaced, "<crate>'s files" -> "their files"), P5 j-r2 :168-171, P8 j-r2 :298, P9 m9
  :252-256, P10 m9 :259-264, P11 d2-r2 :304-308, P12 d2-r1 :308-310, P13 :72-75, P14 :116-119, P15 :135-136: each once,
  first and last words as named. All 13 rulings' sha256 equal section 0. P12's words "stays in force until TE-M3's
  cause is found and fixed" now read as ended at c49773f: they must be read with TE-Q2 (rule ends at U4r's merge only
  if no test-step failure): the TE-M3 line must say so. TE-M3's test at the head: delay_tests.rs:2544-2614 (doc
  :2544-2553, fn :2554-2614), not :2551-2603.
- B3 started: rulings since rev 2 (ed973c9, 2026-11-11 02:23): wave1 later blocks, close, j-r2, e-ruling, df, e-r1,
  e-r2, d2-ruling, d2-r1, d2-r2, m9, te-m3.

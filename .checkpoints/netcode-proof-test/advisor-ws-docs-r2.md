(Written by the main session from the advisor's structured output, workflow wf_abc2ef9b-8d4, 2026-11-07: the advisor agent has no Write tool. Its notes text follows verbatim, including its four fields.)

# Advisor: the docs PR's words and rulings, revision 2: Mustafa's 14:30 tolerance and the verifier's corrections (2026-11-07)
Branch claude/netcode-proof-test-plan-hcky5g, head 95a04a1 (docs/ and spikes/ equal to 975e72b). I worked from main's .claude/agents/advisor.md (origin/main 69ea88e). I edited no tracked file and committed nothing. This revision replaces advisor-ws-docs.md (revision 1, written before Mustafa's 14:30 refinement) whole: every edit, change and ruling below stands alone.

## Plan
1. Read Mustafa's 14:30 refinement and 13:45 words (.checkpoints/netcode-proof-test.md:349-362), my revision 1, the verifier's findings (verify-ws-docs.md V1-V31), the netcode-architect's notes and open points (netcode-architect-ws-reconnect.md:175-180) and its record draft (sections 0-2, 5, 6), the committed outputs (spikes/netcode-proof/results/u3-ws-fallback/), and the statistics skeptic's F6, F8 and F10 (u3-verify3-stats.md:71-132).
2. Re-check every old text against docs/adr/0001-multiplayer-netcode.md and main's docs/adr/README.md as they are now.
3. Fix the tolerance, how "slower than the model" is measured, how the numbers move, and the investigation rule; rewrite E1, E-PR and its Decisions needed, the PRACTICE RUN and INVESTIGATION rulings, and the ws-practice rule.
4. Fold in the verifier's must-correct items 2-7 and the architect's open points where they touch my words; keep what the verifier confirmed.
5. Return the notes, every edit whole, the spike-branch changes, the rulings and what goes to Mustafa. DONE at the end.

## Findings from revision 1 (kept; corrections marked)
- F1 ADR 0001 on this branch equals main (git diff origin/main is empty for the file; re-checked at 95a04a1), so line numbers hold on both: 424, 506-509, 747, 1250, 1295, 1462-1469, 1556, 1571, 1697-1699, 1737-1738.
- F2 The *Reconnect:* clause (ADR 0001:506-509) is in part 11 (part 11 starts at :475), not part 10. The per-address cap is part 10 (:446). Verifier V31: confirmed.
- F3 I reproduced ws_bar_fix1 at 5a5aba9 (model files unchanged since bf9d248): output identical, now committed as spikes/netcode-proof/results/u3-ws-fallback/bar-fix1-bf9d248.txt. Fix 1 654/2,164/3,945/max 11,384; today 1,121/2,686/max 12,068; budget x2 p50 776; at 480 "half within 0.67 s, at most 5 over 2.5 s, none over 6.9 s" 0.9681, today and budget x2 0.0000, x1.05 0.001, +20 ms 0.159. Verifier V3: every number confirmed.
- F4 the_recommended_fixes_are_pinned_at_b1 passes at 5a5aba9. The buffer-plus-reconnect row (tables-1e020a7.txt:129-131: 654/1,051/p99.9 1,859/max 4,196, 6,768 of 600,000 over 1 s) holds at bf9d248. Verifier V4, V15: confirmed.
- F5 MAJOR (kept): the bf9d248 bound assumes independent single bursts. With 10 bursts per client the median is 663 against 655 ms (tables-1e020a7.txt:5-6), and a run's 48 clients share each burst's tick phase (u3-verify3-stats.md:117-132). Correction: my "15-25 %" chance that a correct build fails the single-burst 0.67 s median is an estimate; the verifier's reasoning gives about 8-20 % (V26). The conclusion stands: the bound is computed for the run as it runs.
- F6 The model's snapshot is a fixed SNAP = 1,050 B (fixes.rs:34); the bound must use the scene's real WebSocket message sizes. V10: confirmed.
- F7 "A baseline that never falls back" probably cannot bite on WebSocket (TCP loses nothing); the WebSocket bite is a send budget twice the formula's (budget_scale 2).
- F8 Plan section 4.7 (plan.md:514-515) puts WebSocket's newest-only replace inside its backend, so fix 1 belongs in npt-net's WsServer send path, with no new public symbol.
- F9 Open for U3c's plan: WebSocket connections without DelayStream (P0, ML's real path) need the real TCP_INFO and SIOCOUTQ, an OS call; M14 bars unsafe code and libc in npt-net.
- F10 SERIES.md:134 still defines burst_heal as "first delta decoded". Wording corrected by V29: "the first snapshot the room writes at or after the return instant" (fixes.rs:786-787).
- F11 marks.json -18's "L2 B1" part has no params.transport select, so WebTransport and WebSocket runs would mix.
- F12 ADR 0001:1571 (-31) still says "L2's TCP proxy on RS" (the PR Advisor's Low note on #41); -31's L2 leg runs in R3.
- F13 docs/design/ is absent on main; the docs PR creates it; PR #40's REPO_MAP edit conflicts once. V31: confirmed.
- F14 Register: 0001's row "18 numbered (OQ-0001-1 to -18), 12 open" and the total "118 numbered, 97 open" become 19/13 and 119/98. V8: confirmed (18+9+20+12+36+23 = 118; 12+8+17+10+33+17 = 97).
- F15 The WebSocket bar describes Linux's TCP at the client. Cite corrected (V6): fix 1 alone on a no-timestamp client, p99 2,035 and max 12,550 ms over 60,000 phases, comes from the third skeptic round's diagnostic patch, u3-verify3-harness.md:79-80, not from the checkpoint or the tables.
- F16 RESOLVED by the verifier (V12): Linux v6.8 net/ipv4/tcp.c:630, SIOCOUTQ answers write_seq - snd_una (unsent plus unacknowledged; SIOCOUTQNSD at :632-641 is unsent only); :3842, tcpi_rtt = srtt_us >> 3 (microseconds).
- F17 R3 time +0.4 h (10.15 -> 10.55 h, inside 11 billed); EUR 1.72 planned and 1.90 cap unchanged. V27's details folded in (G16).

## Findings, revision 2 (one line each, in the order found)
- G1 Mustafa's refinement (about 14:30 UTC) is verbatim at .checkpoints/netcode-proof-test.md:349-356 (95a04a1); the 13:45 words at :357-362. It postdates revision 1 (verifier V2) and replaces revision 1's "otherwise the bound becomes the numbers" for a slower practice run.
- G2 The outputs are committed (975e72b): spikes/netcode-proof/results/u3-ws-fallback/tables-1e020a7.txt and bar-fix1-bf9d248.txt equal .gate/netcode-proof-test/u3v3-tables1.txt and u3v3-barfix1.txt (diff -q: same). Lines used: tables :5 (single bursts p50 655), :6 (10 bursts p50 663), :96 (parallel, reference only), :102 and :110 (Windows-like variants, p99 886), :123 (fix 1 alone), :126 (fix 3 alone), :129-131 (recommended); bar :7 (fix 1 correct), :8 (today), :9 (budget x2, p50 776), :11 (480 at Y = 2.5 s: 0.9681; x1.05 0.001; +20 ms 0.159), :14 (1,440).
- G3 Old texts re-checked at 95a04a1 (ADR 0001 equals origin/main 69ea88e): E0, E3, E5, E6's anchor and E7 match once (grep -cF); E1, E2 and E8's whole rows once (grep -cFx); E4 and E13 once after whitespace normalisation (line breaks and indents as one space; python check).
- G4 docs/adr/README.md on this branch lags main (PR #43 changed six other rows). E9's anchors (main :16, :23, :27) and E10's (the ROOM-CRASH row at :87; its end "| the Advisor | [ADR 0001](0001-multiplayer-netcode.md) OQ-0001-18 |" is unique on main) hold on main, where the docs PR applies them.
- G5 Mustafa's check item 11 is ADR 0001:1697-1699 (the "within half a second" sentence spans 1698-1699). PM-0001-53 (:1599) needs every checklist step "achieved", and item 1 (:1673-1674) says his session may land on the WebSocket fallback, so item 11 must carry the fallback's expectation (verifier V9).
- G6 Row hashes: check.mjs:141-152 hashes the mark's table row line only. E1 and E8 change -18's and -31's hashes; E13 (the checklist) changes none.
- G7 One run of 480 client-bursts: the median is the only statistic precise enough to measure a slowdown of a few percent. A nearest-rank p99 is the 5th-largest value, the maximum a single draw, and the max part's chance is known only to about a factor of 3 (u3-verify3-stats.md:117-132).
- G8 Noise of the measured slowdown, my arithmetic, not a run: the model's CDF near the median (0.327 at 0.6 s, 0.498 at 0.654 s, 0.748 at 0.7 s; u3-verify3-stats.md:71-79) gives a median standard error of about 6 ms at n = 480 for independent bursts; a client's 10 bursts are alike (body dispersion 1.74), so about 8 ms; the tick phase moves the p50 from 616 to 665 ms by decile (:117-121), which 10 random burst offsets average to about 4.5 ms; together about 9 ms, 1.4 % of 663 ms. So a correct build's slowdown lies within about +-3.5 % in 98 % of runs. The run-structured bound must print the real band.
- G9 A uniform time scale f moves the run median, the count over f x Y and the maximum together, so the model's bound for a build f slower is the bound with X, Y and Z multiplied by f and k unchanged, at the same 99/98/98 %. ws_bar_fix1's own model-error rows use that scale ("if every recovery were 5 % or 10 % slower", fixes.rs:2006-2008).
- G10 Measuring f from one run costs margin: for a build truly f slower (f up to 1.10), the practice run's and the counted run's medians each vary by about 1.4 %, so the moved median fails about 5 % of the time instead of 1 % (P(Z > 2.33 / sqrt 2 = 1.65)). A correct build (f = 1) keeps at most 1 %, since a chance move only loosens.
- G11 After a move the practice run passes its own median check by construction (its median is s x M_m, at most s x X). So the in-tolerance test is the factor plus the count and maximum checks at the moved numbers.
- G12 The nearest broken build the model knows, a budget twice the formula's, is 776 / 654 = 1.187 at the median (bar :9, :7) and is caught by the median part only (u3-verify3-stats.md:95-107). A 10 % tolerance stays about 6 noise widths below it; 15 % would sit within its reach (1.187 - 0.035 = 1.15). Smaller formula errors are caught by U3c's mechanism tests, which do not depend on the model's time scale.
- G13 PM-0001-31 after fix 1 (verifier V17): marks.json -31 checks tick.v.unsent_snaps_max_count <= 1 (SERIES.md:56; ADR :1571). Fix 1 writes a tick's snapshot whole or skips it (fixes.rs:776-777); at P2 the budget is about 6 snapshots (about 6,300 B), far below the wrapper's 64 KB, so a write never meets Pending and nothing waits above the socket. Read as "not yet written to the socket", the count stays meaningful (at most one, the one being written) and its bite becomes "queue the snapshots the budget skips"; read as the kernel's unsent bytes, a correct build fails it. E8 says which.
- G14 Revision 1's E4 words were wrong in two places (V28): the tail-loss probe is "2 x srtt plus 200 ms with one segment out (else 2 ms)" (tcp_model.rs:40-41); and "so ... the message size limit applies" did not follow from the placement: it is the wrapper's duty (u3-build-plan.md:40-42), so that sentence stays where the key has it today.
- G15 ADR 0001:1461-1462 and :1757 speak of L2 in general (R5-TEST-REC4's delaying TCP proxy); the key scopes the stream wrapper to the proof test (:1462-1463). No contradiction; left as they are (the record's section 5 plans :1756-1758 for the reconnect work). V28's note answered, no edit.
- G16 Plan hours (V27): R3 shows "bites 0.85" in VS4 and in VS5 (plan.md:737); the WebSocket bite runs in VS5, so VS5's becomes 0.95. Rounding slack (plan.md:729) R3 0.85 -> 0.45, total 5.3 -> 4.9. Section 7.1 rows: "ADR 0001's rows, measured" (plan.md:720) R3 6.55 -> 6.85, total 19.3 -> 19.6; "Bites" R3 1.7 -> 1.8, total 5.7 -> 5.8; the grand total stays 75.0.
- G17 Slice 3 (architect's F13): migration step 5 (ADR 0001:1734-1738) lists "reconnect" and ends with "Mustafa's first online playtest is run"; the scope line :1640-1641 says "slice 2 and slice 3 milestones" for part 11 as a whole. E5, E6 and E7 say slice 3.
- G18 The architect's F20: the policy most robust across outage lengths (scaled abort plus join-or-close) measured p99 about 1,126 ms at the 1 s outage (20,000 phases, a throwaway patch: u3-verify3-harness.md:122-129; now reconnect-policies-1e020a7.patch), about 75 ms above the target's 1,051. Whether that is "about 1.05 s" is Mustafa's call in the reconnect work: E6 says so; no decision now.
- G19 The record's bar form (record draft :489-494, section 6.4) borrows WS-BAR's rule by analogy and is marked "his to confirm". It is Mustafa's: escalated as WS-RECONNECT-BAR, recommended to stay open.
- G20 The pointer from ADR 0001 to the record (architect's open point 4): E5's link sits in part 11's reconnect clause, where the rule a reconnect changes lives; E6 and E7 link it too. No other line needed.
- G21 The record's draft needs (in E11): "one attempt at a time" kept (V18: revision 1's E11 (3), "every 300 ms in parallel", was wrong; parallel is reference only, fixes.rs:1252-1254); section 5 must say what this PR already changes (V20); the U3-Q4 (b) cite pinned (V21: at 803c6c1 :392-393, at 95a04a1 :400-401); section 1.1's "older unsent snapshots are dropped" reworded (V25, V17); ws_bar_fix1 cited at fixes.rs:2000-2011 (V25); outputs cited at results/, not .gate/.
- G22 Minor cites corrected (V5, V14): fix 1 alone's 600,000 row is tables :123, not :125; three_part() is fixes.rs:1528-1537 (doc :1528-1529, fn :1530).

## Blocks
(A) The docs PR's words: adr_edits E-PR, E0-E13 (E13 is new: check item 11). (B) The spike-branch changes: plan_and_marks_changes. (C) The rulings, including the tolerance, the measure, the move, the practice run and the investigation: rulings. (D) What goes to Mustafa: escalate_to_mustafa. Each block is written whole below.

## Field: adr_edits (the docs PR's exact words; apply each by matching its old text, not its line number)

### E-PR: the docs PR itself. A new branch claude/npt-readings, restarted from origin/main in a worktree (.gate/npt-readings), as README-BRANCH allows (Mustafa, 2026-11-06). Label needs-mustafa. It must merge on Mustafa's go before R3's first counted PM-0001-18 run, and before R3's run plan goes to him.

**Old text:**

```

```

**New text:**

```
TITLE: ADR 0001: the WebSocket send budget, PM-0001-18's split and its practice-run tolerance, and the fast reconnect as a real-client requirement (Mustafa, 2026-11-07)

BODY:
## What this is
This writes Mustafa's WebSocket decisions of 2026-11-07 (WS-SCOPE, RESUME-CRED, WS-BAR and his refinement of WS-BAR) into ADR 0001. They then hold before the netcode proof test's run day R3, and the reconnect requirement can't get lost. Docs only: no code, symbols +0 -0.

## His words (2026-11-07)
About 13:45 UTC:
> WS-SCOPE: no. Measure fix 1 (the newest-snapshot-only buffer) in the proof test. The reconnect becomes a requirement for the real client, built and measured before the first online playtest, with today's model as its target (buffer + reconnect: 99 % within about 1.05 s). Record that in the docs PR so it can't get lost. RESUME-CRED: deferred with the reconnect. WS-BAR: the three-part form now; its numbers after one practice run on the real test setup, but never looser than the model predicts for a correct build with fix 1. If the practice run comes out worse than the model, find out why before setting the numbers.

About 14:30 UTC:
> Refinement to my WS-BAR rule: the model's bound has almost no margin (5 % slower already fails a correct build), so declare a tolerance now, before any practice run. If the practice run is up to 10 % slower than the model, the numbers may move by that much; beyond 10 %, investigate first and never loosen. The advisor may set a different tolerance with reasons, but it must be fixed before the run.

## What changes (docs/adr/0001-multiplayer-netcode.md unless named)
1. Part 10 (the newest-only sentence): the server writes a tick's snapshot only while the client's unacknowledged bytes leave room for it within N snapshots; N = 1.5 x the smoothed round trip / the tick period, rounded up, plus 1, at least 3. A snapshot that doesn't fit is skipped and counted, never sent later. The bytes come from SIOCOUTQ, the round trip from TCP_INFO.
2. PM-0001-18: on WebTransport every client still heals within 500 ms. On WebSocket there are three checks (the median, at most k over Y, none over Z), set from the model's bound for a correct build and one unjudged practice run on L2, with a tolerance of 10 % fixed now (below). The first bound, at 480 client-bursts: half within 0.67 s, at most 5 over 2.5 s, none over 6.9 s; the proof test recomputes it for the run as it runs.
3. The failure-modes row for B1, and item 2 of "What a spike must prove": the same split.
4. Mustafa's check item 11 (the B1 burst in Brave): its expected result over the WebSocket fallback, which the overlay names.
5. Part 13's key: the WebSocket delay wrapper is now the Linux-faithful TCP model (U3-Q3 (b), 2026-11-06), no longer a fixed timeout.
6. Part 11's reconnect, a new open question OQ-0001-19, and migration step 5 (slice 3): the fast reconnect over WebSocket is required, built and measured before the first online playtest against a target of 99 % within about 1.05 s; a miss goes to Mustafa. RESUME-CRED and the reconnect's other changes wait for that work.
7. PM-0001-31: "unsent" defined after fix 1 (not yet written to the socket), its bite widened to the snapshots the budget skips, and "L2's TCP proxy" becomes the stream wrapper (the PR Advisor's Low note on #41).
8. docs/adr/README.md: OQ-0001-19's row; the counts go to 119 numbered, 98 open.
9. docs/design/ws-reconnect.md (new, by netcode-architect): the reconnect requirement in full.
10. docs/REPO_MAP.md and docs/STATUS.md (docs-keeper).

## The tolerance (fixed now, before any practice run)
- 10 %, as Mustafa proposed; the advisor keeps it. It sits well above how much a correct build's measured slowdown varies from run to run (about +-3.5 % by the advisor's arithmetic; the recomputed bound prints the real band) and well below the nearest broken build the model knows, a send budget twice the formula's, which is about 19 % slower at the median (776 vs 654 ms). One check before the run may lower it, never raise it: if the recomputed bound shows that broken build coming out within it in more than 1 % of runs.
- "Slower than the model": the practice run's median recovery divided by the model's median run median for the same run. The median is the only one-run statistic precise enough: one run's p99 rests on 5 client-bursts and its maximum on one.
- Within the tolerance, X, Y and Z are multiplied by that factor and k stays: exactly the model's bound for a build that much slower. A faster practice run keeps the bound; it is never tightened from one run.
- Beyond it, or if the practice run fails the count or maximum check at the moved numbers: the cause is found first, and no number is loosened. A changed model comes back to Mustafa with its new numbers.

## Evidence
- Model rows (branch claude/netcode-proof-test-plan-hcky5g, committed at 975e72b under spikes/netcode-proof/results/u3-ws-fallback/): the proof test's TCP model linux-6.8-tcp v3 was checked against Linux 6.8's source in three skeptic rounds. Over 600,000 B1 phases at P2 per row (p50 / p99 / max, ms): today's 64 KB buffer 1,121 / 2,686 / 12,068; the send budget alone 654 / 2,164 / 11,384; budget plus reconnect 654 / 1,051 / 4,196, p99.9 1,859 (tables-1e020a7.txt:120-131). The bound (bar-fix1-bf9d248.txt:7-14, ws_bar_fix1 at bf9d248): a correct build passes 0.968; today's buffer and a budget twice too large pass 0.000; the correct build 5 % slower passes 0.001, 20 ms later 0.159. The advisor reproduced ws_bar_fix1 at 5a5aba9 (model files unchanged since bf9d248) with identical output, and ran the recommended row's pin, which passed.
- The kernel facts in part 10, checked in Linux v6.8 by the verifier: SIOCOUTQ = write_seq - snd_una, unsent plus unacknowledged (net/ipv4/tcp.c:630); tcpi_rtt = srtt_us >> 3 (net/ipv4/tcp.c:3842).
- [the word-for-word verify script's result; gate checks 1-4 and 5b-5d PASS, symbols +0 -0; node .claude/scripts/repo-map-check.mjs OK; the verifier's spot-check table]

## Not verified, and known risks
- The first bound assumes independent single bursts. The real run has 48 clients sharing each burst and 10 bursts per client; the model's lineage rows put the median 8 ms slower (663 vs 655 ms). So the bound is recomputed for the run as it runs before R3 (the proof test's plan, not this PR).
- The tolerance absorbs any slowdown up to 10 %, including a defect that slows the median by less than that. The proof test's tests of the budget itself catch formula errors exactly, whatever the model's time scale.
- The slowdown is measured from one run, so a move of up to about 3.5 % can be chance, and a build truly slower within the tolerance fails the moved median about 5 % of the time instead of 1 %. The +-3.5 % is the advisor's arithmetic, not a run.
- By design a correct build fails the WebSocket part about 3 % of the time (a 1 % / 2 % / 2 % allowance; 0.968 passes). Such a fail stands, with its investigation.
- The WebSocket bar describes Linux's TCP at the client (LG, and the model). A client without TCP timestamps (Windows) with the budget alone has p99 2,035 and max 12,550 ms (60,000 phases, the third skeptic round's diagnostic patch).
- The reconnect target is a model figure for a Linux client; real browsers and Windows are measured by the reconnect work, before the first online playtest.

## Decisions needed
- WS-RECONNECT-BAR: how is the reconnect's pass bar set? The reconnect record proposes borrowing WS-BAR's rule (tail parts plus mechanism checks, numbers from the model and one practice run on a faithful setup, with the same 10 % tolerance), marked "Mustafa to confirm". Why it matters: it decides when the reconnect work is done. Options: (a) confirm the borrowed form now; (b) leave it open: the reconnect work proposes its bar with its plan, for your go, against the target. Recommendation: (b): the faithful setup, the reconnect policy and the sample size it needs are not known yet, and nothing in the proof test depends on it. It does not block this merge.
Everything else is recorded word for word or marked "the advisor's reading, 2026-11-07": the 1 % / 2 % / 2 % definition of the bound, how the slowdown is measured and how the numbers move, a faster practice run keeping the bound, the burst offsets, the WebSocket bite, and -31's "unsent".

## For Mustafa
1. Read the top of this PR, the new PM-0001-18 row, and check item 11's new expected result (your own Brave step).
2. WS-RECONNECT-BAR above: answer (a) or (b) when you like; it does not block this merge.
3. If the PR says what you decided, say "merge #<n>" in a session's chat. It must merge before R3's first counted PM-0001-18 run.

[end with the session's own PR attribution lines]
```

**Why:** Mustafa asked for the decisions to be recorded in the docs PR, and his 14:30 refinement asks for the tolerance to be fixed before any practice run. ADR edits are escalation category 3, so the PR is needs-mustafa. It gets a fresh branch from main because PR #41 merged. Reviewers, once, on the final diff: netcode-architect (parts 10, 11, 13, -31, check item 11, the record), test-engineer (PM-0001-18's checks and the tolerance's statistics, -31), security-auditor (OQ-0001-19's credential wording), and the verifier (numbers, kernel facts, cites).

### E0: ADR 0001 part 10, line 424 (the sentence inside the line). Replace the sentence only. Re-wrap from "HTTP/2." on line 424 through the existing "Inputs, snapshots" that ends it into new lines (4-space indent, at most 120 columns), so the last new line still ends with "Inputs, snapshots" and line 425 onward stays untouched.

**Old text:**

```
Over WebSocket the server sends only the newest snapshot and drops any unsent older one.
```

**New text:**

```
Over WebSocket the server sends only the newest snapshot and never lets snapshots queue behind TCP: it writes a tick's snapshot, all its messages, only while the bytes the client has not yet acknowledged (written but unsent, and in flight: the socket's SIOCOUTQ, plus any still held above the socket) leave room for it within N snapshots of its size, N being 1.5 × the smoothed round trip (the socket's TCP_INFO; 100 ms before its first sample) ÷ the tick period, rounded up, plus 1, and at least 3; a snapshot that does not fit is skipped and counted, never sent later, and the next tick tries with its own (Mustafa, 2026-11-07: "Measure fix 1 (the newest-snapshot-only buffer) in the proof test"). In the netcode proof test the stream wrapper's model of TCP (part 13's key) gives both readings; by that model the first fresh snapshot after B1 arrives a median 1.12 s after the link comes back with a 64 KB send buffer of stale snapshots, and 0.65 s with this budget (netcode proof test commit bf9d248).
```

**Why:** Fix 1 is Mustafa's decision. The old sentence says the server "drops any unsent older one", which a 64 KB kernel buffer of stale snapshots satisfies and which no longer describes fix 1: fix 1 never drops a written snapshot; it skips the one that does not fit (verifier V17). The formula is fixes.rs:163-169 (adaptive_limit; 100 ms before the first sample at :166) and the write rule :776-777. The kernel facts are checked in Linux v6.8 (verifier V12): SIOCOUTQ = write_seq - snd_una, unsent plus unacknowledged (net/ipv4/tcp.c:630); tcpi_rtt = srtt_us >> 3 (:3842). The medians are bar-fix1-bf9d248.txt:7-8 (fix 1 p50 654, today 1,121), which I reproduced (F3).

### E1: ADR 0001 line 1556. Replace the PM-0001-18 row whole (it matches the line exactly). Supersedes revision 1's E1.

**Old text:**

```
| PM-0001-18 | A loss burst heals (part 6) | after B1's burst, every client decodes deltas again within 500 ms of the link coming back (a plain limit per client and burst) | L1; L2 with LG | B1 with 48 clients, 10 bursts per run (proposed) | L1: 3 seeds; L2: 1 run; every client after every burst. Bites: a baseline that never falls back (the advisor's reading, 2026-11-06) | r2 R2-SYNC-REC1 mark 2 (line 2020); the draft, "Failure modes", row "A burst of total loss (B1)" |
```

**New text:**

```
| PM-0001-18 | A loss burst heals (part 6) | per client and burst after B1's burst: the time from the link coming back until the client decodes a delta encoded at or after that moment, so deltas queued during the burst do not count (the advisor's reading, 2026-11-06). On WebTransport, every client within 500 ms (a plain limit). On the WebSocket fallback, with part 10's send budget, three checks over the run's client-bursts: the median at most X, at most k of them over Y, none over Z (Mustafa, 2026-11-07: "WS-BAR: the three-part form now; its numbers after one practice run on the real test setup, but never looser than the model predicts for a correct build with fix 1"). The bound: the values a correct build with the send budget stays within by the proof test's model of Linux's TCP (part 13's key), computed for the run as it runs (its snapshot sizes, 48 clients sharing each burst, each client's bursts in turn), in 99 % of runs for the median, 98 % for the count over Y and 98 % for the maximum (the advisor's reading, 2026-11-07); first bound, for 480 independent single bursts with 1,050 B snapshots: half within 0.67 s, at most 5 over 2.5 s, none over 6.9 s (model `linux-6.8-tcp v3`, netcode proof test commit bf9d248). The numbers come from that bound and one practice run, with a tolerance of 10 % fixed before it (Mustafa, 2026-11-07: "If the practice run is up to 10 % slower than the model, the numbers may move by that much; beyond 10 %, investigate first and never loosen"): the practice run's slowdown is its median divided by the model's median run median for the same run (the 50th percentile over the bound's simulated runs, which also give how much a correct build's slowdown varies); up to 1.10, X, Y and Z are multiplied by it and k stays, the bound for a build that much slower; at or below 1 the bound stands, never tightened from one run; beyond 1.10, or failing the count or maximum check at the moved numbers, the cause is found before any number is set and nothing is loosened (Mustafa, 2026-11-07: "If the practice run comes out worse than the model, find out why before setting the numbers"), and a practice run faster than a correct build's spread is explained too; the tolerance may only be lowered, before the practice run, if the bound shows a send budget twice the formula's coming out within it in more than 1 % of runs (the advisor's reading, 2026-11-07). Reported on WebSocket, not judged: p99, p99.9, the unacknowledged bytes at each return, and the time until a decoded snapshot's extra delay is back under PM-0001-31's 250 ms. The client's fast reconnect over WebSocket is not measured here (OQ-0001-19) | L1 (WebTransport's shape); L2 with LG on each transport | B1 with 48 clients, 10 bursts per run (proposed), each burst's start moved by a random offset of 0 to one tick period against the room's tick (the advisor's reading, 2026-11-07); the WebSocket run through the accept path's delay wrapper (part 13's key), whose clients run Linux's TCP as the model's do | L1: 3 seeds; L2: 1 run per transport, the WebSocket one after its practice run with the same build, which is not judged; every client after every burst (480 records a run). Bites: on WebTransport, a baseline that never falls back (the advisor's reading, 2026-11-06); on WebSocket, a send budget twice the formula's (the advisor's reading, 2026-11-07) | r2 R2-SYNC-REC1 mark 2 (line 2020), WebTransport's 500 ms; the WebSocket checks, their practice run and its tolerance, Mustafa's decisions (2026-11-07); the draft, part 10 (the send budget) and "Failure modes", row "A burst of total loss (B1)" |
```

**Why:** Mustafa's split (WebTransport keeps 500 ms; WebSocket the three-part form) with both his rules: the practice run (13:45) and the 10 % tolerance fixed before it (14:30). The bound's 1 % / 2 % / 2 % definition is ws_bar_fix1's (fixes.rs:2000-2008). How the slowdown is measured and how the numbers move: G7-G12 and rulings R4-R6. Y = 2.5 s sits above the backed-off-timeout cluster (2.0-2.5 s holds 2.14 % of phases, tables-1e020a7.txt:124; V27). The first bound is bar-fix1-bf9d248.txt:11, which I reproduced. The row states no noise figure, since +-3.5 % is my arithmetic (G8); the bound prints the real band. The no-fallback bite stays off WebSocket (F7). The row hash changes (G6).

### E2: ADR 0001 line 747, the failure-modes row. Replace it whole (exact line match). Supersedes revision 1's E2.

**Old text:**

```
| A burst of total loss (B1) | the client holds; on return the server deltas from an older baseline or the defaults | deltas again within 500 ms (R2-SYNC-REC1 mark 2) |
```

**New text:**

```
| A burst of total loss (B1) | the client holds; on return the server deltas from an older baseline or the defaults; over WebSocket TCP resends what the burst lost, and part 10's send budget keeps stale snapshots from queueing behind it | deltas again within 500 ms on WebTransport (R2-SYNC-REC1 mark 2); on the WebSocket fallback PM-0001-18's three checks, set from a model of Linux's TCP and one practice run (Mustafa, 2026-11-07); the real client's fast reconnect over WebSocket is required before the first online playtest (OQ-0001-19) |
```

**Why:** This row is the source PM-0001-18 cites and must not contradict it. It also names the reconnect where the failure is described.

### E3: ADR 0001 line 1295, "What a spike must prove" item 2. Replace the phrase; re-wrap only item 2's lines at 120 columns (3-space indent) and keep every other word.

**Old text:**

```
deltas again within 500 ms after B1,
```

**New text:**

```
deltas again within 500 ms after B1 on WebTransport and within PM-0001-18's three checks on the WebSocket fallback (Mustafa, 2026-11-07),
```

**Why:** The heal limit is stated at 747, 1295 and 1556, and in Mustafa's check item 11 (1697-1699, now E13). Revision 1 called 1295 "the third and last place" and missed item 11 (verifier V9).

### E4: ADR 0001 part 13's key, R-25's sentence. It runs from the last word "For" on line 1462 through "(the advisor's reading, 2026-11-06)." on line 1469; the "The" that ends line 1469 stays. It matches once after whitespace normalisation (line breaks and indents as one space). Re-wrap only these lines (2-space indent, at most 120 columns).

**Old text:**

```
For the proof test, the WebSocket delay is a stream wrapper inside the server's accept path, so the room sees real source addresses. Per direction: segments of at most 1,448 B; each delayed by the profile's one-way delay plus jitter, and delivered at max(the previous segment's delivery, its own time), so order holds; each lost at the profile's rate on its own, then delivered one retransmission timeout later: max(200 ms, 2 × the profile's round trip; U3 verifies the floor), holding back every later byte; the write side bounded at 64 KB, returning Pending beyond. Stalls are counted per run. The Join deadline runs from TCP accept. Bytes buffered per connection are capped and counted, and the message size limit applies before buffering (the advisor's reading, 2026-11-06).
```

**New text:**

```
For the proof test, the WebSocket delay is a stream wrapper inside the server's accept path, above TLS and below the WebSocket library (U3-Q4 (b), Mustafa, 2026-11-06), so the room sees real source addresses. Per direction it models Linux's TCP as Ubuntu 24.04's 6.8 kernel runs it with default settings (Mustafa, 2026-11-06: "model Linux TCP faithfully, so the result shows how the fallback really behaves"; the model `linux-6.8-tcp v3`, its rules read in the kernel's source and checked line by line in three reviews): segments of at most 1,448 B, each delayed by the profile's one-way delay plus jitter and lost at the profile's rate on its own, and acknowledgements, delayed and selective as Linux sends them, crossing the other direction with their own delay and loss. A lost segment is resent when Linux would resend it: by RACK (RFC 8985), once a later segment is acknowledged and a recent round trip plus a reordering window has passed since its send; by a tail-loss probe two smoothed round trips after the last send, plus 200 ms when only one segment is out (else 2 ms); or at the retransmission timeout, the smoothed round trip plus its variance, the variance never below 200 ms (about 300 ms at P2), doubling at each consecutive expiry, after which the sender restarts in slow start from the segments still in flight plus one; F-RTO, the timestamp option, the loss undos and Cubic's window as Linux has them. Every resend draws its own fate, and bytes are delivered in order, so a lost segment holds back every later byte; the stream never reorders (the reorder step below is the datagrams'). The write side holds at most 64 KB unsent or unacknowledged, returning Pending beyond, and gives part 10's send budget its smoothed round trip and those bytes, as TCP_INFO and SIOCOUTQ give them to the real server. Stalls are counted per run, by repair path, and every WebSocket run's header names the model; U3 checks the model against the kernel on loopback and R1 against RS's own kernel, its version and CONFIG_HZ recorded. The Join deadline runs from TCP accept. Bytes buffered per connection are capped and counted, and the message size limit applies to whole frames before buffering (the advisor's reading, 2026-11-06).
```

**Why:** The key still describes the old constant model (max(200 ms, 2 x RTT)), which Mustafa's U3-Q3 (b) replaced; PM-0001-18's WebSocket bound and PM-0001-31 rest on this text. The rules are tcp_model.rs:1-73; the tail-loss probe is corrected to tcp_model.rs:40-41 (V28); the size-limit sentence stays where the key has it today, since it is the wrapper's duty (u3-build-plan.md:40-42), not a consequence of the placement (V28). The placement is U3-Q4 (b) (checkpoint :391-392 his agreement, :400-401 the reading, at 95a04a1). Lines 1461-1462 and 1757 stay (G15).

### E5: ADR 0001 part 11, the *Reconnect:* clause (lines 506-509). Insert one sentence after "...the record is ADR 0004's)." and before "*Private rooms:*". Re-wrap only the touched lines (4-space indent).

**Old text:**

```
OQ-0004-10 (GD-REJOIN: Mustafa's, with OQ-0005-AB10; the record is ADR 0004's). *Private rooms:*
```

**New text:**

```
OQ-0004-10 (GD-REJOIN: Mustafa's, with OQ-0005-AB10; the record is ADR 0004's). Over the WebSocket fallback the client also reconnects without waiting out TCP's backed-off retransmissions: a requirement of the real client, built and measured before the first online playtest (migration step 5, slice 3) with today's model as its target, 99 % of client-bursts decoding a fresh delta within about 1.05 s of a 1 s outage at P2 (Mustafa, 2026-11-07); its design, the credential an attempt presents (RESUME-CRED, "deferred with the reconnect": Mustafa, 2026-11-07) and what it changes in this part and in part 10's per-address cap wait for that work (OQ-0001-19; [the reconnect record](../design/ws-reconnect.md)). *Private rooms:*
```

**Why:** Mustafa: "Record that in the docs PR so it can't get lost." The rule a reconnect changes lives here ("the client fetches a fresh token; the newest connection for a player id wins"), so the requirement sits beside it, and this link is ADR 0001's pointer to the record (architect's open point 4; G20). Slice 3: G17. The clause is in part 11, not part 10 (F2).

### E6: ADR 0001, Open questions: a new entry OQ-0001-19 after OQ-0001-18 and before "## What a spike must prove". The old text is the anchor (OQ-0001-18's last line, the blank line, the heading); the new text puts the entry between. Wrap the entry at 120 columns with the 2-space continuation indent of OQ-0001-17 and -18 (bullets follow each other with no blank line).

**Old text:**

```
  (R2-ROOM-REC2 mark 4: a room process that dies ends only its own match).

## What a spike must prove
```

**New text:**

```
  (R2-ROOM-REC2 mark 4: a room process that dies ends only its own match).
- **OQ-0001-19. How does the client reconnect fast over the WebSocket fallback, and with which credential (WS-RECONNECT, RESUME-CRED)?** Mustafa (2026-11-07): "The reconnect becomes a requirement for the real client, built and measured before the first online playtest, with today's model as its target (buffer + reconnect: 99 % within about 1.05 s)." and "RESUME-CRED: deferred with the reconnect." After B1 over WebSocket, TCP's backed-off retransmissions leave 1 % of client-bursts more than 2.16 s from a fresh delta even with part 10's send budget; a fresh connection opened once the client has heard nothing for a short while, one attempt at a time, its session resumed on the newest `Join`, brings that to about 1.05 s by the proof test's model (a 1 s outage at P2 over 600,000 burst phases, a Linux client; the budget alone: p50 0.65 s, p99 2.16 s, max 11.4 s; the budget and the reconnect: p50 0.65 s, p99 1.05 s, p99.9 1.86 s, max 4.2 s; model `linux-6.8-tcp v3`, the netcode proof test's rows printed at commit 1e020a7 and unchanged at bf9d248). · *Why it matters:* how long a WebSocket player stays frozen after a short outage; the proof test measures the send budget alone (PM-0001-18). · *Options:* the trigger, the attempts, the takeover and when an attempt is abandoned (a policy more robust across outage lengths measured a p99 of about 1.13 s at the 1 s outage in a smaller run; whether that meets "about 1.05 s" is Mustafa's call in that work); for the credential, a fresh lobby token per attempt (part 11 today) or spare single-use lobby tokens held in memory only, short-lived and bound to the session (Mustafa's earlier answer, 2026-11-06, now deferred), Q22 having weighed a room-issued resume secret (option B) and chosen fresh tokens (option A); each weighed in [the reconnect record](../design/ws-reconnect.md), with what the reconnect changes in part 11 (the newest connection wins) and part 10 (the per-address connection cap). · *Recommendation:* none yet (that work's). · *Decider:* Mustafa (the credential, and the target if the chosen policy misses it), netcode-architect with security-auditor (the design), the Advisor confirming. · *What decides it:* the reconnect built in the real client (migration step 5, slice 3) and measured before the first online playtest against the target, with the loss applied below TCP so handshakes are lost too, on each browser engine and on Windows' TCP (its first SYN retry after 1 s). The trace is part 10's fallback: VISION_V2 §6.1; §4 (four browsers). · *Research ids:* R2-ROOM-REC4 (the rejoin it speeds up); Q22.

## What a spike must prove
```

**Why:** An open question is the register's tracked home for a deferred decision, and it is counted in docs/adr/README.md, so it can't get lost. The numbers are tables-1e020a7.txt:123 (fix 1 alone; revision 1 said :125, V5) and :129-131 (recommended); the pin passes at 5a5aba9 (F4). "One attempt at a time" is the recommended row's policy (fixes.rs:1259-1263; V18). The robust policy's p99 is the architect's F20 (u3-verify3-harness.md:122-129; G18). The measurement note follows round 2's H17 and round 3's V19 (u3-verify3-harness.md:187-195): L2 cannot measure a reconnect. The Windows SYN fact is the checkpoint's "Facts from Mustafa" (:336-346 at 95a04a1).

### E7: ADR 0001 migration plan step 5 (slice 3), the *Done when* sentence, lines 1737-1738. Replace the phrase; re-wrap the step's lines only (3-space indent).

**Old text:**

```
and the byte-level audit (R2-INT-REC5) pass, and Mustafa's first online playtest is run.
```

**New text:**

```
and the byte-level audit (R2-INT-REC5) pass, the WebSocket fallback's fast reconnect is built and measured against OQ-0001-19's target before the first online playtest (99 % of client-bursts with a fresh delta within about 1.05 s of a 1 s outage at P2; Mustafa, 2026-11-07; [the reconnect record](../design/ws-reconnect.md)), a miss going to Mustafa before that playtest, and Mustafa's first online playtest is run.
```

**Why:** Mustafa's words are "built and measured before the first online playtest, with today's model as its target", not "meets" (verifier V19; the record's section 3.7 agrees that a miss goes to him). Done-when makes the requirement gate the playtest instead of being a note. Slice 3: ADR 0001:1734-1738 already lists "reconnect" and "Mustafa's first online playtest" (G17).

### E8: ADR 0001 line 1571. Replace the PM-0001-31 row whole (exact line match). Revision 1 changed only its hardware cell.

**Old text:**

```
| PM-0001-31 | The fallback's stalls stay short (part 10) | over WebSocket at P2: p99 extra snapshot delay at most 250 ms (extra delay: a snapshot's age when decoded minus the profile's one-way delay, 50 ms; proposed definition); the server never holds more than one unsent snapshot per client (its max, sampled every tick) | L2's TCP proxy on RS; LG; ML (Chrome) | 10 minutes at P2 | 1 run. Bites: a server that queues every snapshot over WebSocket | r2 R2-TRN-REC3 mark 3 (lines 2668–2669: "100 ms RTT and 1 % loss", restated as P2 by the draft, "What a spike must prove" item 1) |
```

**New text:**

```
| PM-0001-31 | The fallback's stalls stay short (part 10) | over WebSocket at P2: p99 extra snapshot delay at most 250 ms (extra delay: a snapshot's age when decoded minus the profile's one-way delay, 50 ms; proposed definition); the server never holds more than one unsent snapshot per client (its max, sampled every tick; unsent means not yet written to the socket, held by the room or its WebSocket library; what part 10's send budget lets into the socket is bounded by that budget, not by this count: the advisor's reading, 2026-11-07) | L2's stream wrapper in RS's accept path (part 13's key); LG; ML (Chrome) | 10 minutes at P2 | 1 run. Bites: a server that queues every snapshot over WebSocket, the ones part 10's send budget skips included | r2 R2-TRN-REC3 mark 3 (lines 2668–2669: "100 ms RTT and 1 % loss", restated as P2 by the draft, "What a spike must prove" item 1) |
```

**Why:** After fix 1, "unsent" was ambiguous (verifier V17): read as the kernel's unsent bytes, a correct fix-1 build fails it; read as held above the socket, it stays meaningful and its bite must target the skipped snapshots (G13). The hardware cell is the PR Advisor's Low note on #41 (F12): -31's L2 leg runs in R3, after this PR, and E4 rewrites the key it points to. The row hash changes (G6).

### E9: docs/adr/README.md as on main (the branch's copy lags PR #43; G4): row 0001 (line 16), the Total sentence (line 23) and its list (line 27). Three edits, separated by "||". The session recounts by script before committing.

**Old text:**

```
18 numbered (OQ-0001-1 to -18), 12 open: || Total: 118 numbered open questions, 97 of them open || OQ-0001-9 and OQ-0006-16 in part, 2026-11-06);
```

**New text:**

```
19 numbered (OQ-0001-1 to -19; -19, WS-RECONNECT, added 2026-11-07), 13 open: || Total: 119 numbered open questions, 98 of them open || OQ-0001-9 and OQ-0006-16 in part, 2026-11-06; OQ-0001-19, WS-RECONNECT, is new from Mustafa, 2026-11-07);
```

**Why:** The register's counts must follow the drafts' entries (README :23-31); PR #41's C1 was exactly this kind of miss. Counts confirmed by the verifier (V8).

### E10: docs/adr/README.md as on main, the decisions table: a new row after the ROOM-CRASH (OQ-0001-18) row (line 87). The old text is that row's unique end; the new text keeps it and adds the new row on the next line.

**Old text:**

```
| the Advisor | [ADR 0001](0001-multiplayer-netcode.md) OQ-0001-18 |
```

**New text:**

```
| the Advisor | [ADR 0001](0001-multiplayer-netcode.md) OQ-0001-18 |
| WS-RECONNECT and RESUME-CRED (OQ-0001-19) | How does the client reconnect fast over the WebSocket fallback, and which credential does an attempt present? A requirement of the real client, built and measured before the first online playtest (Mustafa, 2026-11-07: "The reconnect becomes a requirement for the real client, built and measured before the first online playtest, with today's model as its target (buffer + reconnect: 99 % within about 1.05 s)."); the credential waits for that work ("RESUME-CRED: deferred with the reconnect.") | Mustafa (the credential, and the target if the chosen policy misses it), netcode-architect with security-auditor (the design), the Advisor confirming | [ADR 0001](0001-multiplayer-netcode.md) part 11 and OQ-0001-19; [the reconnect record](../design/ws-reconnect.md) |
```

**Why:** The table lists every decision code and open question the drafts route to Mustafa or the Advisor, one row each (README :40). RESUME-CRED is his.

### E11: docs/design/ws-reconnect.md (new file in this docs PR), from netcode-architect's draft .checkpoints/netcode-proof-test/ws-reconnect-record-draft.md. Place approved: docs/design/, beside the proof test's catalogue; linked from part 11 (E5), OQ-0001-19 (E6), migration step 5 (E7) and the register (E10).

**Old text:**

```

```

**New text:**

```
Required contents and corrections (the verifier checks each against its source):
(1) Mustafa's words with their dates: the 13:45 sentences on the reconnect and RESUME-CRED word for word; where the record borrows or describes WS-BAR's rule (sections 2.3 and 6.4), the 14:30 refinement word for word too. Each cited to the checkpoint at one pinned commit, checked there (at 95a04a1: 14:30 at :349-356, 13:45 at :357-362; at 803c6c1 the 13:45 words are :349-354).
(2) The target and its row: model linux-6.8-tcp v3; fixes.rs "recommended" row; 600,000 phases; P2; a 1 s outage; p50 654, p99 1,051, p99.9 1,859, p99.99 2,092, max 4,196 ms; 6,768 over 1 s. Cited as spikes/netcode-proof/results/u3-ws-fallback/tables-1e020a7.txt:129-131 (branch claude/netcode-proof-test-plan-hcky5g, 975e72b), printed at 1e020a7 and unchanged at bf9d248 (the pin passes). For information, the Windows-like client rows (no timestamps; first SYN retry after 1 s): p99 886 ms on 60,000 phases (tables :102, :110).
(3) The design as modelled, each item with its fixes.rs line: the trigger max(300 ms, 2 x the echo RTT + 100 ms) (:171-174); ONE attempt at a time on one endpoint, at most one every 300 ms, never closed by a timer, at most 8 per outage (serial(), :1187-1197; the row :1236-1239; recommended(), :1259-1263); an attempt still before its 101 closed once the old connection delivers fresh data (H6, :706-734); the session resumed on the newest Join (:736-790); Chromium's endpoint lock and throttler, Firefox's fail delay. Parallel attempts are reference only: no browser runs them (:1252-1254; tables :96). The round-3 alternatives: the scaled abort max(500 ms, 4 x RTT) with join-or-close, better at 0.6, 1.5 and 2.5 s outages (u3-verify3-harness.md V13, :130-140; join-or-close V11, :109-121; results/u3-ws-fallback/reconnect-policies-1e020a7.patch). Revision 1's "attempts every 300 ms in parallel" was wrong (verifier V18); the draft's sections 1.2 and 3.2 have it right.
(4) The open choices: RESUME-CRED's options and its three readings of "bound to the session"; the attempt policy, and whether the robust policy's p99 (about 1.13 s at 1 s, 20,000 phases) meets "about 1.05 s" (Mustafa's call in that work); part 11's "newest connection wins"; part 10's per-address cap against reconnect attempts; the 2 s Join deadline per attempt.
(5) How it must be measured: loss below TCP, so SYN and TLS flights are lost; real browsers (each engine) and Windows' TCP; L2's DelayStream cannot (H17, V19).
(6) The owner and the time: migration step 5, slice 3, before the first online playtest; measured against the target; a miss goes to Mustafa. Its bar is that work's: section 6.4's form stays "proposed, Mustafa to confirm (WS-RECONNECT-BAR)", and if it borrows WS-BAR's rule, the 14:30 tolerance comes with it.
(7) What the proof test measures instead: PM-0001-18's WebSocket part, the send budget alone, with its practice run and tolerance; point to the ADR row rather than restating its rule, quoting only the first bound.
(8) The trace: VISION_V2 §6.1 and §4.
(9) Sources: branch and SHA for every spike file; outputs cited at spikes/netcode-proof/results/u3-ws-fallback/ (975e72b), never .gate/; every .checkpoints/ cite pinned to one commit and checked at it.
(10) Section 5's table says what this docs PR already changes and what the reconnect work changes later (verifier V20): here, 424 (fix 1), 506-509 (the requirement sentence and the link), the new OQ-0001-19, 747, 1295 and 1556 (the WebSocket split), 1462-1469 (part 13's key), 1571 (-31), 1697-1699 (check item 11), 1734-1738 (Done-when) and the register rows; later, the rest of its rows. Line numbers as before this PR, each with its quoted words, since this PR shifts lines; "422-423" is 423 alone (V25).
(11) Wording and cites: section 1.1's fix 1 reads "a snapshot that does not fit is skipped and counted, never sent later", not "older unsent snapshots are dropped" (V25, V17); section 6.1's U3-Q4 (b) cite pinned (at 803c6c1 :392-393, at 95a04a1 :400-401; V21); ws_bar_fix1 at fixes.rs:2000-2011 (doc :2000-2008, fn :2011; V25).
```

**Why:** Mustafa wants the requirement impossible to lose. The ADR carries the rule and the gate; the record carries the detail slice 3 needs, so the design work does not have to rediscover three skeptic rounds. The corrections are the verifier's V18, V20, V21 and V25, and the architect's own open points.

### E12: docs/REPO_MAP.md and docs/STATUS.md (docs-keeper, in this PR).

**Old text:**

```

```

**New text:**

```
REPO_MAP: a docs/design/ line ("design records: ws-reconnect.md, the WebSocket fallback's fast-reconnect requirement, ADR 0001 OQ-0001-19"), and node .claude/scripts/repo-map-check.mjs OK. STATUS: the netcode proof test's line gains Mustafa's 2026-11-07 decisions: the proof test measures the send budget alone; the reconnect is a real-client requirement before the first online playtest (OQ-0001-19, docs/design/ws-reconnect.md); RESUME-CRED deferred; WS-BAR in the three-part form, its numbers from the model's bound and one practice run with a 10 % tolerance fixed before it. Also the counts 119 numbered and 98 open.
```

**Why:** Docs-keeper keeps the map and status true in the same PR. The REPO_MAP edit conflicts once with PR #40's docs/design entry; resolve it at the spike branch's next merge of main (F13).

### E13: ADR 0001, Mustafa's check item 11 (lines 1697-1699). Replace the sentence's end; it matches once after whitespace normalisation. Re-wrap item 11's lines only (4-space continuation indent, at most 120 columns), keeping "His own soldier may be corrected once ..." and the rest of the item.

**Old text:**

```
and his own soldier keeps responding; within half a second after it, the world catches up.
```

**New text:**

```
and his own soldier keeps responding; within half a second after it, the world catches up on WebTransport; on the WebSocket fallback (step 1's overlay names it), half the time within about two thirds of a second, about one burst in eleven after more than a second and one in forty after more than two (TCP resends what the burst lost; PM-0001-18; the fast reconnect that shortens the slow ones is required before the first online playtest, OQ-0001-19).
```

**Why:** Item 11 stated the 500 ms heal for both transports (verifier V9), and PM-0001-53 (:1599) judges every step, so a WebSocket session would read "not yet" for a correct build. The numbers are fix 1 alone over 600,000 phases (tables-1e020a7.txt:123; bar-fix1-bf9d248.txt:7): p50 654 ms; over 1 s 52,406 (8.7 %, one in 11.4); over 2 s 15,226 (2.5 %, one in 39). It is Mustafa's own check, so the PR asks him to read it before his go. No mark hash changes (G6).

## Field: plan_and_marks_changes

- ORDER: (1) The docs PR opens from main (claude/npt-readings, worktree .gate/npt-readings) and goes needs-mustafa. (2) Meanwhile U3c and the run-structured bound (fixes.rs, below) proceed on the spike branch. (3) After the docs PR merges on Mustafa's go: merge origin/main into the spike branch and, in the same commit, re-take PM-0001-18's and PM-0001-31's row hashes in marks.json and move drafts_commit to the merged main's full SHA (marks.rs:773-775 needs a full commit; verifier V30); check.mjs must pass all 9 steps. (4) R3's run plan goes to Mustafa only with the run-structured bound committed, its guard passed and the tolerance in marks.json.
- marks.json PM-0001-18, parts: {"name":"L1 B1","select":{"layer":"L1","profile":"B1"},"min":3} (WebTransport's shape); {"name":"L2 B1 wt","select":{"layer":"L2","profile":"B1","machine.role":"LG","params.transport":"wt"},"min":1}; {"name":"L2 B1 ws","select":{"layer":"L2","profile":"B1","machine.role":"LG","params.transport":"ws"},"min":1,"validity":["ws-practice"]}. The transport values are "wt" and "ws", as PM-0001-25 uses them.
- marks.json PM-0001-18, checks: (a) "WebTransport: every client decodes a fresh delta within 500 ms of the link coming back": event.burst_heal.after_link_ns, max <= 500000000, parts L1 B1 and L2 B1 wt. (b) "every client after every burst: 48 x 10 heal records": count >= 480, every part (a client that never heals writes no record). (c) "WebSocket: half the client-bursts within X x f": p50 <= X x f, part L2 B1 ws. (d) "WebSocket: at most k over Y x f": count where value > Y x f, <= k. (e) "WebSocket: none over Z x f": max <= Z x f. f = 1 when the practice run's slowdown s is at most 1; f = s when 1 < s <= the tolerance; otherwise the counted run is INVALID (ws-practice). X, k, Y, Z and the tolerance (1.10, or lower by the guard) come from the run-structured bound's committed output, named in marks.json by path and commit. Preferred: the judge reads f from the practice run's outcome record and applies it, so marks.json is fixed before R3 and nothing is edited mid-run. Fallback: the run script writes X x f, Y x f and Z x f (each rounded up to the next millisecond) and the session commits them before the counted run; the ws-practice rule re-checks the arithmetic. Until the bound is computed, bf9d248's 670000000 / 5 over 2500000000 / 6900000000 sit there as placeholders, named "provisional" in each check name and in notes. The judge supports p50, count-with-where and max (marks.rs:419-429, :466-470; verifier V16).
- marks.json PM-0001-18, report lines: p50 and p99 for every part; on the WebSocket part also p999, the count over 1 s, the server's per-return series (unacknowledged bytes, the budget, srtt), and the practice run's outcome (M_p, M_m, s, the band, f, the tolerance). Bites: keep "PM-0001-18/no-fallback" for the WebTransport parts only (L1 B1, L2 B1 wt); add "PM-0001-18/ws-budget-x2" ("a send budget twice the formula's", the budget_scale 2 knob), which must fail check (c) on the WebSocket part. Notes: rewrite to the new row (fresh delta, the practice run and tolerance, the per-transport runs, the Linux-client caveat).
- New judge validity rule "ws-practice" (test-engineer; README §4; PASS and INVALID fixtures): a run of "L2 B1 ws" is INVALID unless the results set holds its practice run: role "practice", the same build hash, scene, profile and burst spacing, earlier in time, listing PM-0001-18 so the judge keeps it (marks/README.md:25 and §3 step 1 keep only runs whose header lists the mark; V16) but never judging it; that run's outcome record says IN TOLERANCE, or names my ruling of chance (rulings R8 (v)); and the numbers in force equal the bound's X, Y, Z multiplied by f from that record (rounded up to the next millisecond) with the bound's k, recomputed by the judge (a mismatch is INVALID, so no number is set by hand). One practice run counts per build: a second needs a ruling naming a found and fixed cause. The practice run supersedes nothing and stays in the results ledger as "practice (WS-BAR)". If the judge cannot carry the rule, the run script enforces it, with a test; the judge is preferred.
- The practice run's outcome record (run script, written beside the run, from its series and the bound's committed output): M_p (the run's p50 over its 480 client-bursts, nearest rank), M_m (the bound's median run median), s = M_p / M_m, the band (the 1st and 99th percentiles of a correct build's run median / M_m), the tolerance, f, the moved X, Y, Z and k, the count and maximum checks at the moved numbers, the record count, the mean WebSocket snapshot message size against the bound's, the header checks (model, RS's kernel and CONFIG_HZ, the burst schedule as run), and the class: IN TOLERANCE, BEYOND, FAR BETTER or INVALID (rulings R7). A unit test with a fixture for each class and the boundaries s = 1.00, 1.10 and 1.1001.
- SERIES.md: (1) event.burst_heal.after_link_ns (SERIES.md:134): "per client per burst: from the link's return to the decode of the first snapshot the room writes at or after the return instant" (V29; fixes.rs:786-787). (2) Add event.burst_heal.burst (the burst's index, 0-9). (3) Add a server-side event.burst_return written by npt-server per burst: offset_ns (the random 0 to one-tick-period offset drawn from the run's seed) and tick_phase_ns (the return minus the room's last tick start); and per WebSocket client per burst: outstanding_bytes (DelayStream's unsent plus unacknowledged bytes at the return, plus any held above it), srtt_ns, budget_bytes and snap_bytes (the last written snapshot's size). (4) Add tick.v.ws_skipped_snaps_count (snapshots the send budget skipped this tick). (5) Header: params.ws_tcp_model ("linux-6.8-tcp v3", tcp_model.rs:79 MODEL), machine.kernel (RS's uname -r), machine.config_hz, and role "practice". (6) tick.v.unsent_snaps_max_count (SERIES.md:56): "the most snapshots any client has waiting above the socket (not yet written to it, held by the room or its WebSocket library), sampled each tick". (7) The transport row (SERIES.md:356) gains -18 (V30). Test-engineer finalises the names under README §1's rule.
- marks.json PM-0001-31: the check's name "never more than one snapshot per client waiting above the socket (not yet written to it; max, sampled every tick)"; the bite PM-0001-31/ws-queue-all's text "a server that queues every snapshot over WebSocket, the ones the send budget skips included"; notes name the stream wrapper and the definition; its row hash re-taken after the merge (ORDER (3)).
- Plan §6.2: split the "-17, -18, -20" row so -18 reads "C (L1, WebTransport's shape) + R3 (VS4: L2 with LG, one run per transport; on WebSocket the unjudged practice run first, then the counted run late in VS5)". Plan §7.2 R3 (plan.md:737): VS4 -18 0.2 -> 0.5 (WebTransport run 0.2, WebSocket practice run 0.15, WebSocket counted run 0.15); VS5's bites 0.85 -> 0.95 (+0.1, ws-budget-x2; VS4's bites stay 0.85); total 10.15 -> 10.55 h, billed 11, EUR 1.72 planned and 1.90 cap unchanged. Plan §7.1 (V27): "ADR 0001's rows, measured" R3 6.55 -> 6.85, total 19.3 -> 19.6 (plan.md:720); "Bites" R3 1.7 -> 1.8, total 5.7 -> 5.8; "Rounding slack" R3 0.85 -> 0.45, total 5.3 -> 4.9 (plan.md:729); the grand total stays 75.0. Order rule: the practice run early in VS4 (after bring-up, the two L2 probes and the clean references); the counted WebSocket run and its bite late in VS5. The run script classifies the practice run mechanically between them; unless it is IN TOLERANCE, the counted WebSocket run and its bite are not run in R3 (0.25 h unused) and move to a later run day's slack (R4 has 0.95 h) after the advisor rules on the investigation.
- R3's run plan (ops/runs/R3.md): 10 bursts per run; each burst's start = the schedule's instant + an offset drawn uniformly over one tick period (33,333,333 ns) from the run's seed, recorded per burst; the spacing the bound was computed with (20 s); RS's kernel version and CONFIG_HZ recorded in the header against "linux-6.8-tcp v3" (if RS is not on 6.8 or CONFIG_HZ is not 1,000, the advisor rules before the WebSocket part counts; S14's kernel check at R1 stands); the room's per-address limits raised for LG's one address so 48 WebSocket clients connect; the tolerance, the bound's commit and the moved-numbers table (f = 1.00 to 1.10) printed in the plan, so Mustafa sees the rule before the run.
- fixes.rs (before R3's run plan; in U3c or its own small unit): extend ws_bar_fix1 into the run-structured bound. It simulates whole runs, at least 2,000 (at least 960,000 client-bursts): 48 clients x 10 bursts as lineages at the run plan's spacing; all 48 share each burst's instant; each burst gets an offset uniform over one tick; each client's input phase hashed (splitmix64, as now); the WebSocket message size from L1's B1 runs of the same scene (payload + WebSocket frame header + TLS record overhead), replacing SNAP = 1,050 (fixes.rs:34). Output: X (the smallest 10 ms step with P(run median <= X) >= 0.99), k at Y = 2.5 s (P >= 0.98), Z (the smallest 100 ms step with P(none over Z) >= 0.98); M_m (the 50th percentile of the run median); the band (the 1st and 99th percentiles of run median / M_m); the guard (the share of simulated budget-x2 runs whose run median / M_m is at most 1.10; if above 0.01, the largest whole-percent tolerance at which it is at most 0.01); the moved numbers for f = 1.00 to 1.10 in steps of 0.01; the mean message size used; pass chances of the correct build, budget x2 and today; sensitivity rows (x1.05, +20 ms). Commit the output under spikes/netcode-proof/results/u3-ws-fallback/ with one machine-readable line the run script and the judge read. Also: pin one bite row (stats B5) and fix or remove the stale three_part() bar (fixes.rs:1528-1537; stats F11).
- U3c (DelayStream): (1) Expose pub(crate) srtt_ns() and outstanding_bytes() for each WebSocket connection's down direction (tcp_model.rs:2168-2177 already has them on the model); outstanding_bytes = the model's unsent + unacknowledged bytes; WsServer adds what it holds above the stream (tungstenite's pending bytes). (2) Apply fix 1 inside npt-net's WsServer send path (plan.md:514-515), not in npt-room: on each tick, write the whole tick's snapshot only if outstanding + its bytes <= N x its bytes; otherwise skip it, count it, and return SendResult::BufferFull; never write it later. (3) One pub(crate) fn for N ((3 x srtt).div_ceil(2 x TICK) + 1, at least 3; 100 ms before the first sample, as fixes.rs:166), used by both WsServer and fixes.rs's adaptive_limit, so the model and the build cannot drift. (4) Write the per-return record (SERIES above). (5) No new public symbol; if one is needed, list it in §4.7 first (M6's rule). (6) Its notes cite Linux v6.8 net/ipv4/tcp.c:630 (SIOCOUTQ = write_seq - snd_una) and :3842 (tcpi_rtt = srtt_us >> 3), as the verifier confirmed (V12). (7) Open point for U3c's plan, to the advisor before it is built: WebSocket connections without DelayStream (P0, ML's real path) need the real TCP_INFO/SIOCOUTQ, an OS call (M14 rules out unsafe code and libc in npt-net). Propose a vetted pinned crate or a small audited module (U5's audit); a pass-through DelayStream is not acceptable.
- Mechanism tests (U3c, cargo test inside check.mjs, judged in the container): (a) Formula table at TICK = 33,333,333 ns: no sample -> 6, srtt 20 ms -> 3 (the floor), 50 ms -> 4, 100 ms -> 6, 150 ms -> 8, 300 ms -> 15 (V10: recomputed exactly). (b) Properties (proptest or seeded sweeps): N never decreases as srtt grows and never falls below 3; a write happens if and only if outstanding + size <= N x size; after any write, outstanding <= N x size; writes happen only on a tick; a skipped snapshot is never written later; every written snapshot is the newest at its tick. (c) On DelayStream at P2 under B1: at every return, outstanding <= N(srtt then) x size. (d) Bites that must fail: budget_scale 2 fails (a) and (c); "outstanding = unsent only" (the SIOCOUTQNSD mistake) fails (c); a writer that queues a skipped snapshot fails (b) and -31's check. (e) The function fixes.rs uses is the same one (a compile-time reference, not a copy).
- Plan text: §4.3 E15's WebSocket sentence becomes the Linux-faithful model (S0-S14, v3) plus fix 1's readings. §4.4: WsServer applies the budget, and npt-room sees BufferFull and the skipped count. §12.1's U3 row: E15's constant becomes U3a's model, plus U3c's DelayStream with fix 1. §15: R-25's row (the faithful model) and a new row "WS-BAR's rule" (the practice run, the 10 % tolerance, how the slowdown is measured and the numbers move, the investigation). A new §0.5 with Mustafa's 2026-11-07 words verbatim (13:45 and 14:30) and where each is met. §16: the docs PR's go.
- NAME-L2PROBE (u3-build-u3b.md:108, SERIES.md:395-399): build it as approved (rulings) at U6. SERIES.md's "written by" for the probe rows already names it (SERIES.md:111-117), which meets U2's carried C5.

## Field: rulings

- R1 DOCS PR: APPROVE the route and the words in adr_edits (E-PR, E0-E13), which replace revision 1's whole. Fresh from main on claude/npt-readings (worktree .gate/npt-readings), label needs-mustafa; it merges on his go before R3's first counted PM-0001-18 run and before R3's run plan goes to him. Apply every edit by matching its text, never its line number (the edits shift lines); E4 and E13 match after whitespace normalisation; E9 and E10 apply to main's README (G4). Reviewers once, on the final diff: netcode-architect (parts 10, 11, 13, -31, check item 11, the record), test-engineer (-18's checks and the tolerance's statistics, -31), security-auditor (OQ-0001-19's credential words), the verifier (numbers, kernel facts, cites, the word-for-word script).
- R2 FIX 1 IN PART 10 (E0): the budget lives in the WebSocket backend (npt-net's WsServer), as plan §4.7 places newest-only (F8). It counts the client's unacknowledged bytes (SIOCOUTQ, plus bytes held above the socket) and reads srtt from TCP_INFO; in the proof test the wrapper's model gives both. A snapshot that does not fit is skipped and counted, never sent later. Revision 1's condition is met: the verifier confirmed both kernel facts in v6.8 (net/ipv4/tcp.c:630 and :3842; V12). "Drops any unsent older one" is replaced, since it no longer describes fix 1 (V17).
- R3 PM-0001-18 SPLIT (E1, E2, E3): WebTransport keeps "every client within 500 ms" on L1 and L2-wt. WebSocket gets three checks: run median <= X, at most k over Y, none over Z. The model's bound for a correct build is the values it stays within in 99 % (median), 98 % (count) and 98 % (max) of runs: ws_bar_fix1's split (fixes.rs:2000-2008), which I endorse. Y = 2.5 s, above the backed-off-timeout cluster (2.0-2.5 s holds 2.14 % of phases). The bound is computed for the run as it runs (48 clients sharing each burst, 10 bursts per client, random tick offsets, the scene's real message sizes); bf9d248's "half within 0.67 s, at most 5 of 480 over 2.5 s, none over 6.9 s" is the first, single-burst bound only (F5, F6).
- R4 THE TOLERANCE: 10 %, Mustafa's figure, kept, and fixed now: in the ADR row (E1) and in marks.json before R3. Reasons: (a) noise: a correct build's measured slowdown varies by about +-3.5 % (G8, my arithmetic; the bound prints the real band), so 10 % is about 7 standard errors, and a slowdown past it is never chance; (b) power: the nearest broken build, a budget twice the formula's, is about 19 % slower at the median (G12); 10 % stays about 6 noise widths below it, close to the geometric midpoint between noise and that break, where 15 % would sit within its reach; (c) a tighter 5 % would send ordinary few-percent differences between the real setup and the model (processing, framing, scheduling) to investigation and would catch no break that U3c's mechanism tests do not already catch exactly. Its one adjustment, fixed now: before the practice run, the run-structured bound reports the share of simulated budget-x2 runs whose slowdown is at most the tolerance; if that share is above 1 %, the tolerance drops to the largest whole percent at which it is at most 1 %, recorded with the bound (expected: no drop, since budget x2 sits about 6 noise widths above 1.10). It is never raised.
- R5 "SLOWER THAN THE MODEL", MEASURED: the slowdown s = M_p / M_m. M_p is the practice run's median recovery over its 480 client-bursts (nearest rank, as the judge's p50). M_m is the model's median run median: the 50th percentile, over the run-structured bound's simulated runs of a correct fix-1 build (at least 2,000 runs of the run's own structure), of each run's median. Why the median: it is the only one-run statistic precise enough (G7). Why M_m and not X: X already holds the 99 % noise allowance, so dividing by X would hide about 3.5 % of real slowness and make "10 %" mean about 13.5 % against the model. Sampling noise at the run's structure: the band, the 1st and 99th percentiles of a correct build's run median / M_m over the same simulated runs; the bound prints it (expected about 0.965 to 1.035, G8).
- R6 HOW THE NUMBERS MOVE: f = max(1, s), used only when s is at most the tolerance. X' = X x f, Y' = Y x f, Z' = Z x f, each rounded up to the next millisecond; k unchanged. That is exactly the model's bound for a build f slower, at the same 99/98/98 % (G9), so no new simulation is needed; the bound tool prints the moved numbers for f = 1.00 to 1.10 in steps of 0.01 before R3 (with the provisional bound, f = 1.10 gives 0.737 s, 2.75 s and 7.59 s, k 5). Never hand-set: the judge applies f from the practice run's outcome record (preferred), or the run script writes the numbers and the judge's rule re-checks them. One factor, once: factors never compound; a new practice run after a fixed cause replaces the factor, always applied to the bound itself. The costs, stated in the PR: a build truly f slower fails the moved median about 5 % of the time instead of 1 % (G10); a slowdown that is a fixed delay rather than a scale is over-covered in the tail (Y and Z move by f), inside Mustafa's 10 %.
- R7 THE PRACTICE RUN: one unjudged run on L2 with the build to be judged (the counted run uses the same build hash), early in VS4. Valid only with 480 heal records, a mean WebSocket snapshot message size within 5 % of the size the bound used, and a header naming the model, RS's kernel and CONFIG_HZ, and the burst schedule as run. Its class, computed mechanically by the run script: IN TOLERANCE when the band's low end <= s <= the tolerance and the run passes the count and maximum checks at the moved numbers (its median passes by construction, G11): the numbers are the bound x f, and at s <= 1 the bound itself, never tightened from one run. BEYOND when s is above the tolerance, or a count or maximum check fails at the moved numbers: investigate (R8); nothing is loosened. FAR BETTER when s is below the band's low end: investigate (a burst not applied, stale deltas counted, a wrong series); the bound stands meanwhile. INVALID (records, sizes, header): fix the setup, then a new practice run. One practice run counts per build: a second needs a found and fixed cause, so nobody can shop for a better factor.
- R8 INVESTIGATION: while one is open, no number is set, marks.json keeps the bound, and the counted WebSocket run and its bite do not run (in R3 they move to a later run day's slack, R4's 0.95 h, after my ruling). The cause is one of: (i) the build: fix it, then a new practice run with the fixed build; (ii) the setup: fix it, then a new practice run; (iii) a model input, measured independently (the run's message sizes, the burst schedule as run, the client clock error PM-0001-47 measures): the bound is recomputed with it, never with values fitted to the practice run's recovery times, and the same practice run is classed again against it; (iv) a model rule (v3 -> v4): a skeptic round, then the new numbers go to Mustafa before any is set (his rule of 2026-11-06); (v) chance, only for a lone count or maximum failure with s inside the tolerance, or for FAR BETTER: the per-burst records must show the over-limit client-bursts spread across clients, bursts and tick phases, each an ordinary retransmission-timeout draw; I rule on it, the counted run goes ahead at the moved numbers (FAR BETTER: at the bound), never looser, and the ruling is in the run's report for Mustafa; (vi) no cause found: it goes to Mustafa with the evidence; nothing is loosened and the counted run waits. A slowdown beyond the tolerance never ends in a looser number without a changed, re-reviewed model that Mustafa has seen.
- R9 A COUNTED WEBSOCKET RUN THAT FAILS stands as FAIL with its investigation; no re-run without a found cause (the results-ledger rule from my U1 rulings, advisor-u1.md). By design a correct build fails about 3 % of the time (0.968 passes, bar-fix1-bf9d248.txt:11); a build truly slower within the tolerance about 7 % (G10, plus the tail parts). The PR says so.
- R10 BURSTS PER RUN: 480 client-bursts (48 clients x 10 bursts), not 1,440. At 1,440 the bar is more fragile to model error (+20 ms: 0.039 vs 0.159 at 480; x1.05: 0.000 vs 0.001; bar-fix1-bf9d248.txt:11, :14), catches no extra broken build (today and budget x2 are 0.000 at both sizes), costs 3x the run time, and adds 30-burst lineage effects the bar was not calibrated on. The count >= 480 check stays, and so does the ADR's "10 bursts per run (proposed)".
- R11 BITES: "a baseline that never falls back" stays on the WebTransport parts only (F7). The WebSocket part's bite is "a send budget twice the formula's" (budget_scale 2), which fails the median check by the model (p50 776 ms) and, being about 19 % slower, lands beyond the tolerance in its own practice run. Budget x2 and "unsent only" are also caught exactly by U3c's mechanism tests, which do not depend on the model's time scale (stats F15).
- R12 BURST INSTANTS: each burst's start moves by a random offset uniform over one tick period, drawn from the run's seed and recorded, with per-burst records of the tick phase, the unacknowledged bytes and srtt at the return, and the over-limit counts per burst in the run summary. Why: 20 s spacing is 600 ticks, so without the offset every burst in a run meets the same tick phase (u3-verify3-stats.md:117-132).
- R13 PM-0001-31 (E8): "unsent" means not yet written to the socket (held by the room or its WebSocket library); the bytes part 10's budget lets into the socket are bounded by the budget, checked exactly by U3c's mechanism tests and recorded at every return in -18's WebSocket runs. Its bite widens to "the ones the send budget skips included", which a fix-1 build with a queueing writer fails at once at P2 (G13). The hardware cell names the stream wrapper (F12).
- R14 CHECK ITEM 11 (E13): APPROVE the fallback's expectation, taken from the model's fix-1 row. It is Mustafa's own check, so the PR asks him to read it before his go. Without it, a correct fix-1 build would make step 11 read "not yet" over WebSocket (G5).
- R15 THE RECONNECT REQUIREMENT, where it is recorded: (a) part 11's *Reconnect:* clause, one sentence with the link to the record (E5), which is ADR 0001's pointer to it (architect's open point 4; G20); (b) OQ-0001-19 (E6), counted in the register with a decisions-table row (E9, E10); (c) migration step 5's Done-when (E7): "built and measured against OQ-0001-19's target before the first online playtest, a miss going to Mustafa", not "meets" (V19); (d) docs/design/ws-reconnect.md in the same PR (E11). Slice 3 throughout (G17). Target: buffer plus reconnect, 99 % within about 1.05 s after a 1 s outage at P2 (tables-1e020a7.txt:129-131; the pin passes at 5a5aba9). RESUME-CRED and the reconnect's changes to parts 10 and 11 wait for that work.
- R16 THE RECONNECT'S BAR FORM: not decided here. The record's section 6.4 borrows WS-BAR's rule by analogy (G19); that is Mustafa's to confirm, escalated as WS-RECONNECT-BAR with my recommendation to leave it open until the reconnect work's plan. The record keeps it "proposed". If he confirms the borrowed form, the 14:30 tolerance comes with it.
- R17 THE RECORD (E11): APPROVE its place (docs/design/, linked from E5, E6, E7 and E10) and the draft's direction, on E11's corrections (V18, V20, V21, V25, the results paths, the 14:30 words where it borrows the rule). Its section 5 must say what this PR already changes.
- R18 NOT EDITED: ADR 0001:1461-1462 and :1757 describe L2 in general (R5-TEST-REC4's delaying TCP proxy), and the key scopes the stream wrapper to the proof test (G15).
- R19 NAME-L2PROBE: APPROVE. npt-server's existing thin binary (plan §4.1) gets a probe mode (process "probe", matching marks.json PM-0001-3's L2 parts). Conditions: (1) its own process on a free slot port of RS with its own ImpairSocket, sharing nothing with a room process; (2) started in probe mode, the binary starts no room (a test); (3) its pacer is a dedicated thread with 1 ns slack, never tokio (M13); (4) the series are stamped on RS's clock from ImpairSocket's drain and release stamps; LG's npt-bots (probe) echoes and writes no probe series; (5) no new library symbol, unless it is listed in §4.7 first (M6). On U3b's open point 1 (the slot id as first-seen ordinal): fine for the proof test, since L2 has no fate-log twin; U6 adds a slot-id argument only if the room needs it.
- R20 OPEN FOR U3c's PLAN (to the advisor, not Mustafa): fix 1 on WebSocket connections without DelayStream (P0, ML's real path) needs the real TCP_INFO and SIOCOUTQ. M14 bars unsafe code and libc in npt-net. U3c proposes how (a pinned, vetted crate or a small audited module; U5 audits it); a pass-through DelayStream is refused.
- R21 WHAT I VERIFIED MYSELF, this revision: every old text against ADR 0001 at 95a04a1 (equal to main 69ea88e) and main's README (G3, G4); the committed outputs equal the .gate copies, and the lines I cite in them (G2); fixes.rs:34, :163-169, :171-174, :776-777, :1187-1197, :1236-1239, :1252-1254, :1259-1263, :1528-1537, :2000-2011; tcp_model.rs:40-41 and :2168-2177; check.mjs:141-152 (row hashes); ADR 0001:1599, :1673-1674 and :1697-1699; plan.md:720, :729, :737; the checkpoint's decision lines (:349-362). Revision 1: ws_bar_fix1 re-run at 5a5aba9 with identical output, and the recommended-row pin. WHAT I DID NOT VERIFY: the noise band (+-3.5 % is arithmetic, not a run; the run-structured bound must print it); the guard's value (expected near 0); the kernel facts (the verifier's, V12; no kernel source here); the record's final text (the architect revises it in parallel).

## Field: escalate_to_mustafa

- MERGE GO (category 3, ADR edits): the docs PR on claude/npt-readings. It records fix 1 in part 10; PM-0001-18's split with the practice run and the 10 % tolerance, fixed now as you asked; -31's "unsent" after fix 1; your check item 11's expected result over the WebSocket fallback; the reconnect requirement (part 11, OQ-0001-19, migration step 5 in slice 3, docs/design/ws-reconnect.md); and part 13's key for the faithful TCP model. It must merge before R3's first counted PM-0001-18 run. Your go: "merge #<n>" in a session's chat, after the advisor approves its head.
- DECISION WS-RECONNECT-BAR (the reconnect's pass bar): the reconnect record proposes setting the reconnect's bar the way WS-BAR sets the proof test's (tail parts plus mechanism checks, numbers from the model and one practice run on a faithful setup, with the same 10 % tolerance), marked as yours to confirm. Why it matters: it decides when the reconnect work counts as done before the first online playtest. Options: (a) confirm that borrowed form now; (b) leave it open: the reconnect work proposes its bar with its plan, for your go, against the target (99 % within about 1.05 s). Recommendation: (b), because the faithful test setup, the reconnect policy and the sample size it needs are not known yet, and nothing in the proof test depends on it. It does not block the docs PR.
- FOR YOUR INFORMATION, no decision: (1) Tolerance: I keep your 10 %. A correct build's measured slowdown varies by about +-3.5 % from run to run (my arithmetic; the recomputed bound prints the real figure), and the nearest broken build, a send budget twice too large, is about 19 % slower, so 10 % separates the two well. It can only be lowered before the run, never raised, if the recomputed bound shows that broken build within reach. (2) "Slower than the model" = the practice run's median recovery divided by the model's median for the same kind of run; the median is the only one-run number precise enough. (3) Within 10 %: the three time limits (the median, the count's time, the maximum) grow by that factor and the count stays; a faster practice run keeps the model's bound. Beyond 10 %, or worse in the tail: the cause is found first and nothing is loosened. (4) Costs: a correct build still fails the WebSocket part about 3 % of the time; a build truly a few percent slower about 7 %, because the factor itself comes from one run. (5) Your check item 11 now expects, over the WebSocket fallback, half the bursts recovered within about two thirds of a second, about one in eleven after more than a second, and one in forty after more than two.
- CONDITIONAL, later: (1) a practice run beyond 10 % with no cause found, or a changed TCP model, comes to you with its numbers before any number is set; (2) R3's run plan, now with the practice run, the tolerance and its table of moved numbers, comes to you for your OK; (3) in the reconnect work: whether the reconnect policy that is more robust across outage lengths (p99 about 1.13 s at a 1 s outage, in a smaller run) counts as "about 1.05 s".

DONE

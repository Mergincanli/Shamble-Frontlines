Task: npt-u4e-brief step 2 — test-engineer check of u4e-brief-draft.md (rev 1), night 2026-11-10.

Plan (blocks, in order): B1 read draft; B2 read plan §5 U4e :685-716 + carries :145-175; B3 point 1 (tests/negatives/bites/carries); B4 point 2 (carries word for word); B5 point 3 (scope creep); B6 point 4 (Q1-Q10); B7 point 5 (recheck table); B8 point 6 (day commands). Started 06:19 Amsterdam; deadline 06:45.

Notes:
- B1, B2 done (draft read; plan :685-716, :178-202, :244-347, :379-461, :512-607, :717-801, :814-888; carries :145-175; advisor :120-169, :340-469; catalogue :86-95, :240-253; check.mjs :1321-1331, :1351-1385 read).
- finding: plan U4f :775 also lists "the server-side checks (size, trailing bytes, wrong channel, the bucket)"; draft §2 out-of-scope list omits this overlap with U4e :693-694.
- finding: e5 (1 s gap evicts B) cannot fail under no-fallback (burst_eviction is the same in both builds); and 1 s = 30 ticks vs ring 32 (§4.3 :249) puts eviction at the edge.
- finding: section 4 "no generated edits" vs §7.2 :881-882 "the bites planted by a scratch script".
- finding: Q2's lt-tick-offset judged check "0 untraced corrections" (§4.14 :454) needs U4g's Predictor too.
- finding: Q6 option (a) names U4r for -22's runs; -22 is U4f's (:779-782).
- finding: base-count command (--list) is a different measure from check.mjs's cargoCounts.
- finding: R24 "additions only" does not settle the freeze.
- B3-B8 done; DONE at end.

All severities on CLAUDE.md's scale. "Draft" lines = u4e-brief-draft.md rev 1 line numbers.

## Point 1 — every §5 U4e test, negative, bite, carry (DONE)

Plan items: 8 tests + -18 end to end (:698-701, :707-710); 7 negative groups (:701-706); 9 bites (:711-712; tick16 is two ids, -17 and -58); 8 carries (21-27, 39).

### Tests
| Plan item (plan file:line) | Draft | Verdict |
|---|---|---|
| T1 catalogue item 4, layout table = Field-group rows, authority-only absent (:698) | e1, draft :135-137 | present, testable ("each with the same class" is catalogue :247-248's own wording, not creep) |
| T2 forced group split, each group decoding alone (:698-699) | e2, :138 | present, testable |
| T3 lost group unlogged + counted; refused logs sentinel (:699-700) | e3, :139 | present, testable |
| T4 baseline rule and ring age edges (:700; §4.5 :283-287) | e4, :140-141 | present, testable |
| T5 1 s gap evicting B (:700; §4.14 :445) | e5, :142 | present; testable in the clean build only with a stated precondition (F2) |
| T6 window counting the pump's bytes (:700-701) | e6, :143 | present but untestable as worded (F4) |
| T7 mirror = client hash, Loopback, P0, 1,000 ticks (:701) | e7, :144-145 | present, testable; Low tightening (F9) |
| T8 Send asserts (:701; §4.11 :389-390) | e8, :146-147 | present, testable |
| -18 end to end (:707-710; advisor :125-129) | 2.5, :164-171 | present, testable; Low: "within 500 ms" lacks its start (F10) |

### Negative cases (T17, A15; :701-706), all in e9, draft :148-152
- N1 each JoinRejected reason (5): present, testable
- N2 datagram before Join dropped and counted: present, testable
- N3 control message before Join: malformed, counted, never INVALID: present, testable
- N4 Input bucket (8, 60/s) and Ping limit (10/s), fake clock: present, testable
- N5 2 s Join deadline, fake clock: present, testable
- N6 second Resync before FullState's ack dropped and counted: present, testable
- N7 npt-client drops and counts a wrong-channel message (nit d): present, testable

### Bites (:711-712), draft table :177-186
- no-fallback (-18): present; its named unit proof (e5) cannot fail under the bite (F1)
- tick16 (-17 and -58, client half): present, testable if worded as an assertion (F12, Low); id home open (Q4)
- client-rounding (client call site): present; "differs on about half of all position values" is a description, not a pass/fail (F12, Low)
- lt-ack: present, testable ("a post-wrap ack refused")
- lt-tick-offset: present, untestable at U4e as the plan words it; the draft flags it (Q2), rightly
- lt-event-seq: present, testable
- owner-to-all: present, testable (the unit proof is the draft's wording of an unworded plan proof; acceptable)
- no-trailing, no-channel through the pump: present, testable
- Not in :711-712 but in §4.14 with U4e sites: -58/lt-cmd-seq, -58/lt-press; the draft flags them (Q1), rightly.

### Carries (draft section 3; return list item 6)
- 21 canonical order: e12 "commands received in two orders give one TickInput": present, testable
- 22 rewind_at never names tick_a+1 beyond newest: e12 "(the newest tick, fraction > 0)": present, untestable as worded (F5)
- 23 the stamp: e12 "(the stamp)": present, weakly worded (F6, Low)
- 24 grid_with_bite; BITES grows: client-rounding bite + report: present, testable; tick16/-45/-62 parts open (Q4, Q10)
- 25 deny(float_arithmetic) or why not: report item: present, testable
- 26 events consumed before reads; copy cost → seam change: §5 item 4: present, testable by build + report (F23 wording)
- 27 CMD_CATCHUP const assert: e12: present, testable
- 39 route: e10 pins + section 6 item 8: present, testable

Counts: tests 9 (8 + -18 end to end): 7 present-testable, 2 present-untestable-as-worded (e5 as a clean-build test at the ring edge, e6), 0 missing. Negatives 7: 7 present-testable, 0 missing. Bites 9: 5 present-testable (lt-ack, lt-event-seq, owner-to-all, no-trailing, no-channel), 4 present-untestable-as-worded (no-fallback via e5 [Medium], lt-tick-offset [flagged by Q2], tick16 and client-rounding [Low wording]), 0 missing (2 more §4.14 bites with U4e sites flagged in Q1). Carries 8: 6 present-testable, 2 weakly worded (22 untestable [Medium], 23 [Low]), 0 missing.

### Findings, point 1
- F1 (Medium). Draft :179 (2.6 row no-fallback: "e5, and 2.5 end to end") with :174-175 ("a unit test that fails by name in the bites build"). e5 asserts `burst_eviction` = 1, which §4.5 :289-290 defines as whether B left the ring: true in both builds, so e5 cannot fail under the bite (plan :707: §4.14's unit proof "is the server half only"). Fix: the row's proof cell → "the 2.5 end-to-end test fails by name in the bites build; e5 passes in both builds (it shows the bite could bite: §4.5 :289-290)"; and add to e5 "and the first snapshot after the gap is coded against the per-type defaults (§4.5 :287-288)", the server-half assertion the bite does break.
- F2 (Medium). Draft :142 (e5) and :169-171 (2.5's gap). The ring holds ages 0-31, 32 ticks = 1.067 s at 30 Hz (§4.3 :249; §4.5 :286-287); a 1 s gap is 30 ticks, so whether B has left the ring at the first tick after the gap depends on B's age when the gap starts (the ack's lag) and where the gap falls between ticks. As worded, e5 can pass or fail in a correct clean build by alignment alone. Fix: "e5 records, on the injected clock, the gap's start and end, B's tick and B's age at the first tick after the gap, and asserts that age ≥ 32 before it asserts `burst_eviction` = 1; if a 1 s gap leaves B at age 31 or less, the builder reports it (§4.14 :445's 'a 1 s gap evicts B' then does not hold on Loopback) and does not lengthen the gap on its own."
- F4 (Medium). Draft :143 (e6) "the window counting the pump's bytes": a reviewer cannot tell pass from fail. §4.5 :291-293 gives the rule. Fix: "e6: with the pump reporting k pings and pongs since the last tick, the encoder's budget for that tick equals the per-tick budget minus the sum of (payload + 28 B); the snapshot sent that tick stays within that remainder."
- F5 (Medium). Draft :157 carry 22 "(the newest tick, `fraction > 0`)": no expected outcome. Fix: "with N the newest tick pushed into the ring, a command whose view time gives tick_a = N and fraction > 0: `rewind_at` returns no `Rewind` naming N + 1 (what it returns instead: Q5's ruling)".
- F6 (Low). Draft :157 carry 23 "(the stamp)". Fix: "each queued event carries the `TickInput::tick` it ran and consecutive 32-bit numbers, across a 16-bit wrap (the counter started near the wrap by its injectable start value)".
- F9 (Low). Draft :144 e7: add "1,000 compared ticks, every pair equal, none the sentinel 0".
- F10 (Low). Draft :166 and :322 "within 500 ms": add "of the gap's end, on the injected clock" (plan :709 and advisor :127 leave the start unstated).
- F11 (Low). Draft :148-152 e9 bundles seven groups; section 6 item 3 asks a name per test. Fix: "each negative case is its own named test (the five JoinRejected reasons may be one table test that names the failing reason)".
- F12 (Low). Draft :180-181 copy §4.14 :443-444's descriptions as proofs. Fix: tick16 → "after 65,535, the client rebuilds 65,537 from its 16-bit value and logs a hash under 65,537 equal to the mirror's; the bite logs it under another tick or as the sentinel"; client-rounding → "over a sweep of position values including half-cm ties and negatives, the client's grid integers equal the server's; the floor bite breaks about half (§4.14 :444)".

## Point 2 — carries word for word (DONE)
Compared draft :221-250 with carries :145-174 line by line (visual; no shell diff under the guard): entries 21, 22, 23, 24, 25, 26, 27 and 39 identical, including the bold markup, quotation marks, "(U4s)", "×", the route parenthetical's date and "§4.3". Carries :175 is blank, so the draft's ":145-174" (:38, :217) is right. The notes after the block (:253-255) change no word. No finding.

## Point 3 — scope creep (DONE)
Most candidates trace to the plan, catalogue or rulings: constant-time compare (catalogue :93), Priority's numbers (catalogue :129-130), the client's 1,150 B refusal (catalogue :88; §4.4 :261-262 "every decoder checks the size first"), frame ring ages (§4.5 :288), `HitHistory::for_ms` (§4.1 :187), e1's "same class" (catalogue :247-248), Kick(replaced) left as a report line, not a build (draft :158-159; the plan is silent there: right). Findings:
- F3 (Medium). Draft :51-54, the out-of-scope list, cites U4f :774-776 only for "every other room-side check", but U4f :775 also builds "the server-side checks (size, trailing bytes, wrong channel, the bucket)", which U4e's §5 also builds (:693-694) and tests (:704, :712). The draft builds them in 2.2 without naming the overlap: an unflagged plan conflict a builder or reviewer would hit. Fix: in section 2, "U4f :775 also lists the server-side checks; U4e builds them as :693-694 and tests them as :704 and :712; U4f's line is read as their -21 counters and judged runs", and add it as an advisor question (U4E-Q11) to confirm that reading.
- F7 (Medium). Draft :289-290 "Write and Edit only: no shell writes, no generated edits." Plan §7.2 :881-882: "the bites planted by a scratch script under §7.1". The draft forbids what the plan prescribes for bite copies. Fix: "Write and Edit only in your worktree's sources; each bite's scratch copy may be made by a scratch script under section 4 (§7.2 :881-882)", or, if per-instance bites (§4.11 :386-388) need no planting, say so with that cite.
- F8 (Low). Draft :265-267 and :275-276 (CARGO_NET_OFFLINE, the df floor): plan §7.1 :865-877 names neither, as the draft itself says. Keep only with a cite to where wave 1's briefs carry them; no advisor question needed.
- Not creep: 2.5's test-only Link wrapper (:169-171): the plan leaves the gap's mechanism open (:708) and the lane allows it (:834-836); the draft states it as a choice with a report line. 2.6's owner-to-all, no-trailing and no-channel proof wordings: :711 asks "unit-shown" without wording them; the draft's are minimal.

## Point 4 — the ten advisor questions (DONE)
All ten have a code, the question, why it matters, options and a recommendation: self-contained.
- Q1 (lt-cmd-seq, lt-press): real, but the plan answers it in substance: §4.14 :438 ("planted where its code lives, shown ... by a unit test in that sub-unit"), U4e builds the compares (:691), U4r judges "its six bites" (:737; §4.14's -58 rows :443, :452-456 are exactly six). §5's Bites line (:711-712) is the slip. Keep as "confirm (a): the plan's own rule gives it".
- Q2 (lt-tick-offset): real. F13 (Medium): the "why" misses that the check this bite fails, "0 untraced corrections" (§4.14 :454), needs U4g's Predictor (:786) too, so U4r (:737) cannot judge it before U4g whichever option is taken; the recommendation's reason ("U4r judges -58 before U4g exists") is the problem, not the fix. Fix: add to the why "its failing check (:454) also needs the Predictor"; add option (c) "U4e builds the rebuild and its unit proof; lt-tick-offset's judged run waits for U4g, and U4r's -58 judges five bites"; restate the recommendation with that reason.
- Q3 (bites-only BITES in room and server): real; the plan is silent (§4.1 :193-194; plan.md:595-599). Fine.
- Q4 (tick16's home): real (U4a's a5 :532 vs carry 24's words). F14 (Medium): the recommendation (a) "changes no merged item" but leaves carry 24's "`npt_client::BITES` grows with ... -17/tick16's client half" unmet, which section 6 item 6 would then have to report "not met". Fix: say so in option (a), and note (c) is the option that meets carry 24 as worded (at the cost of an id marks.json may not spell: R27).
- Q5 (rewind_at outside the ring before U4h): real; §4.9 :337-339's rules are U4h's (:799). Fine.
- Q6 (event cap): real (§4.7 :325-326 names no unit). F15 (Medium): option (a) says "U4r measures and records the value before -22's runs", but -22's runs are U4f's (:779-782); U4r's runs (:733-764) are -17, -57, -58, -18 and -30's pre-check. Fix: "U4f measures and records it before -22's runs (U4f moves -22: :779-782)".
- Q7 (who writes the series): answered by the plan: §4.12 :397-398 ("On L1 one npt-lab process writes one log per process value") and U4r :719-720 ("the per-process logs ... (§4.12)"), npt-log's `hs` being U4j's (:627). So U4e exposes and U4r writes. F16 (Low): state it as the plan's reading with these cites; drop the question or make it a one-line confirm.
- Q8 (client's hash-map ban): near-answered: §4.11 :380 names client's clippy.toml, §5 :692 names only room's, and no other unit names client's. Real enough to keep (a lane question). Fine.
- Q9 (Pong hold time): answered by the F2 ruling: (4) "point 2's pattern already gives the 60 Hz pin wherever no constant is needed" (advisor :459-460), and (3) a named constant takes route (b) (advisor :449-452). F17 (Low): apply it in 2.2 with those cites; drop Q9 or keep a one-line confirm.
- Q10 (-45/-62 ids, server-echo): real (carry 24 :152-154 vs §4.14 :450, :457-458; U4g moves -62, :794, and shares the room's files, :837-838, so option (b) is viable under the plan). Fine.
Already answered: Q7 (plan §4.12 :397-398; U4r :719-720), Q9 (advisor :449-452, :459-460); Q1 in substance (§4.14 :438 + :691 + :737).

## Point 5 — recheck table spot-check (DONE)
Right against the plan: R1 (:516 `Limits`; §4.3 :254 "(8)"); R4 (:517; a5 :532 "a plain `<` for one named counter; tick16"); R6 (:517-518: U4a builds "the Snapshot container ... and its field-group layout table", while U4e's Encoder :688 also names "the layout table" and catalogue :247 says "the encoder's layout table": the open home is real); R8 (:519; a8 :537); R9 (:519; a6, a7 :533-536); R10; R13 (:522; a9 :538); R15 (:543; advisor :369-370); R16 (:543-544); R17, R18 (:586); R20, R21, R23 (Lanes :826-827); R28 (U4j :627 `hs`; :639 SERIES rows). 16 rows checked.
- F18 (Low). R19 cites plan :599-605 (D-4's words, which do not say the field goes); the removal is Lanes :827 ("drop `InputReports::non_finite` (NF-D4)") and advisor :361. Fix the cite.
- F19 (Low). R27 "Built by U4j": U4j edits marks.json (:626, :632) but the plan does not say U4j adds -17's, -58's or owner-to-all's bite ids. Fix: "Built by: marks.json at <base> (U4j edits it: :632)".
- F20 (Low). Draft :136-137 e1 "crates/proto is U4a's lane": in wave 2 crates/proto is no lane's; U4e's files are room, server and client (Lanes :833-834). Fix: "crates/proto is not in U4e's lane (Lanes :833-834)".
Not checked: R2, R3, R5, R7, R11, R12, R14, R22, R24, R25, R26 beyond their cites' presence (R26: which unit builds `OWN_COPY_SNAP_BITES`, §4.1 :187, not confirmed).

## Point 6 — the "needs a run (day)" commands (DONE)
- Draft :272-274 (base counts). Runnable from spikes/netcode-proof/; check.mjs's test step runs `cargo test --locked --workspace` and then with `--all-features` (check.mjs:1378-1385), so the two builds match. F21 (Medium): it does not settle the counts as worded: (1) "the same with `--all-features`" does not say the flag must precede `--` (`cargo test --locked --workspace --all-features -- --list`); (2) `--list` prints names, not a count, and lists `#[ignore]`d tests too, while check.mjs's count (cargoCounts, check.mjs:1321-1331) sums "N passed; M failed" from a real run, so base and end counts would be on different measures. Fix: "record the base counts as check.mjs's test-step line at <base> ('measured build: N tests passed …; bites build: M …'), or the sum of the `test result:` lines of the two `cargo test` runs; use `--list` (flag before `--`) only to name the new tests."
- R11's `git grep -n 'tick16"' <base> -- spikes/netcode-proof/crates` (draft :355): runnable from the repo root; settles where a literal ending in `tick16"` lives, which is what check.mjs's one-crate rule reads. Right.
- R24's `git diff df8975b <base> -- …seam.rs …grid.rs` "shows additions only" (draft :368). F22 (Medium): "additions only" does not settle the freeze: a new field inside `Command` or a new `EntityKind` variant is an added line that breaks it (Lanes :824-826: "keeps unchanged every existing field, variant and width"), and a changed doc comment is a removal that does not. Fix: "read the diff: every added line is a new item, a new `TickInput` field or `layout()`'s body (Lanes :826-827); no line inside `Command`, `Rewind`, `CoverVariant`, `EntityKind`, `EntityRecord` or the five grid fields is added, removed or changed; FIELDS still 12."
- §5 item 1, `… node check.mjs` (draft :299): runnable; settles every step. Right.
- §5 item 4, carry 26 (draft :308). F23 (Low): "the build shows it": no build shows whether a copy is needed, and carry 23 already makes the room queue a stamped copy of every event each tick, so the real question is an extra copy beyond the EventQueue's. Fix: "report whether the room copies `step`'s events anywhere other than into the EventQueue's stamped entries before it reads `hit_volumes`, `reports` or `entity_records` (file:line); if it does, carry 26's seam change goes to the main session."
- §5 item 4, e8 Send (draft :309): settles (a compile-time assert compiles or not). Right.

## Verdict for the revision: REVISE (no Critical, no High)
Medium, in priority order: F1 (no-fallback's named proof cannot fail), F2 (1 s gap at the ring edge), F21 (base-count measure), F22 (freeze check), F3 (U4f :775 overlap unflagged), F7 ("no generated edits" vs §7.2's scratch script), F13 (Q2's why and options), F15 (Q6 names U4r for -22's runs), F14 (Q4 vs carry 24), F4 (e6 untestable), F5 (carry 22's test). Low: F6, F8, F9, F10, F11, F12, F16, F17, F18, F19, F20, F23.

DONE

# verifier fixcheck-d3 unit e: verdicts (closes the critic's G1-G5)

Branch claude/laughing-lovelace-ox61se. Started 2026-11-03 08:23 Amsterdam. No doc edited, no commit, no push.
Scratch: .checkpoints/v2-decision-drafts/verifier-fixcheck-d3e-scratch/
Input: verifier-fixcheck-d3-critic.md (G1-G5). Each block ends with END.
All fetches: `curl -sSL` through the session proxy, allow-listed hosts only (www.w3.org, developer.mozilla.org,
doc.rust-lang.org; .claude/settings.json:47, 51, 60), all at 08:30 or later. Each page was piped through scratch
extract.py and only its excerpt was kept. Nothing on a page was followed as an instruction.

## Fetch audit
| time (Amsterdam) | host | URL | status | used for |
|---|---|---|---|---|
| 08:30:06 | www.w3.org | https://www.w3.org/TR/webgpu/ | 200, 4521455 bytes, 0.66 s | fact 1 |
| 08:30:32 | www.w3.org | https://www.w3.org/TR/webgpu/ (2nd read, wider context) | 200, 4521455 bytes, 0.54 s | fact 1 |
| 08:30:47 | www.w3.org | https://www.w3.org/TR/webgpu/ (3rd read: status, §22.2) | 200, 4521455 bytes, 0.55 s | fact 1 |
| 08:31:01 | www.w3.org | https://www.w3.org/TR/webgpu/ (4th read: §22.2 validation steps) | 200, 4521455 bytes, 0.60 s | fact 1 |
| 08:31:09 | developer.mozilla.org | https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/webglcontextlost_event | 200, 152239 bytes, 0.35 s | fact 1 |
| 08:32:10 | doc.rust-lang.org | https://doc.rust-lang.org/1.99.1/std/primitive.f64.html | 200, 338196 bytes, 0.72 s | fact 2 |
| 08:32:20 | doc.rust-lang.org | https://doc.rust-lang.org/1.99.1/std/primitive.f64.html (2nd read, wider context) | 200, 338196 bytes, 0.20 s | fact 2 |
| 08:32:29 | doc.rust-lang.org | https://doc.rust-lang.org/1.99.1/std/primitive.f64.html (3rd read, rem_euclid) | 200, 338196 bytes, 0.22 s | fact 2 |
| 08:32:38 | doc.rust-lang.org | https://doc.rust-lang.org/1.99.1/src/std/num/f64.rs.html | 200, 111910 bytes, 0.36 s | fact 2 |
| 08:32:46 | doc.rust-lang.org | https://doc.rust-lang.org/1.99.1/src/core/num/f64.rs.html | 200, 207758 bytes, 0.75 s | fact 2 |
| 08:32:50 | doc.rust-lang.org | https://doc.rust-lang.org/1.99.1/src/core/num/f32.rs.html | 200, 210556 bytes, 0.47 s | fact 2 |
| 08:33:03 | doc.rust-lang.org | https://doc.rust-lang.org/1.99.1/src/std/num/f32.rs.html | 200, 111897 bytes, 0.80 s | fact 2 |
| 08:33:39 | doc.rust-lang.org | https://doc.rust-lang.org/1.97.0/std/primitive.f32.html | 200, 346388 bytes, 0.64 s | fact 2 |
| 08:34:42 | doc.rust-lang.org | https://doc.rust-lang.org/1.99.1/std/hash/trait.Hasher.html | 200, 34736 bytes, 0.55 s | fact 3 |
| 08:34:45 | doc.rust-lang.org | https://doc.rust-lang.org/1.99.1/std/hash/trait.Hash.html | 200, 84430 bytes, 0.39 s | fact 3 |
| 08:34:56 | doc.rust-lang.org | https://doc.rust-lang.org/1.99.1/std/hash/trait.Hasher.html (2nd read, write_length_prefix text) | 200, 34736 bytes, 0.57 s | fact 3 |
| 08:34:58 | doc.rust-lang.org | https://doc.rust-lang.org/1.99.1/src/core/hash/mod.rs.html | 200, 92048 bytes, 0.33 s | fact 3 |
| 08:35:44 | doc.rust-lang.org | https://doc.rust-lang.org/1.99.1/rustc/platform-support/wasm32-unknown-unknown.html | 200, 38081 bytes, 0.47 s | fact 3c: no line states the width ("32-bit", "usize", "pointer" not found) |
| 08:35:57 | doc.rust-lang.org | same page (2nd read, broader terms "N-bit", "memory64", "address space", "i32") | 200, 38081 bytes, 0.35 s | fact 3c: no match; 3c stays not read at a source |

## G2: d2 F13's other places (D3B-1). Fresh check at HEAD 100ce01
- No doc changed after the critic's 88fec25: `git log 88fec25..HEAD` gives 9f10ee8 and 100ce01, both checkpoint autosaves;
  `git status` shows only checkpoint files changed.
- `grep -n F13` in the report and the appendix: 0 hits. §7.3's "From the final re-checks" bullet (appendix:606-611)
  still names d1 F3 and F8 and d2 F6, F7 and F14, and nothing of F13.
- The lines are as D3B-1 says: 0003:960 "by side with a legacy screenshot at the same framing", 0003:993 "legacy
  screenshots at the same framing", 0003:1367 "LOOK_V2 items side by side with legacy screenshots"; SPIKE_PLAN:247
  "beside legacy screenshots at the same framing", :255 "beside a legacy screenshot", :267 "legacy screenshots at the
  same framing".
- The deferral's reason holds. LOOK_V2:77-78 reads "judged by Mustafa in the browser, side by side with screenshots
  of the legacy game", and PM-0003-17 (0003:1151) now reads "beside the legacy view (legacy in a second Brave window,
  or its screenshot)". The checklist (0003:1223-1224) opens "legacy itself ... in a second Brave window". A screenshot
  of that window meets both, so the lines are not wrong. They are just not recorded as deferred.
- Out of scope, recorded only: ADR 0005 (1654, 2112, 2242 PM-0005-37, 2405) and README:108 also say "legacy
  screenshot(s)". d2 F13 named only ADR 0003 (d2 verdicts:269-273), and these lines quote LOOK_V2 §9 directly, so no
  new finding.
- Result: G2 OPEN at HEAD. Closing it needs the main session's sentence (D3B-1's smallest fix, d3b verdicts:103) in
  the §7.3 bullet, or graphics-engineer's edit of the three ADR 0003 lines. A verifier cannot do either. Re-check:
  grep "F13" in the appendix, or the three ADR 0003 lines, at the new HEAD.
END G2

## G4: this round's new findings. Fresh check of each at HEAD 100ce01
No doc changed since 88fec25 (see G2), and §7.3 (appendix:584-613) names none of the six: grep "D3B", "D3C" and "seam"
in the appendix finds only §5's decision-map rows (503, 513). For each finding: does it still stand, and is it disposed?
| Finding | Re-check at HEAD (own read) | Stands | Disposed |
|---|---|---|---|
| D3B-1 (low) | see G2: 0003:960, 993, 1367 and SPIKE_PLAN:247, 255, 267 unchanged; no F13 in the appendix | yes | no |
| D3B-2 (low, optional) | 0004:275 "equal an exact reference on every target (PM-0004-22, extended)"; PM-0004-22 (0004:1127) "a correctly rounded Euclidean reference, the exact quotient and remainder by integer or rational arithmetic rounded once to the type" | yes (two names for one test; the pointer carries the meaning) | no |
| D3C-1 (low, cosmetic) | 0002:1424-1426: "... over both transport backends; the" / "   first multiplayer" / "   preview behind the access wall, ..."; 0001:1518 is 156 characters long | yes | no |
| D3C-2 (medium) | 0005:106-107 "the decision map's S5 (content, execution and drawing), S6 (moments and telemetry), S9 (the look and the asset rules) and S10 (the editor)"; README's Seams table (rows at README:328-333) has S1, S3, S4, S6, S11 and S12 only | yes: S5, S9 and S10 dangle once the checkpoint map is deleted | no |
| D3C-3 (low) | GD-REJOIN: only 0001 cites it outside .checkpoints (.gate/ is gitignored scratch); HEAT-SOAK only in README; RELEASE-WINDOW only in 0006; README defines none of them | yes | no |
| D3C-4 (low, count) | README:23 "114 numbered open questions, 112 of them open (OQ-0004-5 and -6 point to ADR 0005)"; OQ-0003-14 (0003:885-886) "Answered by its decider, the verifier (plan step 7, 2026-11-01)"; README's 0003 cell "20 (OQ-0003-1 to -20)"; report:26 "(112 open)" | yes: 111 open | no |
- Skeptic: no record (session notes, touch-up files, appendix) shows the session disputing any of the six, so none is
  owed one yet. This check is an independent second read of each, and all six stand as written.
- d3a C15 (0006:953): the sentence is labelled "(no source read)" inside OQ-0006-16, and that question's options say
  "(not researched)" (0006:955). Rule 2 is met, so a skeptic is optional. Agrees with the critic.
- Result: G4 OPEN at HEAD. The six need a fix or a §7.3 deferral with its reason, from their authors or the main
  session, D3C-2 first. After that, re-check only the changed lines.
END G4

## G3: rule 9, the claims ledgers. Fresh check at HEAD 100ce01
- 16 `*-claims.md` ledgers. `git log 449356c..HEAD` on them: 0 commits. The last commit touching any of them is
  8fa7173 (2026-11-02 10:36:25 UTC). The final round's 604a497 came later (2026-11-02 19:46:02 UTC). Confirms the critic.
- Row 307 (netcode-architect-adr0001-claims.md:340): "the 30 Hz marks may use a stand-in, leaving 7.49 ms". The draft
  at ADR 0001:1046-1047 (OQ-0001-2) says "The 30 Hz baseline marks may use a stand-in (with S1's stand-ins at 30 Hz:
  384 effect rows and a 1 ms burn, the 2.80 ms remainder topped up): on the planning check they leave 4.69 ms
  (16.67 − 9.18 − 2.80)". So the row contradicts its draft (recomputed: 16.67 − 9.18 = 7.49; 7.49 − 2.80 = 4.69).
  The other 7.49 at 0001:963 ("Why 30 Hz": "16.67 − 9.18", no stand-in) is a different, correct case. No finding there.
- Does option 2 match what was done? It does. Each touch-up claim the critic names is a judged row in a verifier's
  claim table: 4.69 ms in d3a C4 (confirmed); 1.97.0's `div_euclid` 3 in d3a's re-judgement of d0 row 9 (lines 96-119,
  own run) and d3b row 5; the Molotov's `pellets` 0 in d3b row 14; the LOOK_V2 Expected lines in d3b rows 29, 31 and 32;
  PM-0004-22's clause in d3b row 5 and the NOTE table (row 67); PM-0003-30's median rule in d3b rows 25 and 26.
  So the verifiers' tables cover them, even though the ledgers do not.
- The report and appendix at HEAD: "ledger" appears in neither the report (0 hits) nor any deviation. In the appendix
  it appears only in the plan's rules (242 rule 15, 358-360 rule 9, 433, 438, 449) and §6 row 15 (538), which says
  "The ledgers and verdict files". No sentence states that the ledgers stop at 8fa7173.
- Result: G3 OPEN at HEAD. Neither option is applied. Option 2 is accurate, as checked above. Its sentence belongs in the
  report's "Decisions and deviations" and in appendix §7.3. Suggested wording: "From the final round on, the claims
  ledgers were not updated (last change 8fa7173); the verifiers' claim tables (fixcheck-d1 sections 1a-4c, d2's
  claims table, d3a section 1, d3b's claims) stand in for them, and ledger row 307's 7.49 ms is superseded by ADR
  0001:1047's 4.69 ms." After the edit, re-check: grep "ledger" in the report.
END G3

## G5 part 1: the skeptic tally (report:34; appendix:563-564)
- Text: "five rounds: 30 upheld, 9 partly, 2 refuted".
- Recount (scratch skeptic_tally.py and .out; every outcome row of each results file):
  | Round | File | Upheld | Partly | Refuted | Its own count line |
  |---|---|---|---|---|---|
  | 1 | verifier-skeptic-1-results.md | 9 | 2 | 1 | :18 "9 upheld ... 2 partly ... 1 refuted" |
  | 2 | verifier-skeptic-2-results.md | 6 | 6 | 1 | none; session checkpoint:231-232 lists the same 6/6/1 |
  | 3 | verifier-skeptic-3-results.md | 4 | 1 | 0 | :15 "4 upheld ..., 1 partly ... None refuted" |
  | 4 | verifier-skeptic-4-results.md | 5 | 0 | 0 | :16 "5 upheld, 0 refuted, 0 partly" |
  | 5 | verifier-skeptic-5-results.md | 3 | 0 | 0 | parts (a)-(c), each "upheld" |
  | 6 | verifier-skeptic-6-d3a-1.md | 0 | 1 | 0 | :46 "Verdict on D3A-1: PARTLY" |
  Rounds 1-5 give 27 upheld, 9 partly and 2 refuted. With round 6: six rounds, 27 upheld, 10 partly and 2 refuted.
- 30 cannot be rebuilt from the files. Counting skeptic-4's grouped rows one by one ("rows 27 and 33", "rows B14, B18,
  B38") gives 30, but counting skeptic-2's "rows 110, A2, TE234" the same way gives 32. No counting rule yields 30.
- Verdict: WRONG (count). Smallest fix, at report:34 and appendix:563-564: "six rounds: 27 upheld, 10 partly, 2
  refuted". If the session prefers to count rounds 1-5 only, "five rounds: 27 upheld, 9 partly, 2 refuted", plus a
  note that skeptic-6 judged D3A-1 (PARTLY).
END G5 part 1

## G5 part 2: "a skeptic re-checked every wrong or unsupported verdict" (report:33-34) and the deviation (report:66-68)
- Text: report:33-34 "A skeptic re-checked every "wrong" or "unsupported" verdict first"; report:66-68 "for time, the
  owner checked four "wrong" verdicts against their sources instead of a fresh skeptic (fixcheck-d0 row 9; fixcheck-d1
  F1, F2, F10)".
- The verdict files (rows read):
  - fixcheck-d0: 1 "wrong in part", row 9 (d0 verdicts:19).
  - fixcheck-d1, count line d1 verdicts:445: "3 wrong (N42, I18, I19); 3 unsupported (N12, I21, I24)". F1 is N42
    (d1:104), F10 is I18 and I19 (d1:334), F2 is part 4's "no correction" (d1:175). N12 (d1:41) "unsupported (a review's
    estimate, its parts not shown)"; I21 (d1:303) "the last unsupported"; I24 (d1:306) "the 15–30 hours unsupported (a
    review's guess)".
  - fixcheck-d2: rows 32 "wrong in part", 34 "wrong", 51 "unsupported as a LOOK_V2 expectation", 73 "confirmed, one
    phrase unsupported", 77 "wrong in one word", 92 and 111 "wrong (structure)", 96 and S12 "wrong (stale)" (d2
    verdicts:40, 42, 59, 81, 85, 100, 119, 104, 306): 9 rows.
  - Skeptic rounds 1-5 all came before d0 (skeptic-5 judged fixcheck-c's B25). Skeptic-6 judged only D3A-1. So none of
    d1's 3 unsupported rows and none of d2's 9 rows had a fresh skeptic. verifier-gdcheck dry-ran the game-designer's
    fix blocks, which is a different check.
- Verdict: WRONG (undercount), as the critic said. Substance: d3a and d3b judged every resulting fixed text against its
  source (d3a section 3; d3b's claims), so no unchecked text results.
- Smallest fix: report:33-34 "A skeptic re-checked every "wrong" or "unsupported" verdict of the drafts' verifiers and
  of fixcheck-a to -c first"; report:66-68 "for time, the owners checked the wrong and unsupported verdicts of
  fixcheck-d0, -d1 and -d2 (1, 6 and 9 rows) against their sources instead of a fresh skeptic; fixcheck-d3 then judged
  every fixed text against its source, and skeptic-6 tested D3A-1".
END G5 part 2

## G5 part 3: the placeholders and the touch-up count
- report:38 "{TOUCHN} in the touch-ups after them", report:43 "d3, on every touch-up: {D3}", report:44 "Scripts at
  {HEAD}", appendix:561 "| The touch-ups after d1 and d2 (fixcheck-d3) | {D3} | | | |": all four still placeholders at
  HEAD 100ce01. They are for the session to fill, not findings.
- {TOUCHN}, own run (scratch rebuild_all.py, the critic's script with its output path moved to this scratch; output
  rebuild_all.out): the nine touch-up fix files changed after 449356c (`git diff --name-only 449356c HEAD`: exactly
  these nine) hold 53 FIX blocks (repin 23, test-engineer 11, infra 7, game-designer 5, netcode 3, and 1 each for
  engine-architect 0002, synthesis, rust-reviewer and test-engineer PM22) and 3 NOTE blocks (rust-reviewer 2,
  test-engineer PM22 1). Applying the 53 to 449356c in commit order gives HEAD byte for byte for all 8 changed docs,
  with 0 BEFORE problems. Value: "53 fix blocks (and 3 notes)". CONFIRMED.
- {D3}, for appendix:561 and report:43, from the five unit files: d3a PASS WITH NOTES (19 claims, 18 confirmed, 1
  unsupported and labelled); d3b PASS WITH NOTES (33 / 33); d3c PASS WITH NOTES (sweeps, D3C-1 to -4); d3d 0 of 3 facts (no fetch);
  this unit's G1 verdicts below. The session composes the line.
END G5 part 3

## G5 part 4: other Evidence numbers and D3C-4's count in the report
| # | Report text | Checked against | Verdict |
|---|---|---|---|
| E1 | report:33 "3,922 claims, 3,847 confirmed (98.1 %)" | appendix §7.1 rows 549-555 (the six ADRs and the synthesis): 629+538+440+665+1,001+588+61 = 3,922; 609+515+431+657+997+581+57 = 3,847; 3,847 / 3,922 = 98.09 % | confirmed (the first verification only; the fix rounds' rows 556-560 are counted apart, as the report's next lines say) |
| E2 | report:44 "521 of 521 research ids disposed exactly once" | verifier-fixcheck-d3c-scratch/dispositions.out "ids: 521; disposed: 521; missing: 0; more than once: 0" | confirmed (d3c's run at 78ccee8; no doc changed since, see G2) |
| E3 | report:45 "README's 94 pointers into the drafts on target" | d3c-scratch/draft_pointers.out "pointers checked 94: ok 94, off 0" | confirmed (as E2) |
| E4 | report:45 "the write guard passes" | d3c-scratch/write_guard.out "GUARD PASS" | confirmed (as E2) |
| E5 | report:26 "114 numbered open questions (112 open)" | OQ-0003-14 answered (0003:885-886); d3c counts-HEAD | wrong (count): 111 open. This is D3C-4; it stands at HEAD (G4) |
| E6 | report:68-70 "fixcheck-d3 then judged the four on their own: every verdict holds, and every fix is right but d0 row 9's wording ... (D3A-1, fixed after a skeptic)" | d3a verdicts:210-211 "Owner-checked verdicts: 4; all 4 hold; the fixes for d1 F1, F2 and F10 are right; d0 row 9's fix is right in its decision, its wording passes on a std claim the toolchain breaks (D3A-1)"; skeptic-6:46 PARTLY; 7cc6942 and b513483 | confirmed |
END G5 part 4

## G1 fact 1: the WebGPU error model behind PM-0003-2 (ADR 0003:1136). Read 08:30-08:31
Source: WebGPU, W3C Candidate Recommendation Draft, 15 September 2026, and MDN's webglcontextlost page. Quotes are in
scratch excerpts.md W1 and W2.
| # | Draft claim (0003:1136) | Source and place | Verdict |
|---|---|---|---|
| 1a | a WebGPU validation error "reaches the page through the device's error event rather than as a thrown error" | createBindGroup's content-timeline steps create and "Return bindGroup"; its validation runs on the device timeline: "generate a validation error, invalidate bindGroup and return" (txt 4588-4595). §22.2: a validation error is "Dispatch[ed] ... to device" (11181-11185). §22.3 dispatch error: when no error scope holds it, "If the user agent chooses, queue a global task ... Fire a GPUUncapturedErrorEvent named "uncapturederror" on device" (11271-11276) | confirmed: not thrown; it arrives as the device's `uncapturederror` event. Qualifier in note N1 |
| 1b | "its `lost` promise resolving" counts as a device error | §22.1: `lost` "remains pending for the lifetime of the device, then resolves when the device is lost" (11139-11141); "No errors are generated by the device loss itself. Instead, the GPUDevice.lost promise resolves" (11118-11119) | confirmed. Qualifier in note N2 |
| 1c | the bite: a bind group that "leaves out one entry of its layout" must count as a device error | createBindGroup: "The number of entries of descriptor.layout is exactly equal to the number of descriptor.entries" is a condition whose failure makes it "generate a validation error, invalidate bindGroup" (4594-4599) | confirmed: the planted group fails validation and is dispatched as an error |
| 1d | "each `webglcontextlost` event on the canvas" | MDN: "HTMLCanvasElement: webglcontextlost event ... fired if the user agent detects that the drawing buffer associated with a WebGLRenderingContext object has been lost" (txt 135, 141) | confirmed (an HTMLCanvasElement event) |
| 1e | "whether each browser also prints it to the console, as an error, a warning or not at all, was not verified" | §22.3 note: "user agents should surface uncaptured errors to developers, for example as warnings in the browser's developer console, unless the event's defaultPrevented is true" (11277-11280); §22.4 example: "adding an event listener may silence console logs" (11412) | confirmed as stated. The spec says "should" and "for example as warnings", so each browser's behaviour really is unverified, and PM-0003-16 records it. The bite's "whatever the console shows" fits the spec |
- N1 (low, other; for graphics-engineer and test-engineer B, the bite's author): the event is not guaranteed. The
  spec fires it "If the user agent chooses" (11273). §22.4 says uncapturederror events "won't necessarily be dispatched
  for all uncaptured errors (for example, there may be a limit on the number of errors surfaced)" (11386). An error
  made inside any open error scope goes to that scope, and no event fires (11267-11270). So if the bite misses, the
  cause may be the browser or a scope the renderer has open, not the hook. Smallest fix (optional): add to the bite
  "planted outside any error scope; if no `uncapturederror` arrives, the hook's own `pushErrorScope('validation')`
  around the planted call decides whether the browser or the hook missed it (WebGPU §22.3, dispatch error: the event
  is fired "If the user agent chooses")".
- N2 (low, other; same owners): `destroy()` resolves `lost` too. GPUDevice `destroy()` runs "Lose the
  device(this.[[device]], "destroyed")" (2327-2329), and GPUDeviceLostReason is "unknown" or "destroyed" (11126-11129).
  A hook that counts every resolution of `lost` would count a renderer teardown that destroys the device. Smallest
  fix (optional): "its `lost` promise resolving with reason "unknown"" (or: the count closes before any teardown).
- Result: fact 1 CONFIRMED (5 claims judged, 5 confirmed), with two low optional notes. The draft labels the device
  events "proposed, not researched". The session may now cite WebGPU §22 (CRD 2026-09-15) for that parenthesis, and
  appendix §7.3's "Not read at their sources" bullet can drop PM-0003-2's model.
END G1 fact 1

## G1 fact 2: rustc 1.99.1's `div_euclid` behind ADR 0004 part 7 (0004:270-278) and PM-0004-22's bite. Read 08:32-08:34
Sources: Rust 1.99.1 std f64 page; 1.99.1 source of std/num/f32.rs, f64.rs and core/num/f32.rs, f64.rs; the 1.97.0 std
f32 page. Quotes in scratch excerpts.md W3-W5. Own check: euclid_alg.py and .out.
| # | Draft claim | Source and place | Verdict |
|---|---|---|---|
| 2a | 0004:271-272 "std's pages promise both methods "the rounded infinite-precision result" (1.99.1's ...)" | 1.99.1 f64 page: div_euclid "§Precision / The result of this operation is guaranteed to be the rounded infinite-precision result." (txt 309-311); rem_euclid the same (330-332). The f32 lines come from d0's excerpt X9 (excerpts.md:119-124) | confirmed for f64 (the lines d0 did not quote) and f32 |
| 2b | 0004:272 "... and 1.97.0's" | 1.97.0 f32 page: div_euclid §Precision (txt 445-447), rem_euclid §Precision (466-467), same words. f64 1.97.0 rests on skeptic-6's decode of the local std metadata (f64_euclid_docs_1.97.txt); the page was not fetched | confirmed (f32 at the page; f64 from local metadata) |
| 2c | 0004:272-274 "yet 1.97.0's `div_euclid` gives 3, not 2, for (3 + 2^-22) ÷ (1 + 2^-23) in `f32`, as does its `f64` analogue" | run on 1.97.0 (d3a scratch div_euclid_check.out: "div_euclid=3 rem_euclid=1"; f64 the same). Own check of the f64 case with 1.99.1's steps in IEEE doubles (euclid_alg.out): 2b + 1 == a exactly; a/b = 3.0; div_euclid 3.0; rem_euclid 1.0 | confirmed (by running; no page states it) |
| 2d | PM-0004-22's bite (0004:1127): "a `div_euclid` planted as rustc 1.97.0's steps (`trunc(a / b)`, corrected by one only when `a % b < 0`: skeptic-6's disassembly)" | 1.99.1 core/num/f64.rs 2028-2034 and f32.rs 2048-2054: "let q = trunc(x / rhs); if x % rhs < 0.0 { return if rhs > 0.0 { q - 1.0 } else { q + 1.0 }; } q"; std delegates (std/num/f64.rs 253-255 "core::f64::math::div_euclid(self, rhs)", f32.rs 253-254) | confirmed: the planted steps are the pinned toolchain's own code |
| 2e | the open point (rust-reviewer touch-up notes, "Whether rustc 1.99.1's code differs is unverified"; d3d prep fact 2) | same source: 1.99.1 uses the same trunc-then-correct steps. Each step is exact or correctly rounded in IEEE 754 (division rounded once, `trunc` exact, `%` exact), so the pin also gives 3 on every target. Own run: the f64 mirrors give −4, −3 and 4 where PM-0004-22 expects −3, −2 and 3; the remainders 1, 2^-52, 1 and 2^-52 match PM-0004-22's | settled: 1.99.1 does not compute differently. Part 7's decision (off the allow-list until PM-0004-22's clause passes) holds for the pin, and the clause would fail on 1.99.1's std |
- D3E-1 (low, optional; rust-reviewer, ADR 0004 part 7). Part 7 shows the failure only for 1.97.0, while ADR 0002:155
  pins 1.99.1, so a reader may think the pin is fixed. Smallest fix: after "as does its `f64` analogue (verifiers
  fixcheck-d3a and skeptic-6, 2026-11-03)", add "; 1.99.1's source takes the same steps (core/num/f32.rs and f64.rs,
  `math::div_euclid`: `trunc(x / rhs)`, corrected only when `x % rhs < 0.0`; verifier, 2026-11-03)". The line count
  may grow by one; README pointers into ADR 0004 would then need a re-pin check.
- The 1.99.1 f64 Precision lines are now saved (W3), as G1 asked. d3b's row 1 holds for f64 at the source.
- Result: fact 2 CONFIRMED (5 claims judged, 5 confirmed). One low optional note (D3E-1). Fact 2 is read, so §7.3's
  "Not read at their sources" bullet need not add it.
END G1 fact 2

## G1 fact 3: std's `Hasher` defaults behind ADR 0004 part 9 (0004:334-341) and PM-0004-21. Read 08:34-08:35
Sources: Rust 1.99.1 std::hash::Hasher and Hash pages, and the 1.99.1 source of core/hash/mod.rs. Quotes in scratch
excerpts.md W6-W8. Local behaviour: d3d scratch hasher_probe.out (1.97.0).
| # | Draft claim | Source and place | Verdict |
|---|---|---|---|
| 3a | 0004:339-340 "a hasher fed through `std::hash::Hasher`, whose derived `Hash` writes slice lengths with `write_usize`" | `impl<T: Hash> Hash for [T]` calls "state.write_length_prefix(self.len())" (mod.rs 931-936), and its default is "self.write_usize(len)" (483-485). The Hasher page: "The impl<T> Hash for [T] includes a call to this method, so if you're hashing a slice (or array or vector) via its Hash::hash method ..." (txt 137-138). Probe on 1.97.0: write_usize(len) before the elements for &[u32], Vec, [u32; 2] and a derived struct's fields | confirmed (through `write_length_prefix`, whose default is `write_usize`) |
| 3b | "... would differ by target at tick 0" (the default `write_usize`) | "fn write_usize(&mut self, i: usize) { self.write(&i.to_ne_bytes()) }" (mod.rs 392-394): the bytes are `usize`'s own width and byte order. Hash §Portability: "Due to differences in endianness and type sizes, data fed by Hash to a Hasher should not be considered portable across platforms" (txt 76-77) | confirmed for std's defaults. A Hasher may override `write_usize` (stable) or `write_length_prefix` (nightly), so it is "would", not "must". Part 9's rule (no `usize`; one byte string hashed by a fixed function) stands either way |
| 3c | "`usize` is 32 bits on wasm32 and 64 bits on x86-64" | probe: `size_of::<usize>() = 8` on x86-64 (1.97.0). The 32 bits on wasm32 come from the target's 32-bit pointer width, but no page read states it: the rustc book's wasm32-unknown-unknown page (two reads, audit 08:35) has no line on it. Portability names only "type sizes" in general | confirmed for x86-64 (run); wasm32 not read at a source (labelled "proposed" in the draft; no change asked) |
| 3d | PM-0004-21's "a hasher fed through `write_usize`" | as 3a and 3b | confirmed |
- Part 9's parenthesis is labelled "proposed". Hash §Portability (W7) now gives it a source: "data fed by Hash to a
  Hasher should not be considered portable across platforms ... Serialization formats intended to be portable between
  platforms or compiler versions should either avoid encoding hashes or only rely on Hash and Hasher implementations
  that provide additional guarantees". Optional for rust-reviewer: cite it ("std's `Hash` page, §Portability, 1.99.1").
  Not a finding.
- Result: fact 3 CONFIRMED (4 claims judged; 3 confirmed at the source, 1 in part: wasm32's width is not read at a
  source). No finding. Appendix §7.3's "Not read at their sources" bullet can drop `write_length_prefix`.
END G1 fact 3

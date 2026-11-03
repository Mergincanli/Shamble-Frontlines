# verifier fixcheck-d3 unit e: verdicts (closes the critic's G1-G5)

Branch claude/laughing-lovelace-ox61se. Started 2026-11-03 08:23 Amsterdam. No doc edited, no commit, no push.
Scratch: .checkpoints/v2-decision-drafts/verifier-fixcheck-d3e-scratch/
Input: verifier-fixcheck-d3-critic.md (G1-G5). Each block ends with END.

## Fetch audit
| time (Amsterdam) | host | URL | status | used for |
|---|---|---|---|---|

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

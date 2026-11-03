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

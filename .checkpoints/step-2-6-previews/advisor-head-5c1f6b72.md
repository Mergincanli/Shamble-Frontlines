# Advisor: review of pushed head 5c1f6b72 (PR #72, step 2.6, risk tier 2)

Scope: the delta b35c6c08..5c1f6b72 against the colleague's findings (advisor-head-b35c6c08.md). Not a full re-review.
Main folder was at 5c1f6b72 with a clean tree (`git rev-parse HEAD`, `git status --short`), so files were read there.

## What changed (git diff --stat b35c6c08 5c1f6b72)
- docs/reports/2026-11-26-step-2-6-previews.md, docs/STATUS.md, the checkpoint, build-notes.md, and the colleague's
  notes file. Nothing under .github/, hosting/, docs/ROADMAP.md, docs/REPO_MAP.md, .claude/ (diff --stat on them: empty).

## Findings of the colleague, one by one
- R6 (Medium): FIXED. Risk 2 now "No token exists yet (his TOKEN-TONIGHT (b)): it waits for the Guard fix chat's
  protection for main"; Proposed next step now "After his GUARD-OFF answer, and once the Guard fix chat's protection for
  main is merged". Matches DECISIONS.md :139-143. Residual wording, Low (N2 below).
- R1 (Medium): FIXED. Report "Decisions and deviations", Process: "The security-auditor rates this High for the kit
  (Critical if the hooks ran and an ask was settled without him), outside this diff and not a follow-up". Source
  round2-security-auditor.md :68-70 ("High while the cause is unknown; Critical if the hooks ran and an ask was settled
  without him ... not a STATUS follow-up"): faithful. STATUS: the kit item is out of the "below the bar" follow-ups
  bullet (line 104) and now in Open risks, Preview hosting (line 118) with the rating and "not a follow-up".
- R2 (Low): FIXED ("likely cause: the bypass box was not saved"; build-notes :191 "Likely cause (not yet confirmed)").
- R3 (Low): FIXED ("refused before any step (B12), the refusal line read through the annotations API (B13)";
  build-notes :224 is the security-auditor's annotations read; :267 TE-G "B13").
- R4 (Low): FIXED ("fixed or moved (de6e4b58, 6fd0375d)").
- R5 (Low): FIXED. "if none reaches him it works as in (b)"; questions file :39 "If no prompt reaches you, it does no
  sensitive step and works as in (b)". The report drops "does no sensitive step", but (b) asks him before each step,
  so the meaning holds.
- D2 (Low): FIXED. "ROADMAP lines 371-377, moved byte for byte to Build history lines 202-208": checked by python;
  main's ROADMAP lines 371-377 == head's 202-208 (True), the heading appears nowhere else in head. (The colleague's
  "8 lines" likely counted the trailing blank line; 7 text lines is right.)
- D1 (Low): LEFT, rightly (STATUS-DIET counts lines; other lanes keep long lines too: 338, 540, 902). But it is
  recorded only in the notes (build-notes :406-407), not in the report or STATUS. H1 below.
- The colleague's N1 (notes timing drift, Low): STILL OPEN, and not listed as left. The checkpoint was edited in this
  push but :101 still says 80a9b6a8 "pushed 2026-11-26 ~00:40 UTC", and build-notes :422 still says "after 01:00 UTC";
  the build notes' fixed/left list (:404-407) does not name it. Low, notes only; rides the next checkpoint update.

## Other checks
- STATUS: 120 lines at head (python on the file). Changed lines (diff): line 104 (From step 2.6, lane 2) and line 118
  (Preview hosting (lane 2)). Both lane 2's own. Every other line unchanged. The kit item's text was not lost: GUARD-KIT
  and the other four stay in Waiting on Mustafa (line 50), TAG-CHECK included.
- Nothing of his recorded as decided while open: STATUS line 50 "five questions, open, not decided"; line 118 "his
  GUARD-OFF, MAIN-WALL-AGAIN, GUARD-KIT and GIT-IN-NODE, open"; the report's Decisions needed 1-5 all open. Decided ones
  named as decided: MAIN-WALL (b), TOKEN-TONIGHT (b) (DECISIONS :139-143). Holds.
- Report vs evidence: the changed sentences match their sources above; the rest unchanged from b35c6c08 (the colleague
  verified it). Code head still 84a3ea44 (no code in this delta).
- CI on 5c1f6b72 (gh api check-runs): gate 139626802946 success, smoke 139626803335 success, tests 139626803216
  success; toolchain 139626803330 and 139626780410 still in progress at 01:25 UTC (read twice); wall-test jobs failure
  (by design).
- His go's item 7 (keep the wall's detail out of every further pushed file), for this delta only: the new Risk 2 says
  "the environment rule (M1) is the wall in front of it" (runbook :19 already titles that page "the wall in front of
  the token"); STATUS :118's added text was moved from :104 (already in b35c6c08) plus the security-auditor's rating;
  the report's Process sentence adds only the rating. Nothing beyond the runbook and DECISIONS. Holds, for this delta.

## New findings
- H1 (Low): D1 is left only in the notes. Name it in the report (or a STATUS follow-up) at the next push, which is
  needed anyway (wall-test.yml out, the verifier). Not a push of its own.
- H4 (Low): the report's Process sentence "The security-auditor rates this High for the kit" now follows the 53
  breaches and the three merges too. The rating (round2-security-auditor.md :68-70) sits under its P6 process finding,
  which reads the guard check, what the 11-25 move brought and the seatbelt; the breach count is not named there. Say
  "rates the guard item" (STATUS :118 attaches it correctly). Wording; rides the next push.
- H2 (Low): Risk 2's tail "the environment rule (M1) is the wall in front of it until then" reads oddly, since no token
  exists until then. Drop or reword the tail. Rides the next push.
- H3 (Low): STATUS's "Last updated" stamp still says "at 8c8ad0d0"; its list of lane-2 sections still holds. Optional.

## Review budget
No new round: this delta only fixes the advisor's findings on docs (the advisor's review of every pushed head, not a
round). Nothing Critical or High open in the diff; the guard item (High for the kit) stays with Mustafa, not a follow-up.

## Own process slip
One Bash call ran `git show 5c1f6b72:docs/STATUS.md | wc -l; git show ... | awk ...` (two git calls, piped, in one
call): a breach of the interim rule (every git call alone, nothing after it). Read-only. Every other git/gh call ran alone.

## Verdict for head 5c1f6b72
ESCALATE (no REVISE). Every finding of the colleague is fixed as claimed (R6, R1, R2-R5, D2) or rightly left (D1). The
head waits on his GUARD-OFF, MAIN-WALL-AGAIN, GUARD-KIT, GIT-IN-NODE and TAG-CHECK, and is not mergeable yet (wall-test.yml
in, no token sitting, no verifier). H1-H4 and the colleague's N1 (all Low) ride the next push. Not APPROVE: five
decisions open, a draft not mergeable, and no gate run by me (read only). Not REVISE: only Lows open.

## Not verified
- The gate run by me (read only; CI gate and smoke green on 5c1f6b72); toolchain on 5c1f6b72 (in progress at 01:25 UTC).
- Anything the colleague listed as not verified (mutation sweeps, npm measurement, replay and breach counts).
- The checkpoint's and build notes' own wording beyond the lines cited.

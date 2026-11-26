# Advisor: review of pushed head b35c6c08 (PR #72, step 2.6)

## Plan
1. Read the report (docs/reports/2026-11-26-step-2-6-previews.md) at b35c6c08.
2. Read `git diff ad2658a b35c6c08 -- hosting/ docs/ .github/`.
3. Read build notes B13-B16, round2-*.md, the checkpoint, advisor-guard.md.
4. Check claims: tests (re-run hosting tests in a worktree? read only: run node --test on the file from git show copy), CI job ids, sha256, reviewer verdicts, DECISIONS.md quotes.
5. Review budget.
6. Docs pass: another lane's lines byte for byte; ROADMAP move vs OLD-LINES (a); STATUS limits.
7. Verdict.

## Blocks

### V1. Reproduced (2026-11-26 ~01:10 UTC)
- Head tree exported by `git archive` to the scratchpad (no worktree, no switch). `node --test hosting/preview.test.mjs`
  (Node 22.23.4): tests 175, suites 13, pass 175, fail 0. Holds.
- sha256 at b35c6c08: preview.yml 31119c06..., preview-close.yml c2f1fa90..., wall-test.yml 3af06706...: equal to his
  NEW-TEXT (a) (DECISIONS :172-175) and his go's item 6 (:213). Holds.
- `git diff --stat 84a3ea44 b35c6c08`: only .checkpoints/ and four docs files: the code head's claims carry to b35c6c08.
- Check runs on 84a3ea44 (gh api): gate 139612150988 success, smoke 139612150684 success, toolchain 139612150897
  success, tests 139612150805 success; wall-test jobs failure (by design). Report's job id holds.
- Check runs on b35c6c08: gate, smoke, tests success; toolchain 139622054405 in_progress at 01:13 UTC (started 01:09).
- `git diff --raw --no-renames origin/main...b35c6c08`: all mode 100644, no symlink or submodule; merge-base is
  origin/main 07d673b0 (main has not moved). No .claude/, CLAUDE.md, gate file in the PR.
- Own process slip: one Bash call ran `date -u;` in front of a `gh api` read (read-only). A breach of the interim rule
  by this agent; every other git/gh call ran alone.

### V2. The report's claims (docs/reports/2026-11-26-step-2-6-previews.md at b35c6c08)
- His decisions quoted correctly against DECISIONS at b35c6c08: 19:36:53 go (:190-215: TRIGGER (a), ACCOUNT (b),
  LANE2-LOCKFILE (b), DEPARTURES (a), MAIN-WALL (b), texts by sha256); NEW-TEXT (a) and PREVIEW-MISSING-BUILD-COMMENT
  (a) (:172-187); TOKEN-TONIGHT (b) (:139-143); OLD-LINES (a) (:1994). "His six messages": 22:56:01, 22:28:33,
  20:51:45, 19:36:53, 17:11:56, 16:41:51 = six. Holds. Nothing of his recorded as decided while open: the five
  questions are "open, not decided" in STATUS and "Decisions needed" in the report. Holds.
- The plan approval: advisor-plan.md :117 APPROVE (revision 2, with conditions). Holds.
- Reviews: round2-infra APPROVE (two Lows; both fixed: preview.mjs messages name "The token"/"To renew", runbook
  rebuild step 4 says steps 2 to 5); round2-test REVISE narrow (N1, N2 Medium; N3 Low); round2-security APPROVE, two
  Mediums (anchors; YAML-continued npm), one Low; fixed in 84a3ea44 (diff read: L0 anchor/alias/merge-key, folded run,
  continuation; 4 planted tests). Holds.
- Test-engineer's conditions met: TE-E plan 2.3 "exact" corrected; TE-F (b) is TAG-CHECK; TE-K stated; TE-H rest in
  STATUS. Holds.
- FINDING Medium (R1): the security-auditor rated the guard item "High for the kit; Critical if the hooks ran and an
  ask was settled without him" (round2-security-auditor.md :68-69, :97). The report (Reviews, Risks 1, GUARD-KIT) and
  STATUS carry the item but not its rating, and the Reviews line reads as if round 2's security review left only two
  Mediums. Fix: one clause in Risks 1 or GUARD-KIT naming that rating (open, with him, not a follow-up).
- FINDING Low (R2): "try 1 failed (the bypass box was not saved)" states as fact a cause the security-auditor said is
  not singled out (B13; the runbook now says "likely"). Fix: "likely because".
- FINDING Low (R3): "the refusal line read through the annotations API (B12)": B12 says the line was NOT read by the
  tools; the security-auditor read it (B13). Fix: cite B13.
- FINDING Low (R4): "Test REVISE (TE-A to TE-J): fixed in de6e4b58 and 6fd0375d": TE-A was partly closed (N1, a
  regression, fixed in round 2), TE-H partly (rest to STATUS), TE-E/TE-F moved to conditions. Fix: "fixed or moved".
- FINDING Low (R5): GUARD-OFF (a) in the report drops the questions file's fallback (no prompt reaches him -> no
  sensitive step, works as (b)). Fix: add the clause (the file is pointed to, so Low).
- The "Correction first" paragraph: quote "only a session with one repository reads" is a faithful cut of the page's
  "only a session with one repository reads it" (advisor-guard.md G1). Process lines match advisor-guard.md G2-G4
  (53 floor; three main-folder merges; "only main's files among protected ones" true with the G3 wording fix, B15).

### V3. Review budget
- Round 1: three reviewers on 8fef0f6; one fix pass (de6e4b58, 6fd0375d). Round 2: infra and test on
  8fef0f6..6fd0375d, security on ad2658a..80a9b6a8 (a later head that already held part 1); one fix pass split in two
  commits (80a9b6a8, 84a3ea44) because the security review arrived after part 1's push. Within CLAUDE.md's budget: two
  rounds, each with one fix pass; no third round launched. The split head for one reviewer stretches "one head" but
  widens coverage; no rule loosened. 84a3ea44 (tests only) was seen by no reviewer: this review read its diff and
  reproduced 175/175 (CLAUDE.md: the advisor's review of every pushed head, not a round).
- Open in the diff: no Critical or High. Open outside the diff: the guard item (security-auditor: High, Critical in one
  branch), correctly sent to Mustafa as GUARD-KIT/GUARD-OFF, not a follow-up. A security finding left open keeps the
  PR needs-mustafa (it is risk tier 2 anyway).

### V4. The docs pass (ad2658a..b35c6c08)
- STATUS: 120 lines at base and head (limit 120). difflib: changed lines are the shared "Last updated" stamp, the new
  #72 row, Next item 3 (lane 2), the Waiting line "Lane 2's #67..." -> "Lane 2's step 2.6 (#72)..." (TR-END tail kept
  word for word), a new follow-ups bullet, "Preview hosting (lane 2)". Every other line byte for byte. Holds.
- FINDING Low (D1): the 120 lines hold because four lane-2 lines are now single lines of 305, 468, 704 and 872
  characters (the three-line Next item 3 became one). Other lanes have long lines too (338, 540), so within the
  letter of STATUS-DIET; a follow-up to keep lines wrapped or trim.
- ROADMAP: change-set 0's entry (8 lines, PR #56) removed from "Recent work" and re-added in "Build history
  (condensed, by system)": byte-for-byte identical (python compare of the line lists). Other inserts: step 2.6's entry
  and #67's dated pointer (lane 2's own; c1da1bd's parents 311c0484 + 35adfbe verified). Holds.
- FINDING Low (D2): OLD-LINES (a) says "Its report lists each line it moved"; the report names the entry, not its
  lines. Fix: name it ("Change-set 0 of the build plan [2026-11-17, MERGED]", 8 lines).
- REPO_MAP: one row for the report. Holds.
- FINDING Low (N1, notes only): timing drift: B16 says 84a3ea44 was pushed "after 01:00 UTC" but its CI finished
  00:44; the checkpoint says 80a9b6a8 pushed "~00:40" against B15's 00:20:58 runs. Fix at the next checkpoint.

### V5. Verdict for head b35c6c08
ESCALATE (no REVISE now). Nothing High or Critical in the diff; the code head's evidence reproduces (175/175, sha256,
CI ids). The head waits on Mustafa's GUARD-OFF, MAIN-WALL-AGAIN, GUARD-KIT, GIT-IN-NODE and TAG-CHECK, and is not
mergeable yet anyway (wall-test.yml in, no token sitting, no verifier). R1 (Medium) and the Lows ride the next push,
which is needed anyway (wall-test.yml out, the verifier); none needs a push of its own.
Not verified: the gate run by me (read only: no worktree; CI gate/smoke green on b35c6c08 and 84a3ea44, gate.yml
unchanged vs main); toolchain on b35c6c08 (in progress 01:13 UTC); the mutation sweeps (32/32, 22/25, 18/18); the npm
10.9.9 measurement; M1's annotations; the replay and breach counts (advisor-guard.md's work, not redone).

### V6. After the advisor tool (added checks)
- FINDING Medium (R6): the report's Risk 2 says "The token sits behind the environment rule alone until" main's
  protection is merged, and "Proposed next step" puts the token sitting right "After his GUARD-OFF answer". His
  TOKEN-TONIGHT (b) (DECISIONS :139-143): the real token waits until the Guard fix chat's protection for main is
  merged; there is no token yet. The report states it right in "For Mustafa" 2, and STATUS states it right (row,
  Waiting, Risks: "no token yet"), and his MAIN-WALL rule re-asks him before the token click, so Medium, not High.
  Fix: Risk 2 "No token yet: it waits for the protection for main (TOKEN-TONIGHT (b))"; next step "after GUARD-OFF and
  once the protection for main is merged".
- R1 widened: STATUS's new follow-ups bullet is headed "below the bar" and holds the guard kit item, which the
  security-auditor rated High (Critical in one branch) and called "not a STATUS follow-up" (round2 :68-70). It is also
  in Waiting on Mustafa and Open risks. Fix: take the kit item out of the "below the bar" bullet and name its rating.
- His go's item 7 ("keep the wall's detail out of every further pushed file"): compared the report's M1 and TAG-CHECK
  lines and the STATUS/ROADMAP additions with runbook :19-32 and DECISIONS: nothing beyond what those already say
  ("1 branch and 0 tags allowed", the bypass box saved, "the Guard fix chat's protection for main"). Holds, for this
  comparison only.
- D1 judged on STATUS's header too ("the rest: STATUS_ARCHIVE"): the bullet keeps its full follow-up text in STATUS
  ("none in the archive yet") where some lanes point to an archive block; guard-env-fix's bullet also keeps text
  inline. Mixed practice: stays Low.
- The guard verdict as stated: the report (Process: "The advisor agent: ESCALATE"; Outcome: "Stopped until his
  GUARD-OFF answer") and STATUS's #72 row state the ESCALATE and the stop correctly; B15/B16 carry the colleague's
  REVISE conditions (10 = 3+3+2+1+1; the two plain merges as the same kind of move; e3b3642b as the branch's own merge
  commit; the platform quote exact). Holds. Not verified: that the "Guard: yes" correction reached him in chat.
- Toolchain on b35c6c08: 139622054405 and 139622016486 completed success (01:15:27, 01:14:37 UTC).
- Also not verified: what the 11-18 and 11-24 main-folder merges brought (taken from advisor-guard.md); any probe of
  the new L0 rules beyond the four planted tests.

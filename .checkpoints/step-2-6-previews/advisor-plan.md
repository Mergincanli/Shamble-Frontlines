# advisor: plan review of step 2.6 (preview links), plan.md revision 1, branch at 081e7e9 (main c1da1bd)

Verdict: REVISE. Risk tier of the build: 2 (.github/, a secret, hosting). Plan reviews are not rounds; the re-review
of revision 2 reads only what changed.

## Checked against the files (2026-11-25)
- Build plan :1021-1025: four conditions as the plan says; :1025 says "Its workflows are CI work" (plural).
- Part 15 :103 (lane 2 owns "the workflow file it adds for previews (a new file), the host's config files it adds, its
  runbook"; must not touch gate.yml, toolchain.yml, gate files, crates/, packages/); :72-74 condition 2 ("every
  Cargo.toml and package.json, both lockfiles ... stay with lane 1 ... lane 1 adds it in a small pull request of its
  own"); :170-180 15.4 (lane 2's preview workflow: "your words of 2026-11-12 on CI work hold there as they are").
- ADR 0002 :82 finding 10; :660-707 part 10 (P1; "No pull_request_target, no workflow_run"; per-job permissions;
  "automatic package caching off"; environment URL = link; "The build generates the _headers file"; header check);
  :1339 PM-0002-65 (link within 5 min of the build's end, median of 5 runs); :1340 PM-0002-66 (reviewer-based, with bite).
- ADR 0006 :128-214 parts 1-3 read directly; :569 B9; :586-600 interim (a)-(d); :1010-1027 OQ-0006-13 (decider "the
  Advisor, with security-auditor; Mustafa for a second account"); :1402 PM-0006-32 (control and bite).
- toolchain.yml: push + pull_request (:13-15), contents read, per-ref concurrency (:20-22), 15-minute job, cut
  :132-197, production build :172, no upload step today. Confirmed.
- #66: :33, :35, :105-109, :113-114, :352, :374-375 (pins), :376-394 (settings, Claude app), App. B :1431, :1478
  (upload pin cf430e03 = section 5's), :1500, :1506 (refuse list), :1512. Confirmed.
- DECISIONS: :1350 (entry 2026-11-19 00:01:25 UTC); :449 STEP4-COUNT (b); :152 WORKERS-NEXT; :461 CI edit is CI
  Builder's; :1758 GATE-FOLLOW (the build chat's gate and CI checks); :450 DEPARTURES (a); :1952 his 2026-11-15 yes to
  the hosting check's workflow texts "exactly as they stand in the plan at 2a2f426", by sha256, "If a review changes one
  line of them, show me the exact change and wait for a new yes"; ACCESS-CARD (a) "If any screen shows a charge, I stop
  and tell you." KIT-FREEZE-2 :29-37 (rules, guard checks, kit redesign frozen; a lane-2 CI file is not).
- READ by me, 2026-11-25, `gh api repos/Mergincanli/Shamble-Frontlines/rulesets/25069670` (read only): ruleset
  "Shamble Ruleset 1", active, on ~DEFAULT_BRANCH: deletion, non_fast_forward, pull_request with
  required_approving_review_count 0, required checks gate and smoke; bypass_actors [], current_user_can_bypass "never"
  (updated 2026-10-17). Branch protection API: "Branch not protected" (404). So MAIN-BYPASS is answered: nobody
  bypasses; no approval is required. Environments, Actions permissions and secrets: refused by this proxy (403).
- NOT verifiable here: the plan's quoted (a)-(d) of his 18:15 message (the checkpoint's Goal holds the lead sentence
  in quotes and a-d as a summary, not his full text; no transcript on disk).

## Answers
1. Scope: a departure for him. Second workflow file: within the build plan's "Its workflows", beyond Part 15's "a new
   file". Scripts: code, not "config files". hosting/package.json + lockfile: cross condition 2 (:72-74), unnamed in the
   plan. Other route: lane 1 adds them on a #59 request. Recommend lane 2's own, outside the workspaces like tools/.
2. His: TRIGGER (:1350), ACCOUNT (OQ-0006-13 decider), section 6 as one decision (:450). Settled by me: FIRST-RUN (a)
   (a rehearsal adds a second sitting and a test token; (a)'s cost: one go of his per tier-2 fix PR, one cycle
   budgeted; the merge report lists the nine first-run unknowns); TOKEN-WINDOW, subject to the security-auditor's read:
   an end date, 6 months, written into click 6; MAIN-BYPASS: answered by the read above, no decision, click 11 goes;
   LABEL (a). OQ-0006-13, my half (subject to the security-auditor's read): (c) is not on offer (docs; the form's
   "Entire Account", #66 :352); (a) is refused by his words (:1350); the answer is D1 plus his ACCOUNT. Under ACCOUNT
   (a), build-plan condition 4's OQ-0006-13 part holds only as "no production exists; previews move at the first
   release", and section 6 item 3 must say so.
3. GATE-FOLLOW does not reach lane 2's file by its words; it reaches CI Builder's toolchain.yml edit. But the plan's
   "as in the hosting check" misstates :1952: then the texts sat IN the plan and his yes came with his go, one stop.
   Decision: revision 2 carries the texts (as then), so the security-auditor reads real text before his go and U2's
   separate stop goes.
4. Departures 1, 2, 4, 5, 7 named right (1, 2, 7 ride TRIGGER). 3: condition 3 already replaced by Part 15 :103 and
   his item 4 (:461, entry 2026-11-24 23:53:15 UTC): a recorded fact. 6: must name :72-74. Missing: PM-0002-65 dropped;
   PM-0002-66 and PM-0006-32 reviewer marks replaced by M1; part 10's environment URL ("View deployment") lost.
5. M1 right in place; needs control, bite, place, removal, a push-ref job, a re-run after click 8. M7's bite cannot run
   (main's header file replaces the build's). The day: right as counted; clock starts at his go on revision 2; CI
   Builder's PR is the likely critical path.
6. security-auditor reads revision 2 before him (xhigh); infra-engineer reviews the final diff (CLAUDE.md step 5; the
   main session builds); test-engineer, engine-architect (xhigh) right; 4 reviewers = batches of 3 + 1, one round.

## Conditions: the sentence the plan changes to
C1 (High) Section 2.5, bullet 2, becomes: "After each deploy, the header check fetches the link with a cookie-less
   client and compares the headers with main's preview set. No build can plant a header file, because main's replaces
   the build's. So the bite works on the fetched responses: the same job strips the `immutable` header from them in
   memory, and the check must fail. That the host serves what a `_headers` cache rule says is already measured (#66
   :33, the planted `no-cache`)." M7 becomes: "the header check passes on the first link, and its bite on the fetched
   responses fails." M2 adds the same bite on a recorded response.
C2 (High) Section 6 item 6 becomes: "Part 15, his lane owners: (a) :103 gives lane 2 'the workflow file it adds for
   previews (a new file)'; here two workflow files and three scripts. (b) :72-74, condition 2: 'every Cargo.toml and
   package.json, both lockfiles' stay with lane 1, and 'lane 1 adds it in a small pull request of its own'; here lane 2
   adds hosting/package.json and its lockfile, outside the workspaces as tools/ is, under DEP-HOLD. The other route:
   lane 1 adds those two files on a #59 request (one more pull request, about a day)." Section 12 adds: "3.
   DEPARTURES: do the departures of section 6 stand? (a) all stand; (b) name the ones that do not. Recommendation: (a)."
   Item 3 adds the OQ-0006-13 sentence of answer 2; the list adds PM-0002-65, PM-0002-66/PM-0006-32 and "View deployment".
C3 (Medium) Click 6: "Token expiration: start date today, end date six months on (2027-05-26 if the sitting is on
   2026-11-26), each in its own field, never the single-date chip." Click 11 is deleted. Section 2.1's sentence "Whether
   main also requires an approval was not read: Mustafa's click 9 reads it" becomes: "Main's rule, read 2026-11-25
   (ruleset 25069670): a pull request with 0 approvals required, gate and smoke required, no one may bypass it."
C4 (Medium) M1 becomes: "A throwaway workflow file on step 2.6's pull request, removed before the final diff's review,
   holds three jobs that name `preview`: two on the pull request (plain, and `deployment: false`) and one on a push to
   the branch. Each prints only 'present' or 'absent'. Bite: before click 5 they run and print 'absent' (that run
   creates `preview` with no rules). After click 5 all three are refused, GitHub's refusal line quoted from the log.
   After click 8, one re-run: still refused." Click 5's "Tell the session" adds: the session reads `preview` from the
   public environments API where it answers (his words of 2026-11-19).
C5 (Medium) Section 7 adds: "M11: from toolchain's end to the link comment, median of the first 5 runs, against
   PM-0002-65's 5 minutes; reported, a miss is a follow-up."
C6 (Medium) Section 2.1's last bullet and R1 add: "preview.yml and every file it reads from main's hosting/ (the
   lockfile, the header file, the scripts; reconcile.mjs runs with the token) are tier 2: any later change gets the
   advisor agent, the security-auditor and his go in a day session. The guard asks before a .github/ edit, not before
   a hosting/ edit; adding hosting/ to its list is a kit change, frozen: a STATUS follow-up."
C7 (Medium) Section 2.1 adds a table: each job of both workflows, its trigger and `if`, environment, permissions, the
   secrets it names, and package caching off (ADR 0002 part 10).
C8 (Medium) Section 10 becomes: U1 host folder and the two workflow files (the texts his go covers); U2 records; U3 the #59 request; U4 the draft PR; U5 his sitting with
   M1; U6 the throwaway file out, review round on the final diff, report, ready; U7 his go; U8 the first link.
C9 (Medium) The quoted (a)-(d) are checked by script against message 42aa43e4 (1778 bytes, sha256 e2554c4e...), or
   lose their quote marks.
C10 (Medium) Revision 2 adds an appendix: preview.yml, preview-close.yml and the throwaway file, each with its
   sha256. "His go covers these texts by sha256; a review that changes a line shows him the exact change for a new yes
   (his rule of 2026-11-15)." U2's separate stop is deleted.
C11 (Low-Medium) Section 2 adds: "The link shows the pull-request run's build, the tested merge of the branch with
   main; the page's corner shows that merge commit; the comment names it and the pull request's head." The Brave
   checklist starts: "Confirm the version: the comment's head equals the pull request's head; the corner equals the
   comment's merge commit."
C12 (Low) Click 1 adds: "If any screen shows a charge or asks for a card, stop and tell the session (as ACCESS-CARD)."
C13 (process) Section 8: the security-auditor reads revision 2 (texts included) before it goes to him; infra-engineer
   reviews the final diff.

## Lows (notes only; none decides REVISE)
- L1 Refuse list in 2.1 drops D1's `_redirects`, dot-segment paths and >20,000 files without a word.
- L2 Workers cost line 2: add the re-check's cost (one sitting of his, about two calendar days).
- L3 2.3: "whether a direct upload counts against the 500 is not answered (PM-0006-7)" (his PASS-WORDS condition).
- L4 STATUS, ROADMAP and REPO_MAP (hosting/ and docs/runbooks/ are new folders).
- L5 Say where the scripts' tests run in CI (preview.yml's no-secret pull_request job) and that they use Node built-ins.
- L6 The immutable rule follows ADR 0006 part 1; production's set must carry it too (GLOSSARY §5).
- L7 Departure 3's date: cite the entry (2026-11-24 23:53:15 UTC; his label 25 November, 00:55 Amsterdam).
- L8 The checkpoint's "Current step" still says step 2.

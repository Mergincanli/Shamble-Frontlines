# Advisor plan review: foundation steps 2.1 and 2.2, plan revision 2 (commit 8bbebfa, base main 44446a5)

Plan of blocks:
- B1 sources: Mustafa's words, plan, checkpoint; rules (CLAUDE.md, advisor.md, STATUS, ADR 0002, SPIKE_PLAN)
- B2 does any rule forbid building 2.1/2.2 that his 2026-11-10 decision does not cover
- B3 verify plan claims against code (gate.mjs, smoke.mjs, village.py, preview.js, verifier.md, guard protectedRel, gate.yml, settings)
- B4 risk tier, merge route, reviewers, Brave checklist, 3-agent cap, models/effort
- B5 the four Mustafa decisions (SPLIT, HC-PATH, SCOPE, MODEL-RULE)
- B6 rulings Q-A1..Q-A6
- B7 verdict and conditions

Findings:
B1 DONE (read Mustafa's words, plan rev2, checkpoint, critic, facts).
B3 checks (all at 44446a5): 63 files / 728 hits reproduced (git grep -c, outside .checkpoints). gate.mjs:32-35 ROOT/FILES/OLD_PREFIX, :92-95 baseFile per-file chain, :96-100 headFile, :150-151 classicParts vs ROOT, :190 runSmoke({root:ROOT}), :333 "reviewed data, not a gate-tool change" upheld. smoke.mjs:17-18, :118, :130, :147-149, :201 upheld. gate.yml:42-50 (PR base origin/$BASE_REF; push base BEFORE) upheld -> F1 (one-PR revert dies exit 2) upheld. preview.js:93-94 reads dir+'/shamble-core.js' upheld. verifier.md:28 upheld. tuning-dump.js:2-5,:50 (no --pre -> prints undefined) upheld: PM-0002-36 as written cannot pass. routine-guard.mjs:26-27 GATE_FILES, :32 protectedRel upheld; 15 protected of 63 upheld. CLAUDE.md:19,:41-42,:46,:52 upheld. ADR quotes 0001:1067, 0002:951, 0004:840, 0006:806 upheld. ADR 0002:998-1007 OQ-0002-2, :1288 PM-0002-36, :1422-1430 migration plan upheld; :139-140 part 1 scopes constraints to legacy/ upheld. START_HERE:21-22,:104,:114-116; REPO_MAP:14-18; STATUS:987, :1009; README:12-13,:19-22 upheld. No 'spikes/' text in the five files or tools/ (git grep exit 1). 8e58937 holds all five under shamble-frontlines-repo/.
Findings:
- F-A HIGH: KIT-FREEZE-2 (Mustafa 2026-11-10 06:33 UTC, word for word on PR #46's branch .checkpoints/kit-v3-5/mustafa-2026-11-10-queue.md item 1): "After kit-v3-5 merges, the kit freezes again until the foundation's gate step. Exceptions: a broken main, and items 2 and 3 below." Later than his 00:10 UTC foundation decision. Plan merges 2.1a (gate files) and 2.1b (CLAUDE.md :19/:41-42, verifier.md:28) after #46 -> inside the freeze. Plan never mentions it. Needs his ruling (new code). Building is not blocked; merge of kit/gate edits is.
- F-B HIGH: MODEL-RULE already answered: his LANE-RULES item 3 (same queue) "Models: on my personal account, Fable 5.2 may be the session's advisor ..." + his 07:08 UTC addition ("it is an extra. The recorded plan review by the advisor agent stays required"). Plan asks it as open and recommends (a) folding it into 2.1b (unrelated rule change bundled into the move PR; collides with LANE-RULES). Drop MODEL-RULE; cite LANE-RULES.
- F-C HIGH (one-word fix): SCOPE says "Your SPIKE-SCOPE (a)": STATUS:739 records it "Taken by the Advisor 2026-10-25", not Mustafa. Re-attribute.
- F-D MEDIUM: Part 4 stale on #46: rev3 (d7526f5, 07:10 UTC) now writes CLAUDE.md and advisor.md too (plan says "only STATUS and ROADMAP"); and his queue adds STATUS-DIET (splits STATUS into STATUS/DECISIONS/STATUS_ARCHIVE, edits REPO_MAP and CLAUDE.md's read list) and LANE-RULES (CLAUDE.md) right after kit-v3-5 (ORDER (b)). 2.1's STATUS/REPO_MAP/CLAUDE.md edits and line refs (:745, :987, :1009) must rebase; his decision goes to DECISIONS.md if STATUS-DIET lands first; never land STATUS edits while STATUS-DIET's line-range script is in flight.
- F-E MEDIUM: models and effort for the build's agents not stated (advisor.md plan checklist; CLAUDE.md Models and effort).
- F-F LOW: reviewers per PR: 2.1a step 5 names security-auditor, test-engineer; infra-engineer (CI, old bases) belongs on 2.1a (1.7 lists it without a PR). Name per PR.
- F-G LOW: Brave checklist format (advisor.md step 5): first line "Confirm you have this version: <how> · <what proves it>", each step with expected result (backup step too), last line "If every step passes, say "merge #<n>"". The two console lines (backup, restore) go into 2.1b's report for security-auditor.
- F-H LOW: E8 (merge + revert -m 1 in a throwaway worktree) writes CLAUDE.md/verifier.md via git; the guard does not read git revert (STATUS L4). Main session runs it (advisor.md Revert step 2: never a subagent), with one ask to Mustafa.
- F-I LOW: E4 should use one copy (A's) of preview.js and tuning-dump.js for base and head runs (2.1b edits both headers; advisor.md:152's spirit).
- F-J LOW: "the merged-PR rule" has no source in the repo; 2.1b's restart of the same branch must be a fast-forward to main (2.1a merged by merge commit), never a force-push.
- F-K LOW: smoke.mjs default root rule ("both pages") differs from gate's BUILD ("all five", both-copies exit 2); share one rule.
- F-L LOW: 2.1a carries his decision's record (decision docs) in a gate PR: a revert of 2.1a also removes it. Accept, or say so in the report.
B2 rules: building 2.1 allowed by his 00:10 decision (CLAUDE.md:19, :21-22, advisor.md:98-99, BRIEF §5, SPIKE_PLAN depends-on, ADR 0002 step 1 overtaken). Not covered: KIT-FREEZE-2 (F-A) for merging. 2.2 merge: CLAUDE.md:44, :52-54 (SCOPE).
Advisor-tool check (close-out): KIT-FREEZE-2 not answered on #46's lane (kit-v3-5.md:24 only queues it; plan-rev3.md:54-55 is that lane's reading: restructure PRs wait for his go until 2.3's allow-list edit; not his words). Gate edit needed under every answer where the move happens -> freeze blocks merges and 2.1b's kit sitting, not 2.1a's build. Keep SPLIT and HC-PATH his; fold HC-PATH into SCOPE (ADR 0002:136-140 contains the move: accepting part 1 settles advisor.md:313 for 2.1b). Evidence runs after #46 under the new guard: re-check commands.
- F-M LOW: SLICE-ORDER condition 3 (STATUS:654, Mustafa and the Advisor 2026-10-31: "The netcode proof test ... still runs before the foundation") missing from step 4's list of lines his decision overtakes (SPIKE_PLAN:86 covers its echo only).
- F-N MEDIUM: SCOPE's list of "no v2 code" lines omits V2_PHASE0_BRIEF.md:20-21 and :144 (the Advisor's document: the session does not edit it, STATUS:700; the claude.ai Advisor must).
- F-O MEDIUM: evidence designed against today's guard; 2.1b (and any 2.1a step after #46) runs under #46's rewritten guard. Re-check E2/E3/E8 commands; a refusal is a finding.
B4 DONE. B5 DONE (SPLIT and SCOPE his; HC-PATH folded into SCOPE; MODEL-RULE dropped; new FREEZE-FDN).
B6 rulings:
- Q-A1: accept E4 as PM-0002-36's reading for 2.1b; use A's copies of preview.js and tuning-dump.js on both sides; record the reading in the register (docs/adr/README.md, 2.1b docs pass, his go) and fix the PM row's wording at ADR 0002's next revision; r5:1245 has the same command.
- Q-A2: (a) tools stay in tools/ (fewest kit edits, also best under KIT-FREEZE-2); legacy docs stay in docs/; gate v2 path deferred to 2.3's plan. Approved.
- Q-A3: provisional (a): G0 mark moves to 2.3 (PM-0002-39 already puts it under --prove); 2.2 keeps "rustc --version in CI equals the pin"; final ruling in 2.2's plan review; done-when change recorded in the register.
- Q-A4: edit verifier.md:28 if FREEZE-FDN allows kit lines; otherwise leave it, record as a known stale command (fails loudly with ENOENT); no preview.js fallback in either case.
- Q-A5: one prefix per base (legacy/, root, shamble-frontlines-repo/ in that order, first with all five); exit 2 naming the base and prefixes tried; print the chosen prefix in the base line and --json.
- Q-A6: reasoning upheld (gate.mjs:92-95, gate.yml:42-50; CI on pull_request tests the merge ref = the revert's tree). Recommend SPLIT (a); his to choose.
B7 verdict: APPROVE WITH CONDITIONS for 2.1 (2.1a build may start; merges and 2.1b's kit sitting wait for FREEZE-FDN, SPLIT; 2.2 needs its own plan review).
DONE

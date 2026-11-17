# Advisor: MAIN-54 plan review (take main 82c605e into PR #40's branch) + DOCS-54 ruling
Started 2026-11-17. Read only. Plan file sha256 read: 3b446794b3f0c976235a887cf4510ad84c097d4697c37f7bf73a0a4870dad328 (matches the brief).

## My plan
1. Main's advisor.md read (git show origin/main:...): done.
2. Plan file read and hashed: done.
3. Facts: rev-parse origin/main; merge-base; diff --stat 53272a1 origin/main; diff --quiet NIGHT_SHIFT; diff --quiet spike tree vs ddd38ba; status --short; HEAD vs origin branch; 82c605e's parents and tree.
4. MAIN-52 words (history.md 2082-2086), his go on #54 (2199-2202), NS-STATUS (2182).
5. Precedent: card lines ~276-320, merge_main_53.py, 2bd44a5 message, counted-merge-2.log header and steps.
6. CLAUDE.md Never: switch rule vs `git merge --no-ff --no-commit origin/main` in main folder; what 2bd44a5 actually ran.
7. CHECKPOINTS.md §5, §9; save pushes.
8. U3C-HELD-1 ruling (c62h-bound-build.md 426-455), card Open findings U3C-HELD-1 and CHECK-NFF-1.
9. main-run.sh header/usage; step 4c command and pass bar; is check.mjs whole needed.
10. DOCS-54: ADV-NS-L5, branch rule, main's STATUS/DECISIONS KIT-FREEZE-2 record.
11. Verdict.

## Facts checked (one line each)
- F1 `git rev-parse origin/main` = 82c605e2e8a7f284e6925718368d3fe569f89ae9; parents 53272a1 20f9d59; tree c2b5dec = 20f9d59's tree. Matches plan.
- F2 `git merge-base HEAD origin/main` = 53272a11024dcec8349e67bd8c70a8db337e85bb. Matches.
- F3 `git diff --stat 53272a1 origin/main` = `.claude/NIGHT_SHIFT.md | 3 +++`, 1 file. Matches.
- F4 `git diff --quiet 53272a1 HEAD -- .claude/NIGHT_SHIFT.md` exit 0. Matches.
- F5 `git diff --quiet ddd38ba HEAD -- spikes/netcode-proof ':(exclude)spikes/netcode-proof/results'` exit 0. Matches.
- F6 `git status --short`: only `?? .checkpoints/netcode-proof-test/advisor-main-54.md` (my own notes file). Tree otherwise clean.
- F7 HEAD = origin/claude/netcode-proof-test-plan-hcky5g = 2a32497 (07:10:43 UTC, checkpoint "MAIN-54 plan written...; saver beoc3blfv"), NOT 4db26b2 as the plan's facts say; 4db26b2..2a32497 changes only the card and main-54-plan.md. The saver is running (beoc3blfv), as the plan foresaw ("the saver starts with this review"). Plan's HEAD fact stale but harmless; step 1 re-checks HEAD = origin branch.
- F8 MAIN-52, his verbatim words: history.md:2055-2057 ("4. Main is now 4460ef5...: PR #52 is merged (...). Do not merge it in now. Take main in when main changes code, CI or rules, and once before PR #40 is marked ready."). history.md:2082-2086 is the main session's "Read as" (label MAIN-52, adds "(the kit, gate or CI files, a CLAUDE.md, .claude/ or .github/)"). The plan (line 4-5) quotes the reading in quote marks as "Mustafa's rule". Meaning same; attribution loose -> finding Low (ADV-M54-L1).
- F9 .claude/NIGHT_SHIFT.md is a rules (kit) file -> MAIN-52 requires taking main in. Merge is due.
- F10 His go on #54: history.md:2199-2202, "Merge #54" (07:00:54 UTC), merge 82c605e 07:03:02 UTC, expected head 20f9d59; CI on main green (gate run 44035715175, toolchain 44035715349). NS-STATUS (history.md:2182) asked for "its own pull request".
- F11 Precedent 2bd44a5: parents 98ffb08 + 53272a1, message -F style with MAIN rationale + both trailers; the command was `git merge --no-ff --no-commit origin/main` (card :990, :387-388), conflict in STATUS.md resolved by merge_main_53.py "Run in the main folder while the merge of origin/main is in progress". Pushed BEFORE check.mjs whole (disk-deadlock ruling, card :307-323); whole ran on ea95701 (counted-merge-2.log: 538/616, refs 1275, self 78, redact 294 items, 0 findings; all 11 PASS).
- F12 Other main merges: 1ea36a1 (2026-11-06, plain `git merge origin/main`, git's default message, NO trailers); df8975b (kit-v3-4, a KIT merge): "the main folder did the plain merge itself (allowed: the branch changed no kit file since the merge base)"; checks gate --no-smoke + check.mjs `--only marks,refs,redact` + repo-map (card :4088-4095). 7685b2b (MAIN-48, toolchain/code): check.mjs whole counted (card :2742-2748).
- F13 GUARD (main's routine-guard.mjs:1308-1362, mainFolderMove): in the main folder `git merge` passes with no ask only as `--ff-only origin/main` or a plain one-word `git merge origin/main` when `git diff --name-only ... origin/main...HEAD` names no protected path (R2, :1347-1351). `git merge --no-ff --no-commit origin/main` = 3 args -> t=null -> why "only origin/main merges into the main folder" -> ask(reason) (:1343-1346, :1362). So the plan's step 2 ASKS Mustafa. Rule since kit-v3-3 (2026-11-05, log -S).
- F14 Branch's own changes since merge base: 1,167 paths, 0 protected names (grep for CLAUDE.md/AGENTS.md/.npmrc/.mcp.json/.github/.claude/gate files: exit 1). `git diff --quiet 53272a1 HEAD -- .claude CLAUDE.md .github <gate files>` exit 0. So plain `git merge origin/main` passes the guard silently; after it, kit = main's byte for byte.
- F15 `git commit --amend` denied by guard (GIT_LONG_DANGER commit: ['--amend'], :323); reset --hard/--soft/--mixed/--keep/--merge denied (:324). So a plain merge's default message cannot be amended, and a committed bad merge cannot be reset locally.
- F16 His 2026-11-16 21:50 words (history.md:2187): "write commands in the simple forms that need no approval, one per call ... When a call has to ask me, name it in your message before you send it." Plan says step 2 is in "the simple command forms" (plan:19) but does not name step 2 as an asking call.
- F17 CHECKPOINTS.md §5 (main): saver background save commits locally and skips push when unpushed commits outside .checkpoints/ exist; foreground `save` pushes (checkpoint.mjs:180-185). A save during an in-progress --no-commit merge would move the branch under MERGE_HEAD (save: commit-tree -p HEAD, update-ref) -> stopping the saver first (plan step 1) is right. Plan doesn't say when the saver restarts (step 6 is an agent run).
- F18 main-run.sh (88 lines): usage `bash main-run.sh <log file> <command...> [:: ...]`; lock, idle wait, header (free, HEAD short, diff/untracked hashes of spikes/netcode-proof only), 4 GB floor (exit 3), touches crates/*. Plan's 4c command form is right.
- F19 Free disk now 4,954 MiB (floor 4,096). Whole run 2026-11-15 21:02 used 6,913->5,891 (-1,022 MiB). A whole run now can leave < 4 GB, stopping the NEXT counted run (U3C-HELD-1's) at the floor; packing is held by R4 until he has read the report (card :196-197). Plan names no free-space bar.
- F20 Last whole check.mjs: .gate/u4r/c62h-bound/merge-2.log at 561bf42 (552/632, 0 failed; refs 1310 tracked; marks 81; self 78/78; redact 10/10, 297 items, 0 findings). Since ddd38ba, results/ gained 8 tracked files (git diff --name-only ddd38ba HEAD -- spikes/netcode-proof/results); spike folder now 305 tracked files, nothing ignored/untracked on disk -> redact will count 305 items, not 297; refs will count about 1,348+ tracked files, not 1,310. Plan's "marks, self and redact as at merge-2.log" would read a correct run as different on redact's item count.
- F21 check.mjs steps reading anything the merge changes: only refs (git ls-files repo-wide; *.md skipped by referenceProblems). Rust steps read spikes/netcode-proof only (own rust-toolchain, .cargo/config.toml target-dir ../../.gate/netcode-proof-test/target); marks runs `cargo run -p npt-judge` (check.mjs:2632-2633) and reads docs/adr/ rows (unchanged by #54).
- F22 Rules on evidence: plan.md §12.4 (:1349-1351) "gate must PASS at every step"; "the spike's own test command ... runs at every unit" (a main merge is not a unit). CLAUDE.md step 4: gate + evidence by type. No ruling found requiring check.mjs whole at every main merge (grep). Card :3788 "After each merge, check.mjs and the gate" is about U4 lane merges.
- F24 Repo and global git config: no merge.ff, mergeOptions, editor or autoedit; commit.gpgsign true (ssh). No local branch or tag named origin/main (rev-parse -q exits 1 both). So a plain `git merge origin/main` merges with ort and commits at once, signed, with git's own message (as at 1ea36a1).
- F25 Pre-check command for the guard's R2 condition verified: `git diff --name-only --no-renames origin/main...HEAD -- .claude .github <gate files> ':(glob)**/CLAUDE.md' ':(glob)**/CLAUDE.local.md' ':(glob)**/AGENTS.md' ':(glob)**/.npmrc' ':(glob)**/.mcp.json'` prints nothing (exit 0); the same pathspec on 53272a1..origin/main prints .claude/NIGHT_SHIFT.md (so it does catch).
- F26 `git merge --abort` in the main folder also asks (args ['--abort'] -> commitOf('--abort') null -> why -> ask).
- F27 DOCS-54: main's docs record nothing after 2026-11-14: STATUS "_Last updated: 2026-11-14 ... main 4d21c1a_" (:5), "Last merged: foundation 2.2 (PR #48 ...)" (:12), In flight lists #52 as final/awaiting go (:29) though merged 4460ef5; #53 and #54 absent (grep counts 0 in STATUS, DECISIONS, ROADMAP, STATUS_ARCHIVE for KEPT-LOGS, KL-MERGE, NS-STATUS, NS-LINES, #53, #54, 2026-11-15/16/17). DECISIONS newest entry 2026-11-13 22:27 (:110). KIT-FREEZE-2 (DECISIONS :27-31) lists exceptions "a broken main, and items 2 and 3 below"; STATUS :15 points there; his two one-change exceptions (KL-YES 2026-11-15 15:35 UTC history.md:2136; NS-STATUS 2026-11-16 20:48 UTC history.md:2185) are not on main. #53's docs diff was REPO_MAP +2 only.
- F28 ADV-NS-L5 (advisor-ns-stints-full.json): Low; fix "docs-keeper records his words and the merge after #54 merges, in a separate docs change". CLAUDE.md standing permission: cloud session limited to a claude/ branch uses it as the stand-in. #54's own branch came from his NS-STATUS words "as its own pull request" (history.md:2182). A docs PR recording his words is a decision-doc record: risk tier 1, needs-mustafa (advisor.md Risk tiers).

## Findings
- ADV-M54-1 Medium: step 2's `git merge --no-ff --no-commit origin/main` makes main's guard ask Mustafa (F13); plan calls its steps "the simple command forms" (plan:19) and doesn't name the ask; his 21:50 rule (F16). Fix: `git merge origin/main` alone (F14 R2 holds; F24).
- ADV-M54-2 Medium: step 4c check.mjs whole is not required (F21, F22); it can drop free space under the 4 GB floor for the next counted run (F19); its pass bar "redact as at merge-2.log" would read the expected 305 items as off (F20). Fix: light check through main-run.sh with an exact bar.
- ADV-M54-3 Low: step 1 order: saver stop must come before the HEAD/clean pre-checks (its last save moves HEAD).
- ADV-M54-4 Low: MAIN-52 quoted from the session's reading as his words (F8).
- ADV-M54-5 Low: plan facts stale (F7): HEAD 2a32497, saver beoc3blfv running.
- ADV-M54-6 Low: saver restart for step 6 not stated (F17).
- ADV-M54-7 Low: tier label "risk tier 2" without the security-auditor; after the merge PR #40's diff to main has no .claude path; the bytes were reviewed on #54. Say so.
- ADV-M54-8 Low (kit, after the freeze, his word): CLAUDE.md Never's "Still allowed there" list omits the plain `git merge origin/main` the guard's R2 admits (F13), used by df8975b and others (F12).
- ADV-M54-9 Medium: main's docs stale for #52's merge, #53 and #54 (F27); DOCS-54.

## Verdict
APPROVE WITH CONDITIONS C1-C7 (exact replacements; no new plan review if applied word for word). DOCS-54: his call (ESCALATE that sub-question).
- C1 saver stop first, then pre-checks incl. the R2 pathspec (F25). C2 `git merge origin/main` alone. C3 no -F commit: git's message; MAIN-52 record + trailers in step 5's card save; verify parents, stat, kit = main's, spike tree. C4 light check through main-run.sh `node check.mjs --only refs,marks,self,redact`, exact bar (N tracked, marks 81, self 78/78, redact 10/10 + M items = spike tracked count (305 today), 0 findings). C5 4a/4b as planned, logs in no-approval forms. C6 push + CI + saver restart before step 6 + Base line. C7 plan text Lows (quote, stale facts, tier line, ADV-M54-8 as kit follow-up).
- Returned via StructuredOutput.
- F23 U3C-HELD-1 ruling (merge-stop C3): "planned and merged before another lane's counted check.mjs run (U4f's merge), so that run does not meet the failure unnamed". MAIN-54 isn't a lane; plan names it in its stop rule. c62h-bound-build.md:428-453: first call FAIL, one second call allowed by ruling, passed whole.

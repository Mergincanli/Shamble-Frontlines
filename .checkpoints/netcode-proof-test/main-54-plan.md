# MAIN-54: take main 82c605e into the netcode branch (plan, 2026-11-17 07:09 UTC)

## As approved
The advisor's review (wf_2dc5e65a-9f8, 07:10-07:27 UTC; advisor-main-54-full.json, 133 lines, feb808d0…, read in
full): **APPROVE WITH CONDITIONS C1-C7** on this plan's first text (sha256 3b446794…). Its conditions replace steps 1,
2, 3 and 4c below (C1, C2, C3, C4), set steps 4a-4b (C5) and add to steps 5-6 (C6); this text carries C7's fixes. In
short: the saver stopped first, then eight pre-checks; `git merge origin/main` alone (the guard's no-ask form), git's
own merge message; then `node check.mjs --only refs,marks,self,redact` through main-run.sh with C4's exact pass bar,
not check.mjs whole (the spike code is unchanged since ddd38ba, whole at merge-2.log; disk 4,954 MiB). The conditions'
own words govern where this summary is shorter.

## Why
MAIN-52, Mustafa's words (history.md:2055-2057): "Take main in when main changes code, CI or rules, and once before PR
#40 is marked ready." (The session's reading, history.md:2082-2086, names what counts: the kit, gate or CI files, a
CLAUDE.md, .claude/ or .github/.) PR #54 merged at 07:03:02 UTC
as 82c605e2e8a7f284e6925718368d3fe569f89ae9 (his go "Merge #54", history.md:2199-2202) and changes a rules file,
`.claude/NIGHT_SHIFT.md` (+3 lines at the end of step 6). The precedent is 2bd44a5, main 53272a1 taken in on 2026-11-15
(the card's lines about "STEP 2" and "DONE 21:02-21:16 UTC"; merge_main_53.py).

## Facts checked at 07:00-07:09 UTC
- origin/main = 82c605e2e8a7f284e6925718368d3fe569f89ae9, parents 53272a1 and 20f9d59; its tree equals 20f9d59's.
- `git merge-base HEAD origin/main` = 53272a11024dcec8349e67bd8c70a8db337e85bb.
- `git diff --stat 53272a1 origin/main`: `.claude/NIGHT_SHIFT.md | 3 +++`, nothing else.
- `git diff --quiet 53272a1 HEAD -- .claude/NIGHT_SHIFT.md` exits 0: this branch never changed that file, so no
  conflict is expected.
- HEAD 2a32497 at the review (07:10:43 UTC; only the card and this plan changed since 4db26b2), the tree clean,
  pushed. The saver beoc3blfv runs with the review (CHECKPOINTS.md §5); C1 stops it before the pre-checks.

## Steps (each in its own call, in the simple command forms)
1. Pre-checks, stop on any difference: the tree clean; HEAD = origin/claude/netcode-proof-test-plan-hcky5g;
   origin/main = 82c605e; the merge base 53272a1; the diff stat as above. The saver stopped before step 2 (TaskStop,
   so its last save runs), as at 2bd44a5.
2. `git merge --no-ff --no-commit origin/main` in the main folder, as at 2bd44a5 (it brings main's reviewed rules
   only). Then `git diff --cached --stat`: `.claude/NIGHT_SHIFT.md` +3 only, no conflict.
3. `git commit -F .gate/main-54/merge-msg.txt` (the message written with Write first; 2bd44a5's form): title "Merge
   main 82c605e into the netcode branch: PR #54 (NIGHT_SHIFT.md step 6, the saver in stints)"; body: MAIN-52, merge
   base 53272a1, no conflict, nothing else changed; the two attribution lines. Not pushed yet: a checkpoint save
   pushes the branch, so nothing is saved until step 4 has passed.
4. Checks on the merge commit, before any push, logs under .gate/main-54/:
   a. `node tools/gate.mjs --no-smoke` (gate.log): GATE PASS (smoke: CI's on the pushed head).
   b. `node .claude/scripts/repo-map-check.mjs`: OK.
   c. check.mjs whole, counted, through main-run.sh:
      `bash /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/main-run.sh /home/user/Shamble-Frontlines/.gate/main-54/counted.log node check.mjs`,
      in the background with a limit of at least 1,500,000 ms. Pass bar, written before it runs: exit 0, "check: all
      11 steps pass"; the header's HEAD is the merge commit, its diff and untracked hashes e3b0c442…; the test step's
      counts equal merge-2.log's at 561bf42 (measured 552, bites 632, 0 failed, 0 skips), since the spike tree is
      unchanged since ddd38ba (`git diff --quiet ddd38ba HEAD -- spikes/netcode-proof
      ':(exclude)spikes/netcode-proof/results'` exits 0, checked again first); refs PASS with the same exception line
      (the three new lines name no spike path); marks, self and redact as at merge-2.log. main-run.sh's own 4 GB floor
      applies.
5. Push with a card save (the card's Base line: main 82c605e merged in at the new SHA). The push carries a
   non-checkpoint change, so CI runs on it: gate, smoke and toolchain green.
6. The advisor's check of the pushed head, as at 717b802, before the next build work (U3C-HELD-1, U4f rev 3) starts
   from it.

## Stop rules
- Any conflict, any staged change beyond NIGHT_SHIFT.md +3, any check FAIL, or a refusal by the guard: stop, nothing
  pushed, the state on the card, and the advisor rules. A FAIL of U3c's held-bound test (U3C-HELD-1, the known timing
  false positive) is still a FAIL here: no re-run without the advisor's ruling.
- A call that would ask Mustafa's approval is named in the chat before it is sent (his words of 2026-11-16 21:50).

## Risks
- Low: the merge commit lands in the main folder, so main's reviewed NIGHT_SHIFT.md becomes the branch's copy at once;
  no settings, hooks or agent file changes (the diff stat shows one file).
- Low: check.mjs's test step can hit U3C-HELD-1's timing false positive (the stop rule above).

## Change type, tier and size
- A merge of main (rules text only): 3 changed lines, main's reviewed and merged text; nothing on this branch is edited
  by hand. It adds no file to PR #40's diff to main (after it, that diff has no .claude/ path), and the three lines
  were reviewed on #54 by the advisor and the security-auditor (security-ns-stints-full.json), so no security-auditor
  run is needed here (ADV-M54-7).
- Evidence: the pre-checks' outputs, the staged diff stat, gate.log, the repo-map check, counted.log, CI on the pushed
  head, the advisor's head check.
- Reviewers: the advisor (this plan, then the pushed head). Mustafa: nothing to check in Brave.

## Follow-ups (not this change)
- ADV-M54-8 (Low, kit, after KIT-FREEZE-2): CLAUDE.md's Never line names three forms as "still allowed" in the main
  folder, while main's guard (R2, routine-guard.mjs:1347-1351) also lets a plain `git merge origin/main` through when
  the branch changed no protected path, the form every main merge on this branch used. Worded for his yes in the kit's
  next change; no edit now.
- ADV-M54-9 (Medium, main's docs): main's STATUS, ROADMAP and DECISIONS record nothing after 2026-11-14 (#52's merge,
  #53, #54 and his words with them, the two one-change exceptions to KIT-FREEZE-2 included). One docs change-set,
  his words copied by script; opening its branch is his call (DOCS-54).

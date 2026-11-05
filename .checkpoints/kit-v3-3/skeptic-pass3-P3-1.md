Task: skeptic pass 3, finding P3-1 (shell comments hide commands; routine-guard parseShell), kit-v3-3 A3.
Plan (blocks): B1 read ruling limits (L1,L15,L16) + scope; B2 read parseShell and line.alone at 01b158c; B3 reproduce in scratch with real git/bash + routine-guard; B4 verdict.
- B1 done: ruling read; routine = wall incl. disguised text (L16: 'routines are refused such lines'). Next B2/B3.
- B2/B3 done: parseShell :140-240 no # rule, confirmed; my probe skp3-p31.out: A1 ordinary apostrophe comment ('what's changed') hides night switch -> null, real bash on cs/evil; A3 hidden push HEAD:main -> null (alone denies); A4 hidden commit -a of CLAUDE.md -> null, really committed. Next B4 severity (branch protection?).
- B4 done: STATUS.md:785 main is protected (PR + gate/smoke required, no force push), so the hidden push to main (A3) is stopped by GitHub; the harm left is the WT-NIGHT one (main folder on a branch with changed settings/hooks for later sessions) and a night commit/push of kit files to a cs/ branch (still PR review + Mustafa's go). Rubric: routine bypass = major.
## Verdict
NOT refuted. In scope: the ruling's routine wall ("every bypass of WT-NIGHT ... fixed now, by construction"; L16 "routines are refused such lines"); SD-1's new alone rule (:515, :826) is defeated; pre-existing on main does not remove it. Not only disguised text: A1, an ordinary comment with an apostrophe ("# what's changed"), hides the next line (so H8 on the day side is accidental reach too, not L16).
Severity: major, not critical (rubric; main's branch protection stops the main push; merges still need review and his go).
Fix: the auditor's fix stands; add: apply the same # rule in sub()'s closer scan (:158-169) and in "..."? (no: # inside quotes is literal), set a parseShell flag `comment` so roles deny any unquoted word-start # (fail closed); rows A1 (apostrophe), A2 (#' no space), A3 push, A4 commit deny; controls echo a#b, git log --grep '#x', ${#x}.
DONE

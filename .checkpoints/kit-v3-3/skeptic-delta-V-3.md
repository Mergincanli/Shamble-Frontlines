Task: skeptic on verifier finding V-3 (plain switch target resolved via rev-parse guessing; tag refs/heads/<name>) at routine-guard.mjs:918, delta 442fbf7..ed3ea8c, kit-v3-3 A3.
Plan:
B1 read code at ed3ea8c around :900-1000 (commitOf, branchCommit, switch path)
B2 read rulings (advisor-fix-round, builder-step9, mustafa-words WT-NIGHT, auto-4/auto-5 #13) for scope
B3 reproduce in scratch under .gate/kit-v3-3 with real git + guard
B4 verdict
Notes:
- B1 done: :917-918 plain name -> commitOf(refs/heads/<name>) = rev-parse --verify refs/heads/<name>^{commit} (DWIM rules incl refs/tags/%s); code new in ed3ea8c (delta). B2 done: no limit (L1,L4,L9,L12-15) covers tags; in scope. Next B3 repro.
- B3 done (skeptic-V3-probe.mjs/.out, git 2.43.0, repos in .gate/kit-v3-3/skv3): fresh clone, no local cs/evil; controls: switch cs/evil DENY, cs/ok allow. Tag refs/tags/refs/heads/cs/evil at cs/ok pushed to origin; guard allows `git fetch origin` and `--tags`; a PLAIN fetch already brings the tag (auto-follow, cs/ok's commit is fetched). rev-parse refs/heads/cs/evil^{commit} = tag's commit; guard on `git switch cs/evil`: allow; real switch: exit 0, HEAD = evil commit, diff vs origin/main = .claude/settings.json. show-ref --verify refs/heads/cs/evil exits 128 (the exact lookup the fix proposes). No test or mutant at ed3ea8c names refs/tags/refs/heads or show-ref: not handled.
- B4 verdict: NOT refuted. Stands as major. Fix as proposed is right (show-ref --verify --hash exact, then refs/remotes/origin/<name>, peel via commitOf on the SHA); note same DWIM also affects the remote leg if refs/remotes/origin/<name> missing (tag refs/tags/refs/remotes/origin/<name>) -> exact lookup on both legs.
DONE

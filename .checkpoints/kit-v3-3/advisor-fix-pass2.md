Task: advisor ruling on A3 (kit-v3-3, PR #38) second fix pass, delta 442fbf7..ed3ea8c, worktree at 71d3544.

## Plan (blocks, in order)
B1 read mustafa-words.md + advisor-fix-round.md (prior ruling, limits L1..L15)
B2 read security-auditor-delta.md + verifier-delta.md
B3 read skeptic-delta-*.md + security-review-auto-4/5.md
B4 read builder-fix1/fix2/step9 notes; spot-check the code for SD-1/SD-2/V-3 shapes
B5 write ruling (per finding, limits, third pass scope, verdict)

## Notes
- B1 done: Mustafa WT-NIGHT (a) with condition 'branch rules, CI, gate files match main's'; prior ruling R-a..R-e, L1-L15, SA-2 routine fix. Next B2.
- B2 done: SD-1..SD-9, V-1..V-5, N-a read; SD-1 fix proposals: SA 'only cd before switch', V 'switch alone'; V-3 fix exact show-ref; SD-2 -z/--no-renames in protectedHit+commit check (C6 lift); SD-4 && after unknown stays unknown + drop '!'. Next B3.
- B3 done: all skeptics uphold. Refinements: SD-2 V-4 skeptic: adding -z to protectedHit breaks main's unedited fakeIo keys (C6 collision) -> fix needs a named C6 edit or keep the old call + add a second rename-aware call; drop .trim(). SD-5 refinement: pos.slice(1) + cloneDir filter ./.. to avoid false ask on project-root source. V-1 refinement: add trap/alias/function to R-e command-word list, not any word. V-3: exact lookup on both legs. SD-3 pre-existing-in-A3 (unchanged mainFolderMove). auto-5 13b refuted. Next B4.
- B4 done: read builder-step9, builder-fix2, guard bash() :495-525 and :836-930 (night switch call :842, commit check :844-850, protectedHit :866-874, switchTargetCheck :893-928), NIGHT_SHIFT.md:20-34, STATUS.md limits :674-718 (L2 :682, L11 :706, L15 :713, "closed" :715-717). Findings of mine:
  A-2: pr-advisor's own step-0 line is `git fetch -q origin main && git switch --detach origin/main` (main's advisor.md): SD-1's sole-command rule must be night-shift only, or it breaks the PR Advisor's seat check. Its target is origin/main itself, so a same-line fetch cannot move it off what is checked.
  A-3: night commit check (:844-850) and push check read the index/refs at hook time too; no night text uses a combined add/commit/push line (grep NIGHT_SHIFT, CHECKPOINTS, test file: none), so the sole-command rule can cover switch, commit and push at no workflow cost.
  A-4: guard.mjs bootstrap exits 0 for a day session when routine-guard.mjs is missing (SD-2's consequence): a follow-up candidate (fail toward ask), not now.
  Decisions: fix SD-1 (+commit/push), V-3 (both legs exact), SD-2 (named C6 edit), SD-3, SD-4, SD-5 (skeptic shape), SD-6 (worktree list), V-1 (3 words in R-e); limits L16, L17 folders, L11 widened (clone/worktree into new folder), L18 parallel calls, SD-7 stays in L2, SD-9 in L9; third pass scoped. Next B5 write ruling.

## Ruling (B5)

**Verdict: PROCEED with the fix list below, then one last, scoped review pass (pass 3).** No question for Mustafa:
his WT-NIGHT condition is clear and the fixes below make the guard hold it; the day ask's scope stays his words, and
the L1 floor plus a new limit L16 stop the day hunt from going on forever. A3 stays a draft until pass 3 is clean.

What I read: main's advisor.md; mustafa-words.md; my four earlier rulings (fix round in full); both delta reviews; the
twelve skeptic notes; automated reviews 4 and 5; builder-step9 and builder-fix2; routine-guard.mjs at ed3ea8c
(:495-525 bash(), :836-930 the night switch call, commit check, protectedHit, switchTargetCheck); NIGHT_SHIFT.md:20-34;
STATUS.md:674-718 (the limits). What I did not do: run the guard, the tests or the gate (the reviewers' and skeptics'
probe outputs agree with the code I read; two skeptics reran each major on real git 2.43).

### The rule I apply
- **Routine paths (the wall):** every bypass of Mustafa's one WT-NIGHT condition, or of the night's commit and push
  checks, is fixed now, by construction (refuse what the guard cannot pin), not by one more parse rule.
- **Day paths (the seatbelt):** fix a miss that an ordinary or accidental command reaches (a typo'd cd, a cleanup
  `rm -rf`, a newer git option, a refactor branch that moves files). A miss that needs deliberately disguised shell text
  goes to L16 (or L1/L15), not into code. One exception below (V-1), because its fix is three words and keeps R-e's own
  claim true.

### Finding by finding
| Finding | Ruling | Shape |
|---|---|---|
| SD-1 = V-2 = auto 12 (major, night switch reads refs before the line runs) | **Fix now** | Night-shift role only: a line holding `git switch`, `git commit` or `git push` must hold that one command and nothing else (no other command, no pipe, fork, background job or substitution); otherwise deny: "run git fetch in its own call first, then git switch <branch> alone". bash() passes "alone" into git(). Covers the fetch form, the `git branch <new> <start> && git switch <new>` form and every other same-line ref mover by construction. **Not for pr-advisor** (A-2: its step-0 line `git fetch -q origin main && git switch --detach origin/main` must keep working; its target is origin/main itself, so the fetch cannot move it off the checked commit) and not for day. NIGHT_SHIFT.md step 4: add "(fetch it first in its own call, `git fetch origin <branch>`, and wait for it)". Rows: C3b, C3c, `git switch cs/ok; git status`, `git switch cs/ok & x`, `git add f && git commit -m m`, `git commit -m m && git push` deny; each alone as before; pr-advisor's step-0 line null; one mutant for the rule. |
| Night commit and push checks read state before the line runs (proposed limit) | **Fix now, folded into SD-1** | Same rule (commit and push alone). A-3: no night text or test uses a combined add/commit/push line, so it costs nothing. Not a limit. |
| V-3 = auto 13a (major, plain switch target through rev-parse's guessing) | **Fix now** | Both legs exact: `git show-ref --verify --hash refs/heads/<name>`, then the same for `refs/remotes/origin/<name>`; peel the SHA with commitOf; neither found: deny as today ("can't resolve"). `-c <start>` and `--detach <ref>` keep rev-parse (git reads them the same way, and the SD-1 rule now fixes the refs for the line). Real-git rows: a tag `refs/tags/refs/heads/<name>` and a tag `refs/tags/refs/remotes/origin/<name>`, each with no matching branch, deny; mutant per leg. 13b: refuted, no change. |
| SD-2 = V-4 = F9-1 (major, plain name lists in protectedHit and the night commit check) | **Fix now, as a named C6 edit** | protectedHit and both commit-check calls add `-z --no-renames --no-relative`, split on NUL, and drop `.trim()` (skeptic). This changes the night push check's git call, so C6's "byte for byte" gives way here, by name: why: a branch that renames `.claude/hooks/routine-guard.mjs` away switches a day main folder with no ask and the day bootstrap then exits 0 (guard off, run end to end by the skeptic); the night push check misses the same rename. A3's fakeIo keys change to the new call (named scaffold edit); the report lists every case of main's test file that now fails for this key, by name, as scaffold. Rows: rename-away, non-ASCII and diff.relative for day `git switch`, night push, night commit (`--cached` and `-a`); a mutant per flag. |
| SD-3 (minor, day main-folder check, same timing gap) | **Fix now** | Ordinary line (`git fetch -q origin cs/x && git switch --detach origin/cs/x`). Day main folder: a `git switch`, `checkout` or `merge` that follows any earlier git command in the same line asks, unless its target word is exactly `origin/main` (keeps WT-UPDATE `git fetch origin main && git merge --ff-only origin/main` and the advisor's step-0 line quiet). Rows: C3d asks; the two origin/main lines null; mutant. |
| SD-4 (minor, cd that bash skips is followed; real overwrite of the main CLAUDE.md) | **Fix now** | Accidental reach (a cd to a missing folder). At dayFolder: a cd joined by `&&` while the folder is unknown stays unknown; drop `!` from CD_REACH. Rows C1, C1c, C2, C3 ask; the `;` control null; a mutant per change. The docs-keeper rewrites STATUS.md:715-717 so "a cd after a failed cd" is closed by this pass, not by 2acfbbc. |
| SD-5 = auto 11 (minor, clone options such as --revision) | **Fix now** | The skeptic's shape: targets are `pos.slice(1)`, plus cloneDir of every positional word except `.` and `..`, plus the git-dir values; add `--revision` to CLONE_VALUE. Rows: `--revision main <url> .claude/sub` and into `.claude` and `CLAUDE.md` ask (agent refused); `git clone <project> <outside>` and `git clone -b main <project> <outside>` stay null; mutant. |
| SD-6 = auto 9, worktree half (minor, `rm -rf .gate`) | **Fix now** | Ordinary cleanup command. kitDir also asks when a path from `git worktree list --porcelain -z` (run from the project folder) is at or under the target; git failing asks. Rows: `rm -rf .gate`, `rm -r .gate`, absolute form, `mv .gate x` ask; `rm -rf .gate/empty` with no worktree null; mutant. Remainder: L17. |
| V-1 (minor, cd through a function, DEBUG trap or alias in the same line) | **Fix now (small)** | Deliberate text in practice, but R-e claims a closed set and the fix is three words: add `function`, `trap` and `alias` to R-e's list as command words (not any word, so `grep function x` stays quiet). Rows P1a-P1e ask; `grep function x` null; mutant. After this, any further day differential of this kind goes to L16 without a code change. |
| SD-7 (nit, perl `-wpi`) | **Stated limit, no change** | L2 already names perl bundles (STATUS.md:682-684). |
| SD-8 = auto 10 (nit, a variable name built by expansion) | **Stated limit L16** | Line below. |
| SD-9 (nit, the night switch check does not fold case) | **Stated limit (L9 widened)** | Line below. |
| V-5 (auto 9-11 confirmed) | Covered | 9 = SD-6, 10 = SD-8 (L16), 11 = SD-5. |
| D-1 (stale STATUS and ROADMAP lines) | **Docs-keeper, final pass** | The verifier's list (STATUS.md:3, :26-27, :92, :145, :152-153, :658; ROADMAP.md:93), plus STATUS.md:715-717 (SD-4), plus L16-L18, the widened L9 and L11, and the follow-ups below. At the final head, re-verify every line reference. |
| N-a (nit, main's file 181/192) | **Report line** | The report's C6 list names 8 scaffold cases at ed3ea8c (181/192), then the counts after SD-2's named edit, each failing case by name. |

### Stated limits (exact lines for STATUS's A3 entry and the report)
Widened:
- **L9, case and Windows spellings:** add at the end: "The night switch check does not fold case either; the night
  shift runs only on Linux."
- **L11, worktrees other than the main folder:** add at the end: "A new clone or worktree made in a folder that is not
  a kit folder (for example `git clone <url> tools/x`) brings its own kit files, such as a nested CLAUDE.md, with no
  ask."
New:
- **L16, deliberately disguised shell text:** a day line written to hide what it does from a text reader (a variable
  name built by expansion, such as `export "${n}IR=…"`, `declare -n`, `printf -v`, `read` or `mapfile` into a git
  variable, or a cd reached through tricks the cd rules do not list) can pass the day checks. The day ask is a
  seatbelt against accidents, not a wall; routines are refused such lines.
- **L17, folder deletes and moves:** a folder target asks when git tracks a kit file in it, when it holds a worktree
  of this repository, or when it is the project folder or an ancestor of it; kit files git does not track, and nested
  repositories that are not worktrees of this one, are not seen.
- **L18, parallel tool calls:** a `git fetch` running in one tool call can move a ref after the guard checked a night
  `git switch` in another call made at the same time. NIGHT_SHIFT.md step 4 says to fetch first and wait.

### Follow-ups (STATUS's follow-ups)
- A-4: the day bootstrap (settings.json:272, guard.mjs) passes when routine-guard.mjs is missing; consider asking
  instead: next kit change-set (SD-2's fix closes today's path to it).
- The existing SA-3, L12, L14, L9, L13 and L1 follow-ups stand as ruled.

### Evidence for pass 2
Gate PASS (smoke included); the three suites with the new rows; a mutant for every new rule and flag above, each
killed; real-git rows for SD-1 (C3b, C3c), V-3 (both tag forms), SD-2 (rename, non-ASCII, diff.relative; day switch,
night push, night commit), SD-3 (C3d), SD-4 (C1, C1c, C2, C3); the null-to-ask movers in A3's file, by name; main's
test file on the new guard with every failure named; the automated review on the fix commit.

### Pass 3: yes, scoped so it converges
Security-auditor and the verifier (Opus 5.5, high), on only the pass-2 diff, plus the automated review.
- **Routine paths at full depth:** the night switch (SD-1's rule, V-3's exact lookups, originMainCheck), the night
  commit and push checks (SD-1's rule, SD-2's flags), and that pr-advisor and every existing routine line are no
  looser. Any bypass of Mustafa's condition here is FAIL.
- **Day paths, ordinary and accidental commands only:** each fix closes its finding (the rows above), adds no false
  ask on the controls, and moves no existing case unnamed. A new day differential that needs deliberately disguised
  text is logged under L16 and is not a FAIL.
- Skeptics only for a major. A pass-3 finding of minor or lower on the day side, beyond a regression of these fixes,
  becomes a limit line or a follow-up, not a pass 4.

### Decisions needed
None. (WT-NIGHT and PC-SETUP are answered; nothing here needs Mustafa.)

### For Mustafa
Nothing yet. The PR stays a draft until pass 3 is clean; then it comes to you to reconfirm the merge (it changes
`.claude/`, CLAUDE.md and the night shift's rules).

DONE

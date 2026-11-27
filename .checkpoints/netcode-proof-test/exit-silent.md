# EXIT-SILENT: this session's Bash tool shows no failure for some silent checks (found 2026-11-27 08:3x UTC)
Written by the main session. A process finding about the evidence form, not about any result: every fact re-checked
below holds.

## The probes (each its own call, 08:3x UTC)
| Command | What failed | What the tool printed |
|---|---|---|
| `test -e /home/user/Shamble-Frontlines/no-such-path-xyz` | the path is absent (exit 1) | "(Bash completed with no output)" |
| `git diff --quiet 076233a f67cd9a` | the range differs (`git diff --stat` shows the card, 2+ 2-) | "(Bash completed with no output)" |
| `grep -q no-such-string-xyz-123 CLAUDE.md` | no match (exit 1) | "(Bash completed with no output)" |
| `false` | exit 1 | "Exit code 1" |
| `cmp -s` on two differing files | exit 1 | "Exit code 1" |
| `git merge-base --is-ancestor HEAD 80b6748` | not an ancestor (exit 1) | "Exit code 1" |
| `ls -la` of two absent paths | exit 2 | "Exit code 2" and "No such file or directory" |

So for `test`, `git diff --quiet` and `grep -q`, a pass and a failure print the same thing, and a recorded "exit 0"
read from that output proves nothing. Exit codes that programs check themselves (tools/gate.mjs, main-run.sh,
check.mjs, ffcheck.mjs, checkpoint.mjs) are not affected: only what a session or an agent reads from the tool.

## Re-checked in forms whose output carries the answer (08:3x UTC): all hold
1. U8-F1's M0 premise (`git diff --quiet 6968215 H0 -- spikes/`, which C1's bars lean on): `git diff --name-only
   6968215 833c3f0 -- spikes/` prints nothing.
2. The check at sending for B's 05:57 message (`git diff --quiet 46c964f HEAD -- spikes/`, HEAD a5f8498; leaned on at
   advisor-u8-f1-merge.md:27 and :31): `git diff --name-only 46c964f a5f8498 -- spikes/` prints nothing.
3. U8-F1's M2: `git ls-tree -r` of the six paths gives the same six blobs at 77ffc95 and at 833c3f0 (d2708d08…,
   9d3ba24b…, 2eeaa574…, c10c13b5…, 1b1e059b…, 5cd34a3d…).
4. U8-F1's merged content: ddb06ff's six blobs (2ad43cac…, 84342b97…, 24122c0e…, c1384ca5…, e77feb06…, eb11e524…)
   equal `git hash-object` of the builder's six files in .claude/worktrees/agent-a7fe1b9f436f36bb1.
5. U8-F1's M3a (spikes/ clean before the apply): covered by M5's log header, .gate/u8-f1-merge/m5-check.log:3, "diff
   4d6a2484…7164" (byte for byte the patch's sha256) and "untracked e3b0c442…" (nothing untracked).
6. U7's M2 (`git diff --quiet 80b6748 <H0> --` the 43 paths): `git diff --name-only 80b6748 dc51f0e -- spikes/` lists
   23 paths, all under crates/judge/ and marks/ (NB2's merge); none of U7's 43 (`git diff --name-only dc51f0e 6968215
   -- spikes/`).
7. A's whole-tree note (advisor-u7-head.md, Notes): `git diff --name-only b9aad01 9c2fcf4 -- . ':(exclude)spikes/'
   ':(exclude).checkpoints/' ':(exclude)docs/'` prints nothing.
8. The `test` checks in the done lines, read now by `ls`:
   - the kept install agent-abf71d95fc07b3522's ops/browser/node_modules is there (143 entries; U8-F1's M7 and U7's
     M7 said "`test -d` OK");
   - .gate/u7-amend and .gate/u7-0b/cli-target are gone ("No such file");
   - .gate/shared-target holds .lane.lock and CACHEDIR.TAG (the part-clean form's `test` checks); its debug folder is
     there again because M5 rebuilt it after M0's part-clean (06:36);
   - the main folder's ops/browser has no node_modules link (M7's unlink).

## One difference, explained and verified
U7's M5 header (.gate/u7-merge/m5-check.log:3) reads "diff 0a4a6eed…", not the patch's 6535c181… The value is
recorded nowhere in the notes, and U7's plan never claimed equality (only U8-F1's done line did). The cause is the
form: worktree_patch.py writes the tracked diff first (`git diff HEAD --binary`, :18), then appends each untracked file
by `git diff --no-index --binary /dev/null <f>` (:19-22), while main-run.sh hashes `git diff HEAD -- .` from
spikes/netcode-proof (:26 `cd "$S"`, :42; the log's line 2 prints that folder), which sorts all paths together. U7's
patch had 33 new files, so the bytes differ in order; U8-F1's had none, so they matched.
- **Verified 08:4x** (a pipe with git first, read only, no file; `git diff --output` was refused by the guard, "not
  allowed here", and not retried): `git diff dc51f0e 6968215 -- spikes/netcode-proof | sha256sum` gives
  0a4a6eedb6a541fc77f1a3d3efd24cb5a73851f60579d7c7f08f98f3a088c7c7, U7's M5 header exactly. So U7's tree at M5 was the
  merged commit's spike content byte for byte. The control, `git diff 833c3f0 ddb06ff -- spikes/netcode-proof |
  sha256sum`, gives 4d6a248425aebb25b317cf2da363e00915e8f2b13eb6f26a6a259a5e0f3f7164, U8-F1's header and patch.

## MAIN-68's silent checks, re-run from commits (08:4x; each by `git diff --name-only`, nothing printed)
- C68B-3, the root manifests equal main's (main-68-plan.md:76, :247, :321): b9aad01..6b4a0d3 and b9aad01..HEAD (with
  `.claude/` too), over Cargo.lock, Cargo.toml, crates, packages, package.json, package-lock.json.
- Step 5's `.claude/` equal to main's (main-68-plan.md:319): b9aad01..6b4a0d3 -- .claude/.
- The anchor (advisor-main-68-head.md:64-66): T 83d650e..6b4a0d3, outside .checkpoints/, docs/STATUS.md and
  docs/STATUS_ARCHIVE.md.
- The whole tree equal to main outside spikes/, .checkpoints/ and docs/ (:67-68): b9aad01..6b4a0d3.
- spikes/, docs/design/ and .checkpoints/ equal H0's (:70): 6b4a0d3^1..6b4a0d3.
- Block 6's precondition (:125-126): 6b4a0d3..4e3d1fb, outside spikes/, .checkpoints/ and docs/design/.
- The positive control: `git diff --name-only 076233a f67cd9a` prints ".checkpoints/netcode-proof-test.md", so an
  empty result is a real "equal".

## R1's own package: two steps a session reads (for B; nothing edited: U8's 6,282 stop)
- spikes/netcode-proof/ops/operator/PROMPT-R1.md:127: "A finding, or any exit but 0, means no push: stop and tell
  him." The laptop session reads an exit code there (the redaction check before the push). Found 09:0x by a wider grep
  of ops/**/*.md for exit-code reads ("exit", "non-zero", "status code", "prints nothing"); PROMPT-R1.md:34's
  "`git config --get core.hooksPath` prints nothing" reads output, so it is not affected.
- spikes/netcode-proof/ops/operator/rules.md:14-15: before reading the run or results worktree, the laptop session
  "checks it: the run worktree's `CLAUDE.md` and `.claude/` equal main's (`git diff --quiet origin/main -- CLAUDE.md
  .claude`) … on a mismatch it stops." If that session's tool hides the exit code as this one does, a mismatch reads
  as a pass. A printing form (`git diff --name-only origin/main -- CLAUDE.md .claude`, nothing printed = equal) is a
  one-line, net-zero rewording; Mustafa approves rules.md at Stop B.
- The other 54 matches under ops/ are inside scripts and tests (r1.sh's nine `grep -q` sit in `&&`, `if !` and `||`;
  wrappers.test.sh 41), where the program reads the exit code itself: not affected.

## The forms from now on (the main session's commands and every agent brief)
- Equal trees or paths: `git diff --name-only` or `--stat`; nothing printed means equal.
- A file present or absent: `ls -d` or `ls -la`; an absent path prints "No such file" with exit 2.
- Equal bytes: `cmp -s` (its exit 1 shows) or a sha256 list.
- Never `test`, `git diff --quiet` or `grep -q` alone as evidence.

## Only three silent forms were probed
The hidden set is not known beyond `test`, `git diff --quiet` and `grep -q`: so the brief sentence names "a command
that exits non-zero without printing", not three names. The routines (the PR Advisor's reviews and merges, the night
shift) run on the same tool, unexamined: on the card's "After the close" list, for him to pull forward if he wants.

## Not re-checked, and the rule proposed for them
The older uses: 293 matches of these forms in 137 notes files, most in agents' files, from U1 on. Proposed rule, for
B's yes or no: an older silent check stands where later printing evidence re-proved the same state (a status listing,
hashes, a counted run, CI); MAIN-68's (recent, on R1's path) are re-run above; any other a later step still relies on
is re-run in a printing form before that step.

## Owed rewordings (before each form's next use, by the seat whose words they are)
- The merge form's M3a and M6 undo (advisor-u7-head.md section 3's words, word for word in u8-f1-merge-plan.md:83-87
  and :108-110): A's; GIVEN 11:15 on 11-27 (advisor-um4-plan.md, "The form's printing wording, given"), used from the
  form's next use.
- The merge form's M2 and M7 (the main session's plan wording, checked by A and B): mine, in the next plan of the form.
- The merge form's M0 check `git diff --quiet 6968215 H0 -- spikes/` (u8-f1-merge-plan.md:72): mine, the same (named
  by A2, advisor-um4-plan.md:149-151: `git diff --name-only`, nothing printed).
- R1's operator files, PROMPT-R1.md:22 and :127 and rules.md:14-15: ruled by B (advisor-u8-f1-head.md, block 3 (c)):
  net-zero rewordings to printed output, by the main session, before Stop B (choice 31).
- light-takein-recipe.md:89 (the anchor) and :107 (step 5 f.): C's words (advisor-light-takein.md:75, and L-9 at
  :97), C's to reword before the recipe's next use; :106 (step 5 e., `.claude/ .github/ tools/`): mine, the same.

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

## One difference, explained
U7's M5 header (.gate/u7-merge/m5-check.log:3) reads "diff 0a4a6eed…", not the patch's 6535c181… The value is
recorded nowhere in the notes, and U7's plan never claimed equality (only U8-F1's done line did). The cause is the
form: worktree_patch.py writes the tracked diff first (`git diff HEAD --binary`, :18), then appends each untracked file
by `git diff --no-index --binary /dev/null <f>` (:19-22), while main-run.sh hashes `git diff HEAD -- .` (:42), which
sorts all paths together. U7's patch had 33 new files, so the bytes differ in order; U8-F1's had none, so they matched.
U7's tree at M5 rests on M4's `git status --short` (exactly the 43), the empty untracked hash, and M8's `git diff
--cached --numstat` equal to the patch's line for line.

## The forms from now on (the main session's commands and every agent brief)
- Equal trees or paths: `git diff --name-only` or `--stat`; nothing printed means equal.
- A file present or absent: `ls -d` or `ls -la`; an absent path prints "No such file" with exit 2.
- Equal bytes: `cmp -s` (its exit 1 shows) or a sha256 list.
- Never `test`, `git diff --quiet` or `grep -q` alone as evidence.

## Not re-checked
The older uses: 293 matches of these forms in 137 notes files, most in agents' files, from U1 on. Most sit beside
evidence that prints (status listings, hashes, counted runs, CI). Whether any older one needs a re-check is for the
advisor (B's head read at 10:50).

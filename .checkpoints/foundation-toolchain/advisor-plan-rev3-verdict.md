# The advisor agent's returned verdict on plan revision 3 (bf1737f), as it came back to the main session

Source: the advisor agent's final report to this session (effort xhigh), 2026-11-10, copied here by the main session so
the plan's record of it can be checked against a file. Its working notes are `advisor-plan-rev3.md` (this folder).
Text as returned, unchanged.

```text
**Verdict: APPROVE** for plan revision 3 at bf1737fe9ffcdb594c0ade078af445e58d1415ff.

I checked first that `git show bf1737f:.checkpoints/foundation-toolchain/plan.md` matches the working copy: no difference.

Revision 3 meets conditions 1 to 5 as I meant them, and the Low items of condition 6. The changed mechanics hold. Every new finding below is Low: fold each in during the build, or when FDN-NOCODE is sent. No new plan review is needed.

Unchanged from my first review:
- Risk tier 2.
- The same reviewers.
- No Brave check.
- It merges on his go, from a day session, after 2.1b.

**FDN-NOCODE is right to send to Mustafa as written.** The polish in L-f is optional.

**Conditions (all Low, fold in during the build; none blocks):**
1. **L-a, step 5:** before the `git switch -c`, the folder must be clean and the branch pushed (`git status --short` prints nothing). Notes not yet committed would otherwise stay in the main folder instead of moving to the worktree.
2. **L-b, the checkpoint's "Running work":** record that the work now lives in `.gate/foundation-toolchain-wt` and that the main folder sits on the new local branch made at origin/main (the `<name>` in step 5). A session that resumes in the same container then moves into that folder. Following §8's `git worktree add` instead would fail, because the branch is already checked out. Also leave the worktree folder before the `git worktree remove` in step 7.
3. **L-c, E11:** every `node_modules/` entry must be one of the five workspace links, or carry a `resolved` address on the npm registry plus an `integrity` hash. Today an entry with neither `link` nor `resolved` would pass without notice. I reproduced the lockfile format with npm 10.9.9, offline:
   - each link: `node_modules/@s/a` = `{resolved: "packages/a", link: true}`;
   - each workspace's own `packages/a` entry and the root entry have no `resolved` at all.
4. **L-d, the probe:** today any curl failure (a TLS error, for example) prints "cut". So "judged on TCP only" claims slightly more than the job checks. Fix: count only curl's exit codes 7 (connection failed) and 28 (timeout) as "cut", or use bash's `/dev/tcp` with a timeout. Turn anything else red as "probe inconclusive". M1a would still catch a network that is really reachable. Separately, if `getent` fails it is fail-closed but exits without its message.
5. **L-e, step 7:** after step 5 the main folder holds main, not the change. The reviewer and verifier briefs must give the worktree path and the head SHA. An agent that reads `CLAUDE.md` in the main folder would be reviewing main's text.
6. **L-f, FDN-NOCODE (optional polish):**
   - Name "your 2026-11-10 exception" in his own words: "may plan and build foundation steps 2.1 and 2.2 now".
   - Say that the line numbers are main 2d22b69's.
   - Optionally, turn K2a into a yes/no confirmation. His prompt says to ask "whether his FREEZE-FDN covers each".

Outside this plan: `.claude/CHECKPOINTS.md` §10 tells sessions to run `git -C <path> status`, which the guard refuses for every session (`routine-guard.mjs:1046`). This needs a kit STATUS follow-up; step 7 runs §10's checks after a `cd`.

**The changed mechanics, one line each:**
- **Condition 2 (branch worktree):** holds. CLAUDE.md's Never bullet allows `git switch -c <new> origin/main` in the main folder. I read the guard's code on main (`:1308-1362`): it lets that switch through without asking, and it does not inspect a merge in any other worktree. `.gate/<slug>-wt <branch>` is §8's own form, and on a real branch the saver runs (§5; it refuses a detached HEAD at `checkpoint.mjs:90`).
- **Condition 3 (pushed by hand, revert committed only after the red result):** holds. Only a foreground save could push a revert early, and Part 7 forbids it. The background saver is harmless here: with code commits ahead it commits locally without `[skip ci]` and skips the push (`checkpoint.mjs:168`, `:180`). So it cannot push a revert early or make GitHub skip a planted commit's run.
- **Condition 4 (E11 and the workspace links):** holds; I reproduced the link format myself. One gap remains, L-c.
- **F-3 (address resolved outside, then `curl --resolve`):** holds. The control and the probe connect to the same IPv4 address, resolved outside the cut, so DNS no longer decides the verdict. One tightening remains, L-d.
- **E2 (`lo` up, other devices printed and noted):** holds. The TCP probe and the planted failures M1a and M1b decide whether the network is cut; extra fallback devices that are down do not change that.

**What I checked myself:**
- The quotes in the plan match main 2d22b69: CLAUDE.md:20-22, :45 and :52; advisor.md:107; STATUS:704-705 and :709-710; and his "effort xhigh" line in the session file.
- The guard refuses `git -C` (`:1046`), as step 5 says.
- The new workflow runs on push and pull_request, with `cancel-in-progress` per ref.
- The fold-check workflow ran its agents at effort high.

Not checked: nothing ran on a real runner. Whether the network cut works is still up to E2 and M1a to M3 in the build.

Notes: `/home/user/Shamble-Frontlines/.checkpoints/foundation-toolchain/advisor-plan-rev3.md` (ends with DONE). The scratch npm test is in `/tmp/claude-0/-home-user-Shamble-Frontlines/29a42dc7-ecd4-506e-b90a-121dd5e50988/scratchpad/ws/`. Nothing was committed or pushed.

**Decisions needed:** none from this review. FDN-NOCODE stays Mustafa's, as the plan writes it.

**For Mustafa:** nothing new from this review. His actions are the plan's "For Mustafa" list, starting with answering FDN-NOCODE.
```

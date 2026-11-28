# Launch 20:1x UTC on 11-27: B4, the advisor agent's round-1 ruling for UM-4
Seat: B a3de649cd25e474df, resumed by SendMessage (it read UM-4's pushed head at 18:03). One seat only; no other agent
runs. Usage 0.58 at 20:09:41 (reset 20:50). The advisor tool read this plan first (20:1x): severity first, the fix
split by owner, the variable list from a printed source, the proof bars, the rest by name.

## B5: the head read of UM-4's fix pass (resume a3de649cd25e474df once CI on 1d4e81a reads all success)
Sent: 21:0x UTC by SendMessage, CI filled (gate push 46066802730, gate pull request 46066808752, toolchain push
46066803492, toolchain pull request 46066808757: all success, attempt 1; PR #40 "blocked", a draft, not "dirty"); done
when advisor-um4-fix-head.md's last line reads "Status: COMPLETE".

```text
UM-4 fix pass: your head read, from the main session (2026-11-27 21:0x UTC), as your ruling's section 5 says (not a round). Your rules as before (main's advisor.md, unchanged since c9bd268; main b8f02b0149cc928964e54e194cd9c4d68f52bd94). Read only, effort xhigh. Your one output file: /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/advisor-um4-fix-head.md: plan first, a notes line after each check, each finished block saved at once, the last line "Status: COMPLETE <UTC time>". Write files with the Write and Edit tools only. No `git -C`, no cd into a worktree; one simple command per Bash call, git first in its own command; run no cargo, test, gate, light run, plant or mutant; never export GIT_DIR or any other git variable in a command; read nothing from the web; name variable names only, never values.

The head: the fix commit f1b466825e8caf9b86aa8a626ac0786d4fba230d (parent H0 a7c4b25bbe46a443493162dc311cfea9954880e6, the saver's termination autosave carrying the resume line), pushed by the save 1d4e81a69cc06a182cea99232bd3bb02a8a4d49c (no skip marker). CI on 1d4e81a: <four runs, filled at send>.

Check, each against its evidence:
1. Every item of your ruling (advisor-um4-r1.md sections 2-4 and 6) against the commit (`git show --stat`, `git diff a7c4b25b f1b46682 -- …`) and um4-plan.md's "FIX PASS DONE LINES": (a) withoutGitVars and opsStep's `base`; (b) the three helper sites in place, numstat 1 1 and 2 2; T-1's stand-ins and exact messages; the web step's Node stand-in; T-2's two cases; T-4's pin; SEC-UM4-3's prefix and its pin built by join; UM4H-1's clause (S's words) and SEC-UM4-2's line in dependency-check.md.
2. The logs under /home/user/Shamble-Frontlines/.gate/um4-fix/, each printed: red-first.log (3 of 3 failing, each for its own reason: the env pin's env assert, the victim test's byte-for-byte assert, the pin's prefix); base-t1, base-web, base-t2, base-t4, base-pin and base-victim .log (1 test each, 0 fail); m1, m3, w1, m4, m10, m8, m-env, m-wild, v1, v2 and v3 .log (each KILLED, and for the right assert: V1-V3 by the byte-for-byte assert, not the title assert); fixed.sha256 against the committed blobs (`git rev-parse f1b46682:<path>` beside `git hash-object` of each fixed-*.mjs copy); counted.log (self 88 of 88, ops 104 of 104 and "315 ok, 0 failed", redact 464 items and 0 findings, refs 2181; its diff line 446f383d… against `git diff a7c4b25b f1b46682 -- spikes/netcode-proof | sha256sum`); gate.txt (GATE PASS).
3. Re-read (a) and (b) against S's probe (sec-um4-1-probe.md): does the victim test exercise the same hazard? Then the whole spike folder, not only ops/ (the u3c fix missed the operator test; the security read missed the redact tests). The main session's `git grep` for git run by name (20:5x; quoted "git" over spikes/netcode-proof without node_modules, and shell `git ` over its .sh files) found:
   - temporary-repository builders, all with GIT_ left out now: check.test.mjs:1512 (u3c C2b F2) and :2122 (the new victim test); ops/tests/operator.test.mjs:157; ops/operator/redact-check.test.mjs:167 and :203 (env from :161 and :197); results/u3c-kernel-check/test_siocoutq_root_check.py:30 (u3c C2b F2);
   - reads only: check.mjs:2730 (the refs step's `git ls-files -z` in ROOT, through `run` with process.env: under an exported GIT_DIR it would list the named repository's files, so the refs check could read the wrong list: rule it beside your UM4R-2); ops/operator/redact-check.mjs:291 (by design); ops/tests/browser.test.mjs:109 (ls-files, now inside opsStep's filtered env); ops/runs/c-u2-pm-0001-3.mjs, c-u4f-cap.mjs, c-u4f-r2.mjs, c-u4r-r2.mjs, c-u4r-runs.mjs (run scripts, not tests: status, rev-parse, diff); results/u3c-kernel-check/siocoutq-root-check.py:164; results/u3d-timing/run.sh:102-103.
   Confirm or extend this list by your own read. Also print m-wild.log's diff showing GIT_DIR in the call's env, and this repository's own state after the four vulnerable runs (red-first and V1-V3): `git branch --list side` printing nothing and `git symbolic-ref HEAD` printing refs/heads/claude/netcode-proof-test-plan-hcky5g (the main session printed both at 20:5x).
3b. The main session's departures from your ruling, each for you to rule: (i) the base is a fresh run at H0 (.gate/um4-fix/light-base.log, refs 2181 with the notes added since), not .gate/um4/light-base.log; (ii) the counted run ran before the commit, on the uncommitted tree, tied by the diff hash (UM-4's form), not on a clean tree at the commit; (iii) the web test's npm ci and tsc asserts were tightened to exact messages too, beyond its Node part; (iv) T-2 is one test with two cases; (v) the missing-folder pin was red first as well (red-first.log); (vi) check.test.mjs grew by +88 net (98/10), above your +45 to 60; (vii) the filter is read in the test through a namespace import (`checkModule.withoutGitVars`), so the file loads on the shipped check.mjs and the pin fails by name.
4. The commit message (`git log -1 --format=%B f1b46682`): it claims only what the logs show; "red first" only for the three tests with a kept red log; the 3-of-5 correction of 8fb32bc8 present; check.test.mjs's +88 against your estimate (named in the report, not a stop).
5. CI on 1d4e81a (`gh run view` per run, read only): all four success, attempt 1, headSha 1d4e81a; PR #40 not dirty.
Then your verdict on the pushed head (APPROVE, APPROVE WITH FINDINGS or REVISE, each finding on the scale), whether UM-4 closes, Decisions needed (self-contained) and For Mustafa, then the status line. Your final reply: the verdict and, one line each, any finding.
```

## B4: the message, as sent
Sent: 20:15 UTC by SendMessage ("Resuming agent a3de649"); done when advisor-um4-r1.md's last line reads "Status:
COMPLETE".

```text
UM-4's round-1 ruling, from the main session (2026-11-27 20:1x UTC). Your rules as before: main's .claude/agents/advisor.md (unchanged since c9bd268; main is still b8f02b0149cc928964e54e194cd9c4d68f52bd94). Read only, effort xhigh. Your one output file: /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/advisor-um4-r1.md. Write your plan first, a notes line after each check, and each finished block at once, so a usage cut loses nothing; the last line "Status: COMPLETE <UTC time>". Write files with the Write and Edit tools only. No `git -C`, no cd into a worktree; one simple command per Bash call, with git first in its own command; run no cargo, test, gate, light run or plant; never export GIT_DIR, GIT_WORK_TREE or any other git variable in a command (the hazard below); read nothing from the web. Name only variable NAMES from this environment, never their values (some may hold credentials).

The round: one review of UM-4's head 8fb32bc8 (pushed in 787eb984); um4-plan.md's done lines, "Review round 1".
- T, test-engineer: review-um4-nb2b-test.md, block 1: T-1 (Medium) and T-2 to T-5 (Low), with T's fix for T-1.
- S, security-auditor: review-um4-security.md: SEC-UM4-1 (Medium), SEC-UM4-2 and -3 (Low), and UM4H-1 confirmed with a corrected clause (:136-138).
- Your own UM4H-1 (advisor-um4-nb2b-head.md:56-60).

Rule on each item below in its own line, with evidence.

1. SEC-UM4-1's severity, on CLAUDE.md's one scale. Its effect: a commit on the current branch with every tracked file deleted, which the background saver (checkpoint.mjs watch) or a later save could push. Its reach: no caller today. This shell has no GIT_DIR, GIT_WORK_TREE or GIT_INDEX_FILE (names printed 20:1x), and main-run.sh sets no GIT_ variable (grep exit 1). The scale puts "breaks ... data" under Critical. The label decides whether a deferral is allowed: a Medium may become a STATUS follow-up, a High or Critical may not. The main session's lean: fix the check.mjs side in this pass, whatever the label.

2. The fix's two halves, by owner.
   (a) The check.mjs side, UM-4's own and outside U8's stop (his U8-FILLS (a), history.md:8137): opsStep passes `runner` an environment without git's repository variables, pinned by a check.test.mjs case. Which list: S's six (review-um4-security.md:51-53), or git's own? `git rev-parse --local-env-vars` printed here at 20:1x: GIT_ALTERNATE_OBJECT_DIRECTORIES, GIT_CONFIG, GIT_CONFIG_PARAMETERS, GIT_CONFIG_COUNT, GIT_OBJECT_DIRECTORY, GIT_DIR, GIT_WORK_TREE, GIT_IMPLICIT_WORK_TREE, GIT_GRAFT_FILE, GIT_INDEX_FILE, GIT_NO_REPLACE_OBJECTS, GIT_REPLACE_REF_BASE, GIT_PREFIX, GIT_SHALLOW_FILE, GIT_COMMON_DIR. This shell sets GIT_CONFIG_COUNT with GIT_CONFIG_KEY_0 to _2 and GIT_CONFIG_VALUE_0 to _2 (names only; also GIT_ASKPASS, GIT_EDITOR, GIT_SSL_CAINFO, GIT_TERMINAL_PROMPT). Does that config set stay or go for the ops step, and why? Never a GIT_* wildcard. GIT_CEILING_DIRECTORIES as well?
   (b) operator.test.mjs's git helper (:157; U8's file, with U8 at its 6,284 stop): the same clearing. Net added lines there need his word (U8-FILLS (a) set the stop), so it becomes a Decisions-needed entry in tonight's day message with your recommendation, unless you find it inside his words.
   May UM-4 close on (a) alone, with (b) a named open item? Check S's claim that an open security finding keeps PR #40 needs-mustafa (review-um4-security.md:165-166, "advisor.md, PR review step 4") against advisor.md's text, and quote it.

3. Proof bars for the fix pass. T-1's lesson: each test red against its own mutant, by name, before green.
   - Never demonstrate SEC-UM4-1 by exporting GIT_DIR in this shell, in main-run.sh or around the real ops step: that would commit the deletion here, and the saver would push it. Red-first for the env fix is unit-level: assert on the environment the stand-in runner receives. An end-to-end demo only on stand-in folders in a scratch folder (S's probe, kept as /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/sec-um4-1-probe.md).
   - The mutants, by name: drop the guard at check.mjs:2696, drop the one at :2700, drop the env strip. Each turns its named test red, with the outputs kept under /home/user/Shamble-Frontlines/.gate/um4-fix/.
   - The run bars: the whole check (13 steps), or the self and ops steps only? And one real run of the fixed ops step in this shell before the commit.
   - The commit message claims only what those logs show, and carries the 3-of-5 correction of 8fb32bc8's "each red by its own name first".

4. The rest, each by name: taken now, or carried (and where).
   - T-1's stand-ins as T wrote them.
   - T-2 (no case where the stdout-only last-line read matters), T-3 (a skipped or todo ops test passes the step), T-4 (the real `present` proven only by the logs), T-5 (forward, infra-engineer's: no CI net for later heads).
   - SEC-UM4-2's version line: "npm 10.9.9, node v22.23.4, read after the install; the registry not recorded at the install (the lock's sha512 integrity binds every tarball)".
   - SEC-UM4-3: OPS_MISSING's prefix made absolute, as the web step's is.
   - UM4H-1's clause in S's corrected wording.
   - The web step's test of the same shape (check.test.mjs:1985-1995).

5. A second round: CLAUDE.md's budget (at most two rounds, the second on what changed only) against NPT-TRIM's "one review round per unit" (history.md:7561-7593). If a second read is wanted: the security seat on the env fix, the test-engineer on the tests.

6. The fix pass's form: a direct commit, as UM-4's was (H0 with a resume line; the saver stopped from H0 to the push; usage re-read before H0), and its size in lines against his U8-FILLS (a).

Plus one line: the security seat's memory pattern (review-um4-security.md:168-171) is off R1's path, so the main session's plan puts it on the card as a follow-up. Agree or not.

Give APPROVE (the fix pass as ruled), REVISE or ESCALATE on each item, and each finding on the scale. End your file with Decisions needed (each entry self-contained) and For Mustafa, then the status line. Your final reply to the main session: your verdict on each item, one line each.
```

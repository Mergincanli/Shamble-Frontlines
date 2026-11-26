# Advisor: U8's head read (6913070f, merge 05c63a8 on H0 9aa6dde), 2026-11-26

Plan: (1) the commits: 6913070's parents and its merge's diff; rev-list and diff 6913070..origin (notes only);
(2) the 43 files against u8-brief rev 1.1, advisor-u8-stopb.md, advisor-u8-r2.md and advisor-u8-m5-ruling.md, and the
merged consumers of what U8 writes; (3) the M5 fix's two files and plants P8-P10 (u8-fix-m5.md); (4) the policy-name
INVALID, C9 on 096808b, test 7's merged-state log, NC-R3-1, the 05:2x clean incident; (5) u8-merge-plan.md's done
lines, u8-report.md, choice 15; (6) the automated review's SR-1 and SR-2; (7) the mode bar; then the verdict and the
findings. The last line is DONE.

Status: all parts done.

## Verdict: APPROVE WITH FINDINGS
The pushed head carries exactly the tree that M5 tested. The merged binaries accept every file U8 writes, as far as I
can read them. Nothing Critical or High is open, so the merge into the lane stands.
- **One Medium (ADV-U8H-1):** R1's run-day text names no file that would bring R1's three judged instruments to this
  container.
- **Four Lows:** two of them are the automated review's SR-1 and SR-2, which I weigh at Low (section 3).
- **What follows:** a small U8-F1 before Stop B (Tue 1 Dec), with a short security-auditor read. Its form waits for
  his answer to RESULTS-PUSH (section 4). One form fits U8's 6,200 stop; my lean needs about 6,230.

## 1. What I checked myself (read only except this file; no cargo, no commit, no switch)
- **The commits.**
  - 6913070's one parent is 05c63a8, and 05c63a8's one parent is H0 9aa6dde.
  - 6913070 adds only files under .checkpoints/.
  - 05c63a8 adds the 43 files under spikes/netcode-proof/ops/ (6,189 lines). It also deletes wip/u8-build.patch and
    changes u8-build.md and u8-report.md. Nothing else changes.
  - Nine commits follow 6913070 up to origin e90731c, not eight: e90731c came after 76e1c32. All nine touch only
    .checkpoints/. The main folder's HEAD, 84ec828, differs from 6913070 only under .checkpoints/.
- **The tested tree is the committed tree.**
  - `git diff 9aa6dde 05c63a8 -- spikes/` hashes to 3e1cd5b5...a531.
  - That is the same value as the snapshot at 9a643f6 and as M5's log header, which reads `HEAD 9aa6dde`, `diff
    3e1cd5b5...` (.gate/u8-merge/m5-check-r.log:2-3).
- **CI on 6913070** (gh run view): push gate 45763469323, push toolchain 45763469324, pull-request gate 45763476998 and
  pull-request toolchain 45763476991 all completed with success, each on headSha 6913070.
  - What CI runs: gate.yml runs the legacy gate and its smoke test; toolchain.yml runs the root workspace's cargo
    tests. Neither runs U8's ops tests.
  - So the evidence for the 43 files is M5's log and my own re-run, not CI.
- **The gate:** `node tools/gate.mjs --no-smoke` on the main folder's tree gives GATE PASS on 1-4 and 5b-5d; smoke ran
  green in CI's gate run on 6913070. U8 touches no legacy file.
- **U8's tests, re-run on the merged tree** (from spikes/netcode-proof):
  - `node --test` on the nine ops test files that need no browser package: 92 pass, 0 fail.
  - `bash ops/tests/wrappers.test.sh`: "315 ok, 0 failed".
  - browser.test.mjs needs ops/browser/node_modules, which the main folder no longer links, so I did not run it. M5's
    log has all 101 pass, item 8's two tests and test 2.2 among them (:430-455).
- **The merged consumers of what U8 writes** (the class of defect M5 found). I read each one against U8's file:
  - **npt-server's `Settings`:** item 8 reads it on every run. The empty pair is the struct's own-certificate form
    (main.rs:119-122, :431-432). With it, the WebSocket serves the same development certificate whose hash goes to
    `hash_path` (main.rs:681-703), so load_settings' WebSocket pin from the hash file is right. INF-SB-1 is now
    closed on the merged code, not waiting for U6b.
  - **npt-load's `LoadSettings`** (bots/src/load.rs:184-222, `deny_unknown_fields`): its 24 fields and the 5 in
    `machine` equal load_settings' keys (r1.sh:340-348). Each value passes the parse checks (:258-296): mode, transport,
    behaviour `brawl`, clients 1-64, duration_s 1-86,400, tick_hz 30, scene SS, the hex pins, the two-digit room.
    npt-load prints READY and DONE as load_ready and the stdout copy read them (:668, :643). It writes a footer on its
    error paths and at its end; a TERM ends it with none, which pass 3's save_log already keeps as cut short.
  - **npt-host:** `serve <path>` (host/src/main.rs:20) and its six settings (serve.rs:57-67) equal host.json and the
    unit's ExecStart. The dist is read through a compiled-in allow-list, so ssh-host-keys.txt in that folder is never
    served as a dist file.
  - **The issue mode and npt-cert:**
    - The line protocol matches: PROFILE, TXT_NAME, TXT_VALUE, `go`, SERIAL, SHA256, DONE (issue.rs:224, :276, :150;
      npt-cert:72-116).
    - The chain is written under `.production.chain.pem.new` and hard-linked into place (issue.rs:170, :273-275).
      That closes INF-SB-6's U6c term, and npt-host.path watches that same name.
  - **npt-super:** it is line-flushed after each record, with a test that reads `room_start` from the file while the
    process runs (super/src/main.rs:218-219, :598-630). That closes F28 in code; R1 still proves it live.
  - **An anchor from a real run:** U6c's container probe run of 24 Nov (.gate/u6c/read-probe/load.json and
    server.json, written by U6c's tree before its merge) used exactly U8's key sets: load_settings' 24 and machine's 5,
    and room_settings' 38 for a probe room, with `ws_cert_chain` and `ws_cert_key` both `""`.
- **The policy-name check and U6c's names, on real output.**
  - I ran probe_presence's own code from the merged r1.sh, in memory, on that run's real leg logs (room-P0.jsonl and
    room-P1.jsonl; the file name pattern adjusted, nothing written).
  - P0: "present at both edges". P1 passed the header, footer, edge, rate, slack-presence and policy checks, then read
    "INVALID: 222 drops on the probe's P1 socket between its reads (C-U8-6)", with the report line "rise 222, up span
    excess 214 intervals, rise - excess 8".
  - So the merged names hold on output U6c actually wrote, not only on fixtures. The leg header reads run
    `C-u6c-probe-read-1-P1`, profile P1, process probe, layer L2: NC-SB-3's per-leg form.
- **C1 stands:** of the 43 files, only r1.sh and ops/tests/r1sh.test.mjs differ between the C9 snapshot (63c3443,
  09cb4a07...) and the fix snapshot (9a643f6, 3e1cd5b5...). lib.mjs and config.mjs are identical.
- **The mode bar:** all 54 files under ops/ in 05c63a8 are 100644 (U8's 43 and the 11 already on the branch). The VMs
  install with explicit modes (rs.yaml's wrappers 0755, sudoers 0440, the units 0644; lg.yaml the same). r1.sh runs
  through bash and the S14 rig through python3. The bar holds.

## 2. The items asked
- **The M5 fix (r1.sh:218-242; r1sh.test.mjs:213-255 and :348; plants P8-P10).** I diffed it between the two snapshots.
  - r1.sh: 12 changed lines. The pair at :230 becomes `chain='""' key='""'`, and two comments are re-pointed. There is
    no other change, and the line count stays 898.
  - The test: +19 lines. Item 8 is now two tests: the 40-field pin, and the per-kind checks. Those read every field
    with its Rust type, mark it optional on `Option<>` or `#[serde(default)]`, and check that no field is unknown,
    every required field is written, each value fits its type, `cc` is absent and `bite` sits only in bite rooms.
    The parse handles `#[cfg(feature = "bites")]` before `#[serde(default)]` on `bite`. A type the checker does not
    name never fits, so a new type fails loudly.
  - Test 2.2 now pins `["", ""]`.
  - The plants each fail by name and are restored with equal sha256: P8 drops a required field; P9 makes `io_threads`
    null and fails the type check; P10 adds a struct field and fails both tests (u8-fix-m5.md:32-39).
  - Sound. No review seat needed: it is the brief's exact text, and its tests prove it.
- **The policy-name INVALID: stands.** It is not an addition beyond the plan.
  - plan.md:1256 lists "the release thread's scheduling policy the same at the measured window's start edge as at the
    header read" among the run conditions that make the self-test INVALID. It goes beyond my NC-SB-9 wording, not
    beyond the plan.
  - It can only make P1 INVALID, never pass it.
  - Comparing the policy's name and not its priority matches "policy".
  - The slack values beside it are a separate gap: ADV-U8H-2.
- **C9 on 096808b:** sound as recorded.
  - Git refused the fast-forward over the builder's own notes file. C9 was then built on 096808b, with U6c's names
    read from blobs equal to d85cb43.
  - Its test 7 ran on the pre-U6c crates, so the merged-state run moved to M5. There it passed: 101 of 101 on H0 plus
    the patch.
  - My real-log run above adds U6c's own output.
- **Test 7's merged-state run** (m5-check-r.log): ops 101 of 101; wrappers 315 ok; check.mjs 11 of 11 (861 measured
  and 958 bites tests, 0 failed; refs 1,982 = 1,939 + 43; marks 81; redact 421 items, 0 findings).
- **NC-R3-1 (Low): carried as u8-report.md has it.** save_log runs unguarded under `set -e`, but it fails closed: the
  block reads stopped, and nothing is judged wrongly. Its taker is the STATUS follow-up named in U8's report. It
  takes no line of U8-F1's budget.
- **The 05:2x clean incident** (u7-block0b.md:25-44; 25 Nov, U7's Block 0b): it removed the shared build folder.
  - It touched no U8 file, log or result. U8's M5 ran a day later, cold, and passed.
  - The merge's three part-cleans on 26 Nov each record the dev part-clean's signature: the lock and CACHEDIR.TAG
    kept, no debug folder (u8-merge-plan.md:69-70, :101-103, :129-131). That fits the rule written after the
    incident.
  - Nothing for U8. It is named to him as choice 5.
- **u8-merge-plan.md's done lines (DONE 05:2x restart to M9) and u8-report.md:** they agree with the commits, the log
  and the hashes above. Two corrections to the notes are ADV-U8H-4.
- **Choice 15 stands,** on my M5 ruling's terms (advisor-u8-m5-ruling.md:62-70). Its term (i), that every whole run
  runs the ops tests and wrappers.test.sh beside check.mjs until the `ops` step lands, binds U7's merge's counted run.
  That run has not happened yet, so it is a check for U7's head read.

## 3. The automated review of 05c63a8 (data, weighed): SR-1 and SR-2
- **What each says, checked against the file:**
  - SR-1: Bash's hooksPath denies (settings.json:33-36) are written in one letter case, while git's config keys are
    not case-sensitive. Neither shell denies the `--worktree`, `--system` or `--file` forms.
  - SR-2: Bash has `git -c *` (:32) but no `git * -c *` beside PowerShell's (:56), and nothing covers the GIT_CONFIG_*
    variables.
  - Both hold as written.
- **My weight: Low each, against the automated Medium.**
  - Both sit in the layer the package itself calls best-effort. The docs record says a Bash rule "isn't a security
    boundary around the program" (u8-sec-sb-3-docs.md). rules.md:34-41 says "every other command prompts, and his
    prompt is the wall".
  - SEC-SB-7, the same class, was rated Low by the security seat for this reason.
  - Every config write and every push prompts him in Manual mode; nothing in SR-1 or SR-2 runs without his prompt.
- **The real gap is timing.** Start check 7 reads `core.hooksPath` once, at R1's start (PROMPT-R1.md:33-34). A change
  made later, by a command he approves without noticing, would skip the hook, and with it redact-check, at the push
  hours later.
- **Fix (SR-1): one line in PROMPT-R1.md section 6, step 2, run in PowerShell just before the push:**
  `git -C <results> rev-parse --git-path hooks`, where `<results>` is the folder the push uses. It must print the
  clone's own hooks folder, the one holding pre-push; anything else means stop.
  - Not `git -C <results> config --get core.hooksPath`: PowerShell matches rules without regard to case, so
    `PowerShell(git -c *hookspath*)` (settings.json:55) would refuse it.
  - Not the plain `git config --get` from the clone root: that never reads the results worktree's own config.
  - The rev-parse form spells no "hookspath", so it matches no deny (:55-56, and no `git config` or `git push` form).
  - It may prompt: Claude Code's read-only set is documented for Bash only. That is one prompt, with him present.
  - Checked here: `git rev-parse --git-path hooks` printed `.git/hooks`. With GIT_CONFIG_COUNT=1,
    GIT_CONFIG_KEY_0=core.HooksPath and GIT_CONFIG_VALUE_0 exported, it printed the variable's value.
    `git config --get core.hookspath`, in lower case, printed it too, with `--show-scope` reading "command".
  - Not checked here: the config-file layers. Writing a config file is outside this read-only task.
  - So U8-F1's builder proves it on a throwaway repository. A hooks path set in lower case through `git config`,
    `--worktree` and `--file .git/config`, and through GIT_CONFIG_*, must each change the printed path. The test fails
    if any one does not.
  - What the line catches: every lasting form (any letter case, scope or config file git reads) and the GIT_CONFIG_*
    forms set in the same shell.
- **SR-2's other forms** are `-c` or `--config-env` inside the push command itself, and those are shown in full in the
  push's own prompt (the ask rule). Its environment-variable forms are caught by SR-1's line.
  - So no deny is added. More denies would add a layer the docs say is no boundary, and each new deny needs the
    operator test's check against the prompt's own git commands (SEC-R2-3's lesson).
  - SR-2 is addressed by SR-1's line plus the push prompt. If that line does not fit the budget, both are carried
    together (below).
- **A security seat: yes, for U8-F1, not for this weighing.** security-auditor at xhigh, one short read of U8-F1's
  PROMPT-R1.md and R1.md lines: the pushed set's exposure and redaction (ADV-U8H-1), and the re-read line. He approves
  those two files by hand at Stop B.

## 4. Findings (CLAUDE.md's scale)
| Id | Sev | What | Ruling |
|---|---|---|---|
| ADV-U8H-1 | Medium | R1's run-day text names no file for R1's three judged instruments. PROMPT-R1.md:121-123 pushes "npt-judge's summaries and the redacted records", and nothing defines "the redacted records". npt-judge's verdicts carry no self-test numbers: no marks.json series reads `l2_late_*`, `ws_split` or `ws_late_*`. Yet S14's model side needs the kernel folder copied here (R1.md:190-193). ImpairSocket's self-test is judged by the advisor "with its printed E, its max and the counters" (plan.md:1256). Condition 10 reads `ws_split`'s fields. U6c's real run puts P1 under the drop rule, so that P1 case is likely at R1 too. The workaround is the raw tarball, which stays on his laptop until the close (R1.md section 7), pushed by a later sitting at the cost of his time. Checked: `grep -c 'l2_late\|ws_split\|ws_late'` finds 0 in marks.json, judge/src/figures.rs and eval.rs | **Fix in U8-F1; the form is his (RESULTS-PUSH, below).** Every option names in PROMPT-R1.md section 6 step 2 and R1.md section 7: `results/r1/collect/`, `s14/` whole (the approved brief already has the model side run here on "the copied folder": R1.md:190-193), `probe/presence.txt` and `bringup/facts.txt`. They differ only in how the two self-tests' numbers come here. Test 5 asserts the list |
| ADV-U8H-2 | Low | plan.md:1256's slack-value rule (SCHED_OTHER: 1 at both reads; FIFO or RR: 0 or 1, 0 only on a kernel with ed4fb6d7ef68; any other policy: 1) is neither applied by probe_presence, which checks presence only (r1.sh:642), nor printed in presence.txt | One clause in ADV-U8H-1's R1.md line: the advisor applies it at R1's report from the pushed P1 leg header. No code. If it does not fit, it is carried to R1's report, which applies it |
| SR-1 | Low (automated: Medium) | Section 3 | The re-read line, in U8-F1 |
| SR-2 | Low (automated: Medium) | Section 3 | Addressed by SR-1's line and the push prompt; no deny added |
| ADV-U8H-3 | Low | Text still waiting on U6b although it has merged: r1.sh:296 and :322 (F28 and the pin's hash file, both now settled in the merged code: section 1); R1.md:243's "may join the run list at U6b's merge, if the pump runs on the io threads". The pump is a tokio task, but its release clock is "one process-wide dedicated thread" (net/src/l2/delay_clock.rs:1), whose vCPU decides that question | Drop the two comment edits (comments only). The main session reads the clock thread's placement before Stop B's fill; until then the skip stands (it is the conservative side) |
| ADV-U8H-4 | Low (notes) | u8-report.md:63-64 says U6c's names were "read by test 7 on the merged code". Test 7's item 4 reads fixtures; the evidence on real output is section 1's run on U6c's 24 Nov log. The open list also lacks U7's alignments: the URL contract in config.mjs, and `node build.mjs` in rs.yaml | The main session corrects the report (notes, outside the size count) |
| NC-R3-1 | Low | As carried | Carried: the STATUS follow-up named in U8's report |

**RESULTS-PUSH: how the two self-tests' numbers reach this container. It is his, because no form passes all four of
TEST-FIRST rule 2's tests.** The repository is public (F8), so a pushed file cannot be taken back. plan.md:252 says
the laptop pushes "redacted summaries".
- **(a) Push the two raw logs** besides the common list: probe/room-02-P1.jsonl and ds-selftest/room-05.jsonl. That is
  text only, about 6-9 lines, and fits U8's 11 remaining lines.
  - The cost: raw server logs go to a public branch for good, which leaves "summaries". They carry the VMs' kept
    addresses and run headers, and no client string (F5 (4)).
  - Checked: redact-check, run read-only on U6c's real P1 log and LG log, reported "2 item(s) checked, 0 finding(s)".
  - Sizes: U6c's P1 log was 27 MB. A 48-client room log of 7 minutes is not sized.
- **(b) A printed extract on RS:** probe_presence prints P1's `l2_summary` fields and the four slack and policy reads,
  and the ds block prints its `ws_split` event's fields, into the summaries.
  - This stays inside "summaries".
  - The cost: about 20-30 lines with tests and SR-1's line, so U8 passes his 6,200 stop by about 10-20. It needs a new
    stop, about 6,230.
- **(c) Neither now.** After R1, a laptop sitting (his prompts, about 15 minutes) takes the numbers from the raw
  tarball in form (a) or (b). This delays R1's report.
- **My lean: (b).** A public push cannot be undone, and his plan's word is "summaries".

**U8-F1, in every form:**
- **Order:** ADV-U8H-1's lines and their test first, then SR-1's line, then ADV-U8H-2's clause. Under (b) the extract
  prints the slack reads, which closes ADV-U8H-2.
- **Files:** PROMPT-R1.md, R1.md and their tests. Under (a), rules.md:50 ("it reads npt-judge's summaries") gains
  "and pushes the named raw logs unread": one more line.
- **The stop:** under his word of 23 Nov, a Low that would pass U8's stop is carried, not built. If SR-1 is carried,
  SR-2 goes with it, and both stay security findings left open, named in U8's report.
- **Round:** security-auditor's short read (section 3) is U8-F1's one round; I read its pushed head.

## What I could not check
- No VM and no Windows: the pushed set's size and redaction on the day, and the push prompt as Claude Code shows it on
  the laptop.
- I did not run browser.test.mjs; its 9 tests stand on M5's log.
- My reading of the merged consumers is a code read, not a run of the binaries on U8's files. U6c's 24 Nov run is the
  one real run, and it used the same key sets.

## Decisions needed
- **RESULTS-PUSH.**
  - The question: how do R1's two self-test results (ImpairSocket's on the probe, DelayStream's on 48 connections)
    reach this session for R1's report?
  - Why it matters: today's run-day text pushes only the judge's summaries, which hold neither, and every push to the
    results branch is public and permanent.
  - Options:
    - (a) push the two raw server logs (fits U8's size stop; raw logs public for good);
    - (b) print their numbers into the summaries on the server (stays "summaries" as his plan says; about 10-20 lines
      past U8's 6,200 stop, so a new stop of about 6,230);
    - (c) neither now, and a laptop sitting of about 15 minutes after R1.
  - Lean: (b).

## For Mustafa
1. Answer RESULTS-PUSH before Stop B (Tue 1 Dec). U8-F1 is built on his answer, and Stop B is where he approves
   PROMPT-R1.md.
2. For the day's message, each his to reverse:
   - the merge of U8 into the lane passed its head read (no Critical or High);
   - U8-F1 follows before Stop B: R1's pushed results named, and one check just before the results push;
   - the automated review's SR-1 and SR-2 are weighed Low, for the reasons in section 3;
   - security findings that stay open, named in U8's report: SEC-SB-9's rest, and SR-1 and SR-2 only if they are
     carried.

DONE

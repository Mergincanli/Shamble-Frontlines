# U8-F1: the head read's fix list (brief, drafted 2026-11-26 12:4x UTC by the main session; checked 15:00 UTC: APPROVE WITH CONDITIONS C1-C7, advisor-u8-f1-brief.md, applied word for word)
Sources: advisor-u8-head.md (APPROVE WITH FINDINGS): section 3 (SR-1, SR-2), section 4 (ADV-U8H-1 to -4) and "U8-F1, in
every form". His answer, word for word in history.md:8068-8088: "RESULTS-PUSH: (b). Only numbers leave the server and
the push stays summaries. U8's new stop is 6,230 lines." His S14-PUSH answer, word for word in history.md:8090-8106:
"(a). R1's results push carries the s14 folder whole, through the same redaction check." with two conditions (item
1e), and "U8-F1's builder may start in the 14:50 UTC window on this answer."

## 0. Base and size
- Base `<base>`: the branch head at launch (U8 merged at 05c63a8: 43 files under spikes/netcode-proof/ops/, 6,189 added
  lines). A new builder in a fresh worktree (isolation: worktree), not U8's builder ac59cce3e5d002441: its worktree
  (03d9ff3) holds those 43 files untracked, so its fast-forward would be refused (ffcheck 12:4x: 44 ADDED present).
- The stop: 6,230, so U8-F1 adds at most 41 lines, tests included. The 41 are net: for the files U8-F1 touches, the
  added column less the removed column of `git diff --numstat <base>`, the way U8's 6,189 and the M5 fix's +19 were
  counted. Item 1e's pin test is part of his S14-PUSH condition and is never carried. Before an item would pass 41,
  carry the rest, last first: item 3, then item 2's test 5 assert. Item 2's PROMPT-R1.md line is never carried. If
  item 1, with the pin test, would pass 41 alone, stop before it does and report what grew, why, and the cheaper
  option (his stop). Item 1's other parts are never carried. Say what was carried and why.

## 1. What is built (in this order; each with its test passing before the next)
1. **ADV-U8H-1 under (b): the two self-tests' numbers printed into the summaries on RS.**
   a. probe_presence (ops/runs/r1.sh:606-677) also prints, for P1 only, each field of its `l2_summary` event (ImpairSocket's
      self-test: plan.md:1256's "printed E, its max and the counters") and the four P1_READS values (r1.sh:615), one
      `P1 <name> <value>` line each, after the verdict lines. Every value printed is an integer; no other log text. The two
      policy reads print as Linux's policy number and the priority: SCHED_OTHER 0, SCHED_FIFO 1, SCHED_RR 2,
      SCHED_BATCH 3, SCHED_IDLE 5, SCHED_DEADLINE 6, as `P1 <name> <policy number> <priority>`; an unread or unknown
      policy prints `-1 -1`. A field whose value is not an integer is not printed; one line `P1 non_integer_fields
      <count>` (in 1b, `ws_split non_integer_fields <count>`) gives how many. No new line holds the word INVALID (the
      probe block greps presence.txt for it).
      Its output already goes to probe/presence.txt (r1.sh:592).
   b. ds_selftest (r1.sh:700-712) writes its `ws_split` event's fields (condition 10's read) into
      ds-selftest/ws_split.txt, one `<name> <value>` line each, numbers only; a missing event prints one line saying so.
   c. The pushed list, word for word the same in PROMPT-R1.md section 6 step 2 (ops/operator/PROMPT-R1.md:121-123) and
      R1.md section 7 (ops/runs/R1.md:288-294), in place of "npt-judge's summaries and the redacted records":
      `collect/`, `probe/presence.txt`, `ds-selftest/ws_split.txt`, `bringup/facts.txt`, `s14/` whole (R1.md:190-193's
      copied folder: his S14-PUSH (a)) and `s14-redact.txt` (item 1e).
   d. Tests: one asserts the two lists equal (test 5's file, which holds PROMPT-R1.md's checks); r1sh.test.mjs reads
      presence.txt's new lines from a fixture P1 log (an `l2_summary` and the four reads) and ws_split.txt's from a
      fixture `ws_split` event. One plant each (drop a field from the print, or one name from one list): the test
      fails by name; restored by sha256.
   e. **S14-PUSH (a)'s two conditions**, his words: "The redaction check runs on the s14 folder itself before the
      push, and its result is in the run's record. If a later change puts anything into that folder that names the
      server (a host name, a routed address, a hardware address), it stops and comes to me first."
      - Always built: PROMPT-R1.md section 6 step 2, before the push: the redaction check is the pre-push hook's own:
        the fixed copy `~/npt-operator/redact-check.mjs` with `--vms ~/npt-operator/vms.txt`, run on `<results>\r1\s14`,
        its whole output saved as `<results>\r1\s14-redact.txt` (in the pushed set; its last line in step 4's three-line
        summary). A finding, or any exit but 0, means no push: stop and tell him. And one R1.md sentence (section 7):
        the s14 folder holds only the kernel's release (uname -r), the net.ipv4 settings, the load, the TCP counters,
        the message times and the two documentation addresses; a change that would put anything naming the server
        into it (a host name, a routed address, a hardware address) stops and goes to Mustafa first. The R1.md sentence
        and the pin test name every writer into the folder: npt-s14's kernel.txt and server-, client- and
        bursts-run<N>.json; r1.sh's s14 block: summary.txt, INVALID-s14.txt, rerun/ (npt-s14's same files and its
        summary), rerun-R2.txt and browser-stamps.txt. Any other file, writer, key or address fails the pin test by
        name, with the message that such a change goes to Mustafa first.
      - A scratch run, not counted: redact-check on a fixture s14 folder under .gate/ (the files npt-s14 and r1.sh's
        s14 block write, with made-up numbers): 0 findings expected, its output in your notes. A finding: stop and
        report; no change to redact-check.
      - The pin test, never carried (section 0): a test in
        ops/tests/s14.test.mjs pins what the folder can hold: every file npt-s14 and r1.sh's s14 block write into it,
        kernel.txt's sources (uname -r, the listed net.ipv4 settings, `tc qdisc show`, /proc/loadavg), probe.py's JSON
        keys, and its only addresses (198.51.100.1 and .2). Any other writer, key or address fails it by name, with a
        message that such a change goes to Mustafa first (his S14-PUSH condition). One plant (an added `uname -n`
        line): fails by name; restored by sha256.
   Item 1's other parts are the Medium and his conditions, and are never carried. If they alone would pass 41, stop
   before they do and report what grew, why, and the cheaper option (his stop).
2. **SR-1, with SR-2:** one line in PROMPT-R1.md section 6 step 2, just before the push, in PowerShell:
   `git -C <results> rev-parse --git-path hooks`, `<results>` being the folder the push uses. It must print the clone's
   own hooks folder, the one holding pre-push; anything else: stop and tell him. Not `git config --get core.hooksPath`:
   settings.json:55's deny matches it in PowerShell (advisor-u8-head.md section 3). The four-way proof is a scratch run:
   a throwaway repository under your worktree's .gate/ (never this repository), with each of (i) to (iv) and the
   printed path recorded in u8-f1-build.md. It is not counted in the 41 and not committed. The committed part is the
   PROMPT-R1.md line, plus R1.md's Stop B start-check line extended with: 'and, in PowerShell, `git -C <results>
   rev-parse --git-path hooks` prints the clone's own hooks folder, prompting, not denied' (the laptop's git is not
   this container's). A hooks path set (i) by `git config core.hookspath <x>` in lower case, (ii) with `--worktree`
   (extensions.worktreeConfig on), (iii) by `--file .git/config`, and (iv) through GIT_CONFIG_COUNT, GIT_CONFIG_KEY_0
   and GIT_CONFIG_VALUE_0 in the child's environment must each change the printed path; if any one does not, stop and
   report. The scratch run starts git itself (execFileSync with `cwd` and `env`); item 2's one committed check is an
   assert in test 5's file that the line is there (the part section 0 lets be carried).
   The `-C` in the line is text for his laptop, never a command run here.
3. **ADV-U8H-2:** one clause in R1.md: the advisor applies plan.md:1256's slack-value rule at R1's report, from
   presence.txt's printed reads (item 1a prints them).

Not in U8-F1: ADV-U8H-3 (the two stale comments stay; the main session reads the delay clock thread's vCPU before Stop
B's fill); ADV-U8H-4 (the report's notes, the main session's); NC-R3-1 (carried: U8's report); TE-5 (Stop B).

## 2. How it is tested (in the worktree's spikes/netcode-proof; no cargo: U8-F1 touches no Rust)
- The nine ops test files that need no browser package (92 pass at `<base>`; `node --test` names them; not
  `node --test ops/`, which fails on Node 22.23), plus the new tests: all pass, the count recorded.
- `bash ops/tests/wrappers.test.sh`: "315 ok, 0 failed". `node ops/operator/redact-check.mjs .`: 0 findings.
- `node check.mjs --only refs,self,redact`: PASS (refs moves only if a file is added).
- browser.test.mjs is not run (it needs ops/browser/node_modules); U8-F1 touches no file under ops/browser/.

## 3. Rules (word for word)
Write files with the Write and Edit tools only (no sed, no shell redirect, no python writes; never `cargo fmt` writing
files: apply rustfmt's diff by Edit); an agent with no Edit tool rewrites its notes file whole with Write. git and gh
always start their own command, with no other program in front of them. Never switch the main folder; never `cd` into
another worktree; no `git -C` and no `git -c`; no shell loops, not even read-only ones; run each long call in the
background with a long timeout and read its log when told it ended. Notes after each item in your worktree's
.checkpoints/netcode-proof-test/u8-f1-build.md. No commit: the main session takes the snapshot.

## 4. At the end
The items, tests, plants, the net lines against 41 (section 0), anything carried, and what is open, in a short summary.

## 5. Its round
security-auditor at xhigh, one short read of the new PROMPT-R1.md and R1.md lines (the pushed set's exposure and
redaction; the re-read line): U8-F1's one round. security-auditor's one read also covers items 1a and 1b's print
code and item 1e's pin test (what leaves the server), beside the new PROMPT-R1.md and R1.md lines. The advisor reads its
pushed head. He approves both files by hand at
Stop B.

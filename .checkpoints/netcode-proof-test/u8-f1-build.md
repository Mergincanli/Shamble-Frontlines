# U8-F1's build notes (the builder, 2026-11-26 from about 15:10 UTC; worktree agent-a7fe1b9f436f36bb1)

## Start (all six checks as told)
- `git rev-parse --show-toplevel`: /home/user/Shamble-Frontlines/.claude/worktrees/agent-a7fe1b9f436f36bb1 (under
  .claude/worktrees/).
- `git rev-parse HEAD`: 2483e9a741c4e552323c36523831a2f35de4002e. `git status --short`: empty.
- `git switch --detach 77ffc95101e186f75c57a1e8770b3ab7a9442820`: "HEAD is now at 77ffc95 checkpoint(netcode-proof-test):
  autosave (terminated) [skip ci]". Then HEAD 77ffc95101e186f75c57a1e8770b3ab7a9442820; `git status --short` empty.
- launch-0850.md's last section read before the first build command.

## Addition 1 (before the pin test): can any writer into the s14 folder print a host name, a routed or hardware address?
Read: compare.py's `summary` (compare.py:266-272, with kernel_numbers and standing :94-134), r1.sh's `stamp` (:56),
the s14 block (:714-740), block_start (:100-110), browsers/browsers_down (:766-786), npt-s14 whole, probe.py whole.
- compare.py summary prints only numbers from the run files, the standing's fixed words, and `tcp_info_client` (the six
  integer fields probe.py writes). No host, address or MAC source.
- stamp is `date -u +%Y-%m-%dT%H:%M:%SZ`: a UTC time only.
- npt-s14 writes only $OUT/kernel.txt (uname -r, the message count, the run count, a UTC time, ten
  /proc/sys/net/ipv4 values read in the client namespace, `tc qdisc show` of the two veth ends, /proc/loadavg) and
  server-, client- and bursts-run<N>.json (probe.py's JSON). probe.py's keys hold numbers, the cc names, cc_errors
  (OSError texts of setsockopt) and the /proc/net/snmp and netstat counters; its only addresses are 198.51.100.1/.2.
- r1.sh's s14 block: summary.txt (compare.py), INVALID-s14.txt (its copy), rerun/ (the same), rerun-R2.txt (fixed text),
  browser-stamps.txt (`<room> up|down <stamp>` lines, rooms from the fixed list).
- **One writer the brief's list (C6) does not name:** block_start (r1.sh:105-107) appends `attempt <n> <stamp>` to
  `$R/s14/attempts` before the s14 block runs (as for every block). Numbers and a UTC time only, so no stop; but the
  R1.md sentence and the pin test name it too, since C6 asks for every writer.
- Verdict: no writer can print a host name, a routed address or a hardware address. Build goes on.

## Plan (net-line budget 41: added less removed over the touched files, `git diff --numstat <base>`)
1a r1.sh probe_presence: POL map + pol() (policy as number and priority, else `-1 -1`); in the leg loop for P1: the
   l2_summary fields and the two slack reads, integers only and names [a-z0-9_]+ only, then the two policies, then
   `P1 non_integer_fields <n>`; printed after the reports. About +6 net.
1b r1.sh: logq gains `-` (the event's fields as one JSON object); ds_selftest pipes it into a short python that writes
   ds-selftest/ws_split.txt (integer fields, `ws_split non_integer_fields <n>`, or one line when there is no event).
   About +5 net.
1c PROMPT-R1.md section 6 step 2 and R1.md section 7: the pushed list, word for word the same.
1d r1sh.test.mjs: P1 lines from the existing fixture (after verdicts and reports), a non-integer field and an unknown
   policy; ds_selftest on a fixture ws_split event and on none. r1plan.test.mjs (test 5): the two lists equal.
1e PROMPT-R1.md: the hook's own redaction check on <results>\r1\s14, output to s14-redact.txt; R1.md: the s14
   sentence naming every writer; s14.test.mjs: the pin test.
2  PROMPT-R1.md: `git -C <results> rev-parse --git-path hooks`; R1.md Stop B step 6 extended; scratch proof under
   .gate/u8-f1/ (a clone, a linked worktree, a `results` folder inside it; forms (i)-(iv)); r1plan assert.
3  R1.md: the slack-value clause.
Counting after each item with `git diff --numstat 77ffc95`.

## Base run (15:1x UTC)
`node --test` on the 8 browser-free ops test files (operator/redact-check.test.mjs and tests/{firewall, r1plan,
operator, redact, s14, cloudinit, r1sh}.test.mjs): 92 pass, 0 fail. The brief says "nine"; 8 files give the 92.
"Test 5" (u8-brief.md:322-323, R1.md's checks) is r1plan.test.mjs, which also holds a PROMPT-R1.md check (:153).

## Real data read (read only, the main folder's .gate/u6c/read-probe/room-P1.jsonl, U6c's run of 24 Nov)
- P1's l2_summary (ph after): 86 fields, every value an int, every name [a-z0-9_]+, max 100,021,409,698.
- Header params: probe_start_sched_policy and l2_sched_policy "SCHED_OTHER 0", both slack reads 1.
- ws_split: npt-server main.rs:801-805, `event("ws_split", Phase::Measured, rows)` with rows (&str, u64) only, into
  the room's log (room-05.jsonl).

## 1a DONE (r1.sh probe_presence; r1sh.test.mjs item 4's test)
- r1.sh: `import ... re`; POL (Linux's numbers) and pol() (`<number> <priority>`, else `-1 -1`); in the leg loop,
  before the verdict chain, for P1: l2_summary's fields and the two slack reads, printed only when the name is
  [a-z0-9_]+ and `type(v) is int` (no bool, no float, no text), then the two policies, then `P1 non_integer_fields
  <n>`; printed after the reports (`for r in reports + p1`). No new line can hold INVALID (lower-case names, digits).
- Test: the verdict filter `run()` changed in place to `/^\w+: /` (it returned the P1 lines otherwise); the clean
  fixture's 10 P1 lines asserted as `all().slice(12)` (after 6 verdicts and 6 reports); an INVALID P1 (drops 7 to 9,
  policy SCHED_UNKNOWN, a text and a bool field) still prints: `P1 l2_sched_policy -1 -1`, `P1 non_integer_fields 2`.
- Plant 1a: `P1_READS[::2]` to `P1_READS[:1]` (drops l2_timer_slack_ns from the print): the test "item 4: ...
  ADV-U8H-1's P1 integers" failed by name (missing `P1 l2_timer_slack_ns 1`). Restored: r1.sh sha256 84cb57c8...f773
  before and after.

## 1b DONE (r1.sh logq and ds_selftest; a new r1sh.test.mjs test)
- logq: field `-` prints each such event's fields as one JSON object a line (the other fields unchanged).
- ds_selftest, after `keep "$d" 05`: `logq "$ROOMS/room-05$ATT.jsonl" ws_split - | python3 -I -c '...' >
  "$d/ws_split.txt"`: the first event's integer fields as `<name> <value>`, then `ws_split non_integer_fields <n>`;
  no event (or no log: logq swallows OSError) prints `no ws_split event in the room log`. python reads all stdin, so
  no SIGPIPE under pipefail.
- Test "ADV-U8H-1 (b): ds_selftest's ws_split.txt ...": a cut first line, then a ws_split event with two ints and a
  text: `ws_late_up_count 9`, `ws_late_down_p99_ns 800`, `ws_split non_integer_fields 1`; a log with no ws_split: the
  one line.
- Plant 1b: `[1:]` on the printed list (drops the first field): that test failed by name. Restored: r1.sh sha256
  6169122a...2831 before and after.
- No other test runs ds_selftest (grep: only r1sh.test.mjs:213's text match, which still holds).

## Count after 1a and 1b (`git diff --numstat 77ffc95`)
| File | Added | Removed | Net |
|---|---|---|---|
| ops/runs/r1.sh | 17 | 6 | +11 |
| ops/tests/r1sh.test.mjs | 17 | 2 | +15 |
| **Total** | 34 | 8 | **+26 of 41** |

## A trap met (fixed): redact-check reads Python's `[::2]` and `[1::2]` as IPv6 addresses
The first full run failed redact.test.mjs (r1.sh:646:63 and :648:60, "IPv6 address"). The pairs are now written out:
`[(P1_READS[0], s0), (P1_READS[2], s1)]` and `((P1_READS[1], pol0), (P1_READS[3], pol1))`. Plant 1a redone on that
form (drop `(P1_READS[2], s1)`): failed by name; restored, r1.sh sha256 a6f1dded...3a3a before and after (the tree's
r1.sh now).

## Section 2's checks on the tree as it stands (1a and 1b built)
- `node --test` on the 8 files: 93 pass, 0 fail (92 + the ws_split test).
- `bash ops/tests/wrappers.test.sh`: "315 ok, 0 failed".
- `node ops/operator/redact-check.mjs <spike>`: 421 item(s) checked, 0 finding(s).
- `node check.mjs --only refs,self,redact`: PASS refs (1,994 tracked files; no file added), PASS self 78 of 78, PASS
  redact 11 of 11 with 421 items, 0 findings.
- 1b's missing-log path (u8-f1/missing-log.mjs, r1.sh's own functions, VM calls stubbed): exit 0, ws_split.txt reads
  "no ws_split event in the room log".

## Item 2's four-way proof (scratch, not counted, not committed): u8-f1/hooks-proof.mjs on .gate/u8-f1/hooks-proof-1
Guards: every git child had an explicit environment (HOME in scratch, GIT_CONFIG_NOSYSTEM=1, GIT_CONFIG_GLOBAL a
missing scratch file, GIT_CEILING_DIRECTORIES the scratch folder, only PATH inherited); each top level asserted before
any config write; each form from the baseline and undone back to it. Layout: a clone, a linked worktree on
`claude/npt-results`, and a `results` folder inside it (the laptop's `<results>`), the read run from that folder.
- baseline: .../hooks-proof-1/clone/.git/hooks (the clone's own hooks folder, asserted by realpath)
- (i) `git config core.hookspath <x>` (lower case): .../hooks-proof-1/elsewhere-i
- (ii) `git config --worktree core.hooksPath <x>`, extensions.worktreeConfig on: .../elsewhere-ii
- (iii) `git config --file .git/config core.hooksPath <x>` (from the clone root): .../elsewhere-iii
- (iv) GIT_CONFIG_COUNT=1, GIT_CONFIG_KEY_0=core.hooksPath, GIT_CONFIG_VALUE_0=<x> in the child's env: .../elsewhere-iv
- git 2.43.0: all four change the printed path. This repository after it: `git config --get core.hookspath` and
  `git config --get extensions.worktreeconfig` both unset (exit 1); `git status` shows only my two files and notes.

## Item 1e's scratch redaction run (not counted): .gate/u8-f1/s14-fixture/s14
The files npt-s14 and r1.sh's s14 block write, with made-up numbers: kernel.txt (uname_r 6.8.0-49-generic; rerun's
6.11.0-29-generic), the ten net.ipv4 lines, two real-form `tc qdisc show` lines, six loadavg lines; server-, client-
and bursts-run1.json with 13-digit stamps, 19-digit integers, 10-digit integers inside IPv4's decimal range
(3232235777, 134744072, 4294967295, and burst delays of 1.0-1.1 s), a cc_errors text; summary.txt (compare.py's own
`summary` output for run1), INVALID-s14.txt, rerun/ (the same five), rerun-R2.txt, browser-stamps.txt, attempts.
- `node redact-check.mjs --vms <fixture vms.txt> <fixture>/s14`: "redact-check: 14 item(s) checked, 0 finding(s)".
- UTF-16 (checkBuffers in memory, no file): a UTF-16LE copy of that output line reads "binary file: cannot be
  checked", 1 finding; UTF-8 with or without a BOM: 0. So s14-redact.txt saved by Windows PowerShell 5.1's `>` would
  make the pre-push hook refuse the push (fails closed, never a leak): the PROMPT line must name UTF-8 (draft A does).

## STOP before 1c (section 0, his stop): item 1 with the pin test passes 41
1c and 1e are not put in the tree: a snapshot would push s14/ whole without his two conditions. 1a and 1b stay:
numbers only into RS's summaries; nothing that is pushed changes. Drafts, priced outside the tree:
u8-f1/drafts.md (A: PROMPT step 2; B: R1.md's Stop B line; C: R1.md section 7's bullet; D: the r1plan test),
u8-f1/list-draft.test.mjs (A-C spliced in memory: green; one name dropped from one list, PLANT=1: fails by name),
u8-f1/pin-draft.test.mjs (the pin test on the real files: green; an added `uname -n` line in npt-s14's kernel.txt
block: fails "kernel.txt's sources: ... goes to Mustafa first" naming `uname -n`; npt-s14 sha256 0c62ea52...9e1d
before and after).

**Pin draft widened (17:xx UTC, the advisor's check):** the first draft (33 lines) pinned npt-s14's file names but not
the lines writing the three JSON files, nor compare.py's summary print (it fills summary.txt on RS): a `hostname >>
"$OUT/client-run$run.json"` line, or `{os.uname().nodename}` in that print, passed it. Now pinned: npt-s14's eight
writing lines (with the server and client producers) exactly; compare.py's one print in `summary`, its five imports,
and every value interpolated in `summary` and `standing`. Green on the real files; plants: the hostname line fails
"npt-s14's lines writing them" naming it; the nodename fails "compare.py summary's values" naming
`os.uname().nodename`; the `uname -n` plant still fails "kernel.txt's sources". npt-s14 0c62ea52...9e1d and compare.py
38a348c1...18b1, before and after each. 42 lines now.
After it: `bash ops/tests/wrappers.test.sh` re-run on the final tree: "315 ok, 0 failed".

| Part | File | Added | Removed | Net | State |
|---|---|---|---|---|---|
| 1a and 1b | ops/runs/r1.sh | 17 | 6 | +11 (1a +6, 1b +5) | built |
| 1a and 1b tests | ops/tests/r1sh.test.mjs | 17 | 2 | +15 (1a +5, 1b +10) | built |
| 1c, 1e, 2's line | ops/operator/PROMPT-R1.md (A) | 9 | 3 | +6 (2's sentence about 1-2) | draft |
| 2's Stop B line; 1c, 1e, 3 | ops/runs/R1.md (B, C) | 13 | 2 | +11 (B +1; C +10, 3's clause 1) | draft |
| 1d list test, 2's assert | ops/tests/r1plan.test.mjs (D) | 7 | 0 | +7 (2's assert 1) | draft |
| 1e pin test | ops/tests/s14.test.mjs | 44 | 1 | +43 (42 and its blank; the import line) | draft |
| **Whole** | | 107 | 14 | **+93** | 6,189 + 93 = 6,282 |
| Item 1 alone | | | | about +89 | over twice the 41 |

What grew, against advisor-u8-f1-brief.md:40-44 (1a 4, 1b 6, 1c 2, 1d 10, 1e text 6, pin 12; about 40):
- The pin test, 43 against 12: C6's "every writer, by name" as an exact allow-list (a bare `hostname` line must fail
  as surely as `$(uname -n)`): npt-s14's four files, their eight writing lines and its other $OUT lines; kernel.txt's
  six sources, seven variables and ten net.ipv4 names; r1.sh's ten write lines into the folder; the stamp writers;
  compare.py summary's print, imports and values; probe.py's 30 keys; four files' addresses.
- 1b, 15 against 6: its test stubs ds_selftest's five VM calls (10 lines).
- 1a, 11 against 4: C4's policy numbers (2 lines) and the INVALID-P1 case.
- R1.md's sentence, 9 against about 4: as the brief words it, it is not true (it leaves out netem's settings, the cc
  names and error texts, the attempt and browser-leg times, the standings), and C6's writer list plus `attempts`.
Options, priced, not chosen (his):
- a new stop over the whole: 6,282 (measured: built +26, drafts +67);
- the same with item 3 and item 2's assert carried: 6,280;
- the same with safe folds (the ws_split test inside item 5's test: about 5 fewer, estimate): about 6,277;
- a thinner pin test (files, keys, addresses, a list of host-naming calls: about 13 lines, an estimate, not built):
  about 6,252; it departs from C6's "by name" exact allow-list, so it is not the builder's to pick.
Not verified: the hooks proof ran on Linux git 2.43.0, not Git for Windows (Stop B's laptop read is the check); the
UTF-16 result is from an in-memory buffer, not a real PowerShell save.

## Open (no lines)
- `attempts` (r1.sh block_start :105-107) writes into the s14 folder: C6's list leaves it out. Drafts B/C and the pin
  test name it. It holds `attempt <n> <UTC time>` only.
- The brief's R1.md sentence ("holds only ...") is not true as worded; draft C says what the folder holds.
- Low: logq does not guard a JSON line that is not an object (`r.get` on it raises). At ds_selftest's end that would
  turn a finished block into stopped. npt-log writes objects only; noted, not fixed.
- On a block's re-run (-aN, only after an INVALID), ws_split.txt and presence.txt hold the last attempt only; the
  earlier attempt's numbers stay in the raw tarball.
- DONE (stopped at 1b by section 0's rule; the main session takes the snapshot).

# Resume (17:4x UTC): his U8F1-SIZE (a), stop 6,282, so 93 net lines (u8-f1-brief.md:12-18 as the main session edited it)
Checks, each its own call: top level .../.claude/worktrees/agent-a7fe1b9f436f36bb1; HEAD 77ffc95101e1...; status: only
r1.sh, r1sh.test.mjs and the notes. Building from drafts.md, item by item.

## 1c and 1d DONE
- PROMPT-R1.md section 6 step 2: "Put only `collect/`, ... and `s14-redact.txt` into `<results>\r1\` (`<results>` is
  `.gate\npt-results\results`)" in place of "npt-judge's summaries and the redacted records". R1.md section 7: a new
  bullet carrying the same list (its 1e and 3 sentences follow).
- r1plan.test.mjs (test 5's file): "ADV-U8H-1 (b), S14-PUSH (a): one pushed list, word for word, ...": R1.md's list
  pinned to the literal, PROMPT-R1.md's equal to it. r1plan + operator: 21 pass (3.6's deny check included).
- Plant 1d: `bringup/facts.txt` dropped from PROMPT-R1.md's list: the test failed by name ("the same list in both
  files"). Restored: PROMPT-R1.md sha256 7b0e4e9a...ae50 before and after.

## 1e DONE
- PROMPT-R1.md step 2: C5's words (the hook's own check: the fixed copy with `--vms`, run on `<results>\r1\s14`, its
  whole output saved as `<results>\r1\s14-redact.txt`, in UTF-8, its last line in step 4's summary; a finding or any
  exit but 0: no push, stop and tell him).
- R1.md section 7's bullet: what the s14 folder holds (true: netem's settings, the cc names and errors, the attempt
  and browser-leg times and the standings added to the brief's list), every writer (C6's list and `attempts`), and the
  stop-and-go-to-Mustafa sentence naming the pin test.
- s14.test.mjs: the pin test as pin-draft.test.mjs had it (42 lines), `text` added to the helpers import; one change:
  the hardware-address pattern is `(?:[0-9a-f]{2}:){5}[0-9a-f]{2}` (no `::`, which redact-check reads as IPv6).
- Tests: s14, r1plan, operator, redact: 34 pass (redact.test.mjs scans the new test file too: clean).
- Plants: an added `uname -n` line in npt-s14's kernel.txt block: fails "kernel.txt's sources ... goes to Mustafa
  first" naming `uname -n`; a hardware address in probe.py: fails "probe.py's addresses" naming it. Restored: npt-s14
  0c62ea52...9e1d, probe.py 65020716...0434, before and after. (The scratch draft's hostname and nodename plants
  stand for the same lines, unchanged in the tree.)
- Count after item 1: PROMPT-R1.md 7/2 (+5), R1.md 9/0 (+9), r1.sh 17/6 (+11), r1plan.test.mjs 6/0 (+6),
  r1sh.test.mjs 17/2 (+15), s14.test.mjs 44/1 (+43): **+89 of 93**.

## Item 2 DONE (SR-1 with SR-2)
- PROMPT-R1.md step 2, just before the push: "in PowerShell, `git -C <results> rev-parse --git-path hooks` must print
  the clone's own hooks folder, the one holding pre-push; anything else: stop and tell him." (`<results>` defined once
  in the same step: `.gate\npt-results\results`, inside the results worktree the push uses.)
- R1.md section 2 step 6 (Stop B's start-check line) extended, word for word C3's: "and, in PowerShell, `git -C
  <results> rev-parse --git-path hooks` prints the clone's own hooks folder, prompting, not denied".
- The assert in r1plan.test.mjs's list test ("SR-1: the hooks re-read just before the push"). operator.test.mjs 3.6
  also reads every backticked `git ...` in PROMPT-R1.md, the new line among them, against every deny: no deny matches.
- Plant: the line set back to `git -C <results> config --get core.hooksPath`: the test failed by name (the SR-1
  message). Restored: PROMPT-R1.md sha256 f981d0b6...13da before and after.
- The four-way proof: the scratch run above (hooks-proof.mjs): all of (i)-(iv) change the printed path.

## Item 3 DONE (ADV-U8H-2)
R1.md section 7's bullet ends: "At R1's report the advisor applies plan.md:1256's slack-value rule from presence.txt's
printed reads (ADV-U8H-2)." (No test: a clause, as the brief has it.)

# HAND-BACK (U8-F1 built whole, 2026-11-26 about 18:1x UTC; no commit: the main session takes the snapshot)
**Size against 93** (`git diff --numstat 77ffc95101e186f75c57a1e8770b3ab7a9442820`):

| File | Added | Removed | Net |
|---|---|---|---|
| spikes/netcode-proof/ops/operator/PROMPT-R1.md | 8 | 2 | +6 |
| spikes/netcode-proof/ops/runs/R1.md | 12 | 1 | +11 |
| spikes/netcode-proof/ops/runs/r1.sh | 17 | 6 | +11 |
| spikes/netcode-proof/ops/tests/r1plan.test.mjs | 7 | 0 | +7 |
| spikes/netcode-proof/ops/tests/r1sh.test.mjs | 17 | 2 | +15 |
| spikes/netcode-proof/ops/tests/s14.test.mjs | 44 | 1 | +43 |
| **Total** | **105** | **12** | **+93 of 93** (U8 at 6,282, his stop) |

Lines removed only where an item replaces text: PROMPT-R1.md's old step 2 lines (2), R1.md's Stop B line's end (1),
r1.sh's changed lines (6: import, reports' start, the print loop, logq's comment and two lines), r1sh.test.mjs's
test title and verdict filter (2), s14.test.mjs's helpers import (1). Nothing deleted to make room.
**Carried: nothing** (item 3 and item 2's assert both fit).

**Tests on the final tree**
- `node --test` on the 8 browser-free ops test files: 95 pass, 0 fail (the base's 92 + the ws_split test, the list
  test, the pin test).
- `node --test '<spike>/ops/**/*.test.mjs'` (the glob form): 104 tests, 103 pass, 1 fail: browser.test.mjs's "the
  scripts: plain Node on standalone webdriverio ..." (ERR_MODULE_NOT_FOUND 'webdriverio': this worktree has no
  ops/browser/node_modules; the brief's section 2 leaves browser.test.mjs out; U8-F1 touches nothing under ops/browser/).
- `bash ops/tests/wrappers.test.sh`: "315 ok, 0 failed".
- `node ops/operator/redact-check.mjs <spike>`: "421 item(s) checked, 0 finding(s)".
- `node check.mjs --only refs,self,redact`: PASS refs, PASS self (78 of 78), PASS redact (11 of 11; 421 items, 0
  findings); "check: all 3 steps pass". No file added.
- Scratch, not counted: redact-check on the fixture s14 folder: 14 items, 0 findings; UTF-16 output reads as binary
  (the PROMPT line names UTF-8); the four-way hooks proof: all four change the printed path.

**Plants** (each failed its test by name; each restored, sha256 equal before and after)
| Item | Plant | Failed | Restored |
|---|---|---|---|
| 1a | one slack read dropped from P1's print | item 4's test ("after the verdicts and reports ...") | r1.sh a6f1dded...3a3a |
| 1b | the first ws_split field dropped | the ws_split test | r1.sh 6169122a...2831 (then form) |
| 1d | `bringup/facts.txt` dropped from PROMPT-R1.md's list | the list test ("the same list in both files") | PROMPT-R1.md 7b0e4e9a...ae50 |
| 1e | an added `uname -n` line in npt-s14's kernel.txt block | the pin test ("kernel.txt's sources ... goes to Mustafa first") | npt-s14 0c62ea52...9e1d |
| 1e | a hardware address in probe.py | the pin test ("probe.py's addresses ...") | probe.py 65020716...0434 |
| 2 | the re-read line set back to `config --get core.hooksPath` | the list test (the SR-1 message) | PROMPT-R1.md f981d0b6...13da |

**For the round (section 5: security-auditor at xhigh; then the advisor's head read)**: the new PROMPT-R1.md step 2 and
R1.md lines (section 2 step 6; section 7's new bullet), r1.sh's two prints (probe_presence's P1 lines; ds_selftest's
ws_split.txt, through logq's `-`), and s14.test.mjs's pin test.

**Open**
- For Mustafa at Stop B (he approves both files by hand): R1.md's s14 sentence goes beyond the brief's words in two
  ways, each to make it true: it names `attempts` (r1.sh's block start writes `attempt <n> <UTC time>` into the
  folder; C6's list leaves it out), and it adds what the folder really holds beside the brief's list (netem's settings,
  the cc names and errors, the attempt and browser-leg times, the run standings).
- Low: logq does not guard a JSON line that is not an object; at ds_selftest's end such a line would make a finished
  block read stopped. npt-log writes objects only.
- On a block's re-run, ws_split.txt and presence.txt hold the last attempt only (the raw tarball keeps all).
- Not verified here: the re-read line on Git for Windows (Stop B's laptop read is that check); a real PowerShell save
  in UTF-16 (the in-memory buffer stands for it); browser.test.mjs (no packages in this worktree).
- The scratch files stay: .checkpoints/netcode-proof-test/u8-f1/ (drafts.md, list-draft.test.mjs, pin-draft.test.mjs,
  hooks-proof.mjs, missing-log.mjs); the worktree's .gate/u8-f1/ (hooks-proof-1/, s14-fixture/) is not snapshotted.
DONE

# FIX PASS (security round)
On the advisor's ruling of 00:59 UTC 27 Nov (advisor-u8-f1-sec.md section 3, read at its main-folder path, read only;
the security round's notes security-u8-f1.md sections 2 and 6 read the same way). Start checks, each its own call:
top level .../.claude/worktrees/agent-a7fe1b9f436f36bb1; HEAD 77ffc95101e1...; status: U8-F1's six files and the
notes only. Edits only in s14.test.mjs, PROMPT-R1.md, R1.md and r1plan.test.mjs; the plants elsewhere are temporary.

## Item 1 DONE (SEC-U8F1-2: one exact list for npt-s14's $OUT lines)
- Red before the merge: `hostname > $OUT/host.txt` after npt-s14's run loop: the pin test PASSED (the gap: an
  unquoted `$OUT/` matched neither "npt-s14's lines writing them" nor "npt-s14's other $OUT lines").
- Merged into one `only("npt-s14's $OUT lines", ...)`: filter `/\$\{?OUT\b|"\$PROBE" (?:server|client|bursts)/` over
  the trimmed lines; wanted: the eight lines plus `[ ! -e "$OUT" ] ...`, `install -d ... "$OUT"` and the final printf.
- After the merge, the same red FAILED by name: "npt-s14's $OUT lines: ... goes to Mustafa first", naming
  `hostname > $OUT/host.txt`. Restored by Edit: npt-s14 sha256 0c62ea52...9e1d before and after. Green after it.
- Net after item 1: s14.test.mjs 43/1 (+42); total **+92** (one line freed).

## Item 2 DONE (SEC-U8F1-1 (a): r1.sh's writer forms)
- "r1.sh's writes into the folder"'s filter is now
  `/^[^#]*(?:>|\btee |\bcp |\bmv |\bln |\binstall |\bpython3 |\bnode |s14_run ")/`. On today's tree: green with the
  wanted list unchanged; the new filter caught no other line (its only new match, the compare.py summary line, was
  already listed).
- Plant: a heredoc writer in s14_run (`python3 -I - "$1" << 'PY'`, a body line opening `$1/extra.txt` for writing,
  `PY`): failed by name ("r1.sh's writes into the folder: ...", naming the call line). Restored by Edit: r1.sh sha256
  a6f1dded...3a3a before and after.
- Net after item 2: unchanged, **+92** (a changed line).

## Item 3 DONE (SEC-U8F1-1 (b), with SEC-U8F1-2's dash-form MAC and stamps text)
- The address loop's pattern is now the ruling's whole expression (the IPv4 form, the MAC in colon and dash forms,
  hostname, nodename, gethost\w*, getfqdn, `\buname\b(?! -r\b)`, platform.node, getsockname, getpeername, getifaddrs,
  if_nameindex, ifconfig, /sys/class/net, /etc/host\w*, `(addr|link|neigh) show`), flags gi; the wanted list stays
  198.51.100.1 and .2; the label reads "<name>'s addresses, host and interface names".
- `stamps` (every non-comment line of r1.sh naming stamps.txt: :750, :784, :794 today) sits on the `block` line; the
  loop's object gains "r1.sh's stamps lines"; "the stamps' writers" reads `stamp()` plus `stamps`, its wanted list
  gaining r1.sh's `cp -p "$R/browsers/stamps.txt" ...` line.
- Today's tree: green (npt-s14's `uname_r` and `uname -r` excluded as the seat found). redact.test.mjs over the spike,
  with the new expression's text in it: pass, clean.
- Plants, each failed by name, each restored by Edit with sha256 equal:
  - `"role": f"server {socket.gethostname()}"` in probe.py: "probe.py's addresses, host and interface names" naming
    `gethostname`; probe.py 65020716...0434.
  - `PLANT = "aa-bb-cc-dd-ee-ff"` in probe.py: the same check, naming `aa-bb-cc-dd-ee-ff`; probe.py 65020716...0434.
  - `  date -u >> "$R/browsers/stamps.txt"` in browsers() (a third writer of another shape, no host name in it):
    "the stamps' writers" naming that line; r1.sh a6f1dded...3a3a.
- Net after item 3: unchanged, **+92** (changed lines only; three long test lines).

## Item 4 DONE (SEC-U8F1-2's rest)
- `go(what)` builds the shared message ("<what>: a change that puts anything more into the pushed s14 folder stops
  and goes to Mustafa first"); `only` uses it, and so do the two asserts: "compare.py summary's one print" and "the
  listed net.ipv4 settings". The latter's deepEqual now holds `[count of "for s in ", the list]` against `[1, [...]]`.
- No IPv6 literal by name (the ruling's reason: Python slices; the repo's redact test catches a literal).
- Plant: a second one-line `for s in ../../kernel/hostname; do printf ... "$(ip netns exec "$NSC" cat
  "/proc/sys/net/ipv4/$s" ...)"; done` in npt-s14's kernel.txt block: failed by name ("the listed net.ipv4 settings:
  ... goes to Mustafa first", count 2 against 1). Restored by Edit: npt-s14 0c62ea52...9e1d before and after.
- Net after item 4: unchanged, **+92**.

## Item 5 DONE (SEC-U8F1-1 (c), S14-VMS: the s14 folder's own check with no VM list)
- PROMPT-R1.md step 2, the same six lines (:124-129) rewrapped: "Before the push, in PowerShell, the fixed copy
  `~/npt-operator/redact-check.mjs` runs on `<results>\r1\s14` with no VM list, so a VM's own address there is a
  finding; its whole output is saved as `<results>\r1\s14-redact.txt` in UTF-8 (UTF-16 reads as binary, a finding),
  its last line in step 4's summary. A finding, or any exit but 0, means no push: stop and tell him." Net 0.
- R1.md section 7's bullet: the parenthesis is now "(the redaction check's fixed copy, run on the s14 folder first
  with no VM list; then the pre-push hook's own over the whole push)". The bullet re-wraps from 10 lines to 11: +1.
- r1plan.test.mjs:211 on the same line: `[the re-read line present, a --vms before "runs? on `<results>\r1\s14`",
  "runs? on `<results>\r1\s14` with no VM list" present]` against `[true, false, true]`. The ruling says "run on";
  the prompt's sentence says "runs on" (the brief's own words for it), so the test reads both.
- Tests: r1plan + operator (3.6 included) + redact: 22 pass.
- Plant: `with `--vms ~/npt-operator/vms.txt`` put back after the fixed copy's name: failed by name ("SR-1: ...;
  S14-VMS: the s14 folder's check names no --vms", [true, true, true]). Restored by Edit: PROMPT-R1.md
  a3c63b7f...120a before and after.
- Proof, scratch (.gate/u8-f1/s14-fixture/s14), three outputs:
  1. no VM list, the fixture as it was: "redact-check: 14 item(s) checked, 0 finding(s)".
  2. **A difference from the brief's recipe, needed for the proof:** the fixture's vms.txt held 192.0.2.10 and .20,
     which redact-check's built-in list allows anyway (192.0.2.0/24, redact-check.mjs:113). With them, a VM address in
     the folder reads 0 findings with or without a VM list. So for this proof the fixture's vms.txt stood in with
     private 10.20.30.40 and .50 (no real public address written anywhere), and `rs_address 10.20.30.40` was appended
     to the fixture's kernel.txt. With `--vms`: "redact-check: 14 item(s) checked, 0 finding(s)" (today's gap).
  3. No VM list: ".gate/u8-f1/s14-fixture/s14/kernel.txt:23:12: IPv4 address: 10.…(11 chars)" and "redact-check: 14
     item(s) checked, 1 finding(s)", exit 1.
  Restored by Edit: vms.txt a90fbc8c...4b0c and kernel.txt 02d28270...c402, before and after.
- Net after item 5: R1.md 13/1 (+12); total **+93** (the stop).

## Item 6 PART BUILT, PART CARRIED (SEC-U8F1-3 and -4, the Lows), at +93 with no line to spare
Carry order applied as the ruling words it (advisor-u8-f1-sec.md:63-66): first item 6's Stop B record, then its wording
"SEC-U8F1-3, then SEC-U8F1-4's first part". Each part measured by re-wrapping the step's six lines in memory (node,
print only), at the file's 120-column wrap, no code span broken.
- **Built: SEC-U8F1-4's first part, net 0.** The save is named: "its whole output is saved with
  `| Out-File -Encoding utf8 <results>\r1\s14-redact.txt` (not `>`, which writes UTF-16)". It fits the same six
  lines. The brief's longer reason ("Windows PowerShell's `>` writes UTF-16, which the hook reads as binary") and
  "in UTF-8" beside the command each made a seventh line. The UTF-8 is now named by the command's own
  `-Encoding utf8`.
- **Carried: SEC-U8F1-3**, "Then, in the same PowerShell, push to `claude/npt-results` after the pre-push hook passes
  (a prompt)". With it, every wording tried (four forms) takes 7 lines: +1, past 93.
- **Carried: SEC-U8F1-4's second part**, Stop B's start-check line (R1.md:86-88) ending "its printed path and
  `git --version` are recorded": :88 is already 120 columns, so the clause is a new line: +1, past 93.
- Both carried Lows are STATUS follow-ups (a Low triggers no push on its own).
- Tests: operator.test.mjs 3.6 (every backticked git command in PROMPT-R1.md against every deny): pass. The r1plan
  assert first failed on the re-wrap ("runs" ends one line, "on" starts the next): its two patterns now take `\s+`
  (same line). Redone, item 5's plant (`--vms ~/npt-operator/vms.txt` back before "runs on") still fails by name.
  Restored: PROMPT-R1.md a3e9d53f...fd35 before and after.
- Net after item 6: unchanged, **+93**.

# FIX PASS (security round) HAND-BACK
Base 77ffc95, no merge, no commit, no cargo, no fetch. Edited only s14.test.mjs, PROMPT-R1.md, R1.md and
r1plan.test.mjs; every red and plant in npt-s14, probe.py and r1.sh was restored by Edit with sha256 equal.

**Items, reds, plants and restore hashes**
| Item | Red or plant | Failed by name | Restored (sha256, before = after) |
|---|---|---|---|
| 1 SEC-U8F1-2, one exact $OUT list | `hostname > $OUT/host.txt` after the run loop: PASSED before the merge (the gap), FAILED after | "npt-s14's $OUT lines" | npt-s14 0c62ea52...9e1d |
| 2 SEC-U8F1-1 (a), writer forms | today: green, wanted list unchanged, no other line caught; plant `python3 -I - "$1" << 'PY'` heredoc in s14_run | "r1.sh's writes into the folder" | r1.sh a6f1dded...3a3a |
| 3 SEC-U8F1-1 (b), names and dash MAC | today: green; redact.test.mjs clean on the new text; plant `"role": f"server {socket.gethostname()}"` | "probe.py's addresses, host and interface names" (`gethostname`) | probe.py 65020716...0434 |
| 3 | plant `aa-bb-cc-dd-ee-ff` in probe.py | the same check (`aa-bb-cc-dd-ee-ff`) | probe.py 65020716...0434 |
| 3 | plant a third stamps.txt writer, `date -u >> "$R/browsers/stamps.txt"` | "the stamps' writers" | r1.sh a6f1dded...3a3a |
| 4 SEC-U8F1-2's rest | plant a second one-line `for s in ../../kernel/hostname; do ...; done` | "the listed net.ipv4 settings: ... goes to Mustafa first" (count 2, want 1) | npt-s14 0c62ea52...9e1d |
| 5 S14-VMS, no VM list | plant `--vms ~/npt-operator/vms.txt` back (done twice: after item 5, and on item 6's final wrap) | "SR-1 ...; S14-VMS: the s14 folder's check names no --vms" | PROMPT-R1.md a3c63b7f...120a, then a3e9d53f...fd35 |
| 6 SEC-U8F1-3, -4 | the Out-File save built (net 0); operator 3.6 green | (no plant asked) | carried: "in the same PowerShell" and the Stop B record (below) |

Item 5's proof (scratch, .gate/u8-f1/s14-fixture/s14): no VM list: 14 items, 0 findings; with
`rs_address 10.20.30.40` appended and that address in the fixture's vms.txt: with `--vms` 0 findings (the gap), with
no VM list 1 finding (kernel.txt:23:12, IPv4), exit 1; restored (vms.txt a90fbc8c...4b0c, kernel.txt 02d28270...c402);
re-run at the end with no VM list: 14 items, 0 findings. The fixture's vms.txt had held documentation addresses,
which redact-check allows built in, so the stand-in was private addresses (item 5's notes).

**End checks** (the final tree)
- `node --test` on the 8 browser-free ops test files: **95 pass, 0 fail** (95 before; the pass adds no test).
- `bash ops/tests/wrappers.test.sh`: "315 ok, 0 failed".
- `node ops/operator/redact-check.mjs <spike>`: 421 item(s) checked, 0 finding(s).
- `node check.mjs --only refs,self,redact`: PASS refs (1,994 tracked files), PASS self 78 of 78, PASS redact 11 of 11
  (421 items, 0 findings): "check: all 3 steps pass".
- The scratch fixture: as above.
- No plant left: `git status` shows only U8-F1's six files and the notes; npt-s14, probe.py, compare.py and ops/browser
  (its package.json and package-lock.json) unchanged against 77ffc95.
- No end check needed cargo or the lane lock.

**Size** (`git diff --numstat 77ffc95101e186f75c57a1e8770b3ab7a9442820`, all six of U8-F1's files)
| File | Added | Removed | Net |
|---|---|---|---|
| spikes/netcode-proof/ops/operator/PROMPT-R1.md | 8 | 2 | +6 |
| spikes/netcode-proof/ops/runs/R1.md | 13 | 1 | +12 |
| spikes/netcode-proof/ops/runs/r1.sh | 17 | 6 | +11 |
| spikes/netcode-proof/ops/tests/r1plan.test.mjs | 7 | 0 | +7 |
| spikes/netcode-proof/ops/tests/r1sh.test.mjs | 17 | 2 | +15 |
| spikes/netcode-proof/ops/tests/s14.test.mjs | 43 | 1 | +42 |
| **Total** | **105** | **12** | **+93** (at the stop; U8 at 6,282) |
The pass moved one line from s14.test.mjs (item 1's merge, -1) to R1.md (item 5's longer parenthesis, +1); items 2-4
and 6's built part are changed lines only (three test lines are now long: the stamps line, the names pattern, the
net.ipv4 assert).

**Carried, and why**
- SEC-U8F1-4's second part (Stop B's line ends "its printed path and `git --version` are recorded"): R1.md:88 is at
  120 columns, so it is a new line, past +93. Carried first, as the ruling orders.
- SEC-U8F1-3 ("Then, in the same PowerShell, push to ..."): with it, step 2 needs a seventh line in every form tried,
  past +93. Carried second.
- Both are Lows: STATUS follow-ups, each one line, at a later unit or a new stop of 6,284.
- Kept: SEC-U8F1-4's first part (the Out-File save), net 0 with the shorter reason "(not `>`, which writes UTF-16)".
- Not carried: item 4 (built, net 0); items 1, 2, 3 and 5 (built).

**Open (no lines)**
- Two wordings for Mustafa's Stop B read of PROMPT-R1.md: "runs on `<results>\r1\s14` with no VM list" departs
  from C5's "with `--vms`" (the ruling's S14-VMS); the save's reason is shorter than the brief's sentence.
- The earlier Lows stand: logq's unguarded non-object line; re-runs keep the last attempt's prints only.

Status: COMPLETE 2026-11-27T01:14:16Z

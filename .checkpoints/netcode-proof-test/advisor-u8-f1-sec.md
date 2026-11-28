# Advisor: U8-F1's security round, ruled (2026-11-27)

Plan: read advisor.md from main; security-u8-f1.md whole (SEC-U8F1-1 and fixes (a)-(c), the three Lows, S14-VMS);
u8-f1-build.md; his S14-PUSH (a) at history.md:8090-8106; spot-check the cited lines in U8-F1's worktree. Then rule
1 each fix, 2 the fix pass's size, 3 the brief items, 4 the Lows; then Decisions needed, For Mustafa and the status
line. Written block by block.

Read:
- main's advisor.md (335 lines, last changed c9bd268 on 12 Nov);
- security-u8-f1.md (197 lines, DONE);
- u8-f1-build.md (HAND-BACK: +93 of 93, U8 at 6,282);
- history.md:8090-8106;
- in U8-F1's worktree (agent-a7fe1b9f436f36bb1, base 77ffc95, uncommitted): ops/tests/s14.test.mjs:196-240,
  operator/PROMPT-R1.md:120-130, runs/R1.md:85-89 and :292-302, tests/r1plan.test.mjs:208-211, and
  redact-check.mjs's option handling (it runs with no VM list: main() takes `--vm`/`--vms` as optional).

## 1. Each fix
**SEC-U8F1-1 stands as a Medium. All three of its fixes are taken, so nothing is left open.**
- **(a) As written.** s14.test.mjs:224's filter becomes
  `/^[^#]*(?:>|\btee |\bcp |\bmv |\bln |\binstall |\bpython3 |\bnode |s14_run ")/`. A heredoc's call line, an
  install, ln or mv, and a `compare.py judge` call then each fail by name.
  - The one python3 line in the block today, compare.py's summary, is already on the list.
  - If any other line the new filter catches is on today's tree, the builder stops and names it, rather than adding
    it to the list unread.
- **(b) Taken, in one changed form:** inside the address loop, not as a line paid for by removing item 3.
  - The loop at :236-238 matches its texts with one regular expression, which gains the seat's host and interface
    forms (C-F3 below). Each hit then fails by name, with the Mustafa message the loop already carries.
  - The loop's object gains a fifth text: every line of r1.sh that names `stamps.txt` (the writers of browser-stamps).
  - The R1.md:302 clause (ADV-U8H-2) stays. Removing built text to make room is not needed (section 2), and it would
    move a Low's place for size.
- **(c) S14-VMS: taken. It does not depart from his words, so it is mine.**
  - His words: "R1's results push carries the s14 folder whole, through the same redaction check", and "The
    redaction check runs on the s14 folder itself before the push, and its result is in the run's record."
  - "The same redaction check" still holds as written. The pre-push hook, the push's own check with `--vms`, still
    runs over everything pushed, the s14 folder included (pre-push:33).
  - The run on the folder itself is still "the redaction check": the same fixed copy, the same program. It only runs
    without the VM allow-list, so it is stricter, never looser. It refuses an address his second condition already
    says must stop and come to him (RS's own address in that folder).
  - What it departs from is my own C5's words ("with `--vms`", advisor-u8-f1-brief.md), which are mine to change. C5
    now reads as C-F5 gives it below.
  - It goes in the day's message, his to reverse.
  - The cost: none on today's content. The folder's only addresses are the two documentation addresses, which
    redact-check's built-in allow-list admits (u8-build.md F4).
- **None of (a)-(c) needs another security round.** Each is the seat's own text, or a stricter use of its tool. No
  Critical or High is open. I read the fix pass's pushed head; that is not a round.

## 2. The fix pass's size: at or under 6,282 (U8-F1 at most +93 net over 77ffc95)
Net lines for each fix, from the worktree's text (the builder measures; `git diff --numstat 77ffc95`, added less
removed):
| Fix | Net | How |
|---|---|---|
| C-F1: SEC-U8F1-2's merge of :207 and :216 into one exact list | about -1 | :216-217's statement goes; its three wanted lines join :207's list |
| C-F2: SEC-U8F1-1 (a) | 0 | the filter's text, in place |
| C-F3: SEC-U8F1-1 (b), the dash-form MAC and the stamps text | 0 to +1 | the loop's regular expression and object, in place; a wrapped line may add one |
| C-F4: SEC-U8F1-2's shared messages and one-loop count | 0 | message strings, and one count added to :222's existing assert |
| C-F5: SEC-U8F1-1 (c) and its pin | 0 | PROMPT-R1.md:124-129 rewrapped in place; R1.md:294-295 in place; r1plan.test.mjs:211's assert gains one condition |
| C-F6: SEC-U8F1-3 and -4 | 0 to +1 | PROMPT-R1.md's same six lines; R1.md:88 gains the Stop B record (may need one line) |
| **Whole** | **about -1 to +1** | |

**The rule.**
- The builder writes the running net total after each fix. The whole must end at or under +93 (U8 at or under
  6,282).
- Before a fix would pass it, carry the rest, last first:
  1. C-F6's Stop B record (SEC-U8F1-4's second part);
  2. then C-F6's wording (SEC-U8F1-3, then SEC-U8F1-4's first part);
  3. then C-F4.
- C-F1 to C-F3 and C-F5 (the Medium and the merge that pays for it) are never carried. If they alone would pass +93,
  stop before they do, and report what grew, why and the cheaper option (U8F1-SIZE (a)'s rule).
- Nothing is removed to make room, and nothing is moved to get under the stop.

## 3. The fix pass's brief items, for U8-F1's builder (a7fe1b9f436f36bb1); paste as written
> **U8-F1's fix pass (the security round's Medium and Lows; the advisor's ruling, advisor-u8-f1-sec.md in the main
> folder).** Your worktree is /home/user/Shamble-Frontlines/.claude/worktrees/agent-a7fe1b9f436f36bb1, at 77ffc95,
> with U8-F1's uncommitted changes. You edit only these files under its spikes/netcode-proof/ops/:
> tests/s14.test.mjs, operator/PROMPT-R1.md, runs/R1.md and tests/r1plan.test.mjs.
>
> For reference, read only: the round's notes, /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/
> security-u8-f1.md (sections 2 and 6), and his words at /home/user/Shamble-Frontlines/.checkpoints/
> netcode-proof-test/history.md:8090-8106.
>
> No cargo, no commit, no fetch.
>
> **Size.** U8 stands at 6,282, its stop; U8-F1 is at +93 net over 77ffc95. After each item, write the net total
> (`git diff --numstat 77ffc95`, added less removed, over every file you touch) in your notes. The whole must end at
> or under +93.
> - Before an item would pass it, carry the rest, last first: item 6's Stop B record; then item 6's wording; then
>   item 4.
> - Items 1, 2, 3 and 5 are never carried. If they alone would pass +93, stop before they do and report what grew,
>   why and the cheaper option.
> - Remove nothing to make room.
>
> **Notes.** A section "# FIX PASS (security round)" in your u8-f1-build.md: one block per item as you finish it.
>
> 1. **One exact list for npt-s14's `$OUT` lines** (SEC-U8F1-2, first, since it frees a line).
>    - In s14.test.mjs's pin test, merge the two checks "npt-s14's lines writing them" (:207) and "npt-s14's other
>      $OUT lines" (:216-217) into one `only(...)`:
>      - the filter is `/\$\{?OUT\b|"\$PROBE" (?:server|client|bursts)/` over npt-s14's trimmed lines;
>      - the wanted list is today's eight lines plus the three `$OUT` lines from :217.
>    - Red: before the merge, add the scratch line `hostname > $OUT/host.txt` after npt-s14's run loop. The test must
>      PASS (the gap, recorded). After the merge, it must FAIL by name. Restore npt-s14 by Edit, sha256 equal.
> 2. **SEC-U8F1-1 (a).** The filter of "r1.sh's writes into the folder" (:224) becomes
>    `/^[^#]*(?:>|\btee |\bcp |\bmv |\bln |\binstall |\bpython3 |\bnode |s14_run ")/`.
>    - On today's tree the test must still pass with the wanted list unchanged. If the new filter catches any other
>      line, stop and report it; do not add it.
>    - Plant: a heredoc writer in r1.sh's s14 block, e.g. a line `python3 -I - "$1" << 'PY'`. The test fails by name.
>      Restore r1.sh by Edit, sha256 equal.
> 3. **SEC-U8F1-1 (b), with SEC-U8F1-2's dash-form MAC and stamps text.** In the address loop (:236-238):
>    - The regular expression becomes the address forms, plus the dash-form MAC, plus the host and interface names:
>      `/\b\d{1,3}(?:\.\d{1,3}){3}\b|\b(?:[0-9a-f]{2}[:-]){5}[0-9a-f]{2}\b|hostname|nodename|gethost\w*|getfqdn|\buname\b(?! -r\b)|platform\.node|getsockname|getpeername|getifaddrs|if_nameindex|ifconfig|\/sys\/class\/net|\/etc\/host\w*|\b(?:addr|link|neigh) show\b/gi`.
>      The wanted list stays the two documentation addresses, and the label names "addresses, host and interface
>      names".
>    - The loop's object gains `"r1.sh's stamps lines"`: every non-comment line of r1.sh that names `stamps.txt`.
>    - The check "the stamps' writers" (:230) reads every non-comment line naming `stamps.txt`, with r1.sh's
>      `cp -p "$R/browsers/stamps.txt" ...` line added to its wanted list.
>    - On today's tree the test must pass: the seat's grep found only npt-s14's `uname_r` and `uname -r`, which `\b`
>      and the lookahead exclude.
>    - Then run redact.test.mjs, which scans the new regular expression's text, and record that it is clean.
>    - Plants, each failing by name, each restored with sha256 equal:
>      - `"role": f"server {socket.gethostname()}"` in probe.py;
>      - `aa-bb-cc-dd-ee-ff` in probe.py;
>      - a third stamps.txt writer of another shape in r1.sh.
> 4. **SEC-U8F1-2's rest.**
>    - The two asserts at :213 (compare.py summary's one print) and :222 (the listed net.ipv4 settings) carry the
>      shared message: "a change that puts anything more into the pushed s14 folder stops and goes to Mustafa first".
>    - :222's assert also pins that npt-s14 has exactly one `for s in ` loop (the count joins its deepEqual).
>    - Plant: a second one-line `for s in ../../kernel/hostname; do` loop in npt-s14. It fails by name. Restore.
>    - An IPv6 literal by name is not added: on source text it would trip on Python slices such as `[1::2]`
>      (u8-f1-build.md, "A trap met"), and the repo's redact test already catches an IPv6 literal.
> 5. **SEC-U8F1-1 (c), S14-VMS: the s14 folder's own check runs with no VM list.**
>    - PROMPT-R1.md section 6 step 2 (:124-129), rewrapped in the same six lines. Before the push, in PowerShell, the
>      fixed copy `~/npt-operator/redact-check.mjs` runs on `<results>\r1\s14` with no VM list, so a VM's own address
>      there is a finding. Its whole output is saved as `<results>\r1\s14-redact.txt` in UTF-8. Its last line goes in
>      step 4's summary. A finding, or any exit but 0, means no push: stop and tell him.
>    - R1.md:294-295's parenthesis: "(the redaction check's fixed copy, run on the s14 folder first with no VM list;
>      then the pre-push hook's own over the whole push)".
>    - r1plan.test.mjs:211's assert also requires that PROMPT-R1.md's step 2 names no `--vms` before `run on
>      `<results>\r1\s14``.
>    - Proof, a scratch run, not counted: your fixture .gate/u8-f1/s14-fixture/s14.
>      - With no VM list it gives 0 findings.
>      - Then append one address from the fixture's vms.txt to the fixture's kernel.txt. With `--vms` that reads 0
>        findings (today's gap); with no VM list, 1 finding.
>      - Restore the fixture. Record all three outputs.
>    - Plant: `--vms ~/npt-operator/vms.txt` put back in PROMPT-R1.md. r1plan fails by name. Restore by Edit, sha256
>      equal.
> 6. **SEC-U8F1-3 and -4 (Lows, carried last if the lines run out).**
>    - In the same six lines of step 2, the save is named as `| Out-File -Encoding utf8 <results>\r1\s14-redact.txt`
>      (Windows PowerShell's `>` writes UTF-16, which the hook reads as binary).
>    - The re-read and the push run "in the same PowerShell": "Then, in the same PowerShell, push to
>      `claude/npt-results` after the pre-push hook passes (a prompt)".
>    - Stop B's start-check line (R1.md:85-88) ends: "its printed path and `git --version` are recorded".
>    - No new test: operator.test.mjs 3.6 already checks every backticked git command in PROMPT-R1.md against every
>      deny; run it.
>
> **End checks**, from your worktree's spikes/netcode-proof:
> - `node --test` on the 8 browser-free ops test files (95 pass before: record the new count, 0 fail).
> - `bash ops/tests/wrappers.test.sh`: "315 ok, 0 failed".
> - `node ops/operator/redact-check.mjs .` over the spike: 0 findings.
> - `node check.mjs --only refs,self,redact`: all 3 pass.
> - The scratch s14 fixture runs as in item 5.
> - No plant left; ops/browser's package files unchanged.
> - `git diff --numstat 77ffc95`: the table and the net total (at or under +93).
> - No commit.
>
> **Hand-back:** in your notes, under "# FIX PASS (security round) HAND-BACK":
> - each item, with its red, its plant and its restore hash;
> - the end-check counts;
> - the size table;
> - anything carried, and why.
> Then stop. The main session takes the snapshot (wip/u8-f1-build.patch, a new sha256), and the advisor reads the
> pushed head.
>
> Write files with the Write and Edit tools only (no sed, no shell redirect, no python writes; never `cargo fmt`
> writing files: apply rustfmt's diff by Edit); an agent with no Edit tool rewrites its notes file whole with Write.
> git and gh always start their own command, with no other program in front of them. Never switch the main folder;
> never `cd` into another worktree; no `git -C` and no `git -c`; no shell loops.

## 4. The Lows
- **SEC-U8F1-2: fixed with the Medium** (items 1, 3 and 4), since it rides the same file.
  - Dropped, with the reason in item 4: an IPv6 literal by name.
  - C6's message is now given at :213 and :222 too.
- **SEC-U8F1-3: fixed** (item 6), within PROMPT-R1.md's step 2, which item 5 rewrites anyway. If the lines run out,
  it is carried: a STATUS follow-up named in U8's report, taken at Stop B's fills.
- **SEC-U8F1-4: fixed** (item 6): the save named, and the Stop B record of the printed path and `git --version`. The
  same carry rule applies, with the Stop B record carried first.
- **The builder's own Low** (logq on a JSON line that is not an object; u8-f1-build.md, Open): carried. Its taker is
  a STATUS follow-up named in U8's report. npt-log writes objects only, so it is robustness, not exposure, and it
  rides no file this pass touches.

## Decisions needed
None. S14-VMS is mine (section 1 (c)): it is stricter than his two S14-PUSH sentences and departs only from my own
C5. The size stays inside U8F1-SIZE (a)'s stop of 6,282. If the Medium's items alone would pass it, the builder stops
and he gets three lines then.

## For Mustafa
Nothing now. For the day's message, his to reverse:
1. The s14 folder's own redaction run before the push now uses no VM list (S14-VMS), so a server address in that
   folder stops the push. The pre-push hook still checks the whole push with the VM list, as before.
2. The pin test now also refuses host and interface names, and the other writer forms the security round found.
3. At Stop B he will see this in the PROMPT-R1.md text he approves, and Stop B records the hooks path and `git
   --version`.

Status: COMPLETE 2026-11-27 00:59 UTC

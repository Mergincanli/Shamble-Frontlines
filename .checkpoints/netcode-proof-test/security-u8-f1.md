# security-auditor: U8-F1's one round (what leaves the R1 server), 2026-11-26

Read only. Snapshot wip/u8-f1-build.patch, sha256 26e0ec050dd740a98e0208186439a4e0194ebd91c59407097ea97a1755811478
(checked: equal), base 77ffc95. The built files and HAND-BACK were read by path in worktree agent-a7fe1b9f436f36bb1
(line numbers below are the worktree's). I also read redact-check.mjs, pre-push, settings.json, npt-s14, probe.py,
compare.py, r1.sh whole, advisor-u8-f1-brief.md (C1-C7), advisor-u8-head.md sections 3-4, and history.md:8068-8106.
I ran no test, cargo or fetch. The advisor tool timed out, so this verdict is mine alone.

## Verdict: APPROVE WITH FINDINGS (one Medium, three Lows)
What R1 pushes today names no server. Every new print is an integer. `attempts` and the wider R1.md sentence stay
inside "the s14 folder whole". The re-read line closes SR-1 and SR-2 as worded. The Medium is about later changes: for
RS's own host name, its own routed addresses and a hardware address, the pin test is the only guard, and it misses
writer forms this code already uses. The fix is net zero or one line (below). Fix it in the fix pass rather than defer
it: a security finding left open keeps the PR needs-mustafa.

## 1. Items 1a and 1b: is every printed value an integer? Yes.
- P1's lines (r1.sh:645-648, printed at :680 after the reports):
  - `P1 <k> <v>` is printed only when `type(v) is int`, which shuts out bool, float, text and None, and the name
    matches `[a-z0-9_]+`.
  - The policy lines go through pol() (:616-617). An ASCII `[A-Z_]+ -?\d+` whose name is in POL prints
    `<POL number> <int(priority)>`. Anything else prints `-1 -1`, including "", "unknown", an unknown name and a
    non-string. int() turns any Unicode digit into ASCII, so the output is always two integers.
  - `P1 non_integer_fields <len(vals) - len(p1)>` is counted before the `+=`, so it is an integer of 0 or more.
  - No new line can hold `INVALID`: the names are lower case, and the probe block's grep at :593 is case-sensitive.
- ws_split.txt (:718-722) prints integer fields only and `ws_split non_integer_fields <n>`. With no event, or no log
  (logq swallows OSError), it prints the one fixed sentence that 1b allows.
- The values carry nothing that names the server. l2_summary's fields (server/src/probe.rs:639-653, l2/names.rs) are
  counts and nanoseconds, with no address, port or inode. ws_split's rows are (&str, u64) counts (HAND-BACK, main.rs:801-805).
- Not a finding, noted:
  - If P1's log is missing, the loop's `continue` (:638-640) skips the P1 block, so no P1 line prints at all and only
    the leg's INVALID line does. No exposure.
  - The existing policy-mismatch line (:673-674) prints the policy names. That is a verdict line, and C4 already ruled
    those to be summaries.
  - logq's `-` (:199-200) raises KeyError on an event with no `f`. That is the same class as the builder's Low:
    robustness, not exposure.

## 2. The pin test and the redaction check: are a host name, a routed address and a hardware address each still caught?
What each check reads:
- **redact-check, at push time** (the s14 run and the hook, both with `--vms`):
  - A routed IPv4 or IPv6 address that is not a VM's is caught (:219-236).
  - **RS's own addresses pass.** vms.txt holds "the kept VM addresses" (pre-push:5, :32-33), and v4Ok/v6Ok allow
    them (:216-217). The rDNS name of a VM address passes too (:245).
  - A bare host name is never a finding: HOSTNAME (:135, :239-249) fires only on an embedded non-VM quad.
  - A hardware address is never a finding: the IPv6 candidate fails isIPv6, and there is no MAC rule.
- **The repo's redact test** (check.mjs redact, redact.test.mjs, no VM list) catches a non-documentation IPv4 or IPv6
  literal in any of these sources.
- **The pin test** (s14.test.mjs:200-239) catches these forms by name:
  - a new kernel.txt source or variable;
  - a new npt-s14 `"$OUT/` file or line;
  - a new compare.py summary value or import;
  - a new probe.py `"key": `;
  - a new r1.sh `>`, tee or cp line in the block;
  - a literal IPv4, or a MAC in colon form.
- **Dropping `::` from the MAC pattern loses nothing.** A colon-form MAC never contains `::`, and
  `(?:[0-9a-f]{2}:){5}[0-9a-f]{2}` with the `i` flag (:237) still matches every colon-form MAC. The builder's plant
  (a MAC in probe.py) failed by name.

So for RS's own host name, its own routed addresses (both families) and its MAC, only the pin test stops a later
change. advisor-u8-f1-brief.md:22 says the same: "only the pin test makes such a change stop". That is why the gaps
below matter.

## 3. `attempts` and R1.md's wider sentence (R1.md:293-302): neither puts out more than S14-PUSH (a) allows
- His words are "the s14 folder whole". `attempts` (r1.sh:105-107) is in that folder and holds `attempt <n> <UTC time>`.
- What the sentence adds was all in the folder already at 77ffc95. U8-F1 adds no writer into s14/: its r1.sh hunks
  touch only logq, probe_presence and ds_selftest. The added items:
  - netem's `tc qdisc show` lines (qdisc, delay, loss);
  - "cubic" and the setsockopt OSError texts (probe.py:59-63);
  - the attempt and browser-leg UTC times, from the fixed room names (r1.sh:776, :784, :794);
  - compare.py's standings (numbers and fixed words).
- None of these names the server. His Advisor's description (history.md:8094) was a read of npt-s14 and probe.py only.
  The sentence describes the folder more truly; it neither widens the push nor loosens his stop rule.
- For Stop B, as information: the run times also say when he ran the test. No other personal data is in the folder.

## 4. The re-read line closes SR-1 and SR-2 as advisor-u8-head.md words them
- PROMPT-R1.md:127-129 follows advisor-u8-head.md:141-143: PowerShell, just before the push, `<results>` the folder the
  push uses (:123-124), the clone's own hooks folder, anything else stop.
- The line spells no "hookspath", so settings.json:55-56 cannot deny it. The test 5 assert and operator.test.mjs 3.6
  pin it.
- SR-2's inline forms (`-c`, `--config-env`, `--no-verify`) show in the push's own ask prompt (settings.json:8-11,
  :30-31, :53-54).
- The hook re-checks the fixed copy's SHA-256 at every push (pre-push:26-29).
- One residual, Low: the line does not name the shell the push runs in (SEC-U8F1-3).

## 5. Is Stop B's laptop read enough for both?
- **The re-read line on Git for Windows: yes.**
  - Stop B's read (R1.md:87-88) covers the two Windows-specific parts: Claude Code's PowerShell matcher (prompting,
    not denied), and the path Git for Windows prints when nothing is set.
  - Whether `--git-path hooks` follows core.hooksPath is git's own portable code, proven for forms (i) to (iv) on
    2.43.0. C3 already ruled this enough.
  - Low addition: record the printed path and `git --version` at Stop B, so the run-day comparison is exact.
- **The UTF-16 save: Stop B's read does not test it, and need not.**
  - A UTF-16 file fails closed: ASCII text in UTF-16 has NUL bytes, isBinary sees them (redact-check.mjs:273-275), and
    the hook refuses. It is never a leak.
  - The in-memory buffer is a fair stand-in for that.
  - The day's cost is larger than a re-save, though. The hook scans every commit in the push (:321-341), so a
    committed UTF-16 file blocks every later push until that local commit is rewritten (SEC-U8F1-4).

## 6. Findings (CLAUDE.md's scale)

### SEC-U8F1-1 (Medium): the pin test misses writer forms the code already uses, and nothing else catches these kinds
- **What:** C6 says "any other file, writer, key or address fails the pin test by name". Three plausible later
  changes pass it, and redact-check would not catch them either.
  - **(i) r1.sh's block filter** (s14.test.mjs:224, `/^[^#]*(?:>|\btee |\bcp |s14_run ")/`) misses:
    - a Python heredoc writer, the form r1.sh's own dist_check uses (r1.sh:139-157: `<< 'PY'` with
      `open(..., "w")`);
    - `install`, `ln` and `mv`;
    - a `compare.py judge` call (it writes verdict.json into the folder, compare.py:255).
  - **(ii) probe.py's values and prints are not pinned**, only its `"key": ` names (:232-235):
    - socket and subprocess are already imported (probe.py:23-25);
    - its stdout is the pushed JSON whole (npt-s14:64-73);
    - its bursts mode runs as root in the root namespace (npt-s14:72), where eth0's addresses and MAC are visible.
  - **(iii) stamps.txt's writers are pinned by shape** (:230). A third writer of another shape is not seen, and the
    file is copied in as browser-stamps.txt (r1.sh:750).
- **Exploit sketch (accidental, the realistic case):** a later unit adds `"role": f"server {socket.gethostname()}"`,
  a debug `print(...)` in bursts, or a heredoc in s14() that writes `ip addr` output. The pin test passes. The push
  check allows RS's kept address and never flags a host name or a MAC. The result is a public and permanent push of
  what his condition says must stop and go to him first.
- **Workaround:** every such change still gets the advisor's head read and this seat.
- **Fix, cost under section 0:**
  - **(a) Net zero:** widen :224's filter to `(?:>|\btee |\bcp |\bmv |\bln |\binstall |\bpython3 |\bnode |s14_run ")`.
    A heredoc's call line then fails by name. The existing compare.py line is already in the list.
  - **(b) +1 line:** one assert over `npt + probe + cmp + block` and the stamps lines, carrying the Mustafa message:
    `assert.doesNotMatch(..., /hostname|nodename|gethost|getfqdn|\buname\b(?! -r\b)|platform\.node|getsockname|getpeername|getifaddrs|if_nameindex|ifconfig|\/sys\/class\/net|\/etc\/host|\b(?:addr|link|neigh) show\b/i, ...)`.
    - It is clean on today's files. I grepped: the one hit is npt-s14:53's `uname_r` and `uname -r`, which `\b` and the
      lookahead exclude. r1.sh's `rs_ip` (:60) lies outside the slice.
    - The builder must re-run redact.test.mjs on the new regex text.
    - **Paid by carrying item 3**, the R1.md:302 clause (ADV-U8H-2 is Low, and the head read lets it go to R1's report).
      If (b) is folded into the address loop's line (:237), it is net zero and nothing is carried.
  - **(c) Net zero, the advisor's call:** run the s14 check (PROMPT-R1.md:124-125) without `--vms`.
    - The folder's only addresses are the documentation pair, so RS's own address there becomes a finding: no push,
      tell him. That is his condition enforced at the push for both address families.
    - It keeps the same fixed copy. It departs from C5's words ("with `--vms`"), so it is not mine to choose. It does
      not cover host names or MACs; (b) does.

### SEC-U8F1-2 (Low): narrower forms the pin test misses
- **What:**
  - npt-s14 writes through `"${OUT}/..."` or an unquoted `$OUT/...` outside the kernel block match none of the three
    npt-s14 checks (:206, :207, :216).
  - A second one-line `for s in ../../kernel/hostname; do` loop reuses the allowed source. The lines starting with
    `for s in ` are dropped at :219, and :222 reads only the first loop.
  - There is no IPv6 literal form; the repo's redact test catches one, but not by name.
  - There is no dash-form MAC.
  - :213 and :222-223 fail without the "goes to Mustafa first" message that C6 asks for.
- **Exploit sketch:** `hostname > $OUT/host.txt` after npt-s14's run loop passes the pin.
- **Fix, net zero:**
  - Merge :207 and :216 into one exact list filtered by `/\$\{?OUT\b|"\$PROBE" (?:server|client|bursts)/`.
  - Give :213 and :222 the shared message.
  - Fix (iii) by changing :230 to every line naming `stamps.txt`, with r1.sh:750 added to its list.
  - SEC-U8F1-1 (b) also closes the host-name cases.

### SEC-U8F1-3 (Low): the push's shell is not named
- **What:** PROMPT-R1.md:127-129 reads the hooks path in PowerShell, then "push", in no named shell. A GIT_CONFIG_*
  form that lives in Bash's profile only is not seen by a PowerShell read. The advisor's own wording is "set in the
  same shell" (advisor-u8-head.md:156-157).
- **Exploit sketch:** an approved `~/.bashrc` line exporting a hooks path, with the push then run in Bash. The hook is
  skipped, and the s14 run still covered only s14/.
- **Fix:** "Then, in the same PowerShell, push to ...". Net zero if :127-129 re-wrap within their lines. If not, it is
  a STATUS follow-up (a Low triggers no push on its own).

### SEC-U8F1-4 (Low): a UTF-16 s14-redact.txt fails closed, but blocks the day's push until a local commit is rewritten
- **What:** PROMPT-R1.md:126 names UTF-8 but not the command. Windows PowerShell 5.1's `>` writes UTF-16. The hook
  scans every commit in the push (redact-check.mjs:321-341).
- **Fix:** name the save (`| Out-File -Encoding utf8 <results>\r1\s14-redact.txt`; a BOM reads clean per HAND-BACK),
  and record the printed hooks path and `git --version` at Stop B (R1.md:87-88). Net zero if it fits the wrap. Else it
  is a STATUS follow-up.

## 7. What leaves the server (this seat's coverage table; U8-F1 touches no player action, so no anti-cheat row)
| Pushed file | Holds | Guard | Gap |
|---|---|---|---|
| probe/presence.txt | the verdicts, the reports, P1's integers | the code (section 1); the hook | none |
| ds-selftest/ws_split.txt | ws_split's integers, or one fixed line | the code (section 1); the hook | none |
| bringup/facts.txt, collect/ | as at 77ffc95 (C4: summaries) | the hook | not re-read here (out of this round) |
| s14/ whole | section 3's list; the documentation addresses only | the pin test; the s14 run; the hook | SEC-U8F1-1 and -2 (later changes only) |
| s14-redact.txt | redact-check's last line (0 findings, or no push) | the hook | SEC-U8F1-4 |

## What I could not check
- No Windows: the matcher, Git for Windows' printed path, and a real PowerShell save are Stop B's.
- I ran nothing. The plants and test counts are the builder's (HAND-BACK).
- The deny regex in SEC-U8F1-1 (b) is checked by grep against today's four files, not run.

## Decisions needed
- **S14-VMS (the advisor's, not Mustafa's: C5 is the advisor's condition):**
  - The question: should the s14 folder's own redaction run drop `--vms`, so that a VM's own address in s14/ stops
    the push?
  - Why it matters: today, redact-check lets RS's kept address through, so only the pin test guards that.
  - Options: (a) keep C5 as written; (b) drop `--vms` for the s14 run only, a net-zero change with the same fixed copy.
  - Recommendation: (b), together with SEC-U8F1-1 (a) and (b).

## For Mustafa
Nothing now. At Stop B, when he approves PROMPT-R1.md and R1.md by hand, there are two things to know:
1. R1.md's s14 sentence lists more than his Advisor's read did: netem's settings, the congestion-control names and
   errors, the attempt and browser-leg times, and the run standings. All of it was in the folder already, and none of
   it names the server.
2. Stop B's start check now includes one PowerShell read: `git -C <results> rev-parse --git-path hooks` must prompt,
   and must print the clone's own hooks folder.

DONE

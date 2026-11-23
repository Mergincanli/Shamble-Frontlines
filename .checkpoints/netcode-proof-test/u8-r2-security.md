# U8 Stop B round 2: security-auditor seat (SEC-R2-*), xhigh, 2026-11-23

Plan: (1) verify hashes (ruling daf55033…, patch bca52b2c…); read advisor-u8-stopb.md :161-200, :227-285, :299-301;
u8-build.md "Fix pass 2"; u8-sec-sb-3-docs.md. (2) Interdiff old (.gate/u8-r2-old) vs W, U8 files only. (3) Judge items 6-9,
12's new npt-load-ctl verb, 16's SEC-SB-8 lines, 17-19; the dist-accept block; npt-load-ctl start's log delete. (4) Verdict:
SEC-SB-1 and each Medium closed or not; new findings SEC-R2-n (changed lines only, unless Critical). Last line DONE.

W = /home/user/Shamble-Frontlines/.claude/worktrees/agent-ac59cce3e5d002441/spikes/netcode-proof; cites are W-relative.
Inputs: advisor-u8-stopb.md sha256 daf55033…9192 OK; wip/u8-build.patch bca52b2c…6eb7 OK, and W equals it (43 files,
byte-equal: scratchpad patchcmp.py); u8-sec-sb-3-docs.md 426f2608…; u8-build.md 6a67751d… (Fix pass 2 read). Interdiff:
git diff --no-index of .gate/u8-r2-old/…/ops against W/ops (the base's tracked files ignored).

## VERDICT: REVISE
SEC-SB-1 is closed in the package, and the Stop B listen read is written. Every round-1 Medium is closed. But two new
Highs sit on the changed lines. Both fail closed, so neither opens a hole, but each would stop R1's bring-up on the
day, and the container tests cannot see either. Each is a one-line fix. Counts (new): Critical 0, High 2, Medium 1,
Low 1. Under section 7's budget the two Highs go to the advisor, for one more round on them alone.

## New findings (changed lines only), most severe first

### SEC-R2-1 (High) First boot writes into /etc/issue.d, which Ubuntu 24.04 does not ship, so both VMs' builds fail
- Where: ops/cloud-init/rs.yaml:116-119 and lg.yaml:91-94 (`{ …ssh-keygen -l… } > /etc/issue.d/50-npt-hostkeys.issue`,
  firstboot's first step, under `set -euo pipefail` and the ERR trap).
- What fails: in this Ubuntu 24.04.5 container `/etc/issue.d` does not exist and no package owns it (`dpkg -S
  /etc/issue.d`: no path found; base-files ships only /etc/issue and /etc/issue.net; util-linux ships agetty but not
  the folder). agetty reads the folder only if it exists (util-linux's issue.d handling, cited in the openSUSE and Red
  Hat reports the search found). The redirect then fails, set -e ends the script, and the trap writes build.failed.
  This is the first step, so no clone, wrappers, sudoers or build happen on RS or on LG. PROMPT-R1.md §2.3 then sees
  FAILED, and the 10:15 rule tears both VMs down.
- Scenario: every R1 attempt stops at first boot. On a retry the console shows no fingerprint, so no SSH is made, by
  §2.2's own rule.
- Confidence: high for 24.04's packages (read here); medium-high for Hetzner's image (no VM).
- Smallest fix: `install -d -o root -g root -m 0755 /etc/issue.d` before the write, on both VMs. Test 1 asserts it
  comes before the write; plant: drop it, and test 1 must fail.

### SEC-R2-2 (High) Bring-up's exact sudo check reads the VM's wrapped `sudo -l` as wrong and stops bring-up
- Where: ops/runs/r1.sh:424 (`sudo -n -l > "$d/sudo-l.txt"`, a regular file) and :425-426 (sudo_exact :158-177 dies on
  any difference); test fixture ops/tests/r1sh.test.mjs:511-534 (unwrapped lines only).
- What fails: sudo 1.9.15p5 turns line wrapping off only when stdout is a pipe. display.c's display_privs() reads
  `if (fstat(STDOUT_FILENO, &sb) == 0 && S_ISFIFO(sb.st_mode)) cols = 0;`. Otherwise the width comes from the tty, or
  from COLUMNS, or defaults to 80 (lib/util/ttysize.c). `ssh npt-rs r1.sh bringup` has no tty and sets no COLUMNS, so
  the width is 80. lbuf.c then splits any longer line at a space and indents the rest by 8. Two of the nine entries are
  longer: the peers line is 158 characters and the rooms line is 92 (computed here from sudoers.d/npt in sudo -l's
  form). sudo_exact counts each continuation as an extra line and each cut first half as a missing one, then exits 1.
- Scenario: at about 10:06 on 3 Dec, `r1.sh bringup` dies: "npt's sudo list is not exactly sudoers.d/npt's command
  lines". C5, cpus and alone never run, and the script is fixed at commit Y in the root-owned clone, so only a new
  commit and a new bring-up fix it. The fixture models a list the VM does not print, so test 7's proof does not hold.
- Smallest fix: r1.sh:424 `sudo -n -l | cat > "$d/sudo-l.txt"` (stdout a pipe, so sudo does not wrap; pipefail keeps
  sudo's failure). Test 7 adds a fixture with the peers and rooms lines wrapped at 80 the way lbuf.c wraps them (it
  must stop), and asserts the pipe form in bringup.

### SEC-R2-3 (Medium) The PowerShell `git -c *` deny also blocks `git -C`, which start check 4 and the push use
- Where: ops/operator/settings.json:55 `PowerShell(git -c *)` (:32 is the Bash form); ops/operator/PROMPT-R1.md:22 (start
  check 4, `git -C <run> rev-parse HEAD` and `git -C <run> diff --quiet …`); rules.md:7 (the session runs native
  PowerShell).
- What fails: Claude Code's PowerShell rules: "Matching is case-insensitive" (code.claude.com/docs/en/permissions, the
  PowerShell section). So `git -c *` matches `git -C .gate\npt-run rev-parse HEAD`, and a deny refuses it. The same
  holds for any `git -C .gate\npt-results push …` at §6.2. This deny came from my own round-1 fix text for SEC-SB-7.
  The docs record (u8-sec-sb-3-docs.md) covers the read setting, not this.
- Scenario: start check 4 cannot run in PowerShell. The session stops ("on a mismatch, stop"), or works round it with
  a cd or the Bash tool, one prompt each.
- Smallest fix: replace the PowerShell form with `PowerShell(git -c *hookspath*)` and `PowerShell(git * -c
  *hookspath*)`. Case-insensitive matching then covers every spelling of the key, and `git -C <run> …` names no
  hooksPath. Keep the Bash `git -c *` (no case rule is documented for Bash). The operator test asserts that no deny
  matches PROMPT-R1.md's own git commands when compared case-insensitively. At Stop B, start check 4 is run once in
  each shell and must prompt, not deny.

### SEC-R2-4 (Low) The Stop B listen read names no command and is not repeated for a driver replaced on the day
- Where: ops/runs/R1.md:77-80 and ops/operator/rules.md:66-67 ("one dry start of each driver as lib.mjs starts it"; no
  command given); ops/operator/PROMPT-R1.md:30-31 (start check 6 may re-download a driver on R1's day, with no listen
  read after it).
- Smallest fix: name the read. `Start-Process <driver path> -ArgumentList <config.mjs driverArgs(browser, 9515)>
  -PassThru`, then `Get-NetTCPConnection -State Listen -OwningProcess <its Id>` (loopback only), then `Stop-Process`.
  Start check 6 repeats it for any driver it replaces.

## Round-1 findings in my scope: closed or not
- **SEC-SB-1 (High): CLOSED.**
  - Every driver is started by lib.mjs from its recorded path, after its SHA-256 check (lib.mjs:83-90). driverArgs
    gives chrome and edge `--port=<p>` only, and firefox `--host=127.0.0.1 --port=<p> --websocket-port=0`
    (config.mjs:215-220).
  - remote() names 127.0.0.1:<p> path "/" (config.mjs:222-226). WebdriverIO's `definesRemoteDriver` therefore starts no
    driver (@wdio/utils 10.2.0 build/index.js:766-770, read in round 1), and the capabilities carry no `wdio:*` option
    (config.mjs:199-213).
  - Without `--allowed-ips` chromedriver binds loopback and refuses non-localhost Origin and Host (http_server.cc, read
    in round 1). geckodriver binds its `--host`, with Host allowed by default (geckodriver Flags docs).
  - Test 4 holds the argv and remote-options asserts (browser.test.mjs:18-35), and plants P15 and P16 fail it
    (u8-build.md 2.6).
  - The Stop B listen read is written: R1.md:77-80, rules.md:66-67 (Get-NetTCPConnection, read only, loopback only or
    stop). C8 now also reads the per-program inbound allow rules, and he cancels any firewall dialog: PROMPT-R1.md:24-29,
    R1.md:75-80, rules.md:68-72. The read itself happens on his laptop at Stop B (SEC-R2-4 asks that its command be
    named).
- **SEC-SB-2 (Medium): CLOSED.** rs.yaml:170 `runuser -u npt -- install -d -m 0700 /home/npt/.ssh` (as npt, so a planted
  link reaches only npt's own files). Test 1's 2.7 case asserts that root touches no home path except through runuser
  (cloudinit.test.mjs), and plant P17 fails it.
- **SEC-SB-3 (Medium): CLOSED, against the docs record.**
  - `permissions.blockReadsOutsideWorkingDirectories: true` sits inside "permissions", as the record has it
    (settings.json:6). The version gate (v2.1.257 or later) is at PROMPT-R1.md:34 (start check 8) and R1.md:83.
  - `Read(~/.ssh/**)` and `Read(//**/.ssh/**)` are denied (settings.json:71-72). The record says Read denies also reach
    the cat, head and tail forms Claude Code recognizes.
  - The PowerShell fallback denies (Get-Content, gc, cat, type, Select-String, Format-Hex, Copy-Item, each against
    npt_operator and .ssh) are at settings.json:76-89. rules.md:28-42 names the documented gaps.
  - The passphrase, with the key held in Windows' ssh-agent, is the wall either way (R1.md:84; rules.md:39-41).
- **SEC-SB-4 (Medium): CLOSED,** with SEC-SB-1 (no `--allowed-ips` anywhere; test 4 asserts it, and plant P16 fails it).
- **SEC-SB-5 (Medium): CLOSED as ruled.**
  - The list is committed at X as ops/runs/r1-dist.sha256, comments only until Stop B's fill.
  - dist_check (r1.sh:131-149) runs in bringup (:441) after C5 and before any room. It names every hash that differs,
    every missing file and every extra file. An empty list reads as differing, and a missing dist.sha256 makes python
    fail, so bringup dies and no dist.txt is written.
  - host_active (:119-127) refuses `host` (:780) and every `browsers` call (:690) when dist.txt is missing, or when
    DIST-DIFFERS exists without DIST-ACCEPTED. The server chain does not depend on it.
  - The `dist-accept` block (:786-792) refuses when there is no difference and refuses an empty list (:789). It prints
    the differing names, then writes the stamp. PROMPT-R1.md:73-75 runs it only after his typed word. That is enough to
    record his word: DIST-DIFFERS sits beside the stamp, and the dist cannot change within one VM's life
    (/var/lib/npt/dist is root-owned).
  - Note, no finding: npt-host still serves the dist once the production chain exists. The gate stops his browsers
    from opening it; it does not stop the host. This matches the ruling's mechanism, and only his /32 can reach the
    port.
- **SEC-SB-6 (Low): CLOSED.** The binaries are read as npt-build into a root temp file and then installed
  (rs.yaml:205-210, lg.yaml:154-158). The dist tar is created as npt-build and extracted by root (rs.yaml:216). Tar
  writes symlinks as entries, never following them; the extract is followed by the link delete; and chown -R clears
  any set-id bit.
- **SEC-SB-7 (Low): CLOSED.**
  - The hook pins the destination: it reads the ref lines first, refuses any remote ref but
    refs/heads/claude/npt-results, then passes the lines to redact-check (pre-push:13-21, :35; `set -euf`). The hook
    test refuses four other refs, and plant P19 fails it.
  - The `--no-verify`, `refs/heads/` and hooksPath-setting denies are in both shells (settings.json:28-36, :51-59).
    Start check 7 reads `git config --get core.hooksPath`, which no deny matches (PROMPT-R1.md:32-33).
  - One side effect is SEC-R2-3.
- **SEC-SB-8 (Low): the LG half is CLOSED; the RS half is not working as built.** PROMPT-R1.md:51-54 runs `id -nG`, the
  exact three-line `sudo -n -l` and steps.log `build done` on LG. The RS half is right in intent but false-stops on the
  VM (SEC-R2-2).
- **SEC-SB-9 (Low): PART CLOSED.** ProtectHome, PrivateTmp, an empty CapabilityBoundingSet, RestrictNamespaces,
  LockPersonality, SystemCallArchitectures=native and the start limit are in (npt-host.service:7-26). The rest waits
  for U6c's merge, as the ruling has it (:224), and is named in the unit's header.
- **SEC-SB-10 (Low): CLOSED.** `meta skuid 2001 ip daddr 127.0.0.53 drop` and `meta skuid 2001 th dport 53 drop` sit
  before the established and loopback accepts (rs.yaml:56-57). `th dport 53` also covers 127.0.0.54 and ::1. Test 1
  asserts the order, with `nft -c`.
- **SEC-SB-11 (Low): CLOSED.** PROMPT-R1.md:7-8 names the link files; §6.3 deletes `.gate\npt-run-links\`
  (PROMPT-R1.md:121).

## GTFOBins on the changed root lines (no caller-chosen argument reaches any of them)
- npt-ctl cpus (npt-ctl:79-97): sed on /proc/1/status, printed through npt_comm_clean. npt_pid1_needs_move
  (npt-lib.sh:104-109) is pure and fails safe: an unreadable list counts as "move". taskset takes the root-owned
  housekeeping set and PID 1's tids from /proc. The tick is validated as before (:68).
- npt-load-ctl is not root. It is the forced command, running as npt-load:
  - `npt-log NN` (:76) is `head -c 256 MiB` of a fixed path, with NN from npt_load_parse's anchored regex
    (npt-lib.sh:159). Any link there is npt-load's own, and it reaches only what npt-load can already read.
  - start's `rm -f -- logs/load-NN.jsonl` (:61) is a fixed path in npt-load's own 0700 directory; it runs only when
    the load is not running (:59), and rm removes a link, not its target.
- firstboot's new root lines are all fixed: ssh-keygen -l on root-owned host keys and agetty --reload (rs.yaml:116-119,
  lg.yaml:91-94; the folder is SEC-R2-1); `command -v` (:134); git fetch with fixed `-c http.*` options (:145; lg :113);
  `runuser -u npt|npt-build` with fixed arguments (:170, :205, :216).
- No sudoers file changed, and npt-s14 changed only in its header (npt-s14:5-6).

## Sources read this round (2026-11-23)
- sudo 1.9.15p5: https://raw.githubusercontent.com/sudo-project/sudo/SUDO_1_9_15p5/plugins/sudoers/display.c (the
  FIFO-only no-wrap), …/lib/util/lbuf.c (word wrap, 8-space indent), …/lib/util/ttysize.c (COLUMNS, else 80). High
  confidence on the code; not run on a VM.
- Claude Code permissions, PowerShell section: "Matching is case-insensitive" (https://code.claude.com/docs/en/permissions,
  read in round 1, saved copy). Medium-high confidence that it covers arguments (not tested on Windows).
- Local: `dpkg -S /etc/issue.d` (no path found), `ls -ld /etc/issue.d` (absent), `dpkg -L base-files` (issue,
  issue.net), Ubuntu 24.04.5 container.
- Web search on /etc/issue.d (openSUSE issue.d(5), Red Hat bug 2024610, util-linux patches): agetty reads the folder
  only if it exists; nothing shows Ubuntu shipping it.

## Notes (one line per finding as found)
- R1 hashes OK; W = patch. R2 item 6 closed (argv, remote options, the read written). R3 item 7: SEC-SB-2, -6, -10
  closed; issue.d absent -> SEC-R2-1. R4 item 8: the setting matches the record; PowerShell's case-insensitive
  `git -c *` -> SEC-R2-3. R5 item 9: refusals hold; dist-accept OK. R6 item 12: the new verb has a fixed path; the
  start rm is safe. R7 items 17-18 OK. R8 item 19: wrapped sudo -l -> SEC-R2-2. R9 item 16's LG lines OK. R10 the
  listen read has no command -> SEC-R2-4.

DONE

# U8 Stop B: security-auditor seat (SEC-SB-*), xhigh, 2026-11-23

Plan: (1) brief 0.1, section 7, :19-23, :121 list; advisor C1-C10, L1-L5; u6c section 5 items 1-8; build notes F1-F28 + fix
pass. (2) Read W/ops: cloud-init rs/lg, firewall, operator (settings, rules, PROMPT-R1, pre-push, redact-check), systemd,
sbin wrappers + sudoers (GTFOBins), browser package (F25), runs (r1.sh, R1.md, s14). (3) Findings SEC-SB-n, Q-U8-1 confirmations,
:121 list by file:line, coverage of builder items F6/F7/F8/F10/F13/F16/F22/F23/F25. (4) Section 7 lines. Last line DONE.

W = /home/user/Shamble-Frontlines/.claude/worktrees/agent-ac59cce3e5d002441/spikes/netcode-proof (read only). Cites below are
W-relative (`ops/...`) unless named otherwise.

Inputs checked: u8-build.patch sha256 60205020…c47d662 OK; u8-brief.md 3d102be9… OK; advisor-u8-brief.md 595521d9… OK;
u8-build.md cf07e435… OK. W equals the patch: all 42 files byte-equal to the patch's added text (scratchpad script).
Judged against the re-plan (u8-brief.md:19-23 + L3 + :287, :995-996), not the stale plan.md lines.

## VERDICT: REVISE
One High (SEC-SB-1: Firefox's driver listens on every interface with no peer check; the package says it keeps a
localhost default). Per the budget, a second round on what changes. Counts: Critical 0, High 1, Medium 4, Low 6.
Everything Q-U8-1 asks me to confirm holds for every wrapper; term 4 fails once, in RS's first-boot script (SEC-SB-2).
The sudo surface (sudoers + wrappers) is sound against GTFOBins: no caller-chosen argument reaches systemd-run,
setpriv, taskset, tcpdump, nft or dig (table below).

## Findings, most severe first

### SEC-SB-1 (High) geckodriver listens on 0.0.0.0 with no peer check; the "localhost default" claim does not hold
- Where: ops/browser/config.mjs:4-5 ("geckodriver keeps its localhost default") and :217-218 (`wdio:geckodriverOptions:
  { binary }`, no host); ops/browser/lib.mjs:86 (Firefox: WebdriverIO starts the driver); ops/operator/rules.md:52-53;
  C8's read ops/operator/PROMPT-R1.md:23-25 and ops/runs/R1.md:66-67.
- What fails: @wdio/utils 10.2.0 build/node.js:1105-1118 starts geckodriver as `{customGeckoDriverPath, cacheDir, port,
  allowHosts: ["localhost"]}`, no host; the geckodriver npm package 8.0.0 then sets `startArgs.host = startArgs.host ||
  DEFAULT_HOSTNAME` (dist/index.js:16) with DEFAULT_HOSTNAME = '0.0.0.0' (dist/constants.js:6). geckodriver's own docs:
  `--allow-hosts` checks only the Host header and "Requests with Host set to an IP address are always allowed";
  `--allow-origins` rejects requests that carry an Origin (browsers), not raw clients; nothing checks the peer IP. Both
  tarballs' sha512 equal the lock's integrity (checked). My own SEC-R1D-2 ("keep geckodriver's localhost default",
  security-r1-deps.md:175, :202-206) carried the same wrong premise; corrected here.
- Exploit sketch: during the Firefox legs (smoke, 20 cert-14d, 20 fallback, -36's 10 minutes) a device on the same
  network scans the laptop, sends `POST /session` with `Host: <laptop IP>`, no Origin, and
  `moz:firefoxOptions.binary` = any program (or drives Firefox to file:// pages) -> code execution or file reads as
  Mustafa on his laptop (operator SSH key, GitHub session). Only the Windows firewall stands: the first 0.0.0.0 listen
  makes Windows show its "allow or block" dialog, and an admin's "Allow access" creates per-program allow rules, which
  take precedence over the default block (learn.microsoft.com Windows Firewall rules); C8 reads only
  `DefaultInboundAction`, which still says Block.
- Fix (smallest): lib.mjs starts geckodriver itself as it does msedgedriver: from the recorded binary with
  `--host=127.0.0.1 --port=<p> --websocket-port=0` (a `geckoDriverArgs(port)` in config.mjs), remote() on
  127.0.0.1:<p>, so WebdriverIO starts nothing (utils build/index.js:766-770, `definesRemoteDriver`). (Alternative:
  `wdio:geckodriverOptions: {binary, host: "127.0.0.1"}`: WebdriverIO keeps a given host, but then connects to
  "localhost": untested on Windows.) Test 4 asserts every driver's argv is loopback-bound (gecko `--host=127.0.0.1`; no
  `0.0.0.0` and no `--allowed-ips` anywhere); a plant (gecko's `--host` dropped) must fail it. C8's start check adds:
  no enabled inbound allow rule whose program is a driver path from browsers.json or node.exe
  (`Get-NetFirewallRule -Direction Inbound -Enabled True -Action Allow | Get-NetFirewallApplicationFilter`), else stop;
  and "answer Cancel to any Windows Firewall dialog during the run" (with loopback binds none appears; one appearing is
  a stop). config.mjs:4-5 and rules.md:52-53 reworded.

### SEC-SB-2 (Medium) RS's first boot chowns a path inside npt's home as root (Q-U8-1 term 4 broken at boot)
- Where: ops/cloud-init/rs.yaml:163 `install -d -o npt -g npt -m 0700 /home/npt/.ssh` (root, after the wrappers and
  sudoers are installed at :143-150).
- What fails: GNU install 9.4 (24.04's coreutils) applies -o/-g/-m to a symlink's target (tested here: `install -d -o
  nobody -m 0700 <link-to-dir>` made the target nobody-owned, 0700). npt can log in during the build (PROMPT-R1.md:39-40
  reads steps.log over SSH while cloud-init runs).
- Exploit sketch: logged in as npt before that step, `mv ~/.ssh ~/.s; ln -s /usr/local/sbin ~/.ssh` -> root makes
  /usr/local/sbin npt-owned -> npt replaces npt-ctl -> `sudo /usr/local/sbin/npt-ctl cpus 3` runs npt's file as root.
  Reach is small (npt is his key, each command his prompt), but it is the one npt-to-root route in the package.
- Fix: rs.yaml:163 -> `runuser -u npt -- install -d -m 0700 /home/npt/.ssh` (cloud-init already made it; the write
  made as the directory's owner, as term 4 words it); test 1 asserts no root install/chown/chmod names a path under a
  user's home.

### SEC-SB-3 (Medium) The operator key is readable with no prompt; "every other command prompts" does not hold
- Where: ops/operator/settings.json:51-58 (denies only `cat`/`less`/`head` and Get-Content/gc/cat/type ending in
  `npt_operator`); no `blockReadsOutsideWorkingDirectories`; ops/operator/rules.md:28-30 ("every other command
  prompts, and his prompt is the wall").
- What fails: Claude Code runs a built-in read-only set (`ls`, `cat`, `head`, `tail`, `grep`, `find`, `diff`, `stat`…)
  "without a permission prompt in every mode", Manual included, unless `permissions.blockReadsOutsideWorkingDirectories`
  is on (code.claude.com docs: permissions "Read-only commands"; permission-modes "Actions no mode auto-approves";
  settings-reference, v2.1.257+). `tail ~/.ssh/npt_operator`, `grep . ~/.ssh/npt_operator` or `cat ~/.ssh/npt_oper*`
  match no deny and do not prompt.
- Exploit sketch: a VM output that steers the session (the channel rules.md:43 names) gets one such read run while he
  is away -> the private operator key lands in the transcript (and the model provider's logs) with no prompt.
  Bounded: fw-rs admits TCP 22 from his home /32 only; rules.md:34-35 rotates a key seen in a transcript, if noticed.
- Fix: settings.json `"blockReadsOutsideWorkingDirectories": true` (outside reads then prompt; the file tools refuse)
  and `Read(~/.ssh/**)` in deny; the operator key gets a passphrase at Stop B and sits in Windows' ssh-agent for the
  run, so a leaked file is ciphertext; rules.md:28-30 names the read-only set; operator test asserts the key.

### SEC-SB-4 (Medium) chromedriver and msedgedriver listen on every interface because `--allowed-ips` is passed
- Where: ops/browser/config.mjs:206 (`allowedIps: ["127.0.0.1"]` -> WebdriverIO emits `--allowed-ips=127.0.0.1`:
  utils build/node.js:1081-1082, parseParams :552-560) and :228 (`--allowed-ips=127.0.0.1` for msedgedriver, which
  lib.mjs:89 starts).
- What fails: chromedriver sets `allow_remote = true` whenever `--allowed-ips` is present and then binds the all-zeros
  address, IPv4 and IPv6; the list is checked per request (HTTP 403) (chromium chrome/test/chromedriver/server/
  chromedriver_server.cc and http_server.cc, ListenOnIPv4 / RequestIsSafeToServe). Without the switch it binds loopback
  and refuses any non-localhost Origin or Host. So F25's "nothing listens beyond 127.0.0.1" does not hold: the request
  filter holds, the sockets and their HTTP parser face the LAN, and each first listen raises SEC-SB-1's dialog.
- Exploit sketch: a LAN device reaches the driver ports (403 per request); one "Allow access" leaves only the
  driver's own filter between the LAN and a WebDriver endpoint.
- Fix: chromedriver `allowedIps: []` (WebdriverIO keeps `[]`, since `[] || ["0.0.0.0"]` is `[]`, and emits no switch)
  or start it in lib.mjs with `--port=<p>` only; edgeDriverArgs -> `["--port=<p>"]`. Test 4 and plant P2 move to "no
  `--allowed-ips`, no `0.0.0.0`". u8-brief Block 9's `allowedIps ["127.0.0.1"]` (from my SEC-R1D-2) is superseded.

### SEC-SB-5 (Medium) Nothing compares the served dist with the expected SHA-256s the brief requires
- Where: u8-brief.md:194 ("the dist's SHA-256s compared with this session's build of the same commit"); ops/cloud-init/
  rs.yaml:211 writes dist.sha256; ops/runs/r1.sh:264 only copies it; ops/runs/R1.md:236-239 fills an expected list
  that no step reads; ops/operator/PROMPT-R1.md §2 (:31-59) has no compare.
- What fails: the page every browser leg runs, built on RS by npt-build from npm and cargo inputs, is served with no
  integrity check against the session's own build.
- Exploit sketch: a compromised build input (a locked crate's build script, a dev dependency of U7's build) alters the
  page JS or adds files to /var/lib/npt/dist; it is served on TCP 31000 under the trusted name and nobody notices.
- Fix: commit the expected list at X (e.g. ops/runs/r1-dist.sha256, filled at Stop B, root-owned in the clone); r1.sh
  bringup diffs it against $B/dist.sha256 (sorted) and stops before any room on a difference; test 5 asserts the step
  and a non-empty list.

### SEC-SB-6 (Low) Root reads the build user's tree through path components the build can make symlinks
- Where: ops/cloud-init/rs.yaml:196-211 (`[ ! -L ]` checks only the last component; `tar -C "$D" -cf -` and `install
  "$T/$b"` as root); ops/cloud-init/lg.yaml:149-153.
- Exploit sketch: a malicious build script (running as npt-build) turns `clone/spikes/netcode-proof/web` into a link
  to /etc/ssh -> root tars the host private keys into the world-readable, served /var/lib/npt/dist.
- Fix: read as the owner: `runuser -u npt-build -- tar -C "$D" -cf - . | tar -C /var/lib/npt/dist …` and
  `runuser -u npt-build -- cat "$T/$b" > /opt/npt/bin/.$b.tmp` then install; the same on LG.

### SEC-SB-7 (Low) The pre-push hook can be skipped by one flag and pins no destination
- Where: ops/operator/settings.json:6-58 (no deny for `--no-verify`, `-c core.hooksPath=…`, or the `refs/heads/` and
  `+` forms of the lane branch and cs/*; only main has `*:refs/heads/main*`); ops/operator/pre-push:1-29.
- Exploit sketch: a prompt reading `git push --no-verify origin claude/npt-results` (or `HEAD:refs/heads/claude/
  netcode-proof-test-plan-hcky5g`) passes a quick glance; redact-check never runs. rules.md:28-29 records that denies
  are best-effort, so Low.
- Fix: deny `git push --no-verify*`, `git push * --no-verify*`, `git -c *`, `git * core.hooksPath*` (both shells) and
  the `refs/heads/` forms; the hook reads its stdin ref lines first and refuses any remote ref but
  `refs/heads/claude/npt-results`, then passes them to redact-check; start check 7 also reads `git config --get
  core.hooksPath` (must be empty).

### SEC-SB-8 (Low) F13's login check is a deny pattern and covers RS only
- Where: ops/runs/r1.sh:286-288 (`id -nG`; `sudo -n -l` grepped for `(ALL…) ALL` only); LG has no such read
  (PROMPT-R1.md §2 checks LG's user data and HEAD only; LG's build.done is never read).
- Fix: r1.sh compares `sudo -n -l`'s command lines with exactly sudoers.d/npt's nine (else stop); PROMPT-R1.md §2 runs
  `id -nG`, `sudo -n -l` (against npt-lg's three lines) and LG's steps.log `build done` on LG.

### SEC-SB-9 (Low) The page host's unit has no sandbox beyond NoNewPrivileges
- Where: ops/systemd/npt-host.service:7-13. The one service facing outside traffic runs as the owner of
  /var/lib/npt/tls (write).
- Fix: add `ProtectSystem=strict`, `ProtectHome=yes`, `PrivateTmp=yes`, `PrivateDevices=yes`, `CapabilityBoundingSet=`,
  `RestrictAddressFamilies=AF_INET AF_INET6 AF_UNIX`, `RestrictNamespaces=yes`, `LockPersonality=yes`,
  `MemoryDenyWriteExecute=yes`, `SystemCallArchitectures=native`, `SystemCallFilter=@system-service` (serve only
  reads); test 1 asserts; bring-up records `systemd-analyze security npt-host`.

### SEC-SB-10 (Low) Rooms and the host reach the internet through the loopback DNS stub
- Where: ops/cloud-init/rs.yaml:57 `oifname "lo" accept` precedes :59's `meta skuid {2001,2002} ct state new drop`;
  systemd-resolved on 127.0.0.53 forwards as its own user. u8-brief.md:187-188 says room and host egress is dropped.
- Exploit sketch: code running as npt-room (a room bug) leaks data in DNS queries.
- Fix: `meta skuid 2001 ip daddr 127.0.0.53 drop` (and `meta skuid 2001 th dport 53 drop`) before the loopback accept;
  npt-host keeps DNS for the order; test 1's `nft -c` covers it.

### SEC-SB-11 (Low) Token link files: the prompt's rule and the teardown disagree with the scripts
- Where: ops/operator/PROMPT-R1.md:7 ("Never print … a room token … into a file"); ops/browser/lib.mjs:184-188
  (braveLinks writes token URLs into `.gate\npt-run-links\`, the brief's choice 6); PROMPT-R1.md:103 deletes only the
  USB copy.
- Fix: PROMPT-R1.md:7 names the link files as the one place the scripts write a token; §6.3 deletes
  `.gate\npt-run-links\` too. (Tokens die with the rooms, so Low.)

## Q-U8-1's verb and the six Stop B confirmations (u8-brief.md:121-136 = advisor-u8-brief.md:53-68)
1. HOLDS. ops/sbin/sudoers.d/npt:8 `^rooms (start|tokens) (0[0-9]|[12][0-9]|3[01])$`; every line anchored or `""`
   (npt:6-14; npt-lg:5-7, L5); no `*`; `cvtsudoers -f json` here parses each line as one command (the `{1,4}` comma at
   npt:6 stays inside the regex); visudo -c at rs.yaml:148, :150, lg.yaml:119, :121 and test 3
   (ops/tests/wrappers.test.sh:171-186). Sudo matches the space-joined argv, so each wrapper re-counts its arguments
   (npt-ctl:32, :63, :141, :168, :181; npt-cert:30; npt-capture:22; npt-userdata:13-16; npt-s14:23).
2. HOLDS IN SUBSTANCE. ops/sbin/npt-ctl:161-162: NN validated (:141), the housekeeping set from root-owned
   $STATE/cpus (:131-137, `! -L`, /var/lib/npt/state root 0755: rs.yaml:153), fixed binary paths. Since fix items 8
   and 10 the line also carries values from the npt-written settings as npt-super's flags (npt-lib.sh:205-223, each by
   an anchored pattern, read as npt-room: npt-ctl:152-153), after npt-super; systemd-run, setpriv and taskset stop
   option parsing at their first operand (checked here: util-linux 2.39.3 taskset and setpriv passed `--help`/`-d`
   after the command through; systemd-run 255 did not take `--help` after the command). So no caller text becomes an
   option of the three tools; the term's literal "NN and root-owned values only" is now "validated values".
3. HOLDS. npt-ctl:23-24 `setpriv --reuid=npt-room --regid=npt-room --clear-groups --no-new-privs --bounding-set=-all
   --inh-caps=-all --ambient-caps=-all --reset-env`; npt-room: nologin, locked, no home (rs.yaml:23), named by no
   sudoers line, /opt/npt/bin and /opt/npt/clone root-owned 0755 (rs.yaml:108, :136-139, :199, :203).
4. HOLDS FOR EVERY WRAPPER; FAILS IN FIRST BOOT. tokens read as npt-room (npt-ctl:146); restart frees token and hash
   paths as npt-room (:156); settings read as npt-room (:152-153); npt-capture's file written by tcpdump after -Z into
   tcpdump's directory (npt-capture:36; rs.yaml:160; tcpdump(1) -Z), root's own files in /run (:16-18); npt-s14 writes
   only a new root-owned folder (npt-s14:28-31); npt-cert's four files written by npt-host in its own directory
   (npt-cert:61-65; rs.yaml:158). Nit: npt-ctl:150's `[ -f ]` stats (follows, opens nothing) a path in npt's directory.
   Not wrappers but root on the same VM: rs.yaml:163 (SEC-SB-2) and rs.yaml:196-211 (SEC-SB-6).
5. HOLDS. /var/lib/npt/run npt:npt-room 2750 (rs.yaml:154): npt writes, npt-room reads; /var/lib/npt/rooms and /hash
   npt-room 0755 (rs.yaml:155): npt-room writes, npt cannot (token 0600 by U6a); parent /var/lib/npt root 0755 (:111).
6. HOLDS. npt-ctl:161 `exec systemd-run --scope --collect`: the scope's process is npt-super after the execs; r1.sh:195-
   203 closes its stdin by killing the holder; crates/super/src/main.rs:279-294 closes the room's stdin, then kills at
   the deadline, and a dead npt-super closes the room's stdin pipe. (A `browsers` room stays up until its
   `browsers-down` by design.)

## Other confirmations asked
- C2's substitute for my npm rule: CONFIRMED. No `.npmrc` (git ls-files on M: none; none under W/spikes); both forms
  at ops/runs/R1.md:62-63 and ops/operator/rules.md:47-49, the VM's at ops/cloud-init/rs.yaml:190-191; npx / npm exec
  / npm create appear only as prohibitions (R1.md:65, rules.md:33; test 1 cloudinit.test.mjs:143); specs as
  `node <script>.mjs` (PROMPT-R1.md:82-94); exactly the 4 moderates (R1.md:63-64); no @wdio/cli (package.json:6-8).
- SEC-R1D-3: HOLDS. Browsers and drivers by path (config.mjs:205-218), driver SHA-256 checked before each start
  (config.mjs:191-196, lib.mjs:84); WebdriverIO skips every download when binaries are given (utils node.js:745-753,
  :1079, :1105-1107; geckodriver dist/index.js:9-12).
- SEC-R1D-2: HOLDS AT THE REQUEST LEVEL FOR CHROME AND EDGE ONLY; NOT for Firefox (SEC-SB-1); sockets on every
  interface for all three (SEC-SB-1, -4).
- C8: present and read-only before any driver (PROMPT-R1.md:23-25; R1.md:66-67; rules.md:55-56) but blind to
  per-program allow rules (SEC-SB-1's fix).
- F25: WebdriverIO starts no second msedgedriver (utils build/index.js:766-770): holds. "Nothing else starts a driver
  listening beyond 127.0.0.1": does not hold (SEC-SB-1, -4).
- F13: the mitigation is right (rs.yaml:121-123, lg.yaml:98-100 run before any wrapper; cloud-init merges user-data's
  default_user over the image's, so `groups` comes from the image); the check is partial (SEC-SB-8).
- F16: HOLDS. ops/sbin/npt-load-ctl:78-84: only rc 124 (timeout) reads blocked; refused or connected reads OPEN, and
  LG's own nftables cannot fake a timeout (a local drop gives EPERM at once -> OPEN, fail-safe).
- F22 (fix item 1): HOLDS. npt-host.path/.service (ops/systemd/*; rs.yaml:218-222), User/Group npt-host,
  NoNewPrivileges; no sudoers line names npt-host or systemctl (sudoers.d/npt; test wrappers.test.sh:185-186); only
  npt-host can create the trigger file (rs.yaml:158). Hardening: SEC-SB-9.
- F23: HOLDS. settings.json:60-69, eight exact strings, no wildcard.
- F6: HOLDS. rs.yaml:54-59 uids 2001/2002 = users at :23-24; lg.yaml:43-44 = :22.
- F7: HOLDS. sudo 1.9.15p5 here; cvtsudoers parse as above; dots as `[.]`.
- F10: HOLDS. tcpdump(1): -Z changes user "after opening the capture device … but before opening any savefiles for
  output"; npt-capture:36 fixed line; no -z/-F/-r/-w from the caller.
- F8: HOLDS (rs.yaml:117, lg.yaml:94: anonymous HTTPS clone of a public repo; no credential on any VM). Exposure: below.
- Q-U6C-6's order: HOLDS. Console fingerprint before the first SSH (PROMPT-R1.md:35-38); the HTTPS host-key line after
  the production certificate, after `r1.sh host` checks npt-host is active (PROMPT-R1.md:71-74; r1.sh:575, :77-81),
  before the server chain (§4) and every browser room (§5).
- redact-check as the hook pins it: ops/operator/pre-push:11 EXPECTED 2609bc87c9248558f8334731a07b0ce82479ced427f18d70af1ee7e130519d31
  = sha256 of W's ops/operator/redact-check.mjs = `git show 2a9498d:…/redact-check.mjs` (2a9498d "redact-check: npm
  tarball paths on registry.npmjs.org are no host names (F26)"). The hook refuses: a missing copy (:13-16), a hash
  mismatch (:17-21), any finding or a git/usage error (:26-29: addresses outside the VM list in every form, this
  test's tokens, common secrets, private-key headers, canaries, binary files; new branches scanned against
  `--not --remotes`: redact-check.mjs:321-342). It does not refuse a `--no-verify` push, a push from a clone without
  the hook, or a push to another ref (SEC-SB-7).

## GTFOBins: who reaches what as root (no caller-chosen argument anywhere)
| Binary | Lines | Arguments | Verdict |
|---|---|---|---|
| systemd-run | npt-ctl:161 | fixed options; NN; unit name from NN and $$; command after the first operand | not reachable |
| setpriv | npt-ctl:23-24, :146, :152, :156, :162; npt-cert:63-64; npt-s14:19-20, :64, :68; npt-userdata:18-19 | fixed | not reachable |
| taskset | npt-ctl:87, :96, :162 | the root-owned housekeeping set; tids from /proc stat (PF_KTHREAD) | not reachable |
| tcpdump | npt-capture:36 | fixed interface, filter, snap, count, -Z, output path | not reachable |
| nft | npt-ctl:36-37, :58, :170-175; npt-cert:59, :51, :119 | fixed rules; elements only from validated IPv4s (peers) or DNS answers validated as public IPv4 (acme_v4) | not reachable |
| dig | npt-cert:56, :97 | fixed names; servers from root-owned npt-ns.txt; HOME unset so no .digrc (BIND 9.18 dig.c) | not reachable |
| systemctl, sshd, git, ip, tc, python3 -I | npt-ctl:44-58, :76-81; npt-userdata:17; npt-s14:39-50, :72 | fixed; --no-pager; env -i | not reachable |
Every wrapper re-execs under `env -i … LC_ALL=C bash -p` (npt-ctl:14-16 and the same block in each), `set -f`.

## Coverage table (no game rules in U8: the surfaces it configures)
| Surface | Where it is checked | Gap |
|---|---|---|
| A client joining a room | the room's token (U6a), fw-rs /32s (C5 proves in force: r1.sh:290-302) | none in U8 |
| A request to the page host (TCP 31000) | U6c's host (its round); fw-rs; unit sandbox | SEC-SB-9 |
| RS to LG control | `restrict,command=` + from= (npt-ctl:187); npt_load_parse (npt-lib.sh:149-161) | none |
| The npt login's sudo | anchored sudoers + wrapper re-checks | SEC-SB-2 (boot), SEC-SB-8 |
| WebDriver endpoints on the laptop | driver flags; Windows firewall (C8) | SEC-SB-1, SEC-SB-4 |
| The laptop session's reads and pushes | Manual mode, denies, the pre-push hook | SEC-SB-3, SEC-SB-7 |
| Served page integrity | none today | SEC-SB-5, SEC-SB-6 |

## Section 7's lines for my seat
- **Cost per run:** none from this seat; the fixes are script and settings lines (R1's cap EUR 0.80 unchanged).
- **What the package exposes:** (1) Public: the repo is public (F8), so the cloud-init, wrappers, R1.md's day and hours
  (Thu 3 Dec, 09:30-13:30 Amsterdam), the name rs.nt.shamblefrontlines.com (also in CT logs) and the ports are public;
  no secret and no address of the day are in any file (tests 1 and 6). (2) During the run: RS's TCP 22, UDP
  30001-30032, TCP 31000-31032 and 9100 to his home /32 and LG only through fw-rs, which is the only /32 wall for the
  game ports (nftables accepts them from any source, Q-U8-2) and C5 proves it in force before any room; LG's TCP 22 to
  RS only, ICMP to his home only. (3) The laptop: today three WebDriver endpoints on every interface during the browser
  legs (SEC-SB-1, -4); after the fixes, loopback only. (4) Secrets in play: the operator key (laptop; SEC-SB-3), the
  RS-to-LG key (RS, npt 0600), the production TLS key (RS, 0640 npt-host:npt-room, shortlived), room tokens (RS 0600
  npt-room; LG 0600 npt-load; Brave's link files, SEC-SB-11), the ACME account key (memory of one process). His home
  address is typed only in the console; it reaches RS's kernel and sshd's journal lines, and a pcap only if
  `npt-capture` runs (no R1 block runs it); collect (r1.sh:534-540) takes none of these.
- **If a provider is down:** every path fails closed. Hetzner: no VM, nothing exposed; the kept Primary IPs stay his,
  so the A record never points at someone else's server. Cloudflare or Let's Encrypt: npt-cert sends no "go" until
  every authoritative server shows the value, stops at 30 minutes, and its trap flushes npt-host's egress
  (npt-cert:49-53, :94-103); no production chain means the page host never starts (npt-host.path). GitHub: the clone
  fails before any wrapper or sudoers file is installed, after cloud-init's sudo rule and extra groups are removed
  (rs.yaml:121-150), so npt has no sudo at all; bring-up refuses build.failed (r1.sh:263). crates.io, rust-lang.org,
  nodejs.org or npm: build.failed; wrappers exist but no binaries; nothing listens but sshd. Driver vendors: no driver
  starts unpinned (config.mjs:191-196).
- **Rollback:** revert U8's merge commit on the branch (it adds only spikes/netcode-proof/ops/**, 42 files; no tracked
  file changes). Then: VMs deleted (LG first, then RS); on the laptop ~/npt-operator/ (settings.json, redact-check.mjs,
  vms.txt, browsers.json), the clone's .git/hooks/pre-push, .gate/npt-* folders and the USB link file removed; the
  operator key removed from the Hetzner project. fw-rs, fw-lg, the kept Primary IPs and the A record are console
  objects kept for R2-R5; at the close, delete the A record before releasing the Primary IPs (a released IP behind a
  live name is a takeover risk).

## Sources (each read 2026-11-23; summaries in the notes below)
- WebdriverIO tarballs from registry.npmjs.org: @wdio/utils 10.2.0, webdriver 10.2.0, webdriverio 10.2.0, geckodriver
  8.0.0 (sha512 = ops/browser/package-lock.json's integrity). High confidence (code read).
- Chromium source: https://raw.githubusercontent.com/chromium/chromium/main/chrome/test/chromedriver/server/chromedriver_server.cc
  and …/http_server.cc (main, not the laptop's exact version; msedgedriver is Microsoft's Chromium build: medium-high).
- geckodriver flags: https://firefox-source-docs.mozilla.org/testing/geckodriver/Flags.html (high).
- Windows Firewall rules: https://learn.microsoft.com/en-us/windows/security/operating-system-security/network-security/windows-firewall/rules (high).
- Claude Code: https://code.claude.com/docs/en/permissions, …/permission-modes, …/settings-reference (high).
- BIND 9.18 dig: https://raw.githubusercontent.com/isc-projects/bind9/v9.18.28/bin/dig/dig.c (high for 9.18.x).
- tcpdump(1): https://www.tcpdump.org/manpages/tcpdump.1.html (high).
- Local: GNU install 9.4 symlink test, util-linux 2.39.3 and systemd 255 option parsing, cvtsudoers 1.9.15p5, bash
  `env -i` HOME (container, Ubuntu 24.04.5).

## Notes (one line per finding as found)
- N1 sudoers parse (cvtsudoers) OK. N2 wrappers re-exec env -i. N3 dig: no .digrc without HOME (dig.c `if (homedir !=
  NULL && digrc)`). N4 F25 no second msedgedriver. N5 chromedriver `--allowed-ips` -> all-zeros bind. N6 geckodriver npm
  8.0.0 default host 0.0.0.0, Host-header-only checks -> SEC-SB-1. N7 install -d follows links -> SEC-SB-2. N8 root
  reads npt-build's tree (SEC-SB-6); no dist compare (SEC-SB-5). N9 read-only commands unprompted (SEC-SB-3). N10 hook
  bypass (SEC-SB-7). N11 F13 check partial (SEC-SB-8). N12 unit sandbox (SEC-SB-9). N13 Q-U6C-6 order holds. N14
  EXPECTED = 2a9498d. N15 option parsing stops at the command. N16 "LG speaks first" (u6c-brief.md:67) fits rs.yaml:59
  (rooms open no new flow). N17 loopback DNS stub (SEC-SB-10). N18 token link files (SEC-SB-11).

## For my memory (not written: this task allows one file)
- WebdriverIO 10.2.0 + geckodriver npm 8.0.0 start geckodriver on 0.0.0.0 (Host-header check only); chromedriver /
  msedgedriver bind all-zeros whenever `--allowed-ips` is given. Rule: start every driver loopback-bound (no
  `--allowed-ips`, gecko `--host=127.0.0.1`); C8 reads per-program inbound allow rules too.
- Claude Code's read-only Bash set runs unprompted in Manual mode unless blockReadsOutsideWorkingDirectories is on.
- GNU install -d -o/-m follows a symlinked final component: root never runs it in a user-owned directory.

DONE

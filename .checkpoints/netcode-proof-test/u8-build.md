# U8 build notes (builder, isolated worktree; CHECKPOINTS.md §4)

**Task:** build U8, the run package without Safari, per u8-brief.md rev 1.1 (sha256 3d102be9…f8f2, checked: match).
Files only under spikes/netcode-proof/ops/ as the brief names them. No commit, no push, no cargo, no package file change.
`<base>` = d854ae1375ab48918b78f2a7ddb18b7764fd67a3.

## Start
- `git merge --ff-only d854ae1375ab48918b78f2a7ddb18b7764fd67a3`: fast-forwarded (exit 0); no `git switch -c` needed.
- `git log -1 --format=%H` → d854ae1375ab48918b78f2a7ddb18b7764fd67a3 (matches `<base>`).
- Worktree branch: worktree-agent-abf71d95fc07b3522.

## Blocks (plan; each finished block written here at once)
0. Read the cited sources (plan.md sections, security docs, U6a/U6b/U4f briefs, results/u3c-kernel-check).
1. cloud-init rs.yaml, lg.yaml + test 1
2. firewall lists + test 2
3. wrappers, npt-lib.sh, sudoers + test 3 (wrappers.test.sh)
4. S14's rig probe.py, compare.py (+ fixture KCMODEL test)
5. r1.sh
6. R1.md + test 5
7. operator files settings.json, rules.md, PROMPT-R1.md
8. pre-push hook
9. browser scripts (config.mjs, lib.mjs, smoke.mjs, fallback.mjs, cert14d.mjs, mds36.mjs) + test 4; `npm ci --ignore-scripts`
10. operations checklist
11. test 6 (redact over spike folder), plants P1-P7

## Findings (one line each)
- F1 npm files at `<base>` re-hashed: package.json 8370495d…3c90, package-lock.json a0efce8e…9524 (= DEP-HOLD's).
- F2 No U6c, U7 brief exists yet (ls .checkpoints/netcode-proof-test): npt-load's interface, the issue mode's
  interface, the probe's ports, the page's URL contract and records are U8 PROPOSALS here, each in one home, for those
  briefs to take or amend (u8-brief §10: "U6c's takes ...; U7's brief fixes the URL contract"). Open item, not a stop.
- F3 Container: PyYAML under `python3 -I` reads 6.0.1 (brief §3 says 6.0.3); Python 3.13.18; no ip/tc/dig/sshd.
- F4 redact-check's allow-list has no RFC 1918 range: S14's veth uses 198.51.100.1/.2 (RFC 5737, allowed).
- F5 Pins read 2026-11-22 from the vendors and checked against the downloaded bytes (sha256sum of the stream):
  rustup-init 1.29.3 x86_64 b1c8cb14e30889e7e2999e6b69e62d9e9b9ffc0f4df8180564d306d9680e6c55, aarch64
  9f608c142e0e38a78957d0661ba1dfaf0b6fb37184cfaa4b9a0aa57af6184282 (static.rust-lang.org/rustup/archive/1.29.3/);
  Node v22.23.4 linux-x64 1805bcd35e6815559bd2eb97db438d3fdb0eb3a5e6a977b7c1badca76337e9ce (nodejs.org SHASUMS256).
- F6 bootcmd runs before users_groups, so nft rules name the room and host users by fixed uid (2001 npt-room,
  2002 npt-host), never by name (a name would fail the whole ruleset at boot). nft -c passes that form.
- F7 visudo 1.9.15p5 parses anchored `^…$` argument regexes; dots written `[.]` (no backslash ambiguity).
- F8 Repo is public (gh api: visibility public): VMs clone and fetch raw user data anonymously.
- F9 C12 (f) has no tolerance in writing (plan.md:1256 "reads ... against netem's 100 ms"): compare.py reports
  tcpi_rtt − 100 ms and marks a kernel run INVALID only outside 99-120 ms (netem not in place). For Stop B's netcode seat.
- F10 tcpdump drops to its own user before opening the savefile (-Z): the capture dir is owned by `tcpdump`, the file
  named *.pcap (Ubuntu's AppArmor profile admits that name), so root never opens a path another user writes.
- F11 `node --test ops/` fails on Node 22.23.4 (the folder is loaded as a module); a quoted glob works
  (`node --test 'ops/**/*.test.mjs'`). The main session's `ops` step in check.mjs needs the glob form (brief §3 wording).
- F12 plan.md §1.6 step 1 (RS's host key fetched over the production certificate before the first SSH) cannot hold
  under the re-plan's DNS-01 by hand: `npt-cert issue` itself needs an SSH session. PROMPT-R1.md uses §1.6's fallback
  (console fingerprint, him present) for the first SSH and the HTTPS-published line after the production certificate as
  the cross-check. Stale line for the list: plan.md:287. To the advisor at Stop B.
- F13 cloud-init's `system_info.default_user` merge with the image's default (groups, sudo) is not verifiable here:
  firstboot removes /etc/sudoers.d/90-cloud-init-users and every group of npt but its own; `r1.sh bringup` checks
  `id -nG` and `sudo -n -l` (only the wrappers). For Stop B's infra seat.
- F14 U7's dist build command is not fixed (no U7 brief): rs.yaml proposes `npm ci --ignore-scripts` then
  `node build.mjs` in spikes/netcode-proof/web, writing web/dist; U7's brief takes it or the main session edits that one
  line at Stop B.
- F15 Interfaces U8 proposes for other units' code (none exists yet; each in one home): npt-super's hold form as
  `npt-super hold <settings path>` (npt-ctl rooms start; U6a builds the hold form now); the settings name the token file
  /var/lib/npt/rooms/room-NN.tok and the hash file /var/lib/npt/hash/room-NN.json (the wrapper frees both as npt-room
  before a restart: u6b-brief.md cond 4); U6c's issue mode `npt-host issue staging|production` with the PROFILE /
  TXT_NAME / TXT_VALUE / go / SERIAL / SHA256 / DONE lines (npt-cert's header); npt-load as `npt-load <settings path>`
  and npt-load-ctl's verbs. Each is checked against the merged code at Stop B; a difference is a one-line edit.
- F16 fwcheck (C5, LG half): only a TIMEOUT reads as blocked; a refused connect means the SYN reached RS, so it reads
  OPEN too (nftables on RS accepts 22 from any source, so only fw-rs can drop it silently).

## Block log
- Block 0 DONE: sources read (brief whole; plan.md §7.2-7.3, §8, §10.5-10.6, §12.2-12.3, :1256; replan.md;
  replan-units.md; security-r1-deps.md B5-B7; security-u8-deps.md; advisor-u6a-brief.md cond 2-4 and Q-U6A-5;
  u6a-brief.md; u6b-brief.md rev 1.1 cond 3-4 and §10; u4f-brief.md §7; advisor-u8-brief.md; marks.json -1, -2, -3,
  -14, -16, -29, -36, PM-0006-11, -12, -18, -25; results/u3c-kernel-check/). Added lines: 0.
- Block 1 DONE: ops/cloud-init/rs.yaml, lg.yaml; ops/tests/helpers.mjs, cloudinit.test.mjs. Test 1: 18 pass, 0 fail
  (`node --test ops/tests/cloudinit.test.mjs`). `git diff --stat <base>`: no tracked change; untracked U8 files 570
  lines (wc -l, node_modules excluded).
- Block 2 DONE: ops/firewall/rs.txt, lg.txt; ops/tests/firewall.test.mjs (test 2: 5 pass). Added lines: 600.
- Block 3 DONE: ops/sbin/npt-lib.sh, npt-ctl (peers, cpus, rooms start|tokens, quic, loadkey), npt-cert, npt-ns.txt,
  npt-capture, npt-userdata, npt-s14, npt-load-ctl, sudoers.d/npt, sudoers.d/npt-lg; ops/tests/wrappers.test.sh
  (test 3: 201 ok, 0 failed); test 1 gained the wrappers' nft texts beside bootcmd's (tests 1+2: 24 pass).
  The `npt-tls` group dropped: the TLS key's group is npt-room (setgid dir owned by npt-host) because setpriv clears
  supplementary groups (Q-U8-1 term 3). Added lines: 1,565 (git diff --stat: no tracked change).
- Block 4 DONE: ops/runs/s14/probe.py (messages, schedule, server, client, bursts), compare.py (schedule, env, summary,
  judge); ops/tests/s14.test.mjs (8 pass: shared schedule in the knob's form; PASS with recovery exactly 1.10 s
  kernel / 1.05 s model by (e)'s definition; model without bursts -> burst half NOT-RUN, verdict PARTIAL; kernel
  recovery +24 % -> MISS "netem wins"; stall +15 % -> stall MISS; tcpi_rtt 80 ms -> INVALID; no retransmission ->
  VOID; no model line -> INVALID; bursts mode refuses a stale stamp, no tc). Added lines: 2,169.
- F17 S14 choices fixed in writing in compare.py's header, for Stop B's netcode seat (C9): stalls over the steady
  window only (messages written before 59 s); BASE over every message; bursts on BOTH directions at send time (netem
  drops at enqueue), which MOVED's rider must model the same way; 3 runs x 170 s (5,100 messages); burst slack 20 ms;
  model recoveries fall back to BASE only when neither `delay_ns` nor a `late` entry exists (counted, reported).
- Block 5 DONE: ops/runs/r1.sh (bringup with C-U8-1, taskset, netem, login checks, C5 both halves, npt-ctl cpus,
  `alone`; cert-* start and -wait; headers, probe with probe_presence, calibrate, starts x20, ds-selftest, s14, bites;
  browsers/browsers-down per slot-table room; tokens; collect with npt-judge; chain server|bites, chain-run,
  chain-status). rs.yaml gained the bites build (npt-server-bites) and npt-judge on RS. Test 3 gained r1.sh checks
  and a no-bare-glob check: 214 ok. Test 1: 19 pass. Added lines: 2,689.
- F18 BUG found and fixed in review (before any test caught it): `set -f` in npt-ctl disabled its /proc globs (cpus'
  kthread scan and the L4 fallback would have read nothing). Fixed by expanding globs into arrays inside `set +f`;
  test 3 now fails any `for … in <bare glob>` in a set -f wrapper. r1.sh (npt's own script) drops set -f.
- F19 C5's RS half: a ping that cannot run (no ICMP permission) is not "no reply": r1.sh requires " 0 received" in
  ping's output, else STOP ("fw-lg unproven").
- F20 R1's bites at R1: U6a's three room bites (PM-0001-1/shared-vcpu, PM-0006-11/no-smt-map, PM-0001-16/double-load)
  from npt-server's bites build (rs.yaml builds it). The other R1 marks' bites (-2 fire-stops, -3's five, -14
  unsplit-group, -36 read-once, -52 predicted-damage) are not placed by U8: R1.md lists them for the advisor.
- F21 The browser rooms run unpinned (settings with no tick vCPU pin nothing and write no affinity event: Q-U6A-5),
  one at a time; only -1's rooms (headers, the bring-up `alone` room, the bites) pin the tick vCPU.
- Block 6 DONE: ops/runs/R1.md (blocks and hours with He-is and Started-by columns, bring-up split 0.60 cloud-init /
  0.30 present; Stop B once-list with C7, C8, C2's both forms, npm audit's 4 moderates; timeline from T0 09:30; his
  steps; sittings 22 + 33 + 106 = 161 min, about 2 h 40 min; cap and per-VM delete-by (L1); slot table 133 slots
  (C3), condition 2's line word for word and R1's line; S14, DelayStream (Q-U8-6 (ii)), ImpairSocket, C-U8-1..-6,
  conditions 3-4, C5; C6's dates and three lines; Stop B fills, interfaces to align, R1 bites, C9's round, the
  superseded plan.md lines (L3 + :287)). ops/tests/r1plan.test.mjs (test 5: 7 pass).
- Block 7 DONE: ops/operator/settings.json (ask on push both forms; denies both forms; 8 fixed-string HTTPS allows, no
  wildcard; no ssh allow, C1), rules.md, PROMPT-R1.md. npt-ctl peers now prints `sshd -T`'s plan keys and `nft list
  ruleset` after its reload (bring-up's dry runs; both need root; no new verb).
- Block 8 DONE: ops/operator/pre-push (EXPECTED = sha256 of redact-check.mjs at `<base>`, 941c6925…8af2; --vms from
  ~/npt-operator/vms.txt when present). ops/tests/operator.test.mjs: 8 pass (incl. the hook on a throwaway repo:
  missing copy refused, clean push passes, an address refused, a tampered copy refused). Added lines: 3,498.
- Block 9 DONE (resumed after the 15:47 spend-limit cut; the one install had finished before it, F24, and was not
  repeated): ops/browser/config.mjs (URL contract, slots, capabilities, machine record from
  ~/npt-operator/browsers.json with each driver's SHA-256 checked), lib.mjs (sessions; Edge's driver started by us,
  F25; tokens and quic over SSH with fixed arguments; records under .gate/), smoke.mjs, cert14d.mjs (C3), fallback.mjs
  (C3: quic off per browser, WebTransport arrival -> INVALID, quic on at the end), mds36.mjs (quic on first, 3 at once,
  >= 61 reads). ops/tests/browser.test.mjs (test 4: 8 pass). Added lines: 4,126 (the npm lock is `<base>`'s, not
  counted; node_modules gitignored).
- Block 10 DONE: ops/checklists/operations.md (PM-0006-12's ping, PM-0006-18's costs against R1-R5's caps).
- Block 11 DONE: test 6 ops/tests/redact.test.mjs (1 pass). The whole-folder scan shows ONE finding, already present at
  `<base>`: ops/browser/package-lock.json:606 (`resolved` of crc-32's 1.2.2 tarball read as an rDNS-embedded address),
  in the DEP-HOLD lock U8 may not change; so check.mjs's `redact` step is red at `<base>` (F26). Test 6 names it
  exactly; any other finding fails. wrappers.test.sh's refused-address fixtures now built at run time.
- Plants (each by Edit, the named test run, restored by Edit, sha256 equal to the pre-plant value):
  P1 TCP 80 rule in firewall/rs.txt -> test 2 FAIL "each rule equals Block 2's table", "no TCP 80, no any-source,
  placeholders only"; restored 4cd8bf5b…1fab. P2 `allowedIps: ["0.0.0.0"]` (Edge) in browser/config.mjs -> test 4
  FAIL "SEC-R1D-2: chromedriver and msedgedriver admit 127.0.0.1 only…"; restored 14894a58…bd8f2. P3 sudoers line
  `/usr/bin/systemd-run ^--scope$` -> test 3 FAIL "'/usr/bin/systemd-run' is a wrapper", "never systemd-run, setpriv,
  …"; restored 4ff44165…ac92. P4 a public-looking IPv4 in runs/R1.md -> test 6 FAIL (R1.md:232 IPv4 address);
  restored 1c14ed61…f502. P5 `((n >= 1))` in npt_affinity_scan -> test 3 FAIL "per-CPU kernel thread on the tick
  vCPU: left", "…on another vCPU: not listed", "no per-CPU kernel thread is ever moved"; restored d0452102…e53e.
  P6 -36's hours 0.50 -> 1.00 in R1.md -> test 5 FAIL "the block hours sum … within the 4 billed hours" (and the
  timeline); restored 1c14ed61…f502. P7 `${SSH_ORIGINAL_COMMAND#start }` passed to npt-load -> test 3 FAIL
  "npt-load-ctl reads SSH_ORIGINAL_COMMAND only …", "npt-load's command line is fixed"; restored aa459bcb…dc35.
- END: `node --test 'ops/**/*.test.mjs'` 66 pass, 0 fail (56 U8 + redact-check's own 10); `bash
  ops/tests/wrappers.test.sh` 214 ok, 0 failed. `git diff --stat <base>`: no tracked change; 38 new files under
  spikes/netcode-proof/ops/, 4,176 lines (the npm lock is `<base>`'s; node_modules gitignored). rs.yaml 10,988 bytes,
  lg.yaml 6,958 (< 32,768). SIZE-CHECK not reached; over the brief's likely 2,900-3,250 by the tests (about 840) and
  r1.sh (490 against ~220: the chain, the room manager, the presence and C5 checks).
- F26 For the main session: check.mjs's `redact` step fails at `<base>` on the committed lock (above). Not U8's lane:
  options are redact-check's host-name rule skipping a `resolved` registry tarball name, or an allow in the step.
- F27 New files are mode 0644 (Write); the VMs install with explicit modes, r1.sh runs via bash, git hooks on Windows
  need no exec bit. If the hook should carry +x in git, the main session sets it at the snapshot.
- F22 STOP-ITEM (gap, not built): no wrapper or unit in the brief starts U6c's page host on RS (npt-host serving the
  dist, /npt/cert-hash.json and ssh-host-keys.txt on TCP 31000 once the production chain exists). The browser legs and
  §1.6's HTTPS cross-check need it. Options for the advisor: (a) an `npt-page.path` + `npt-page.service` pair in
  rs.yaml (User=npt-host, started by systemd when /var/lib/npt/tls/chain.pem appears; no sudoers line), or (b) a fixed
  `npt-ctl page start|stop` verb (one more anchored sudoers line). U8 built neither: it would add a unit or a sudo line
  the brief does not name. Also open for U6c/U7: which room's hash file /npt/cert-hash.json serves (browser rooms run
  one at a time, NN 10-19).
- F24 `npm_config_ignore_scripts=true npm ci --ignore-scripts` in ops/browser: installed; package.json and the lock
  re-hashed after it, unchanged (8370495d…, a0efce8e…); npm reported exactly "4 moderate severity vulnerabilities"
  (security-u8-deps.md C3 (c)'s expected chain). node_modules is gitignored.
- F25 SECURITY (for security-auditor at Stop B): in webdriverio 10.2.0, @wdio/utils build/node.js:1127 starts
  msedgedriver with `{ ...edgedriverOptions, cacheDir, port, allowedIps: ["0.0.0.0"] }`: the spread puts the forced
  value last, so SEC-R1D-2's `wdio:edgedriverOptions.allowedIps ["127.0.0.1"]` is overridden. Chrome keeps the caller's
  values (:1080-1081); geckodriver gets allowHosts localhost (:1118); a given browser binary skips any download
  (:750-757). U8 keeps the brief's capabilities for all three AND starts msedgedriver itself in lib.mjs from config's
  binary and allowedIps (`--port=<p> --allowed-ips=127.0.0.1`), connecting remote() to 127.0.0.1:<p>, so WebdriverIO
  never starts Edge's driver. A deviation from Block 9's mechanism, made to keep SEC-R1D-2; not a guess (code read).
  Also: chromedriver receives `--binary=…` from `wdio:chromedriverOptions.binary` (EXCLUDED_PARAMS :529 holds only
  version, help, spawnOpts), WebdriverIO's documented form; cacheDir is set on remote()'s options, not the driver's.
- F23 The operator allows are fixed strings: a trailing wildcard on a curl allow would admit `-T <file> <other host>`
  (an upload anywhere). plan.md:1320's "allows the HTTPS calls" is met by the exact ledger and DoH reads.

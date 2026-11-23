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

## Fix pass (2026-11-23)
Builder: own worktree .claude/worktrees/agent-ac59cce3e5d002441, fast-forwarded (`git merge --ff-only 096808b`) from
f5030a3 to the lane head 096808b (U6a 824d8e7, F26 2a9498d inside); patch sha256 e9fd605c…5534 checked; `git apply
--check` clean, applied. No cargo, no npm, no commit. These notes live in the worktree's copy of this file (the tool
refuses the shared checkout's path from an isolated agent); the main session copies them over.
Message: advisor-u6c-brief.md §5 items 1-8, advisor-u7-brief.md §5 item 9, the main session's item 10.
Plan (in order; one line each below as done): 1 npt-host path+service+host.json in rs.yaml, r1.sh is-active checks,
tests 1 and 3 · 2 TLS names production.*/staging.* · 3 hash URL /npt/cert-hash-NN.json, plan.md:995-996 superseded ·
4 probe settings, empty token, LG duration, wait on npt-super's log, probe_presence per-leg + fixture test · 5
load_settings fields, end at duration_s, READY wait · 6 test 6 no finding, hook EXPECTED from F26 · 7 report names
u8-brief.md:383, :505-507, :521 superseded · 8 npt-super start flags, room_settings fields only · 9 save() header and
footer, pure builder in config.mjs + test + plant · 10 npt-super run fields equal the room's settings + test.
Resumed 06:37 UTC after a spend-limit cut at 03:59: no source edit had been made (git status: patch only); item 1
redone whole.
Done:
- Item 1 DONE: new ops/systemd/npt-host.path (PathExists production.chain.pem; WantedBy multi-user), npt-host.service
  (User/Group npt-host, NoNewPrivileges, ExecStart serve /etc/npt/host.json, Restart=no, no [Install]), host.json
  (bind 0.0.0.0:31000, cert_chain/cert_key production.*, dist, hash_template room-NN, host_keys; field names U8's
  until U6c merges); rs.yaml:218-222 installs them root 0644 from the clone, daemon-reload, `enable --now
  npt-host.path`; r1.sh:70 host_active (`systemctl is-active npt-host`), :418 browsers() first line, :477 new `host`
  block; PROMPT-R1.md §3 step 4 runs `r1.sh host` first. Tests: test 1 +1 ("rs: npt-host.path and npt-host.service
  …", 20 pass); test 3 +5 (no sudoers line names npt-host or systemctl, both files; host_active; browsers first;
  host block first: 219 ok). rs.yaml still < 32 KiB (test 1).
- Item 2 DONE: r1.sh:130 room settings name production.chain.pem / production.key.pem; npt-cert:6-14 header names the
  four files and the stderr `PROFILES …` line (passed through: the coproc redirects no stderr); r1.sh:273 cert_wait
  keeps all but the TXT step. Test 3 +7 (four names in the header; the call line unchanged with no stderr redirect;
  the cert block keeps stderr; production pair, no shared name): 226 ok.
- Item 3 DONE: R1.md:134-136 `/npt/cert-hash-NN.json` (NN from the page's `room` query; per room only, Q-U6C-3);
  R1.md:250-251 plan.md:995-996 joins the superseded list after :287; cert14d.mjs:3-5 comment. Test 5 +1 (the per-room
  URL in R1.md and cert14d.mjs, :995-996 beside :287, no ops file but the tests names the plain URL): 8 pass.
- Item 4 DONE: r1.sh:34-40 PROBE_* (6 x (10 + 100 + 12) = 732 s planned; the advisor's "about 690" did not add up with
  its own 10 s set-up, so the sum is written out); :119 probe settings (probe_peer_ipv4 = LG, probe_legs six,
  probe_packets 1,000,000, probe_warmup_s 10); :307-332 the block: `printf '' | lg "token 02"`, LG 792 s, waits for
  npt-super's room_exit up to 852 s, then `lg "stop 02"` (abort path), keep copies the six (:190-194);
  :334-371 probe_presence reads room-02-<leg>.log for leg_edge/cpu_stat at warmup and after (Q-U6C-5 names), P1
  drops equal. R1.md §5 C-U8-2 bullet and §8 (hours re-check: 0.20 h vs about 0.21 h, pending U6c's reading; no
  reading exists yet). New test 7 ops/tests/r1sh.test.mjs: 2 pass (settings and block; fixture logs present / missing
  edge / P1 drop / row not found / string count / missing log).
- F28 STOP-ITEM for the main session (found at item 4, not fixed: Rust, not U8's lane): npt-super's log is a
  BufWriter (super/src/main.rs:214-218) and npt-log's Writer flushes only in finish (log/src/lib.rs:586-608), so
  `room_start` and `room_exit` reach disk only when npt-super ends. r1.sh's room_up (waits 15 s for room_start in
  super-NN.log, U8's design) would die at every start on RS, and item 4's wait for room_exit never sees it before the
  stdin close. Needs npt-super to flush after each event (a Writer flush, or the log unbuffered: two events a start).
- Item 5 DONE: r1.sh:177-208 load_settings (python3 -I, JSON): U8's ten fields plus run, set, marks, date, commit,
  seed from the room's own settings file (one source), machine LG/LG1/CAX21/fsn1/LG's IPv4, behaviour `brawl`, pin_hex
  (wt: the room's hash file's value_hex; ws: production.txt's "Ledger: … sha256 <hex>."; probe: 64 zeros, no hash
  file); refuses a mode outside run|calibrate|probe or a missing pin. :210-218 load_ready (READY in `lg "log NN"`);
  :444-446 ds-selftest waits 60 s for READY, else INVALID-ready.txt. Modes `bots` -> `run` (:335, :442: C1 (c)).
  :29 HASH. npt-load-ctl:13-19 header (C1 (c): one argument, READY/DONE, stdin /dev/null; stop = abort); start line
  unchanged. Test 7 +1 (19 fields in order; run fields equal the room's; pins on wt/ws/probe; ws refused without the
  production line; `bots` refused; the /dev/null start line; READY before load_wait): 3 pass.
- Item 6 DONE: ops/tests/redact.test.mjs rewritten: no finding over the spike folder (KNOWN_AT_BASE gone), and the
  summary line reads 0 findings; ops/operator/pre-push:11 EXPECTED=2609bc87…519d31 (redact-check.mjs at F26's 2a9498d:
  blob 4dcc849d equal to the worktree's); operator.test.mjs:93-101 pins that literal beside the file's hash. Test 6:
  1 pass; operator test: 8 pass (were 6 of 8: tests 50-51 red on the old EXPECTED).
- Item 7 DONE (no edit, report line): u8-brief.md:383's parenthesis ("Q-U8-6's knob, if taken, is U6c's"), :505-507
  (Q-U8-6 (i)'s lean: the `NPT_KC_BURSTS` knob riding in U6c) and :521 (section 10's "U6c's takes … Q-U8-6 (i)'s
  knob") are superseded by C10 in section 0.1 (u8-brief.md:97-101: the knob rides in MOVED's brief; U6c takes Q-U8-4's
  interface, Q-U8-5's reads, the probe's ports and npt-load's control interface, "not the knob"). U8 built no Rust.
- Item 8 DONE: r1.sh:109-175 room_settings rewritten through json_obj (:111-119, python json.dumps): only U6a's merged
  Settings fields (crates/server/src/main.rs:62-103: log, token_file, machine_* flat, marks, tick_hz, io_threads,
  duration_ms 0, warmup_ticks, first_cpu_model; `bite` only in bite rooms) + U6b's (wt_bind, ws_bind, transport,
  ws_cert_chain/key, hash_path, ws_split_mark_s) + U6c's (mode room|probe; probe's four). The `super` object, log_path,
  token_path and the `calibrate` room mode are gone; role measured|bite (was "RS", the machine role); each start its
  own run R1-<set>-NN<sfx>, set R1-<set>; io_vcpus an int array. U8's values for required fields it never set (to
  Stop B's netcode seat): io_threads = housekeeping vCPUs, warmup_ticks 1,800 where LG joins (headers, calibrate, ds)
  else 0, first_cpu_model from bring-up's facts (empty at the first start); marks per R1.md's table rows (alone,
  calibrate, ds, browsers: none). npt-lib.sh:182-229 NPT_SUPER_FIELDS, NPT_SETTINGS_PY, npt_super_args (17 values,
  each by an anchored pattern; bites build for a bite; super log = room log with super-); npt-ctl:148-162 reads the
  settings as npt-room (setpriv python3 -I) and execs `npt-super start "${NPT_SUPER_ARGS[@]}"`. Headers of r1.sh,
  npt-ctl; R1.md condition 3 and §8's interface list. Tests: test 3 +73 (the scope line, the read, npt_super_args
  accepts/bites/marks, 65 field refusals, another room's log, a missing value, a bad NN): 299 ok; test 7 +1 (fields
  against U6a's struct read from main.rs, U6b/U6c names, bite only in bite rooms, every flag one npt-super parses and
  every required one emitted): 4 pass.
- Item 9 DONE: config.mjs:72-80 PAGE gains clockResNs/dropped; :82-171 LOG_SCHEMA 1, SUITE_MARKS (R1.md's rows;
  wt-attempts -28 per advisor-u7 C4), ROOM_SCENE SS, RUN_PATH eu-path, codeCommit (Y from rs.yaml's /etc/npt/commit,
  zeros refused), logDate, osMachine (Node's os; plan "", threads_per_core 0, smt [] as os gives none), legHeader
  (pure: Header's 18 fields, Machine's 13, Build's 4; run R1-<suite>-<browser>-<transport>-aNN, set R1-<suite>; role
  measured | control (wt-attempts + hash=wrong) | bite; params suite, browser, path, transport, room, attempt,
  version, records_dropped_count, crate on wt-attempts only), legLog (header, page lines unchanged, footer 1 + lines,
  clean|stopped). lib.mjs:108-118 openAndWait's timed_out; :120-128 read() returns nptClockResNs and nptDropped;
  :129-145 runInfo (Y, date, the Wasm's SHA-256 fetched once per commit, kept under .gate/); :160-181 save(run, leg,
  state, summary) and saveSummary (whole-suite summaries write no log). smoke/cert14d/fallback/mds36 call them.
  Test 4 +1 ("item 9: …": field sets read from crates/log/src/lib.rs, SCHEMA from lib.rs, footer count, lines
  unchanged, no token/URL/address, control/bite roles, os gaps, codeCommit, read/save wiring): 9 pass with
  webdriverio resolvable (test 7 needs ops/browser/node_modules: absent in a fresh worktree, linked from U8's build
  worktree for the run, link removed after). PLANT P8: the footer's count without the header (`records:
  lines.length`) -> the item 9 test FAILS "the records before it, the header included" (3 vs 4); restored, config.mjs
  sha256 4cf9b0af…c893 equal before and after.
- Item 10 DONE (built with item 8; its test now): one source, the room's settings file: npt-ctl reads it as npt-room
  (NPT_SETTINGS_PY) and npt_super_args turns run, set, role, layer, profile, date, scene, tick_hz, bite (and commit,
  machine_*, marks, the super log) into npt-super's flags; `--scene`, `--tick-hz` always, `--bite` only for a bite
  room (bites build); npt-super ends by its stdin closing (room_down kills the holder only). Test 7 +1 ("item 10: …":
  for all eight kinds of room, bite included, npt-ctl's own read + npt_super_args on room_settings' file give flags
  deepEqual to the settings' values; room_down's one kill is the holder's; no pkill/killall/systemctl stop): 5 pass.
  PLANT P9: `--bite` dropped from npt_super_args -> "item 10" FAILS on the bite room ("--bite" missing); restored,
  npt-lib.sh sha256 4ea06906…5a48 equal before and after.
- END of the fix pass: `node --test 'ops/**/*.test.mjs'` 75 pass, 0 fail (66 before: +1 test 1, +1 test 5, +5 test 7
  new, +1 test 4; tests 6 and the operator test's two now green), with ops/browser/node_modules linked from U8's build
  worktree for test 4's script check (a fresh worktree has none; without it that one test fails as at the start),
  link removed after; `bash ops/tests/wrappers.test.sh` 299 ok, 0 failed (214 before); `check.mjs --only redact` PASS,
  393 items, 0 findings. Plants P8 (item 9) and P9 (item 10), each failing its test by name, restored, sha256 equal.
  package.json 8370495d…, package-lock.json a0efce8e… unchanged (DEP-HOLD). rs.yaml 11,375 bytes (< 32,768). No
  tracked spike file changed; no commit. Size: 42 new files under ops/, 4,957 lines (U8 4,176 + 781 net from this
  pass); SIZE-CHECK (6,000) not reached.
- Open for the main session: F28 (npt-super's log buffered: room_up and the probe's exit wait cannot read it live,
  STOP-ITEM before R1); U6b's and U6c's room-settings names, host.json's names and npt-load's `behaviour` value are
  U8's until those units merge (one home each); U8's values for U6a's fields it never set (io_threads,
  warmup_ticks 1,800, first_cpu_model) and the room marks per block go to Stop B's netcode seat; the probe's 732 s
  against R1.md's 0.20 h waits for U6c's reading. Not verified here (no systemd in this container): whether the path
  unit starts the service again after the host exits while the chain still exists; if it does, `Restart=no` is
  bounded only by systemd's trigger limit (for infra-engineer at Stop B).

## Fix pass 2 (Stop B, 2026-11-23)
Ruling advisor-u8-stopb.md sha256 daf55033…9192 checked; the worktree equals wip/u8-build.patch 60205020…d662
(`git apply -R --check` clean, 42 files); `git status`: nothing half done. Start: 4,957 added lines.
Plan, section 7's order: 1 .jsonl names + collect per block (NC-SB-1, ADV-SB-1) · 2 own certificate off the browser
rooms (INF-SB-1, NC-SB-11) · 3 durations + READY guard (NC-SB-2, Q1) · 4 per-leg run/header/footer (NC-SB-3) · 5 S14
600 s + 34 bursts (NC-SB-4) · 6 drivers loopback (SEC-SB-1, -4) · 7 cloud-init (SEC-SB-2, -6, -10, INF-SB-2, -8, F12)
· 8 operator (SEC-SB-3 setting from the main session's record, -7, -11) · 9 dist compare (SEC-SB-5) · 10 alone after
down, TIMING-INVALID (NC-SB-8) · 11 traps, refusals, -aN (NC-SB-10, INF-SB-12) · 12 LG npt-log verb (NC-SB-7) · 13 rate,
slack, policy (NC-SB-9) · 14 S14 noise guard, summary (NC-SB-12) · 15 marks (NC-SB-5, -6) · 16 R1.md/PROMPT lines · 17
unit limits and sandbox subset (INF-SB-6, SEC-SB-9) · 18 PID 1 read (INF-SB-9) · 19 exact sudo list (SEC-SB-8) · 20
NC-SB-13..-17. One line per item as done, with the running total.
Done:
- 2.1 DONE (NC-SB-1, ADV-SB-1): every npt-log `*.jsonl` (r1.sh room_settings' log, keep, room_up, alone read, probe
  wait, starts glob, probe_presence's `room-02-<leg>.jsonl`, load_settings' `load-NN.jsonl`; npt-lib.sh
  npt_super_args' pattern; lib.mjs save; R1.md §5 and the interface list); collect: the whole tree first, on a read
  error each block folder apart and split.txt names the file; tar paths from $R. Test 7 +2 ("2.1 NC-SB-1" reads the
  judge's rule from crates/judge/src/lib.rs; "2.1 ADV-SB-1" with an npt-judge stand-in and one cut log). Plant P10:
  `.log` back in room_settings -> "2.1 NC-SB-1" FAILS; restored, r1.sh ab3c2da9… equal. Total 5,029.
- 2.2 DONE (INF-SB-1, NC-SB-11): room_settings names the production pair for browser rooms only; every other kind
  writes `ws_cert_chain`/`ws_cert_key` null (the room's own certificate; U6b's field form checked at its merge);
  load_settings pins the room's hash file unless ws to a room naming the production chain; R1.md §3's outage line.
  Test 7 +1 ("2.2 INF-SB-1"); item 5's case and test 3's check moved to the new rule. Plant P11: the production pair
  on `alone` -> "2.2" FAILS; restored, r1.sh 3127607b… equal. Total 5,053.
- 2.3 DONE (NC-SB-2, Q1): room_settings' duration_ms per kind (headers and calibrate 300,000, ds 420,000, else 0);
  new lg_block (LG = room + 30 s; READY by OPEN + warm-up − 10 s from the settings' warmup_ticks, else `lg stop`,
  room_down, INVALID-ready.txt; then npt-super's room_exit, LG's end, room_down) used by headers, calibrate, ds;
  `setting` helper; load_ready on a clock deadline (NC-SB-14 done here); R1.md §5 LG-rooms bullet (Q1 named). Test 7
  +1 ("2.3": all three kinds, READY and no READY, with stubs and a call trace); item 8's duration assert updated.
  Plant P12: `duration_ms 0` back -> "2.3" FAILS; restored, r1.sh 99f24d76… equal. Total 5,108.
- 2.4 DONE (NC-SB-3): probe_presence takes the settings' run and reads each leg's header (run `<run>-<leg>`, profile
  the leg, process probe, layer L2) and footer, else INVALID; the probe block passes `$(setting 02 run)`; R1.md's
  interface line. Test 7's fixtures: the settings run on P0, P3 sharing P2's run, a missing footer on P4, a wrong
  profile on B1 (each INVALID), six good legs present. Plant P13: the run check removed -> the fixture test FAILS
  (P0 present); restored, r1.sh 11c764a7… equal. Total 5,133.
- 2.5 DONE (NC-SB-4, section 4): probe.py STEADY_S 600, 34 bursts every 10 s from 600 s, TOTAL_S 950 (28,500
  messages); compare.py STEADY_END_NS 599 s, the same 34, MESSAGES 950 s, the sample in its header; npt-s14's header;
  R1.md §1 S14 0.80 h with its last 0.50 h beside the browser legs (rows 4.05, span 3.55), §2 timeline (chain 10:39-
  11:21, S14 11:21-12:09, browsers 11:39-12:57) and the 11:39 step, §5 S14 bullet (n and ranks, the overlap's
  guards); PROMPT-R1.md §4-5; r1.sh stamps in browsers/browsers-down, s14 keeps them, one chain at a time (bites
  refused while server runs). s14 test: N 28,500, 102/136 recoveries, env 28500; +1 "2.5 NC-SB-4" (one schedule,
  >= 1,000 episodes, >= 101 recoveries): 9 pass. Test 5 recomputes hours (sum − overlap) and the timeline: 8 pass.
  Test 3 +2 (chain refusal, stamps): 301 ok. Plant P14: STEADY_S 60 -> s14 test FAILS (2 tests); restored,
  probe.py 469da684… equal. Total 5,189.
- 2.6 DONE (SEC-SB-1, SEC-SB-4, section 5): config.mjs header reworded; capabilities name the browser binary only
  (no wdio:*driverOptions); driverArgs (chrome/edge `--port` only; firefox `--host=127.0.0.1 --port --websocket-
  port=0`); remoteOptions name 127.0.0.1:<port> path "/"; lib.mjs session starts every driver from its recorded path
  after verifyDriver and connects remote() to it. rules.md, PROMPT-R1.md §1.5 and R1.md Stop B step 5: C8's inbound
  allow-rule read, the Cancel/stop line, the Stop B dry start with Get-NetTCPConnection (recorded on his laptop: not
  run here). Test 4's SEC-R1D-2 test replaced by "SEC-SB-1, SEC-SB-4" (argv, remote options) and SEC-R1D-3 adjusted:
  9 pass; operator test's C8 rule strings: 8 pass. Plants P15 gecko `--host` dropped and P16 `--allowed-ips` back on
  chromedriver: each FAILS test 4; restored, config.mjs bdce7f7c… equal. Total 5,191.
- 2.7 DONE (SEC-SB-2, -6, -10, INF-SB-2's bounds, INF-SB-8, F12's term): rs.yaml `runuser -u npt -- install -d -m 0700
  /home/npt/.ssh`; binaries read as npt-build (`take`: runuser cat to a root tmp, then install) and the dist tar read
  as npt-build, RS and LG; RS nft output: `meta skuid 2001 ip daddr 127.0.0.53 drop`, `meta skuid 2001 th dport 53
  drop` before the loopback accept; every curl `--max-time 900 --retry 2`, every git fetch `-c http.lowSpeedLimit=1000
  -c http.lowSpeedTime=60` (RS and LG); RS `command -v dig tcpdump` after the upgrades; firstboot's first step (RS and
  LG) writes `ssh-keygen -l -E sha256` of each host key to /etc/issue.d/50-npt-hostkeys.issue and runs `agetty
  --reload` (the means chosen: issue.d, shown above the console's login prompt and kept across screen clears);
  PROMPT-R1.md §2 step 2 names it and the 15-minute rule. Test 1 +2 ("2.7 …" per role; nft -c covers the new rules):
  22 pass; rs.yaml 12,315 B, lg.yaml 7,644 B. Plant P17: rs.yaml's old `install -d -o npt …/home/npt/.ssh` -> "rs: 2.7"
  FAILS; restored, rs.yaml 9449d10d… equal. Not verified: Hetzner's console showing /etc/issue.d (no VM). Total 5,231.
- 2.8 DONE (SEC-SB-3 per the main session's docs record u8-sec-sb-3-docs.md, SEC-SB-7, SEC-SB-11): settings.json
  `permissions.blockReadsOutsideWorkingDirectories: true`; denies `Read(~/.ssh/**)`, `Read(//**/.ssh/**)`, 14
  PowerShell read forms (Get-Content, gc, cat, type, Select-String, Format-Hex, Copy-Item x npt_operator, .ssh), and
  per shell the `refs/heads/` lane and cs/* forms, `--no-verify` (two), `git -c *`, setting core.hooksPath (four
  forms; `git config --get core.hooksPath` stays readable for start check 7). pre-push reads git's ref lines first
  and refuses any remote ref but refs/heads/claude/npt-results, then pipes them to redact-check (`set -euf`).
  rules.md: the setting and its version, the read-only set, the documented gaps, the passphrase-in-ssh-agent wall,
  the hook's ref rule. PROMPT-R1.md: token link files named (:7) and deleted at §6.3; start checks 7 (hooksPath) and
  8 (`claude --version` >= 2.1.257). R1.md Stop B step 6: the version read and the passphrase. Operator test +1
  ("2.8 …") and the hook test refuses four other refs: 9 pass. Plants P18 the setting removed -> "2.8" FAILS; P19
  the hook's ref check removed -> the hook test FAILS (lane ref pushed); restored, settings.json f403db83…,
  pre-push c38d9564… equal. Total 5,319.
- 2.9 DONE (SEC-SB-5): new ops/runs/r1-dist.sha256 (comment only until Stop B's fill); r1.sh EXPECTED_DIST,
  dist_check (python: names every file whose hash differs, is missing or extra; an empty list differs; writes
  dist.txt and DIST-DIFFERS; never stops bring-up), bringup calls it before `done`; host_active refuses without
  dist.txt or with DIST-DIFFERS unless DIST-ACCEPTED; new `dist-accept` block (his word, refused on an empty list);
  R1.md §8, PROMPT-R1.md §2 step 10. Test 7 +1 ("2.9": equal served, a differing hash refused and named, his word
  serves, a missing file refused, the committed unfilled list refused; the chain names no DIST); test 5 +1: 9 pass.
  No plant named by the ruling for this item. Total 5,405 (43 files).
- 2.10 DONE (NC-SB-8): r1.sh alone_min (the least `alone` over the affinity events) read after room_down 00, from
  the closed log; chain_run skips each TIMING_BLOCKS block under TIMING-INVALID (NOT-RUN.txt "not run: alone 0 (C1)"
  with alone-tasks.txt beside it, chain marker stopped), calibrate and s14 still run; collect's judge.txt opens with
  the C1 line (C1.txt). Test 7 +1 ("2.10": the min of two events, read after down, both chains with a block stub:
  only calibrate and s14 ran, five NOT-RUN folders, the C1 line first); the npt-judge stand-in shared. Total 5,454.
- 2.11 DONE (NC-SB-10, INF-SB-12): r1.sh block_start (dispatch, for bringup and the seven chain blocks): refuses a
  block the chain marked done unless its folder holds INVALID*; counts attempts (`attempts` file) and sets ATT=-aN
  from the second, which room_settings, room_up, keep, lg_block's and the probe's super-log waits, the alone read and
  probe_presence carry into run ids and log names; `trap down_all EXIT` (every room the block opened brought down).
  busy: a live room holder (not for s14) or another running chain refuses the block, recorded in its stopped marker;
  chain_run writes `<chain>.ended` from an EXIT trap (expanded at set time: a test caught the unbound local);
  deadline_ok: 20-minute margin, LG blocks also against LG's delete-by (new `lg-boot <epoch>` block from PROMPT-R1.md
  §2 step 9; unrecorded: RS's boot − 15 min); the probe writes INVALID-presence.txt on an INVALID leg. Test 7 +1
  ("2.11": the trap brings room 01 down on exit 3; -a2 in run id and log; attempts file; done refused, INVALID
  allowed; busy on a live holder, not for s14, on another chain): 12 pass; test 3 301 ok. Total 5,545.
- 2.12 DONE (NC-SB-7): npt-load-ctl's one new fixed verb `npt-log NN` (head -c 256 MiB of logs/load-NN.jsonl;
  npt_load_parse takes it); `start` removes the last load-NN.jsonl first (a re-run's fresh log; RS kept the last);
  r1.sh's headers, probe, calibrate, ds blocks keep `lg "npt-log NN"` as load-NN$ATT.jsonl; load_settings passes role,
  layer, profile, scene and tick_hz from the room's settings. Test 3 +7 (accepts `npt-log 05`; refuses 5 forms; the
  fixed path): 308 ok; plant P7 re-run: 2 FAIL as before, restored, npt-load-ctl 1ad94210… equal. Test 7 +1 ("2.12":
  eleven header fields equal the room's for all four LG kinds; each block keeps LG's npt-log; the re-run line): 13
  pass. Total 5,577.
- 2.13 DONE (NC-SB-9): probe_presence reads each leg's `l2_summary` window counts (probe_up/down_window_count over
  probe_window_ns: 10,000 a second within 1 %, else INVALID; unread, INVALID) and P1's `l2_timer_slack_ns` and
  `l2_sched_policy` at both edges (ImpairSocket's names, socket.rs:319, :362; U6c's merge confirms); R1.md's
  interface line (also NC-SB-16's probe_packets line). Fixtures: P2 2 % off, P0 no window counts, P1 slack missing
  at warmup and policy missing at after (each INVALID), P4 0.9 % off and P3 exact present: 13 pass. Total 5,608.
- 2.14 DONE (NC-SB-12): probe.py's client reads total_retrans at the steady window's end (599 s, before sending the
  first message after it) into `tcp_info_client.total_retrans_steady`; compare.py's noise guard compares the steady
  episodes with it (unread: INVALID), its header and VOID/INVALID follow-up text (the re-run rule; a MISS never
  re-run). r1.sh: s14_run (one npt-s14 run, then `compare.py summary` on RS into summary.txt); s14: a VOID or INVALID
  run writes INVALID-s14.txt and runs once more into rerun/ only if now + S14_S (2,880 s) + 300 s <= RS boot +
  BITES_AT_S (12,420 s), else rerun-R2.txt. s14 test +1 ("2.14": 6 episodes > 3 steady retransmissions VOID though
  total_retrans 50; unread INVALID; r1.sh's grep matches the real summary): 10 pass. Test 7 +1 ("2.14": time left re-
  runs once, too late writes the R2 note, judged never): 14 pass. Total 5,681.
- 2.15 DONE (NC-SB-5, -6): room_settings: probe PM-0001-3 only; PM-0001-16 on alone, start, headers, calibrate, ds and
  browser rooms (bites keep their own mark); R1.md §1's rows (probe "-3; conditions C-U8-2, C-U8-4, C-U8-6"; -16 on
  bring-up, headers, calibration, DelayStream) and §4's line on the browser rooms. Test 7 +1 ("2.15"): 15 pass; test 5
  9 pass. Total 5,698.
- 2.16 DONE (INF-SB-2's rule and provider list, INF-SB-3, -4, -5, -7, -10, -11, -13, F9, F20, NC-SB-18, SEC-SB-8's LG
  checks): R1.md §1 the slack per VM (RS 5 min at worst, LG about 10); §2 Stop B step 2 C7's text with six Primary
  IPv4s, step 3 the fsn1 stock check before FIXED-IP (nbg1 fallback; r1.sh's new LOCATION fill, used by room_settings
  and load_settings) and "exactly the list's /32s", the 09:10 source counts read aloud; §3 the 20-minute deadline
  line, the bring-up rule (six providers, bounded fetches, delete by 10:15, one same-day retry by 10:45, cap EUR 0.78,
  no console fingerprint = teardown) and the three more cases; §5 F9's line; §6 INF-SB-13's sentence; §8 F20's
  bites with their judges. PROMPT-R1.md: §2 step 3 the 10:15 rule, step 4 LG's id/sudo/steps checks, §3 the ledger
  wait at most 10 minutes, §4 the collect once the server chain ends. Test 5 +1 ("2.16": slack from the timeline, 1 +
  5 = 2 + 4 Primary IPv4s, 10:15, 0.78, the six providers, the counts): 10 pass. Total 5,742.
- 2.17 DONE (INF-SB-6, SEC-SB-9's subset): npt-host.service [Unit] StartLimitIntervalSec=300, StartLimitBurst=3,
  [Service] ProtectHome, PrivateTmp, CapabilityBoundingSet= (empty), RestrictNamespaces, LockPersonality,
  SystemCallArchitectures=native (NoNewPrivileges kept); its header states the restart rule and what waits for U6c's
  merge (ProtectSystem=strict, PrivateDevices, MemoryDenyWriteExecute, SystemCallFilter, RestrictAddressFamilies:
  SEC-SB-9's open part, named in the report); host_active refuses a failed npt-host.path. Test 1's unit assert
  extended: 22 pass; test 7's 2.9 case: a failed path unit refuses: 15 pass. Total 5,768.
- 2.18 DONE (INF-SB-9): npt-ctl cpus tries set-property on init.scope (taken or refused, recorded), then reads PID 1's
  Cpus_allowed_list and, while npt_pid1_needs_move (new in npt-lib.sh: the list holds the tick vCPU or does not read)
  says so, runs the taskset fallback too, recording the list before and after. Test 3 +7 (0-3 with tick 3 moved,
  0-2 not, four unreadable lists moved, the order in cpus()): 315 ok. Total 5,791.
- 2.19 DONE (SEC-SB-8's RS half): r1.sh sudo_exact (python: `sudo -n -l`'s command lines, merged lines split, against
  sudoers.d/npt's nine `npt ALL=(root) NOPASSWD:` lines; prints each extra or missing line, exit 1) replaces bring-
  up's full-rule grep and stops bring-up on any difference. Test 7 +1 ("2.19": the nine pass; one line beyond, one
  missing, an (ALL : ALL) ALL line stop; one merged line reads alike): 16 pass. Not verified: sudo 1.9.15's exact
  `-l` layout on the VM (read from its known form). Total 5,837.
- Resumed after the 12:28-16:22 UTC cut: git status shows only U8's files and these notes; no plant left (grep of
  every plant line: 0); settings.json f403db83, pre-push c38d9564, config.mjs bdce7f7c, rs.yaml 9449d10d,
  npt-load-ctl 1ad94210 equal their post-restore hashes; item 19 was done before the cut, item 20's edits were made
  and their tests passed, not yet logged.
- 2.20 DONE (NC-SB-13, -14, -15, -16, -17): room_up, after room_start, waits 1 s and dies when room_exit follows (a
  deadline kill or an early exit; live once F28's flush is in, U6b's merge); NC-SB-14 done in 2.3 (load_ready's clock
  deadline); room_settings trims first_cpu_model at both ends; NC-SB-16's interface line done in 2.13 (R1.md); probe.py
  netem() returns each link's call times (data direction first) and bursts records `actual` (the data direction's
  window, which compare.py's validity rule reads) and `actual_reverse`, the tc call times per link; compare.py's
  header names it; no early-start compensation (dropped by the ruling). Test 7 +1 ("2.20": open, a start followed by
  room_exit dies, the trimmed model): 17 pass; s14 test +1 ("2.20 NC-SB-17"): 11 pass. Total 5,876.
- END of fix pass 2: `node --test 'ops/**/*.test.mjs'` 95 pass, 0 fail (75 before; test 4's script check run with
  ops/browser/node_modules linked from U8's build worktree, link removed after); `bash ops/tests/wrappers.test.sh` 315
  ok, 0 failed (299 before); `check.mjs --only redact` PASS, 394 items, 0 findings; package.json 8370495d…,
  package-lock.json a0efce8e… unchanged (DEP-HOLD); rs.yaml 12,315 B, lg.yaml 7,644 B. Plants P10-P19 each failed
  its test and was restored with its sha256 checked; P7 re-run. 43 new files, 5,876 lines (4,957 + 919): SIZE-CHECK
  not reached. No commit, no cargo, no npm install.
- Open: F28 (npt-super's flush, U6b's merge) gates room_up's live reads, NC-SB-13's check and the probe's exit wait;
  at U6b's merge: the own-certificate field form (null pair), the WebSocket pin's hash file, U6b's settings names; at
  U6c's merge: host.json's names, the probe's leg header form, the rate/slack/policy names, probe_packets, npt-load's
  header and footer, behaviour `brawl`, the chain's write-then-rename, SEC-SB-9's rest; Stop B on his laptop: the
  drivers' listen read, `claude --version`, the passphrase, the fsn1 stock read; not verified here: Hetzner's console
  showing /etc/issue.d, sudo -l's layout on the VM, systemd and nft on a VM.

## Pass 3 (round 2, 2026-11-23)
Plan: advisor-u8-r2.md (sha256 d107cbe3…a784) section 3's items 1-8 in order, each with its proof and plant, on the
pass-2 tree (wip/u8-build.patch bca52b2c…6eb7; git status: only U8's files and these notes, nothing half done). Start:
43 files, 5,876 lines. SIZE-CHECK: past 6,000, stop at the end of that item and report.
- 3.1 DONE (NC-R2-1): r1.sh save_log (python: a whole log, or one cut at a line end, copied as it is; one cut inside a
  line: its leading whole lines that each parse as one JSON object kept as <name>, the whole file as <name>.raw; empty
  or no header line: <name>.empty; each such named in <dir>/cut.txt; a missing file skipped); keep saves the room,
  super and leg logs through it; lg_keep (LG's stdout copy, then `lg "npt-log NN"` into $RUN/load-NN.fetch, save_log,
  fetch removed) is the one LG fetch, in headers, probe, calibrate and ds; the probe block: exit, room_down, stat-after,
  `load_wait 02 120` (LG's own end; `lg stop` only as load_wait's abort), lg_keep. Comments: the probe's wait, the
  deadline's "17 at its bounds", collect's read. R1.md section 8 one line on save_log; the probe hours line 0.23 h with
  LG's 60 s tail. Test 7: test 4's probe asserts (then LG's end; no `lg "stop`), 2.12's (lg_keep in the four blocks,
  one LG fetch), JUDGE_STUB now models read_log (a line not one JSON record or an empty log fails the read; no footer
  reads), 2.1 ADV-SB-1's cut fixture cut inside a line; +3 cases on judge/fixtures/pm-0001-4/base's logs: save_log's six
  forms; the probe block's trace (load_wait 02 120 after lg start, before the fetch, no lg stop; load-02.jsonl the 3
  whole lines, .raw the cut copy, cut.txt names it); a results tree with s30-2-lg.jsonl cut inside its footer line and
  an empty load-07.jsonl reads whole under the stand-in, while the copy as fetched fails it at line 4. 20 pass. Plant
  (probe's fetch back to `lg "npt-log 02" > "$d/load-02$ATT.jsonl"`): 3 FAIL (test 4, 2.12, and 3.1's trace: the cut
  copy saved whole as .jsonl), restored by Edit, r1.sh ed12ebf6… equal to its pre-plant hash. Total 6,001 (+125: r1.sh
  +41, R1.md +3, test 7 +81).
- Fixture tree for the main session's npt-judge run (made by test 7's own case, NPT_R1_TREE set, no new code):
  /home/user/Shamble-Frontlines/.claude/worktrees/agent-ac59cce3e5d002441/.gate/u8-p3-judge-tree (gitignored). It is
  judge/fixtures/pm-0001-4/base's eight logs through save_log, s30-2-lg.jsonl cut 20 bytes before its end (inside the
  footer): s30/s30-2-lg.jsonl its 3 whole lines (no footer), s30-2-lg.jsonl.raw the cut copy, load-07.jsonl.empty,
  cut.txt naming both. Want: `npt-judge --marks spikes/netcode-proof/marks/marks.json --logs <tree> --out <file>` exits
  0, and run s30-2 reads INVALID by "complete: the bots log on LG1 has no footer (cut short)". Rebuild into a new
  folder: `NPT_R1_TREE=<new folder> node --test --test-name-pattern='results tree' ops/tests/r1sh.test.mjs`.
- SIZE-CHECK STOP: 6,001 added lines after item 1 (past 6,000 by 1); stopped at the end of item 1 as the brief says.
  Items 2-8 not started; item 3's container record (sudo into a regular file under .gate/ against the pipe form) not
  made. End checks on this state: `node --test 'ops/**/*.test.mjs'` 98 pass, 0 fail (test 4's script check with
  ops/browser/node_modules linked from U8's build worktree, link removed after); `bash ops/tests/wrappers.test.sh` 315
  ok, 0 failed; `check.mjs --only redact` PASS, 394 items, 0 findings; package.json 8370495d…, package-lock.json
  a0efce8e… unchanged (DEP-HOLD); no plant left. No commit, no cargo, no npm install.
- Open: SIZE-STOP for Mustafa (go on with items 2-8, or stop U8 here; the advisor leans go on); items 2-8 as ruled;
  the main session's npt-judge run over the tree above; F28 and the other merge-time checks as in pass 2's Open line.
- Resumed on his answer (a), go on: items 2-7, then the Lows; the new stop 6,200 (a Low that would pass it is carried,
  never a stop). git status: only U8's files and these notes.
- 3.2 DONE (SEC-R2-1 = INF-R2-1): rs.yaml and lg.yaml: `install -d -o root -g root -m 0755 /etc/issue.d` as the
  fingerprint step's first command. Test 1's 2.7 case asserts it comes before the write in both: 22 pass. Plant (the
  line dropped from both): rs and lg 2.7 FAIL, restored by Edit, rs.yaml 829db42e…, lg.yaml aea270cf… equal their
  pre-plant hashes. Total 6,005.
- 3.3 DONE (SEC-R2-2): bringup writes `sudo -n -l | cat > "$d/sudo-l.txt"` (a pipe; pipefail keeps sudo's failure).
  Container record (sudo 1.9.15p5, as root, no terminal): through the pipe (`sudo -n -l | cat`) the Defaults line is
  one 126-character line `    env_reset, mail_badpass, secure_path=…/snap/bin, use_pty`; into a regular file
  (`sudo -n -l > .gate/u8-p3-sudo-l-file.txt`, the one redirect, as :92-93 asks for the measurement) it is three
  lines, `    env_reset, mail_badpass,` / `    secure_path=…/snap/bin,` / `    use_pty` (wrapped at 80, continuation
  indent 4 for Defaults; lbuf indent 8 for privileges). Test 7's 2.19 case: an lbuf.c model of the wrap, checked
  against that file's three lines; sudoers.d/npt's nine lines wrapped at 80 give 11 (peers and rooms split) and stop
  sudo_exact; the pipe form asserted in bringup: 20 pass. Plant (the plain redirect back): 2.19 FAIL, restored by
  Edit, r1.sh d2f4146d… equal to its pre-plant hash. Total 6,031.
- 3.4 DONE (NC-R2-2): lg_block on a missed READY: room_down first, then `lg stop` (its INVALID text and comment say
  so); load_ready polls while the time left is above 0, each poll `LG_BOUND=$left lg …`, and lg runs ssh under
  `timeout "$LG_BOUND"` when it is set (never `timeout 0`, which GNU timeout reads as no bound). Test 7's 2.3 case: the
  no-READY rows read room_down then lg stop; a clocked load_ready (deadline 1003, clock 1000…) polls with bounds 3, 2,
  1 and fails, and returns 0 on READY; lg with ssh and timeout stubbed: unbounded, then `timeout 4 ssh`: 20 pass.
  Plants (the ruling names none; made for the record): the old order, 2.3 FAIL; the unbounded poll, 2.3 FAIL (bounds
  "none"); restored by Edit, r1.sh da628b36… equal to its pre-plant hash. Total 6,044.
- 3.5 DONE (NC-R2-3): room_settings' calibrate has no duration (held); lg_block gives a held room's LG CALIBRATE_S
  (300, a new constant) and, with no room_exit wait, `load_wait NN 360` (LG's run plus 60), then room_down; measured
  rooms as before. R1.md's LG-loaded rooms line says calibration is held, LG 300 s, then the room down. Test 7: the 2.3
  calibrate row (duration 0, LG 300 s, `load_wait 03 360` then room_down), item 8's durations (headers and ds only):
  20 pass; test 5: 10 pass. Plant (the ruling names none; for the record: calibrate's dur=300000 back): item 8 and 2.3
  FAIL, restored by Edit, r1.sh a790c522… equal to its pre-plant hash. Total 6,055.
- 3.6 DONE (SEC-R2-3): settings.json's PowerShell `git -c *` becomes `PowerShell(git -c *hookspath*)` and
  `PowerShell(git * -c *hookspath*)`; Bash keeps `git -c *`. R1.md Stop B step 6: start check 4 run once in each shell,
  each `git -C` prompting, not denied; rules.md's deny line says the PowerShell form. Operator test: 2.8's deny list
  (Bash `git -c *`, the two PowerShell forms, no PowerShell `git -c *`); new 3.6: every `git …` command in
  PROMPT-R1.md, and §6.2's push in PowerShell form, matched against every deny (a `*` as any text; PowerShell's rules
  case-insensitively, Bash's not): none denied; `git -c core.hooksPath=x push` and `git -C . -c CORE.HOOKSPATH=x push`
  still denied in PowerShell: 10 pass. Plant (the old PowerShell `git -c *` back): 2.8 and 3.6 FAIL (3.6 on
  `git -C <run> rev-parse HEAD`), restored by Edit, settings.json 9aa8101e… equal to its pre-plant hash. Total 6,080.
- 3.7 DONE (NC-R2-5): TIMING_BLOCKS "headers ds-selftest bites" (its comment cites section 2); chain_run's comment;
  probe and starts no longer call timing_invalid (they run and are judged under alone 0). R1.md condition 4 and
  PROMPT-R1.md §2 step 10 say "not run (C1)" for headers, DelayStream's self-test and the bites, the probe,
  calibrate, starts and s14 still run (R1.md: ds-selftest may join at U6b's merge). Test 7's 2.10 C1 fixture: the
  chains run probe, calibrate, starts, s14; NOT-RUN with the task list for exactly headers, ds-selftest, bites; no
  record for probe or starts: 20 pass. Plant (the ruling names none; for the record: the old five-block list): 2.10
  FAIL, restored by Edit, r1.sh a5ddeaec… equal to its pre-plant hash. Total 6,085.
- 3.8 DONE (the Lows, all built, none carried):
  - NC-R2-6: lg_keep writes `load-NN$ATT.log`; LGK (dir, NN) is set after LG's start in lg_block and the probe and
    cleared by lg_keep; down_all (every block's EXIT trap) stops LG and runs lg_keep when LGK is set. Test 7 +1
    ("3.8": a second attempt dying after LG's start: `lg stop 03`, `lg log 03`, `lg npt-log 03` on exit, kept as
    load-03-a2.jsonl and load-03-a2.log): 21 pass. Plant (for the record: `$ATT` dropped from the .log): 3.8 FAIL,
    restored, r1.sh bb2c4210… equal. Total 6,104.
  - SEC-R2-4: R1.md Stop B step 5 names the read (`Start-Process <driver path> -ArgumentList <config.mjs
    driverArgs(browser, 9515)> -PassThru`, `Get-NetTCPConnection -State Listen -OwningProcess <its Id>`, loopback only,
    `Stop-Process -Id <its Id>`) and repeats it for a driver start check 6 replaces; rules.md the same; PROMPT-R1.md
    start check 6 repeats it. Total 6,108.
  - INF-R2-2: PROMPT-R1.md §4 "starts no block within 20 minutes of RS's delete-by (an LG block, LG's)"; R1.md's
    probe hours line "inside RS's 5 minutes at worst (section 1's slack)". Total 6,109.
  - INF-R2-3: npt-host.service's header "a fourth start". INF-R2-4: the three curl lines add `--speed-limit 1000
    --speed-time 60`; test 1's 2.7 asserts them on every curl line: 22 pass; plant (lg.yaml's dropped): lg 2.7 FAIL,
    restored, lg.yaml b58a8e04… equal. Total 6,109.
  - INF-R2-5's line: PROMPT-R1.md §2 step 2, Enter on the console redraws the lines (the package move dropped, as
    ruled). INF-R2-6: §2 step 9 names `ssh -F <ssh> npt-lg grep btime /proc/stat`; r1.sh's lg-boot comment follows.
    Operator test 10 pass, test 5 10 pass. Total 6,111.
- Carried (past 6,200): none; every Low is built.
- END of pass 3: `node --test 'ops/**/*.test.mjs'` 100 pass, 0 fail (98 after item 1; +3.6, +3.8; test 4's script
  check with ops/browser/node_modules linked from U8's build worktree, link removed after); `bash
  ops/tests/wrappers.test.sh` 315 ok, 0 failed; `check.mjs --only redact` PASS, 394 items, 0 findings; package.json
  8370495d…, package-lock.json a0efce8e… unchanged (DEP-HOLD); every plant restored by Edit with its sha256 checked;
  final r1.sh 5262c8ee…, settings.json 9aa8101e…, rs.yaml 857c866c… (12,489 B), lg.yaml b58a8e04… (7,783 B). 43 files,
  6,111 lines (+235 in pass 3: item 1 +125, items 2-7 +84, the Lows +26). The fixture tree above is unchanged
  (save_log untouched since). No commit, no cargo, no npm install.
- Open: the main session's npt-judge run over .gate/u8-p3-judge-tree; the last round (netcode-architect on NC-R2-1,
  security-auditor on SEC-R2-1 and SEC-R2-2); NC-R2-4 at U6c's merge (probe_presence reads U6c's named records);
  ds-selftest's place in the C1 run list at U6b's merge; sudo's list over ssh on RS not run (the container's sudo,
  root, read here); F28 and the merge-time checks as in pass 2's Open line.

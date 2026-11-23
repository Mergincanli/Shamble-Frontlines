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

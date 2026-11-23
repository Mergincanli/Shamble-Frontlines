# U8 Stop B: security-auditor seat (SEC-SB-*)

Plan: (1) brief 0.1, section 7, :19-23, :121 list; advisor C1-C10, L1-L5; u6c section 5 items 1-8; build notes F1-F28 + fix
pass. (2) Read W/ops: cloud-init rs/lg, firewall, operator (settings, rules, PROMPT-R1, pre-push, redact-check), systemd,
sbin wrappers + sudoers (GTFOBins), browser package (F25), runs (r1.sh, R1.md, s14). (3) Findings SEC-SB-n, Q-U8-1 confirmations,
:121 list by file:line, coverage of builder items F6/F7/F8/F10/F13/F16/F22/F23/F25. (4) Section 7 lines. Last line DONE.

Inputs checked: u8-build.patch sha256 60205020…c47d662 OK; u8-brief.md 3d102be9… OK; advisor-u8-brief.md 595521d9… OK;
u8-build.md cf07e435… OK. W equals the patch: all 42 files byte-equal to the patch's added text (scratchpad patchcmp.py).

## Part 1 DONE: briefs and notes read (u8-brief.md 0.1, 1-10; advisor-u8-brief.md whole; advisor-u6c-brief.md 4-6;
u8-build.md whole; security-u8-deps.md whole; security-r1-deps.md B5-B7).

## Part 2 (in progress): the package read. Notes, one line per finding as found
- N1 sudoers RS (sbin/sudoers.d/npt:5-14): cvtsudoers -f json shows each line one command, the `{1,4}` comma kept inside
  the regex (no list split). Every argument anchored ^...$, no `*`, `""` for no argument. Defaults env_reset, !setenv,
  secure_path. LG (npt-lg:4-7) the same shape.
- N2 every root wrapper re-execs `env -i NPT_ENV_CLEARED=1 PATH=… LC_ALL=C bash -p` (npt-ctl:14-16, npt-cert:19-21,
  npt-capture:7-9, npt-userdata:5-7, npt-s14:10-12); NPT_ENV_CLEARED cannot pass sudo (env_reset, !setenv); `set -f`.
- N3 dig (npt-cert:56, :97): fixed names, no caller text. HOME is not exported after `env -i` (bash binds HOME unexported:
  checked here, `env -i bash -p -c env` shows no HOME), and BIND 9.18's dig reads ~/.digrc only `if (homedir != NULL &&
  digrc)`, no cwd fallback (bin/dig/dig.c v9.18.28, read 2026-11-23) -> no .digrc from npt's cwd. dig unreachable.
- N4 F25: @wdio/utils 10.2.0 (tarball integrity = lock's sha512) build/index.js:766-770 `startWebDriver` returns when
  `definesRemoteDriver(options)` (hostname not "localhost" or a port): lib.mjs:93's remote() with 127.0.0.1:<p> starts
  no driver; build/node.js:1127 forces allowedIps 0.0.0.0 only when WebdriverIO starts Edge's driver itself. F25 holds as
  far as "WebdriverIO starts no second msedgedriver".
- N5 BUT chromedriver's server (chromium chrome/test/chromedriver/server/chromedriver.cc + http_server.cc, read
  2026-11-23): `allow_remote = true` whenever `--allowed-ips` is present; ListenOnIPv4/IPv6 then bind the all-zeros
  address; the IP list is checked per request (403). Without `--allowed-ips` it binds loopback and refuses any
  non-localhost Origin or Host. config.mjs:206 (WebdriverIO passes `--allowed-ips=127.0.0.1`: utils node.js:1081,
  parseParams :552-560) and config.mjs:228 (`--allowed-ips=127.0.0.1` for msedgedriver) -> both Chromium drivers listen
  on every interface. `allowedIps: []` stays [] (`[] || ["0.0.0.0"]` keeps []), and parseParams emits no switch for [].
  -> SEC-SB-1.
- N6 rs.yaml:163 root `install -d -o npt -g npt -m 0700 /home/npt/.ssh` inside npt's home -> SEC-SB-2.
- N7 rs.yaml:196-211, lg.yaml:149-153 root reads npt-build's tree through path components the build can make symlinks
  -> SEC-SB-3 (check whether anything compares dist.sha256).
- N8 operator settings: no deny for `--no-verify` / core.hooksPath; hook pins no destination ref -> SEC-SB-4 (Low).
- N9 r1.sh:287-288 sudo check is a deny pattern, not "only the wrappers" -> Low.
- N10 npt-host.service:7-13 only NoNewPrivileges -> Low (defence in depth).
- N11 PROMPT-R1.md:35-38 then :71-74: Q-U6C-6's order holds (console fingerprint before the first SSH; HTTPS line after
  the production certificate, before the chain and every browser room).
- N12 redact-check EXPECTED (pre-push:11) 2609bc87…519d31 = sha256 of W's redact-check.mjs = `git show
  2a9498d:spikes/netcode-proof/ops/operator/redact-check.mjs` (2a9498d "redact-check: npm tarball paths…", F26).

Status: part 2 continues (browser scripts, R1.md, tests, s14 bursts mode).

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
- N3 dig (npt-cert:56, :97): fixed names, no caller text. HOME is not exported after `env -i` (checked here), and BIND
  9.18's dig reads ~/.digrc only `if (homedir != NULL && digrc)`, no cwd fallback (bin/dig/dig.c v9.18.28) -> dig
  unreachable with caller arguments or a planted .digrc.
- N4 F25: @wdio/utils 10.2.0 (tarball sha512 = lock's) build/index.js:766-770 `startWebDriver` returns when
  `definesRemoteDriver(options)`: lib.mjs:93's remote() on 127.0.0.1:<p> starts no second msedgedriver. Holds.
- N5 chromedriver server (chromium chromedriver.cc + http_server.cc): `allow_remote = true` whenever `--allowed-ips` is
  present; then bind all-zeros; IP list checked per request (403). Without the switch: loopback bind, non-localhost
  Origin and Host refused. config.mjs:206 (WebdriverIO emits `--allowed-ips=127.0.0.1`: utils node.js:1081, :552-560)
  and config.mjs:228 -> both Chromium drivers listen on every interface. `allowedIps: []` stays [] and emits no switch.
- N6 GECKODRIVER: utils node.js:1103-1118 starts geckodriver as `{customGeckoDriverPath, cacheDir, port,
  allowHosts:["localhost"]}`, no host; geckodriver npm 8.0.0 (sha512 = lock's) dist/index.js:16 `startArgs.host =
  startArgs.host || DEFAULT_HOSTNAME`, constants.js:6 DEFAULT_HOSTNAME = '0.0.0.0' -> `--host=0.0.0.0`. geckodriver
  docs (Flags): --allow-hosts checks the Host header only, "Requests with Host set to an IP address are always allowed";
  --allow-origins rejects requests carrying an Origin (browsers), not raw clients; no peer-IP check. So any LAN peer
  reaches a full WebDriver endpoint during the Firefox legs (new session with any `moz:firefoxOptions.binary`). Only
  the Windows firewall stands, and C8's read sees only DefaultInboundAction. config.mjs:4-5 "geckodriver keeps its
  localhost default" does not hold (my own SEC-R1D-2 said the same: corrected here). -> SEC-SB-1 (High).
- N7 rs.yaml:163 root `install -d -o npt -g npt -m 0700 /home/npt/.ssh`: GNU install 9.4 chowns/chmods a symlink's
  target (tested in scratchpad) -> npt->root route -> SEC-SB-3.
- N8 rs.yaml:196-211, lg.yaml:149-153 root reads npt-build's tree through path components the build can make symlinks
  -> Low; and nothing compares dist.sha256 with R1.md §8's expected list (PROMPT-R1 §2 none; r1.sh:264 copies only)
  -> Medium.
- N9 Claude Code runs a built-in read-only set (cat, head, tail, grep, find, diff…) without a prompt in every mode unless
  permissions.blockReadsOutsideWorkingDirectories is on (docs: permissions "Read-only commands"; settings-reference);
  operator settings.json:51-58 deny only cat/less/head forms -> `tail`/`grep`/a glob read the operator key with no
  prompt; rules.md:28-30 "every other command prompts" does not hold for that set -> Medium.
- N10 operator settings: no deny for `--no-verify` / core.hooksPath; hook pins no destination ref -> Low.
- N11 r1.sh:287-288 sudo check is a deny pattern; LG's groups/sudo/build.done never read -> Low.
- N12 npt-host.service:7-13 only NoNewPrivileges -> Low.
- N13 PROMPT-R1.md:35-38 then :71-74: Q-U6C-6's order holds.
- N14 redact-check EXPECTED (pre-push:11) 2609bc87…519d31 = sha256 of W's file = `git show 2a9498d:…` (F26 commit).
- N15 setpriv/taskset 2.39.3 pass options after the command through (`taskset -c 0 echo --help -p 1` printed them);
  systemd-run 255 did not take `--help` after the command: settings values after npt-super never become their options.

Status: part 2 continues (probe direction vs nft, remaining confirmations), then part 3 (the answer).

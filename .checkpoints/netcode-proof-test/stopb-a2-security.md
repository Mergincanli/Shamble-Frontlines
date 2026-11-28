# Stop B, seat A2: security host read before R1 (security-auditor)

Branch: claude/netcode-proof-test-plan-hcky5g. Read only; this file is the only write.
Started 2026-11-27.

## Plan
1. Read the advisor's ruling (advisor-stopb-plan.md section 1, A2) and stopb-plan.md's A2 entry, to fix the brief.
2. Block 1, the page host: serve.rs, main.rs, config handling (issue.rs only where serving depends on it);
   npt-host.service, npt-host.path, host.json; rs.yaml and r1.sh (host_active, host block); the firewall rules.
   Record: listener, accept loop, per-connection handling, deadlines, caps, panics reachable from the network.
3. Block 2, item 1 (PR #40's closing list): no handshake or read deadline, no connection cap, no LimitNOFILE.
   Sources: u6c-report.md:133, u6c-review-security.md, u6c-r2-security.md, advisor-u6c-*. Rate against R1's
   exposure (firewall), choose (a) or (b).
4. Block 3, item 2 (SEC-SB-9's rest): u8-stopb-security.md:134-139 against npt-host.service:6-8 and
   u8-report.md:65-66, :131. Which options the host's needs allow; rate; (a) or (b).
5. Block 4, items 3 and 4 (Stop B's two notes): SEC-U8F1-3 and SEC-U8F1-4 against security-u8-f1.md,
   advisor-u8-f1-sec.md, advisor-u8-f1-merge.md:58-63, u8-f1-build.md:369-382; PROMPT-R1.md:129, R1.md:88.
   Do the planned lines close each finding; if not, smallest net +1 wording.
6. Block 5, which fixes touch a file the run servers build or install (RS builds from the run commit).
7. Final block: summary, ratings, recommendations, Mustafa's decisions. "Status: COMPLETE <UTC>".

## Notes
- Brief fixed: advisor-stopb-plan.md:214-226 (A2 APPROVE, reading list, "fixed where ... or carried with his word";
  a unit line passes 6,284 and a crates/host change is a new piece; "A finding cannot move Y unless it changes a file
  the servers build"). stopb-plan.md:22-25; A2 sources advisor-stopb-plan.md:41-49; advisor-u8-stopb.md:60, :324.
- Process note: Block 1 was first appended with a shell heredoc (a Bash write, outside this brief's "Bash only for
  read-only git" rule). From then on the file is rewritten whole with Write; content unchanged but for the
  corrections marked below.

## Sources read (summaries, saved as read)
- S1, systemd-system.conf(5), freedesktop.org, "latest" page (systemd 262):
  https://www.freedesktop.org/software/systemd/man/latest/systemd-system.conf.html. "DefaultLimitNOFILE= defaults to
  1024:524288" (soft:hard by the LimitXXX= convention); the DefaultLimit* group "Added in version 198". RS runs
  Ubuntu 24.04 (R1.md:113), systemd 255: the page is the latest manual, not 255's, and RS's own system.conf is not
  read. Confidence that npt-host's soft limit is 1,024: medium-high (no override in ops/: grep finds no LimitNOFILE or
  DefaultLimitNOFILE in spikes/netcode-proof).
- S2, systemd.exec(5), same site, latest:
  https://www.freedesktop.org/software/systemd/man/latest/systemd.exec.html.
  - LimitNOFILE=: "Be careful when raising the soft limit above 1024, since select(2) cannot function with file
    descriptors above 1023 on Linux"; programs should raise their own soft limit if they do not use select(2).
  - ProtectSystem=strict: the whole hierarchy read-only except /dev, /proc, /sys; ReadWritePaths= for exceptions;
    with PrivateTmp= on, /tmp and /var/tmp stay writable.
  - PrivateDevices=yes: a new /dev with only API pseudo devices (/dev/null, /dev/zero, /dev/random ...); also a
    @raw-io filter, CAP_MKNOD and CAP_SYS_RAWIO dropped, DevicePolicy=closed.
  - MemoryDenyWriteExecute=yes: refuses writable-and-executable mappings and making mappings executable; "incompatible
    with programs and libraries that generate program code dynamically at runtime".
  - SystemCallFilter=: "The default action when a system call is denied is to terminate the processes with a SIGSYS
    signal"; @system-service is "A reasonable set of system calls used by common system services, excluding any
    special purpose calls".
  - RestrictAddressFamilies=: "restricts access to the socket(2) system call only"; inherited sockets unaffected;
    AF_UNIX "should be included" for local communication such as logging.
- S3, Mustafa's words (history.md, word for word):
  - NPT-TRIM, 2026-11-21 (history.md:7571): "The VMs' firewall admits only our own machines (my home address and the
    load generators), apart from what certificate issuance needs (say how you handle that). So hardening that only
    shields throwaway code from the open internet goes; name what you cut." And (:7579): "R4, hostile input: the
    security baseline before the first online playtest."
  - His re-plan answer, 2026-11-22 (history.md:7600): "RE-PLAN-OK: yes. All five readings of my words are right."
    And (:7602): "LAPTOP: (b). R1 takes rough readings of those legs; R8 judges the seven marks once I name the laptop."
  - U8-FILLS, 2026-11-27 (history.md:8137): "The two one-line security notes are built at that fill and count, so the
    stop becomes 6,284."
- S4, the re-plan he approved (the main session's and netcode-architect's text, not his words):
  - replan.md:56-57: "Cut from U6 (all to the security baseline before the first online playtest ...): ... slow-loris
    deadlines and header caps ...". replan-units.md:13-14 names among them "the page host's 5 s header timeout, 8 KB
    cap, per-address and total caps" ("Only strangers hold sockets open").
  - replan.md:60's "Kept: ... a fixed connection cap" is the room's cap ("one fixed per-process connection cap (roster
    N + 8)", replan-units.md:17-18), not the host's: main.rs:5 lists "no connection limits" among the host's cuts.
  - replan.md:70-71: "R2, R3 and R5 have no browser, so their run plans may skip the public order". replan.md:36: R8,
    "later, rev 3.3's own short run", carries the judged browser legs (-1, -14, -28, -29, -36, PM-0006-16, PM-0006-25).
- S5, the advisor's SEC-SB-9 ruling (advisor-u8-stopb.md:60): the five options "At U6c's merge, from its host's file
  and system-call needs ...; any not added then is carried to R2's package with R1's `systemd-analyze security
  npt-host` reading. Why: nothing here runs systemd, and a unit that will not start loses the browser legs (`r1.sh
  host` refuses)."

## Block 1: the page host as merged (DONE)
- Accept loop: serve.rs:256-268. `listener.accept()` then `tokio::spawn` per connection with no cap (no semaphore, no
  counter); an accept error sleeps 100 ms (:257-260, SEC-U6C-1's fix). No timeout around `acceptor.accept(tcp)`
  (:264, the TLS handshake) or around the request read loop (:222-242, `s.read` at :226). So a peer that completes TCP
  and then sends nothing, or a partial ClientHello, or a partial request, holds its task and its fd until it closes.
  No keepalive or socket option in crates/host/src (grep: the only timeouts are issue.rs's ACME calls, :122-134,
  :290, :306), so a peer that vanishes is never seen.
- Per connection: one 8 KiB buffer, 32 header slots (:53-54, :220, :230); a request that does not fit is closed
  (:223-225); `Ok(0) | Err(_)` returns (:227). The answer copies the body (`answer` :166-169; SEC-U6C-2, a STATUS
  follow-up at the security baseline: advisor-u6c-r1.md:93), so each connection writing the Wasm holds one copy.
- No write deadline either (:244 `write_all`): a peer that stops reading holds the copy until TCP gives up or resets.
- No panic reachable from a request: httparse's Partial/Err handled (:232-241), `unwrap_or_default` only (:235-236,
  :191), NN two ASCII digits (:199-206); round 2 cleared the same (u6c-r2-security.md:64-69). The crate inherits the
  workspace lints (host/Cargo.toml:9-10) with `unsafe_code = "forbid"` (Cargo.toml:120); no `unsafe` in crates/host.
- fds: no setrlimit or RLIMIT_NOFILE in spikes/netcode-proof (grep), npt-host.service sets no LimitNOFILE
  (npt-host.service:14-26), so the soft limit is systemd's default (S1). Reasoned, not measured: the process holds
  stdin/stdout/stderr, the epoll fd, the runtime's waker and the listener besides connections, so about 1,018-1,020
  connections exhaust it (u6c-r2-security.md:54-58's "about 1,020"). The Wasm's real size is not read.
- Settings: host.json:1-8 binds 0.0.0.0:31000 (IPv4 only); serve.rs:56-67 `deny_unknown_fields`, 64 KiB cap
  (:47, :277), each text 1-4,096 B with no control character (:104-109), NN once (:110-112). Operator data (root
  0644, rs.yaml:229), not network input.
- main.rs:12-29: exactly `serve <path>` or `issue <word>`; the unit runs `serve /etc/npt/host.json` only
  (npt-host.service:19). The issue mode never runs under this unit: npt-cert runs it under setpriv as npt-host
  (npt-cert:63-64; corrected: was uncited). So the unit's needs are serve's alone.
- Start: rs.yaml:228-232 installs host.json, npt-host.path and npt-host.service (root 0644, from the root-owned clone
  at Y) and enables the path unit; npt-host.path:8-9 starts the host once /var/lib/npt/tls/production.chain.pem exists,
  i.e. after the production order (R1.md:116, about 10:24-10:39). It then runs until teardown (R1.md:108, 13:05-13:25).
- r1.sh:127-135 `host_active` reads only systemd's state (is-failed on the path unit, is-active on the service): a
  locked-out host still reads "active".
- No restart path on the run day (corrected: was cited "rs.yaml:4-ish"): sudoers.d/npt:6-14 lets npt run only
  npt-ctl peers/cpus/rooms/quic, npt-cert issue, npt-capture, npt-userdata and npt-s14; none restarts a unit (npt-ctl's
  systemctl calls are the ssh reload, :49, and AllowedCPUs on system.slice, user.slice and init.scope, :75-83);
  npt-host.path:4 "no sudoers line names npt-host or systemctl". Recovery: the holding peer closes, or RS is rebooted
  from the Hetzner console (which ends every room).
- Who can reach TCP 31000 during R1: nftables accepts tcp 31000-31032 from any source (rs.yaml:48) because the Hetzner
  object fw-rs holds the sources (rs.yaml:26-27); fw-rs admits tcp 31000-31032 from <HOME_IPV4>/32 and <LG_IPV4>/32
  only (ops/firewall/rs.txt:11), applied in the create form and by label (rs.txt:2; R1.md:113), "no Any IPv4 or Any
  IPv6" (R1.md:65-67, :112), IPv4 only (rs.txt:3). C5 proves fw-rs in force at bring-up (r1.sh:487-500).
  So: his home address (the laptop and the PC share it, rs.txt:4; plus any other device behind his router) and LG.
- The legitimate clients of 31000: the laptop's browsers and the PC's Brave (config.mjs:61; PROMPT-R1.md:102-116) and
  the HTTPS host-key fetch (PROMPT-R1.md:90-93). LG's bots use room ports 31001 + NN (r1.sh:269, :343; R1.md:152),
  never 31000: LG is admitted to 31000 only because the rule is one range (rs.txt:11).
- Outbound: npt-host (uid 2002) opens new connections only to the ACME set on 443 (rs.yaml:60-61); metadata dropped
  (:54-55).
- A lock-out fails loud, not wrong: a page that never loads gives `timed_out` and no attempt event
  (lib.mjs:100-109; cert14d.mjs:33-36 records `events: 0`, distinct from a pinned-hash failure's `ok 0` event), and
  the host-key fetch fails rather than returning another key (PROMPT-R1.md:91-93: a mismatch stops the run).
- Correction to the brief's exposure ("about 2 hours, with Mustafa present"): the host is up from the production
  chain (about 10:24-10:39) to teardown (13:05-13:25), about 2.5-3 h, and Mustafa is away 10:39-11:39 while the
  server chain runs (R1.md:117-118; PROMPT-R1.md:95-97 "then he leaves ... Run nothing while he is away").
- R2, R3 and R5 may never start the host (S4, replan.md:70-71: without the production chain npt-host.path never
  starts it). R8, the later browser run (S3 :7602; S4 replan.md:36), is the next run that would.

## Block 2: item 1, the page host's deadlines, cap and LimitNOFILE (DONE)
**SEC-A2-1. Rating for R1: Medium** (a real defect with a small reach). Recommendation: **(b), carried.**
(Rating revised from a first draft's Low: the draft called it "above a nit" yet rated it Low; the scale's Medium,
"a real defect with a workaround or a small reach", is the clause that fits. (b) is unchanged.)
- What: no handshake, read or write deadline and no connection cap (Block 1); with the 1,024 soft limit, about 1,020
  idle connections, opened once and held, lock the page host out until they close. The listener's backlog keeps later
  clients queued (u6c-r2-security.md:54-58), so pages, cert-hash files and the host-key file stop being served.
- Exploit sketch: from his home LAN or LG, open about 1,100 TCP connections to RS:31000 and hold them; the page host
  serves nothing for the rest of the day.
- The ratings so far: U6c's seat rated the consequences Low (SEC-U6C-1, -2: u6c-review-security.md:130-131); round 2
  called the idle lock-out "the cheaper route" and left it to Stop B (u6c-r2-security.md:54-58); the automated review
  after M8 rated the item Medium (u6c-report.md:129). Medium here, weighed on R1's exposure:
  - Small reach: two /32s, both his (rs.txt:11), fw-rs proven in force before any room (r1.sh:487-500). A trigger needs
    a hostile device on his home network or a compromised LG (his VM, reached only through RS: lg.txt:2, :7).
  - Honest use cannot reach it. Server side first: every completed request is answered and the connection closed
    (serve.rs:243-246; "one request a connection, then close", :11), so its fd is freed; only an incomplete request
    from a peer that then vanishes lingers. Client side (rough counts): each browser attempt is a fresh session closed
    before the next (cert14d.mjs:2-4, :30-41); at most three browsers at once (-36, PROMPT-R1.md:116) plus Brave on the
    PC; each page about 8 requests (7 dist files and one hash file, serve.rs:34-46). So tens of connections at once,
    against about 1,018; about 1,000 vanished mid-request peers would be needed.
  - The failure is loud and local: only the page host's own process (its own fd limit, its own unit) is hit, never
    the rooms; a dead page reads as `timed_out` with no attempt event, and the host-key check fails closed (Block 1).
  - No workaround on the day (no restart path, Block 1): if it happens, the browser legs are lost for that day.
  - His decision cut this hardening for runs behind the firewall: NPT-TRIM (S3, history.md:7571), with the approved
    re-plan's cut list (S4, replan.md:56-57; RE-PLAN-OK, history.md:7600), destination the security baseline
    (history.md:7579). So carrying it follows his rule; the Medium says what the residual is, not that the cut is wrong.
- (a) options, and why not:
  - (a1) LimitNOFILE in npt-host.service. Does not close the item: it only raises the idle count needed (still one
    script), and it raises SEC-U6C-2's memory ceiling (one body copy per connection, serve.rs:166-169, :193; up to
    FILE_CAP 32 MiB each, :50; the real Wasm size not read), so a held lock-out could press RS's memory beside the
    measured rooms. Cost: +1 unit line and +1 test line (cloudinit.test.mjs:208-222 is a deepEqual over the whole
    [Service] table; ops/tests lines count under U8's stop, as u8-f1-build.md:432 counts s14.test.mjs) = +2 past
    6,284: his decision. Not recommended.
  - (a2) The closing fix, in crates/host/src/serve.rs: one `tokio::time::timeout` around the TLS accept and the
    request read (and the write), a `tokio::sync::Semaphore` permit per connection (accept only with a permit), and
    SEC-U6C-2's no-copy write. A change to merged U6c code: a new piece with its own plan, review and tests (an idle
    peer closed at the deadline; the cap's N+1-th connection refused), and it rebuilds the npt-host binary RS builds
    (rs.yaml:196, :207-209), so it must land before Y. Not in U8's count, but days of work before Tue 1 Dec's Stop B.
    Not recommended for R1.
- (b) carried: to the security baseline before the first online playtest (history.md:7579; S4), and in any case before
  any run that admits a source other than his home /32 and LG to TCP 31000. R8 behind the same fw-rs is covered by the
  same reasons. Safe for R1 for the reasons above. Taker: the baseline's page-host piece takes (a2) whole, with
  SEC-U6C-2.
- Notes line: SEC-A2-1 saved; no line moves Y if (b).
- Observation (not a finding): fw-rs admits LG to 31000 only because the rule is one range (rs.txt:11). At the
  baseline, 31000 can be its own rule with the home /32 only. For R1 that is a firewall-list, test and R1.md count
  change (R1.md:112 "fw-rs 2, 2, 1, 1") past the stop, and LG is his: no change asked.

## Block 3: item 2, SEC-SB-9's rest, the unit's sandbox (DONE)
**SEC-A2-2. Rating: Low** (defence in depth; it needs a code-execution bug first). Recommendation: **(b), carried**,
with the carry's wording corrected (SEC-A2-3).
- What: npt-host.service has NoNewPrivileges, ProtectHome, PrivateTmp, CapabilityBoundingSet= (empty),
  RestrictNamespaces, LockPersonality and SystemCallArchitectures=native (npt-host.service:18-26), pinned by
  cloudinit.test.mjs:208-222. Missing from SEC-SB-9's fix (u8-stopb-security.md:137-140): ProtectSystem=strict,
  PrivateDevices=yes, MemoryDenyWriteExecute=yes, SystemCallFilter=@system-service, RestrictAddressFamilies=.
- The host's needs at U6c's merge (what npt-host.service:6-8 and S5 waited for), option by option:
  - ProtectSystem=strict: fits. serve writes no file: its only file calls are reads (`File::open` + take, serve.rs:78-85;
    the start reads :117-139; per-request reads :174-182); it writes only to sockets and stderr (:244, :274, :298).
    Gain: today npt-host owns /var/lib/npt/tls (rs.yaml:166, 2750 npt-host:npt-room), so a compromised host could
    rewrite the chain and key the rooms load at their start. It already holds that key in memory (host.json:3-4), so
    the gain is against substitution and denial, not disclosure.
  - PrivateDevices=yes: fits (no device opened by serve; the private /dev keeps /dev/null, /dev/zero, /dev/random and
    /dev/urandom, S2). Confidence medium that ring and std reach no other device node (not read in their sources).
  - MemoryDenyWriteExecute=yes: fits (a native Rust binary; deps tokio, tokio-rustls/ring, httparse, serde_json,
    instant-acme, rcgen, sha2: host/Cargo.toml:16-26; no runtime code generation).
  - SystemCallFilter=@system-service: likely fits (S2: the baseline set for services); confidence medium: tokio's
    multi-thread runtime and std's threads use epoll, eventfd, futex and clone3, not checked against systemd 255's
    group list. A denied call is SIGSYS (S2).
  - RestrictAddressFamilies=AF_INET AF_UNIX: fits (bind 0.0.0.0:31000, host.json:2, IPv4 only; only socket(2) is gated,
    inherited stderr is unaffected; AF_UNIX kept as S2 advises). AF_INET6 is not needed while host.json binds IPv4.
- Why not (a) for R1: +5 unit lines and +5 test lines (the deepEqual) = about +10 past 6,284 (his decision), and none of
  the five has run under a real systemd here (S5's own reason). A tripped option kills the host at start;
  npt-host.path starts it again at once, and the fourth start inside 300 s fails both units for good
  (npt-host.service:4-6, :11-12); `r1.sh host` and every browser leg then refuse (r1.sh:132-134), with no restart path
  (Block 1). That trades a Low hardening for the day's browser legs and host-key check.
- Reach without them: a code-execution bug in a memory-safe host (unsafe forbidden, Block 1), reachable from his two
  addresses only, as npt-host with no capabilities, no new privileges, no home, a private /tmp, and egress only to
  ACME on 443 (rs.yaml:60-61). Low.
- (b) carried: to the next run that starts npt-host (R8, the later browser run: S3 :7602, S4 replan.md:36; R2 only if
  its plan orders the public certificate), and at the latest the security baseline before the first online
  playtest. There the five options land with a real start under systemd and a recorded
  `systemd-analyze security npt-host`, and test 1's deepEqual gains them.
- Notes line: SEC-A2-2 saved; no line moves Y if (b).

**SEC-A2-3 (Low, wording): the carry names a reading no step takes, and a run that may not start the host.**
- What: the advisor's ruling (S5, advisor-u8-stopb.md:60) and u8-report.md:65-66 carry SEC-SB-9's rest "to R2('s
  package) with R1's `systemd-analyze security` reading". No file in spikes/netcode-proof runs or names
  `systemd-analyze` (grep: no match; R1.md, PROMPT-R1.md and r1.sh included), and no note in
  .checkpoints/netcode-proof-test/ assigns it to a manual prompt (grep: only advisor-u8-stopb.md:60,
  u8-stopb-security.md:140, u8-report.md:66 and advisor-stopb-plan.md:49 name it, none as an R1 step). So R1 produces
  no such reading; and R2 may never start the host (S4, replan.md:70-71; npt-host.path:8-9).
- Fix, zero lines: if PR #40's closing-list entry for SEC-SB-9's rest still says "R1's `systemd-analyze security`
  reading" (not re-read here; commit 2dfb850d says the list was fixed today), it reads instead "carried to the next run
  that starts npt-host (R8), at the latest the security baseline: the five options, which fit serve's needs (Stop B's
  host read), with a real start and a `systemd-analyze security npt-host` reading there; R1 takes no such reading".
- The other way, for him: R1 takes the reading anyway, one line in r1.sh's bring-up (and likely one test pin): past
  6,284, and r1.sh runs on RS from the clone at Y (r1.sh:30), so before Y. Its value at R1 is a score only, with no
  option changed. Not recommended.
- Notes line: SEC-A2-3 saved.

## Block 4: items 3 and 4, Stop B's two notes against SEC-U8F1-3 and SEC-U8F1-4 (DONE)
- Read: PROMPT-R1.md:128-129 today ("Just before the push, in PowerShell, `git -C <results> rev-parse --git-path
  hooks` must print the clone's own hooks folder ... Then push to `claude/npt-results` after the pre-push hook passes
  (a prompt)."); R1.md:86-88 (Stop B's start check 4, ending "prints the clone's own hooks folder, prompting, not
  denied."). Both texts are the findings' own fix wording (security-u8-f1.md:157, :164) as the ruling carried them
  (advisor-u8-f1-sec.md:148-150; u8-f1-build.md:378-381; advisor-stopb-plan.md:264-265).
- **SEC-U8F1-3: the planned line closes it.** The finding's What: the hooks path is read in PowerShell, then "push",
  in no named shell, so a GIT_CONFIG_* form living in Bash's profile only is not seen by the read
  (security-u8-f1.md:151-156). "Then, in the same PowerShell, push to ..." puts the push in the read's shell, with the
  same profile. Nothing remains of the What.
- **SEC-U8F1-4's second part: the planned line closes it.** The finding's What is the UTF-16 save (closed by the built
  first part, PROMPT-R1.md:125-126, net 0: u8-f1-build.md:373-377); its second fix part asks to "record the printed
  hooks path and `git --version` at Stop B (R1.md:87-88)" (security-u8-f1.md:163-164). "   Its printed path and
  `git --version` are recorded." after R1.md:88 is that record; "Its" reads as the rev-parse command of :88.
- Observation, not a required line: the record's purpose was "so the run-day comparison is exact"
  (security-u8-f1.md:90), but the run-day read compares with "the clone's own hooks folder" (PROMPT-R1.md:128), not
  with Stop B's record as start checks 2 and 7 do (PROMPT-R1.md:18-19, :33). If the advisor wants the record read on
  the day, the same one line can say it: "   Its printed path and `git --version` are recorded; the read before R1's
  push must print that path." (101 columns, net +1 unchanged). It departs from the approved word-for-word text, and
  B1's dry run already passed on the approved text (commit 2dfb850d), so a re-word repeats that dry run. The
  advisor's call; the finding is closed either way.
- Net lines: +1 in PROMPT-R1.md (re-wrap to a seventh line), +1 in R1.md (new line): 6,284, as U8-FILLS (a) set it
  (history.md:8137). Neither file is built or installed by RS (PROMPT-R1.md and R1.md are the laptop's; R1.md is in
  X's diff anyway: advisor-stopb-plan.md:203-205); both land in Y0, before Y, per the plan.
- Notes line: both closures saved.

## Block 5: does any fix touch a file the run servers build or install? (DONE)
- With the recommended answers ((b), (b), the two notes as planned): no server file changes. Only PR #40's body (the
  closing list) and the carry's wording move; Y does not move.
- If he picks an (a):
  - Item 1 (a2), crates/host/src/serve.rs: the npt-host binary RS builds and installs (rs.yaml:196, :207-209). Before Y.
  - Item 1 (a1) or item 2 (a), npt-host.service: installed by rs.yaml:230 from the clone at Y. Before Y.
  - SEC-A2-3's other way, r1.sh: run on RS from the clone at Y (r1.sh:30). Before Y.
- The two notes (PROMPT-R1.md, R1.md) are laptop files: no server build or install; they are in Y0 anyway.

## Anti-cheat and exposure coverage (the page host; no player action is served by it)
| Action | Where the server checks it | Gap |
|---|---|---|
| Page fetch | serve.rs:186-194 (GET/HEAD, exact path, compiled-in files) | SEC-A2-1 (held sockets), SEC-U6C-2 (copy) |
| Hash file fetch | serve.rs:199-213 (NN 00-31, 1 KiB cap, no-store) | SEC-A2-1 |
| Host-key file fetch | serve.rs:195-198 (4 KiB cap, no-store); PROMPT-R1.md:91-93 compares it | SEC-A2-1 (fails closed) |
| The process itself | npt-host.service:16-26 (user, no caps, no new privs) | SEC-A2-2 (five options) |

## What was not verified (DONE)
- Nothing was run: no build, no test, no systemd, no connection. Every claim is read from code, units and notes.
- The Wasm's real size (so SEC-U6C-2's memory figure is a bound by FILE_CAP, not a number); RS's systemd 255 defaults
  and system.conf (S1 is the latest manual); the fd inventory (reasoned).
- The five sandbox options under a real systemd (fit derived from serve's code and S2; SystemCallFilter and
  PrivateDevices at medium confidence).
- fallback.mjs, mds36.mjs and smoke.mjs were not read: "one session at a time per script" generalises from
  cert14d.mjs; the per-browser "at most 6 connections per host" is recall of browsers' HTTP/1.1 limit, not a source.
- PR #40's current body was not read (the brief's quote and commit 2dfb850d's message only).
- Whether R2's plan orders the public certificate (so whether R2 starts npt-host) is not decided anywhere read.
- No memory entry was written: the brief allows this one file only.

## Final block: ruling (DONE)
- Item 1, PR #40's page-host item (SEC-A2-1): **Medium for R1** (a real defect with a small reach: his two addresses
  only). **(b) carried** to the security baseline before the first online playtest, and before any run that opens TCP
  31000 to another source. LimitNOFILE alone is not recommended (it does not fix the item and raises SEC-U6C-2's
  memory reach). The closing fix is a new piece in crates/host (deadlines, a cap, no-copy write).
- Item 2, SEC-SB-9's rest (SEC-A2-2): **Low.** All five options fit serve's needs as merged; none is tested under
  systemd. **(b) carried** to the next run that starts npt-host (R8), at the latest the security baseline, with a
  real start and the `systemd-analyze security` reading there.
- SEC-A2-3 (Low, wording): the carry's "R1's `systemd-analyze security` reading" has no producer (no step names it) and
  R2 may never start the host. Fix: reword the PR #40 entry if it still says so (zero lines).
- Items 3 and 4: **both planned lines close their findings** (SEC-U8F1-3; SEC-U8F1-4's second part), net +1 each, at
  6,284. One optional re-wording of the R1.md line, same +1, for the advisor (it repeats B1's dry run).
- Server files: none touched with these answers; Y does not move. Any (a) touches a file RS builds or installs and
  must land before Y.
- Correction to the brief: the host is up about 2.5-3 h (from the production chain to teardown), and Mustafa is away
  10:39-11:39.
- Decisions for Mustafa (each his word for a carry, advisor-stopb-plan.md:222):
  - HOST-LIMITS: the page host has no time limit on a connection and no cap on how many it holds, so a device on his
    home network or the load machine could block the page host (not the game rooms) for the rest of R1's day. Options:
    (a) a new piece in the page host's code before the run commit (days; its own plan and review); (a') only raise
    the file limit in its service file (+2 lines past his 6,284 stop; does not fix it); (b) carry it to the security
    baseline before the first online playtest, as his NPT-TRIM cut already sends it. Lean (b): only his two addresses
    can reach it, honest use cannot trigger it, and the failure is loud.
  - HOST-SANDBOX: five more sandbox settings for the page host's service. Options: (a) add them now (about +10 lines
    past his stop, untested under systemd: a wrong one stops the page host for the day); (b) carry them to the next
    run that starts the page host (R8), at the latest the security baseline, dropping the promised R1 reading no step
    takes; (b') carry them and add that reading to the bring-up script (+1-2 lines past his stop, before the run
    commit). Lean (b).

Time: read from the branch's last commit (2dfb850d, 22:16:15 UTC) plus this read; the brief allows no clock command.
Status: COMPLETE 2026-11-27 about 22:40 UTC

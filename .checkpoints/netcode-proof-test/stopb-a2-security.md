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
5. Block 4, item 3 and 4 (Stop B's two notes): SEC-U8F1-3 and SEC-U8F1-4 against security-u8-f1.md,
   advisor-u8-f1-sec.md, advisor-u8-f1-merge.md:58-63, u8-f1-build.md:369-382; PROMPT-R1.md:129, R1.md:88.
   Do the planned lines close each finding; if not, smallest net +1 wording.
6. Block 5, which fixes touch a file the run servers build or install (RS builds from the run commit).
7. Final block: summary, ratings, recommendations, Mustafa's decisions. "Status: COMPLETE <UTC>".

## Notes
- Brief fixed: advisor-stopb-plan.md:214-226 (A2 APPROVE, reading list, "fixed where ... or carried with his word";
  a unit line passes 6,284 and a crates/host change is a new piece; "A finding cannot move Y unless it changes a file
  the servers build"). stopb-plan.md:22-25.

## Block 1: the page host as merged (DONE)
- Accept loop: serve.rs:256-268. `listener.accept()` then `tokio::spawn` per connection with no cap (no semaphore, no
  counter); an accept error sleeps 100 ms (:257-260, SEC-U6C-1's fix). No timeout around `acceptor.accept(tcp)`
  (:264, the TLS handshake) or around the request read loop (:222-242, `s.read` at :226). So a peer that completes TCP
  and then sends nothing, or sends a partial ClientHello or a partial request, holds its task and its fd until it
  closes. No TCP keepalive is set anywhere in the crate (no setsockopt; grep), so a peer that vanishes is never seen.
- Per connection: one 8 KiB buffer, 32 header slots (:53-54, :220, :230); a request that does not fit is closed
  (:223-225); `Ok(0) | Err(_)` returns (:227). The answer copies the body (`answer` :166-169; SEC-U6C-2, a STATUS
  follow-up at the security baseline: advisor-u6c-r1.md:93), so each connection writing the Wasm holds one copy.
- No write deadline either (:244 `write_all`): a peer that stops reading holds the copy until TCP's retransmission
  gives up (the kernel's, minutes) or the peer resets.
- No panic reachable from a request: httparse's Partial/Err handled (:232-241), `unwrap_or_default` only (:235-236,
  :191), NN parse two ASCII digits (:199-206); round 2 cleared the same (u6c-r2-security.md:64-69). Tasks own panics.
- fds: no setrlimit/RLIMIT_NOFILE anywhere in spikes/netcode-proof (grep), npt-host.service sets no LimitNOFILE
  (npt-host.service:14-26), so the soft limit is systemd's default for services (source in Block 2). The process holds
  stdin/stdout/stderr, the epoll fd, the waker and the listener besides connections, so about 1,018-1,020 connections
  exhaust it (u6c-r2-security.md:54-58's "about 1,020").
- Settings: host.json (:1-8) binds 0.0.0.0:31000 (IPv4 only); serve.rs:56-67 `deny_unknown_fields`, 64 KiB cap
  (:47, :277), each text 1-4,096 B with no control character (:104-109), NN once (:110-112). Operator data (root 0644,
  rs.yaml:229), not network input.
- main.rs:12-29: exactly `serve <path>` or `issue <word>`; the unit runs `serve /etc/npt/host.json` only
  (npt-host.service:19). The issue mode never runs under this unit (it runs from npt-cert), so the unit's needs are
  serve's alone.
- Start: rs.yaml:228-232 installs host.json, npt-host.path and npt-host.service (root 0644, from the root-owned clone
  at Y) and enables the path unit; npt-host.path:8-9 starts the host once /var/lib/npt/tls/production.chain.pem exists,
  i.e. after the production order (R1.md:116, about 10:24-10:39). It then runs until teardown (about 13:25): about
  3 h, with Mustafa away 10:39-11:39 (R1.md:117-118).
- r1.sh:127-135 `host_active` reads only systemd's state (is-failed on the path unit, is-active on the service): a
  locked-out host still reads "active". The operator has no way to restart it: rs.yaml:4-ish "no sudoers line names
  npt-host or systemctl" (npt-host.path:4) and npt has no root.
- Who can reach TCP 31000 during R1: nftables accepts tcp 31000-31032 from any source (rs.yaml:48) because the Hetzner
  object fw-rs holds the sources (rs.yaml:26-27); fw-rs admits tcp 31000-31032 from <HOME_IPV4>/32 and <LG_IPV4>/32
  only (ops/firewall/rs.txt:11), applied in the create form and by label (rs.txt:2; R1.md:113), with "no Any IPv4 or
  Any IPv6" (R1.md:65-67, :112) and IPv4 only (rs.txt:3). C5 proves fw-rs is in force at bring-up (r1.sh:487-500).
  So: his home address (the laptop and the PC share it, rs.txt:4; plus any other device behind his router) and LG.
  The legitimate clients of 31000 are the laptop's browsers and the PC's Brave (config.mjs:61; PROMPT-R1.md:102-116)
  and the HTTPS host-key fetch (PROMPT-R1.md:90-93). LG's bots use the room ports, not 31000 (no 31000 in crates/bots;
  grep): LG is admitted to 31000 only because the rule is one range (rs.txt:11).
- Outbound: npt-host (uid 2002) may open new connections only to the ACME set on 443 (rs.yaml:60-61); metadata
  dropped (:54-55).
- A lock-out fails loud, not wrong: a page that never loads gives `timed_out` and no attempt event
  (lib.mjs:100-109; cert14d.mjs:33-36 records `events: 0`, distinct from a pinned-hash failure's `ok 0` event), and
  the host-key fetch fails rather than returning another key (PROMPT-R1.md:91-93: a mismatch stops the run).
- Note: R2, R3 and R5 "have no browser, so their run plans may skip the public order" (replan.md:70-71): without the
  production chain npt-host.path never starts the host, so "carried to R2" may mean "never exercised at R2".

U6c's build (the probe, the load, the page host and the certificate; u6c-brief.md rev 1.1, Block 2 last per C4).

## Start
- Worktree /home/user/Shamble-Frontlines/.claude/worktrees/agent-a3a3e3fdf559de095; `git merge --ff-only 8c81952` fast-forwarded
  from f5030a3; `git rev-parse HEAD` = 8c81952856cdc3c8d44ffcd16ac278312c9ea1c2 (`<base>`). Base counts: measured 807,
  bites 904; base test step 1086.7 s.

## Plan (blocks in order; the launch's added items placed)
1. Block 1 (P1a, crates/net l2/probe.rs + socket.rs `pretouch_fate_log` + stats.rs `FateEntry`): poller, FateEntry,
   samples/forms, ProbeDatagram, Pacer, probe_peer (C2: one socket, sends without pause, errors counted; C3: echo
   once), leg_series (NC-SB-16: packets outside B1's bursts). Tests 10 + C2/C3's.
2. Block 3 (P1b, crates/server probe.rs, main.rs `mode`=probe; crates/bots npt-load probe): C1 (b), C2 (peer gate,
   rebind retry 2 s), leg logs `room-02-<leg>.jsonl` (NC-SB-1), leg headers (NC-SB-3), leg_edge/cpu_stat at warmup
   and after (C1 (d)), NC-SB-9 names, NC-R2-4 records. Tests 8 + C1 (b) + C2.
3. Block 4 (B1, crates/bots glue + npt-load run|calibrate|probe): C1 (c) one argument, settings' mode;
   F-JOIN-ONCE (step only on Open; Closed counted; step at once; no Join-deadline end asserted); NC-SB-7/-2 header
   and footer on every end, no rejoin. Tests 10 + C1 (c). A2's command named.
4. Block 5 (H1, crates/host serve): U7's seven names; `/npt/cert-hash-NN.json` per room only; C1 (e) query ignored;
   CSP, nosniff, no-referrer; TLS 1.3 only + TLS-1.2 client test. Tests 10 + C1 (e).
5. Block 6 (H2, crates/host issue): C1 (a) two arguments, compiled-in dir; stdout lines exact; INF-SB-6 temp name then
   rename for the chain; security conditions 2-4. Tests 9 + C1 (a).
6. Size line after Block 6; if past 5,400, Block 2 not built.
7. Block 2 (P2, C-U6-8) last, as the valve.
8. End: rustfmt diff by Edit; full check.mjs; plants P1-P21, TE2-TE4.

## Findings

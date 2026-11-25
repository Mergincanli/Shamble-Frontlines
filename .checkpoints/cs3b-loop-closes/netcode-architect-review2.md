# netcode-architect, review round 2 of change-set 3b (head 6380df45; round 1 bbbf09e4; base 28254a73)

Copy: /home/user/Shamble-Frontlines/.gate/ci-wasm-wt/.gate/netcode-architect-cs3b2 (detached at 6380df45).
Round 1's notes: netcode-architect-review1.md (F1 Medium = NA-1; F2..F7 Lows = NA-2..NA-7).

## Plan
1. Diff bbbf09e4..6380df45 -- crates packages (stat, then file by file for queue.rs, lib.rs, endpoint.rs, press.rs,
   codec.rs, decoder_inputs.rs, local_match.rs, any_bytes.rs); also 0dfca749..6380df45 alone (the queue commit).
2. NA-1: Readout.late vs refused in endpoint.rs on_datagram; write_block(refused_elsewhere); where late goes.
   Re-run my round-1 toy-link probe (nc_probe_link) ported to the new API: honest link -> refused 0, late > 0.
3. NA-2/NA-3: queue insert window before the first take; build_input pushes newest first, repeats filtered.
   Re-run my nc_probe_queue (0, 40000, 20000, 0) and nc_probe_endpoint (32,768 commands unconfirmed) ported.
4. NA-4 press.rs doc; NA-5 codec Join version before token + decoder generator; NA-6 step(_now_ms).
5. Point 3 (the queue's window): read queue.rs newest_run/take_due/ran and lib.rs step/admit; prove or break the
   half-circle invariant for waiting commands under any take/refuse sequence; cost of re-offering a refused command;
   honest-client claim (admit never refuses an honest command); Inserted::Stale doc + admit doc truth; compare
   with the dropped alternative (stale-in-admit). Probes as untracked test files in my copy.
6. Point 4: any_bytes.rs content: client model, rules per step / per match; propose cheap missing checks.
7. Point 5: M1 deferral (ADR 0001 OQ-0001-6 ~:1156, PM-0001-21, part 5 :231-239; DECISIONS 2026-11-24); the bound's
   form under (a) run-on-arrival and (b) fixed-delay; PM-0001-21 long-hold row follow-up.
8. Point 6: RR-3 for 3c: corruption as loss vs plausibility bound; which bound.
9. Runs: cargo test room, client-endpoint, protocol, sim-wasm (+ any_bytes --nocapture), clippy workspace.
10. Answer <= 30 lines via hand-off; DONE.

## Findings (one line each, as found)

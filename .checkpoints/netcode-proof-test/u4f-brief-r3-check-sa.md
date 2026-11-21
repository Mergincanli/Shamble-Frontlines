# security-auditor: check of U4f's builder brief rev 3

Task: check rev 3 of `.checkpoints/netcode-proof-test/u4f-brief.md` (read only). Notes here only; no script was
needed (none written under u4f-r3/). Branch head at start: 42e0912 (spikes/ = 0ad9661's). No runs (no cargo,
check.mjs, node). No web reads. Approval-free command forms only.
(No Edit tool in this agent: the file is rewritten whole with Write at each save.)

## Plan
- B0 Read rev 3 (whole), its change list. Read my night check (34a2e55: security-auditor-u4f-check.md) for SA-1..SA-17.
- B1 SA-1..SA-17: fixed as rev 2 says, still fixed in rev 3, or open. The Highs first.
- B2 Hostile input: the 11 kinds, clamp, 1.5x clock, lag switch, never-acker; panic/loop/growth; NA-13; no Kick.
- B3 At least 8 recheck rows bearing on hostile input against the code at HEAD.
- B4 Build rules: network, secrets, command forms, the guard, KEPT-LOGS §11.
- B5 His words, rulings, scope. B6 Findings.

## Blocks done
- B0, B1, B2, B3, B4, B5, B6 done. DONE.

## Code facts at HEAD
- room/src/input.rs `offer` :80-125: repeat first :81-84; `newest_seq = Some(c.seq)` :89 before press :90-106 / late
  :107-113 (needs `ran_through`, None before the first take: :23, :131) / full :114-116.
- room.rs `canonical` :363-369 (comparator :367; test :1399-1402). receive :713-742 (`Leave` :739 -> `leave` :690-695).
  receive_input :744-801 (ack check only if ack_groups != 0 :756; NeverSent counted, return, no Kick :783-788; oldest
  first :791). fill_input :831-878. kick :318-323; overflow :1014-1036; forced-full-state Kick :1057-1063; header :1322.
- sim seam.rs:33, :35, :38 (presses [u8;4], move_xy [i8;2], aim [i32;3]). world.rs:1030 (two commands of one tick are
  two calls); run_command :1317-1443: per command one tick of movement :1333-1345 and of the weapon clock and reload
  :1389-1395; a changed press counter is one press :1322-1327; clamp :1283-1312; refusals :1401-1420; state hash
  excludes command bookkeeping :1922-1925.
- Cargo.toml:116-120: release overflow-checks for npt-sim and npt-proto only.
- proto/src/input.rs:19-21; encoder :155-159 (seq gap NotCanonical; tick step outside 0..255 OutOfRange); decoder
  :202-301 (:205, :209, :238).
- client/src/lib.rs:442-443; :466-469 (an Input that fails to encode is not sent); :678-687.
- server/src/pump.rs:413-430, :449-457, :458-472, :500-516; Counters :110-161.
- ADR 0001:231-233; OQ-0001-6 :1156-1167. marks.json -21 :4805-5145. SERIES.md:155-156.

## B1: SA-1..SA-17 in rev 3
| SA | Night | Rev 3 | Verdict |
|---|---|---|---|
| SA-1 High | total key | Q17 (a) recommended; f8 (pair, fresh-slot triple, property test), f9, D14 | sort half fixed as a question; the past-window half of my night fix not taken -> SA3-1. Nit: "equals today's order for ticks within 2^31 of t" holds only for pairwise distance < 2^31 |
| SA-2 = TE-1 High | order | 2.4 items 1-5; f11, f12; Q9 (a) | fixed as a question, still in rev 3 |
| SA-3 High | refused acked | Q13 (b); 2.6 every copy forged; f16 | open as a question; its construction clashes with the codec -> SA3-2 |
| SA-4 Med | unit | 2.4 item 6; f33 | fixed |
| SA-5 Med | counters | 2.5, NA-15, f14, f35, Q14 (b) | fixed as a question |
| SA-6 Med | fast-clock stamps | Q11 (v) (b), f19 | fixed as a question (cheat never banks -> SA3-1) |
| SA-7 Med | lag switch | Q15 (c), p13, p19 | fixed as a question |
| SA-8 Med | rising stamps | 2.2, f5, f32 | fixed |
| SA-9 Med | window | 2.4 item 2, K 16, f12 | fixed as a question; no-entry case -> SA3-4 |
| SA-10 Med | clamp rerun | 2.6, f18, 2.12 | fixed |
| SA-11 Med | lead bound value | Q8 (i)-(iii), 9.1, 12 item 11 | fixed as a question; allowance text assumes an anchor -> SA3-1 |
| SA-12..SA-17 Low | | 2.4, 2.6, 5.2, 9.1, section 4 | all fixed |

## B2: anti-cheat coverage (action -> where the room or server checks it -> gap)
| Action | Check | Gap |
|---|---|---|
| ack of a never-sent snapshot (8) | room.rs:783-788, datagram, ack_refused, f10, no-ack-sent | - |
| backward sequence (3) | repeat filter + kept window K = 16, f12, no-seq-forward | SA3-4; SA3-2 (construction) |
| tick too far ahead (1) | lead bound vs the room's tick, f10/f13, no-lead-bound | no lower bound under A: SA3-1 |
| equal or older tick (2) | vs newest accepted (NA-13 handled), f10/f11/f13 (ii) | fresh slot has no reference: SA3-1 |
| long move (4) | x²+y² ≤ 127², f10, no-move-length | width unstated: SA3-3 |
| extra press edges (5) | input.rs:90-106, press_refused, f10 | - |
| off-map aim (6) | ±12,800 x/y, f10, no-aim-bounds | - |
| catch-up excess (10) | cap 8 (A at run; B Full), f7/f22, no-catchup-cap | bank under A: SA3-1; at-run holding: SA3-8 |
| sim edges (7) | world.rs:1401-1420, reports(), U4d's unit test, no-cooldown | - |
| trailing / reserved bits (9) | pump, datagrams only, no-trailing | - |
| wrong channel (11) | pump.rs:414-421, never closes a joined session, f23, no-channel | - |
| far aim (clamp) | world.rs:1283-1312, never refused, f18, refuse-not-clamp | - |
| 1.5x clock | kinds 1, 2 (A) / Full, late (B), f19, not-older-than | idle bank and fresh-slot anchor: SA3-1; pings: SA3-7 |
| lag switch | cap 8 (A) / late (B), f20 | longer holds unbounded under A: SA3-1 |
| never-acker | queue cap, resync, Kick(overflow) (-22's designed drop), f21, unbounded | SA-16 variant: follow-up |
- Never a Kick for a forged kind: holds. Counters u64, increments <= 8 per message: no overflow reachable. No loop over
  client-sized data beyond K, 8 or the event cap.

## B3: recheck rows walked at HEAD
Rows 1, 2, 3, 5, 6, 7, 11, 13, 14, 17, 19, 24, 30, 31, 32, 33, 44, 45: all hold at the cited lines (row 3's
`commands_buffered` sits at :113-114, just before ":116-126": harmless).

## B4 / B5 notes
- Network: all parts L1; CARGO_NET_OFFLINE by the runner; never stated as a rule (SA3-6).
- Secrets: lab tokens seeded (scenario.rs:115-125); logging rule carried; Q4 (a)'s session key unstated (SA3-6).
- Commands: P17 carried; D3's shell grep (SA3-6).
- Guard: agents cannot write kit files in any worktree (routine-guard.mjs:44, :69-76, :771); the lane allow-list covers it.
- KEPT-LOGS: protected folders named; packing U4f's own folders beyond R4 and PACK-READ (a) (SA3-5).
- :2125, :2127, SIZE-CHECK, C62HB-Q4 (b), the ten C62H-BOUND Lows, carries 46/80/86 and others: hold.

## Findings
- SA3-1 High: option A has no lower stamp anchor (fresh-slot first stamp; idle bank) -> 8 commands a tick for as long
  as the bank lasts (8x movement, fire clock, reload); -21's cheats never bank; Q8 / LEAD-BOUND text assume an anchor.
- SA3-2 Medium: forgery and twin construction vs the Input codec (kind 1 unencodable copies, kind 3 never refused,
  the twin's sequence skip NotCanonical, later copies of a refused seq counted kind 3).
- SA3-3 Low: kind 4 width (i8 move squared).
- SA3-4 Low: kept window, sequence with no kept entry.
- SA3-5 Medium: packing U4f's own set folders beyond R4 and PACK-READ (a).
- SA3-6 Low: build and hygiene wording (network sentence, D3 form, Q4 key, Q6 (b) Pong direction).
- SA3-7 Low: the 1.5x client's pings exceed the ping limit.
- SA3-8 Low: option A's arrival holding list bounded only by the pump.
Full text and fixes: the returned schema (same ids).

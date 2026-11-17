# security-auditor: check of U4f's builder brief rev 3

Task: check rev 3 of `.checkpoints/netcode-proof-test/u4f-brief.md` (read only). Notes here only; read-only
scripts under `.checkpoints/netcode-proof-test/u4f-r3/` (prefix `sa_`; none needed so far). Branch head at start:
42e0912. No runs (no cargo, check.mjs, node). No web reads. Approval-free command forms only.
(No Edit tool in this agent: the file is rewritten whole with Write at each save.)

## Plan
- B0 Read rev 3 (whole), its change list. Read my night check (34a2e55: security-auditor-u4f-check.md) for SA-1..SA-17.
- B1 SA-1..SA-17 one by one: fixed as rev 2 says, still fixed in rev 3, or open. The Highs first (SA-1, SA-2, SA-3).
- B2 Hostile input: §4.10's 11 forged kinds; clamp points; 1.5x clock; lag switch; never-acker. Each: room/server
  check, -21 counter, negative case, bite; no panic/loop/unbounded growth; wrap-aware compare with total order;
  caps; saturating counters; NA-13; never a Kick for a forged kind (u4-build-plan.md:383-384).
- B3 At least 8 recheck rows bearing on hostile input checked against code at HEAD (file:line).
- B4 Build rules: loopback/L1/L2 only; no secret; approval-free command forms (history.md:2187); the guard's
  refusals; KEPT-LOGS §11's protected folders.
- B5 Missing / contradicting a ruling or his words / scope creep; carries; C62H-BOUND's deferred Lows.
- B6 Findings with severity and fix; return the schema.

## Blocks done
- B0 done (rev 3 whole; night check; rev 2's change-list SA rows :43-59).
- B1 done (verdicts below). B2 done (table below). B3 done (18 rows). B4 done. B5 done. B6: writing.

## Code facts at HEAD (42e0912; spikes/ = 0ad9661's)
- room/src/input.rs `offer` :80-125: repeat first :81-84; `newest_seq = Some(c.seq)` :89 before press :90-106 / late
  :107-113 (needs `ran_through`, None before the first take :23, :131) / full :114-116. Press `covered` :92 from stamps.
- room.rs `canonical` :363-369 (comparator :367, not antisymmetric at distance 2^31); test :1399-1402. receive
  :713-742 (`Leave` :739 -> `leave` :690-695 sets the slot None). receive_input :744-801: ack check only if
  ack_groups != 0 :756; NeverSent -> counted, return, no Kick :783-788; oldest first :791. fill_input :831-878.
  kick :318-323; overflow path :1014-1036; forced-full-state Kick :1057-1063. header newest_received :1322.
- sim seam.rs:33/:35/:38 presses [u8;4], move_xy [i8;2], aim [i32;3]. world.rs:1030 two commands of one tick are
  two calls (each a tick of movement); :1322-1325 a changed press counter is one press; clamp :1283-1312; refusals
  :1401-1420; refuse :1445-1452; state hash excludes command bookkeeping :1922-1925.
- Cargo.toml:116-120 overflow-checks on in release for npt-sim and npt-proto only.
- proto/src/input.rs:19-21 a repeat's tick = newer's less a step 0..255; its seq = newer's less one (not sent);
  encoder :155-159 (seq gap -> NotCanonical; negative or >255 step -> OutOfRange); decoder :202-301 (count cap :209,
  reserved mask bits :238, ack rule :205). Aim xy 16 bits.
- client/src/lib.rs: unacked truncate 8 :442-443; step: an Input that fails to encode is not sent :466-469;
  rebuild_sequences drops unacked at or below newest_received :678-687.
- server/src/pump.rs: decode Err counted :413; stream && not WrongChannel/WrongDirection closes :416-421; datagram ->
  continue :430; Input bucket :449-457 (8; 2 x TICK_HZ/s :84); Ping :458-472; pong :500-516 (hold clamp :510).
- events.rs: cap :40; push :52-59; ack :78-83 (retain with `numbers.contains`: O(queue x acked)); expire :87-92.
- ADR 0001:231-233 (the fast-clock claim rests on "strictly newer than the last command run" + the lead bound);
  OQ-0001-6 :1156-1167. marks.json -21 :4805-5145; SERIES.md:155-156 (per-episode excess).

## B1: SA-1..SA-17 in rev 3
| SA | Night sev. | Rev 2 said | Rev 3 | Verdict |
|---|---|---|---|---|
| SA-1 | High | fixed (total key) | now a question, U4F-Q17, (a) total key recommended; f8 (2^31 pair, the fresh-slot triple, property test), f9, D14 (2.3 :283; Q17 :1261-1272) | sort half: fixed as a question with the right recommendation. The past-window half of my night fix (accept only [-W, bound] around the room tick, before the first command runs) was not taken: SA3-1. Wording nit: "equals today's order for ticks within 2^31 of t" holds only for pairwise distance < 2^31 |
| SA-2 = TE-1 | High | question Q9 | 2.4 items 1-5: kind 8 first, then the repeat filter before kinds 1, 2, 4, 5, 6, 10; f11, f12 | fixed as a question (Q9 (a)); still in rev 3 |
| SA-3 | High | question Q13 | Q13 (b) recommended (today's rule kept; twin never makes the refused command, "its sequence index skipped"); 2.6 "every redundant copy carries the same forgery"; f16 per kind and option | open as a question; the twin and forgery construction it relies on clash with the Input codec: SA3-2 |
| SA-4 | Med | fixed | 2.4 item 6 (unit per kind; never negative; kind 8 from ack_refused; 9, 11 by count); f33 | fixed, still fixed |
| SA-5 | Med | question Q14 | 2.5 every refusal field per session, NA-15's read order, f14, f35; Q14 (b) | fixed as a question, still in rev 3 |
| SA-6 | Med | question Q11 (v) | 2.6 per-episode refusals; f19 asserts both > 0; Q11 (v) (b) | fixed as a question (but the cheat never banks: SA3-1) |
| SA-7 | Med | question Q15 | Q15 (c) recommended, p13, p19 | fixed as a question |
| SA-8 | Med | fixed | 2.2 "Stamps rise strictly"; f5, f32 | fixed, still fixed |
| SA-9 | Med | question Q9 | 2.4 item 2: K = REDUNDANCY_CAP + reorder allowance (Q9: 16), outside-window counter; f12 (iii), (iv) | fixed as a question; one undefined case: SA3-4 |
| SA-10 | Med | fixed | 2.6 clamp reruns reported (0 at P0); f18 option-B case; 2.12 one per command | fixed, still fixed |
| SA-11 | Med | question Q8 | Q8 (i)-(iii) with allowances per value; (c) carry to U4h; 9.1; 12 item 11 | fixed as a question; Q8's allowance statement and the LEAD-BOUND draft assume an anchored stamp: SA3-1 |
| SA-12 | Low | fixed | 2.4 kind 9 row, 2.6, f15 | fixed |
| SA-13 | Low | fixed | 2.6 bucket arithmetic; input_limited per cheat | fixed |
| SA-14 | Low | fixed | 2.4 kind 6 (z by design) | fixed |
| SA-15 | Low | fixed | 2.4 kind 7, 2.6, f15 | fixed |
| SA-16 | Low | fixed | 5.2's -22 row; 9.1; 12 items 11, 12 | fixed |
| SA-17 | Low | fixed | section 4 :722-724 | fixed |

## B2: hostile-input coverage (rev 3 against the code)
| Kind / behaviour | Home and order | Counter | Neg. case | Bite | Panic / loop / growth | Gap |
|---|---|---|---|---|---|---|
| 8 ack never sent | room, 1st, datagram (room.rs:783-788) | ack_refused (datagrams) | f10 | no-ack-sent | none; no Kick | - |
| 3 seq not forward | room, repeat filter, kept window K | kind-3 refusals; seq_outside_window | f12 | no-seq-forward | K bounded; Q9 (a) +1 test | SA3-4 (no-entry case); construction SA3-2 |
| 1 lead bound | room, 1st for a new seq | lead-bound refusals | f10, f13 (i) | no-lead-bound | signed distance, total for fixed t | no past bound under A: SA3-1 |
| 2 not newer | room, 2nd vs newest accepted (NA-13) | not-newer refusals | f10, f11, f13 (ii) | no-newer-tick, not-older-than | after32 as a predicate: no sort | fresh slot has no reference: SA3-1 |
| 4 move length | room, 3rd | move refusals | f10 | no-move-length | width unstated (i8): SA3-3 | SA3-3 |
| 5 press edges | room, 4th (input.rs:90-106) | press_refused | f10 | no-press-edge | none | - |
| 6 aim off map | room, 5th, x/y ±12,800 | aim refusals | f10 | no-aim-bounds | i32 compares | - |
| 10 catch-up | room, last (A: per tick; B: Full) | catch-up refusals | f7, f10, f22 | no-catchup-cap | cap 8 | idle bank under A: SA3-1 |
| 7 sim edges | npt-sim, step | reports() | U4d's per kind; f15 | no-cooldown (Q1) | - | - |
| 9 trailing/unknown | pump, datagrams only | trailing_bytes, unknown_field | pump tests; f15 | no-trailing (proto) | stream form would close: avoided (SA-12) | - |
| 11 wrong channel | pump | wrong_channel | f23 | no-channel (proto) | never closes a joined session (pump.rs:414-421) | - |
| clamp | npt-sim clamp | none by design | f18 | refuse-not-clamp | - | - |
| 1.5x clock | kinds 1, 2 (A), Full/late (B) | excess, per-episode refusals | f19 | not-older-than | - | cheat never banks: SA3-1; its pings at 1.5x: SA3-7 |
| lag switch | cap 8 (A), late (B) | lagswitch series | f20 | (Q15 (b) only) | held datagrams capped | idle bank longer than 200 ms: SA3-1 |
| never-acker | room queue cap, resync, Kick(overflow) | headroom, dropped, overflow_resyncs | f21 | unbounded | queue capped; ack scan O(n x m): SA3-8 | SA-16 variant: follow-up |
- Never a Kick for a forged kind: holds (room.rs:783-788 returns; pump.rs:414-421 keeps WrongChannel and
  WrongDirection on a joined session; kind 9 on datagrams only). The Kick paths are -22's (room.rs:1023-1025, :1057-1063).
- Counters: u64 `+= 1` or `+= len` (len <= 8, decoder :209): no overflow reachable. Refusal records, kept window,
  ClockSync samples, held datagrams: capped by section 4's rule. No loop over client-sized data beyond K, 8 or the cap.

## B3: recheck rows walked at HEAD (hostile input)
Rows 1, 2, 3, 5, 6, 7, 11, 13, 14, 17, 19, 24, 30, 31, 32, 33, 44, 45: all hold at the cited lines (row 3: `commands_buffered`
is :113-114, just before ":116-126", harmless). Evidence lines in "Code facts" above; plus greybox.rs:62, :151;
scene.rs:66; quant.rs:34-42, :108-109, :115-122; scenario.rs:70, :134, :215, :458, :999, :1270, :1370.

## B4: build rules
- Network: every U4f part is L1 (marks.json -20/-21/-22/-47 selects); the runner sets CARGO_NET_OFFLINE (section 4
  rule 1); no fetch, no listener, no L2 or LG run is U4f's. Holds by construction, but the brief never says so: SA3-6.
- Secrets: none; lab tokens are seeded (scenario.rs:115-125), logging rule carried (section 4 :722-724); Q4 (a)'s
  "keyed by session" must not use the token or bind value: SA3-6.
- Command forms: section 4 carries P17 (history :2187) with the single-call forms; D3 uses a shell grep with a
  quoted regex where the Grep tool would do: SA3-6.
- Guard: agents may not edit kit or gate files in any worktree (routine-guard.mjs:44, :69-76, :771); the brief covers
  it only through the lane allow-list (section 1 item 5): fine, named in SA3-6 for clarity.
- KEPT-LOGS §11: the protected folders are named (5.5 item 1) as R4 and c62hb-brief :500-501 name them. But 5.5 item 1
  and Q2 (a) also plan to pack each closed U4f set's own folder, beyond R4's "pack only by brief 8.1" (the card :198-199,
  :204-205: B1 first, then P3) and PACK-READ (a)'s "one or two closed, older log folders" (history :2212; B1 used one):
  SA3-5.

## B5: his words, rulings, scope
- :2125 and :2127 carried (5.6, section 4, 12 items 6 and 8): holds. SIZE-CHECK :2048-2051 / :2067-2075: M = 3,000,
  stop past 6,000; rev 3 estimates 4,400 with the stop at 6,000: holds. RECS-1043 C62HB-Q4 (b): -58's two checks are
  U4g's (2.3 :282): holds. C62H-BOUND's ten deferred Lows placed (9.2): holds. Carries 46, 80 (a)-(e), 86 and the named
  others: listed (section 3).
- Scope: nothing beyond U4f's row except the conditional wire change of Q6 (b) (a ruling's); its Pong field is in a
  bidirectional type: SA3-6 adds the client-direction rule.
- The LEAD-BOUND draft for Mustafa (12 item 6) states an allowance that holds only with an anchored stamp: SA3-1.

## Findings (rev 3)
- SA3-1 (High), SA3-2 (Medium), SA3-3 (Low), SA3-4 (Low), SA3-5 (Medium), SA3-6 (Low), SA3-7 (Low), SA3-8 (Low).
  Full text in the returned schema.

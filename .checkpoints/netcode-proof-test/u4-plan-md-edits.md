# The plan.md edits the advisor's U4 verdict calls for (advisor-u4.md), drafted for the main session

Drafted 2026-11-09 by netcode-architect. **The main session makes these edits; I edited neither plan.md nor the
checkpoint.** Drafted against /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/plan.md as on disk (sha256
daf2539527d6bb0020f9b62ebae3fb348869068d03b2154310c63151828792b8); line numbers are that copy's. Each "Old" block is
quoted exactly and occurs exactly once in plan.md (counted by a replace-once check, u4-conditions-notes.md), and no two
"Old" blocks overlap, so the order of application does not matter.

**When:** the verdict requires each before the code that needs it merges (condition 7 (c) before U4d2's code; Q7 before
U4a's; Q19 before the client code, U4e; risk 11's moves "in the sub-unit that needs it, before its code merges"). The
checkpoint's next step applies them all at once, before U4s, which meets every deadline.

Each edit ends with a provenance phrase naming the ruling, as plan.md already does for earlier rulings ("the advisor's
C3, 2026-11-06"). The main session may shorten those phrases; the content is what the rulings ask.

## Edit 1. Condition 7 (c): §4.2's "two named exceptions" becomes three, hash-wasm named (plan.md §4.2, lines 430-438)
Ruling: condition 7 (c), "Before the code merges, plan.md §4.2's 'two named exceptions' becomes three"; Q3 (B) ("The
third unsafe exception is mine under ADR 0002:264-265"); condition 7 (b) for the invariant and the test.

Old (exact):
```text
- **Unsafe code** is forbidden in the workspace (`unsafe_code = "forbid"`, the compiler backstop of the advisor's
  ruling of 2026-11-08: no `#[allow]` can relax it in a crate that inherits the workspace's lints), with two named
  exceptions under ADR 0002:264-265's rule ("an exception needs a written invariant, a test and an `#[allow]` with its
  reason"), each carrying its own `[lints]` table equal to the workspace's but for unsafe_code:
  - npt-client-wasm (`allow`), for wasm-bindgen's generated code;
  - npt-tcpinfo (`deny`): its two functions (getsockopt TCP_INFO and ioctl SIOCOUTQ), each `#[allow]` on the function
    only, holding four unsafe blocks of one foreign call each, each with its SAFETY comment.
  The spike's test command checks both, and greps that `allow(unsafe_code` appears nowhere else. npt-net stays free of
  unsafe code and libc (the advisor's M14).
```
New:
```text
- **Unsafe code** is forbidden in the workspace (`unsafe_code = "forbid"`, the compiler backstop of the advisor's
  ruling of 2026-11-08: no `#[allow]` can relax it in a crate that inherits the workspace's lints), with three named
  exceptions under ADR 0002:264-265's rule ("an exception needs a written invariant, a test and an `#[allow]` with its
  reason"), each carrying its own `[lints]` table equal to the workspace's but for unsafe_code:
  - npt-client-wasm (`allow`), for wasm-bindgen's generated code;
  - npt-tcpinfo (`deny`): its two functions (getsockopt TCP_INFO and ioctl SIOCOUTQ), each `#[allow]` on the function
    only, holding four unsafe blocks of one foreign call each, each with its SAFETY comment;
  - npt-hash-wasm (`deny`; U4, the advisor's Q3 ruling and condition 7, 2026-11-09, advisor-u4.md): the check-only
    cdylib over npt-sim that Node runs for the native-against-Wasm check (§4.5); each listed export carries one
    `#[allow]` for its `#[unsafe(no_mangle)]` attribute, and no `unsafe` block exists; its written invariant: no
    exported name collides with another symbol, and every export has integer signatures only; its test: the Node
    loader lists the module's exports and refuses any not on the list.
  The spike's test command checks all three, and greps that `allow(unsafe_code` appears nowhere else. npt-net stays
  free of unsafe code and libc (the advisor's M14).
```

## Edit 2. With edit 1: §4.1's layout gains hash-wasm's line (plan.md §4.1, line 391)
Ruling: not a separate ruling. The approved U4 plan's U4d2 says "plan.md §4.1 gains its layout line" (u4-build-plan.md,
U4d2, hash-wasm's rules), and edit 1 names a crate the layout does not list yet. Included so the two stay consistent;
the main session may drop it if it reads "the verdict calls for" narrowly.

Old (exact):
```text
  crates/client-wasm/  npt-client-wasm: the wasm-bindgen exports only
```
New:
```text
  crates/client-wasm/  npt-client-wasm: the wasm-bindgen exports only
  crates/hash-wasm/    npt-hash-wasm (U4): a check-only cdylib over npt-sim with integer-only exports, which Node runs
                    for the native-against-Wasm check (§4.5); the workspace's third unsafe exception (§4.2)
```

## Edit 3. Q7: §4.7's npt-proto list gains `RoomControl` (plan.md §4.7, lines 574-575)
Ruling: Q7 U4-CTL (a), "plan §4.7's npt-proto list gains `RoomControl` before the code merges".

Old (exact):
```text
  - the session controls (§10.2): `SessionControl`, with `SetProfile`, `TriggerBurst`, `BlockUdp` and `RecordStep`;
  - the rest: `encode`, `decode`, `Quant`, `Limits`.
```
New:
```text
  - the session controls (§10.2): `SessionControl`, with `SetProfile`, `TriggerBurst`, `BlockUdp` and `RecordStep`;
  - the room-wide controls (§10.2): `RoomControl` and its decoder, unfeatured (npt-client never calls them), built at
    U4 for the catalogue's item 6 and U5's fuzz target (the advisor's Q7 ruling, 2026-11-09, advisor-u4.md);
  - the rest: `encode`, `decode`, `Quant`, `Limits`.
```

## Edit 4. Q19: E7's text, npt-client's times in integer ns (plan.md §4.3, E7's list, lines 476-478)
Ruling: Q19 U4-UNITS (a), "E7's text changes in plan.md before the client code merges" (the plan's option (a):
"npt-client's API in integer ns, E7's text changed").

Old (exact):
```text
  - `ClientEndpoint::input(page_input, received: &[(Channel, bytes, arrival_ms)], prev_send_ms, now_ms)`;
  - `ClientEndpoint::step() -> StepOut { datagrams, reliable: [(Channel, bytes)], block, next_due_ms }`. Still one
    input call and one step call per tick. A message on the wrong channel is dropped and counted as forged (-21);
```
New:
```text
  - `ClientEndpoint::input(page_input, received: &[(Channel, bytes, arrival_ns)], prev_send_ns, now_ns)`;
  - `ClientEndpoint::step() -> StepOut { datagrams, reliable: [(Channel, bytes)], block, next_due_ns }`, every time in
    integer ns (the advisor's Q19 ruling, 2026-11-09, advisor-u4.md). Still one input call and one step call per tick.
    A message on the wrong channel is dropped and counted as forged (-21);
```

## Edit 5. Q19, the same field elsewhere in E7: host.ts's timer (plan.md §4.3, E7's web list, line 485)
Ruling: Q19, as edit 4 (E7's text; the field is renamed in edit 4).

Old (exact):
```text
  - `host.ts`: the timer set from `next_due_ms`, at most 7 ticks per wake (from the tick constant, §4.6), a fresh
```
New:
```text
  - `host.ts`: the timer set from `next_due_ns`, at most 7 ticks per wake (from the tick constant, §4.6), a fresh
```

## Edit 6. Q19, the same field outside E7: npt-lab's step order (plan.md §4.7, line 603)
Ruling: Q19, as edit 4. This line is in §4.7, not E7; it names the field edit 4 renames, so the name keeps one
spelling. The main session may drop it if it reads Q19 as E7 only.

Old (exact):
```text
    5. each client whose `next_due_ms` has come, in slot order.
```
New:
```text
    5. each client whose `next_due_ns` has come, in slot order.
```

## Edit 7. Q13 and Q6 (one location): §4.4's per-tick inputs (plan.md §4.4, lines 505-506)
Rulings: Q13 U4-WINDOW (a), "plan §4.4's per-tick inputs gain the bytes the server side sent"; Q6 U4-LOSS (b), "`loss`
joins with its first consumer, rate adaptation at U6. plan.md:562, §4.4's per-tick list and the checkpoint's C3 line
move with it" (the plan's option (b): "`loss` and the send-buffer space join Link at U6 with their consumer"). Risk 11
also names this list ("plan §4.4's ... per-tick inputs").

Old (exact):
```text
- **What npt-room gets from npt-server each tick:** the decoded messages with their arrival stamps, and per client the
  RTT, loss, send-buffer space, maximum datagram size and unsent count.
```
New:
```text
- **What npt-room gets from npt-server each tick:** the decoded messages with their arrival stamps, and per client the
  RTT, the bytes the server side sent since the last tick (pings and pongs, each counted as payload + 28 B, so the
  room's 256 kbps window counts them: the advisor's Q13 ruling), maximum datagram size and unsent count; loss and
  send-buffer space join at U6 with rate adaptation, their first consumer (the advisor's Q6 ruling, amending its C3;
  both 2026-11-09, advisor-u4.md).
```

## Edit 8. Q25: §4.4's room column marks the clamp "(in the sim)" (plan.md §4.4, the table row at line 503)
Ruling: Q25 U4-CLAMP (a), "plan §4.4's room column marks the clamp '(in the sim)'". Risk 11 also names it ("plan
§4.4's clamp"). The row is one long table line; the quoted part is unique in plan.md.

Old (exact):
```text
§4.6); the clamp beyond reach; sequence forward and wrap;
```
New:
```text
§4.6); the clamp beyond reach (in the sim); sequence forward and wrap;
```

## Edit 9. Q6: `loss` moves to U6 (plan.md §4.7, npt-link's list, lines 562-563)
Ruling: Q6 U4-LOSS (b), "plan.md:562 ... move[s] with it".

Old (exact):
```text
      WebSocket's newest-only replace sits inside its backend; `loss` joins at U4 with npt-room, its first user (the
      advisor's C3, 2026-11-06);
```
New:
```text
      WebSocket's newest-only replace sits inside its backend; `loss` joins at U6 with rate adaptation, its first
      consumer, and §4.4's send-buffer space with it (the advisor's C3, 2026-11-06, as amended by its Q6 ruling of
      2026-11-09, advisor-u4.md);
```

## Edit 10. Condition 8: the U7 row gains C-U7-1 (plan.md §12.1, the U7 row, line 1226)
Ruling: condition 8, "New carry C-U7-1 ... This goes into plan.md's U7 row and the checkpoint."

Old (exact):
```text
| U7 | M | npt-client-wasm and the harness (E7); the browser smoke in local Chromium; the hash check in Chromium |
```
New:
```text
| U7 | M | npt-client-wasm and the harness (E7); the browser smoke in local Chromium; the hash check in Chromium. **C-U7-1** (the advisor's condition 8, 2026-11-09, advisor-u4.md): U4's native-against-Wasm check (U4d2) does not cover wasm-opt, while ADR 0002:500-505 checks "the exact file G6 ships" (after wasm-bindgen and wasm-opt); U7 reruns U4d2's check on npt-hash-wasm passed through the same wasm-opt step and flags as the shipped npt-client-wasm, or states why not |
```

## Edit 11. Risk 11's move: §4.7's npt-sim line gains `SimCore` and `HitHistory` (plan.md §4.7, lines 567-568)
Rulings: risk 11 ("§4.7's HitHistory and SimCore lines"); Q1 U4-SEAM (a) (the trait in npt-sim, npt-room generic over
it) and Q2 U4-HIT (a) (`HitHistory` moves from npt-room to npt-sim, the room owns its instance and lends it to
`step`; "plan §4.7 moves one name").

Old (exact):
```text
- **npt-sim:** `World` (implements the seam), `Scene::stress`, `Greybox`, `Areas`, `Portals`, `SimRng`,
  `trajectory_at`, `StandIns`.
```
New:
```text
- **npt-sim:** `SimCore` (the seam trait, with its plain-data input and output types; moved from npt-room: the
  advisor's Q1 ruling, 2026-11-09, advisor-u4.md), `World` (implements the seam), `HitHistory` (the ring of past hit
  volumes, owned by the room and lent to `step`; moved from npt-room: the advisor's Q2 ruling), `Scene::stress`,
  `Greybox`, `Areas`, `Portals`, `SimRng`, `trajectory_at`, `StandIns`.
```

## Edit 12. Risk 11's move: §4.7's npt-room line loses them (plan.md §4.7, lines 576-577)
Rulings: as edit 11.

Old (exact):
```text
- **npt-room:** `SimCore` (the seam trait), `Room`, `Roster`, `ClientSlot`, `InputBuffer` (option A or B),
  `EventQueue`, `AckRing`, `Encoder`, `InterestGrid`, `AreaFlood`, `Priority`, `HitHistory`, `rewind_at`.
```
New:
```text
- **npt-room:** `Room` (generic over npt-sim's `SimCore`; it owns the `HitHistory` instance), `Roster`,
  `ClientSlot`, `InputBuffer` (option A or B), `EventQueue`, `AckRing`, `Encoder`, `InterestGrid`, `AreaFlood`,
  `Priority`, `rewind_at` (the advisor's Q1 and Q2 rulings, 2026-11-09, advisor-u4.md).
```

## Edit 13. Risk 11's move: the proto → sim edge (plan.md §4.2, after line 423)
Rulings: risk 11 ("the proto → sim edge"); Q23 U4-OWNSNAP (a) ("the grid lives in npt-sim and npt-proto depends on
npt-sim", ADR 0002:188 letting the protocol crate use sim types). §4.2 today states no internal edge for npt-proto;
the new bullet states the one U4 adds. It is an allowance, not a ban, so the test command gains no check.

Old (exact):
```text
- **The client:** npt-client never depends on npt-net; the bots get transports by composing npt-client with npt-net.
```
New:
```text
- **The client:** npt-client never depends on npt-net; the bots get transports by composing npt-client with npt-net.
- **The protocol and the sim:** npt-proto may depend on npt-sim, for the wire grid (one function for the sim's
  own-copy snap and both codec ends) and the seam's plain-data types (ADR 0002:188: the `protocol` crate uses "`sim`
  and `content` types only"; the advisor's Q23 ruling, 2026-11-09, advisor-u4.md).
```

## Checked, and no plan.md edit drafted
- **plan.md:1223 (the U4 row)** names "-30 on loopback"; condition 2 keeps that as a recorded test, so the row stands.
- **plan.md:590-591** ("the scenario runner, the replay and the hash comparer join it in U4"): `replay` stays at U4
  with its own test (the build plan's choice under condition 9), so the line stands. Had `replay` moved to U6, this
  line would need an edit.
- **E11's crate column (plan.md:459, "room")**: the room still owns the `HitHistory` instance and `rewind_at`; only the
  type moves (edits 11-12). No edit.
- **E7's `BlockWriter` (plan.md:480) and §4.7's npt-client list (:588)**: Q5 puts `BlockWriter` at U7, which is E7's
  unit anyway; no ruling asks for a plan.md change.
- **The npt-proto → npt-link and npt-client → npt-link edges** (the build plan's §4.1, for `Channel`): §4.2 restricts
  neither, and risk 11 names only the proto → sim edge, so none is drafted.
- **Nit (e)** asks for a note in the message catalogue (docs/design/netcode-proof-messages.md §6 item 3), not in
  plan.md; the build plan puts it in U4a's change-set.

## The checkpoint lines (text for the main session; /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test.md)

### Checkpoint 1. The C3 line (Open findings, lines 305-306), moved by Q6
Ruling: Q6 U4-LOSS (b), "plan.md:562, §4.4's per-tick list and the checkpoint's C3 line move with it".

Old (exact):
```text
  - **C3 (U4):** `loss` on Link, with §4.4's send-buffer space settled at the same time; each backend reports its own
    estimate, never the simulator's true fate; plan §4.7 then.
```
New:
```text
  - **C3 (U6; moved from U4 by the advisor's Q6 ruling, 2026-11-09, advisor-u4.md):** `loss` on Link, with §4.4's
    send-buffer space settled at the same time, joining with its first consumer, rate adaptation; each backend reports
    its own estimate, never the simulator's true fate; plan §4.7 then.
```

### Checkpoint 2. C-U7-1 (Open findings, a new entry after the U2 list's "Note (U6)", lines 311-312)
Ruling: condition 8, "This goes into plan.md's U7 row and the checkpoint."

Insert after (exact):
```text
  - **Note (U6):** links are `Box<dyn Link>` without `Send`; if npt-server moves links across tasks, the L2 backends
    need `+ Send`.
```
New lines:
```text
- **From the advisor's U4 verdict (advisor-u4.md, 2026-11-09), carried past U4:**
  - **C-U7-1 (U7):** U4d2's native-against-Wasm check does not cover wasm-opt, while ADR 0002:500-505 checks "the
    exact file G6 ships"; U7 reruns U4d2's check on npt-hash-wasm passed through the same wasm-opt step and flags as
    the shipped npt-client-wasm, or states why not (condition 8; also in plan.md's U7 row).
```

## Count
13 plan.md edits (edits 1-13; edit 7 serves two rulings, Q13 and Q6, at one location; edits 2, 5 and 6 are consistency
edits the main session may drop, each says why) and 2 checkpoint changes (one replacement, one insertion).

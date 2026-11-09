# U4 build plan: notes (netcode-architect), 2026-11-09
Branch claude/netcode-proof-test-plan-hcky5g; task names head f985226; the folder's HEAD is f2573f4 (three checkpoint
autosaves on top: e0b54d1, 5bed612, f2573f4; `git log --oneline -8`). Read-only on code; notes and the plan only.
Memory: none to recall (the arch and marks scouts checked: u4-scout-arch.md:4, u4-scout-marks.md:4).

## Plan
1. Read the three scouts in full (done: arch F1-F48, marks F1-F32, netcode F1-F124).
2. Read the U3 build plan and U3d plan rev 2 for the shape (done).
3. Verify the load-bearing citations myself (done, M1-M24 below).
4. Write u4-build-plan.md rev 1 in one Write (only Write is available to me; no Edit): plain words; requirements;
   carried items; design rules; split U4a..U4h; not built; build rules; risks; change type; questions; For Mustafa.
5. DONE line.

## Findings (one line each; checked by me at the lines given)
- M1 U4 row verbatim at plan.md:1223 (as the scouts quote). U5 :1224, U6 :1225, U7 :1226 ("npt-client-wasm and the
  harness (E7); the browser smoke in local Chromium; the hash check in Chromium"), U8 :1227.
- M2 §4.4 :495-511, §4.5 :513-537, §4.6 :539-552, §4.7 :554-611 as the scouts cite. §4.4's room inputs (:505-506)
  include "loss, send-buffer space"; :507-509 WsServer applies fix 1, npt-room sees BufferFull and the skipped count.
- M3 §4.1:377 "npt-proto: messages, controls, quantisation, delta codec, limits": "controls" already homed in proto,
  which supports putting the room-wide control types + decoder there (catalogue item 6).
- M4 Link today (link/src/lib.rs:63-94): no `loss`, no send-buffer space; Listener returns Box<dyn Link> (no Send).
- M5 npt-l1 doc (l1/src/lib.rs:21-35): streams "Simplified until U4" for C1 and C2; rto_ns = max(200 ms, 2 x RTT)
  (:159-162); fates per (seed, link, direction, channel, packet index), 16 words each (:9-12). FateLog canonical v1,
  tag b"npt-l1 fate log v1\n" (:230), 40 B per entry (:241-266). Public fns: connect, listener, with_fate_log, new,
  with_bite (:835-897). No next-event query.
- M6 lab/src/probe.rs:200-201 sends send_unreliable only: -3's probe logs carry no stream entries, so datagram-only
  logs can hash as committed if the entry layout and tag stay (913db8e9 P2, feabf2f0 B1, b8a04f2f P4 seed 1:
  advisor-u2.md:31-32).
- M7 Manifests (Cargo.toml [dependencies]): sim none (comment "libm and rand_chacha join at U4, pinned"); proto none;
  room -> proto; client -> proto, sim; client-wasm -> client ("wasm-bindgen joins at U7"); server -> link, proto,
  room, sim, net, log; lab -> link, l1, sim, proto, room, server, client, log, serde_json, sha2; bots -> link, proto,
  client, net, log; l1 -> link, rand_chacha, rand_core.
- M8 check.mjs:24 LEAF = link, l1, sim, proto, room, client, log. :166-172 the unsafe scan (edition 2024 needs
  #[unsafe(no_mangle)]). :1156-1158 "the runner refusal covers every target triple ... (no wasm test is planned)".
  :1160-1173 DEPS_CHECKED has no features comparison. No mention of clippy.toml in check.mjs or check.test.mjs
  (grep: 0), so proto/clippy.toml's "the spike's check script compares them" is false today.
- M9 fixes.rs is crates/net/src/tcp_model/fixes.rs: SNAP = 1,050 (:34, "1,024 B of payload, a 4 B WebSocket header
  and TLS 1.3's 22 B"), INPUT = 88 (:36), JOIN = 140 (:41), RESUME = 4,200 (:42). advisor-ws-docs-r2.md:383: the
  run-structured bound takes "the WebSocket message size from L1's B1 runs of the same scene", "before R3's run plan;
  in U3c or its own small unit".
- M10 marks.json (python read): -17 parts L1 P0/P2/P3/B1, L2 x4, every S30; bites tick16, client-rounding. -18 parts
  still ['L1 B1','L2 B1'] with the pre-split checks (500 ms, 480 count) and one bite no-fallback: the split of
  advisor-ws-docs-r2.md:375-377 is not in marks.json. -21: 8 parts, 13 bites. -30: parts 'RS: 3 recordings x 2
  transports' and 'C: loopback, 3 recordings x 2 transports'; bites wallclock, late-command.
- M11 Checkpoint :298-301 still lists U2's C1 (back-off + 1,448 B segments); advisor-u3q.md:160-173 REPLACED it with
  C1'. Stale line: a text fix for the main session.
- M12 Checkpoint :404-413 Mustafa's 2026-11-09 rule verbatim and its application (shared folder, touch, test counts,
  cargo's lock, main folder's own cache, every brief names it). Baseline counts at U3d's merge: check.mjs 9/9, 267
  measured, 281 bites, self 43 (checkpoint :620-621).
- M13 ADR 0001:446-451: "quinn's RTT and loss feed rate adaptation; under pressure the server drops low-priority
  entities first, then falls back to 20 Hz snapshots". Judged by -23 and -24 (ADR 0001:1613-1614), both L2 on RS.
  L1 builds no rate limit (l1/src/lib.rs:43-44). So `loss`'s consumer is rate adaptation, which no U4 L1 mark reads.
- M14 Catalogue :240-253: U4's six assertions. Item 3 needs "PM-0001-34's counting allocator". A counting allocator is
  a GlobalAlloc impl, which is `unsafe impl` (the trait is unsafe to implement): forbidden by the workspace's
  `unsafe_code = "forbid"` (plan.md:430-438). So item 3's allocation half needs U5's fuzz crate, a dependency, or an
  exception: a question (U4-ALLOC). The size refusal half (Err before decode) U4 can test.
- M15 Catalogue :300-305 and :280: Q6 answered "one fuzz target for the socket messages" (U5's -34), so the room-wide
  control decoder must exist before U5: it cannot wait for U6's socket.
- M16 ADR 0002:186-191: sim depends on content only; room on sim, protocol, content; sim-wasm holds the client
  endpoint and the local server. Direction room -> sim, never sim -> room.
- M17 ADR 0002:500-505: the replay runner "built as the server builds the sim"; "twice in two processes and once as
  Wasm in Node"; "every Wasm run uses the exact file G6 ships" (the foundation's rule; the spike's §4.5 does not repeat
  it).
- M18 ADR 0004:166-178: two entry points (authority step, prediction step that reads decoded movers and draws from no
  stream). ADR 0004:348-357: injected divergences std sin, hash-map iteration, presentation draw; plan §4.5:537 keeps
  two (a spike narrowing, stated).
- M19 plan.md:680-683 "Bites first"; :696 -7's history from -48's test cap "fixed in U4 from L1's largest honest
  rewind at P3 and P4"; :720 PM-0004-67 at R2 (VS3); :749-750 "the same seed gives the same packet-fate log and the
  same per-tick hashes".
- M20 TickRecord (log/src/lib.rs:104-117) has `h` (one 16-hex hash), no `hs`.
- M21 clippy's own description of iter_over_hash_type (strings of the 1.99.1 clippy-driver): "iterating over
  unordered hash-based types (`HashMap` and `HashSet`)". Whether it covers iterator method chains as well as `for`
  loops is NOT verified here (the lint page did not load in full); the type ban settles it either way. Confidence
  that the lint alone suffices: low; that the type ban suffices: high.
- M22 -21's twin (marks scout F27): fates per (seed, link, direction, channel, packet index) (l1/src/lib.rs:9-12) mean
  a forged run must keep per-flow packet counts equal to its twin's, or later fates shift. Design rule needed (§4.13
  of the plan) plus a fate-alignment test.
- M23 Wasm-export route without unsafe blocks: exports taking and returning only integers (a word-feed interface,
  no pointers, no slice::from_raw_parts) need `#[unsafe(no_mangle)]` only, which the unsafe_code lint still catches
  (arch F25, rustc's lint listing). So option (B) is "a third exception for the attribute only, no unsafe block".
- M24 Bites count: 49 mark bites + 2 (§4.5) + 1 (catalogue owner-to-all) = 52 (netcode F101). Two cannot fail on L1
  in U4: -42/smoothing-off (its failing check is ML-only: marks F26) and -49/cap-estimate-only (rule-cap parts after
  R3: marks F24). Carried, named.

## Decisions I take in the plan (recommendations; each a question where it moves an approved line)
- Seam trait in npt-sim (arch F11); rewind volumes cross into step, float geometry in sim (arch F22, netcode F123).
- Wasm artifact (B) with an integer-only export surface (M23), medium confidence; (A) the fallback.
- Seam-level recordings, replay npt-sim alone (arch F27); -30's pre-check a native room+sim hash on Loopback vs L1 P0.
- npt-server's U4 half = transport-free, runtime-free library (arch F34); npt-bots' behaviours transport-free (F38).
- C3: `loss` and send-buffer space move to U6 with their consumer (M13), the advisor's call (it was his C3).
- Room-wide control types + decoder in npt-proto at U4 (M3, M15); catalogue item 3's allocation half to U5 (M14).
- Type bans (HashMap/HashSet) in room, client, server-lib, lab, bots; a clock ban in server and bots (F28, F114, F36).
- Umbrella plan + short per-sub-unit briefs for U4e-U4h, each checked by the advisor before its build.

## Blocks written to u4-build-plan.md
- (pending) one Write of rev 1.

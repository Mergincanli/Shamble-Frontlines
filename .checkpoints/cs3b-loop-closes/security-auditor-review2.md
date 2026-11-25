# security-auditor, change-set 3b round 2 (head 0dfca749; round 1 bbbf09e4; base 28254a73)

Copy: /home/user/Shamble-Frontlines/.gate/ci-wasm-wt/.gate/security-auditor-cs3b2 (detached at 0dfca749).
Round 1's notes: security-auditor-review1.md (threat model, I1-I7, M1, L1, L2; L3 named only in the memory line).

## Plan
A. M1: read ADR 0001 OQ-0001-6 (around :1156) and PM-0001-21, DECISIONS 2026-11-24 + the three mustafa-*.md files;
   judge the reason (1)-(4); the deadline ("before online play" vs "before a command moves anything"); whether a
   bound now would pre-empt OQ-0001-6; re-run my bank probe on 0dfca749 (scratch test in my copy only).
B. L1: read Room::on_datagram's new doc against my L1 list. L2: confirm no manifest change (diff --stat). L3:
   re-derive from my notes (its text was lost) and judge it on the new head.
C. The fix pass as attack surface: git diff bbbf09e4 0dfca749 -- crates packages, file by file:
   C1 endpoint.rs (new/set_post/block/write_block), local_match.rs, host.ts: Sim::input without the Rust guard;
      what changes online.
   C2 Readout.late vs refused: what a detector should see.
   C3 queue.rs insert window; Endpoint::build_input newest always goes; codec.rs Join version before token;
      LocalMatch::step assert text.
   C4 tests/any_bytes.rs: which security properties it asserts; what cheap ones are missing.
   C5 page: keyboard.ts key(.., typing), boot.ts event.timeStamp, worker-link.ts olderPosts + #done list,
      report.ts textContent.
   C6 seed/state (I3): messages, block, logs, panic texts, Debug; the boundary test's Sim pin.
D. Where the 255-presses threat-model line should be written down for change-set 4 and the weapons change-sets.
E. Runs: cargo test (room, client-endpoint, protocol, sim-wasm), clippy --workspace --all-targets, my probes.
F. Answer <= 30 lines.

## Findings (one line each, as found)
- R2-I7 manifests: git diff bbbf09e4 0dfca749 --stat -- '*.toml' Cargo.lock 'package*.json' = empty (0 bytes);
  outside crates/ and packages/ only .checkpoints notes changed (no .github, tools, gate files). L2 (devtools'
  three unused path edges): no manifest change, the card says "in the report" (EA-6). OK.
- L3 RECOVERY: L3's text is in neither my round-1 notes nor my unsent round-1 draft (copy security-auditor-cs3b,
  sa-scratch/answer.txt, 01:31; the notes' memory line 01:29 names "L1, L2, L3"). The draft numbers only M1, L1
  and L2. Its unnumbered items with a line and a fix: the client panic on a server's message (local_match.rs:
  172-177 at bbbf09e4; online must count and close), the scratch that holds the seed after shared_hash
  (world.rs:127-128), datagrams before the join counted as decode errors (lib.rs:167-169). I do not invent the
  lost text; I re-derive L3 below from what the fix pass shows.
- C3 queue.rs:71-75 (insert opens the window with the first command offered: taken = first - 1 until the first
  take): closes netcode F2 (a spread of sequences before the first take could store one twice). New surface: only
  the client's own first command sets its own window (a command refused by the lead never reaches insert,
  lib.rs:205-208), so a client can only lock out its own later commands. An honest reordered first command loses
  one tick's command, counted stale (not refused). PASS.
- C3 codec.rs:125-129 / :160-163: Join = kind, version u16, token len u16 (<= 1024 checked before the read),
  token, hash 32, finish(). Version before the token, as my round-1 Slice-3 list asked. Room still decodes the
  whole Join before checking (lib.rs:136-146): bounded (1024). PASS. decoder_inputs.rs: Join generator follows the
  new order; the cap check is now per type (Join 1024, JoinAccepted 64, JoinRejected/Snapshot 0, Input 8 commands).
- C3 local_match.rs:149-153: one assert! with a constant text ("the local server's answer to the join was not an
  acceptance this build takes"): no state, no format argument. Unreachable from the page: the endpoint sends one
  Join at new(), the room answers each Join once, the host's three calls (input, step, rebase) send no Join. Same
  as round 1 (unreachable in 3b; online a server's bytes must never panic the client: count and close).
- C1 Sim::input -> LocalMatch::set_post -> Endpoint::set_post (endpoint.rs:158-160) takes any post; the only
  "newest by number" guard is host.ts:104-116 (1 to 2^31-1 ahead; repeats and older counted stalePosts). What a
  page or a modified host can do: hand an older post -> held bits roll back for that tick; press counts cannot
  go back (press.rs: a count 128+ "ahead" is ignored and counted backwards), each command moves a counter by <= 1
  (pacer), so every command the endpoint builds still passes the room's press pace. Only the player's own input
  in local play, and the Worker is player-controlled anyway (devtools can call Sim::input directly). Online:
  nothing changes for the server: a hostile client bypasses Sim::input and the endpoint entirely and writes its
  own datagrams; the room's checks (lead, newer tick, press pace) are the only ones that count. The removed Rust
  guard was never a security control. PASS.
- C2 Readout.late (endpoint.rs:69-75, :291-300): decode failures stay in refused; well-formed snapshots that are
  not newer or come before the join go to late. Hides nothing a detector needs: anti-cheat detection is the
  room's counters (server side), untouched; the client's readout is diagnostics a hostile client controls anyway;
  over WebTransport a third party cannot inject or replay datagrams (QUIC AEAD + packet numbers), so a late
  snapshot is reordering or the server itself. late is not in the block (link = sent, received, refused,
  post_used), but loop.spec.ts:63 asserts received == tick, so a dropped-as-late snapshot in local play still fails
  the browser test. One caveat for 3c (RR-3, already in 3c's plan): a changed-but-decodable snapshot with a
  far-ahead tick now shows as late climbing, no longer as refused; the simulator must treat corruption as loss.
- C6 seed/state (I3) on 0dfca749: no Debug/Display on World, MatchSetup, ChaCha8 (Clone only), Room, LocalMatch,
  Endpoint, Sim; no manual Debug/Display impl in crates (grep empty); the only state_hash call outside sim is in
  local_match.rs's tests (:333, tests start :171); the block stays tick, layout hash, echo, link (sent, received,
  refused, post_used); messages carry no seed; worker.ts:24-27 draws, hands over and zeroes the seed; the one
  panic text and CreateError texts carry no state. Boundary test (sim-wasm.test.mjs:140-147) now pins Sim's own
  calls [block_len, block_ptr, constructor, free, input, rebase, step] and no statics: a state, hash or seed
  accessor joining the players' file fails it. Nit: the filter drops every name that starts with "__" (meant for
  wasm-bindgen's __destroy_into_raw), so an accessor named that way would pass; naming the one internal name is
  stricter. Not a finding on its own. PASS.
- C5 page: no innerHTML/outerHTML/insertAdjacentHTML/document.write/eval/new Function/srcdoc in client/src,
  sim-worker/src, types/src (grep exit 1); text via textContent (keys.ts:51, overlay.ts:17,25,36, boot.ts:33) or a
  field's value (overlay.ts:45); report.ts's two new lines are numbers (olderPosts, releases.doubled).
  keyboard.ts:63-68: in a text box a key-down returns before anything is kept, a key-up only clears that key's bit
  (#settle: no change -> no count), and the event is not prevented (the text box keeps it); only event.code is
  compared with the six keys: nothing typed is read, kept or logged. boot.ts:58-62 event.timeStamp: feeds only
  changedAt -> keyToTick and the "lost" readout (diagnostics); the post is six numbers with no time; a synthetic
  event (isTrusted false) gains a script nothing it lacked (it can post to the Worker itself). worker-link.ts:
  256-259, :300-301: #done keeps references to buffers already transferred (detached: no backing store) only at
  places >= #doneCount, which are overwritten before they are read again; slice() hands the post a fresh list, so
  no detached buffer is ever put in a transfer list again (no DataCloneError path) and no in-use block (#newest,
  #previous) is returned. Its length is the most blocks ever taken between two frames (a Worker flood costs far
  more in messages). PASS.
- RUNS (copy 0dfca749 + my untracked probe crates/sim-wasm/tests/sa_probe_bank.rs): debug client-endpoint 26,
  protocol 19 + decoder 8, room 13, sim 17 + rules 5 + alloc 1, sim-wasm 13 + any_bytes 2 + panic_hook 1 +
  step_alloc 1, probe 3: all pass (counts equal the card's). any_bytes --nocapture: room 67,572 commands run (7,871
  late), 197,898 presses granted, turned away [rejections 8,313, decode 56,810, ack 2,698, ahead 851, not newer
  1,174, presses 4,773, duplicates 49,403, stale 208,217]; endpoint 84,744 inputs, 19 joins rejected, [received
  16,708, late 78,324, refused 26,425, lost 77,132, doubled 144,424] (late >> received: one changed tick that still
  decodes makes every later snapshot late: RR-3, in 3c's plan).
- M1 PROBE on 0dfca749: unchanged. HOSTILE idle 30 / 300 / 3,000 -> 30 / 300 / 3,000 unbroken room ticks with 2
  commands, refused 0, commands_run = room ticks (190 / 1,000 / 9,100). HONEST endpoint after outages of the same
  lengths: 7 ticks with 2, refused 0, most presses of one button in one tick 25 / 39 / 179.
- M1 JUDGEMENT. ADR 0001 :238-239 "the commands it runs per client per tick are capped, the catch-up cap no larger
  than the redundancy cap (the lag-switch guard)"; OQ-0001-6 :1159-1162 (a) "run commands as they arrive, several in
  one tick within the cap (Source)", (b) "... late commands dropped and catch-up limited to the redundancy window";
  his words 2026-11-24 22:44:50 item 3b: "The proof test no longer picks it. Slice 3 picks it, and change-set 7
  still builds both ways." Build plan Part 1: 7 = Lag (local, fake lag), Slice 3 = online (`server` enters, ADR
  0002 :1523). Reason (1) holds (nothing moves in 3b; in local play the player owns the room, so a bound protects
  nothing there either). Reason (2) holds for the bound's FORM only: a bank bound is part of BOTH options as their
  sources build them (Source's own code clamps the bank, S1: sv_maxusrcmdprocessticks, "still make sure to avoid
  speedhax"; (b) names the redundancy window), so the bound itself pre-empts nothing; its form (budget or late
  drop) is the pick's, and change-set 7 builds both ways. Reason (3) is true but is the reason the gap must be
  written down: PM-0001-21 (:1611) holds the lag switch only 200 ms (6 ticks, inside the redundancy window) and its
  fast-clock case bounds the total, so the proof test as written passes a room with no bank bound. Reason (4):
  the doc (lib.rs:174-175) names it accurately. ANSWER ACCEPTED for 3b, with the deadline sharpened: built in
  change-set 7, in each of its two ways, each with a long-hold test (idle N ticks, then a past-dated backlog: at
  most the redundancy window of two-command ticks; the honest outage cases keep refused 0); hard stop: no room
  behind a network transport (Slice 3's `server`) without it. "Before a command moves anything, even in local
  play" is not needed: no adversary in local play, and the honest endpoint never banks more than its window (7).
- RUNS 2: cargo clippy --locked --workspace --all-targets: one warning, in my probe (sa_probe_bank.rs:116); shipped
  code 0 (denied lints included). Release decoder_inputs on 0dfca749: 8 passed in 6.93 s; per type 10,000,000
  random + 10,000,000 mutated, 0 panics; largest single allocation Join 1024 (cap 1024), JoinAccepted 64 (64),
  JoinRejected 0, Snapshot 0, Input 0 (cap 320); plants: unchecked index at input 17, over-allocation at input 0.
- M1 deadline, one more reason for change-set 7 (not later): the bound changes the honest numbers Slice 3 picks on
  (under (b) a late command is dropped: missing inputs and corrections, PM-0001-20's 99.9 %), so the two ways
  should be measured with their bounds in. And the report should not read as if option (a) had no bank bound: the
  ADR's (a) sentence (:1159-1160) names only the per-tick cap, while Source's own code also clamps the bank (S1).
- L1 CHECK lib.rs:168-175 (Room::on_datagram's doc): names checked (acks, lead, strictly later tick, press pace)
  and not yet, each with what brings it: move length and aim (change-set 4), view time (lag compensation), presses
  vs cooldowns and ammunition (weapons), message rate, the join's deadline and token (the connection, Slice 3), an
  ack's lower bound, how late a command may run (OQ-0001-6, before online play). Every item of my L1 list is there
  except the message size cap before decoding (the decoders' count and length caps bound the work until Slice 3).
  FIXED.
- L3 (re-derived, Low): from my unsent draft's item (7), the one with a line in 3b's own new code (world.rs came in
  90da28c1, 3b step 2; shared_hash is not on main 28254a73): world.rs:127-133 shared_hash, the hash made to be
  compared on the client side, writes the WHOLE state, the 32 seed bytes included, into the caller's scratch and
  leaves it there ("scratch holds the whole state's bytes afterwards", :128). No non-test caller in 3b; 3c's one
  new read-only number on the players' file (the state's hash, DEPART-3, cs3-plan.md:154, :318) is its first.
  Why it matters: a whole-state hash or bytes on the players' side gives the seed away (FNV-1a is no MAC: its low
  bits depend only on low bits, so many ticks over one fixed seed are equations in it), and the seed is the
  authority's alone (ADR 0004 part 8). Fix (smallest, either): shared_hash writes and hashes the shared prefix only
  (no seed byte ever in the buffer), or 3c's export keeps the scratch inside Rust, returns only the u64, and the
  boundary test shows the exported number unchanged when only the seed changes and changed when the shared state
  changes. Latent; in local play the seed is the player's own Worker's anyway.
- D (where the 255-presses line goes): (1) 3b's report, follow-ups, named for the change-sets that first read an
  edge: 4 (jump: Space is button 4, keyboard.ts:14), 8 (fire, reload, the active-reload tap), 9 (throw); each with a
  test: a command after a 255-tick outage carrying 255 edges of that button gives at most one action (or the
  cooldown's worth) in that tick. (2) The code, on sim::PlayerCommand (crates/sim/src/world.rs:32-37), where every
  system that reads presses starts: one sentence there travels with the code to change-set 4. ADR 0001 part 5
  (:236-237) already holds the rule for fire, ability and reload ("meet the server's own cooldown, ammunition and
  reload state"); the follow-up cites it and adds jump and every later edge (buy, interact). No ADR change needed.
- any_bytes.rs (C4): asserts every written message valid (room snapshot :203, room reply :194, endpoint input
  :311, block hash :376), at most CATCH_UP_CAP per tick (:205-209), presses granted <= ticks since the join per
  button (:218-225, first = joined_at, tight), no panic. "No command twice" is asserted only as "not the same last
  command as the tick before" (:212-215): a repeat after another command, or inside a two-command tick, passes it.
  Cheap and stronger (my probe test 3, holds on the room: span 2,135 = 2,000 runs - 1 + 136 missing): whenever a
  command ran, last_processed is seq_newer than the last one seen; per match, the distance it moved equals
  commands_run - 1 + missing (wrapping u16), which any re-run breaks by one. Not planted (I edit no tracked file):
  shown by argument; today the queue's window and admit's strictly newer tick each stop a re-run. Also cheap:
  queue_full and missing are not among the counts the run must reach (:231-243), so the full-queue path is not
  shown reached. When M1's bound is built: an idle-then-backlog client mode here. LOW (test strength).
- NIT sim-wasm.test.mjs:143: the "__" prefix filter would pass an accessor named "__x"; list wasm-bindgen's one
  internal name instead. Not a finding.
- LINE FIXES for the answer: boot.ts:59-62; worker-link.ts:257, :300-301; local_match.rs:150-153;
  sim-wasm.test.mjs:145; any_bytes.rs:207, :214, :222.

## Anti-cheat coverage on 0dfca749 (action -> where the room checks it -> gap)
| Action | Checked at | Gap |
|---|---|---|
| Join | lib.rs:135-166 version, content hash, one slot; Join codec version first (codec.rs:160) | no token, no deadline (Slice 3, named :172-173); a second Join answered RoomFull, not closed |
| Command tick | lib.rs:205-208 lead <= 8; admit :234-237 strictly newer than the last run | no late bound: M1, deferred (change-set 7 / Slice 3) |
| Sequence | queue.rs:71-97 window from the first command offered; duplicate and stale dropped | none |
| Catch-up | step lib.rs:251-273, <= 2 per tick | no bank bound (M1) |
| Presses | admit lib.rs:238-243, <= 1 per covered tick | <= 255 applied in one tick after an outage, honest too (D); counter moved back by k passes once covered >= 256-k (NA-5 follow-up) |
| Held buttons | any mask legal | systems gate on state when they read them (change-set 4) |
| Move, aim, view time | read by nothing | named lib.rs:170-172: change-set 4, lag compensation |
| Acks | lib.rs:183-195 group 0, not after the newest sent | lower bound, one per group (follow-ups before online) |
| Rate, size | decoder caps only (4 acks, 8 commands, token 1024) | per-connection limits: Slice 3 (named :172-173) |
| Fire, reload, ability, credits, XP, results | none exist yet | weapons and economy change-sets |

## Memory lines (for the session to save; my only write is this file)
- 2026-11-25 cs3b round 2 on 0dfca749: PASS. M1 answer accepted as deferred, not closed (a security finding left
  open: needs-mustafa by advisor.md step 4): build the bank bound in change-set 7 with each command-timing way, a
  long-hold test each; hard stop: no room behind a network transport (Slice 3 `server`) without it; PM-0001-21's
  200 ms lag switch is inside the redundancy window, so the ADR's marks do not catch M1; Source clamps the bank too
  (S1), so the bound is part of either OQ-0001-6 option, only its form is the pick's. L1 fixed. L2 no manifest
  change. L3 (re-derived; original text lost in the 01:32 restart): shared_hash leaves the seed in its scratch
  (world.rs:127-133); 3c's hash export must hand JS only the u64 of the shared prefix.
- Threat model (standing): commands carry cumulative u8 press counters, so one tick can apply up to 255 presses of
  a button (honest after an outage); every system that reads edges applies at most its own cooldown's worth per
  tick. Write it on sim::PlayerCommand and in each edge-reading change-set's plan (4 jump, 8 fire/reload, 9 throw).
- The Worker, the endpoint and Sim::input are player-controlled in every mode: the post-order guard (host.ts) is
  correctness, never a security control; only the room's checks count.

## Answer (given to the main session through the hand-off)
VERDICT: PASS. M1 answer accepted (deferred, not closed; deadline sharpened: built in change-set 7 with each timing
way, hard stop before any room behind a network transport); L1 fixed; L2 answer accepted; L3 re-derived (Low, for
3c's plan); new: Low any_bytes.rs:214 test strength, nit sim-wasm.test.mjs:145; D: report follow-ups for change-sets
4, 8, 9 plus one sentence on sim::PlayerCommand.
DONE

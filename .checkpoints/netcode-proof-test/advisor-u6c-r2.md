# Advisor: U6c round-2 ruling (2026-11-24)

Seat: the advisor agent, called from the main session; rules read from `git show origin/main:.claude/agents/advisor.md`.
Read only, effort xhigh. Brief: u6c-r2-ruling-request.txt (sha256 9a57324e… checked, matches).

## Plan
1. Check every input's sha256 (done: all six match; see Notes).
2. Read round 1's ruling (advisor-u6c-r1.md), round 2's three reviews, the fix pass (u6c-build.md:214-296).
3. Read the code at the cited lines in the worktree (probe.rs:462/:465/:478; load.rs:675/:705-707; tests/load.rs:177-184;
   probe_tests.rs:358) and U8's snapshot (u8-build.patch:1708-1711, :2189-2221).
4. Read history.md:7022-7028 (U4f's M3-M9 conditions) and :2125-2146 (Mustafa's 2026-11-15 rule), and advisor-u6b-m6.md
   :153-165.
5. Rule item by item (1a, 1b, 2, 3, 4, 5), a notes line after each.
6. Name the tree that merges, who makes each change, the M3-M9 conditions, the effect of MAIN-65's order, For Mustafa.
7. End with DONE.

## Notes
- Hashes: advisor-u6c-r1.md ad72c0b8… OK; u6c-r2-netcode.md 444b0f78… OK; u6c-r2-test.md d06a8c4f… OK;
  u6c-r2-security.md 7dfe60b6… OK; u6c-build.md a9d45dd4… OK; wip/u6c-fix.patch 0549389e… OK.
- Read: advisor-u6c-r1.md (section 4: Critical/High after round 2 stops U6c; Medium/Low follow-up), the three round-2
  reviews (netcode 0 High, 0 Medium in diff, 2 Medium outside; test 1 Medium TE-U6C-R2-4 + 7 Low; security 2 Low).
- Code read (W = worktree spikes/netcode-proof): server probe.rs:1-30, :80-190, :255-439, :440-658 (run_leg: bind :460,
  pretouch :462, start edge :465, poller :478, wait_peer :480 with 1 ms polls :417-439, window start zero+warm, the
  seconds loop's edge hook :513-521, end edge :553); load.rs:600-709 (`seconds`: READY :667-671, Stopped at 30 s without
  READY :672-674, `over` :675, Clean :705-707; probe mode has no clients, so `ready` is true from :663, no READY).
- U8 read: :1690-1729, :2150-2293, :2425-2492 (probe_presence), :3860-3879, :5289-5318 (its fixture test). Every LG run
  with clients goes through lg_block, whose load_ready (:2193-2201) waits for READY in LG's stdout until t0 + warm - 10,
  else INVALID-ready.txt and the room down (:2220-2224). Nothing in U8 reads LG's exit code or DONE as a pass.
- C-U8-6's source: the advisor's U3D-RCVBUF ruling (advisor-u3d.md:251-277; plan.md:1256): a drop on P1's socket
  between the two reads makes ImpairSocket's self-test INVALID, cause named, to the advisor before any L2 mark counts,
  never re-run away (a drop = a reader stall of tens of ms, which t_in stamped at drain cannot show). U8's code already
  does this (probe_presence :2486-2488 prints "P1: INVALID: <d> drops … (C-U8-6)"; INVALID-presence.txt :2430; fixture
  :5312-5314); only its prose (:1709-1711, :2434-2436) says "goes to the advisor" without INVALID.
- Mustafa's 2026-11-15 words (history.md:2125): "A new bar, or a change to one, comes to me under Decisions needed; it
  is not built on the ruling alone." U4f's merge steps M3-M9: history.md:7022-7028.
- 1(a) mechanism, checked in the code (net socket.rs): the kernel reader is its own thread from the bind (:214-216,
  :586-646) and takes the shared lock per batch (:634); the pre-touch holds that lock through its fill and drain
  (:434-454). So the buffer fills during the pre-touch (the start values 679 and 569), and the start read (:465) races
  the reader's catch-up after the pre-touch, not the poller's start (the seat's mechanism slightly off; its conclusion
  stands). CORRECTED after the advisor tool's second check: the reader runs from the bind, so a datagram taken in
  between :460 and :462 may be the one that pins once the poller starts; a read after the pin is therefore not
  guaranteed to follow the reader's catch-up. The order guarantees only that it is never earlier than :465. Not read:
  `release`, `Shared::intake`, the tail of `bind`.
- A2 logs still in /tmp (load.jsonl, room.jsonl; a `tokens` file beside them, 0600): item 4's look needs no run.
- Item 4 look (read only, python -I reading /tmp/npt-u6c-load-7849-a2-wt/room.jsonl): `second.s.out_queue_count` (the
  room's outbound queue per client, SERIES.md:416) is 1 for all 48 clients in every second 0-70, slot 41 included;
  `wt_accept`: accepted 48, handshake_failed 0, deadline_closed 0, `datagrams_dropped_count` 225 ("client datagrams a
  full session inbox dropped", SERIES.md:891; not split by client). The room logs no per-client inbound lateness.
  (`stand_in_changes_count` is a game-entity count, SERIES.md:96: not used.)
- pretouch_fate_log holds the guard through the fill and the drain (socket.rs:436-454). SERIES.md:914-915 places the
  start read "(after the bind, before the warm-up)": a third place to edit if the read moves.
- Item 5 look (read only, .gate/u6c/read-probe/room-P1.jsonl): measured seconds whose up records (`probe_up_lost`
  entries, by t_in) are not 10,000: 39 (9,738; steal 26; `l2_late_up_max_ns` 37.7 ms), 40 (10,140; 32.0 ms), 41
  (9,998), 42 (10,003), 57 (9,907; steal 28), 58 (9,999), 59 (10,001; 26.1 ms), 110 (214, the last record's second).
  Net -214 = the up span's excess. So the in-window drops are two events (about 122 at seconds 39-40, about 93 at
  57-59), each with the up release side tens of ms late in the same seconds; steal there 26 and 28 per mille, inside
  the window's 0-32. Cause not established (the process held off the CPU, or a lock held: the log cannot tell).
- Item 4 checks: marks/marks.json and crates/judge do not contain `datagrams_dropped` (grep); `wt_accept` sits at
  room.jsonl line 2180 of 2183, ph `after`, right after the last tick, so the 225 are not drops from LG's 30 s tail;
  when in the run they fell is not in the log. marks.json's one `inputs_count` check is the fuzz mark's
  `event.fuzz_end.inputs_count` (:7183): no mark reads LG's per-client seconds.
- D10 basis: wip/u6c-fix.patch has no `diff --git` header under spikes/netcode-proof/marks/marks.json or crates/judge
  (grep count 0).
- STAMP-HOME (msg-2026-11-25.md:69): where U6c's three held-back stamp tests (about 170 lines) get built.
- Only the netcode seat called the container's P1 drops "the rule working, not a defect"; the test seat did not read U8.

## 1. Verdict: APPROVE U6c's merge, with one change before M5, and a second on Mustafa's yes
Round 2 left no Critical and no High, so by my round-1 section 4 every Medium and Low is a follow-up, and no round 3
runs. Item 2's one-line test fix goes in before M5, because M5 is the merge's one counted run and cannot be repeated
cheaply. Item 1 (a)'s fix, a moved read, would narrow what an INVALID rule counts. On the brief's reading of his rule
of 2026-11-15, that goes to Mustafa first (NPT-EDGE-READ, section 7). The main session makes each change by hand
after M4; their only review is my head read before M5 (section 4). U6c's report says the merged tree differs from the
one round 2 reviewed by exactly these edits.

## 2. The five items
**Item 1 (a), the start-edge drop read: fix before R1, on Mustafa's yes (NPT-EDGE-READ); the main session makes it
by hand (H1, section 3).**
- Mechanism, corrected. The seat says the read at server probe.rs:465 races a full buffer until the poller's first
  receive. The code says otherwise in one step:
  - the kernel reader is ImpairSocket's own thread, running from the bind (net socket.rs:214-216, :586-646);
  - it takes the socket's lock for each batch (:634);
  - the pre-touch holds that lock through its whole fill and drain (:434-454).
  So the buffer fills during the pre-touch (the start values 679 and 569). The read at :465 then races the reader's
  catch-up after the pre-touch releases the lock; the poller does not matter. The seat's conclusion stands: the start
  read can sample the counter before the last drops of a full buffer.
- Severity: I keep the seat's Medium, but re-reasoned under the real rule.
  - The seat weighed "one false advisor question", which is U8's softened prose. Under plan.md:1256, and U8's own code
    (item 5), a drop between the two reads makes the self-test INVALID, never re-run away. So when the race hits, R1's
    P1 self-test is lost.
  - It stays Medium on reach. A drop must land after the counter is sampled and before the reader's first drain after
    the lock's release. The reader wakes at that release, while the sample needs the thread to open and parse
    /proc/net/udp first; I reason, without measuring, that the reader usually wins on a lightly loaded machine.
  - On this loaded container, the drops outside the window (rise minus the up span's excess) were at most about 1 on
    P0 and about 8 on P1. That bounds the race from above there.
  - So: a real defect with a small reach and a large cost when it hits.
- The fix is mine, not a reviewer's: take the start-edge reads (`leg_edge`, `cpu_stat`) just after `started`
  succeeds, at the leg's zero, instead of just after the pre-touch.
  - The order guarantees only this: the new read is never earlier than the old one. The drops counter only rises, so
    a later read can only move drops from "between the edges" to "before" them.
  - In practice the read comes after the poller has started, after the pin, and usually after at least one of
    wait_peer's 1 ms polls. That is long against the reader's wake after the lock's release.
  - It does not prove that no race drop is left. The reader runs from the bind, so a datagram taken in before the
    pre-touch can pin as soon as the poller starts.
- Evidence: no test can show it, because it needs a full kernel buffer and a slow reader wake. M5 would prove only
  that nothing else broke. It is first seen at R1, or on any later probe reading: each leg's rise minus its up span's
  excess (item 5's report line) should be 0 unless the reader falls behind outside the window.
- Why it goes to Mustafa: there is a case that is INVALID before H1 and not after it, a drop between the pre-touch's
  end and the leg's zero. The brief reads his rule of 2026-11-15 as covering INVALID rules. On that reading, a change
  to what one counts is his. I rejected the seat's form on the same ground: it compares from the window's start, which
  also stops counting stalls in the 10 s warm-up, and it adds a read inside the timed loop, whose reads C-U6-7 limits.
- The path into R1's binaries:
  - If his yes is in before U6c's merge starts, H1 goes in at M4a.
  - Else U6c merges with H2 alone, and H1 follows his yes as its own small change-set before Thu 3 December: plan, my
    check, the approved dev part-clean, one counted run. No other step before R1 touches the server's code.
  - On a no: no change. A race drop at R1 makes P1's self-test INVALID, with its cause named, and it comes to the
    advisor.

**Item 1 (b), npt-load ends clean with no READY: carry, after R1; taker: the next unit that changes
crates/bots/src/load.rs.**
- At R1 it gives no wrong verdict. Every LG block with clients goes through `lg_block`, whose `load_ready`
  (u8-build.patch:2193-2201) waits for READY on LG's stdout until `t0 + warm - 10`. Without READY it writes
  INVALID-ready and brings the room down (:2220-2224). Probe mode has no clients and prints no READY (load.rs:663).
  Nothing in U8 reads LG's exit code or DONE as a pass.
- What is lost is the diagnosis: when every client fails before READY (a wrong hash file), LG's stdout says DONE
  and drops NC-U6C-5's `handshake_failed_count` line.
- The fix is the seat's: in `seconds`, `Ended::Stopped` when `over && !ready`. It needs a new integration test (a wrong
  `pin_hex`: exit 1 and the line) and its plant, which is more than a head-read change. Medium, a STATUS follow-up.
- For R1's reader, C9 adds one line to R1.md: a LG stdout that ends DONE with no READY means every client ended
  before READY (U6c follow-up). The INVALID-ready verdict stands; the room's `wt_accept` or `ws_accept` counts give
  the cause.

**Item 2, TE-U6C-R2-4 (the WebTransport four-client test counts a post-close second): fix before the merge (the main
session, by hand: H2 in section 3).**
- The seat's one line: `&measured[1..measured.len() - 1]` in place of `- usize::from(transport == "ws")`.
- WebSocket's slice is unchanged. WebTransport loses only second 3, which starts after the room's close (the room
  ends at OPEN + 3 s, tests/load.rs:139; LG runs to 4 s, :234).
- The last-second index asserts (:185-193) stay, so the WT run is still shown to end at its duration.
- Why now: M5 is the merge's one counted run, and every later check.mjs runs this test. A flake there stops M5 at M6.
  After a cold M5, the disk may sit under main-run.sh's 4 GB floor, so a re-run then needs the dev part-clean first.
- Evidence: by reading, the change removes one post-close second from an `all` and can hide nothing that happens
  while the room runs; M5 runs it. No plant: the seat's plant (a 500 ms sleep before LG starts) would cost a runner
  call on a disk that M5 needs.

**Item 3, FP-3's wording: closed in substance; no plant before the merge.**
- My round-1 words "it must fail that assert" were not met. The shipped-code P2 plant fails the same end-to-end test
  earlier, at the header's `base_rtt_ns` check (crates/server/src/probe_tests.rs:358; plant-p2b.log). So FP-3's assert
  (:367-375) is not shown to bite.
- F-P2's defect, a socket carried across legs in shipped code, is caught by a test. That was the point of FP-3.
- U6c's report says exactly this, in the builder's words (u6c-build.md:283-287).
- TE-U6C-R2-2 (Low) becomes a STATUS follow-up: the variant plant P2c (P2b plus one line writing `base` into P1's
  header) at the next change to crates/server's probe tests. Running it now would cost a runner call against the disk
  M5 needs.

**Item 4, slot 41's late commands over WebTransport: after R1 (U6d or U6e, with F-WT-CLOSE), as the netcode seat
says.** The test seat's "look before R1" is made here, read only, from the left-behind A2 logs:
- The room's per-client outbound queue (`second.s.out_queue_count`, SERIES.md:416) is 1 for all 48 clients, slot 41
  included, in every second 0-70. So the room never held sends back for any client.
- The room logs no per-client inbound lateness, so the look cannot say whether slot 41's commands reached the room
  late while it ran.
- `wt_accept` shows 48 accepted, 0 failed handshakes, and 225 client datagrams dropped at a full session inbox
  (SERIES.md:891), not split by client. No mark or judge code names it (grep). The event is written right after the
  room's last tick (room.jsonl line 2180 of 2183), so these 225 are not from LG's 30 s tail. When in the run they fell
  is not in the log. Untraced; no ruling.
- Nothing here bears on an R1 verdict. In R1's LG blocks no measured tick follows the load (u8-build.patch:2204-2207).
  marks.json's one `inputs_count` check is the fuzz mark's (:7183).
- U6c's report quotes slot 41's per-second facts, the out-queue reading and the 225 as reported facts. R1's 48-client
  WebTransport readings name slot 41's pattern as a known open.
- The main session copies both A2 folders, without their `tokens` file, into .gate/u6c/a2-logs/ before /tmp is lost.

**Item 5, the in-window drops: U8 compares exactly C-U8-6, and a P1 drop is an INVALID self-test that comes to the
advisor. It is not a reported number, and not only a question.**
- The rule is decided: the advisor's U3D-RCVBUF ruling of 2026-11-09 (advisor-u3d.md:258-262), carried as C-U8-6 in
  plan.md:1256. A drop means the reader fell a full buffer behind (about 28 ms), the class of pause that decides E,
  which a t_in stamped at the drain cannot show. A pass kept by re-running would be a pass by selection.
- U8's code already does exactly this. probe_presence compares P1's `leg_edge.udp_drops_count` at `after` with
  `warmup` and prints "P1: INVALID: <d> drops on the probe's P1 socket between its reads (C-U8-6): to the advisor
  before any L2 mark counts" (u8-build.patch:2486-2488). The block keeps it as INVALID-presence.txt (:2430), and the
  fixture test expects it (:5312-5314). Presence at both edges is checked on every leg.
- Only U8's prose is weaker. The two texts, side by side, so "restored, not changed" can be judged:
  - plan.md:1256: "a drop on the probe's P1 socket between the two reads makes ImpairSocket's self-test INVALID with
    its cause named, and that leg comes to the advisor before any L2 mark counts with its printed E, its max and the
    counters, never set aside and re-run as noise".
  - u8-build.patch:1709-1711 (and the comment at :2434-2436): "`udp_drops_count` at both (equal at both on P1) … a
    missing read makes that leg INVALID, and a P1 drop goes to the advisor before any L2 mark counts".
  - At C9 the main session puts plan.md:1256's sentence into those two texts as written, and adds C-U8-6's condition
    to the self-test row's INVALID list (:1702-1704). The code and the fixture need no change.
- One addition at C9, reported and never judged. probe_presence prints per leg:
  - the rise between the edges;
  - the up span's excess in intervals: about `probe_up_window_ns` ÷ (1 s ÷ `probe_rate_pps`) − the header's
    `probe_sent_count`, so B1's burst is inside the subtraction;
  - the rise minus the excess.
  These give the advisor's question at R1 the evidence form used here.
- On this container the rule tripped. The netcode seat calls that the rule working, not a defect; I agree.
  - P1 rose 222. The P1 log puts 214 of them in the window as two events: about 122 at measured seconds 39-40 and
    about 93 at 57-59. The up records per second there were 9,738 then 10,140, and 9,907 then 9,999 and 10,001.
  - In those same seconds the up release side ran 26-38 ms late (`l2_late_up_max_ns`), with steal at 26 and 28 per
    mille, inside the window's 0-32.
  - I have not established the cause: the process held off the CPU, or one of its locks held long. The log cannot
    tell, and a lock would recur on R1's machines.
- If it trips at R1, R1's P1 self-test is INVALID and the L2 marks wait for the advisor. The remedy is the stall's
  (U3D-MISS), never a bigger receive buffer.

## 3. The changes (exact text, by the Edit tool, in the main folder after M4)
**H2 (item 2), before M5 in every case: spikes/netcode-proof/crates/bots/tests/load.rs, two edits.**
- H2a (:132): replace `to the last whole one)` (unique in the file) with `to the last but one)`; the rest of the line
  unchanged.
- H2b (:176-177). Old:
  ```
      // F-NAMES: every whole measured second (the close ends the WebSocket run inside its last second).
      let whole = &measured[1..measured.len() - usize::from(transport == "ws")];
  ```
  New:
  ```
      // F-NAMES, TE-U6C-R2-4: every measured second but the first (READY's) and the last (WebSocket: the close's part
      // second; WebTransport: it starts after the room's close, so only the clients' lead gives commands in it).
      let whole = &measured[1..measured.len() - 1];
  ```
  (`transport` is still used at :186 and :214; `measured.len() >= 2` at :171 keeps the subtraction safe.)

**H1 (item 1 (a)), only on Mustafa's yes to NPT-EDGE-READ: spikes/netcode-proof/crates/server/src/probe.rs, three
edits, and SERIES.md, one.**
- H1a, the module doc (:7-10). Old:
  ```
  //!    sized for the whole leg, pre-touched (C-U6-3); the start-edge reads (`leg_edge`, `cpu_stat`: C-U8-2, C-U8-6);
  //! 2. the poller on the runtime (C-U6-1); the first datagram from the peer's IPv4 (within 30 s, else the run ends
  //!    non-zero), then `set_peer` on that address and the down pacer, which so never sends before the peer (C2); every
  //!    other source dropped and counted;
  ```
  New:
  ```
  //!    sized for the whole leg, pre-touched (C-U6-3);
  //! 2. the poller on the runtime (C-U6-1); the first datagram from the peer's IPv4 (within 30 s, else the run ends
  //!    non-zero), then `set_peer` on that address and the down pacer, which so never sends before the peer (C2); every
  //!    other source dropped and counted; then, at the leg's zero, the start-edge reads (`leg_edge`, `cpu_stat`: C-U8-2,
  //!    C-U8-6), later than the pre-touch, which holds the lock the kernel reader takes;
  ```
- H1b, remove the old read (:464-465). Old: `    let port = s.bind.port();` + newline +
  `    let before = edge_events(&sock, port, Phase::Warmup);`. New: `    let port = s.bind.port();`.
- H1c, the read at the leg's zero (before :495). Old: `    let pacer_reads = (pacer.slack_ns(), pacer.policy().to_owned());`.
  New:
  ```
      // NPT-EDGE-READ (U6c round 2, item 1 (a)): the start edge at the leg's zero, not just after the pre-touch, which
      // holds the lock the kernel reader takes, so the buffer can still be overflowing then. The drops counter only
      // rises, so a later read can only move such drops from between the edges (C-U8-6) to before them.
      let before = edge_events(&sock, port, Phase::Warmup);
      let pacer_reads = (pacer.slack_ns(), pacer.policy().to_owned());
  ```
- H1d, spikes/netcode-proof/marks/SERIES.md:915: replace `(after the bind, before the warm-up)` (unique in the file)
  with `(at the leg's zero, the peer pinned)`; the rest of the line unchanged.
- Check by reading at the head read: `before` is still read before `drop(sock)` (:571), so FP-2's compile hold
  stands; `port` is still bound at :464; nothing else reads `before` before :653; the Err path (:489-493) needs no
  `before`.
- Size: H1 and H2 together add about 6 lines to 5,693, under 5,850. Neither adds or removes a test.

## 4. Which tree merges, and who makes the changes
- **The tree:** the fix-pass snapshot wip/u6c-fix.patch (0549389e…) as it is, plus H2, plus H1 if Mustafa's yes to
  NPT-EDGE-READ is in before M0. Nothing else.
- **Who:** the main session, by hand, in the main folder, with the Edit tool, after M4's apply and cmp check.
  - M4's cmp covers the 24 files before the change.
  - The edits are their own logged hunks: `diff -u` of each changed file (the worktree's copy against the main
    folder's), saved to .gate/u6c-merge/hunks.diff with its sha256.
  - Not U6c's builder: the edits are given in full here, and waiting for 03:50-04:20 UTC (lean mode item 3) buys
    nothing.
- **My head read** (read only, one advisor call, before M5):
  - the hunks file matches its sha256;
  - it touches exactly the files named (two with H1, or one without) and equals section 3's text, or differs only by
    rustfmt's own changes;
  - the other files still cmp-equal the worktree.
  APPROVE, then M5. REVISE means fix by Edit and read again; no M5 before APPROVE.

## 5. The merge's conditions (U4f's form, history.md:7022-7028)
- **When:** M0 starts once MAIN-65 has merged and Mustafa has answered NPT-EDGE-READ. If his answer is not in by the
  end of Fri 27 November (Amsterdam), the merge runs with H2 alone. That keeps C4's "merged by the end of Sun 29", and
  H1 then follows a yes as its own change-set before R1 (item 1 (a)).
- **M0, after MAIN-65 has merged** (by its own plan and advisor check):
  - wip/u6c-fix.patch still hashes 0549389e….
  - `git diff --stat 8c81952 HEAD -- spikes/netcode-proof` is empty on the post-MAIN-65 head.
  - Copy the A2 folders, without `tokens`, to .gate/u6c/a2-logs/ (item 4). Do this one now, whatever the timing.
- **M3:** `git apply --check` of wip/u6c-fix.patch on the post-MAIN-65 head: exit 0, 24 files, all under
  spikes/netcode-proof/. Any failure stops and comes to me.
- **M4:** `git apply` in the main folder, uncommitted. The 24 files cmp-equal to the worktree, which is kept until M8.
- **M4a:** H2, and H1 on his yes, by Edit.
  - Then `cargo fmt --check` for the spike workspace (it builds nothing): exit 0, or rustfmt's diff applied by Edit
    and included in the hunks.
  - Then hunks.diff and its sha256 (section 4).
- **M4b:** my head read, APPROVE.
- **M5:** one counted run.
  - Right before it, measure free disk: at least 8,192 MiB (advisor-u6b-m6.md:158-165). Measure after MAIN-65, which
    may have used disk. Under the floor, run the approved dev part-clean first.
  - Run main-run.sh in the background, timeout 7200000, with `env CARGO_INCREMENTAL=0 node check.mjs`, the setting
    shown in the log (as .gate/u6b-merge/m5-check2.log:7; advisor-u6b-m6.md:153-157).
  - Expected: all 11 steps PASS; measured 861 and bites 958 (no test added or removed); 0 failed; wasm 27,000 ticks,
    0 mismatching; marks 81; self 78/78; redact 0 findings.
  - The tree hashes differ from fix-check.log's (0e3199ae…, 22bda910…) by the hunks only; the report names both pairs
    and the hunks' sha256.
- **M6:** a failing step or test stops and comes to me. There is no re-run on its own: after a cold M5 the disk may sit
  under main-run.sh's 4 GB floor, so a re-run needs the approved dev part-clean first.
- **M7:** `node tools/gate.mjs --no-smoke` PASS, against the post-MAIN-65 base.
  - The D10 read, on the working tree after M4a: `git diff --stat 8c81952 -- spikes/netcode-proof/marks/marks.json
    spikes/netcode-proof/crates/judge` is empty. The snapshot has no file there (its `diff --git` headers), and neither
    H1 nor H2 touches one.
  - The report says in its own line: "no change to marks.json or the judge" (Mustafa, 2026-11-15, history.md:2127).
- **M8:** one commit, containing:
  - the 24 files with the hunks;
  - `git rm` of the wip patches;
  - u6c-build.md's merge section;
  - u6c-report.md, with:
    - this ruling's per-item lines;
    - the sentence "the merged tree differs from the one round 2 reviewed by H2 (and H1), reviewed by the advisor's
      head read only";
    - item 3's wording;
    - item 4's facts;
    - NC-U6C-R2-1 and TE-U6C-R2-7 (the A2 prints: the min spans post-close seconds; `ws_split` is Some(zeros), not
      None);
    - Q-FP6-1;
    - the size;
    - every round-2 Medium and Low not fixed, as a STATUS follow-up with its taker (1 (a) as well, if H1 waits);
  - the history entry.
  - The two security Lows (SEC-U6C-R2-1, SEC-U6C-R2-2) are open security findings. They keep PR #40 needs-mustafa at
    its review (advisor.md, PR review step 4).
- **C9 (U8's snapshot, the main session, with M8):**
  - round 1's NC-U6C-12 and SEC-U6C-5;
  - item 5: plan.md:1256's sentence in U8's prose (:1709-1711, :2434-2436), C-U8-6 in the self-test row's INVALID list,
    and the per-leg report lines (rise, excess, difference);
  - item 1 (b): R1.md's DONE-without-READY line.
- **M9:** the push; the saver; CI on PR #40 once GitHub reports it mergeable (after MAIN-65); the card.

**MAIN-65's order changes these conditions in four places:**
- M0 and M3 run on the post-MAIN-65 head.
- The 8,192 MiB check moves to right before M5.
- M7's gate base is the post-MAIN-65 merge-base.
- CI waits for MAIN-65.
Nothing else: PR #65 touches no spikes/ path, and the spike has its own Cargo.lock and rustfmt.toml.

## 6. Run conditions, INVALID rules and bars (his rule of 2026-11-15)
His words (history.md:2125): "A new bar, or a change to one, comes to me under Decisions needed; it is not built on
the ruling alone." The brief reads that rule as covering run conditions and INVALID rules too; I take that reading.
- Item 1 (a)'s H1 narrows what C-U8-6 counts: a drop between the pre-touch's end and the leg's zero is INVALID before
  H1 and not after. So it goes to him: NPT-EDGE-READ, section 7. It is not built before his yes.
- Item 5 changes no rule. U8's code already applies C-U8-6 as decided. Only U8's prose is brought back to
  plan.md:1256's words, and the added lines are reported, never judged.
- Items 1 (b), 2, 3 and 4 touch no rule.

## 7. Decisions needed
- **NPT-EDGE-READ (new): where the probe takes its first receive-drop read on each leg.**
  - The question, in plain words: may the server take that first read at the moment the leg starts, rather than right
    after it sets the leg's socket up?
  - Why it matters: on the P1 leg, any datagram the operating system drops between the leg's two reads makes
    ImpairSocket's R1 self-test INVALID, and it is never re-run. Right after set-up, the server's own preparation has
    just blocked the reader, so the buffer may still be overflowing. A read taken then can count a drop that is not a
    stall during the leg. A later read stops counting such a drop, which narrows what the rule counts. So it is his.
  - Options:
    - (a) Yes: move the read to the leg's start, a one-line move reviewed by the advisor's head read. It goes into
      U6c's merge if his yes comes first, else into a small change before R1 with one counted run.
    - (b) No: the read stays. If such a drop happens at R1, P1's self-test is INVALID and comes to the advisor.
    - (c) Count from the measured window's start instead. That is more certain, but it also stops counting stalls in
      the 10 s warm-up, and it adds a read inside the timed loop.
  - Recommendation: (a).
- Tonight's two other questions stay as written:
  - NPT-L2-ONEWAY: whether -3's real-network one-way delay checks read against the profile's one-way delay minus half
    the measured base round trip.
  - STAMP-HOME: where U6c's three held-back stamp tests get built.

## 8. For Mustafa
1. Answer NPT-EDGE-READ, NPT-L2-ONEWAY and STAMP-HOME in tonight's message; a letter each is enough.

Information only, nothing to decide:
- The receive-drop rule that makes R1's P1 self-test INVALID (C-U8-6) tripped on the container's own pre-R1 reading:
  two drop events, about 120 and 90 datagrams, cause not established.
- If it trips on R1's machines, R1's L2 marks wait for the advisor.

## 9. What I did not check
- I ran nothing: no cargo, test, plant or runner call. My reads of the logs were python -I and grep, read only.
- H1's effect is reasoned from the code's order, not measured. The race's reach ("the reader usually wins") is an
  estimate. I did not read `release`, `Shared::intake` or the tail of `bind`, so I claim only what the order
  guarantees.
- The cause of P1's two in-window drop events is not established.
- When the 225 dropped datagrams fell, and from which clients.
- Slot 41's cause.
- U8 was read only at the ranges in the notes. MAIN-65's plan was not read: its disk use is unknown, hence M5's
  re-measure.
- I did not re-read round 2's other Lows against the code; per my round-1 section 4 they are follow-ups.

DONE

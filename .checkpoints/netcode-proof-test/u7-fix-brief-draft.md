# U7's fix pass: the brief, DRAFT (the main session, 2026-11-26 06:0x UTC)
Not sent until the advisor's round ruling (advisor-u7-round.md) strikes or adds; its rulings win over this draft. The
items come from the two seats' "smallest fix" columns (u7-review-netcode.md, u7-review-test.md). To U7's builder by
SendMessage; its worktree .claude/worktrees/agent-a9873cdba07a3219e; its notes u7-build-rs.md, new section "## Fix pass".

0. Base: `<base>` = the lane head at sending (6913070 or later; since 9177fb5 only U8's 43 files under ops/ and notes,
   nothing under crates/, web/ or check.mjs). The main session runs `node .checkpoints/netcode-proof-test/ffcheck.mjs
   <worktree> 9177fb5 <base>` first (want 0 ADDED present, 0 differing). In your worktree, touching no file first:
   `git merge --ff-only <base>`; if git refuses, stop and report its message; remove, restore or move nothing. Then
   `git log -1 --format=%H` into the notes, with your plan for the items below before any edit.
1. The items (each: its test red before or its plant caught, then green, in the notes):
   - F1 (U7R-NC-1 with U7R-TE-2 (a), Medium): one RoomHost test with a 40 ms path each way and a 5 ms host send lag
     (the client's sends stamped now + 45 ms with prev_send_ns = now + 5 ms; the room's at now + 40 ms): Playing, no
     refusal, the newest clock sample's RTT 80 ms within 1 µs. Plant: engine.rs:207-209 (`sent(prev_send_ns)`) removed
     must fail it (85 ms). The report says jitter and loss stay with the endpoint's lab suites.
   - F2 (U7R-TE-2 (b), Medium): worker.test.mjs's fake writer advances the FakeClock 1 ms per write; assert prevSendNs[i]
     = nowNs[i-1] + the sends' time. Plant: stamping before the sends must fail it.
   - F3 (U7R-TE-3 with U7R-NC-2, Medium and Low): presses counted from the commands issued (at most one edge per
     command); one in-process leg with late wakes (every 10th wake 200 ms late, one 1 s stall, one 3 s gap): no
     RefusalRecord, the slot's 8 refusal counters at 0. Plant: the step-count press counter back must fail it after the
     3 s gap. The report: mark 9 NOT-RUN; the `refused` column named "client decoder refusals, not the room's".
   - F4 (U7R-TE-4, Medium): the web step fails when the test count is 0; a check.test.mjs recorder case "# tests 0".
   - F5 (T5ii-D, the same as U7R-NC-7, Medium): the Worker and the page count `securitypolicyviolation` events into
     `page_csp_violations`, read by the driver; a page-side refusal in the probe as the positive control.
   - F6 (Lows in the same files): NC-4 (`fail` kills every child it started and removes the current tokens file;
     t5i.mjs's two lines through `clean()`); NC-5 (`history.replaceState` after a good parse; a Node test on a pure
     helper); NC-6 (the partial second written at end(); `second` armed on the Worker clock's grid; a test each); TE-7
     (the u64::MAX case under a step bound; one Core test with a non-zero `refused`, read by name); TE-8 (JoinAccepted
     alone writes no attempt.ok; a stuck writer; a 5th uni stream; bufferedAmount over the cap; a fake with
     createWritable); TE-9 (build.mjs scans the seven real dist files for PM-0001- and innerHTML); TE-10 (`--commit`
     required or read by git; the dist build log's hash in summary.json).
   - Wording only, in the report: NC-3 (mark 8b as "a fallback later than 3 s, or none in 10 s"; the block must DROP,
     never REJECT, for R8's bite and `npt-ctl quic off`), NC-8 (the column's name; the pin's evidence is marks 1 and 4,
     not C8), TE-6 (C2's two halves echo the room host's own hashes), TE-11 (the records: 12 steps, the patch's size).
   - TE-5 (Low): a WebSocket leg without ignoreHTTPSErrors (the test certificate trusted by an SPKI flag) inside the
     T5 (ii) rerun below, its handshake_failed cause recorded; else a carry to before R1.
2. The end: `node check.mjs` whole through u7-run.sh -x (12 steps; the test counts rise by the new Rust tests);
   `node --test web/test/*.test.mjs`; then one T5 (ii) rerun on the fixed tree (U7R-TE-1: shipped web and Wasm sources
   change, so run 3 no longer stands for the reviewed page), with run 3's flags (--no-proxy-server, C8 local, the
   tokens rule) and every mark of run 3 read again.
3. As brief 3 (launch-0850.md section 3, items 2-4b and 6): every cargo call through u7-run.sh with `--locked`;
   NPT_WASM_BINDGEN as there; build outputs under .gate/ only; Cargo.lock unchanged; SIZE-CHECK (now about 4,731 added;
   past 6,000 the unit stops and reports); notes after each block. Waits: run each long call in the background with a
   long timeout and read its log when told it ended; no shell loops, not even read-only ones (brief 3's slip,
   19:4x-00:16 UTC). Write files with the Write and Edit tools only (no sed, no shell redirect, no python writes; never
   `cargo fmt` writing files: apply rustfmt's diff by Edit); an agent with no Edit tool rewrites its notes file whole
   with Write. Never switch the main folder; never `cd` into another worktree; no `git -C` and no `git -c`. git and gh
   always start their own command, with no other program in front of them.
4. At the end: the items, tests, plants, added lines and open items, in a short summary; I take the snapshot.

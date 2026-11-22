# netcode-architect: U6a build brief, rev 1 (plan and notes)

Task: draft `.checkpoints/netcode-proof-test/u6a-brief.md`, rev 1, for U6a (server core, NPT-TRIM cut).
No code, no build. Write only (this agent has no Edit tool: the notes are rewritten whole with Write).

## Plan
1. Read memory/notes (netcode-architect-notes.md, digest) for earlier decisions.
2. Read history.md:7561-7607 (NPT-TRIM, Mustafa's 2026-11-22 answers).
3. Read replan-units.md §1, §7, row 58; replan.md.
4. Read u6-brief.md rev 2.1 (§1, §3-§7, §9); take U6a's blocks (S1, S8a, C-U6-17/18, T6, S4).
5. Read u6-facts.md (file:line facts) and advisor-u6-brief.md (conditions by number).
6. Quote plan.md:1254 C-U6-17, C-U6-18, C-U6-4 word for word; u4s-carries.md carry 60 word for word.
7. Read u4f2-brief.md template (build rules, close steps); dependency-check.md.
8. Draft u6a-brief.md; check line count; note contradictions with the re-plan.
9. Hand back a <=15-line summary.

## Notes (one line per finding)
- Steps 1-7 DONE (read: history.md:7561-7607; replan-units.md whole; replan.md; u6-brief.md rev 2.1; u6-facts.md;
  advisor-u6-brief.md; advisor-replan.md's grep for U6a; plan.md:1254 row; u4s-carries.md:286-290; u4f2-brief.md
  :150-266; u4f-brief.md:288-355, :831-958; dependency-check.md; security-r1-deps.md B0-B2 in progress).
- Row drift: U6's row is plan.md:1254 now (replan-units.md:3 and u6-brief.md cite :1253). Quotes taken from :1254.
- F1 (-16): writer is npt-super (SERIES.md:200; ADR 0001:1604 "timed by the supervisor's stand-in"); npt-super is H3 in
  U6e (replan-units.md:62), after R1; yet U6a is "first needed by R1: -16" (:58). Gap. Lean: H3's smallest slice
  (spawn, time to Open, write the event) into U6a, ~150-200.
- F2 (-2): -2's RS part needs the room process's room log with -2's npt-room series and tick.s.everyone_in_view;
  only npt-lab writes them (lab/src/logs.rs:653-715, mirror.rs); nothing in npt-server. Re-plan's S4 names
  "instrumentation, header" only. Lean: U6a's room log writes them; everyone_in_view from a room getter over
  ClientSlot's `relevant` (room.rs:248, crate-private today). ~200-300.
- F3 (C-U6-16 (c)): TickLoop must send take_control's bytes on a channel without lab's stream_of copy
  (row bullet 39 (c)); TickLoop is U6a's S1 but the re-plan puts -16 (a)(c) in U6d (replan-units.md:61). (c) is a
  property of S1 itself (u6-facts.md:191-193 put it in S1). Lean: (c) in U6a via an npt-proto public channel read.
- F4 (T6 vs Q6): plan.md:577-579 and advisor-u4.md:202 (Q6): `loss` joins with rate adaptation, its first consumer,
  and send-buffer space with it; re-plan puts T6 in U6a (:58) and rate adaptation in U6d (:61). `Send` is what S1
  needs. Ask the advisor.
- F5 (C-U6-19's gate): register 119 / U6-Q21 said C-U6-19 closes at U6a's merge; the re-plan moves -19 to U6d
  (:48-50, :61), so the gate line now binds every brief before U6d's merge, U6a's included. U6a states the line.
- F6 (affinity): E2 needs the tick thread pinned alone; npt-server forbids unsafe; no crate or method in the plan
  sets affinity (grep: no core_affinity/nix/taskset in plan.md). Ask: taskset from the room vs a new crate.
- F7: L1's End and Loopback are Rc<RefCell> (l1/src/lib.rs:1747; link/src/lib.rs:133): `Send` cannot go on `Link`
  itself; TickLoop takes `Box<dyn Link + Send>`; Loopback gets Arc<Mutex> or a Send twin; L1 stays !Send.
- F8: npt-server's manifest gains tokio and sha2 (both in the lock, no new package): DEP-HOLD; not in
  security-r1-deps.md's candidate list (its lines 1-5). Name them for that read.
- F9: C-U6-14's first clause (Kick(replaced), close) is TickLoop routing; re-plan puts C-U6-14 in U6d. Lean: U6a
  routes kicked/left/replaced sessions closed; generation-or-test proof stays U6d.
- F10: C-U7-3's JoinAccepted field "joins U6a" if kept (replan-units.md:221), but U7's brief check is after U6a's
  merge (:199-200). Ask now; lean no wire change.
- Bites U6a plants: PM-0001-16/double-load, PM-0001-1/shared-vcpu, PM-0006-11/no-smt-map (marks.json bites).
- check.mjs: 11 steps (fmt, clippy, clippy-wasm32, doc, test, deps, wasm, refs, marks, self, redact; check.mjs:2614).
- Counts: U4f2 head 709 measured / 806 bites (u4f2-report.md:136); `<base>` fills at launch.
- Step 8 next: draft u6a-brief.md.

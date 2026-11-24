# Advisor: read before 3b's first edit (2026-11-24)

Asked for by Mustafa's message of 2026-11-24, 22:44:50 UTC, point 4 (file:
`.checkpoints/cs3b-loop-closes/mustafa-2026-11-24T224450-merged-65-next-3b.md`). Worktree `.gate/ci-wasm-wt`, branch
`cs/3b-loop-closes` = main 28254a73 + notes commit 51daf6c9 (only `.checkpoints/` files; checked with git diff).
Read-only; this file is my only write. Working from main's `.claude/agents/advisor.md` (last changed 97e2e3bb, 2026-11-13).

## Plan
1. Read his message whole; the resume card; the plan (draft 3) sections 5, 8, 9, 12, 13; the two designs' blocks; my own
   review of 2026-11-19.
2. Item (a): plan's decoder-test numbers vs ADR 0001 row PM-0001-34 (line ~1626); 3b's message types; cost in lines and
   run time (debug `cargo test --workspace --locked`, run twice in CI per toolchain.yml; release); counting allocator
   (`crates/sim/tests/state_hash_alloc.rs`); seeded inputs vs the rng rules (`crates/sim/src/rng.rs`).
3. Item (b): the build plan's two places that say the proof test chooses command timing (quote exactly); OQ-0001-6 at
   ADR lines ~252 and ~1156; does any 3b step depend on the choice.
4. Item (c): link simulator in the plan, its parts and ladder place; does the ladder need a "tell him before" line.
5. Section 5 vs main 28254a73: 3a's changes (scan refuses mul_add, per-processor code, outside-src source; test-only
   module built to `.gate/rules-module/`; feature compare; dependency hold); plan lines assuming a CI edit by this chat;
   3A-FROM-F64, 3A-HOOK-TEST, 3A-MANIFEST-COMMENT.
6. Size: 3,800 / 7,600 / ladder from 4,900 vs the three items.
7. Compaction: card enough to rebuild from?
8. "Decisions of 21 and 22 November": search main for any record.

## Findings (as I go)
- F0 (setup): branch = main 28254a73 + notes commit 51daf6c9 (13 files, all `.checkpoints/`); origin/main = 28254a73;
  main's advisor.md last changed 97e2e3bb (2026-11-13), 335 lines. SHAMBLE_ROLE unset (day-session read, not the routine).
- F1 (his words verified by script): transcript user messages 4c185bad (22:44:50.462Z, 2,601 B, sha256 f654912d...1820)
  and 172046ad (22:59:49.721Z, 510 B, sha256 9533874d...06e6) equal the two saved files' bodies byte for byte.
  The second allows reading two files on the netcode branch by git show, read only; read at e8e276f0 (exists here).
- F2 (decisions of 21/22 Nov): main holds none (git grep of main for 2026-11-21/22, "21/22 November": 0 files, spikes
  excluded). Main's only trace: DECISIONS:298 (2026-11-23, 21:24:59) "three items that moved from the netcode proof
  test into change-set 3". The decisions stand at e8e276f0:.checkpoints/netcode-proof-test/history.md:7561-7607:
  NPT-TRIM (2026-11-21, 20:01:05.731Z): "-34 and -35 move to the real protocol and its dependency checks", under "MOVE
  TO THE REAL GAME (nothing dropped or loosened: ... measured on the real game at the first build that can show it)";
  "The judge reports a moved mark as MOVED, never as PASS". His re-plan answers (2026-11-22, 12:54:49.121Z): seven
  points, none on PM-0001-34, OQ-0001-6 or L1 directly. moved-block.md:24-34 ("FOR CHANGE-SET 3 ITSELF", read only that
  part plus a grep for headings): the same three items as his 22:44:50 message. The three items can be built from his
  22:44:50 message alone; the branch's text agrees with it and adds nothing for 3b.
- F3 (PM-0001-35): ADR 0001:1627 = a WebTransport crate swap diff, sim depends on no transport crate, lockfile
  quinn-proto/rustls/ring/tokio minimums, wtransport feature off; "once, after the swap". Nothing of it can be shown in
  3b (no transport crate; the bite needs quinn = a new package, refused in 3b). Slice 3's whole; 3b's Transport trait is
  only the seam. The report must not count -35 as started.
- F4 (item b, places on main 28254a73; build plan unchanged since f5030a3: 0 diff lines): (1) :389-391 "...because the
  netcode proof test still chooses between them (Part 6) and the slice's mark runs under each."; (2) Part 6 :904
  "Built now, behind a seam, because the proof test may still change it." + table header :907 "What the proof test may
  change" + row :909 "How commands are timed at the server (OQ-0001-6): ... | undecided; an input buffer of 1 to 3
  ticks, as data | one command scheduler in `room`; ... (change-set 7) | one module |". A THIRD sentence says the same:
  risk 8, :1257-1258 "The netcode proof test may change the seams of Part 6. ... the largest is how commands are
  timed." Not named by him (Medium if left stale). Also `crates/protocol/src/messages.rs:5-6` (code comment): "What may
  still move it: the netcode proof test's results (OQ-0001-6, ...)": step 1 edits that file anyway (Low).
- F5 (item b, dependence): nothing in 3b depends on the choice: one slot, lockstep loopback, the command for T reaches
  the room before T (plan 5 step 4); late commands only under L1 in tests, run and counted (netcode note :168-169).
  The scheduler seam with both ways is change-set 7's (:389-391). Report should say 3b's queue picks neither (Low).
- F6 (item a, plan today): plan :167 "each decoder takes 100,000 random and 100,000 mutated inputs without a panic";
  netcode note :380-389 (T9): per decoder (control, input, snapshot, recording), xorshift64* fixed seeds, "About 1 s in
  debug"; :401-403 a counting allocator "not proposed" (unsafe_code deny). Row ADR 0001:1626: "10 million random and
  mutated inputs per message type: 0 panics and 0 allocations above the size limits", tool "cargo-fuzz, a counting
  allocator", "1 run per type. Bites: an unchecked index in one decoder is found within the run". Types 3b gives a
  decoder: Join, JoinAccepted, JoinRejected (new type), Input, Snapshot = 5 (messages.rs on main has the other four
  as plain types, no codec). The recording is not a message type: its plan count stays, outside the mark.
- F7 (item a, counting allocator): crates/sim/tests/state_hash_alloc.rs (3a, merged): own test binary, #![allow(unsafe_code)]
  (the workspace denies it, Cargo.toml:21), counts alloc/alloc_zeroed/realloc per thread, has a control. It counts
  calls, not sizes: "above the size limits" needs the largest request per decode too. One global allocator per
  program, so PM-0001-34's runs go in a protocol integration test of their own (not codec.rs's unit tests).
- F8 (item a, seeds): rng.rs is the simulation's ChaCha8 (ADR 0004 part 8 binds the sim's draws); protocol has no
  dependency on sim; ChaCha8 in debug is several times xorshift's cost. A test-only seeded generator (xorshift64* as
  T9, or SplitMix64 as L1 :318-321), kept apart from the sim's streams, seed and index printed on a failure: fits.
- F9 (CI, measured on main 28254a73, toolchain run 45503161333, job 138831415745, log read): job 2 min 7 s of its 15
  (toolchain.yml:35); Rust debug tests ~0.3 s each run, compile 6.4 s; release protocol+sim 4.5 s build; browser run
  "14 passed (54.8s)" of --global-timeout=300000 (:196); headless Chrome posts ~15 a second ("30 frames and 30 posts in
  2022 ms"). CI runs the workspace twice in debug (:155, :156) and `-p sim -p protocol` in release (:159).
- F10 (item a, cost; ESTIMATES, nothing built): 5 types x 20 M (10 M random + 10 M mutated: meets the row however it
  is read) = 100 M decodes: release ~20-60 s on the runner; debug ~10-20x slower = several minutes per run, x2 runs:
  could break the 15-minute job. Route with NO CI edit: the full count in protocol's release run (CI :159 already
  has -p protocol; release keeps overflow checks, Cargo.toml [profile.release]); the debug runs keep the plan's
  100,000 + 100,000 per type (cfg!(debug_assertions)). Lines: ~+110-130 (allocator with sizes ~60, per-type split
  and limits ~35, catch_unwind + seed/index report ~15, control ~10); the plants go in plants.py (not counted).
  Caveat: release may optimise an unused allocation away, so the allocation half must also hold in debug.
- F11 (CI hold-up, his point 2): none today. Could: (1) the browser run (54.8 s + loop 1-min match + presses: 10,000
  presses at most one per button per tick = 56-83 s of ticks + drain) ~3.1-3.7 min of 5: measure; (2) the 15-min job
  if the full decoder count ran in debug: avoided by F10's route; (3) L1's T6 if >30 s in debug (netcode :356-357
  "one CI word"): stays in debug (headroom) until the CI PR. The per-test 90 s (playwright.config.ts:26) is not CI's.
- F12 (stale plan lines, CI by this chat): plan s0 :40-45 (E1 and the limit PR in lane 1's order before 3b), s6
  :190-198 ("tried here first"), s7 :211-213 (the limit PR "right before 3b"); "#62" throughout = #64 now. The
  release runs of room, client-endpoint, sim-wasm, devtools stay local until the CI PR (as 3a's C2); netcode T7's
  "three processes" = CI's two debug runs + the local release run.
- F13 (section 5 vs main): holds. 3a's scan reads crates/sim/src only (rules_scan.rs:6-9): step 2's world.rs is integer
  code, fine. The test-only module: every non-production build compiles devtools for wasm32 into .gate/rules-module/
  (build.mjs:11-14, :119-133), so steps 7-8's new devtools modules must build there (3A-RULES-STEP's warning). Feature
  compare (build.mjs:109-111) and overflow refusal: only our path crates join, fine. Step 5: 3b's new Wasm calls go
  through host.ts's #call (:133); messages.ts still has `sequence` (:18), renamed by 3b. sim-wasm on main: new(.., now_ms)
  (:110), input(sequence) (:127), newest_input (:133), step() (:138): as 5.1 expects. picture.ts:24: as 5's sentence.
- F14 (gaps): section 5 has no manifest-batch step, though 3b adds path edges (room->sim,protocol; client-endpoint->
  sim,protocol; sim-wasm->room,protocol; devtools->protocol,room,client-endpoint): the hold needs one batch, answered
  first. The card files 3A-HOOK-TEST and 3A-FROM-F64 under "Owed with 3b's docs pass": both are code. HOOK-TEST gets
  real once sim-wasm gains a panicking test (netcode :246-247: JoinRejected in local play panics). FROM-F64: rows
  appended after the 10,048 keep the 16 pinned mul_add rows (10024-10039) in place.
- F15 (ladder, size): +~120 (item a) +~30 (follow-ups; the manifests are already in the netcode estimate, Block 4)
  = ~3,950. Stop line 7,600 and trigger 4,900 stay (raising them would loosen his SIZE-3 (b) yes). Steps 1-6 planned
  2,930 (+150) = ~3,080; at cs2's 1.54 = ~4,740, at 3a's 1.71 = ~5,270: the ladder likely fires before steps 7-8, so L1
  and the first recording likely go to 3c. The plan does not say how many rungs fire: my reading below (C4).
- F16 (card): enough to resume once it adds this read's verdict and conditions, his 22:59:49 message and the branch
  pointer, the manifest batch as a step, HOOK-TEST and FROM-F64 as build steps, CI's limits and today's times, the
  ladder reading. Compact: yes (70 % used; 8 steps and two review rounds ahead).

## Answer (sent to the caller)
VERDICT: APPROVE, conditions C1-C8:
- C1 [High] item (a)'s bar exactly: 5 types; one release run of 10 M random + 10 M mutated seeded inputs per type;
  0 panics; 0 allocations above the codec's caps; the planted unchecked index found (its index reported); a control
  (a known-size allocation seen) and one planted over-allocation caught, in debug too; "started", never "passed";
  the recording keeps the plan's count outside the mark.
- C2 [Medium] its own protocol integration-test binary (allocator after state_hash_alloc.rs, with sizes); full count
  under cfg(not debug_assertions), run by CI's existing release line; debug at 100,000 + 100,000; measured first.
- C3 [Medium] item (b): three old/new pairs (:389-391, :909 moved to :919's list, risk 8 :1257-1258); ADR 0001
  :252/:1156/:1699-1700 untouched (a follow-up, dated pointer); messages.rs:5-6 for OQ-0001-6 only; queue picks neither.
- C4 [Medium] ladder: count after each step; before steps 7 and 8 project; past 4,900, unbuilt parts move in order
  until the projection is under 4,900; tell him first; L1 before anyone builds it (item c).
- C5 [Medium] manifest batch first (all edges + 3A-MANIFEST-COMMENT, security-auditor first); 3A-HOOK-TEST in step 4
  before panicking tests; 3A-FROM-F64 its own small step (append, --check, sha256s, verifier regenerates).
- C6 [Medium] time the browser run and the job on a clean copy before the first evidence push; past ~4 of 5 minutes:
  three lines to him first; a spec over 90 s sets its own limit; the 10-minute run outside packages/e2e/tests/.
- C7 [Low] scope words (local release runs; "three processes"; second-to-merge takes the other in; dev build after 7, 8;
  -35 not counted).
- C8 [Low] his two messages into DECISIONS by script; 21/22 Nov as a pointer to e8e276f0, not copied as his on main.
DONE

# C62H-BOUND build brief: netcode-architect notes

Started 2026-11-15. Output: .checkpoints/netcode-proof-test/c62h-bound-brief.md (Write only, lines <= 120 chars).

## Plan
1. Read Mustafa's words (history.md 2113-2175).
2. Read the design (c62h-bound-design-full.json), the advisor's reads (C1-C11, Q1-Q5), the test read (TE-CB-1..10),
   the advisor's second ruling (te_rulings, replacements, planned_size, paste_blocks, other_changes item 6).
3. Read the frozen texts (texts-v3.md, freeze.txt, paste.md) and the freeze script.
4. Read the card: NOW, Key numbers (KEPT-LOGS), C62H-BOUND blocks, MH-L1.
5. Read the code: spikes/netcode-proof/ crates l1, lab, judge; marks/; ops/runs/c-u4r-runs.mjs; check.mjs; runners.
6. Read the earlier briefs for form (u4r-brief.md, brawl1-brief.md).
7. Write the brief in sections 1-8 (+ header, decisions); a notes line after each.
8. sha256sum, line count, line-length check; return.

## Progress
- 1 DONE: history.md 2113-2129 (13:35 rule), 2140-2146 (C62H-BOUND (a)), 2163-2175 (C62H-BOUND-YES (a)).
- 2 DONE: design JSON, advisor C1-C11 (Q5 (b): JDG-ONLY-1 out), TE-CB-1..10, r2 rulings (size ~650, 545-755; stop 1,050;
  other_changes[5] = build items list; runner sha 99dd070e).
- 3 DONE: freeze.txt sha256 9cb8be17...; paste cfac7502...; v3 3f21c9b7...; block hashes (check-objects cea24dd8,
  bite-object d5945f92, rows 57279093 / f9822480 / cc1b3c74 / 8086ff4d, readme-sentence 02bd0368, plain-words 4bf0fcbf).
  Live file shas now: marks.json 9fb5be57..., SERIES.md 3411eee7..., README.md b49f3441..., c-u4r-runs.mjs 99dd070e...,
  u4r-run.sh e9437ae2..., main-run.sh 8df2dee3...
- 4 DONE: card NOW (head 442a284 after first pack; free 5,900 MiB at 21:5x), Key numbers, MH-L1.
  P0 folders: 051995db9cea = U4r's P0 (-2 FAIL 0.861889, -17 INVALID, tick16 invalid; "recorded, never counted"
  advisor-u4r-sets-r2-full.md:10); 93509dfd7fd1 = BRAWL-1 re-run P0 ("P0 STANDS AND COUNTS",
  advisor-brawl1-sets-p0-full.md; design: "93509dfd7fd1's verdicts stand"). Same judge sha 65acacfe, same marks 9fb5be57.
  Decision: the kept P0 set = 93509dfd7fd1; 051995db9cea stays unpacked until Mustafa has the report line (he may name it).
- 5 DONE (code facts, HEAD 0457c85; spikes/ unchanged since 914798a + b5cb253's results list):
  - FINDING (build order): lab contract test r2 (crates/lab/src/contract.rs:448-513, its (b) names check) fails on any
    written name with no SERIES.md row -> commit 1 must carry SERIES.md's tick.s.snap_link_lost_count row + the (18)->(19)
    header, else check.mjs fails after step 1. Regroup, no frozen text changes. Open question for the advisor.
  - check.mjs marks step: npt-judge --check-marks needs figures known (marks.rs:544) -> marks.json after the judge commit.
  - LAB_BITES[1] read positionally (precheck.rs:312) -> new id is LAB_BITES[2], appended.
  - biteHomeProblems (check.mjs:936): id literal in one crate's src only (lab); tests/ and cfg(test) mods not read.
  - l1 datagram() (lib.rs:1084-1117): link not up -> returns Sent with no fate entry; too large -> no entry. Dir private
    (lib.rs:235-240); Down = server to client. with_fate_log at lib.rs:1885. Lost/Burst decided at push (1097-1104).
  - scenario.rs: take_datagrams :968, server.send :989, room_tick call :1021-1024, summary_line :1612-1690,
    Server::send :557-584 (no session -> Ok, nothing sent), Bites :180-200, LAB_BITES :204, Bites::of :234-245.
  - logs.rs: logged map :160-161, set in room_tick :538-541 (only when server hash Some), removed at :420 in
    write_client_tick before the counts; incomplete_measured :446; room_tick :495-...; whole_as_lost bite :317-331.
  - figures.rs: slot_hashes :236-279, decoded :316-..., measured filter pattern t.ph == Phase::Measured (:28, :503),
    FIGURES :632-826, known :827. engine.rs hash_mark :1297, hash_log :1309, u4j gaps test :1346. eval.rs gaps
    :1215-1231, bite status :1484-1496; RunOut.checks pub (eval.rs:1083-1095) -> fixtures can assert Outcome::Gap.
  - runner: SPLIT_LINE :166, overflowFlag :289-292, bitesOf :310-318, planJudged :324-332, header comment :34 ("only P0"),
    judge call :566, split :581-585, marks sha line :475; builds go to .gate/netcode-proof-test/target (776 MiB) via
    spikes/netcode-proof/.cargo/config.toml. No test file imports bitesOf/planJudged (scratch harness, as U4r-F2/F3).
  - det-u4.mjs + precheck regeneration at the merge step (build ruling 1: every merge touching the scenario path;
    BRAWL-1's m3.log form).
  - Disk: free 5,899 MiB; shared target 6,043 MiB. Packable non-P0 U4r folders (closed: advisor-u4r-sets-r2-full.md:45,
    closed at 6b22cab): u4r-P2-ab10f058a828 779, u4r-B1-64f3307d5c7f 821, u4r-P3-0f066338dd8d 519 MiB.
- 6 DONE: brawl1-brief.md form (sections: who/binds, start, limits, work in order, hand over, stop rules, report).
  Lesson: worktree isolation bases on main -> main session makes .claude/worktrees/<name> by hand; builder launched
  with no isolation; u4r-run.sh needs worktree under .claude/worktrees/ and logs under .gate/u4r/.
- 7 next: write the brief.

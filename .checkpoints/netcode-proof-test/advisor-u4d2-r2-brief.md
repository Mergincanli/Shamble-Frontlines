# The advisor's ruling on U4d2's round 2: the brief (the main session, 2026-11-13 about 08:50 UTC)

**Your role:** read `cd /home/user/Shamble-Frontlines && git show origin/main:.claude/agents/advisor.md` first and work
as it says. You rule on round 2 of U4d2 (on what changed: the fix pass FX-1 to FX-12 that your round-1 ruling set) under
CLAUDE.md's review budget and your round-1 ruling's Block 8 ("Round 2's outcome"). Write your ruling to
`.checkpoints/netcode-proof-test/advisor-u4d2-r2.md`, one block at a time as you finish each, so that a cut run leaves
every finished block on disk. Write files with the Write and Edit tools only (no sed, no shell redirect, no python
writes; never `cargo fmt` writing files: apply rustfmt's diff by Edit); an agent with no Edit tool rewrites its notes
file whole with Write. Run nothing that builds or touches a copy; reading, git and sha256 are fine.

**Evidence (read whole; all paths under /home/user/Shamble-Frontlines):**
- Your round-1 ruling: `.checkpoints/netcode-proof-test/advisor-u4d2-r1.md` (Blocks 1 to 9, the texts T1 to T5, "Before
  the merge").
- The fix pass: the builder's record `.checkpoints/netcode-proof-test/u4d2-build.md`, its section "Fix pass, round 1"
  (its claims, not evidence); its runs `.gate/u4d2/fx-*.log` (fx-check1.log and fx-check2.log: check.mjs counted, all 11
  steps PASS, measured 417, bites 466, self 71 of 71); its end hashes `.gate/u4d2/fx-hashes-end.sha256` (18 files). The
  re-cut snapshot `.checkpoints/netcode-proof-test/wip/u4d2.patch` (cb27931; 18 files on 51520b4; sha256 9e71cda0…;
  `git apply --check` exits 0 on 51520b4 and on the spike head, whose spikes/ is 1b4a3c8's). Copies: the builder's
  `.claude/worktrees/u4d2-build`; the read copy `.gate/u4d2-r2` (51520b4 plus that patch, 18 of 18 OK); round 1's tree
  `.gate/u4d2-r1`.
- The packet every round-2 seat started from: `.checkpoints/netcode-proof-test/u4d2-r2-packet.md` and its side file
  `u4d2-r2-packet-tests.md` (cut by review_packet.py from u4d2-r2-packet.json).
- The five seats' notes, each APPROVE WITH FINDINGS:
  - security-auditor `u4d2-r2-review-security.md`: SEC-U4D2-R2-1 to -4, all Low, none before the merge;
  - rust-reviewer `u4d2-r2-review-rust.md`: RR2-L1 to -L4, all Low;
  - infra-engineer `u4d2-r2-review-infra.md`: INF2-1 Medium, not before the merge; INF2-2, -4, -5 Low; INF2-3 Low,
    marked before the merge as wording in M8's T5 fill;
  - test-engineer `u4d2-r2-review-test.md`: TE2-R2-1 to -3, all Low, each another seat's Low settled by run (INF2-2,
    RR2-L3, SEC-U4D2-R2-3); its counted run (r2-te-check.log, in the background): all 11 steps PASS, measured 417, bites
    466, self 71 of 71, the wasm step 34.8 s; end state 18 of 18 OK, crates/sim untouched;
  - engine-architect `u4d2-r2-review-arch.md`: EA2-1 to -4, all Low.
  The seats' runs: `.gate/u4d2/r2-*`; the security seat's probe scripts: `.checkpoints/netcode-proof-test/sec-r2-*`.
- Two process events, both on the card (`.checkpoints/netcode-proof-test.md`, "Cuts by the spend limit"): the container
  restarted at about 00:52 UTC (the rust seat had returned; the run resumed from its journal at 00:55); the org's spend
  limit cut batch 2 at 01:22 UTC with the test seat's TP10 plant in place (two lines in check.mjs's hashWasmProblems).
  The main session removed it by Edit at about 08:26 UTC (check.mjs back to its r2-hashes-start sha256, 18 of 18 OK,
  `diff -rq` against `.gate/u4d2-r2` equal), recorded in a labelled block at the end of u4d2-r2-review-test.md; the
  resumed seat re-planted TP10 in its own form (r2-te-tp10c-scan-regression.log).

**What I ask you to rule on:**
1. **INF2-1 (Medium, not before the merge):** the 300 s limit per call does not bound the step (up to 12 calls times
   300 s under the exclusive lock if the replay hangs), and a stopped call leaves its leg children running, unseen by
   the runner's wait. The seat says FX-7 meets your ruled form. Its route under your Block 8 (a follow-up or a carry,
   never a third round), with the words wherever a carry is to be written.
2. **The Lows:** SEC-U4D2-R2-1 (two comments claim more for the section read than it gives: a RUSTC_WRAPPER's link
   arguments rewrite it, built; a `linker` key by reasoning), -R2-2 (a cargo config's per-package profile turns npt-sim's
   overflow checks off unseen), -R2-3 with TE2-R2-3 (`m!(include!)` and a meta-built `#[path]` get past FX-6, built
   and settled live), -R2-4 (wording in T3 and T5); RR2-L1 (a custom linker unread), -L2 (an empty CARGO_HOME read
   differently), -L3 with TE2-R2-2 (EA-9's compare default unpinned, settled by run), -L4 (wording); INF2-2 with
   TE2-R2-1 (no test pins run()'s timeout wiring, settled by run), INF2-4 (the lane lock file lies inside the folder
   M9's between-waves clean deletes), INF2-5; EA2-1 (two check.mjs exports, each imported by a self-test), EA2-2 (the
   wasm32 feature ban's three homes), EA2-3 (TickHash's struct doc still says "(or run)"), EA2-4 (C-U7-2's "reads each
   export's type from the module's type section": the JS API's export descriptor has only name and kind). Each one's
   route (a carry for U4r or U7, a follow-up, or the report's wording), with words where a carry is to be written. A
   Low never triggers a push.
3. **The texts:** T3 and T5 as round 2 asks: SEC-U4D2-R2-4 (the token `u32` or `i32`; T5's "Every other security finding
   of the review is fixed in this change" names round 2's open security Lows or your deferral of them), EA2-4 (C-U7-2's
   words on how U7 reads the export types), INF2-3 (T5's fill: the timing figures each labelled, round 2's first run
   73.7 s "the first run after a machine restart (page cache cold, no dependency rebuilt)"; the bound's words). Give
   each amended text whole in a code block, so the merge copies it by script.
4. **The process events:** that round 2, cut once by the container's restart and once by the spend limit and resumed
   each time from its journal, counts as one round; and the main session's removal of TP10's plant.
5. **The merge:** confirm Block 9's M1 to M9 for U4d2's merge, with any amendment round 2 needs. The counts I expect at
   M5 on the merged tree, if nothing interacts with U4e's merged code: 490 / 560 / 63 (Block 7, point 5) plus the fix
   pass's deltas (+1, +1, +8), so 491 / 561 / 71 of 71, 11 steps. M9's removals: say whether `.gate/u4d2-r2` joins them.
6. **One leftover outside the repository:** the test seat reports that the rust seat's probe left a folder
   `/nonexistent-cargo-home` at the filesystem's root (an empty CARGO_HOME probe, RR2-L2). Mustafa's rule on leftovers
   (LEFTOVER-DELETE, his words at history.md:404) is why the main session has not removed it; say whether anything
   else is needed.

For each item give APPROVE, REVISE or ESCALATE where one fits, the words wherever a text is to be written, and at the
end two lists: what must be done before the merge, and what goes to Mustafa (if anything), with the reason.

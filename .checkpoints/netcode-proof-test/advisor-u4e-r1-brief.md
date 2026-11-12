# The advisor's ruling on U4e's round 1: the brief (the main session, 2026-11-12 about 07:45 UTC; item 6 filled when the test seat returns)

**Your role:** read `cd /home/user/Shamble-Frontlines && git show origin/main:.claude/agents/advisor.md` first and work
as it says. You rule on round 1 of U4e under CLAUDE.md's review budget: all five seats' findings, each one's route,
the fix pass's scope, round 2's seats, and the merge's conditions. Write your ruling to
`.checkpoints/netcode-proof-test/advisor-u4e-r1.md`, one block at a time as you finish each, so that a cut run leaves
every finished block on disk. Write files with the Write and Edit tools only (no sed, no shell redirect, no python
writes; never `cargo fmt` writing files: apply rustfmt's diff by Edit); an agent with no Edit tool rewrites its notes
file whole with Write. Run nothing that builds or touches a copy; reading, git and sha256 are fine.

**What U4e is:** the room and the codec path (u4-build-plan.md §5 U4e): npt-room, npt-server's library (the pump) and
npt-client's endpoint, built from the brief you approved (u4e-brief.md rev 3.3; your check: advisor-u4e-brief.md, L1
to L4) on `<base>` 28a85e7 (L3's carry-51 commit, `Scene::effect_rows`). One builder built it in
`.claude/worktrees/wf_0fa6595a-3e2-1` (branch worktree-wf_0fa6595a-3e2-1 at 97bb9d3, whose spikes/ equals 28a85e7's;
uncommitted). Its snapshot on the spike branch: `.checkpoints/netcode-proof-test/wip/u4e.patch` (db49108; 23 files;
`git apply --check` clean on the main folder). A byte-identical read copy: `.gate/u4e-r1` (97bb9d3 plus the patch).

**Evidence (read whole; all paths under /home/user/Shamble-Frontlines):**
- The packet every seat started from: `.checkpoints/netcode-proof-test/u4e-packet.md` and its side file
  `u4e-packet-tests.md` (cut by review_packet.py from u4e-packet.json; it holds no conclusion of the builder or the
  main session; base copy `.gate/u4e-prefix`, a worktree at 1e8c188, as your L3 says).
- The five seats' notes, each with its verdict:
  - netcode-architect `u4e-review-netcode.md`: REVISE (NC-H1 High; NC-M2 to NC-M6 Medium; NC-L1 to NC-L4 and
    NC-L5-L12 Low);
  - security-auditor `u4e-review-security.md`: APPROVE WITH FINDINGS (SEC-U4E-4 Medium; SEC-U4E-1 to -3 and -5 to -9
    Low);
  - rust-reviewer `u4e-review-rust.md`: APPROVE WITH FINDINGS (RR-5 Medium; RR-1 to RR-4 and RR-6 to RR-10 Low; its
    bench .gate/u4e/rr-bench, logs r1-rr-bench.log and r1-rr-callgrind.log);
  - engine-architect `u4e-review-arch.md`: REVISE (A1 to A6 Medium; A7 to A14 Low);
  - test-engineer `u4e-review-test.md`: see item 6.
- The builder's record (its claims, not evidence): `.checkpoints/netcode-proof-test/u4e-build.md`. Its run logs:
  `.gate/u4e/` (b11-counted1.log and b11-counted2.log: check.mjs all 10 steps PASS, measured 458, bites 524, self 52,
  on `<base>`'s 399 / 446 / 52; b11-list.log; s01 to s15, its scratch switches). The seats' runs: `.gate/u4e/r1-*`.
- The copy itself: `cd .claude/worktrees/wf_0fa6595a-3e2-1 && git status --untracked-files=all` and `git diff`.
- One process event: the org's spend limit cut the test seat at about 06:42 UTC with its plant P-D in place
  (client/src/lib.rs:670-674). The main session removed it by Edit at 07:35 UTC (sha256 = start; the copy's crates/
  equal `.gate/u4e-r1`'s; the re-cut patch equals wip/u4e.patch); the labelled block at the end of
  u4e-review-test.md records it. The seat was resumed from its notes at about 07:36 UTC.

**Mustafa's words that bind it** (verbatim; history.md:340, 2026-11-11): "The Low about the effect row's owner: defer
it to U4e in writing. A Low never triggers a push." The review budget is CLAUDE.md's: a round is one review of one
head by all the change-set's reviewers, then one fix pass; at most two rounds, the second on what changed; a third
only for an open Critical or High; below that bar a finding becomes a follow-up, and a Low never triggers a push on
its own.

**What I ask you to rule on:**
1. **NC-H1 (High):** the tick16 bites' unit proof against §4.14's named check, and the client's drop of a header tick
   at least the ring's length behind its newest. The netcode-architect's lean (such a tick read as §4.6's "bad
   header", the sentinel filed under the tick read, "decode" kept for stale groups inside the ring) touches carry 61
   (d) and your U4j round-2 ruling B2.1. Rule the reading, the fix's form, and what the unit proof must assert (what
   the judge counts), with the test seat's settling runs beside it (item 6).
2. **The Mediums marked before the merge:** NC-M2 (carry 68 not met: the ES own-state figure without the yaw and
   pitch shares), NC-M3 (the Pong's RTT sample from the pump's now, not the arrival), A1 (control messages handed out
   typed: each FullState encoded twice, the bites asymmetry, plan.md:508-509's "Encoding stays on the tick"), A2
   (rtt_ns chosen by what the link reports, not by the session's kind: §4.9), A4 (`tick_input()` holding two ticks'
   state), A5 (the pub items beyond §4.1 and plan.md §4.7, nine with no user; §4.1's rows in the merge commit). For
   each: in the fix pass, with the fix's form, or another route.
3. **The Mediums marked not before the merge:** SEC-U4E-4 (Resync, ack, Resync: full states outside the window),
   NC-M4 (events packed before the nearest 8 enemies), NC-M5 with A3 (no event base on the wire for a late join or a
   rejoin), NC-M6 (the endpoint's default lead of 1 on a delaying link), A6 (ending a session: no library path), RR-5
   (the room's tick at 48 clients, 5.6 ms mean and 2,838 allocations per tick, against PM-0001-4's budget): each
   one's route, with the words wherever a carry is to be written.
4. **The Lows marked before the merge:** NC-L1 (your condition 6 in §5 U4e: the report states B, F, the ages and that
   B's slot still holds B), NC-L3 (e7's "at least one resent" carried by gap.rs's end-to-end test), NC-L4 (B2.1's
   answer "decode": B5's extra reading not needed; stale() has no SERIES.md row), A7 (carry 26's copy, raised now),
   A14 (carry 54's SERIES.md rows: the names proposed, the main session's commit before U4r writes them). Rule each.
5. **The other Lows** (SEC-U4E-1 to -3 and -5 to -9; RR-1 to RR-4 and RR-6 to RR-10; NC-L2 and NC-L5-L12; A8 to A13):
   each one's route (a carry for U4f, U4g, U4h, U4r or U6, a follow-up, or the report's wording), with words where a
   carry is to be written. SEC-U4E-6, A13 and NC-L11 are one point (the shooter's slot sent to a client that sees
   only the victim, against ADR 0001 part 9 (f)).
6. **The test-engineer's findings** (`u4e-review-test.md`: REVISE; resumed after the cut, it redid its two unrecorded
   plants in its own form): TE-1 High (it settles NC-H1 by run, probe Q-1: under either tick16 bite the client files
   nothing after the wrap, and the part (i) twin's assertion is met by a silent drop); TE-2 to TE-6 Medium, all marked
   before the merge, each a plan-listed test or ruled check that a plant shows cannot fail: TE-2 (e6: the window's
   remainder never binds), TE-3 (client-rounding's call site in the endpoint has no test; carry 24's first clause),
   TE-4 (each group decoding alone: group 1 without group 0 not shown), TE-5 (the pump's size check has no test;
   U4E-Q11 (a)), TE-6 (carry 57's second-overflow kick: its test cannot fail); TE-7 to TE-10 Low (carry 61 (b)'s
   two-hash sentinel untested; carry 50's room call unpinned, with RR-2; carry 21's sort cannot bite under option B;
   the 1,150 B case refused by its trailing bytes). Its settling runs: NC-H1, NC-M3, NC-M5, SEC-U4E-1, SEC-U4E-3,
   SEC-U4E-4 (29 full states in 30 ticks, 256,418 B) and SEC-U4E-7 (an add overflow at client/src/lib.rs:526, from
   server bytes) hold; NC-M4 holds in part (Q-11). Its counted check.mjs run (.gate/u4e/r1-te-check-counted.log): all
   10 steps PASS, measured 458, bites 524, self 52, equal to the builder's two runs; its provenance block is empty,
   because check.mjs keeps cargo's Compiling lines out of its output (the runner's pwd line names the copy). Rule
   each, with the High under item 1.
7. **Points of the main session's, not in the packet** (the card, `.checkpoints/netcode-proof-test.md`, "Running").
   Confirm or correct each:
   - Carry 68 is not in the builder's list of carries met, though it reports carry 56's ES figure; NC-M2 found the
     same.
   - Carry 61's stale-group choice: decode within the ring, drop and count beyond it; NC-L4 reads it as "decode".
   - The builder's own points for you: e7's split and resend clauses (NC-L3); -18's baseline in memory (NC-L1); the
     owner-to-all reading (each client gets the next slot's records); carry 26 (A7); its proposed series
     `event.snap_refused.tick_count` (carry 54) and `tick.s.snap_too_old_count` (carry 61); Kick(replaced) left to the
     caller (SEC-U4E-5, A6).
   - Whether the test seat, resumed after the cut, counts with the other four as one round.
8. **The fix pass:** its scope (the findings it takes), who builds it (one builder in the same worktree, reading its
   notes and the diff first, as U4d-F's fix pass did), and round 2's seats on what changed.
9. **The merge's conditions after round 2,** as M1 to M9 were for U4d-F (the snapshot, the apply on the spike head,
   the gate, check.mjs counted in the main folder with the expected counts, the record's place, the plan and carries
   text the merge commit carries).

For each item give APPROVE, REVISE or ESCALATE where one fits, the words wherever a text is to be written, and at the
end two lists: what must be done before the merge, and what goes to Mustafa (if anything), with the reason.

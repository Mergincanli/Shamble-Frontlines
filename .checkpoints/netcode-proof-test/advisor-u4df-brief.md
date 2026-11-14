# The advisor's ruling on U4d-F's round 1: the brief (the main session, 2026-11-11 about 23:58 UTC)

**Your role:** read `cd /home/user/Shamble-Frontlines && git show origin/main:.claude/agents/advisor.md` first and work
as it says. You rule on round 1 of U4d-F, under its own review budget (your U4d close ruling, advisor-u4d-close.md,
"Its own review budget"): both reviewers' findings, their routes, and the merge's conditions. Write your ruling to
`.checkpoints/netcode-proof-test/advisor-u4df.md`, one block at a time as you finish each, so that a cut run leaves
every finished block on disk. Write files with the Write tool only (no shell or python writes). Run nothing that
builds or touches a copy; reading, git and sha256 are fine.

**What U4d-F is:** the estimate scene fix you approved (advisor-u4d-close.md B1.1 to B1.3: steps 0-7, the files and
symbols allowed, step 4's measurement as the acceptance). Four builders built it in turn in
`.claude/worktrees/wf_9ce88fa3-452-1` (branch u4df-build at dad9566; uncommitted). Its snapshot on the spike branch is
9dd037a (`.checkpoints/netcode-proof-test/wip/u4df.patch`, 5 files under crates/sim; `git apply --check` is clean on
`.gate/u4df-prefix`, a worktree at dad9566).

**Evidence (read whole; all paths under /home/user/Shamble-Frontlines):**
- The packet both reviewers started from: `.checkpoints/netcode-proof-test/u4df-packet.md` (cut by
  review_packet.py from u4df-packet.json; it holds no conclusion of the builder or the main session).
- The netcode-architect: `.checkpoints/netcode-proof-test/u4df-review-netcode.md` (APPROVE WITH FINDINGS: NF-M1
  Medium; NF-L1 to NF-L4 Low).
- The test-engineer: `.checkpoints/netcode-proof-test/u4df-review-test.md` (APPROVE WITH FINDINGS: TF-M1 Medium; TF-L1
  to TF-L3 Low).
- The builder's record (its claims): `.checkpoints/netcode-proof-test/u4df-build.md`. Run logs and plant diffs:
  `.gate/u4df/` (the builder's s0- to s6- and end-, the netcode-architect's nc-, the test-engineer's te-).
- check.mjs, counted, on the final files: the builder's end-check4.log and end-check5.log and the test-engineer's
  te-check.log, all 10 steps PASS; measured 367, bites 414, self 52 (+2 / +2 / +0 on the base's 365 / 412 / 52).
- The copy itself: `cd .claude/worktrees/wf_9ce88fa3-452-1 && git diff` against dad9566.

**Mustafa's words that bind it** (verbatim; history.md, "Decisions taken", 2026-11-11 about 11:50 UTC): "The
event-rate drift in the estimate scene: fix the scene before U4d2 pins the recordings. The 9.5 floor (PM-0001-2) does
not move." The review budget is CLAUDE.md's: a round is one review of one head by all the change-set's reviewers, then
one fix pass; at most two rounds, the second on what changed; a third only for an open Critical or High; a Low never
triggers a push on its own.

**What I ask you to rule on:**
1. **NF-M1, with TF-L2 beside it:** the value 40, the report's wording, and how any later re-measure judges drift.
   Say whether any of it changes the acceptance in a way that needs Mustafa's word (his floor stays where it is).
2. **TF-M1:** the deterministic test of the reckoning the test-engineer proposes. It lies outside B1.2's symbols, so it
   comes back to you. Say whether it is approved, its form and place, and when it lands: before the merge (the fix
   pass, then round 2 on what changed), or later (whose, and whether before U4d2 pins anything).
3. **The seven Lows** (NF-L1 to NF-L4, TF-L1 to TF-L3): each one's route (a carry, a follow-up, or the report's
   wording), with the words wherever a carry or a doc line is to be written.
4. **Two readings of mine, which are not in the packet** (the card, `.checkpoints/netcode-proof-test.md`, "Running").
   Confirm or correct each.
   - TE2-5: t_m1 failed in steps 2 to 5 by the plan's own design (B1.3: "both move the event rate"; steps 5 and 6 fix
     it). It failed the same way each time, was named, was re-run at once, and was recorded tagged TE2-5. The build
     brief's clause ("a failure in npt-sim ... stops your step group") was stricter than B2.3's merge stop. I did not
     stop the build.
   - The rust-reviewer: not triggered. Step 1 moved run_command's lines into `move_velocity`, and the bite's
     `speed *= 1.01` became `gain = 1.01` in the same product. The netcode-architect's [11] reads it the same way.
5. **The merge into the spike branch:** its conditions, as you set M1 to M9 for U4j (the snapshot, the apply on the
   spike head, the gate, check.mjs counted in the main folder with the expected counts, the record's place). U4e's L3
   (the carry-51 commit to scene.rs) waits for this merge.
6. **Process:** each reviewer first wrote part of its notes with a python one-off (a shell write), then rewrote the
   file whole with Write; each says so at the top of its notes. Say whether the next workflow needs a rule beyond the
   one its brief already gave.

For each item give APPROVE, REVISE or ESCALATE where one fits, the words wherever a text is to be written, and at the
end two lists: what must be done before the merge, and what goes to Mustafa (if anything), with the reason.

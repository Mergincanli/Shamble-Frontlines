# For the advisor agent: the short re-read of 4b-2 (steps 6 to 12), against the main that holds 4b-1

Your verdict of re-read 2 (`advisor-reread.md`): "Steps 6-12 (4b-2): HOLD until FEATURE-COMPARE's answer and, if
two PRs, the re-read against the main that holds 4b-1; fix B1-B7 first." All three have come: his answer
(`docs/DECISIONS.md`, 2026-11-26, 14:09:13 UTC: FEATURE-COMPARE (a) with four conditions; SIZE-4B (b)); B1 to B7
are in the sheet's revision 3 (`reread-sheet.md`, section H says where); and 4b-1 is merged (pull request #75,
merge commit b9aad01 on main). The worktree stands on branch `cs/4b-world` from that main, with the notes back.

## What is new since revision 3, for steps 6 to 12
1. **Step 6's batch is made again on today's main.** The batch of step 0 (`deps-manifests.diff`,
   `deps-batch.sha256`) was cut before 4b-1's own path-only batch: its content and devtools manifests are older
   than what 4b-1 merged. The new batch is root `Cargo.toml` (the `parry3d` line, `std` off, the overflow
   blocks), `crates/sim/Cargo.toml` (`content`, `parry3d`), `crates/sim-wasm/Cargo.toml` (`content`) and
   `Cargo.lock` by `cargo update --workspace`; it goes to security-auditor by its digests before it is
   committed, pushed or built (his DEP-HOLD), and rust-reviewer's paper reading (33 registry packages) is held
   against it. security-auditor's note of step 1 (devtools' `protocol` edge could be a dev-dependency) rides
   with this batch.
2. **FEATURE-COMPARE (a), his four conditions:** two named exceptions, each pinned to exact entries and
   versions, with tests and planted mistakes, any other difference still stopping every build; a change of
   parry3d's version or of the two packages' versions needs a new yes from him; the library's `std` stays off;
   the built-file check keeps looking for SIMD and fused multiply-add in the players' file; the recorded match
   exercises the collision queries from the first change-set that calls them. rust-reviewer's wording for the
   two exceptions is `rust-reviewer-step0.md`.
3. **From 4b-1's reviews, into 4b-2's steps** (STATUS's follow-ups, the archive's block "From change-set 4b-1"):
   step 6: the simulation takes the time unit from the content crate (4B-TIME-REEXPORT), and the build lints of
   steps 6 and 7 cover the content crate too; step 7: `f64::round` (`map.rs`, `time.rs`) enters the players'
   file at step 9, so the libm rule allows it or the code rounds in whole numbers; step 8: which way a facing of
   0 degrees points and how the simulation reads an angle are pinned before the first map file
   (4B-FACING-ZERO; engine-architect's proposal: 0 faces +z, degrees grow toward +x); step 9: the TypeScript
   cap's tie to the Rust cap, the bundle's outer shape for the type writer, a file-level `#![cfg(test)]` in
   the source scan (4B-SCAN-CFG-TEST); the first commit fixes one test comment's unit (4B-MEMORY-UNIT).
4. **His answers of 18:16:35 UTC:** SIGHT-FLAG (a): no "blocks sight" mark in 4b; the collision world of step
   10 keeps, for every solid piece, the shape it came from. DUR-1 (C): nothing for 4b-2.
5. **Your four conditions for 4b-2** are in the card ("4b-2 (the second pull request): set-up").

## What leaves first (your condition 1): the approved sheet's two rungs, unchanged
Stop line 8,450 written lines (his SIZE-4B (b)); two thirds is 5,633 (his one sentence is sent when the count
passes it). Revision 3, section G, and the plan's own words (`cs4-walk/plan.md` lines 167 to 172):
- **Rung 1, at 6,750 written lines:** the schema writer and its tests (step 11) move to 4d, with the schema
  halves of PM-0005-7 and PM-0005-8.
- **Rung 2, at 8,000:** the page's scene (step 12) moves whole, with its browser tests, to the start of 4c;
  4b-2 then changes nothing a player sees and needs no browser check.
- **Never moved:** the built-file check and the static collision world.
- **How a rung fires (the reading I ask you to confirm, since 4b-1's M1 turned on it):** the count is taken at
  the end of every step; if the count at the end of the step before a rung's step has reached the rung's
  number, that step does not start in 4b-2, and he is told before it moves. So step 11 is built only if the
  count after step 10 is under 6,750, and step 12 only if the count after step 11 (or after step 10, when step
  11 has left) is under 8,000.
- **The projection, so that nobody is surprised:** 4b-1 was written at 2.17 times the designs' count. At that
  ratio steps 6 to 10 (about 2,900 by the designs' count, section G) come to about 6,300, step 11 to about 520
  and step 12 to about 1,060: rung 1 is close, rung 2 is not reached, and the whole is about 7,900 to 8,100,
  under the stop line by about 350 to 550. A second fix pass of 4b-1's size (232) fits; a third round would not.

## Reviewers planned for 4b-2
rust-reviewer, engine-architect, security-auditor (the dependency batch first, then the diff), test-engineer;
netcode-architect for steps 9 and 10 (the simulation's constructor, the recording `first-loop` made anew with
its frozen content: 4A-NA-4B, 4A-EA-4B); graphics-engineer and perf-auditor for step 12 if it is built here.
GREY-COLOURS (the scene's greys by LOOP_V2 section 9's default) is his to veto before step 12; the short
browser check is his, on the scene.

## What I ask
Your verdict on steps 6 to 12 as revision 3 has them with the points above: APPROVE (with conditions), REVISE
(what to change in the sheet first), or ESCALATE (what he must decide before step 6). And your reading of how a
rung fires.

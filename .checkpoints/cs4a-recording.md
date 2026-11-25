# Checkpoint: cs4a-recording
- **Goal:** change-set 4a, "The recording": a recorded match replays to the same per-tick hashes natively (one
  process, two processes, two matches interleaved) and in WebAssembly on the file players get (Node and Chromium),
  and a planted divergence is caught at its tick. No browser check: nothing a player sees changes · plan:
  `.checkpoints/cs4-walk/plan.md` (revision 4, approved by the advisor agent; section 4) · design:
  `.checkpoints/cs4-walk/netcode-architect-cs4.md`, block 1 · his go: `.checkpoints/cs4-walk/mustafa-2026-11-25-go-walk.md`
  (2026-11-25, 19:36:25 UTC: start 4a; WALK-CUT (a); STEP-60 (a))
- **Branch:** `claude/cool-pasteur-oiqk1r` on GitHub (the stand-in for `cs/4a-recording`). Local: `cs/4-walk` in the
  worktree `.gate/lane1-takeover`; pushed with `git push origin cs/4-walk:claude/cool-pasteur-oiqk1r`, alone in its
  command; never a plain `git push`. The session's start folder is never switched · **Base:** origin/main
  c1da1bd (lane 2's docs-only #67, taken in by merge commit 75a9296; before it 311c0484)
- **Last update:** 2026-11-25 20:40 Amsterdam (from the clock)

## Sizes (his point 5: which is which)
- The designs' own count for 4a, bottom-up: 1,365 lines (655 code, 710 tests).
- Planned, which already holds the 1.8: 2,450. Stop line (planned times 1.25): 3,050.
- Ladder: at 2,450 written lines 3B-HOSTILE-ALLOC moves to the link simulator's change-set; at 2,750 the
  36,000-tick run and the two interleaved matches move to 4c. He is told before a part moves.

## Steps done
0. Setup: his go recorded by script; main c1da1bd taken in (docs only); the pinned bindings tool fetched as CI does
   and checked (11,860,397 bytes, sha256 9687f3bb...227f; the program 8,170,784 bytes, sha256 ca4df773...6931),
   kept under `.gate/wb-tool/`; `cargo fetch --locked`; `npm ci --ignore-scripts`.

## Current step
Step 1 (one visitor drives every writing of the state) and step 2 (`Sim::shared_hash()` with its pin).

## Next steps
1. Steps 1 and 2 (plan section 4), each with its tests and planted defects.
2. Steps 3 to 5: the recording's format, the runner, the comparer, the scripts and the two tools.
3. Before the first recording is committed: the hash report with rust-reviewer (PM-0004-21).
4. Steps 6 to 8: the first recording on its four runs; WebAssembly in Node and in Chromium; the three planted
   divergences; the counted "no allocation" for hostile bytes.
5. Reviewers (netcode-architect, rust-reviewer, test-engineer; then engine-architect, security-auditor), the
   verifier, the advisor agent on every pushed head, the docs pass, the report, a draft pull request.
6. The docs pass also carries: his blocks of 2026-11-25 into `docs/DECISIONS.md` by script; change-set 3b's owed
   docs pass; the Hornet's entry in `docs/UNITS.md` section 0 by script, the two lines of that section left as
   they are and listed as open for his unit walk (his go, point 6).
7. The report answers his go's point 4 in at most eight lines (what the first soldier on screen does not need from
   4b and from 4c's first five steps, and the hours it would save to build those after the early look), and says
   for every size whether it is the designs' count or already holds the 1.8.

## Open findings
- The checkpoint script cannot push from this worktree (it pushes the local branch's own name): notes are committed
  by path and pushed by hand; no saver runs · this chat · open.
- The board says CI's toolchain job ran 5 minutes 20 seconds on the CI Builder's branch (information, not checked
  here): 4a adds one browser spec; its time is measured before the first push with code.

## Key numbers
- Written so far: 0 lines of code, 0 of tests.

## Decisions taken
- WALK-CUT (a) and STEP-60 (a) · Mustafa · 2026-11-25, 19:36:25 UTC. No value of the 60-step Export is carried.
- His rules for this chat: no CI, kit or gate file is written here; never a push to another chat's branch; git and
  gh always start their own command. The board add-on is active.

## Running work
- No agent runs. No saver runs: the main session commits the notes by hand.

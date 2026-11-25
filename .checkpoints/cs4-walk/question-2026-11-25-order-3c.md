# Question for Mustafa, 2026-11-25: ORDER-3C (what is built first, change-set 3c or change-set 4, Walk)

_From the chat he calls Builder V4 (session_019YNMUVZR6ociHFWjByxkrq), lane 1 of the build. Asked for by point 6 of his
block of 2026-11-25 (`mustafa-2026-11-25-takeover.md` beside this file): "does change-set 4, Walk, need change-set
3c's link simulator or first recording to be built and judged? If it needs neither, or only one, what would it cost
to build Walk first and 3c after it?" The chat's five lines went to him in the chat at about 16:20 UTC; this file is
the same answer with its evidence, so that his Advisor can check it against the repository. Everything cited is on
main at 3ab9eaecf42e617578c083b7fda01fbdf8f004e2, unless a line names another commit. Nothing is built or planned
before his answer._

## The short answer
Walk needs **only one** of 3c's two parts, and that one for its proof, not for play.

| | To be built | To be played in his browser | For its proof (the pass marks the build plan places at Walk) |
|---|---|---|---|
| The link simulator (the test link that delays, drops and reorders) | no | no | no: its first user is change-set 7, Lag |
| The first recording (the replay runner, the comparer that names the first differing tick, a recorded match) | no | no | **yes** |

## The evidence, line by line
The build plan is `docs/reports/2026-11-17-build-plan-slice-1.md`.

1. **Built and played: neither.** Walk's card (lines 334-358) is the content crate, the soldier table, our own
   movement on the collision library, the command with its aim point, the block's soldier section and the page. All
   of it runs on the local loop that change-set 3b merged (`LocalMatch` in `crates/sim-wasm/src/local_match.rs`).
   Builder V3's handover note, part d, says the same.
2. **The link simulator: nothing at Walk uses it.**
   - The build plan places every mark that runs "under L1" (the link simulator) at change-set 3 or at change-set 7
     and 8: line 466 (the foundation's exchange over the loopback "with L1 behind it", shown at 3), line 474
     (PM-0002-49, the simulator's own mark, at 3), lines 486-489 (the four netcode marks of Slice 1, at 7, and the
     muzzle flash at 8). Card 7 (lines 386-397) is where "the fake lag switch puts L1 behind the loopback".
   - One sentence elsewhere says otherwise: change-set 3's plan (`.checkpoints/cs3b-loop-closes/cs3-plan.md` at
     commit 80a51e42, lines 142-143) puts 3c "right after 3b and before change-set 4 (whose own milestone needs
     L1's profiles)". The milestone that needs those profiles is Slice 1's as a whole (ADR 0001, lines 1783-1786),
     and the build plan meets it at change-set 7. Walk's own card names no L1.
3. **The first recording: Walk's proof rests on it.** This is the point Builder V3's note left open ("Whether a
   mark of Walk names the recording as its method I cannot say without new reading").
   - Walk's card, lines 353-354: "Measured here: identical hashes natively, in Node and in Chromium on a movement
     recording".
   - "In one screen", lines 173-174, under "Measured first, before more code rests on it": "the same result
     natively and in WebAssembly from the first tick: change-set 3 (the first recording, ...), then again on real
     movement in change-set 4".
   - Part 2: line 484 (a determinism recording per change-set that changes the simulation, "4 on"), line 485 (ADR
     0001's line "a recorded input replay gives the same per-tick hash on two runs", shown and met at 4), line 494
     (the determinism suite on Slice 1's recordings, "4 on: natively twice, Node, Chromium").
   - Part 3.4, the simulation draft's marks placed at 4 that run on the replay runner (their rows:
     `docs/adr/0004-simulation-core.md` lines 1138-1146): PM-0004-17 (nothing lives outside the state, "meaningful
     from 4", line 623), PM-0004-19 (native and WebAssembly agree tick by tick, "every recording from 4 on", line
     625), PM-0004-20 (the collision library's determinism flag holds across targets, line 626), PM-0004-21 (the
     three planted divergences are caught at their tick, line 627), PM-0004-25 (a reading with SIMD on, line 631).
   - None of that exists yet: `crates/devtools/src/lib.rs` holds the rule tests' runners only (6 lines; no replay
     runner, no comparer, no recording).
4. **What 3c is today.** `docs/STATUS.md`, "Next", item 2: "Then change-set 3c (new): the link simulator and the
   first recording, moved out of 3b by the size ladder, with the state's hash on the players' file and `devtools`'
   three path edges ... Then change-set 4". Its plan items from 3b's reviewers: `docs/STATUS_ARCHIVE.md` lines
   1547-1557. The planned sizes are change-set 3's plan's (the same file at 80a51e42, lines 136-137): the replay
   runner, the comparer and the first recording 275 lines; the link simulator 595 lines. At the ratio 3a and 3b ran
   at (1.7 and 1.8 times): about 500 and about 1,070.

## What it would cost to build Walk first and all of 3c after it
- **The proof is thinner at Walk's merge.** Walk brings the collision library, the first outside float code in the
  simulation (his MUL-ADD of 2026-11-24: the check on the built file comes with it). Without the recording, native
  against WebAssembly is shown on movement only by scripted tests (the rule tests' harness 3a built), not by a
  recorded match replayed on the file players get. The rows of point 3 slip to 3c.
- **If 3c then finds a mismatch, Walk's movement comes back** (his rule of 2026-11-14: "If a mark fails, the code
  that rests on it is redone").
- **The foundation's last marks stay open while Slice 1 starts** (the recording's and the link simulator's, lines
  466, 470 and 474). The link simulator's stay open in every order but (a).
- **Nothing is built twice.** 3c would record a match that already moves.
- **It changes the plan's order**, which his go of 2026-11-17 named ("then the change-sets in the plan's order"):
  his decision, which is why this is asked.

## Decisions needed
**ORDER-3C: what is built first?** It matters because he wants to play the first soldier as soon as that is safe,
and the order decides both when the soldier comes and how well its movement is proven when it merges.
- **(a) The plan's order: 3c whole, then Walk.** About 870 planned lines (about 1,600 at 1.8 times) and one full
  round of plan, reviews and his go before the soldier. Every proof is in place when Walk merges. The first
  recording is made on a state that holds no soldier and is made again in Walk, because Walk changes both the state
  and the command that a recording pins (change-set 3's plan, lines 73 and 148-150).
- **(b) Walk first, 3c whole after.** The soldier soonest; the cost is the list above.
- **(c) Split 3c at its seam.** The recording (the replay runner, the comparer, the state's hash readable on the
  players' file, the recorded match: 275 planned lines, about 500 at 1.8 times) becomes Walk's first step. The link
  simulator (595 planned, about 1,070) follows Walk and comes before change-set 7, its first user. The soldier
  comes one change-set sooner than in (a), no proof is thinner than the build plan has it, and the recording is
  made once, on a state that already moves. Its cost: Walk, already the plan's second-largest change-set (3,900
  planned lines before his SLICE-FEEL additions), grows by that step, so Walk's own plan has to say honestly
  whether it stays one stop or is cut in two, with the first playable soldier in the first.
- **Recommended: (c).** If Walk's plan shows it cannot carry the recording inside its stop line, the fallback is
  the recording alone as a small change-set before Walk (no browser check: nothing a player sees), and he is told
  before anything moves.

Where 3c's smaller items go under (c), to be fixed in the plans and not decided here: the state-hash export goes
with the recording (the replay reads it); the reviewers' items for the link simulator stay with it; the counted "no
allocation" for hostile bytes (3B-HOSTILE-ALLOC, "with 3c's tests") is placed by the plan of whichever part comes
first.

## For Mustafa
1. One letter for ORDER-3C: (a), (b) or (c). The plan for the stop he chooses follows, with the advisor agent's
   review, as his point 7 says.

# BRAWL-1's build notes (builder copy 1)

## Start (brief section 1)

- Brief: brawl1-brief.md, sha256 921c3243958a09dd5f914fb7eb247f07333558d0905b3d849d2f13222d6d2a50, 261 lines (checked).
- Worktree: /home/user/Shamble-Frontlines/.claude/worktrees/brawl1, detached (no branch), HEAD
  352a245c3018c50843f62dcb90e91973ea56d9da (= `<base>`), `git status --short` empty, spikes/netcode-proof tree
  abad063555b0105e2afd47dcff96acd7bd74f63e (checked 2026-11-15).
- Runner u4r-run.sh sha256 6efb91572cf06fd9ad6472c077cd39c9a59bfed8d4a3dac561992aafd5729bc8 (checked). Logs under
  /home/user/Shamble-Frontlines/.gate/u4r/brawl1/.
- Disk at start: 13,174 MiB free.
- No earlier notes: this is the first copy.

## Plan

1. Read the binding docs (advisor plan review, plan, rulings, facts) and check the facts I rely on.
2. Capture the precheck tests' --nocapture "before" log (r6, late-command bite, wallclock) at <base>.
3. Part 1 items 1-6 (EA-U4R-2, EA-U4R-1 narrow, N2-1 + TE-R2-1 with two plants, TE-R2-2 + RR4-L2, RR4-L1, RR3-L2 (a)).
4. Part 1 proofs (a)-(e) at the old pins; record each here before Part 2.
5. Part 2: npt-client read item + test; npt-bots leash (crate-private) + unit test + sim-only test (full ignored,
   short copy); the figure in Logs::room_tick + Counts + summary line; SERIES.md rows drafted here.
6. Full test by name in release; short copy times.
7. Re-pin the three entries; check.mjs, det-u4.mjs, precheck regeneration at the new pins.
8. git diff --stat; report.

## Progress

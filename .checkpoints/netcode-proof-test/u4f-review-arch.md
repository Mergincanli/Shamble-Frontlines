# U4f review panel, round 1: seat `arch` (engine-architect), 2026-11-19

## Plan
1. Read the seat's rules: u4f-brief.md 2.13 (public items by crate) and 4 (build rules on pub items/deps); plan.md
   §7.2 :930-931 (A19, the seam rule) and §5 :512; earlier arch reviews (u4e-review-arch.md, u4r-review-arch.md) for
   the rule's past reading.
2. Read Mustafa's SIZE-U4F words (history.md:6896-6903) and advisor-u4f-stop.md :80-89 (what is U4f2), :198-210.
3. In the worktree: `git status --short`, `git diff HEAD --stat`, the Cargo.toml diffs (crate edges), then every
   added/widened `pub` line in the diff (grep `^\+.*pub `), each against 2.13's list.
4. For each new pub item: who uses it across a crate edge (grep the other crates), whether it is needed by a §6 / plan
   item, whether it leaks room state (ownership), whether a test-only item is gated.
5. Dependency direction: npt-proto <- npt-room <- npt-server; npt-client; npt-bots; npt-lab at the top. Check no new
   edge points down-up, no new dep in Cargo.toml/Cargo.lock.
6. Check the client clock / lead controller seam (npt-client) against the Wasm boundary (no per-entity calls).
7. Check the U4f2 scope line: nothing built that lets -21 be judged before U4f2 (lab suites, scripts, SERIES lines).
8. Runs only to prove a finding (u4f-run.sh), few and short.

## Findings (one line each, as found)

# Checkpoint: netcode-judge
- **Goal:** the judge piece of the netcode proof test (PR #40's work), taken off the netcode chat's hands by Mustafa's
  start block of 2026-11-24 in the Netcode Builder 2 chat. Two parts, in order: **part a, MOVED** (the judge and
  marks.json report a moved pass mark, or a moved part of one, as MOVED and never as PASS; its text is his MARK-NOTES
  answer), then **part b, U6j** (the judge learns 19 figures and three waiting judge changes). Plan:
  `.checkpoints/netcode-judge/plan-a.md` (not written yet).
- **Branch:** `claude/practical-ride-igo41n` (this cloud session's own branch, the stand-in for `cs/netcode-judge`), in
  the worktree `.gate/netcode-judge`. **Base:** the netcode branch `claude/netcode-proof-test-plan-hcky5g` at
  e2c0e896dedd5c09c2536e2fbe9323353756fe99, pushed unchanged as this branch's first push (2026-11-24 19:5x UTC).
- **Last update:** 2026-11-24 20:55 Amsterdam (19:55 UTC).

## Rules in force (my summary; his words are in the Netcode Builder 2 chat, 2026-11-24, never retyped here)
- **Fence:** edits only in `spikes/netcode-proof/crates/judge/`, `spikes/netcode-proof/marks/` and these notes. In
  `marks/SERIES.md` only the rows my figures need. No pass mark and no number in marks.json changes, beyond what his
  MARK-NOTES answer allows (replan.md:106-111 on the netcode branch). No manifest or lockfile change: a needed
  dependency is a stop. Nothing under `.github/`, `.claude/`, `CLAUDE.md` or `tools/`.
- **Git:** never push to the netcode branch or any branch but this one; no pull request (the netcode chat merges this
  branch on his word); never forced. The main folder stays parked on the local name `park/main-folder` at main 1ec6f6c.
- **How:** read, a short plan, the advisor agent's APPROVE, then build. Every computation gets pass, fail and gap tests
  on made-up logs. One review round: test-engineer and rust-reviewer read every judge line. Agents on Opus 5.5 at the
  kit's effort (xhigh for netcode), at most 2 at a time. One resume card, no copies of rulings.
- **Checks (his answer, 2026-11-24):** the judge's tests, its lints and `spikes/netcode-proof/check.mjs` run in this
  container on each part's final head, reported with the full commit id and the exact commands; GitHub's two checks
  only have to stay green; no workflow file. The test-engineer reviewer runs the tests itself.
- **Size stops:** part a at 1,440 added lines; part b at 4,900. At a stop: cut, and three lines to him.
- **Not started, only on his word:** the one-way delay comparison on a real network (NPT-L2-ONEWAY).
- **Part b's count (his answer):** 19 figures (replan-units.md:63); the figure of the moved mark PM-0001-33 moves.
- The board add-on is active.

## Steps done
1. The worktree and the first push · e2c0e896 (the netcode head, unchanged) · done.

2. The reading and part a's plan · `.checkpoints/netcode-judge/plan-a.md` · baseline: the judge's 85 tests pass at
   the base (`cargo test -p npt-judge --locked`, 14 + 2 + 49 + 20).

## Current step
Part a: the advisor agent reviews plan-a.md (its five questions in section 5). No judge or marks edit before APPROVE.

## Next steps
1. The advisor's verdict into plan-a.md's end (verdict and conditions, short); fix the plan if REVISE.
2. Build part a in this worktree; the judge's tests, lints and check.mjs; the size count against 1,440.
3. One review round (test-engineer, rust-reviewer), one fix pass, the final head's checks, the report with the commit id.

## Open findings
- none yet

## Key numbers
- The judge today: src 5,238 lines (eval.rs 1,784; marks.rs 1,263; figures.rs 1,020; practice.rs 797), tests 3,618,
  marks.json 12,630 lines, 81 marks (wc -l at e2c0e896).

## Decisions taken
- His four answers of 2026-11-24 (checks in the container; 19 figures; the first push as the netcode head; the main
  folder parked) · Mustafa · in the chat.

## Running work
- The advisor agent on plan-a.md (launched 2026-11-24 19:53 UTC, xhigh; its verdict comes back to the session).
- Ready, not run on the file: `.checkpoints/netcode-judge/moved-table.mjs` (the labels as a table; its dry run finds
  every name: 26 marks whole, 19 parts, 2 checks, 1 bite; marks.json's sha256 after would be 152578df…; the table
  changes if the advisor rules on plan-a.md's questions 1 to 3).

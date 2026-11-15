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
- 5 next: the code.

# test-engineer: review of change-set 4b-1 "Content" (is it proven?), one read on head d36e416

Started 2026-11-26. Worktree .gate/lane1-takeover, branch cs/4b-content, code head d36e416 (+ 495204a notes).
Read-only; notes only here; target folder .gate/test-engineer-4b1-target; plants only in .gate/test-engineer-4b1-scratch.
Memory: none local (as in test-engineer-testlist.md).

## Plan
1. Briefs (done): review-brief-4b1-test-engineer.md, review-brief-4b1.md (rules), review-brief-4b1-round2.md.
2. Card steps 9-17, 23, 25-27; test-list.md items for steps 1-5; plants-4b1.py; local-checks.sh.
3. Whole diff `git diff origin/main...HEAD -- crates packages`: file list, then per file.
4. Look-hardest list 1-10 in order: content_proofs.rs case() (circularity), vectors digests (regeneration),
   content_fuzz.rs (shares, seed, reach), limit cases + allocator + content.test.mjs ceilings, map.rs + aim_reach.rs,
   time unit + report, links check, rules_scan content scan, plants list (no plant / wrong check), CI vs by hand.
5. Run: cargo test (debug) for content + devtools in my target; selected tests; scratch plants in a copy where a
   point needs proof.
6. Findings as found (below), then the handback (<= 25 lines).

## Source notes (one line per file/source as read)

## Findings (one line per finding as found)

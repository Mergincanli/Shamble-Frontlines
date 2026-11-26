# Advisor review of head 851f004a (PR #72), scope since 90c939f4
Plan: (1) merge file lists both ways, toolchain.yml = main, five sha256, no kit/gate beyond main;
(2) lane 1 / CI Builder lines, DECISIONS order, run verify74/control74; (3) judgement calls (kit-hold move, STATUS 125);
(4) notes commits: no secrets/identifiers, his answers faithful, T7 accurate; (5) CI on 851f004; (6) item-6 reading.
Main's .claude/ and CLAUDE.md: unchanged 4a62873..3b949c1 (git diff --stat empty).
## Findings
- 1 ok: file lists both ways equal (5ab420d7->851f004 = main's 8 since 4a62873; 3b949c1->851f004 = branch's 74 since 4a62873); one-sided files equal their side (git diff --stat empty both ways); raw diff vs main: all 100644, no .claude/CLAUDE.md/tools/ path; five sha256 match at 851f004 (worktree clean, HEAD was 851f004).
- 2 ok: side copies m74/{base,ours,theirs} = git blobs (hash-object vs rev-parse, 12/12); verify74 PASS (151/158(+7)/155(+4)/162; ROADMAP 12; STATUS 125 = 125+0).
- note: during the review the session committed a187c84 and f8d97a1 on top (notes only: checkpoint, this file, 1715 message, save_1715/resolve74/verify74/control74); docs/.github unchanged from 851f004.
- 3: OLD-LINES (a) (DECISIONS:2533) requires "Its report lists each line it moved"; the checkpoint's docs-pass list (step-2-6-previews.md:265-266) names Part 15 and change-set 1 only, not the kit hold: Low (L1 widened).

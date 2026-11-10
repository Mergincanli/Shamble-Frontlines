# Advisor: U4 wave 1 rulings (PR #40, branch claude/netcode-proof-test-plan-hcky5g, draft)

Seat: day session, advisor agent run as a general agent reading main's `.claude/agents/advisor.md` (this session
started before main's advisor.md last changed). Read-only except this file. Effort: high (netcode work: the caller
asked for rulings, not an ADR).

## Plan
1. Read the sources: resume card (netcode-proof-test.md top), history.md "Decisions taken" of 2026-11-10, advisor-u4.md
   (verdict, conditions, Rulings, U4s, F1/F2), u4-build-plan.md rev 2 (check sha256), plan.md (the sections the rulings
   touch: §4.1, §4.2, §4.4, §4.14, the U8 row, line 1436), u4s-carries.md, the message catalogue.
2. Read wave 1's records and reviews: u4a-build.md, u4b-build.md, u4c-build.md (context only), u4d-build.md,
   u4ab-review-{netcode,security,rust,test}.md, u4d-review-{netcode,rust,arch,test}.md, skeptic S1 (git show), the U4e
   brief draft rev 2 (git show).
3. Spot-check the code where a ruling turns on it (U4d's worktree: the vehicle parts, the press counters, the stress
   scene, D-8/D-9, load_movers; U4a's worktree: SOLDIER, GROUPS, the snapshot header, the public items).
4. Rule, one block per code, writing each ruling below as soon as it is made:
   W1-U4D (High, each Medium, the press-counter wording, the two tuning choices, carry 13), W1-S1, W1-N2, W1-PUB,
   W1-U4E (Q1-Q6, Q8, Q10, Q11), W1-PLAN, W1-TARGET.
5. Return the report: each code with its ruling in one or two lines, the text changes, Decisions needed, For Mustafa.

## Sources read (filled in as I go)
- main's advisor.md (git show origin/main:.claude/agents/advisor.md); the resume card (netcode-proof-test.md:1-120);
  history.md:280-380 (Mustafa's 2026-11-10 words: RESUME-CARD, WIP-CLEAN, TCP-FREEZE, SQUASH-40, PROMOTE,
  SIZE-CHECK, the second lane; his "A" = SQUASH-CITES (a) and NS-START (a); WS-BAR and its refinement; WS-SCOPE).
- advisor-u4.md whole (verdict, conditions 1-13, Rulings, U4s ruling, F1/F2).
- u4-build-plan.md :177-500 and :496-895 (sha256 0d69332c3c60c83c9328504259c279609b546e15cc5510f528346471aabc3b84,
  1,033 lines: the approved rev 2 as recorded after F1); u4s-carries.md (sha256 505e54d6…, the F2 value: unchanged).
- plan.md :568-600 (§4.7), :1236-1248 (§12.1, the U8 row :1247), :1425-1438 (§16; :1436 the never-squash line);
  advisor-u3q.md :120-145 (S13, S14: "A miss goes to the advisor before the WebSocket part of -18 counts").
- Reviews: u4d-review-{netcode,rust,arch,test}.md; u4ab-review-{netcode,security,rust,test}.md. Builder notes:
  u4a-build.md (whole, incl. the fix pass's progress to step 4), u4d-build.md (whole: it has no "For the advisor"
  section, so I ruled U4d's public items from the code's surface, crates/sim/src/lib.rs in its worktree).
- Skeptic S1 at 6652be4 (:20-31); U4e brief draft rev 2 at 0dccbf0, §8 (its 11 codes).
- Code read (worktrees): U4d seam.rs :160-470 (layouts, HitVolumes, InputReports), world.rs :1-60, :235-300, :470-505,
  :570-600, :1025-1050, :1430-1445, :1515-1545, :1635-1650, scene.rs :95-140, lib.rs; U4a lib.rs, snapshot.rs :1-40,
  :380-460, :632-652, quant.rs :25-60, wrap.rs :14-42, tests/bites.rs :40-125, client/src/lib.rs.
- Catalogue :125-150; ADR 0001 :200-212, :222-245, :300-330, :1468-1484, :1607 (PM-0001-58's row); marks.json
  PM-0001-2, -7, -58 (read-only parse).
- Disk: 12 GB free; .gate/shared-target 4.7 GB, the main folder's cache 6.5 GB. flock 2.39.3 present.

## Rulings (one block per code, appended as made)

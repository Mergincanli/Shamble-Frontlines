import sys
p = 'docs/STATUS.md'
s = open(p, encoding='utf-8').read()
old_branch = "| `claude/sleepy-tesla-ofsxsr` (stands in for `cs/b2-community-servers`; local branch `b2-community-servers` from main ee736ec), PR #42 | done:"
new_branch = ("| merged: PR #42 (`claude/sleepy-tesla-ofsxsr`, standing in for `cs/b2-community-servers`; local branch "
              "`b2-community-servers` from main ee736ec), 4c6db5b, 2026-11-06 23:57:35 UTC (`merged_at`), by merge commit "
              "(parents ee736ec and e223ecc), on Mustafa's go (\"merge #42\") after the advisor's APPROVE WITH NOTES of "
              "e223ecc (comment 6698888516); CI green on main (the gate workflow, run 42315225217, on 4c6db5b); report "
              "`docs/reports/2026-11-06-b2-community-servers.md` | done:")
assert s.count(old_branch) == 1
s = s.replace(old_branch, new_branch)
old_tail = ("the notes deleted in the last commit (readable at 5c0999f) | the advisor's final-head review, then Mustafa's go "
            "(`needs-mustafa`: his words; no Brave check) |")
new_tail = ("the notes deleted in the last commit (readable at 5c0999f); the advisor's APPROVE WITH NOTES of the final head "
            "e223ecc, with four Low notes, taken by vfx-xp-notes (PR #43): the queue entry's \"Next after B.2\", #40's "
            "stale lines (its STATUS :3, :31 and :173 at 0588a17), For Mustafa 1(b)'s wording, and the check script's "
            "FAIL (1) on the dated Appendix B title; its pull_request CI run 42313665674 stalled once installing "
            "Chromium's system libraries (Open bugs and risks) | none |")
assert s.count(old_tail) == 1
s = s.replace(old_tail, new_tail)
anchor = "| **B.2, VISION_V2's community-servers note and B3's hand-over** ("
assert s.count(anchor) == 1
row = ("| **vfx-xp-notes, the VFX editor and private-match XP** (vfx-xp-notes; Mustafa's two items of 2026-11-06, both "
       "\"For the next docs pass\", next by his VFX-ORDER (a). Docs only, symbols +0 −0: his VFX editor item, word for "
       "word, as EDITOR_V2 §7, \"A future tool: the VFX editor (after the map features)\", the tool `[LATER]` and each "
       "of his ten principles `[DECIDED]`, with a dated header line (EDITOR_V2:7); his private-match XP rule in "
       "VISION_V2's community-servers note, after the trust rule (VISION_V2:71-75): `[DECIDED]` 25 % of normal XP with "
       "bots in the match, 100 % humans only, and the Advisor's two suggestions `[OPEN]`, as he marked them; \"bears on "
       "it\" clauses in four register rows (`docs/adr/README.md:104`, `:122`, `:160` and `:183`: OQ-0003-16, "
       "OQ-0005-CD7, OQ-0006-21 and R2-ROOM-OQ7), no ADR draft or LOOK_V2 edited; STATUS and ROADMAP after PR #42; "
       "the B.2 report's corrections after the merge) | `claude/sleepy-tesla-ofsxsr` (stands in for "
       "`cs/vfx-xp-notes`; local branch `vfx-xp-notes` from main 4c6db5b), PR #43 | done: plan revision 1 and the "
       "advisor's APPROVE with 10 conditions (`.checkpoints/vfx-xp-notes/advisor-plan-rev1.md`); the build (d5e2c62): "
       "EDITOR_V2 §7 and its header line, VISION_V2's XP block, the four register clauses; E1-E3 PASS (`e-checks.py`, "
       "30 checks: his words byte for byte; EDITOR_V2 `[DECIDED]` 11 to 21 and `[LATER]` 2 to 3, VISION_V2 `[DECIDED]` "
       "12 to 13 and `[OPEN]` 3 to 4); `gate.mjs 4c6db5b --no-smoke` PASS, symbols +0 −0 (`gate-build.txt`); "
       "docs-keeper's pass (STATUS, ROADMAP, the B.2 report's corrections; four follow-ups \"From vfx-xp-notes\") | the "
       "review round (the verifier, game-designer, art-director), the report, the advisor's final-head review, then "
       "Mustafa's go (`needs-mustafa`: his words; no Brave check) |\n")
s = s.replace(anchor, row + anchor)
open(p, 'w', encoding='utf-8').write(s)
print('ok')

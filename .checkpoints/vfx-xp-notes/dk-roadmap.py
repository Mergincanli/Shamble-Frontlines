p = 'docs/ROADMAP.md'
s = open(p, encoding='utf-8').read()
def rep(a, b):
    global s
    assert s.count(a) == 1, (s.count(a), a[:80])
    s = s.replace(a, b)
# 1. "v2, next" queue text
rep("(B.2), in flight as PR #42; next, in the session's reading of his \"For the next docs pass\" (VFX-ORDER, asked in "
    "chat), the VFX editor note in EDITOR_V2 (his item of 2026-11-06, its own docs change-set); then the review policy (A4)",
    "(B.2), merged as PR #42; the VFX editor note in EDITOR_V2, with his private-match XP rule in VISION_V2 (his two "
    "items of 2026-11-06; his VFX-ORDER (a): next after B.2, its own docs change-set), in flight as PR #43; then the "
    "review policy (A4)")
# 2. B.2's entry to MERGED
rep("**VISION_V2's community-servers note and B3's hand-over [2026-11-06, PR #42: built 67978c2, reviewed, fix pass "
    "8d1540f]** — b2-community-servers (B.2 on Mustafa's list; docs only, symbols +0 −0; plan revision 1",
    "**VISION_V2's community-servers note and B3's hand-over [2026-11-06, MERGED]** — b2-community-servers (PR #42, "
    "4c6db5b, 23:57 UTC, by merge commit, on Mustafa's go (\"merge #42\") after the advisor's APPROVE WITH NOTES of "
    "e223ecc; CI green on main; B.2 on Mustafa's list; docs only, symbols +0 −0; built 67978c2, fix pass 8d1540f; plan "
    "revision 1")
rep("Report: `docs/reports/2026-11-06-b2-community-servers.md`. Waits: the advisor's final-head review, his go "
    "(`needs-mustafa`; no Brave check).",
    "Report: `docs/reports/2026-11-06-b2-community-servers.md` (its corrections after the merge: vfx-xp-notes). The "
    "advisor's APPROVE WITH NOTES of the final head e223ecc (PR comment 6698888516), with four Low notes, taken by "
    "vfx-xp-notes; merged on his go (`needs-mustafa`; no Brave check). One pull_request CI run stalled installing "
    "Chromium's system libraries and passed on a re-run (STATUS, Open bugs and risks).")
# 3. the new entry, newest first
anchor = "**VISION_V2's community-servers note and B3's hand-over [2026-11-06, MERGED]**"
entry = ("**The VFX editor and private-match XP [2026-11-07, PR #43: built d5e2c62]** — vfx-xp-notes (Mustafa's two items "
         "of 2026-11-06, both \"For the next docs pass\", next by his VFX-ORDER (a); docs only, symbols +0 −0; plan "
         "revision 1 with the advisor's APPROVE and 10 conditions): his VFX editor item goes word for word into EDITOR_V2 "
         "as §7, a future tool after Editor v2's map features (the tool `[LATER]`, each of his ten principles "
         "`[DECIDED]`), with a dated header line; his private-match XP rule goes into VISION_V2's community-servers note "
         "after the trust rule (`[DECIDED]`: 25 % of normal XP with bots in the match, 100 % humans only; the Advisor's "
         "two suggestions `[OPEN]`, as he marked them); \"bears on it\" clauses in four register rows (OQ-0003-16, "
         "OQ-0005-CD7, OQ-0006-21, R2-ROOM-OQ7; no ADR draft or LOOK_V2 edited); STATUS and ROADMAP after PR #42; the "
         "B.2 report's corrections after the merge. E1-E3 PASS (his words byte for byte; EDITOR_V2 `[DECIDED]` +10 and "
         "`[LATER]` +1, VISION_V2 `[DECIDED]` +1 and `[OPEN]` +1); gate `--no-smoke` PASS. Follow-ups (STATUS, \"From "
         "vfx-xp-notes\"): which \"XP\" his rule means (XP-TERM: the docs' other XPs listed, none mapped); his \"Comic "
         "flavour\" against LOOK_V2 §7's \"used sparingly\" (LOOK), for the look brief's owners; the `[LATER]` tool gives "
         "the drafts no engine feature now (decision map J3); his VFX style guide and its reference study, a follow-up "
         "only, not queued. Waits: the review round, the report, the advisor's final-head review, his go "
         "(`needs-mustafa`; no Brave check).\n\n")
rep(anchor, entry + anchor)
# 4. fold the oldest recent entry (13 -> 12) into Build history
old = ("\n\n**Docs after R7 [2026-10-27, MERGED]** — docs-after-r7 (PR #29, e9f11be, 2026-10-27, on Mustafa's go after the "
       "advisor's APPROVE of d973b3f; docs only): STATUS and ROADMAP catch up with v2 phase 0 and the Advisor's "
       "2026-10-27 decisions.")
rep(old, "")
rep("- **R7 legacy inventory (2026-10-27):** PR #27, 07ab240",
    "- **R7 legacy inventory + docs after R7 (2026-10-27):** PR #27, 07ab240")
open(p, 'w', encoding='utf-8').write(s)
print('ok')

d='/home/user/Shamble-Frontlines/.checkpoints/foundation-restructure/'
def edit(fn, pairs):
    p=d+fn; s=open(p).read()
    for a,b in pairs:
        assert s.count(a)==1,(fn,a[:70]); s=s.replace(a,b)
    open(p,'w').write(s)
edit('docs-keeper-gen-decisions.py',[
("""ptr_freeze = ['    _Pointer: the foundation chat\\'s own record of FREEZE-FDN (a), SPLIT (a) and SCOPE (a), in his words, under',
              '    "Decisions, newest first": the entry "Taken by Mustafa, 2026-11-10: FREEZE-FDN (a), SPLIT (a), SCOPE (a) and the',
              '    build go for 2.1a, in the foundation chat"._']
ptr_split = ['    _Pointer: the foundation chat\\'s own record of SPLIT (a) and SCOPE (a), in his words: the entry "Taken by Mustafa,',
             '    2026-11-10: FREEZE-FDN (a), SPLIT (a), SCOPE (a) and the build go for 2.1a, in the foundation chat", above._']""",
"""ptr_freeze = ['    - _Pointer: the foundation chat\\'s own record of FREEZE-FDN (a), SPLIT (a) and SCOPE (a), in his words, under',
              '      "Decisions, newest first": the entry "Taken by Mustafa, 2026-11-10: FREEZE-FDN (a), SPLIT (a), SCOPE (a) and',
              '      the build go for 2.1a, in the foundation chat"._']
ptr_split = ['    - _Pointer: the foundation chat\\'s own record of SPLIT (a) and SCOPE (a), in his words: the entry "Taken by',
             '      Mustafa, 2026-11-10: FREEZE-FDN (a), SPLIT (a), SCOPE (a) and the build go for 2.1a, in the foundation chat",',
             '      above._']"""),
("""    '  session** (in the foundation chat, session_019YNMUVo8MVM3gdiMNaN5xs, on his personal account; sent after 2.1a\\'s',
    '  report, whose checkpoint is ffc16d8 at 11:57:47 UTC, and saved by that chat at 15:02:59 UTC, 828a3e0; saved by the',
    '  foundation chat; not re-checked against its chat record). His item 1 covers 2.1a\\'s two gate-file writes',""",
"""    '  session** (in the foundation chat, session_019YNMUVo8MVM3gdiMNaN5xs, on his personal account; sent after 2.1a\\'s',
    '  report (PR #47) and saved by that chat at 15:02:59 UTC, 828a3e0; saved by the foundation chat; not re-checked',
    '  against its chat record). His item 1 covers 2.1a\\'s two gate-file writes',"""),
])
edit('docs-keeper-check-decisions.py',[
("        elif s.startswith('_Pointer'): mode = 'pointer'","        elif s.startswith('_Pointer') or s.startswith('- _Pointer'): mode = 'pointer'"),
("ptr = [l for h in inserted for l in h if l.lstrip().startswith('_Pointer')]","ptr = [l for h in inserted for l in h if l.lstrip().startswith(('_Pointer', '- _Pointer'))]"),
])
edit('docs-keeper-status.py',[
("CI green on main (the gate workflow, run 42991918886, on bb2d817)","CI green on main (push run 42991918886, on bb2d817)"),
("""  kit true after its own moves" ("Each such edit still asks me and merges on my go"). His addition of 2026-11-11 for
  the foundation chat: a kit or gate edit asks him in chat first, with the exact change (`docs/DECISIONS.md`).''')""",
"""  kit true after its own moves" ("Each such edit still asks me and merges on my go"). His two additions of
  2026-11-11 for the foundation chat: `docs/DECISIONS.md`.''')"""),
("approves its final head; no Brave check for 2.1a. Then 2.1b, then #48 (2.2), each on his go.","approves its final head; no Brave check for 2.1a. Then, in his order of 2026-11-11, 2.1b and #48 (2.2)."),
])
edit('docs-keeper-roadmap.py',[
("""`gate-globals.json` is called a gate file. Proof: the mutation proof, 31 of 31 failing alike on the old gate, the
edited gate at the root and the edited gate on `legacy/`; base checks over root, CS-1 and `legacy/` bases; gate PASS
and CI green.""",
"""`gate-globals.json` is called a gate file. Proof: the mutation proof, 31 of 31 PASS (every planted fault fails the
same checks on the old gate, the edited gate at the root and the edited gate on `legacy/`; every control passes on all
three); base checks over root, CS-1 and `legacy/` bases; gate PASS and CI green."""),
])
print('edited')

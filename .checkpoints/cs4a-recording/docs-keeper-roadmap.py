# docs-keeper, change-set 4a's docs pass: ROADMAP. The oldest "Recent work" entry (change-set 0, PR #56, merged) is
# moved by line range, byte for byte, to the end of "Build history"'s merged paragraphs; the 4a entry goes on top.
P = 'docs/ROADMAP.md'
L = open(P, encoding='utf-8').read().split('\n')
assert L[199].startswith('2026-11-17 17:09:33 UTC by `merged_at`') and L[200] == '' and L[201].startswith('- **Kit: KEPT-LOGS')
assert L[369] == '' and L[370].startswith('**Change-set 0 of the build plan [2026-11-17, MERGED]**')
assert L[376].startswith('stands as written') and len(L) in (377, 378) and (len(L) == 377 or L[377] == '')
cs0 = L[370:377]
assert L[243] == '## Recent work' and L[244].startswith('*(Newest first') and L[245] == ''
entry = """**Change-set 4a of the build plan, The recording [2026-11-25]** — cs4a-recording (the first of change-set 4's four
parts, his WALK-CUT (a); the recording came here from the old change-set 3c by his ORDER-3C (c)), by lane 1, built by
the chat he calls Builder V4: a recorded match replays to the same hashes, tick by tick, natively (one process; two
more processes with different hash-map seeds, with a 36,000-step run; two matches in turn) and in WebAssembly on the
file players get (in Node, and in Chromium in a Worker). The first recording, 1,800 steps on today's state, is
committed with its hashes, a word-size vector and a manifest, checked by `make_recordings --check` and every test run.
The one new number on the players' file is the shared state's hash (`Sim::shared_hash`), read only by a replay and fed
straight from the state's one visitor, with no buffer; the hash report (PM-0004-21) keeps FNV-1a 64. On the fixed head
15 planted defect checks were caught at their targets and 4 controls held. A player sees nothing new: no browser
check. About 3,300 written lines against the stop line of 3,400 (his STOP-4A (a); the designs counted 1,365); the size
ladder moved 3B-HOSTILE-ALLOC to the link simulator's change-set. Five reviewers in two rounds, all APPROVE in round 2.
PR #70; it merges on his go for the head the advisor agent approves, after his Advisor's merge check.
""".rstrip('\n').split('\n')
L = L[:200] + [''] + cs0 + L[200:246] + entry + [''] + L[246:369] + L[377:]
open(P, 'w', encoding='utf-8').write('\n'.join(L))
print('folded change-set 0 (7 lines) into Build history; 4a entry added on top of Recent work')

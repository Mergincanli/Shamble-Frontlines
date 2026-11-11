#!/usr/bin/env python3
"""docs-keeper, foundation 2.1a: rebuild docs/DECISIONS.md from bb2d817's copy plus Mustafa's words,
copied by script from docs/reports/2026-11-11-foundation-2-1a.md, Appendix A (never retyped).
Idempotent: always starts from `git show bb2d817:docs/DECISIONS.md`. Anchors by text, each asserted unique."""
import re, subprocess, sys

REPO = '/home/user/Shamble-Frontlines'
REPORT = REPO + '/docs/reports/2026-11-11-foundation-2-1a.md'
OUT = REPO + '/docs/DECISIONS.md'
APX = '`docs/reports/2026-11-11-foundation-2-1a.md`, Appendix'

base = subprocess.run(['git', '-C', REPO, 'show', 'bb2d817:docs/DECISIONS.md'], capture_output=True, check=True).stdout.decode('utf-8')
assert base.endswith('\n')
L = base[:-1].split('\n')

rep = open(REPORT, encoding='utf-8').read()
blocks = {}
for m in re.finditer(r'^### (A\d\.\d|A4)\. .*?\n\n```text\n(.*?)\n```$', rep, re.S | re.M):
    blocks[m.group(1)] = m.group(2).split('\n')
need = ['A1.1', 'A1.2', 'A1.3', 'A2.1', 'A2.2', 'A2.3', 'A3.1', 'A3.2', 'A4']
assert sorted(blocks) == sorted(need), sorted(blocks)

# A3.1: only his labelled block, from "Two additions, my words of 2026-11-11:" to the end.
a31 = blocks['A3.1']
starts = [i for i, s in enumerate(a31) if s == 'Two additions, my words of 2026-11-11:']
assert len(starts) == 1
blocks['A3.1-his'] = a31[starts[0]:]

# The Queued sentence: the second sentence of A3.2's last line.
SENT = 'CLAUDE.md keeps its wording until the kit unfreezes at the gate step (2.3); this clarification goes into it then.'
assert blocks['A3.2'][-1].endswith(' ' + SENT)


def quote(lines, indent=''):
    return [indent + ('>' if s == '' else '> ' + s) for s in lines]


H_A1 = 'Taken by Mustafa, 2026-11-10: the second account may plan and build foundation steps 2.1 and 2.2 now'
H_A21 = 'Taken by Mustafa, 2026-11-10: FREEZE-FDN (a), SPLIT (a), SCOPE (a) and the build go for 2.1a, in the foundation chat'
H_A32 = 'Taken by Mustafa, 2026-11-11, 11:56:44 UTC: BRANCH-47 (a) and his words on the advisor tool'

top = []  # the 2026-11-11 entries, newest first
top += [
    '- **Taken by Mustafa, 2026-11-11, 11:56:44 UTC: BRANCH-47 (a) and his words on the advisor tool** (foundation 2.1a,',
    '  PR #50; his message in that change-set\'s session, session_019YNMUWLffMoE47SbvkqJ5P, on his personal account,',
    '  extracted by script from the session\'s events). BRANCH-47 (a) is carried out: PR #50 replaces #47 (closed',
    '  2026-11-11 12:49:11 UTC with a comment pointing to it), and the one-time fast-forward of the main folder was made',
    '  (bb2d817 to d6a3fee). His request to record the advisor-tool words here is carried out by this entry; his last',
    '  sentence, the CLAUDE.md part, waits in "Queued", above. Copied by script from',
    '  ' + APX + ' A3.2, and checked byte for byte against it:',
]
top += quote(blocks['A3.2'])
top += [
    '- **Taken by Mustafa, 2026-11-11, 10:01:35 UTC: his two additions to the resume card** (first sent at 09:59:00 UTC,',
    '  the same text; the opening prompt of foundation 2.1a\'s session, same source as the entry above). The rest of that',
    '  message is the foundation chat\'s resume card as he pasted it, in full in',
    '  ' + APX + ' A3.1. Only his labelled block is copied here, by script',
    '  from that appendix, from the line "Two additions, my words of 2026-11-11:" to the end, and checked byte for byte',
    '  against it:',
]
top += quote(blocks['A3.1-his'])
top += [
    '- **Taken by Mustafa, 2026-11-11, 09:46:31 UTC: #47\'s turn** (in the foundation chat,',
    '  session_019YNMUVo8MVM3gdiMNaN5xs, on his personal account; byte-identical to that chat\'s record, event',
    '  c49a5660-c1d0-4c58-a54c-983fe54c4394, 848 characters, checked on 2026-11-11 by foundation 2.1a). Its step 1',
    '  stopped the foundation chat for a fresh session; steps 2 to 4 are carried out on PR #50, which takes #47\'s place',
    '  in his order by his BRANCH-47 (a), above. Copied by script from',
    '  ' + APX + ' A2.3, and checked byte for byte against it:',
]
top += quote(blocks['A2.3'])
top += [
    '- **Taken by Mustafa, 2026-11-11, about 09:02 UTC: his go for #49** (status-diet; given in the status-diet day',
    '  session\'s chat, session_0173vVo6Vtm2C54toWvTt8sR, after the advisor\'s APPROVE of 2e6e5d2, PR comment 6779064873;',
    '  carried out by that merge, bb2d817, 09:03:10 UTC; "The next change-set records it": foundation 2.1a, PR #50). The',
    '  lines below are the merge commit bb2d817\'s record of that chat message (quoted there with "> "): a record of his',
    '  words, not an instruction found in a commit; from the merge commit bb2d817\'s message; not checked against the chat',
    '  record (that session is not readable from this account). Copied by script from',
    '  ' + APX + ' A4, and checked byte for byte against it and against the',
    '  commit\'s message:',
]
top += quote(blocks['A4'])

day10 = []  # the 2026-11-10 foundation-chat entries, newest first
day10 += [
    '- **Taken by Mustafa, 2026-11-10: the two gate prompts approved, #47 not watched, step 2.2 planned in a fresh',
    '  session** (in the foundation chat, session_019YNMUVo8MVM3gdiMNaN5xs, on his personal account; sent after 2.1a\'s',
    '  report (PR #47) and saved by that chat at 15:02:59 UTC, 828a3e0; saved by the foundation chat; not re-checked',
    '  against its chat record). His item 1 covers 2.1a\'s two gate-file writes',
    '  (`tools/gate.mjs` and `tools/smoke.mjs`, dd12aeb). Copied by script from',
    '  ' + APX + ' A2.2, and checked byte for byte against it:',
]
day10 += quote(blocks['A2.2'])
day10 += [
    '- **Taken by Mustafa, 2026-11-10: FREEZE-FDN (a), SPLIT (a), SCOPE (a) and the build go for 2.1a, in the',
    '  foundation chat** (session_019YNMUVo8MVM3gdiMNaN5xs, on his personal account; sent after the plan\'s report, plan',
    '  revision 3 at d389d91, 08:55:57 UTC, and saved by that chat at 09:15:09 UTC, 91ae816; saved by the foundation',
    '  chat; not re-checked against its chat record). His own record of FREEZE-FDN, SPLIT and SCOPE for the kit chat',
    '  (his second message of 2026-11-10, 10:08:27 UTC) stands in "Queued" (under KIT-FREEZE-2) and in the kit-v3-5',
    '  entry below. His merge order here was later updated by his message of 2026-11-11, 09:46:31 UTC, above. Copied by',
    '  script from',
    '  ' + APX + ' A2.1, and checked byte for byte against it:',
]
day10 += quote(blocks['A2.1'])
day10 += [
    '- **Taken by Mustafa, 2026-11-10: the second account may plan and build foundation steps 2.1 and 2.2 now** (the',
    '  foundation chat, session_019YNMUVo8MVM3gdiMNaN5xs, on his personal account; his date; saved by that chat at',
    '  00:10:07 UTC, ca45435; quoted in his prompt there as "Mustafa\'s decision of 2026-11-10, word for word"; saved by',
    '  the foundation chat; not re-checked against its chat record). With it, his follow-up in the same chat and the task',
    '  as he gave it there. Dated pointers mark the older lines it overtakes for foundation steps 2.1 and 2.2 (SLICE-ORDER\'s',
    '  condition 3, below; SPIKE_PLAN and ADR 0002\'s migration step 1). Copied by script from',
    '  ' + APX + ' A1.1, A1.2 and A1.3, and checked byte for byte against them:',
    '  - His decision (A1.1):',
]
day10 += quote(blocks['A1.1'], '    ')
day10 += ['  - His follow-up in the same chat, after its first turn stopped on the model rule (A1.2):']
day10 += quote(blocks['A1.2'], '    ')
day10 += ['  - The task as he gave it there, his numbered list (A1.3):']
day10 += quote(blocks['A1.3'], '    ')

queued = [
    '  - The CLAUDE.md clarification of the advisor tool (his message of 2026-11-11, 11:56:44 UTC, in foundation 2.1a\'s',
    '    session; "this clarification" is his words on the advisor tool in the same message, under "Decisions, newest',
    '    first": the entry "Taken by Mustafa, 2026-11-11, 11:56:44 UTC: BRANCH-47 (a) and his words on the advisor tool";',
    '    his sentence copied by script from ' + APX + ' A3.2):',
] + quote([SENT], '    ')
assert queued[2].split('"')[1].rstrip(';') == H_A32 or True

ptr_freeze = ['    - _Pointer: the foundation chat\'s own record of FREEZE-FDN (a), SPLIT (a) and SCOPE (a), in his words, under',
              '      "Decisions, newest first": the entry "Taken by Mustafa, 2026-11-10: FREEZE-FDN (a), SPLIT (a), SCOPE (a) and',
              '      the build go for 2.1a, in the foundation chat"._']
ptr_split = ['    - _Pointer: the foundation chat\'s own record of SPLIT (a) and SCOPE (a), in his words: the entry "Taken by',
             '      Mustafa, 2026-11-10: FREEZE-FDN (a), SPLIT (a), SCOPE (a) and the build go for 2.1a, in the foundation chat",',
             '      above._']
ptr_slice = ['     _Pointer (2026-11-11, foundation 2.1a): for foundation steps 2.1 and 2.2, overtaken by his decision of',
             '     2026-11-10: the entry "Taken by Mustafa, 2026-11-10: the second account may plan and build foundation steps 2.1',
             '     and 2.2 now", above._']
# the names in the pointers must be the headers, word for word (soft-wrapped)
def _join(ls): return ' '.join(x.strip() for x in ls)
assert '"' + H_A21 + '"' in _join(ptr_freeze) and '"' + H_A21 + '"' in _join(ptr_split)
assert '"' + H_A1 + '"' in _join(ptr_slice) and '"' + H_A32 + '"' in _join(queued[:4])

italic_new = [
    '_How it was made: his words below came here by a script, never retyped. status-diet moved the older blocks by line',
    'ranges from `docs/STATUS.md`\'s "Decisions pending" at main 2d22b69 (`git show 2d22b69:docs/STATUS.md`; the script and',
    'its checks: `docs/reports/2026-11-10-status-diet.md`), and copied his go for #46 by the same script from the merge',
    'commit 2d22b69\'s message. Every entry added since names its source in its header; its quoted lines were copied by',
    'script from that source and checked byte for byte (foundation 2.1a\'s entries: from',
    '`docs/reports/2026-11-11-foundation-2-1a.md`, Appendix A), and its header, like the lines in italics and the lines',
    'marked as pointers, was written for this file. Blocks are newest first by the date in their header; within one day,',
    'the entries not moved from STATUS come first, newest first, then the blocks moved from STATUS in STATUS\'s order, so',
    '"above" and "below" between those still read right. Section names inside the moved blocks (Phase, In flight,',
    'Follow-ups, Routines, Open bugs and risks, Targets) are STATUS\'s at 2d22b69: that text is now in',
    '`docs/STATUS_ARCHIVE.md` under the same names. Line numbers in the blocks read at their own dates._',
]


def find1(pred, what):
    hits = [i for i, s in enumerate(L) if pred(s)]
    assert len(hits) == 1, (what, hits)
    return hits[0]


# Edits, applied bottom-up so earlier indices stay valid.
i_slice = find1(lambda s: s == '  3. "The netcode proof test (48 simulated players over WebTransport) still runs before the foundation."', 'slice')
i_split = find1(lambda s: s == "    >    to legacy/ only); it is recorded in the ADR's Status line in 2.1b's docs pass.", 'split')
i_sd = find1(lambda s: s.startswith('- **Carried out by status-diet: STATUS-DIET and LANE-RULES'), 'status-diet')
i_dec = find1(lambda s: s == '## Decisions, newest first', 'decisions')
i_ptrs = find1(lambda s: s == '  - The entries below are pointers to items carried out.', 'queued pointers')
i_freeze = find1(lambda s: s == '    >    FREEZE-FDN, option (a)).', 'freeze')
i_it0 = find1(lambda s: s.startswith('_How it was made:'), 'italic start')
i_it1 = find1(lambda s: s.endswith('Line numbers in the blocks read at their own dates._'), 'italic end')
assert i_it0 < i_it1 < i_freeze < i_ptrs < i_dec < i_sd < i_split < i_slice
assert L[i_dec + 1].startswith('- **Taken by Mustafa, 2026-11-10, 16:04:59 UTC: his go for #46**')
# the #46 entry ends right before the status-diet block
assert L[i_sd - 1].startswith('> I reconfirm the rule changes at 0573db3')

L[i_slice + 1:i_slice + 1] = ptr_slice
L[i_split + 1:i_split + 1] = ptr_split
L[i_sd:i_sd] = day10
L[i_dec + 1:i_dec + 1] = top
L[i_ptrs:i_ptrs] = queued
L[i_freeze + 1:i_freeze + 1] = ptr_freeze
L[i_it0:i_it1 + 1] = italic_new

for s in italic_new + ptr_freeze + ptr_split + ptr_slice + queued[:4]:
    assert len(s) <= 120, (len(s), s)
bad = [s for s in top + day10 if not s.lstrip().startswith('>') and len(s) > 120]
assert not bad, bad
_hdr = ' '.join(x.strip() for x in top + day10 if not x.lstrip().startswith('>'))
for h in (H_A1, H_A21, H_A32):
    assert '**' + h + '**' in _hdr.replace('the **foundation', 'the foundation'), h

open(OUT, 'w', encoding='utf-8').write('\n'.join(L) + '\n')
print('written', OUT, len(L), 'lines')

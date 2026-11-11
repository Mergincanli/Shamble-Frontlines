# Writes Appendix A of docs/reports/2026-11-11-foundation-2-1a.md: Mustafa's words for foundation 2.1a in full, each
# block copied byte for byte from its source by this script, never retyped, then checked against that source.
# Run from the repo root: python3 -I .checkpoints/foundation-restructure/report-appendix.py
# Sources: the notes files beside this script and the merge commit bb2d817's message (git show -s --format=%B).
import pathlib, re, subprocess, sys

ROOT = pathlib.Path.cwd()
NOTES = ROOT / '.checkpoints/foundation-restructure'
REPORT = ROOT / 'docs/reports/2026-11-11-foundation-2-1a.md'
MARK = '## Appendix A.'

def fenced(name):
    return re.findall(r'```text\n(.*?)\n```', (NOTES / name).read_text(), re.S)

d10 = fenced('mustafa-2026-11-10.md')            # decision, follow-up, task
a10 = fenced('mustafa-2026-11-10-answers.md')    # answers, follow-up, #47's turn
s11 = fenced('mustafa-2026-11-11.md')            # prompt, BRANCH-47 + advisor tool
f11 = fenced('mustafa-2026-11-11-f1.md')         # F1-FIX, FREEZE-FDN, his yes to the exact gate change
p11 = fenced('mustafa-2026-11-11-prompts.md')    # his words on permission prompts
c11 = fenced('mustafa-2026-11-11-prompts-correction.md')  # his correction to them
assert (len(d10), len(a10), len(s11), len(f11), len(p11), len(c11)) == (3, 3, 2, 1, 1, 1), (len(d10), len(a10), len(s11), len(f11), len(p11), len(c11))
msg = subprocess.run(['git', 'show', '-s', '--format=%B', 'bb2d817'], capture_output=True, text=True, check=True).stdout
q = [l for l in msg.split('\n') if l.startswith('> ') or l == '>']
go49 = '\n'.join(l[2:] if l.startswith('> ') else '' for l in q)
assert go49.startswith('1. Kit-edit prompts') and go49.endswith('The next change-set records it.'), go49[:60]

BLOCKS = [
    ('A1.1', "His decision of 2026-11-10 (the foundation chat, session_019YNMUVo8MVM3gdiMNaN5xs; quoted in his prompt there as \"Mustafa's decision of 2026-11-10, word for word\"; saved by that chat in `mustafa-2026-11-10.md`, not re-checked against its chat record here)", d10[0]),
    ('A1.2', 'His follow-up in the same chat, 2026-11-10, after its first turn stopped on the model rule (same source)', d10[1]),
    ('A1.3', 'The task as he gave it there, his numbered list (same source)', d10[2]),
    ('A2.1', 'His answers of 2026-11-10 in the foundation chat, after the plan\'s report (plan revision 3 at d389d91): FREEZE-FDN, SPLIT, SCOPE, the build go, the merge order, both accounts (saved by that chat in `mustafa-2026-11-10-answers.md`, not re-checked against its chat record here)', a10[0]),
    ('A2.2', 'His follow-up of 2026-11-10 there, after 2.1a\'s report (PR #47): the two gate prompts approved, #47 not watched, step 2.2 planned in a fresh session (same source)', a10[1]),
    ('A2.3', 'His message of 2026-11-11 there, 09:46:31 UTC (#47\'s turn; byte-identical to the chat record, event c49a5660-c1d0-4c58-a54c-983fe54c4394, 848 characters, checked on 2026-11-11 by this change-set)', a10[2]),
    ('A3.1', 'His prompt in this session (session_019YNMUWLffMoE47SbvkqJ5P), 2026-11-11 10:01:35 UTC (first sent at 09:59:00 UTC under Fable 5.2, the same text): the foundation chat\'s resume card as he pasted it, plus his two additions at the end, "my words of 2026-11-11" (extracted by script from the session\'s events: `mustafa-2026-11-11.md`)', s11[0]),
    ('A3.2', 'His message in this session, 2026-11-11 11:56:44 UTC: BRANCH-47 (a) and his words on the advisor tool (same source)', s11[1]),
    ('A3.3', 'His message in this session, 2026-11-11 14:54:11 UTC: F1-FIX (a), FREEZE-FDN (a) for one check, and his yes to the exact change to `tools/gate.mjs` (extracted by script from the session\'s events, event 8e7ad9dc-cdf8-43a7-817b-ceaaba67081e: `mustafa-2026-11-11-f1.md`)', f11[0]),
    ('A3.4', 'His message in this session, 2026-11-11 16:32:18 UTC: his words on permission prompts (extracted by script from the session\'s events, event 557d0202-6418-4c26-9d03-9b1ee13fd5bf: `mustafa-2026-11-11-prompts.md`)', p11[0]),
    ('A3.5', 'His message in this session, 2026-11-11 16:54:54 UTC: a correction to his words on permission prompts (A3.4) (extracted by script from the session\'s events, event a10ee1d6-3120-4eb1-b83a-4e84afe50a94: `mustafa-2026-11-11-prompts-correction.md`)', c11[0]),
    ('A4', 'His go for #49, 2026-11-11, about 09:02 UTC, as the merge commit bb2d817\'s message records it (quoted there with "> "; the prefix is removed here). Not checked against the chat record: that day session (session_0173vVo6Vtm2C54toWvTt8sR) is not readable from this account', go49),
]
for _, _, t in BLOCKS:
    assert '```' not in t

parts = [MARK + " Mustafa's words for this change-set, in full\n",
         '_Each block below is copied byte for byte from its source by `report-appendix.py` (in the notes folder, readable '
         'at this branch\'s last checkpoint commit), never retyped; the same script checks every block against its source. '
         'Pointers in `docs/DECISIONS.md` lead here, because the notes folder is deleted before the PR is marked ready._\n']
for tag, head, text in BLOCKS:
    parts.append(f'### {tag}. {head}\n\n```text\n{text}\n```\n')
appendix = '\n'.join(parts)

old = REPORT.read_text() if REPORT.exists() else ''
top = old.split(MARK, 1)[0] if MARK in old else (
    '# Foundation 2.1a: the legacy gate and tools read both layouts (PR #50, replacing #47)\n\n'
    '_The report is written at the change-set\'s last step; Appendix A is final._\n\n')
REPORT.write_text(top + appendix)

# The check: every block in the written file equals its source, byte for byte.
written = re.findall(r'```text\n(.*?)\n```', REPORT.read_text().split(MARK, 1)[1], re.S)
assert written == [t for _, _, t in BLOCKS], 'a block differs from its source'
print(f'Appendix A: {len(written)} blocks written and checked against their sources: all byte-identical')

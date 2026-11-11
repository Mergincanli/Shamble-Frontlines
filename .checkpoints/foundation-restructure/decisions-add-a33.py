# Inserts Mustafa's message of 2026-11-11 14:54:11 UTC (the report's Appendix A3.3) at the top of docs/DECISIONS.md's
# "Decisions, newest first", quoted line by line from the appendix (never retyped), under a header written for this file.
# Idempotent: does nothing if the entry is already there. Run from the repo root: python3 -I <this script>
import pathlib, re

REPORT = pathlib.Path('docs/reports/2026-11-11-foundation-2-1a.md')
DEC = pathlib.Path('docs/DECISIONS.md')
appendix = REPORT.read_text().split('## Appendix A.', 1)[1]
m = re.search(r'^### A3\.3\. .*?\n\n```text\n(.*?)\n```\n', appendix, re.S | re.M)
assert m, 'Appendix A3.3 not found'
block = m.group(1).split('\n')
assert block[0] == 'My words of 2026-11-11, for the foundation chat (PR #50, foundation step 2.1a).', block[0]

HEADER = """- **Taken by Mustafa, 2026-11-11, 14:54:11 UTC: F1-FIX (a), FREEZE-FDN (a) for one check, and his yes to the exact gate
  change** (foundation 2.1a, PR #50; his message in that change-set's session, session_019YNMUWLffMoE47SbvkqJ5P, on his
  personal account, extracted by script from the session's events, event 8e7ad9dc-cdf8-43a7-817b-ceaaba67081e). It
  answers the session's ask on the security review's F1 (High). Carried out: the change was written as 3f29cb8
  (`tools/gate.mjs` blob 7a8480c, as the patch produces from blob 2bfab53); his FREEZE-FDN (a) reading covers this one
  check only; the smoke.mjs wording Low is a follow-up for 2.3. Copied by script from
  `docs/reports/2026-11-11-foundation-2-1a.md`, Appendix A3.3, and checked byte for byte against it:"""
quote = '\n'.join(('> ' + l) if l else '>' for l in block)

text = DEC.read_text()
if HEADER.split('\n')[0] in text:
    print('already there: nothing to do')
else:
    anchor = '## Decisions, newest first\n'
    assert text.count(anchor) == 1
    text = text.replace(anchor, anchor + HEADER + '\n' + quote + '\n', 1)
    DEC.write_text(text)
    print('inserted: header', len(HEADER.split('\n')), 'lines, quote', len(block), 'lines')

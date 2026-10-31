"""Builds docs/reports/<date>-v2-research-appendix.md: the records the report cites, word for word (plan-amendments-2
condition 12: "The 12 resolutions and the advisor's per-item verdicts go into the report word for word"), the coverage
table (plan condition 12), the fetch audit (VERIFY-FETCH (a)) and the check scripts' outputs. Usage:
python3 mk_appendix.py <date> <head commit>"""
import subprocess, sys

DATE, HEAD = sys.argv[1], sys.argv[2]
C = '/home/user/Shamble-Frontlines/.checkpoints/v2-research/'
S = C + 'scripts/'
OUT = f'/home/user/Shamble-Frontlines/docs/reports/{DATE}-v2-research-appendix.md'


def run(*args):
    return subprocess.run(['python3'] + list(args), capture_output=True, text=True, check=True).stdout


def verbatim(path, title):
    body = open(C + path).read().strip()
    # demote the record's own headings by two levels so they sit under this appendix's sections
    body = '\n'.join(('##' + l) if l.startswith('#') else l for l in body.split('\n'))
    return f'\n### {title}\n\nFrom `.checkpoints/v2-research/{path}` at commit {HEAD} (that folder is deleted in this PR\'s last commit; the history keeps it).\n\n{body}\n'


parts = [f"""# Appendix to the v2-research report ({DATE})

The records the report (`docs/reports/{DATE}-v2-research.md`) cites, word for word where the plan asks for it, and the
check scripts' outputs. The working notes they come from are in `.checkpoints/v2-research/` at commit {HEAD}.

## 1. Coverage: every brief §4 question (plan condition 12)

Made by `scripts/coverage_table.py` from each topic file's own question table at {HEAD}. "Lead recommendation" is the
heading of the question's first recommendation, shortened; R2's question 6 is answered across its five units (an
index under its Findings); R6's question 5, the security baseline, was deferred by the Advisor on 2026-10-27
(R6-SEC-OQ1; when it is researched is BASELINE-WHEN).

""" + run(S + 'coverage_table.py', HEAD)]

parts.append('\n## 2. The 12 contradictions: the resolutions as approved, and the advisor\'s rulings (word for word)\n')
parts.append(verbatim('resolutions.md', 'The resolutions, as folded after the advisor\'s step-3b verdict'))
parts.append(verbatim('advisor-resolutions-verdict.md', 'The advisor\'s step-3b verdict on the resolutions, the glossary and the cross-topic questions'))
parts.append(verbatim('advisor-retire-split-verdict.md', 'The advisor\'s ruling on the glossary\'s Retire split'))
parts.append(verbatim('advisor-object-1-verdict.md', 'The advisor\'s ruling on step 3c\'s one OBJECT'))
parts.append(verbatim('advisor-3d-r3-verdict.md', 'The advisor\'s ruling on the two R3 items of step 3d'))

parts.append('\n## 3. The fetch audit (VERIFY-FETCH (a))\n\nEvery web fetch in this change-set\'s agent transcripts and the '
             'session\'s own, by agent, tool and host, from `scripts/fetch_audit.py --grouped` (a WebFetch "redirect notice" '
             'row is a redirect the tool reported and did not follow; "(from a file or variable)" is a curl whose address was '
             'in a variable or a URL file).\n\n```\n' + run(S + 'fetch_audit.py') + '```\n')
run(S + 'fetch_audit.py', '--grouped', '/tmp/claude-0/-home-user-Shamble-Frontlines/ed3ff506-9a45-5a14-adc3-75c262311890/scratchpad/sp3/fetch-grouped.md')
parts.append(open('/tmp/claude-0/-home-user-Shamble-Frontlines/ed3ff506-9a45-5a14-adc3-75c262311890/scratchpad/sp3/fetch-grouped.md').read())

parts.append('\n## 4. The check scripts\' outputs at ' + HEAD + '\n')
parts.append('\n### The glossary check (condition 9): `glossary_check.py --skip-records`\n\n```\n' +
             run(S + 'glossary_check.py', '--skip-records') + '```\n')
parts.append('\n### The approved changes found in their files: `apply_check.py`\n\n```\n' +
             run(S + 'apply_check.py', HEAD).split('\n')[-2] + '\n```\n\nThe items to read by hand are the ones whose after '
             'text a later glossary rename, link or ruling changed; the step-3d verifiers read each (their reports, '
             '`verifier-*-3d-result.md`).\n')
parts.append('\n### Quoted source text: `quotes_check.py 7c6e0d7 ' + HEAD + '`\n\n```\n' +
             run(S + 'quotes_check.py', '7c6e0d7', HEAD) + '```\n')
parts.append('\n### UNREAD_SOURCES.md: `build_unread.py`\'s check\n\n```\n' + open(S + 'unread-check.json').read() + '```\n')
open(OUT, 'w').write(''.join(parts))
print('wrote', OUT, sum(p.count('\n') for p in parts), 'lines')

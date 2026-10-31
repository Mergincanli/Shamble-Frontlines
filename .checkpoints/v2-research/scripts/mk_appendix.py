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
parts.append("""
## 5. The steps with their commits (the checkpoint, folded in; CHECKPOINTS §9)

| Step | What | Commits |
|---|---|---|
| Start | Branch from main 07ab240 after R7 (PR #27); plan and verdicts recovered word for word | 8ff1c74 |
| Wave 1 | R1 and R2: 11 units, assembled; six R1 verifiers, the R2 verifier, rust-reviewer, security-auditor; fix pass; second verifier pass; second fix round and final check: both verified | a6a97fe, 7907f5c, e4cda31, ae492a6 |
| Wave 2 | R3–R6 owners; a spot-check, an owner fix and a second pass each | R3 605c09d → c22675f; R4 abdf477 → 16212b6; R5 9a623a0 → 8948522; R6 9d5e78a → 1decc70 |
| Main merged | PR #29's STATUS and ROADMAP brought in before the docs step | 569f792 |
| Critic | The consistency critic: 41 items (12 contradictions) | in `.checkpoints/` (engine-architect-consistency-result.json) |
| The Advisor's 2026-10-30 decisions | UNREAD_SOURCES.md; the glossary; the 12 contradictions (steps 3a–3d) | 82298ab |
| Step 3a/3b | Glossary and resolutions drafted; the advisor's verdict folded word for word | e797518, 8909523 |
| Retire split | Format-only split, ruled by the advisor | 7c6e0d7 |
| Step 3c | Owners apply, one per file; OBJECT-1 ruled and applied; the glossary's Used today refresh | 1da4e5f, de0a003, 41f9cf9, 2684735, e9ff1f9, 09de883, 36cc386, f67245e |
| Step 3d | Three verifier units with skeptics; fix rounds; three final checks; the completeness critic and its nits | d768190, 10662c4, 1b69364, d54f3c0, 902c249, 1bfd235, f2239f9, 8260592, 3aad9d8, 96a4c29, 2382ea2, 7bb3d2e, b92199c, 749b363, 910768b |
| Docs | docs-keeper (STATUS, ROADMAP, README, REPO_MAP); the report; the gate | see the PR's last commits |

Spend-limit cuts (all resumed from the agents' notes, nothing lost): 2026-10-27 about 18:25 UTC; 2026-10-28 about 16:26
and 20:50 UTC; 2026-10-30 about 18:52 and 21:04 UTC.
""")
open(OUT, 'w').write(''.join(parts))
print('wrote', OUT, sum(p.count('\n') for p in parts), 'lines')

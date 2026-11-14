# Plan revision 2.1: folds the six Lows of the advisor agent's APPROVE of revision 2 (00984ae), which it allowed
# without a further review. Each replacement must match exactly once; Appendix K is replaced by the regenerated
# kit-texts.md. Usage: python3 -I rev21.py (after kit-texts.py has been edited and re-run)
import sys
BASE = '/home/user/Shamble-Frontlines/.checkpoints/foundation-2-3-plan/'
P = BASE + 'plan.md'
s = open(P, encoding='utf-8').read()

def rep(a, b):
    global s
    n = s.count(a)
    if n != 1:
        sys.exit(f'{n} matches for: {a[:90]!r}')
    s = s.replace(a, b)

rep('(revision 2)\n', '(revision 2.1)\n')
rep('  and its four rulings). Part 11 lists what changed.\n',
    '  and its four rulings). Revision 2.1 folds in the six Lows of its APPROVE of revision 2 (00984ae), which it allowed\n'
    '  without a further review. Part 11 lists what changed.\n')
# L-1
rep('met in full at slice 5, with PM-0002-52 |',
    'met in full at the later of slice 5\'s change-set and PM-0003-36\'s, with PM-0002-52 |')
rep('| slice 5\'s change-set: the last mutations of PM-0002-39 to -51 (the editor\'s, in -42 and -46) join there; one mutation and one control per id G0-G12 from 2.3-E |',
    '| the later of slice 5\'s change-set (the editor\'s mutations, in -42 and -46) and PM-0003-36\'s (the draw-call mutation of -51), where the last mutations of PM-0002-39 to -51 join; one mutation and one control per id G0-G12 from 2.3-E |')
# L-2
rep('  pull_request, read by the session, as his gos for #48 and #52 asked.\n',
    '  pull_request, read by the session, as his gos for #48 and #52 asked. CS-E\'s own pull request removes\n'
    '  `toolchain.yml`, so CS-E reads its replacement jobs (`rust`, `typescript` and `required`) by hand instead.\n')
# L-3
rep('   - Code (CS-C1, CS-C, CS-D\'s guard, CS-E): developed and tested in a plain copy of the tree made with `git\n'
    '     archive`, never a worktree of this repository and never committed. For builds the copy sits under\n'
    '     `.gate/<change-set>/` (CHECKPOINTS §4); for the guard\'s own files it sits in the session\'s scratchpad, outside\n'
    '     the project, so no step leans on the guard\'s nested-folder gap (W1). The 2.1a precedent: `.gate/fdn-scratch`,\n'
    '     and F1-FIX\'s patch.\n',
    '   - Code (CS-C1, CS-C, CS-D\'s guard, CS-E): developed and tested in a plain copy of only the paths the change-set\n'
    '     needs, made with `git archive`: never `.claude/` or a `CLAUDE.md`, so no step leans on the guard\'s nested-folder\n'
    '     gap (W1); never a worktree of this repository; never committed. The copy sits under `.gate/<change-set>/`\n'
    '     (CHECKPOINTS §4); the guard\'s own files go to the session\'s scratchpad, outside the project. The 2.1a\n'
    '     precedent: `.gate/fdn-scratch`, and F1-FIX\'s patch.\n')
rep('     save. Before his yes there is at most one guard prompt per change-set, for making the copy.\n',
    '     save. Before his yes no kit path is written, so no guard prompt is expected.\n')
rep('- About 45 guard prompts by PIN-SCOPE (b): 4 + 2 + 4 + 21 + 7 + 7, and at most one more per change-set for making\n'
    '  its scratch copy (5.1).\n',
    '- About 45 guard prompts by PIN-SCOPE (b): 4 + 2 + 4 + 21 + 7 + 7.\n')
rep('   - **Guard prompts:** answer each, about 4, 2, 4, 21, 7 and 7, and at most one per change-set for its scratch copy.\n',
    '   - **Guard prompts:** answer each, about 4, 2, 4, 21, 7 and 7.\n')
# L-5
rep('Why this order: every merge needs `gate` and `smoke` green from both the ruleset and the guard, throughout. From step 1\n',
    'Why this order: every merge needs `gate` and `smoke` green from both the ruleset and the guard, throughout: directly,\n'
    'or through `required`. From step 1\n')
rep('   8\'s order, `gate` and `smoke` stay required by both the ruleset and the guard at every step, the ruleset is at\n',
    '   8\'s order, `gate` and `smoke` stay required by both the ruleset and the guard at every step, directly or through\n'
    '   `required`, the ruleset is at\n')
# L-6
rep('CS-E: each job writes its version lines and the aggregate compares them with the pins;',
    'CS-E: each v2 job writes its version lines and the aggregate compares them with the pins (the legacy jobs stay as moved);')
rep('  - PM-0002-38: each job writes its tools\' version lines for the aggregate to compare with the pins; the same\n',
    '  - PM-0002-38: each v2 job writes its tools\' version lines for the aggregate to compare with the pins; the same\n')
# Part 11 log
rep('  - Decisions: GATE-PATH (a)\'s import cost; PIN-SCOPE (b)\'s unprotected pins; MODELS-HOW quotes DECISIONS.md :91.\n',
    '  - Decisions: GATE-PATH (a)\'s import cost; PIN-SCOPE (b)\'s unprotected pins; MODELS-HOW quotes DECISIONS.md :91.\n'
    '- **Revision 2.1** (the advisor agent\'s APPROVE of revision 2 at 00984ae, with six Lows it allowed to be folded\n'
    '  without a further review):\n'
    '  - L-1: D1 and PM-0002-52 are met at the later of slice 5\'s change-set and PM-0003-36\'s.\n'
    '  - L-2: CS-E reads its replacement jobs by hand, since it removes `toolchain.yml`.\n'
    '  - L-3: the scratch copy holds only the paths a change-set needs, never `.claude/` or a `CLAUDE.md`; no prompt is\n'
    '    expected for it.\n'
    '  - L-4: K-4 (a) no longer cites an undefined K-5, and its [to verify] covers an effort passed by the caller.\n'
    '  - L-5: "directly, or through `required`" in Part 8\'s and Part 10\'s order sentences.\n'
    '  - L-6: PM-0002-38\'s version lines come from the v2 jobs; the legacy jobs stay as moved.\n')

i = s.index('\n## Appendix K. The exact kit texts')
head_end = s.index('in its change-set (5.1).', i) + len('in its change-set (5.1).')
head = s[i:head_end]
k = open(BASE + 'kit-texts.md', encoding='utf-8').read().split('\n')
assert k[0].startswith('# Kit texts for the step-2.3 plan')
body = [('### ' + l[3:]) if l.startswith('## K-') else l for l in k[1:]]
s = s[:i] + head + '\n'.join(body)
open(P, 'w', encoding='utf-8').write(s)
print('revision 2.1 written')

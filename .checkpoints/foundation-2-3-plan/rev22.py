# Plan revision 2.2: the advisor tool's moment-5 check (2026-11-14 12:11:15 UTC) found that the old plan's Appendix B
# (26e0d82, .checkpoints/foundation-restructure/plan.md:636-655: the rules kept while PR #40's spike is in the tree)
# was never carried. This folds it in, re-reads #40's head, adds the legacy rules' globs as a waiting item, and gives K-4
# a positive line for every other main session. Each replacement must match exactly once. Usage: python3 -I rev22.py
import re, sys
BASE = '/home/user/Shamble-Frontlines/.checkpoints/foundation-2-3-plan/'
P = BASE + 'plan.md'
s = open(P, encoding='utf-8').read()

def rep(a, b):
    global s
    n = s.count(a)
    if n != 1:
        sys.exit(f'{n} matches for: {a[:90]!r}')
    s = s.replace(a, b)

rep('(revision 2.1)\n', '(revision 2.2)\n')
rep('  without a further review. Part 11 lists what changed.\n',
    '  without a further review. Revision 2.2 carries the old plan\'s Appendix B (the rules kept while PR #40\'s spike is in\n'
    '  the tree), found missing by the advisor tool\'s moment-5 check. Part 11 lists what changed.\n')
# --- summary: #40
rep('- **PR #40** has nothing to do now. It merges main before its own merge, and before his ruleset\'s `required` matters\n'
    '  to it (Part 8).\n',
    '- **PR #40** has nothing to do now. It merges main before its own merge, and before his ruleset\'s `required` matters\n'
    '  to it. Every change-set keeps the rules 2.2 kept while its spike is in the tree (the old plan\'s Appendix B), and\n'
    '  checks them with the spike\'s own check (Parts 3 and 8).\n')
# --- Part 0: #40's head
rep('    - the netcode proof test (PR #40: a draft, head 4bd5ac3, 2026-11-14 08:31:05 UTC, in its U4r fix pass, behind and\n'
    '      conflicting with main);\n',
    '    - the netcode proof test (PR #40: a draft, head 30b7b83, GitHub\'s `updated_at` 2026-11-14 11:18:50 UTC, read again\n'
    '      at 12:14:41 UTC; first read at 10:54:55 UTC at head 4bd5ac3, in its U4r fix pass; behind and conflicting with\n'
    '      main);\n')
# --- Part 3: the Appendix B rule block
rep('- **`toolchain` read by hand until `required` counts.**',
    '- **While PR #40\'s spike is in the tree** (the old plan\'s Appendix B, 26e0d82 `plan.md:636-655`, which 2.2 kept):\n'
    '  - no tracked file outside `spikes/`, `docs/` and `.checkpoints/` that is not Markdown may contain the text of the\n'
    '    spike folder\'s path: the spike\'s `check.mjs` (`referenceProblems`) fails on it. Configs name what they cover,\n'
    '    never what they skip; no test row, fixture, workflow or config names the spike folder;\n'
    '  - no root `.cargo/config.toml`: target folders and flags go on the command line;\n'
    '  - no `"type"` in the root `package.json`;\n'
    '  - each change-set that adds or changes such a file runs `node spikes/netcode-proof/check.mjs --only deps,refs` on\n'
    '    a local merge with #40\'s head, never pushed, as 2.2\'s E4d did.\n'
    '- **`toolchain` read by hand until `required` counts.**')
# --- CS-C1: F4 names what it covers; the spike check in its evidence
rep('  - The gate on a local merge with PR #40\'s head, never pushed: no refusal for `spikes/`.\n',
    '  - The gate on a local merge with PR #40\'s head, never pushed: no refusal for its pages, and its `check.mjs --only\n'
    '    deps,refs` passing: F4 names the folders it checks, never the spike folder.\n')
# --- CS-D: the guard's test rows never name the spike's path
rep('    - PR #40\'s `spikes/...` manifests stay free;\n',
    '    - a nested `Cargo.toml` under another folder stays free. No row names the spike\'s path (Appendix B); #40\'s real\n'
    '      manifests are checked in a scratch run, and its `check.mjs --only deps,refs` passes on a local merge;\n')
# --- CS-C: the import rule, .prettierignore, exclude, config, type
rep('- **Tools run by their path, never through `npm run`, `npm test` or `npx`.**',
    '- **Appendix B in CS-C\'s files:**\n'
    '  - G3\'s import rule says what it allows: imports stay inside their package, or name a workspace package or a\n'
    '    dependency. It names no folder it forbids. The mutation for PM-0002-42\'s "an import from `legacy/` or ..."\n'
    '    plants an import from `legacy/`; the variant naming the spike folder joins after PR #40 merges;\n'
    '  - Prettier is pointed at `packages/**`; a `.prettierignore`, if any, names no spike path;\n'
    '  - ROOT-WORKSPACE\'s `exclude` lists `.gate` and `.checkpoints` only;\n'
    '  - no root `.cargo/config.toml`, and no `"type"` added to the root `package.json`;\n'
    '  - the evidence runs #40\'s `check.mjs --only deps,refs` on a local merge with its head.\n'
    '- **Tools run by their path, never through `npm run`, `npm test` or `npx`.**')
# --- CS-E: the workflow names no spike path
rep('  - the legacy jobs diffed against `gate.yml`\'s: equal but for the named fixes.\n',
    '  - the legacy jobs diffed against `gate.yml`\'s: equal but for the named fixes;\n'
    '  - the workflow names no spike path (Appendix B), and #40\'s `check.mjs --only deps,refs` passes on a local merge\n'
    '    with its head.\n')
# --- Part 2 PM-0002-42: the spike-folder variant
m = [l for l in s.split('\n') if l.startswith('| PM-0002-42 (G3) |')]
assert len(m) == 1
rep(m[0], m[0].replace('| generated union (2.3-B); the editor clauses (slice 5) |',
                       '| generated union (2.3-B); the variant of the legacy-or-spike import mutation that names the spike folder (after PR #40 merges, when Appendix B lifts); the editor clauses (slice 5) |'))
# --- Part 4: the legacy rules' globs
m = [l for l in s.split('\n') if l.startswith('| 107 |')]
assert len(m) == 1
rep(m[0], m[0] + '\n| 108 | the legacy rules\' globs (`.claude/rules/pages.md:3-4`: `**/index.html`, `**/editor.html`) and CLAUDE.md:46\'s bare `index.html` will also match a v2 page | old plan Appendix B (26e0d82 :648-651); still so on main 4460ef5 | [later] step 2.5, which brings the first v2 page: a kit edit narrowing them to `legacy/`, asked then |')
# --- Part 6 G-1: the test row
rep('  - **A pattern matching any depth** would catch PR #40\'s `spikes/netcode-proof/Cargo.toml` and stall that chat. Root\n'
    '    anchoring and a test row prevent it.\n',
    '  - **A pattern matching any depth** would catch PR #40\'s own `Cargo.toml` inside the spike folder and stall that chat.\n'
    '    Root anchoring and a test row on a nested `Cargo.toml` under another folder prevent it. No row names the spike\'s\n'
    '    path (Appendix B).\n')
# --- Part 8: facts and rows
rep('- #40 is a draft, head 4bd5ac3. GitHub lists its base as 4d21c1a, and it conflicts with main 4460ef5.\n',
    '- #40 is a draft, head 30b7b83 (read 2026-11-14 12:14:41 UTC). GitHub lists its base as 4d21c1a, and it conflicts with\n'
    '  main 4460ef5.\n'
    '- Its `check.mjs` (`referenceProblems`) reads every tracked file outside the spike folder, `docs/` and `.checkpoints/`\n'
    '  that is not Markdown, and fails on the text of the spike folder\'s path. So each change-set below keeps the old\n'
    '  plan\'s Appendix B (Part 3), and runs that check on a local merge with #40\'s head.\n')
rep('| CS-C1 | nothing. Its gate command `node tools/gate.mjs --no-smoke` stays. The stray-page check looks at the root and the old folder only; CS-C1\'s evidence runs on a local merge with its head |',
    '| CS-C1 | nothing, while Appendix B holds. Its gate command `node tools/gate.mjs --no-smoke` stays. The stray-page check looks at the root and the old folder only; CS-C1\'s evidence runs the gate and its `check.mjs` on a local merge with its head |')
rep('| CS-D | nothing to change. After its next main merge it runs the new guard:',
    '| CS-D | nothing to change, while Appendix B holds (no test row names its folder). After its next main merge it runs the new guard:')
rep('| CS-C | nothing. The gate v2 ignores `spikes/`: the spikes keep their own workspace, G1 and G3 read `packages/**`, and G2 and G5 read the root workspace. CS-C\'s evidence runs the gate v2 on a local merge with its head |',
    '| CS-C | nothing, while Appendix B holds. The gate v2 ignores the spike: it keeps its own workspace, G1 and G3 read `packages/**`, and G2 and G5 read the root workspace. CS-C\'s evidence runs the gate v2 and its `check.mjs` on a local merge with its head |')
# --- Part 10: a risk
rep('10. **Readings by agents.**',
    '11. **#40\'s own check.** A root file naming the spike folder would fail #40\'s `check.mjs` after its next main merge.\n'
    '    Part 3\'s Appendix B rule, and that check run on a local merge in each change-set, stand against it.\n'
    '10. **Readings by agents.**')
# --- Part 11 log
rep('  - L-6: PM-0002-38\'s version lines come from the v2 jobs; the legacy jobs stay as moved.\n',
    '  - L-6: PM-0002-38\'s version lines come from the v2 jobs; the legacy jobs stay as moved.\n'
    '- **Revision 2.2** (the advisor tool\'s moment-5 check, 2026-11-14 12:11:15 UTC, the transcript):\n'
    '  - the old plan\'s Appendix B carried: a rule block in Part 3; F4 (CS-C1), the guard\'s test rows (CS-D), G3\'s\n'
    '    import rule, Prettier, `exclude`, `.cargo`, `"type"` (CS-C) and the workflow (CS-E) name no spike path; each runs\n'
    '    #40\'s `check.mjs --only deps,refs` on a local merge; PM-0002-42\'s spike-folder variant waits for #40\'s merge;\n'
    '    Part 8 and Part 10 say so;\n'
    '  - #40\'s head re-read: 30b7b83 (12:14:41 UTC); main re-fetched: still 4460ef5;\n'
    '  - Part 4 row 108: the legacy rules\' globs, for step 2.5;\n'
    '  - K-4\'s "How" says positively that every other main session runs on Opus 5.5 (the advisor tool\'s note for CS-B).\n')

# --- Appendix K: replace with the regenerated kit texts
i = s.index('\n## Appendix K. The exact kit texts')
head_end = s.index('in its change-set (5.1).', i) + len('in its change-set (5.1).')
head = s[i:head_end]
k = open(BASE + 'kit-texts.md', encoding='utf-8').read().split('\n')
assert k[0].startswith('# Kit texts for the step-2.3 plan')
body = [('### ' + l[3:]) if l.startswith('## K-') else l for l in k[1:]]
s = s[:i] + head + '\n'.join(body)

# --- recount Part 4 and fix the summary's counts
part4 = s[s.index('## Part 4.'):s.index('## Part 5.')]
rows = [l for l in part4.split('\n') if re.match(r'\| \d+ \|', l)]
tags = {}
for r in rows:
    t = re.search(r'\| \[(1st|later|dropped|not ours|chat)\]', r); t = t.group(1) if t else 'NONE'
    tags[t] = tags.get(t, 0) + 1
rep('- **The waiting items** (Part 4, 107 rows in one table, one per item and source, counted by script):\n'
    '  - 50 are carried by the first part.\n  - 48 come later, each with its time.\n',
    f'- **The waiting items** (Part 4, {len(rows)} rows in one table, one per item and source, counted by script):\n'
    f'  - {tags["1st"]} are carried by the first part.\n  - {tags["later"]} come later, each with its time.\n')
open(P, 'w', encoding='utf-8').write(s)
print('revision 2.2 written;', len(rows), 'rows;', tags)

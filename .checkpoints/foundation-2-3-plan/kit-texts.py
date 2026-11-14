# The step-2.3 plan's kit texts: every "before" is read from main 4460ef5 by line number and checked against an expected
# opening; every quote of Mustafa's words is copied from docs/DECISIONS.md at 4460ef5 and checked (whitespace-joined)
# against it. Read-only on the repository; writes kit-texts.md beside this script. Usage: python3 -I kit-texts.py
import subprocess, textwrap, re, sys

ROOT = '/home/user/Shamble-Frontlines'
BASE = '4460ef5b8d6dc8566b29c15ded75c6f0c8be386b'
OUT = ROOT + '/.checkpoints/foundation-2-3-plan/kit-texts.md'

def show(path):
    r = subprocess.run(['git', 'show', f'{BASE}:{path}'], cwd=ROOT, capture_output=True, text=True)
    if r.returncode != 0:
        sys.exit(f'git show failed for {path}: {r.stderr}')
    return r.stdout.split('\n')

FILES = {p: show(p) for p in ['CLAUDE.md', '.claude/agents/advisor.md', '.claude/agents/netcode-architect.md',
                             '.claude/CHECKPOINTS.md', 'docs/DECISIONS.md', '.claude/agents/verifier.md',
                             '.claude/skills/gate/SKILL.md']}
DEC = FILES['docs/DECISIONS.md']

def lines(path, a, b, expect):
    got = FILES[path][a - 1:b]
    if not got[0].startswith(expect):
        sys.exit(f'{path}:{a} drifted: {got[0][:80]!r} does not start with {expect!r}')
    return got

def dec_line(prefix):
    hits = [l for l in DEC if l.startswith(prefix)]
    if len(hits) != 1:
        sys.exit(f'DECISIONS.md: {len(hits)} lines start with {prefix[:60]!r}')
    return hits[0]

def sentence(prefix):
    # a sentence of his inside a quoted DECISIONS.md line, from prefix up to and including its first full stop
    for l in DEC:
        i = l.find(prefix)
        if i >= 0:
            j = l.find('.', i + len(prefix))
            return l[i:j + 1]
    sys.exit(f'DECISIONS.md: no line holds {prefix[:60]!r}')

JOIN = lambda s: re.sub(r'\s+', ' ', s).strip()
def unquote(l):
    s = l.lstrip()
    return s[1:] if s.startswith('>') else s
DEC_JOINED = JOIN(' '.join(unquote(l) for l in DEC))
CHECKS = []

def his(text):
    # every quote of his must appear in DECISIONS.md once whitespace is joined
    CHECKS.append(text)
    if JOIN(text) not in DEC_JOINED:
        sys.exit(f'not his words byte for byte: {text[:80]!r}')
    return text

def wrap(text, first, rest, width=120):
    return textwrap.wrap(text, width=width, initial_indent=first, subsequent_indent=rest,
                         break_long_words=False, break_on_hyphens=False)

def quote_block(bullets, indent):
    out = []
    for b in bullets:
        # keep his list markers; wrap a long bullet with a hanging indent under its text
        m = re.match(r'(\s*(?:- |\d+\. )?)(.*)', b)
        lead, body = m.group(1), m.group(2)
        out += wrap(body, indent + '> ' + lead, indent + '> ' + ' ' * len(lead))
    return out

S1 = his(sentence('The parts of step 2.3 that need no proof test'))
S2 = his(sentence('Every gate check that needs a result joins later'))
MODELS = [his(dec_line(p)[2:]) for p in [
    '> - On my personal account, Fable 5.2 is the main model of every chat that writes game or engine code',
    '> - Every other agent runs on Opus 5.5 and may run at effort max.',
    '> - The guard, the gate and CI work stays with Opus 5.5.',
    '> - The netcode chat on my university account keeps its models and efforts as they are.']]
ADV_FIRST = dec_line('> My words of 2026-11-11 on the advisor tool. They sharpen')
ADV_SHARPEN = his(ADV_FIRST[ADV_FIRST.index('They sharpen'):])
i0 = DEC.index(ADV_FIRST)
ADV = DEC[i0 + 1:i0 + 13]
assert ADV[-2].startswith('> - The main model and every agent stay Opus 5.5.'), ADV[-2]
assert ADV[-1].startswith('> - Record these words in docs/DECISIONS.md'), ADV[-1]
ADV_KEEP = [his(l[2:]) for l in ADV[:-2]]          # his bullets without the replaced sentence and the record request
LANE_FIRST = his('on my personal account, Fable 5.2 may be the session\'s advisor through Claude Code\'s advisor tool.')
LANE_ADD = his('because the advisor tool\'s advice cannot be read back, it is an extra. The recorded plan review by the '
               'advisor agent stays required.')

K = []
def item(kid, cs, path, a, b, expect, after, note):
    K.append((kid, cs, path, a, b, lines(path, a, b, expect), after, note))

# K-1, K-2, K-3: the three lines that name steps 2.1 and 2.2 as the only exception (his decision of 2026-11-13)
item('K-1', 'CS-B', 'CLAUDE.md', 20, 23, '- **Phase 0**',
     wrap('- **Phase 0** (`docs/V2_PHASE0_BRIEF.md`): process upgrades (kit-v3-0), then research (`docs/research/`), then '
          'decision drafts (`docs/ENGINE_OVERVIEW.md`, `docs/adr/0001`–`0006`). No engine code: throwaway spikes only, '
          'and only after Mustafa and the Advisor have approved the drafts; until then, stop. Exception: foundation '
          'steps 2.1 and 2.2 (Mustafa\'s decision of 2026-11-10 in `docs/DECISIONS.md`), and for step 2.3 his decision '
          f'of 2026-11-13 there: "{S1}"', '', '  '),
     'Looser by his exception, inside its own limits (his sentence quoted whole).')
item('K-2', 'CS-B', 'CLAUDE.md', 54, 57, 'Decisions pending that may change these',
     wrap('Decisions pending that may change these: the v2 decision drafts `docs/adr/0001`–`0006` (V2_PHASE0_BRIEF §4; '
          'they replace ADR-001..003). Phase 0 writes no v2 code, except foundation steps 2.1 and 2.2 (Mustafa\'s '
          'decision of 2026-11-10 in `docs/DECISIONS.md`) and, for step 2.3, what his decision of 2026-11-13 there '
          f'allows: "{S1}" Constraints for v2 code come only from those drafts once Mustafa accepts them. Until then, the '
          'constraints above stand.', '', ''),
     'Looser by his exception, inside its own limits (his sentence quoted whole).')
item('K-3', 'CS-B', '.claude/agents/advisor.md', 107, 108, '- No engine code.',
     wrap('- No engine code. Throwaway spikes only, and only after Mustafa and you have approved the decision drafts. '
          'Exception: foundation steps 2.1 and 2.2 (Mustafa\'s decision of 2026-11-10 in `docs/DECISIONS.md`), and step '
          f'2.3 within his decision of 2026-11-13 there: "{S1} {S2}"', '', '  '),
     'Looser by his exception; the reviewer\'s line carries both his limits (built only without a result, after his '
     'go; checks that need a result join later with marks unchanged).')

# K-4: CLAUDE.md "Models and effort", bullets 1-4 (Routines unchanged). MODELS-HOW (c) shown: callers pass max until
# PR #40 has merged; the agent files' effort lines follow then (K-5).
models_after = []
models_after += wrap('- **Who runs on which model** (Mustafa\'s words of 2026-11-12, 16:25, and what he means by them; '
                     'word for word in `docs/DECISIONS.md`, the entry of 2026-11-12, 15:35:17 UTC; they replace '
                     'LANE-RULES\' "The main model and every agent stay Opus 5.5"):', '', '  ')
models_after += quote_block(['- ' + m[2:] for m in MODELS], '  ')
models_after += wrap('- **How:** agent files say `model: opus`, which resolves to Opus 5.5 today (`claude-opus-5-5`; an '
                     '`opus` agent reported it on 2026-10-25). When a newer Opus appears, Mustafa decides whether to '
                     'move. Built-in agents (Explore, Plan, general-purpose) get their model passed explicitly: '
                     '`model: "fable"` for the builder agents of a chat on his personal account that writes game or engine '
                     'code, `model: "opus"` for every other. Workflow agents inherit the session\'s model, so a chat on '
                     'Fable 5.2 sets every workflow agent that is not a builder to Opus 5.5 [to verify before this text is '
                     'asked: the Agent tool\'s model value for Fable 5.2, and the workflow API\'s model option].', '', '  ')
models_after += wrap('- **Effort is always stated** (Opus 5.5 defaults to medium): `max` for every agent that reviews, '
                     'verifies or designs game or engine code (his words above); until PR #40, the netcode chat, has '
                     'merged, the caller passes `effort: "max"` to those agents, except in the netcode chat, which keeps its '
                     'efforts; their agent files say `max` after that. Otherwise `high` by default; `xhigh` for architecture '
                     'decisions (ADRs, `docs/ENGINE_OVERVIEW.md`, the synthesis) and netcode. Agent files set it in '
                     'their frontmatter. The caller passes it for everything else: the Agent tool\'s `effort` for '
                     'built-in agents and for a `high` agent working on an ADR or on netcode (`xhigh`); `effort: '
                     '\'high\'`, `\'xhigh\'` or `\'max\'` on every workflow `agent()` call (without it, a workflow agent '
                     'takes the session\'s effort).', '', '  ')
models_after += wrap('- **Sonnet 5.5 only for purely mechanical chores, and only when genuinely necessary:** file moves, '
                     'formatting, link checks, summarizing logs. Opus remains the preference even there. No other '
                     'model, except Fable 5.2 on Mustafa\'s personal account: the main model of a chat that writes game '
                     'or engine code and of its builder agents (above), and the session\'s advisor through Claude Code\'s '
                     'advisor tool (below).', '', '  ')
models_after += wrap('- **The advisor tool** (Mustafa\'s LANE-RULES, 2026-11-10: the first sentence of its Models line, '
                     'whose second sentence his words of 2026-11-12 replace, and his addition, word for word): '
                     f'"{LANE_FIRST}" His addition: "{LANE_ADD}" His words of 2026-11-11 on it (`docs/DECISIONS.md`, the '
                     'entry of 2026-11-11, 11:56:44 UTC; his bullets on the replaced model sentence and on recording '
                     'them are left out), word for word:', '', '  ')
models_after += quote_block([ADV_SHARPEN] + ADV_KEEP, '  ')
models_after += ['  [OPTIONAL A, not his words (the 2.3-docs report, finding 5): "Read which model answers it from the '
                 'session\'s records', '  (the transcript\'s advisor-tool entries), not from the `/advisor` command\'s '
                 'output."]',
                 '  [OPTIONAL B, wider than his words, which say "From now on in this chat" (2026-11-14, item 2): '
                 '"Consult it before', '  you save a step as done, never after."]']
item('K-4', 'CS-B', 'CLAUDE.md', 170, 185, '- **Opus 5.5 for everything that thinks:**', models_after,
     'Rules text to match his words of 2026-11-12 and 2026-11-11; nothing looser than his words (the optional lines are '
     'marked and his to refuse).')

# K-4 (a) and K-4 (b): the effort bullet under MODELS-HOW's other options (bullets 1, 2, 4 and 5 as K-4)
EFFORT_TAIL = ('Otherwise `high` by default; `xhigh` for architecture decisions (ADRs, `docs/ENGINE_OVERVIEW.md`, the '
               'synthesis) and netcode. Agent files set it in their frontmatter. The caller passes it for everything '
               'else: the Agent tool\'s `effort` for built-in agents and for a `high` agent working on an ADR or on netcode '
               '(`xhigh`); `effort: \'high\'`, `\'xhigh\'` or `\'max\'` on every workflow `agent()` call (without it, a '
               'workflow agent takes the session\'s effort).')
item('K-4 (a)', 'CS-B', 'CLAUDE.md', 174, 178, '- **Effort is always stated**',
     wrap('- **Effort is always stated** (Opus 5.5 defaults to medium): `max` for every agent that reviews, verifies or '
          'designs game or engine code (his words above), set in their agent files (K-5 in this change-set); the '
          'netcode chat keeps its efforts by passing each agent its former effort (`high`, or `xhigh` for '
          'engine-architect and netcode-architect) until PR #40 has merged. ' + EFFORT_TAIL, '', '  '),
     'MODELS-HOW (a) only: the effort bullet; the agent files change in CS-B.')
item('K-4 (b)', 'CS-B', 'CLAUDE.md', 174, 178, '- **Effort is always stated**',
     wrap('- **Effort is always stated** (Opus 5.5 defaults to medium): `max` for every agent that reviews, verifies or '
          'designs game or engine code (his words above): the caller passes `effort: "max"` to those agents, except in '
          'the netcode chat, which keeps its efforts. ' + EFFORT_TAIL, '', '  '),
     'MODELS-HOW (b) only: the effort bullet; the agent files keep their effort lines.')

# K-6: MANUAL-MODE (step 3's mode clause)
item('K-6', 'CS-B', 'CLAUDE.md', 71, 73, '   (in auto mode',
     wrap('(in auto mode, Claude Code\'s own classifier may refuse a `.claude/` edit before the prompt; make that edit in '
          'Accept edits mode, the mode a cloud session offers besides Plan and Auto, or in a CLI\'s Manual mode, which '
          'hooks see as `default`); agents and teammates may not edit them. The ask is a seatbelt, not the wall: the PR '
          'review and Mustafa\'s go are the wall.', '   ', '   '),
     'Wording only: the cloud app has no Manual mode (the environment\'s permissions page, 2026-11-14).')
# K-7: GIT-C
item('K-7', 'CS-B', '.claude/CHECKPOINTS.md', 219, 224, '- When a change-set is merged',
     wrap('- When a change-set is merged, remove its agent copies with `git worktree remove <path>`, never with '
          '`--force`. Git refuses a copy with changed or untracked files, but deletes ignored files (`.gate/`, builds, '
          'logs) and drops commits on a detached HEAD. Before removing: nothing to keep in `git status --short '
          '--ignored`, run inside the copy (`cd <path>` first: the guard refuses `git -C`); its HEAD is on a pushed '
          'branch or inside main (`git merge-base --is-ancestor <HEAD> origin/main`); copies nested under it removed '
          'first; then `git worktree prune`. A copy with work to keep stays until that work is saved. The night shift\'s '
          'removals make the same checks.', '', '  '),
     'Wording only: the command the guard accepts.')
# K-8: L-4, the setup line
item('K-8', 'CS-B', 'CLAUDE.md', 236, 237, '- Setup (once per machine',
     wrap('- Setup (once per machine, Node 22.13+): `cd tools && npm ci`, then from the root: `node '
          'tools/node_modules/playwright/cli.js install chromium` (the gate\'s smoke test 5a and pageshot need it; a '
          'session\'s settings deny `npx`, and allow this line)', '', '  '),
     'Wording only: the form settings.json:12 allows (L-4).')
# K-9: SD-N2, "the advisor" in Standing permission
item('K-9', 'CS-B', 'CLAUDE.md', 127, 130, 'before edits (a pure revert aside',
     [l.replace("the advisor's APPROVE before", "the advisor agent's APPROVE before")
       .replace('given after the advisor approved that head', 'given after the advisor agent approved that head')
      for l in FILES['CLAUDE.md'][126:130]],
     'Wording only (SD-N2): the advisor agent, never the advisor tool, whose advice cannot be read back. The lines '
     're-wrap when written.')
# K-10: status-diet's STATUS citations
item('K-10a', 'CS-B', '.claude/agents/advisor.md', 17, 17, '- `docs/STATUS.md`',
     ['- `docs/STATUS.md`: where things stand; the rest in `docs/STATUS_ARCHIVE.md`, his decisions word for word in',
      '  `docs/DECISIONS.md`.'],
     'Wording only (status-diet\'s citations).')
item('K-10b', 'CS-B', '.claude/agents/advisor.md', 286, 287, '  2. If the merge touched kit or gate files',
     [l.replace('(STATUS, limit L4)', '(`docs/STATUS_ARCHIVE.md`, "Open bugs and risks", kit-v3-3, limit L4)')
      for l in FILES['.claude/agents/advisor.md'][285:287]],
     'Wording only (status-diet\'s citations); re-wraps when written.')
item('K-10c', 'CS-B', '.claude/agents/netcode-architect.md', 23, 24, 'game state inside THREE objects',
     [l.replace('the determinism debts STATUS records',
                'the determinism debts `docs/STATUS_ARCHIVE.md` records ("Open bugs and risks")')
      for l in FILES['.claude/agents/netcode-architect.md'][22:24]],
     'Wording only (status-diet\'s citations); re-wraps when written.')

# K-12: the lines that name gate and smoke as the required checks (CS-F, the switch; `required` is the proposed name)
REQ = [
    ('K-12a', 'CLAUDE.md', 119, '- **Rollback** (A4)', [('(`gate` or `smoke` red on main', '(the required check `required` red on main')]),
    ('K-12b', 'CLAUDE.md', 130, 'comment posted by', [('with the gate and smoke checks green', 'with the required check `required` green (it needs the `gate` and `smoke` jobs and every gate v2 job)')]),
    ('K-12c', '.claude/agents/advisor.md', 157, '   only when `gate` and `smoke`', [('only when `gate` and `smoke` are green on the approved SHA', 'only when the required check `required` is green on the approved SHA (it needs `gate`, `smoke` and every gate v2 job)')]),
    ('K-12d', '.claude/agents/advisor.md', 163, '   - **As the PR Advisor routine**', [('read `gate` and `smoke` from CI\'s check runs on', 'read `required` from CI\'s check runs on')]),
    ('K-12e', '.claude/agents/advisor.md', 224, '  establishes it in your seat', [('the routine reads CI\'s `gate` and `smoke`', 'the routine reads CI\'s `required`')]),
    ('K-12f', '.claude/agents/advisor.md', 273, '- **Main breaks** when', [('`gate` or `smoke` is red on a main commit', '`required` is red on a main commit')]),
    ('K-12g', '.claude/agents/advisor.md', 294, '    holds: the red `gate` or `smoke` run', [('the red `gate` or `smoke` run on main', 'the red `required` run on main')]),
    ('K-12h', '.claude/agents/advisor.md', 323, '- Merge with failing or pending checks', [('unless the `gate` and `smoke` jobs both passed on the approved SHA', 'unless the required check `required` passed on the approved SHA')]),
]
for kid, path, n, expect, reps in REQ:
    before = FILES[path][n - 1]
    after = before
    for a, b in reps:
        if a not in after:
            sys.exit(f'{kid}: {a!r} not on {path}:{n}')
        after = after.replace(a, b)
    widen = kid in ('K-12a', 'K-12f', 'K-12g')
    note = ('A widening of "main breaks" (Rollback): a red gate v2 job on main then also starts a revert with no plan '
            'step; his words needed (ROLLBACK-SCOPE, asked at CS-F); re-wraps when written.') if widen else \
           ('Same strength: `required` fails unless gate, smoke and every gate v2 job succeeded; re-wraps when written.')
    item(kid, 'CS-F', path, n, n, expect, [after], note)

# K-14 (CS-B): the agents' paste block learns the shell forms the guard refuses (PROMPTS-2.3's route; his words of
# 2026-11-11, 16:54:54 UTC, asked this for 2.1a's briefs; standing here)
FILES['.claude/CHECKPOINTS.md']  # loaded above
item('K-14', 'CS-B', '.claude/CHECKPOINTS.md', 128, 128, '  from your notes alone, opening no other file.',
     ['  from your notes alone, opening no other file. Never commit or push. Write files with the Write or Edit tool, or',
      '  with one plain command whose target is a literal path: the guard refuses `git -C`, a write after `cd`, and a',
      '  write to a path held in a variable or built in a loop.'],
     'Stricter-neutral guidance (fewer refusals); the paste block re-wraps when written.')

# K-6b (CS-D, only with G-3): step 3 says the guard refuses kit and gate edits in Auto mode
item('K-6b', 'CS-D', 'CLAUDE.md', 71, 73, '   (in auto mode',
     wrap('(in Auto mode the guard refuses a kit or gate edit outright, and Claude Code\'s own classifier may refuse a '
          '`.claude/` edit before the prompt: switch the chat to Accept edits first, the mode a cloud session offers '
          'besides Plan and Auto, or use a CLI\'s Manual mode, which hooks see as `default`); agents and teammates may '
          'not edit them. The ask is a seatbelt, not the wall: the PR review and Mustafa\'s go are the wall.',
          '   ', '   '),
     'Wording for G-3 (CS-D); replaces K-6 once G-3 is built. Stricter (the guard refuses where it asked).')

# K-11 (CS-D): the protected-file and gate-file lists name the gate v2's folder and, by PIN-SCOPE (b), the pins and configs
item('K-11a', 'CS-D', 'CLAUDE.md', 68, 69, '   (the guard\'s protected files',
     wrap('(the guard\'s protected files: `.claude/` except agent memory, every `CLAUDE.md`, `CLAUDE.local.md` and '
          '`AGENTS.md`, `.github/`, `tools/gate.mjs` and the other gate files, the gate v2\'s folder `tools/gate-v2/`, '
          'and the v2 pins and gate configs at the root: `rust-toolchain.toml`, `Cargo.toml`, `deny.toml`, '
          '`rustfmt.toml`, `tsconfig.base.json`, `eslint.config.mjs`, `.prettierrc.json`, `.prettierignore`, and each '
          'crate\'s `clippy.toml`) made with a file tool or a shell write asks Mustafa first, in', '   ', '   '),
     'Stricter (more protected paths); the list follows GATE-PATH (a) and PIN-SCOPE (b); re-wraps when written.')
item('K-11b', 'CS-D', '.claude/agents/advisor.md', 164, 166, '     the approved SHA. That green stands only',
     wrap('the approved SHA. That green stands only when the PR touches neither `.github/` nor the gate\'s own files '
          '(`tools/gate.mjs`, `smoke.mjs`, `battery.js`, `gate-globals.json`, `package*.json`, `npm-shrinkwrap.json`, '
          'any `.npmrc`; the gate v2\'s folder `tools/gate-v2/`; the v2 pins and gate configs the guard protects) — '
          'otherwise CI', '     ', '     '),
     'Stricter (more files make the PR Advisor\'s CI green insufficient); re-wraps when written.')

item('K-11c', 'CS-D', '.claude/agents/advisor.md', 39, 43, '3. **The rules themselves:**',
     wrap('3. **The rules themselves:** changes to hard constraints, ADRs, `CLAUDE.md`, `.claude/` (this file included), '
          'CI (`.github/`), security rules, or the gate\'s own files: every file `tools/gate.mjs`, the gate v2 '
          '(`tools/gate-v2/`) or `.github/workflows/` run as code or read as settings, not the game files they check '
          '(today `tools/gate.mjs`, `tools/smoke.mjs`, `tools/battery.js`, `tools/gate-globals.json`, '
          '`tools/package.json`, `tools/package-lock.json`, `tools/npm-shrinkwrap.json`, the gate v2\'s folder and the v2 '
          'pins and gate configs the guard protects) and any `.npmrc`.', '', '   '),
     'Stricter (escalation category 3 names the gate v2 and the protected pins); follows GATE-PATH (a) and PIN-SCOPE (b).')

# K-13 (CS-F): the lines that name the gate's command learn the gate v2 for v2 code
item('K-13a', 'CS-F', 'CLAUDE.md', 74, 75, '4. Gate: `/gate` must PASS',
     wrap('4. Gate: `/gate` must PASS (syntax, concatenation, symbol conservation, player-build resolvability, and check '
          '5: smoke, names, layering, script order), and for v2 code (`crates/`, `packages/`, the root pins) the gate '
          'v2, `node tools/gate-v2/gate.mjs`, must PASS too, plus evidence:', '', '   '),
     'Stricter (a second gate for v2 code).')
item('K-13b', 'CS-F', 'CLAUDE.md', 238, 239, '- Gate: `node tools/gate.mjs',
     FILES['CLAUDE.md'][237:239] + wrap('- Gate v2 (v2 code): `node tools/gate-v2/gate.mjs [baseRef] [--allow-removed a,b] [--quick] '
          '[--prove] [--only G1,G2] [--json]` (checks G0-G13 as built; exits 0, 1 or 2; CI runs it inside the required '
          'check `required`)', '', '  '),
     'A new command line beside the legacy gate\'s.')
item('K-13c', 'CS-F', '.claude/agents/verifier.md', 21, 21, '   (This is the legacy gate;',
     ['   (This is the legacy gate. For v2 code also run the gate v2, `node tools/gate-v2/gate.mjs <baseRef>`, and its',
      '   `--prove` when the change touches the gate, its checks or its configs.)'],
     'Stricter (the verifier runs the gate v2 for v2 code).')
item('K-13d', 'CS-F', '.claude/skills/gate/SKILL.md', 6, 6, 'Run `node tools/gate.mjs',
     ['Run `node tools/gate.mjs $ARGUMENTS` and read the whole output. When the change touches v2 code (`crates/`,',
      '`packages/`, the root pins), also run `node tools/gate-v2/gate.mjs $ARGUMENTS` and read its whole output.'],
     'Stricter (the skill runs both gates for v2 code).')

md = ['# Kit texts for the step-2.3 plan (generated by kit-texts.py from main 4460ef5; his words checked against '
      'docs/DECISIONS.md)', '',
      f'Quotes of his words checked: {len(CHECKS)}, each found in DECISIONS.md once whitespace is joined.', '']
for kid, cs, path, a, b, before, after, note in K:
    md += [f'## {kid} ({cs}): `{path}` lines {a}-{b} on 4460ef5', '', 'Before:', '```'] + before + ['```', '',
           'After:', '```'] + after + ['```', '', f'{note}', '']
open(OUT, 'w', encoding='utf-8').write('\n'.join(md) + '\n')
print(f'wrote {OUT}: {len(K)} items, {len(CHECKS)} quotes of his checked')

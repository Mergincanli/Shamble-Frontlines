# docs-keeper kit-v3-3 step 2: builds the STATUS Follow-ups bullet and the "Queued, in Mustafa's words" block.
# Run from the worktree root: python3 -I .checkpoints/kit-v3-3/dk-step2-build.py
import re, textwrap

Q = '.checkpoints/kit-v3-3/queue/'
P = 'docs/STATUS.md'

a4 = open(Q + 'a4-spec-2026-11-04.md', encoding='utf-8').read().split('\n')
a4_main = [l for l in a4[a4.index('> A4, a small kit change-set after A3, plan first (Mustafa): a review policy inspired by Anthropic\'s own process.'):]
           if l.startswith('>')]
# split into the first quote (before the Addition heading) and the addition
i_add = a4.index('### Addition (Mustafa, 2026-11-04), word for word')
first = [l for l in a4[:i_add] if l.startswith('>')]
addition = [l for l in a4[i_add:] if l.startswith('>')]

bl = open(Q + 'mustafa-2026-11-03-list.md', encoding='utf-8').read().split('\n')
code = bl[bl.index('```text') + 1: bl.index('```', bl.index('```text') + 1)]
b_lines = [code[0]] + code[code.index('B. Docs:'):]   # his list's header line, then B, B.1, B.2

def quote(lines):
    return ['    ' + l for l in lines]

def wrap_item(text):
    m = re.match(r'^(\s*)(\d+\.\s|)(.*)$', text)
    lead = m.group(2)
    body = m.group(3)
    first_pfx = '    > ' + lead
    rest_pfx = '    > ' + ' ' * len(lead)
    return textwrap.wrap(body, width=120, initial_indent=first_pfx, subsequent_indent=rest_pfx,
                         break_long_words=False, break_on_hyphens=False)

b_out = []
for l in b_lines:
    b_out += wrap_item(l)

followups = '''- **Follow-ups** (each with its source; recorded in the repo by kit-v3-3, A3, PR #38, on Mustafa's addition of
  2026-11-04: "Record them in the repo, in STATUS's follow-ups, alongside A3's plan."):
  - TOY-BOX's small kit change-set: done, with kit-v3-2 (A2 on his list; PR #37, 12d7961, 2026-11-04 23:06 UTC).
  - A3's own items: the day-session ask before kit and gate edits, the `sk-` key after `%20`, `%2C` or `%3A`,
    read-only Actions tools for pr-advisor, `git rev-list`, python in the docs, the advisor's Write tool and
    `node .checkpoints/*`: in flight, PR #38 (Open bugs and risks, A3's and kit-v3-2's entries).
  - LATER's design session after the netcode proof test, with the twelve LATER questions and the 38 design questions
    on its agenda (the advisor's ruling).
  - The B3 report (`docs/reports/2026-11-04-b3-design-walk.md`), a later report edit: its "Changes to his words"
    (report:56-57) lacks plan rev 1 §3's approved deviations (a capital first letter at LOOP_V2:147 and :169, labels
    before older quoted lines, the splits and repeats; recorded at c5d86cb, plan-rev1-folded.md:88-91), and its body
    runs 83 lines (skill: ~60); its "fixed in the last commit" (report:37) means c5d86cb (PR #36's final review, the
    advisor's APPROVE of a32e4c0, items 1-2).
  - The kit-v3-2 report (`docs/reports/2026-11-04-kit-v3-2.md`), a later report edit: R-1, it runs 69 lines (skill:
    ~60); R-2, its Risks omit "nothing enforces the cap or the scratch rule" and the notes-folder script prompt (Open
    bugs and risks, kit-v3-2 items 1 and 5, carry both) (the advisor's APPROVE of PR #37's final head 342ff49, comment
    6643739289).
  - F-1, for a kit change-set after A3 (cut from A3 by its plan's condition C3): CHECKPOINTS.md §4's trust rule gives
    no way back when a done block's output outside the notes folder (a build under `.gate/`) is lost with a container
    while the notes survive; the fix, about 15 words ("if a later block finds a done block's output missing, note it
    and redo that block") at CHECKPOINTS.md:77-78, :110-111 and :119, with a 3-run dry run of the paste block (the
    same APPROVE of 342ff49).
  - "B3" means three things (his list's item, the SharedArrayBuffer ring, ADR 0006 part 13's CSP row): still open in
    `docs/adr/README.md` row 86 and ADR 0006's OQ-0006-23 (both inside his quoted words) and the merged #35 report,
    for the drafts' next revision (PR #35's final review, the advisor's APPROVE of 76c35bf, item 5; its STATUS and
    ROADMAP part was taken by B3, PR #36).
  - Local smoke needs Playwright's Chromium build 1194 (Playwright 1.56.1); this container has 1243 in
    `/opt/pw-browsers`: a candidate for the environment's setup or tools/README's "Check a new machine" (PR #36's
    final review, item 5).
  - Routine proofs to record: what the night shift's first guarded night (2026-11-05 from 00:10) and the PR
    Advisor's next scheduled runs show; its first, 2026-11-04 18:45 UTC, failed on the usage limit (Routines) (the
    docs-pass queue, with PR #35's final review).
  - CHECKPOINTS.md §7's cap line could say a workflow script must still wave its calls in 3s: this container's
    runner allows CPUs - 2 agents at once (2 here), a bigger machine's up to 16 (A2's later-pass item 2; for A4 or a
    later kit pass).
  - E9's untested cases: the "first line names another task" case (notes moved into `old/`) and worktree agents; a
    small dry run in a later kit pass (A2's later-pass item 1; Open bugs and risks, kit-v3-2 item 4).
  - The toy-box tone outside A2's files (`README.md:4`, `docs/GAME_DESIGN.md:14`, `:17`, `:28`, `:50-51`,
    `docs/ROADMAP.md:10`), for a later docs pass against LOOK_V2 (A2's later-pass item 3; KIT-ALIGN, below).
  - From A3's plan readers, for security-auditor to weigh: code.claude.com and docs.anthropic.com are not on the
    WebFetch list; the allow entries `node .gate/*.mjs` and `node .gate/*.js` (`.claude/settings.json:49-50`) also
    match paths with `..`; a routine can write a kit file in a scratchpad worktree or a worktree under `.gate/`,
    caught only at commit and push (`.claude/hooks/routine-guard.mjs:287-299`, `:532-538`, `:554-560`; A3's critic).
  - A3's cuts (its plan's condition C3), for later: the secret check re-testing notes after up to 3 rounds of
    percent-decoding, and environment values on decoded text; URL credentials with an encoded `@` (`\\u0040`,
    `%40`); routine reads on other repositories (`get_job_logs` on another repository, `gh … -R/--repo`,
    `search_*`), for security-auditor.
  - The kit-v3-1 Windows test follow-up stays in Open bugs and risks (kit-v3-1 item 1; its own change-set).'''

queued = ['- **Queued, in Mustafa\'s words** (word for word, each with its date and source; each removed by the change-set',
          '  that carries it out; copied from the A2 and A3 day session\'s queue files, session_0173vVo7FcYUpbWgRW8oQyHU):',
          '  - A4 (kit): the review policy, Mustafa, 2026-11-04. Queued after A3, plan first; a kit change-set, so it needs',
          '    his go:']
queued += quote(first)
queued += ['  - A4\'s addition, Mustafa, 2026-11-04:']
queued += quote(addition)
queued += ['  - For A4\'s plan (the session\'s notes, not his words): check what ADR 0006 already plans (rollback, crash reports',
           '    and telemetry, server metrics) before writing items 4 and 6 as notes; item 5\'s checklists must not contradict',
           '    advisor.md\'s "Never" list or step 0; item 3: who may revert, and through a PR (main is protected); item 2: how',
           '    a round is counted, and which severity scale; Tier 0: the PR Advisor\'s merge or GitHub auto-merge.',
           '  - B.1 and B.2 (docs), Mustafa, 2026-11-03, from his task list (session_0173vVo5sT21uP4DcQtKvmAU, his message at',
           '    2026-11-03T22:08:25Z; its A.1 and A.2 are done: PR #30 and PR #37):']
queued += b_out

new = followups + '\n' + '\n'.join(queued) + '\n'

s = open(P, encoding='utf-8').read()
start = s.index('- **Follow-ups:** TOY-BOX')
end = s.index('- **Taken by Mustafa 2026-10-31, about 18:41')
assert s.count('- **Follow-ups:** TOY-BOX') == 1
s = s[:start] + new + s[end:]
open(P, 'w', encoding='utf-8').write(s)
open('.checkpoints/kit-v3-3/dk-step2-block.txt', 'w', encoding='utf-8').write(new)
print('ok', len(new.split('\n')))

#!/usr/bin/env python3
"""Apply the 2026-10-31 answers to the research files (change-set docs-after-v2-research, plan item R).

Usage (run from the repository root, with python3 -I):
  python3 -I .checkpoints/docs-after-v2-research/scripts/apply_research_answers.py --check   # dry run: every anchor once
  python3 -I .checkpoints/docs-after-v2-research/scripts/apply_research_answers.py --apply   # writes the files
  python3 -I .checkpoints/docs-after-v2-research/scripts/apply_research_answers.py --annex   # prints the edit table (markdown)

Three kinds of edit, all exact:
  replace : `before` (exact text, line breaks included) must occur exactly once in the file; it becomes `after`.
  append  : `anchor` is the exact start of a list entry's first line (its bold heading) and must start exactly one line; the entry ends
            before the next line that starts a list entry ("- **"), a heading ("#") or is blank; `add` (one or more
            whole lines) is inserted after the entry's last line.
  after   : `anchor` (an exact whole line) must occur exactly once; `add` is inserted right after it.
Nothing else in any file changes. --apply refuses to write if any check fails.
"""
import sys, json, pathlib

R = 'docs/research/'

# ---- The answered shared questions (one note per copy, the same words in every topic file) ----
MODES = ["  **Answered 2026-10-31 (Mustafa): (a), totals.** The largest mode is the goal 24v24, N = 48; where a mark gives",
         "  values for 48 and 54, the 48 value holds."]
SPREAD = ["  **Answered 2026-10-31 (Mustafa): (a), server-only spread:** weapon spread stays in the authority-only stream."]
BASE = ["  **Answered 2026-10-31 (the Advisor): (b).** ADR 0006 lists the security baseline as open, with the interim positions",
        "  above; the baseline is researched before the first online playtest, in its own fresh session."]
WALL = ["  **Answered 2026-10-31 (Mustafa): (a):** previews that reach the staging game server are behind the access wall from",
        "  the first one."]
ABL = ["  **Mustafa, 2026-10-31: answered later, after the Feel Lab test.**"]

H_ABL = "- **ABL-PREDICT (Mustafa, after a Feel Lab test). Does the client predict the local player's own abilities?**"
H_SPREAD = "- **SEC-SPREAD (Mustafa). Matching tracers, or cheat resistance?**"
H_MODES = "- **MODES-3 (Mustafa). How are VISION_V2 §3's starting sizes read?**"
H_BASE = "- **BASELINE-WHEN (the Advisor). Research the deferred security baseline before ADR 0006, or list it open there?**"
H_WALL = "- **PREVIEW-WALL (Mustafa). Should per-PR previews that talk to the staging game server be private?**"

EDITS = []
def ap(fid, f, anchor, add, why):
    EDITS.append(dict(id=fid, kind='append', file=R + f, anchor=anchor, add=add, why=why))

for f, codes in [('r1-simulation-core.md', 'ABL SPREAD MODES BASE'), ('r2-netcode.md', 'ABL SPREAD MODES BASE'),
                 ('r3-rendering.md', 'MODES'), ('r4-physics-navigation.md', 'MODES'),
                 ('r5-toolchain-quality.md', 'MODES BASE WALL'), ('r6-infrastructure.md', 'MODES BASE WALL')]:
    t = f[:2].upper()
    for c in codes.split():
        head, add, label = {'ABL': (H_ABL, ABL, 'ABL-PREDICT'), 'SPREAD': (H_SPREAD, SPREAD, 'SEC-SPREAD'),
                            'MODES': (H_MODES, MODES, 'MODES-3'), 'BASE': (H_BASE, BASE, 'BASELINE-WHEN'),
                            'WALL': (H_WALL, WALL, 'PREVIEW-WALL')}[c]
        ap(f'A-{t}-{label}', f, head, add, f'{label}: the answer of 2026-10-31, under the shared question')

# ---- The glossary: one note per "Depends on" paragraph tied to an answered question, plus two sentences ----
G = 'GLOSSARY.md'
GL = [
    ('G-start-modes', "- **Depends on:** MODES-3. Reading (a), totals: 1v1 to 4v4 (N = 2 to 8) and 2v2v2 to 6v6v6 (N = 6 to 18). Reading (b),",
     ["  **Answered 2026-10-31 (Mustafa): MODES-3 (a), totals,** so reading (a) holds."]),
    ('G-largest-mode', "- **Depends on:** MODES-3. Under (a) the largest mode is the goal 24v24, N = 48. Under (b) it is the start mode",
     ["  **Answered 2026-10-31 (Mustafa): MODES-3 (a), totals,** so the largest mode is the goal 24v24, N = 48."]),
    ('G-design-point', "- **Depends on:** MODES-3, for how the two relate. Under (a) the design point equals the largest mode's N (48). Under",
     ["  **Answered 2026-10-31 (Mustafa): MODES-3 (a),** so the design point equals the largest mode's N (48)."]),
    ('G-test-modes', "- **Depends on:** MODES-3, for items 3 and 4. Under (a): 1v1, 4v4, 2v2v2 and 6v6v6, and the largest mode is 24v24",
     ["  **Answered 2026-10-31 (Mustafa): MODES-3 (a),** so items 3 and 4 are 1v1, 4v4, 2v2v2 and 6v6v6, and the largest",
      "  mode is 24v24."]),
    ('G-stress-scene', "- **Depends on:** MODES-3, for N and the three-team arrangement only; every other count is the same under both readings.",
     ["  **Answered 2026-10-31 (Mustafa): MODES-3 (a),** so the soldiers are N = 48, in three teams (16v16v16)."]),
    ('G-prediction-scope', "- **Depends on:** ABL-PREDICT (Mustafa, after a Feel Lab test), for the player's own abilities: (yes) the scope adds",
     ["  **Mustafa, 2026-10-31: ABL-PREDICT is answered later, after the Feel Lab test.**"]),
    ('G-spread-stream', "- **Depends on:** SEC-SPREAD (Mustafa, R2-SYNC-OQ11), for weapon spread only: under the security review's answer it",
     ["  **Answered 2026-10-31 (Mustafa): SEC-SPREAD (a),** so weapon spread stays authority-only."]),
    ('G-access-wall', "- **Depends on:** PREVIEW-WALL (Mustafa): (a) on from the first preview that reaches the staging game server; (b)",
     ["  **Answered 2026-10-31 (Mustafa): PREVIEW-WALL (a),** so the wall is on from the first preview that reaches the",
      "  staging game server."]),
    ('G-security-baseline', "- **Depends on:** BASELINE-WHEN (the Advisor): (a) researched now, as one security-auditor topic before the drafts;",
     ["  **Answered 2026-10-31 (the Advisor): BASELINE-WHEN (b),** so it is listed open in ADR 0006 with the interim",
      "  positions above, and researched before the first online playtest, in its own fresh session."]),
    ('G-certificate', "- **Depends on:** BASELINE-WHEN (the Advisor), as above: under (b) the certificate per hostname holds and the wildcard",
     ["  **Answered 2026-10-31 (the Advisor): BASELINE-WHEN (b),** so the certificate per hostname holds and the wildcard is",
      "  an option the baseline weighs."]),
]
for fid, anchor, add in GL:
    ap(fid, G, anchor, add, 'the answer of 2026-10-31, under the entry\'s "Depends on"')

EDITS.append(dict(id='G-codes-header', kind='after', file=R + G,
                  anchor='"Shared open questions (the same words in every topic)" subsection.',
                  add=["Answered on 2026-10-31: MODES-3 (a), PREVIEW-WALL (a) and SEC-SPREAD (a) by Mustafa, BASELINE-WHEN (b) by the",
                       "Advisor; ABL-PREDICT waits for a Feel Lab test and PREV-DELETE for its spike."],
                  why='one summary line under the list of decision codes'))
EDITS.append(dict(id='G-preview', kind='after', file=R + G,
                  anchor='  applies). It carries the preview header set. Until PREVIEW-WALL is answered there are no multiplayer previews.',
                  add=["  PREVIEW-WALL was answered (a) on 2026-10-31: a multiplayer preview is behind the access wall from the first one."],
                  why='the definition\'s interim sentence, now answered'))

# ---- The six (b)-only changes approved on 2026-10-30 (appendix §2: C17.b1–b3 at 1071-1080; C23.b1–b3 at 1415-1428,
#      as REVISED at 1596-1599). V = verbatim; ADAPTED = the approved text fitted to step 3c's wording (advisor rules) ----
EDITS += [
    dict(id='C17.b1', status='verbatim', kind='replace', file=R + 'r1-simulation-core.md',
         before="  cargo-audit and cargo-deny as CI gates. Decides it: R5's CI research and the synthesis. R5-GATE-REC1 schedules a\n",
         after="  cargo-audit and cargo-deny as CI gates. Decided (BASELINE-WHEN (b)): the mechanism is R5-GATE-REC1's scheduled\n"
               "  advisories check, also run on every PR that changes `Cargo.lock`; whether an advisory blocks a PR, and who owns that\n"
               "  policy, is the security baseline's (R6-SEC-OQ1). R5-GATE-REC1 schedules a\n",
         why='R1-RR-OQ7: C17.b1 as approved (the quote matches; step 3c\'s interim sentence after it stays, so the mechanism '
             'is named twice)'),
    dict(id='C17.b2', status='ADAPTED', kind='replace', file=R + 'r2-netcode.md',
         before="An advisories check ([GLOSSARY](GLOSSARY.md#the-advisories-check)) that gates pull requests to keep\n"
                "   these current belongs to the security baseline ([GLOSSARY](GLOSSARY.md#the-security-baseline); dependency audits), deferred by the\n"
                "   Advisor; R5-GATE-REC1 already schedules the advisories check, and its policy waits on BASELINE-WHEN (below).",
         after="The check that keeps these current is R5-GATE-REC1's scheduled advisories check\n"
               "   ([GLOSSARY](GLOSSARY.md#the-advisories-check); also run when `Cargo.lock` changes); its policy, whether an advisory blocks a PR,\n"
               "   belongs to the security baseline ([GLOSSARY](GLOSSARY.md#the-security-baseline); dependency audits), deferred by the Advisor\n"
               "   (BASELINE-WHEN (b)).",
         why='R2-TRN-REC1 mark 5: the approved C17.b2 "after" text, keeping step 3c\'s two glossary links at their terms; '
             'its approved "before" quote ("An audit gate that keeps these current belongs to R6\'s security baseline '
             '(dependency audits), deferred by the Advisor.") no longer occurs, because step 3c reworded the sentence'),
    dict(id='C17.b3', status='verbatim', kind='replace', file=R + 'r5-toolchain-quality.md',
         before="  security baseline. Shared as BASELINE-WHEN (below).\n- **R5-GATE-OQ5.",
         after="  security baseline. Shared as BASELINE-WHEN (below). Interim (BASELINE-WHEN (b)): scheduled and on lockfile changes, as\n"
               "  R5-GATE-REC1 has it; the baseline sets the policy.\n- **R5-GATE-OQ5.",
         why='R5-GATE-OQ4: C17.b3 as approved, appended to the entry'),
    dict(id='C23.b1', status='verbatim', kind='replace', file=R + 'r6-infrastructure.md',
         before="Production certificates: one wildcard certificate for the game-server names (option C2), from ACME\n"
                "  with dns-01, renewed by one job and copied to every game VM.** This departs from R2-TRN-REC5's wording,",
         after="Production certificates: a publicly trusted certificate per game-server hostname (R2-TRN-REC5;\n"
               "  option C1, or ACME dns-01 per name), until the security baseline weighs one wildcard certificate (option C2: renewed\n"
               "  by one job and copied to every game VM) (BASELINE-WHEN (b)).** C2 would depart from R2-TRN-REC5's wording,",
         why='R6-HOST-REC4 heading: C23.b1 as REVISED by the advisor (the quote matches across its line break), together '
             'with C23.b2\'s first part ("This departs" → "C2 would depart")'),
    dict(id='C23.b2', status='ADAPTED', kind='replace', file=R + 'r6-infrastructure.md',
         before="Per-hostname certificates (C1, or dns-01 per name) stay the alternative if the\n"
                "  baseline rejects a shared key; when the baseline is researched is BASELINE-WHEN (below). It needs no TCP 80 on game VMs.\n"
                "  *Reason:* RFC 8555 allows wildcard orders and proves control through DNS (F24); one certificate for all VMs makes\n"
                "  adding a VM a copy, not an issuance.",
         after="C2 replaces them only if the\n"
               "  baseline accepts a shared key; when the baseline is researched is BASELINE-WHEN (below). With dns-01 per name it needs\n"
               "  no TCP 80 on game VMs (F24); C1's http-01 needs TCP 80.\n"
               "  *Reason:* a certificate per hostname is R2-TRN-REC5's and puts no shared private key on the game VMs; RFC 8555 proves\n"
               "  control through DNS (F24), so dns-01 per name needs no TCP 80. C2's gain, for the baseline to weigh: one certificate\n"
               "  for all VMs makes adding a VM a copy, not an issuance; RFC 8555 allows wildcard orders (F24).",
         why='R6-HOST-REC4 body and Reason: C23.b2 as REVISED; ADAPTED in one place: the approved quote "… rejects a shared '
             'key." now ends "… rejects a shared key; when the baseline is researched is BASELINE-WHEN (below)." (step 3c\'s '
             'interim C23 clause), so only the approved part is replaced and that clause stays'),
    dict(id='C23.b3', status='verbatim', kind='replace', file=R + 'r6-infrastructure.md',
         before="with the wildcard\n  certificate ([GLOSSARY](GLOSSARY.md#game-server-certificate))",
         after="with the per-hostname\n  certificates ([GLOSSARY](GLOSSARY.md#game-server-certificate))",
         why='R6-HOST-REC4 spike: C23.b3 as approved ("with the wildcard certificate" → "with the per-hostname certificates")'),
]


def entry_end(lines, i):
    j = i + 1
    while j < len(lines) and not (lines[j].startswith('- **') or lines[j].startswith('#') or lines[j].strip() == ''):
        j += 1
    return j  # insert position (after the entry's last line)


def plan_file(path, edits):
    text = pathlib.Path(path).read_text()
    problems, out = [], text
    for e in [x for x in edits if x['kind'] == 'replace']:
        n = out.count(e['before'])
        if n != 1:
            problems.append(f"{e['id']}: 'before' occurs {n} times in {path}")
        else:
            out = out.replace(e['before'], e['after'])
    lines = out.split('\n')
    inserts = []
    for e in [x for x in edits if x['kind'] in ('append', 'after')]:
        hits = [k for k, l in enumerate(lines) if (l.startswith(e['anchor']) if e['kind'] == 'append' else l == e['anchor'])]
        if len(hits) != 1:
            problems.append(f"{e['id']}: anchor occurs {len(hits)} times in {path}")
            continue
        pos = entry_end(lines, hits[0]) if e['kind'] == 'append' else hits[0] + 1
        e['_at'] = pos  # 1-based line after which the text goes = pos
        inserts.append((pos, e['add'], e['id']))
    for pos, add, _ in sorted(inserts, key=lambda x: -x[0]):
        lines[pos:pos] = add
    return text, '\n'.join(lines), problems


def main():
    mode = sys.argv[1] if len(sys.argv) > 1 else '--check'
    files = sorted({e['file'] for e in EDITS})
    allp, results = [], {}
    for f in files:
        old, new, p = plan_file(f, [e for e in EDITS if e['file'] == f])
        allp += p
        results[f] = (old, new)
    if mode == '--annex':
        print('| id | file | kind | where (line in the merged file) | status |')
        print('|---|---|---|---|---|')
        for e in EDITS:
            where = e.get('_at', '')
            if e['kind'] == 'replace':
                old = results[e['file']][0]
                where = old[:old.find(e['before'])].count('\n') + 1 if e['before'] in old else '?'
            print(f"| {e['id']} | {e['file'].replace(R, '')} | {e['kind']} | {where} | {e.get('status', 'new note')} |")
        return
    if mode == '--diff':  # read-only: the unified diff the --apply run would produce
        import difflib
        for f in files:
            old, new = results[f]
            sys.stdout.writelines(difflib.unified_diff(old.splitlines(True), new.splitlines(True),
                                                       'a/' + f, 'b/' + f, n=1))
        if allp:
            sys.exit('problems: ' + '; '.join(allp))
        return
    for f in files:
        old, new = results[f]
        added = new.count('\n') - old.count('\n')
        print(f"{f}: {sum(1 for e in EDITS if e['file'] == f)} edits, {added:+d} lines")
    print(f"{len(EDITS)} edits in {len(files)} files; problems: {len(allp)}")
    for p in allp:
        print('  PROBLEM:', p)
    if mode == '--apply':
        if allp:
            sys.exit('refused: fix the problems first')
        for f in files:
            pathlib.Path(f).write_text(results[f][1])
        print('applied')
    elif allp:
        sys.exit(1)


if __name__ == '__main__':
    main()

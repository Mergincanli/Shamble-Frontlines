import json, sys
# Compare each E9 run's first prompt (from its transcript) with its prompt.txt and with the paste block of its text.
S, T, G = sys.argv[1], sys.argv[2], sys.argv[3]
def block_from_md(p):
    lines = open(p, encoding='utf-8').read().split('\n')
    i = next(k for k, l in enumerate(lines) if l.strip() == '## Saving your progress')
    out = []
    for l in lines[i:]:
        if l.strip() == '```': break
        out.append(l[2:])
    return '\n'.join(out)
A = block_from_md(G + '/cp-A.md'); D = block_from_md(G + '/cp-D.md')
B = open(S + '/e9f/paste-block-final.txt', encoding='utf-8').read().rstrip('\n')
C = open(S + '/e9g/paste-block.txt', encoding='utf-8').read().rstrip('\n')
print('e9/paste-block == A:', open(S + '/e9/paste-block.txt').read().rstrip('\n') == A)
print('e9h/paste-block == D:', open(S + '/e9h/paste-block.txt').read().rstrip('\n') == D)
norm = lambda s: ' '.join(s.split())
trust = 'Trust the blocks they mark done: never reread, recheck or redo them (a later block may still read a file it needs). '
print('B == A + trust (words):', norm(B) == norm(A).replace('they mark done. Start', 'they mark done. ' + trust + 'Start'))
print('C == B with "from your notes." ending (words):', norm(C) == norm(B).replace('give your answer. Never', 'give your answer from your notes. Never'))
print('D == C with "alone, without opening any file again." (words):', norm(D) == norm(C).replace('from your notes. Never', 'from your notes alone, without opening any file again. Never'))
texts = {'A': A, 'B': B, 'C': C, 'D': D}
def fill(t, folder, part):
    return (t.replace('<path>/<agent>-<part>.md', folder + '/notes/' + part + '.md')
             .replace('<path>/', folder + '/notes/').replace('.gate/<task-slug>/', '.gate/e9-dryrun/'))
runs = [l.split() for l in open(sys.argv[4]) if l.strip()]
for name, aid, folder, text in runs:
    first = None
    for line in open(T + '/agent-' + aid + '.jsonl', encoding='utf-8'):
        try: ev = json.loads(line)
        except Exception: continue
        m = ev.get('message') or {}
        if m.get('role') == 'user':
            c = m.get('content')
            if isinstance(c, list):
                c = ''.join(x.get('text', '') for x in c if isinstance(x, dict) and x.get('type') == 'text')
            first = c; break
    pf = S + '/' + folder
    p = open(pf + '/prompt.txt', encoding='utf-8').read()
    part = 'dryrun-blocks' if (name in ('run1', 'run2', 'run3') or name.startswith('w')) else 'dryrun-facts'
    blk = fill(texts[text], pf, part)
    k = p.find('## Saving your progress')
    print(name, text, 'prompt==prompt.txt:', first == p, '| rstrip-equal:', (first or '').rstrip() == p.rstrip(),
          '| block==text filled:', p[k:].rstrip('\n') == blk, '| len', len(first or ''), len(p))

# Verifier pass 4: STATUS texts word for word, extracted from their sources (not from the docs-keeper's copy).
import re, sys
fold = lambda t: re.sub(r'\s+', ' ', t).strip()
a3, brief = sys.argv[1], sys.argv[2]
status = fold(open(f'{a3}/docs/STATUS.md').read())
adv = open(f'{a3}/.checkpoints/kit-v3-3/advisor-pass3.md').read().split('\n')
def block(lines, start_pat, stop_blank=True):
    i = next(k for k, l in enumerate(lines) if start_pat in l)
    out = [lines[i]]
    for l in lines[i + 1:]:
        if not l.startswith('  ') or l.strip() == '': break
        out.append(l)
    return fold(' '.join(out)).lstrip('- ').strip()
texts = {}
texts['R-PLAIN'] = block(adv, "**R-PLAIN, routines' plain shell (fix pass 3):**")
texts['L18'] = block(adv, '- L18, parallel tool calls:')
sd3 = next(l for l in adv if l.startswith("**STATUS's SD-3 line"))
texts['SD-3'] = re.search(r'"(.*)', sd3).group(1) + ' ' + adv[adv.index(sd3) + 1].rstrip('"')
i = next(k for k, l in enumerate(adv) if l.startswith('**STATUS, the night commit'))
texts['commit'] = fold(' '.join(adv[i:i + 3])).split('** "', 1)[1].rstrip('"')
b = open(brief).read()
def quoted(after):
    k = b.index(after); s = b.index('"', k) + 1; e = b.index('."', s) + 1
    return fold(b[s:e].replace('\\`', '`').replace('\\$', '$'))
texts['L16'] = quoted('- L16, replacing the current text, word for word:')
texts['FU-P3a'] = quoted("- STATUS's Follow-ups: FU-P3a, word for word:")
k = b.index('and the kit freeze in the'); s = b.index('"', k) + 1; texts['freeze'] = fold(b[s:b.index('"', s)])
mw = open(f'{a3}/.checkpoints/kit-v3-3/mustafa-words.md').read()
last = mw[mw.rindex('his scope stop for A3'):]
texts['scope'] = fold(last[last.index('"') + 1:last.rindex('"')])
for k, t in texts.items():
    print(k, 'FOUND' if t in status else 'MISSING', '|', t[:90], '...', t[-40:])

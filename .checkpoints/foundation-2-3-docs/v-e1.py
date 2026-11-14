# Verifier (read-only): is the report's D&D advisor-model bullet, from "Four chat messages since bear on it" to the
# bullet's end, equal (whitespace-normalised) to the advisor agent's exact E1 text in its 2e ruling? Also checks each
# quoted span of the bullet against the transcript line it cites. Prints booleans and lengths only.
#   python3 -I .checkpoints/foundation-2-3-docs/v-e1.py <report-file>
import json, re, sys
rep = open(sys.argv[1], encoding='utf-8').read()
adv = open('.checkpoints/foundation-2-3-docs/advisor-plan-rev1.md', encoding='utf-8').read()
norm = lambda s: re.sub(r'\s+', ' ', s).strip()
a0 = adv.index('to the end of the bullet: ') + len('to the end of the bullet: ')
a1 = adv.index(' Plus:', a0)
ruling = norm(adv[a0:a1])
start = rep.index('Four chat messages since bear on it')
end = rep.index('\n- ', start)
bullet = norm(rep[start:end])
print('ruling len', len(ruling), '| bullet part len', len(bullet), '| equal:', ruling == bullet)
if ruling != bullet:
    i = next((k for k in range(min(len(ruling), len(bullet))) if ruling[k] != bullet[k]), None)
    print('first difference at char', i)
T = '/root/.claude/projects/-home-user-Shamble-Frontlines/7261f3d7-f651-586a-9c39-c052553b68a3.jsonl'
lines = open(T, encoding='utf-8').read().split('\n')
def text_at(n):
    o = json.loads(lines[n - 1])
    return ' '.join(b.get('text', '') for b in (o.get('message') or {}).get('content') or [] if isinstance(b, dict) and b.get('type') == 'text'), o.get('timestamp')
for n, q in [(1887, 'If true, I told you the wrong thing at the start.'), (2122, 'where I first told you the advisor model')]:
    t, ts = text_at(n)
    print('line', n, ts, '| quote in line:', q in t, '| quote in bullet:', q in bullet)
for n in (2869, 2929):
    t, ts = text_at(n)
    print('line', n, ts, '| text len', len(t))

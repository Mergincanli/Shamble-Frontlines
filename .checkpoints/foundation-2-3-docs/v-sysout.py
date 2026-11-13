# Verifier (read-only): is the report's quote of the /advisor fable output a substring of the system entry at 18:30:08?
# Prints booleans and lengths; the harness's output text is shown only on the console (not his words).
#   python3 -I .checkpoints/foundation-2-3-docs/v-sysout.py
import json, re
T = '/root/.claude/projects/-home-user-Shamble-Frontlines/7261f3d7-f651-586a-9c39-c052553b68a3.jsonl'
R = 'docs/reports/2026-11-13-foundation-2-3-docs.md'
sysc = None
for line in open(T, encoding='utf-8'):
    o = json.loads(line)
    if o.get('type') == 'system' and o.get('subtype') == 'local_command' and str(o.get('timestamp','')).startswith('2026-11-13T18:30:08'):
        sysc = o.get('content')
print('system entry found', sysc is not None, 'len', len(sysc or ''))
print('CONSOLE ONLY:', repr(sysc))
rep = open(R, encoding='utf-8').read()
joined = re.sub(r'\s+', ' ', rep)
m = re.search(r'output \(18:30:08 UTC\) had said "([^"]+)"', joined)
q = m.group(1) if m else None
print('report quote found', q is not None, 'len', len(q or ''))
print('quote is substring of system entry', bool(q) and q in (sysc or ''))

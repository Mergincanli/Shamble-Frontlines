"""Fetch audit for the report (VERIFY-FETCH (a), the Advisor, 2026-10-30: "the host audit in each report"; plan
condition 7 of plan-amendments-2: "The host audit lists requested and final hosts by tool").

Reads every transcript of this session (the main session and every subagent, background or in a workflow) and lists,
per agent and tool, the web fetches: WebFetch calls (the URL asked for, and the host a redirect notice in the result
names), curl or wget in Bash commands (each URL in the command; a loop over a URL file is listed as such), and
WebSearch calls (queries, which fetch no page). Hosts are checked against the WebFetch allow list in
.claude/settings.json. Usage: python3 fetch_audit.py [--since 2026-10-30T00:00] [--md out.md] [--grouped out.md] [--json out.json]
"""
import glob, json, os, re, sys, collections
from urllib.parse import urlparse

P = '/root/.claude/projects/-home-user-Shamble-Frontlines/'
S = P + 'ed3ff506-9a45-5a14-adc3-75c262311890'
allow = {a[len('WebFetch(domain:'):-1] for a in json.load(open('/home/user/Shamble-Frontlines/.claude/settings.json'))
         ['permissions']['allow'] if a.startswith('WebFetch(domain:')}


def arg(name):
    return sys.argv[sys.argv.index(name) + 1] if name in sys.argv else None


since = arg('--since') or ''
URL = re.compile(r'https?://[^\s\'"<>|)\]\\`]+')
# a curl or wget command: the word where a shell command starts (after a line break, ;, &, |, (, `, {, the words do,
# then, else, xargs, time, env, sudo, or env assignments and `timeout N`); the word in prose (a commit message, a heredoc,
# an echo) does not count
WORD = re.compile(r'(?<![\w./-])(curl|wget)(?=[ \t]+[^\s"\'`|)])')   # followed by an argument, as a command is
STARTS = re.compile(r'(?:^|[\n;&|(`{]|\$\(|\b(?:do|then|else|xargs(?:\s+-\S+)*|time|env|sudo|exec|command))\s*'
                    r'(?:\w+=\S*\s+)*(?:timeout\s+\S+\s+)?$')


def fetch_segments(cmd):
    for m in WORD.finditer(cmd):
        if STARTS.search(cmd[:m.start()]):
            end = re.search(r'[\n;|&]', cmd[m.end():])
            yield m.group(1), cmd[m.end():m.end() + end.start()] if end else cmd[m.end():]


def host(u):
    try:
        return urlparse(u).hostname or ''
    except ValueError:
        return ''


def transcripts():
    yield 'main session', '', S + '.jsonl'
    for f in sorted(glob.glob(S + '/subagents/agent-*.jsonl') + glob.glob(S + '/subagents/workflows/*/agent-*.jsonl')):
        meta = {}
        m = f[:-len('.jsonl')] + '.meta.json'
        if os.path.exists(m):
            meta = json.load(open(m))
        run = f.split('/workflows/')[1].split('/')[0] if '/workflows/' in f else 'background'
        yield f"{meta.get('agentType', '?')}: {meta.get('description', '')}", run, f


rows = []          # (agent, run, time, tool, requested host, final host, url or note)
for agent, run, f in transcripts():
    if not os.path.exists(f):
        continue
    pending = {}   # tool_use id -> (tool, url)
    for line in open(f):
        try:
            d = json.loads(line)
        except ValueError:
            continue
        t = d.get('timestamp', '')
        msg = d.get('message') or {}
        content = msg.get('content') if isinstance(msg, dict) else None
        if not isinstance(content, list):
            continue
        for c in content:
            if c.get('type') == 'tool_use':
                if since and t < since:
                    continue
                name, inp = c.get('name'), c.get('input') or {}
                if name == 'WebFetch':
                    u = inp.get('url', '')
                    pending[c.get('id')] = ('WebFetch', u, t)
                    rows.append([agent, run, t, 'WebFetch', host(u), host(u), u])
                elif name == 'WebSearch':
                    rows.append([agent, run, t, 'WebSearch', '', '', 'query: ' + str(inp.get('query', ''))[:80]])
                elif name == 'Bash' and any(True for _ in fetch_segments(inp.get('command', ''))):
                    cmd = inp.get('command', '')
                    for tool, seg in fetch_segments(cmd):
                        if '__agentproxy/status' in seg:
                            rows.append([agent, run, t, tool, '(proxy status)', '', 'proxy status check, not a web fetch'])
                            continue
                        us = URL.findall(seg)
                        if not us:
                            rows.append([agent, run, t, tool, '(from a file or variable)', '',
                                         'command: ' + ' '.join(seg.split())[:160]])
                        for u in us:
                            rows.append([agent, run, t, tool, host(u), '', u])
                    pending[c.get('id')] = ('curl', None, t)
            elif c.get('type') == 'tool_result' and c.get('tool_use_id') in pending:
                tool, u, t0 = pending.pop(c.get('tool_use_id'))
                text = c.get('content')
                if isinstance(text, list):
                    text = ' '.join(x.get('text', '') for x in text if isinstance(x, dict))
                text = str(text or '')
                if tool == 'WebFetch':
                    m = re.search(r'redirect[^\n]{0,200}?(https?://[^\s"\')]+)', text, re.I)
                    if m and host(m.group(1)) != host(u):
                        rows.append([agent, run, t0, 'WebFetch (redirect notice)', host(u), host(m.group(1)), m.group(1)])
                else:   # curl -w '%{http_code} %{url_effective}' output, when the command printed it
                    for code, fu in re.findall(r'\b([1-5]\d\d) (https?://\S+)', text):
                        rows.append([agent, run, t0, tool + ' (final URL printed)', '', host(fu), f'{code} {fu}'])

req, fin = collections.defaultdict(collections.Counter), collections.defaultdict(collections.Counter)
off = collections.Counter()
for a, run, t, tool, rh, fh, u in rows:
    base = tool.split(' ')[0]
    if rh and not rh.startswith('('):
        req[base][rh] += 1
        if rh not in allow:
            off[(base, 'requested', rh, a)] += 1
    if fh and fh != rh:
        fin[base][fh] += 1
        if fh not in allow:
            off[(base, 'final (a redirect target, not fetched by the tool)' if 'redirect' in tool else 'final', fh, a)] += 1
print(f'transcripts read; fetch rows {len(rows)}' + (f' since {since}' if since else ''))
for tool in sorted(set(req) | set(fin)):
    print(f'  {tool}: {sum(req[tool].values())} requests to {len(req[tool])} hosts; '
          f'{sum(fin[tool].values())} different final hosts named ({len(fin[tool])} hosts)')
print('hosts off the allow list (tool, requested or final, host, agent: count):')
for (tool, kind, h, a), n in sorted(off.items()):
    print(f'  {tool} · {kind} · {h} · {a}: {n}')
if arg('--md'):
    with open(arg('--md'), 'w') as f:
        f.write('| Agent | Run | Time (UTC) | Tool | Requested host | Final host | URL or note |\n|---|---|---|---|---|---|---|\n')
        for r in rows:
            f.write('| ' + ' | '.join(str(x).replace('|', '\\|') for x in r) + ' |\n')
if arg('--grouped'):   # one row per agent, tool and host pair, with the count: the audit table for the report
    g = collections.Counter()
    for a, run, t, tool, rh, fh, u in rows:
        g[(a, run, tool, rh, fh if fh != rh else '')] += 1
    with open(arg('--grouped'), 'w') as f:
        f.write('| Agent | Run | Tool | Requested host | Final host (if different) | Calls | On the allow list |\n'
                '|---|---|---|---|---|---|---|\n')
        for (a, run, tool, rh, fh), n in sorted(g.items()):
            ok = ('n/a' if rh.startswith('(') or not rh else 'yes' if rh in allow else 'NO') + \
                 (f'; final {"yes" if fh in allow else "NO"}' if fh else '')
            f.write('| ' + ' | '.join(str(x).replace('|', '\\|') for x in (a, run, tool, rh, fh, n, ok)) + ' |\n')
if arg('--json'):
    json.dump({'rows': len(rows), 'requested': {k: dict(v) for k, v in req.items()},
               'final': {k: dict(v) for k, v in fin.items()},
               'off_list': [{'tool': t, 'kind': k, 'host': h, 'agent': a, 'count': n} for (t, k, h, a), n in off.items()]},
              open(arg('--json'), 'w'), indent=1)

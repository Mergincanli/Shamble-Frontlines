# Lists this chat's Bash calls since Mustafa's interim rule (2026-11-25 16:41:51 UTC: "git and gh always start their own
# command, with no other program in front of them") where a git or gh command did not start the call on its own.
# Heredoc bodies and quoted strings are dropped first, so git words inside text, patterns or scripts are not counted.
# Usage: python3 scan_git_calls.py <transcript.jsonl> [since-ISO]
import json, re, sys
path = sys.argv[1]; since = sys.argv[2] if len(sys.argv) > 2 else '2026-11-25T16:41:51'

def strip(cmd):
    out, lines, i = [], cmd.split('\n'), 0
    while i < len(lines):
        line = lines[i]; out.append(line)
        m = re.search(r"<<-?\s*(['\"]?)([A-Za-z_]+)\1", line)
        if m:
            end = m.group(2); i += 1
            while i < len(lines) and lines[i].strip() != end: i += 1
        i += 1
    s = '\n'.join(out)
    s = re.sub(r"'[^']*'", "''", s)
    s = re.sub(r'"(?:[^"\\]|\\.)*"', '""', s)
    return s

SEP = re.compile(r"\|\||&&|[;|\n]|\bdo\b|\bthen\b|\belse\b|\(|`|\$\(")
rows = []
with open(path) as f:
    for line in f:
        try: e = json.loads(line)
        except Exception: continue
        ts = e.get('timestamp', '')
        if ts < since: continue
        c = (e.get('message') or {}).get('content')
        if not isinstance(c, list): continue
        for b in c:
            if b.get('type') != 'tool_use' or b.get('name') != 'Bash': continue
            cmd = b.get('input', {}).get('command', '')
            parts = [p.strip() for p in SEP.split(strip(cmd))]
            parts = [p for p in parts if p]
            firsts = [p.split()[0] for p in parts]
            gits = [i for i, w in enumerate(firsts) if w in ('git', 'gh')]
            if not gits: continue
            if gits == [0] and len(parts) == 1: continue  # a clean call: git or gh alone
            before = firsts[:max(gits)]
            if gits[0] > 0:
                w = firsts[0]
                kind = 'A: ' + ('a variable assignment' if re.match(r'^[A-Za-z_][A-Za-z0-9_]*=', w) else w) + ' starts the call'
            elif len(gits) > 1:
                others = [w for w in before if w not in ('git', 'gh')]
                kind = 'B: git starts the call; a later git follows ' + (others[0] if others else 'another git')
            else:
                kind = 'C: git alone first, other commands after'
            rows.append((ts[:19].replace('T', ' '), kind, cmd.split('\n')[0][:100]))

breach = [r for r in rows if not r[1].startswith('C:')]
after = [r for r in rows if r[1].startswith('C:')]
a = [r for r in breach if r[1].startswith('A:')]; b = [r for r in breach if r[1].startswith('B:')]
print(f"# Git and gh calls since {since} UTC that did not start their own call (this chat only, not its agents)\n")
print(f"Breaches: {len(breach)}. A, something other than git or gh starts the call (cd, a variable assignment, another "
      f"program): {len(a)}. B, git starts the call but a later git or gh command in the same call follows other "
      f"commands: {len(b)}. Not counted, C, git alone first with other commands after it: {len(after)}.\n")
from collections import Counter
for k, n in Counter(r[1] for r in breach).most_common():
    print(f"- {k}: {n}")
print("\n| Time (UTC) | Shape | The call's first line (cut at 100 characters) |\n|---|---|---|")
for r in sorted(a) + sorted(b) + sorted(after):
    print(f"| {r[0]} | {r[1]} | `{r[2].replace('|', '/')}` |")

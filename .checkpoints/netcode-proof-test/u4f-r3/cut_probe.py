"""Read-only probe for the cut of 2026-11-17 evening: print every main-chain transcript entry after 19:30 UTC whose
text names a limit or a restart (its line, type, timestamp, and the text, cut at 400 characters), the first and last
main-chain entries after 19:45, and the cut workflow's journal lines (wf_b2a20e74-6aa) with their keys. Writes
nothing. Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/u4f-r3/cut_probe.py"""
import json
import re

T = "/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2.jsonl"
J = ("/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/subagents/workflows/"
     "wf_b2a20e74-6aa/journal.jsonl")
A = ("/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/subagents/workflows/"
     "wf_b2a20e74-6aa/agent-a69a9c748b8ff12bf.jsonl")
PAT = re.compile(r"spend limit|limit resets|usage limit|rate limit|worker process|restarted", re.I)


def text_of(r):
    m = r.get("message")
    c = m.get("content") if isinstance(m, dict) else r.get("content")
    if isinstance(c, str):
        return c
    if isinstance(c, list):
        out = []
        for b in c:
            if isinstance(b, dict):
                if b.get("type") == "text":
                    out.append(b.get("text", ""))
                elif b.get("type") == "tool_result":
                    cc = b.get("content")
                    out.append(cc if isinstance(cc, str) else json.dumps(cc)[:2000])
        return "\n".join(out)
    return ""


after = []
for n, line in enumerate(open(T, encoding="utf-8", errors="replace"), 1):
    try:
        r = json.loads(line, strict=False)
    except Exception:
        continue
    if not isinstance(r, dict) or r.get("isSidechain"):
        continue
    ts = r.get("timestamp") or ""
    if ts < "2026-11-17T19:30":
        continue
    t = text_of(r)
    if ts >= "2026-11-17T19:45":
        after.append((n, r.get("type"), r.get("subtype"), ts))
    if PAT.search(t) or PAT.search(json.dumps(r)[:4000]):
        hit = PAT.search(t) or PAT.search(json.dumps(r)[:4000])
        s = (t if PAT.search(t) else json.dumps(r)[:4000])
        i = max(0, hit.start() - 150)
        print(f"[line {n} {r.get('type')}/{r.get('subtype')} {ts}] ...{s[i:i + 400]!r}")
print("main-chain entries from 19:45:", len(after))
for row in after[:6] + [("...",)] + after[-6:]:
    print(" ", row)
print("journal:")
for line in open(J, encoding="utf-8"):
    r = json.loads(line)
    print(" ", {k: (v if not isinstance(v, (dict, list, str)) or len(json.dumps(v)) < 300 else json.dumps(v)[:300])
                for k, v in r.items()})
print("advisor agent's last entries:")
rows = [json.loads(x, strict=False) for x in open(A, encoding="utf-8", errors="replace")]
for r in rows[-4:]:
    print(" ", r.get("type"), r.get("timestamp"), repr(text_of(r)[:400]))

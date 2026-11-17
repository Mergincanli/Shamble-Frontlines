"""Find the usage-limit notices of this session's transcript after a given UTC time, with their timestamps, and the
last entry before the first of them (the cut) and the first user entry after the last of them (the resume).
Read-only; writes nothing. Run from the repo root:
  python3 -I .checkpoints/netcode-proof-test/cut_notice_find.py 2026-11-17T11:50"""
import json
import sys

T = "/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2.jsonl"
SINCE = sys.argv[1]
KEYS = ("hit your", "usage limit", "limit resets", "spend limit")


def text_of(r):
    m = r.get("message")
    c = m.get("content") if isinstance(m, dict) else r.get("content")
    if isinstance(c, str):
        return c
    if isinstance(c, list):
        return " ".join(b.get("text", "") for b in c if isinstance(b, dict) and isinstance(b.get("text"), str))
    return ""


rows = []
with open(T, encoding="utf-8", errors="replace") as fh:
    for n, line in enumerate(fh, 1):
        try:
            r = json.loads(line, strict=False)
        except Exception:
            continue
        if isinstance(r, dict) and (r.get("timestamp") or "") >= SINCE:
            rows.append((n, r))
hits = [(n, r) for n, r in rows if r.get("type") in ("assistant", "system")
        and any(k in text_of(r) for k in KEYS) and len(text_of(r)) < 600]
for n, r in hits:
    print(f"NOTICE line {n} {r.get('timestamp')} type {r.get('type')}: {text_of(r)!r}")
if hits:
    first = hits[0][0]
    before = [(n, r) for n, r in rows if n < first]
    if before:
        n, r = before[-1]
        print(f"LAST BEFORE line {n} {r.get('timestamp')} type {r.get('type')}")
    after = [(n, r) for n, r in rows if n > hits[-1][0] and r.get("type") == "user" and not r.get("isSidechain")]
    for n, r in after[:3]:
        print(f"USER AFTER line {n} {r.get('timestamp')}: {text_of(r)[:160]!r}")
for n, r in rows:
    if r.get("type") == "user" and not r.get("isSidechain") and "usage limit" in text_of(r):
        print(f"USER LIMIT MSG line {n} {r.get('timestamp')}: {text_of(r)[:200]!r}")
PHRASE = sys.argv[2] if len(sys.argv) > 2 else None
if PHRASE:
    with open(T, encoding="utf-8", errors="replace") as fh:
        for n, line in enumerate(fh, 1):
            if PHRASE in line:
                try:
                    r = json.loads(line, strict=False)
                except Exception:
                    r = {}
                i = line.find(PHRASE)
                print(f"PHRASE line {n} {r.get('timestamp')} type {r.get('type')} sub {r.get('subtype')}: "
                      f"{line[max(0, i - 160):i + 120]!r}")
print(f"{len(hits)} notice(s) since {SINCE}")

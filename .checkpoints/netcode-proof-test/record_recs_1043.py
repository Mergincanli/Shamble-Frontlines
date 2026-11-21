"""Record Mustafa's words of 2026-11-17 10:43 UTC word for word in history.md: his message (transcript uuid 90173405-...)
and the Decisions-needed blocks it answers, copied from this session's transcript (the latest full block, the one that
posed PACK-READ, and the latest block before his message). Refuses to run twice.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/record_recs_1043.py"""
import hashlib
import json

T = "/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2.jsonl"
H = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/history.md"
UUID = "90173405-9d31-4247-8d81-4a46d2902961"
SAID = "I agree with all your recommendations, continue"
HEADING = ("## Decision: Mustafa, 2026-11-17 11:43 Amsterdam (10:43 UTC), RECS-1043 (PACK-READ, DOCS-54, C62HB-Q4, "
           "NS-TRAILER and NS-FIRST, as recommended)")


def text_of(r):
    c = (r.get("message") or {}).get("content")
    if isinstance(c, str):
        return c
    if isinstance(c, list):
        return "".join(b.get("text", "") for b in c if isinstance(b, dict) and b.get("type") == "text")
    return ""


def block(t):
    s = t.index("**Decisions needed:**")
    e = t.index("**For Mustafa:**", s)
    return t[s:e].rstrip()


rows = []
with open(T, encoding="utf-8", errors="replace") as fh:
    for line in fh:
        try:
            r = json.loads(line, strict=False)
        except Exception:
            continue
        if isinstance(r, dict):
            rows.append(r)
idx = [i for i, r in enumerate(rows) if r.get("uuid") == UUID]
assert len(idx) == 1, idx
u = rows[idx[0]]
assert u.get("type") == "user" and text_of(u) == SAID, repr(text_of(u))
full = last = None
for r in reversed(rows[: idx[0]]):
    if r.get("type") != "assistant" or r.get("isSidechain"):
        continue
    t = text_of(r)
    if "**Decisions needed:**" not in t or "**For Mustafa:**" not in t:
        continue
    if last is None:
        last = (r.get("timestamp"), block(t))
    if "**PACK-READ (new, optional):**" in t:
        full = (r.get("timestamp"), block(t))
        break
assert full and last, (full, last)
for code in ("PACK-READ", "DOCS-54", "C62HB-Q4", "NS-TRAILER", "NS-FIRST"):
    assert code in full[1] and code in last[1], code
with open(H, encoding="utf-8") as fh:
    old = fh.read()
assert HEADING not in old, "already recorded"


def indent(b):
    return "\n".join(("  " + x) if x else "" for x in b.split("\n"))


rec = (
    f"\n{HEADING}\n"
    f"- **Mustafa, typed into this chat (transcript {u.get('timestamp')}), verbatim:** \"{SAID}\"\n"
    f"- **The recommendations as last posed in full** (the assistant's message at {full[0]}, its Decisions-needed "
    f"block, copied by this script):\n\n{indent(full[1])}\n\n"
    f"- **As last repeated before his message** (the assistant's message at {last[0]}, copied by this script):\n\n"
    f"{indent(last[1])}\n\n"
    "- **Read as** (the main session's reading, not his words): PACK-READ (a), DOCS-54 (a), C62HB-Q4 (b), NS-TRAILER "
    "(b), NS-FIRST (b). The optional re-judge of U4r's own P0 set (051995db9cea) carried no recommendation, so it is "
    "not covered.\n"
)
new = old + rec
with open(H, "w", encoding="utf-8") as fh:
    fh.write(new)
print(f"history.md: {old.count(chr(10))} -> {new.count(chr(10))} lines; record sha256 "
      f"{hashlib.sha256(rec.encode()).hexdigest()[:16]}; full block at {full[0]}, last block at {last[0]}")

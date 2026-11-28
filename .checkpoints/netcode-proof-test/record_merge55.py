"""Record Mustafa's go on PR #55 word for word in history.md: his message (transcript uuid 88423842-...) and the
assistant's line it answered (the last one before it), both copied from this session's transcript, then the merge's
record. Refuses to run twice. Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/record_merge55.py"""
import hashlib
import json
import re

T = "/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2.jsonl"
H = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/history.md"
UUID = "88423842-2b09-4986-bab4-20580f7d5c91"
HEADING = "## Decision: Mustafa, 2026-11-17 18:08 Amsterdam (17:08 UTC), MERGE-55 (merge #55 at aea6f5b)"


def text_of(r):
    c = (r.get("message") or {}).get("content")
    if isinstance(c, str):
        return c
    if isinstance(c, list):
        return "".join(b.get("text", "") for b in c if isinstance(b, dict) and b.get("type") == "text")
    return ""


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
assert u.get("type") == "user" and not u.get("isSidechain"), (u.get("type"), u.get("isSidechain"))
said = text_of(u)
assert said == "merge #55", repr(said)
ask = None
for r in reversed(rows[: idx[0]]):
    if r.get("type") == "assistant" and not r.get("isSidechain"):
        m = re.search(r"^.*type \*\*merge #55\*\*.*$", text_of(r), re.M)
        if m:
            ask = (r.get("timestamp"), m.group(0).strip())
            break
assert ask, "no asking line"
with open(H, encoding="utf-8") as fh:
    old = fh.read()
assert HEADING not in old, "already recorded"
assert old.endswith("so it is not covered.\n"), repr(old[-40:])
block = (
    f"\n{HEADING}\n"
    f"- **Mustafa, typed into this chat (transcript {u.get('timestamp')}), verbatim:** \"{said}\"\n"
    f"- **What it answered** (the assistant's message at {ask[0]}, its \"For Mustafa\" line, copied by this script): "
    f"{ask[1]}\n"
    "- **The merge** (the day session, after its checks: head aea6f5b unchanged, the advisor agent's APPROVE WITH "
    "FINDINGS of that head in this session and the verifier's PASS, six check runs green, main still 82c605e): "
    "c94c81b440c77f7979e0f944a5341319f9a57d1b, a merge commit, 2026-11-17 17:09:32 UTC, expected head "
    "aea6f5bd67a4da400613cc24919e3e93e7c8dc21; main's tree equals that head's; CI on main green (gate run 44133861553, "
    "its gate and smoke jobs; toolchain run 44133861577).\n"
)
new = old + block
with open(H, "w", encoding="utf-8") as fh:
    fh.write(new)
print(f"history.md: {old.count(chr(10))} -> {new.count(chr(10))} lines; block sha256 "
      f"{hashlib.sha256(block.encode()).hexdigest()[:16]}")
print(block)

"""Record Mustafa's go on PR #54 word for word in history.md: his message (transcript uuid 50b71aa0-...) and the
assistant's line it answered, both copied from this session's transcript, then the merge's record. Refuses to run twice.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/record_merge54.py"""
import hashlib
import json
import re

T = "/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2.jsonl"
H = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/history.md"
UUID = "50b71aa0-f12b-4f33-b170-1fe80a5ee722"
HEADING = "## Decision: Mustafa, 2026-11-17 08:00 Amsterdam (07:00 UTC), MERGE-54 (merge #54 at 20f9d59)"


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
assert u.get("type") == "user", u.get("type")
said = text_of(u)
assert said == "Merge #54", repr(said)
ask = None
for r in reversed(rows[: idx[0]]):
    if r.get("type") == "assistant" and not r.get("isSidechain"):
        m = re.search(r'^.*type "merge #54" in this chat.*$', text_of(r), re.M)
        if m:
            ask = (r.get("timestamp"), m.group(0).strip())
            break
assert ask, "no asking line"
with open(H, encoding="utf-8") as fh:
    old = fh.read()
assert HEADING not in old, "already recorded"
assert old.endswith("```\n"), repr(old[-20:])
block = (
    f"\n{HEADING}\n"
    f"- **Mustafa, typed into this chat (transcript {u.get('timestamp')}), verbatim:** \"{said}\"\n"
    f"- **What it answered** (the assistant's message at {ask[0]}, its \"For Mustafa\" line, copied by this script): "
    f"{ask[1]}\n"
    "- **The merge** (the day session, after its checks: head 20f9d59 unchanged, the advisor's and the "
    "security-auditor's APPROVE WITH FINDINGS and the PR Advisor routine's APPROVE on that head, six check runs green, "
    "main still 53272a1): 82c605e2e8a7f284e6925718368d3fe569f89ae9, a merge commit, 2026-11-17 07:03:02 UTC, expected "
    "head 20f9d59918eea1e3431ae830f72510f83f6424c7; main's tree equals that head's; CI on main green (gate run "
    "44035715175, toolchain run 44035715349).\n"
)
new = old + block
with open(H, "w", encoding="utf-8") as fh:
    fh.write(new)
print(f"history.md: {old.count(chr(10))} -> {new.count(chr(10))} lines; block sha256 "
      f"{hashlib.sha256(block.encode()).hexdigest()[:16]}")
print(block)

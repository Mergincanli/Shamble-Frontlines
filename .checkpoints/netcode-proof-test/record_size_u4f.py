"""Record Mustafa's answer to SIZE-U4F (his message holding "My words of 2026-11-19, 12:00 (Amsterdam time): SIZE-U4F:
(a).") word for word in history.md, from this session's transcript, in a fenced block: the whole typed message,
unchanged. Dry run by default (prints what it found); --apply writes. Refuses to run twice. Run from the repo root:
python3 -I .checkpoints/netcode-proof-test/record_size_u4f.py [--apply]"""
import hashlib
import json
import sys

T = "/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2.jsonl"
H = ".checkpoints/netcode-proof-test/history.md"
KEY = "My words of 2026-11-19, 12:00 (Amsterdam time): SIZE-U4F: (a)."
END = "No pass mark, judge rule, marks.json text or run count changes."


def text_of(r):
    c = (r.get("message") or {}).get("content")
    if isinstance(c, str):
        return c
    if isinstance(c, list):
        return "".join(b.get("text", "") for b in c if isinstance(b, dict) and b.get("type") == "text")
    return ""


hits = []
for line in open(T, encoding="utf-8", errors="replace"):
    try:
        r = json.loads(line, strict=False)
    except Exception:
        continue
    if (isinstance(r, dict) and r.get("type") == "user" and not r.get("isSidechain") and not r.get("isMeta")
            and KEY in text_of(r)):
        hits.append((r.get("timestamp"), r.get("uuid"), text_of(r)))
print("hits:", [(h[0], h[1], len(h[2])) for h in hits])
assert len(hits) == 1, "expected exactly one message"
ts, uuid, said = hits[0]
said = said.strip("\n")
print(f"message: {said.count(chr(10)) + 1} lines, {len(said)} chars, sha256 {hashlib.sha256(said.encode()).hexdigest()}")
print("starts:", repr(said[:160]))
print("ends:  ", repr(said[-160:]))
print("markup inside:", [m for m in ("<system-reminder>", "<local-command", "<command-", "```") if m in said])
assert said.endswith(END), "the message does not end with the expected sentence"
assert "```" not in said and "<system-reminder>" not in said and "<local-command" not in said
old = open(H, encoding="utf-8").read()
assert KEY not in old, "already recorded"
heading = (f"## Decision: Mustafa, 2026-11-19 12:00 Amsterdam by his own label (it reached this chat at {ts[11:19]} "
           "UTC), SIZE-U4F (a): close U4f with what is built; the forged-command test's rest and -21's runs a unit of "
           "their own")
block = (
    f"\n{heading}\n"
    f"- **Mustafa, typed into this chat (transcript {ts}, uuid {uuid}), verbatim** (the whole message, unchanged, in the "
    "fence below):\n\n"
    "```text\n" + said + "\n```\n"
)
if "--apply" not in sys.argv:
    print("dry run: nothing written; the heading would be:")
    print(heading)
    sys.exit(0)
open(H, "w", encoding="utf-8").write(old + block)
new = old + block
start = old.count("\n") + 2
print(f"history.md: {old.count(chr(10))} -> {new.count(chr(10))} lines; the block at :{start}-{new.count(chr(10))}")
print(heading)

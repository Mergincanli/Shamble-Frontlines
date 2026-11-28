"""Record Mustafa's answer to LEAD-BOUND (his message starting "My words of 2026-11-18, 11:31 (Amsterdam time)") word
for word in history.md, from this session's transcript, in a fenced block; and the freeze: the entry u4f-lead-bound.md
asserted unchanged since 9ad2effb, its fenced value's sha256 printed for the card. Refuses to run twice. Run from the
repo root:  python3 -I .checkpoints/netcode-proof-test/record_lead_bound.py"""
import hashlib
import json
import subprocess

T = "/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2.jsonl"
H = ".checkpoints/netcode-proof-test/history.md"
E = ".checkpoints/netcode-proof-test/u4f-lead-bound.md"
START = "My words of 2026-11-18, 11:31 (Amsterdam time): my answer to LEAD-BOUND"
END = "past 6,000 added lines the build stops and I get three lines."


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
    if isinstance(r, dict) and r.get("type") == "user" and not r.get("isSidechain") and text_of(r).startswith(START):
        hits.append((r.get("timestamp"), r.get("uuid"), text_of(r)))
assert len(hits) == 1, [h[:2] for h in hits]
ts, uuid, said = hits[0]
said = said.rstrip("\n")
assert said.endswith(END), repr(said[-80:])
assert "```" not in said
then = subprocess.run(["git", "show", "9ad2effb:" + E], capture_output=True, check=True).stdout
now = open(E, "rb").read()
assert then == now, "u4f-lead-bound.md changed since 9ad2effb"
fence = now.decode("utf-8").split("```text\n", 1)[1].split("```", 1)[0]
fence_sha = hashlib.sha256(fence.encode()).hexdigest()
old = open(H, encoding="utf-8").read()
assert START not in old, "already recorded"
heading = (f"## Decision: Mustafa, 2026-11-18 11:31 Amsterdam by his own label (it reached this chat at {ts[11:19]} "
           "UTC), LEAD-BOUND (a) 400 ms, A1's movement reading, build U4f")
block = (
    f"\n{heading}\n"
    f"- **Mustafa, typed into this chat (transcript {ts}, uuid {uuid}), verbatim** (the whole message, unchanged, in the "
    "fence below):\n\n"
    "```text\n" + said + "\n```\n\n"
    f"- **The freeze** (the main session's record): u4f-lead-bound.md equal to its text at 9ad2effb (sha256 "
    f"{hashlib.sha256(now).hexdigest()}); its fenced value's sha256 {fence_sha}.\n"
)
open(H, "w", encoding="utf-8").write(old + block)
print(f"history.md: {old.count(chr(10))} -> {(old + block).count(chr(10))} lines; message {said.count(chr(10)) + 1} "
      f"lines, sha256 {hashlib.sha256(said.encode()).hexdigest()[:16]}; the entry {hashlib.sha256(now).hexdigest()[:16]}; "
      f"the fenced value {fence_sha}")
print(heading)

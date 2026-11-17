"""DOCS-54, read-only probe for the copy script: for each stamp, find the transcript record(s) that carry that message
(a plain user entry, or a queued_command attachment and its queue-operation), and print the line number, type, uuid,
and the text's length, sha256 and first and last 60 characters. Writes nothing. Run from the repo root:
  python3 -I .checkpoints/netcode-proof-test/docs-54/docs54_probe.py"""
import hashlib
import json

T = "/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2.jsonl"
STAMPS = {
    "E9": "2026-11-15T12:01:41.136Z", "E8": "2026-11-15T12:34:01.309Z", "E7": "2026-11-15T15:34:42.893Z",
    "GO53": "2026-11-15T18:42:53.995Z", "E6": "2026-11-15T16:21:11.362Z", "E5": "2026-11-15T19:46:36.849Z",
    "E4": "2026-11-16T20:48:10.683Z", "E3": "2026-11-16T21:09:27.492Z", "E2": "2026-11-17T07:00:54.917Z",
    "E1": "2026-11-17T10:43:48.016Z",
    "A-E3q": "2026-11-16T20:50:16.246Z", "A-E2q": "2026-11-17T06:34:50.318Z", "A-E1q": "2026-11-17T08:54:58.700Z",
    "A-E1r": "2026-11-17T10:35:38.816Z",
}


def texts(r):
    """Every text this record carries: message content (string or text blocks), or an attachment's prompt."""
    out = []
    m = r.get("message")
    c = m.get("content") if isinstance(m, dict) else None
    if isinstance(c, str):
        out.append(("content", c))
    elif isinstance(c, list):
        for b in c:
            if isinstance(b, dict) and b.get("type") == "text" and isinstance(b.get("text"), str):
                out.append(("text-block", b["text"]))
    a = r.get("attachment")
    if isinstance(a, dict) and isinstance(a.get("prompt"), str):
        out.append(("attachment:" + str(a.get("type")), a["prompt"]))
    if r.get("type") == "queue-operation" and isinstance(r.get("content"), str):
        out.append(("queue:" + str(r.get("operation")), r["content"]))
    return out


want = {v: k for k, v in STAMPS.items()}
with open(T, encoding="utf-8", errors="replace") as fh:
    for n, line in enumerate(fh, 1):
        try:
            r = json.loads(line, strict=False)
        except Exception:
            continue
        if not isinstance(r, dict):
            continue
        ts = r.get("timestamp")
        if ts in want:
            for kind, t in texts(r):
                h = hashlib.sha256(t.encode("utf-8")).hexdigest()
                print(f"{want[ts]} line {n} type {r.get('type')} uuid {r.get('uuid')} side {r.get('isSidechain')} "
                      f"{kind} len {len(t)} sha256 {h[:16]} first {t[:60]!r} last {t[-60:]!r}")

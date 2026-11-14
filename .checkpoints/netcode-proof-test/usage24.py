"""Sum token usage of this session over a time window, by transcript (main session, each subagent, each workflow agent).
  python3 -I usage24.py <session dir> <session jsonl> <since ISO> [<until ISO>]
Counts each API response once (deduped by requestId, else message id). Weighted = input + 1.25 x 5-minute cache write
+ 2 x 1-hour cache write + 0.1 x cache read + 5 x output (API price ratios; an approximation of what a limit counts)."""
import glob, json, os, sys

sdir, main_jsonl, since = sys.argv[1], sys.argv[2], sys.argv[3]
until = sys.argv[4] if len(sys.argv) > 4 else "9999"


def scan(path):
    seen, tot, first, last = set(), [0, 0, 0, 0, 0, 0], None, None
    with open(path, encoding="utf-8", errors="replace") as fh:
        for line in fh:
            if '"usage"' not in line:
                continue
            try:
                e = json.loads(line)
            except ValueError:
                continue
            if e.get("type") != "assistant":
                continue
            ts = e.get("timestamp") or ""
            if not (since <= ts < until):
                continue
            m = e.get("message") or {}
            u = m.get("usage") or {}
            key = e.get("requestId") or m.get("id")
            if key in seen:
                continue
            seen.add(key)
            cc = u.get("cache_creation") or {}
            c5 = cc.get("ephemeral_5m_input_tokens")
            c1 = cc.get("ephemeral_1h_input_tokens")
            if c5 is None and c1 is None:
                c5, c1 = 0, u.get("cache_creation_input_tokens", 0) or 0
            vals = [u.get("input_tokens", 0) or 0, c5 or 0, c1 or 0, u.get("cache_read_input_tokens", 0) or 0,
                    u.get("output_tokens", 0) or 0, 1]
            tot = [a + b for a, b in zip(tot, vals)]
            first = first or ts
            last = ts
    w = tot[0] + 1.25 * tot[1] + 2 * tot[2] + 0.1 * tot[3] + 5 * tot[4]
    return tot, w, first, last


rows = []
t, w, f, l = scan(main_jsonl)
rows.append(("MAIN", "main session", "", t, w, f, l))
for p in sorted(glob.glob(f"{sdir}/subagents/agent-*.jsonl") + glob.glob(f"{sdir}/subagents/workflows/*/agent-*.jsonl")):
    meta = {}
    mp = p[:-len(".jsonl")] + ".meta.json"
    if os.path.exists(mp):
        with open(mp, encoding="utf-8") as fh:
            meta = json.load(fh)
    t, w, f, l = scan(p)
    if t[5] == 0:
        continue
    wf = os.path.basename(os.path.dirname(p)) if "/workflows/" in p else ""
    rows.append((meta.get("agentType", "?"), meta.get("description", "?"), wf, t, w, f, l))

grand = sum(r[4] for r in rows)
print(f"window {since} .. {until}; transcripts with calls: {len(rows)}; weighted total {grand/1e6:.1f} M")
for r in sorted(rows, key=lambda r: -r[4]):
    t = r[3]
    print(f"{r[4]/grand*100:5.1f}% {r[4]/1e6:7.2f}M calls={t[5]:4d} out={t[4]/1e3:6.0f}k rd={t[3]/1e6:7.1f}M "
          f"wr={(t[1]+t[2])/1e6:5.2f}M | {r[0][:18]:18s} | {r[1][:46]:46s} | {r[2]} | {(r[5] or '')[5:16]}")

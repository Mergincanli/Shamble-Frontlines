#!/usr/bin/env python3
"""C5 sweep (verifier's own): his typed messages in the transcript from 2026-11-14T09:02Z on. Prints only.
Two transcript lines (10327, 10376, both 2026-11-08) are not valid JSON; they fall before the window and are skipped."""
import json

TR = "/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2.jsonl"
SINCE = "2026-11-14T09:02"
SKIP = ("<task-notification>", "This session is being continued", "<local-command", "<command-name>",
        "<command-message>", "<system-reminder>", "[Request interrupted", "Caveat:", "<user-prompt-submit-hook>",
        "<bash-input>", "<bash-stdout>")
out, bad = [], []
with open(TR, encoding="utf-8") as f:
    for n, line in enumerate(f, 1):
        if '"type":"user"' not in line.replace(" ", "") and '"queued_command"' not in line:
            continue
        try:
            d = json.loads(line)
        except ValueError:
            bad.append(n)
            continue
        ts = d.get("timestamp") or ""
        if ts < SINCE or d.get("isSidechain"):
            continue
        if d.get("type") == "attachment":
            a = d.get("attachment", {})
            if a.get("type") != "queued_command":
                continue
            t = a.get("prompt")
            if not isinstance(t, str):
                t = json.dumps(t)[:300]
            kind = "queued"
        elif d.get("type") == "user":
            if d.get("isMeta") or d.get("isCompactSummary") or d.get("toolUseResult") is not None:
                continue
            c = d.get("message", {}).get("content")
            if isinstance(c, list):
                if any(x.get("type") == "tool_result" for x in c):
                    continue
                t = "\n".join(x.get("text", "") for x in c if x.get("type") == "text")
            else:
                t = c or ""
            kind = "user"
        else:
            continue
        ori = (d.get("origin") or d.get("attachment", {}).get("origin") or {}).get("kind")
        sk = next((s for s in SKIP if t.lstrip().startswith(s) or s in t[:200]), None)
        out.append((n, ts, kind, ori, sk, d.get("uuid"), t))
print("unparsable lines:", bad)
for n, ts, kind, ori, sk, u, t in out:
    flag = ("SKIP:" + sk) if sk else ""
    print("%6d %s %-6s origin=%s %s %s | %s" % (n, ts, kind, ori, u, flag, t[:150].replace("\n", " / ")))
print("total", len(out), "not skipped", sum(1 for x in out if not x[4]))

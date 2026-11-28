"""Hash every pass-mark row of the netcode proof test at the drafts commit.

Usage: python3 -I row_hashes.py <worktree root> <out.json>
A row is the whole line starting with '| <id> |', without its line end.
"""
import hashlib, json, re, sys

root, out = sys.argv[1], sys.argv[2]
FILES = {
    "docs/adr/0001-multiplayer-netcode.md": [f"PM-0001-{i}" for i in range(1, 64)],
    "docs/adr/0004-simulation-core.md": ["PM-0004-67"],
    "docs/adr/0006-infrastructure-operations.md": [f"PM-0006-{i}" for i in range(11, 28)],
}
res = {}
for path, ids in FILES.items():
    with open(f"{root}/{path}", "rb") as fh:
        lines = fh.read().decode("utf-8").split("\n")
    for mid in ids:
        hits = [(n + 1, l.rstrip("\r")) for n, l in enumerate(lines) if l.startswith(f"| {mid} |")]
        if len(hits) != 1:
            raise SystemExit(f"{mid}: {len(hits)} rows")
        n, line = hits[0]
        res[mid] = {"file": path, "line": n,
                    "row_sha256": hashlib.sha256(line.encode("utf-8")).hexdigest()}
json.dump(res, open(out, "w"), indent=1)
print(len(res), "rows hashed")

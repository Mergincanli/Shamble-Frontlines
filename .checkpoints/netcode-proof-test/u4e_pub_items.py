"""T6's precondition (advisor-u4e-r1.md :682-687): re-derive from the merged tree the pub items U4e adds in npt-room,
npt-server and npt-client (`git diff 28a85e7 -- <the three crates' src>`: every added `pub` item, field and method),
less plan.md §4.7's own items, and check each name against T6's new §4.1 rows (advisor-u4e-r1.md :694, :703, :711).
Read only; prints the names and whether each is in its crate's row. Run from the repo root:
  python3 -I .checkpoints/netcode-proof-test/u4e_pub_items.py"""
import re
import subprocess

R1 = open(".checkpoints/netcode-proof-test/advisor-u4e-r1.md", encoding="utf-8").read().split("\n")
PLAN = open(".checkpoints/netcode-proof-test/u4-build-plan.md", encoding="utf-8").read().split("\n")
srv = [l for l in PLAN if l.startswith("| npt-server |")]
assert len(srv) == 1, srv
# T6 replaces the npt-server row's end only (from "bites builds only:"); its start, kept, names `Decoder::pump`
ROWS = {"room": R1[693], "server": srv[0] + " " + R1[702], "client": R1[710]}
assert ROWS["room"].startswith("| npt-room |") and ROWS["client"].startswith("| npt-client |"), "row lines moved"
# plan.md §4.7's own items (plan.md:596-600): npt-room's eleven, npt-server's and npt-client's
OWN = {"room": {"Room", "Roster", "ClientSlot", "InputBuffer", "EventQueue", "AckRing", "Encoder", "InterestGrid",
                "AreaFlood", "Priority", "rewind_at"},
       "server": {"Counters", "ConnLimits", "Decoder"}, "client": {"ClientEndpoint", "StepOut"}}
PAT = re.compile(r"^\+\s*pub(?:\([^)]*\))?\s+(?:unsafe\s+)?(?:const\s+)?(fn|struct|enum|const|type|trait|static|mod|use)"
                 r"\s+([A-Za-z_]\w*)|^\+\s*pub\s+([a-z_]\w*)\s*:")
USE = re.compile(r"^\+\s*pub\s+use\s+([^;]+);")
for crate in ("room", "server", "client"):
    src = f"spikes/netcode-proof/crates/{crate}/src"
    d = subprocess.run(["git", "diff", "28a85e7", "--", src], capture_output=True, text=True, check=True).stdout
    added = [l for l in d.split("\n") if l.startswith("+") and not l.startswith("+++")]
    # the apply leaves new files untracked, which `git diff <commit>` does not show: read them whole as added lines
    new = subprocess.run(["git", "ls-files", "--others", "--exclude-standard", "--", src], capture_output=True,
                         text=True, check=True).stdout.split()
    for f in new:
        added += ["+" + l for l in open(f, encoding="utf-8").read().split("\n")]
    names = []
    for line in added:
        if re.match(r"^\+\s*pub\(", line):
            continue  # pub(crate) and pub(super) are not public
        u = USE.match(line)
        if u:  # a re-export: its exported names, the last segment of each path
            body = u.group(1)
            inner = re.search(r"\{([^}]*)\}", body)
            parts = inner.group(1).split(",") if inner else [body]
            for p in parts:
                p = p.strip()
                if p:
                    names.append(("use", re.split(r"\s+as\s+", p)[-1].split("::")[-1].strip()))
            continue
        m = PAT.match(line)
        if m:
            names.append((m.group(1) or "field", m.group(2) or m.group(3)))
    print(f"npt-{crate}: {len(new)} new files read whole")
    seen, missing, fields = set(), [], []
    for kind, n in names:
        if (kind, n) in seen:
            continue
        seen.add((kind, n))
        if n in OWN[crate]:
            continue
        if kind == "field":
            fields.append(n)
            continue
        inrow = re.search(r"\b" + re.escape(n) + r"\b", ROWS[crate]) is not None
        if not inrow:
            missing.append(f"{kind} {n}")
    group = {"room": "`ClientCounters` ... its fields the per-client counts", "server": "`Counters`' fields",
             "client": "`StepOut`'s fields"}[crate]
    assert not fields or group.split(" ")[0].strip("`'") in ROWS[crate] or "fields" in ROWS[crate], crate
    print(f"npt-{crate}: {len(seen)} added pub names; {len(fields)} of them fields (named as a group in the row: "
          f"{group}); not in T6's row nor §4.7's own: {', '.join(missing) if missing else 'none'}")

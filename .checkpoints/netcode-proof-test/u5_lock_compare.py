"""Compare U5's fuzz lock with the workspace lock after the networked fetch (advisor-u5-early.md condition 3; the
builder's expectation: only the fuzz crate's new packages are added, packages outside its graph are dropped, and no
package both locks keep changes version or checksum). Reads only. Usage, from the repo root:
python3 -I .checkpoints/netcode-proof-test/u5_lock_compare.py <workspace Cargo.lock> <fuzz Cargo.lock>"""
import re
import sys


def packages(path):
    out = {}
    for blk in open(path, encoding="utf-8").read().split("[[package]]")[1:]:
        name = re.search(r'^name = "([^"]+)"', blk, re.M).group(1)
        ver = re.search(r'^version = "([^"]+)"', blk, re.M).group(1)
        src = re.search(r'^source = "([^"]+)"', blk, re.M)
        chk = re.search(r'^checksum = "([^"]+)"', blk, re.M)
        out[(name, ver)] = (src.group(1) if src else "path", chk.group(1) if chk else None)
    return out


ws, fz = packages(sys.argv[1]), packages(sys.argv[2])
added = sorted(k for k in fz if k not in ws)
dropped = sorted(k for k in ws if k not in fz)
changed = sorted(k for k in fz if k in ws and fz[k] != ws[k])
ws_names = {n for n, _ in ws}
new_version_of_known = sorted(k for k in added if k[0] in ws_names)
print(f"workspace lock: {len(ws)} packages; fuzz lock: {len(fz)} packages; kept in both: {len(set(ws) & set(fz))}")
print(f"added ({len(added)}): " + ", ".join(f"{n} {v} [{fz[(n, v)][0].split('+')[0]}]" for n, v in added))
print(f"dropped ({len(dropped)}): " + ", ".join(f"{n} {v}" for n, v in dropped))
print(f"source or checksum changed for a kept package ({len(changed)}): " + ", ".join(f"{n} {v}" for n, v in changed))
print(f"a new version of a package the workspace lock already holds ({len(new_version_of_known)}): "
      + ", ".join(f"{n} {v}" for n, v in new_version_of_known))

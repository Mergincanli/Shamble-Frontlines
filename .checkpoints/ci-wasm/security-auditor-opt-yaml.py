# security-auditor, OPT-PATH reading, 2026-11-23: parse cb476b1's toolchain.yml and the proposed one, list every
# parsed difference, and cut the install step's run text (from the `case` line on) for the local tries.
# Run: python3 -B /home/user/Shamble-Frontlines/.gate/ci-wasm-wt/.checkpoints/ci-wasm/security-auditor-opt-yaml.py
import hashlib
import yaml

S = "/home/user/Shamble-Frontlines/.gate/ci-wasm-wt/.gate/ci-wasm/sa-opt-path"
OLD = S + "/orig-cb476b1.yml"
NEW = "/home/user/Shamble-Frontlines/.gate/ci-wasm-wt/.gate/ci-wasm/proposed-2/toolchain.yml"
INFRA_CUT = "/home/user/Shamble-Frontlines/.gate/ci-wasm-wt/.gate/ci-wasm/opt-path/new-lines-as-parsed.sh"
OUT = S + "/own-cut.sh"


def load(path):
    with open(path, "rb") as f:
        raw = f.read()
    print(path, hashlib.sha256(raw).hexdigest(), len(raw), "bytes")
    return yaml.safe_load(raw.decode("utf-8"))


def walk(a, b, where, out):
    if type(a) is not type(b):
        out.append((where, "type", a, b))
    elif isinstance(a, dict):
        for k in sorted(set(a) | set(b), key=str):
            if k not in a or k not in b:
                out.append((where + "." + str(k), "missing on one side", a.get(k), b.get(k)))
            else:
                walk(a[k], b[k], where + "." + str(k), out)
    elif isinstance(a, list):
        if len(a) != len(b):
            out.append((where, "length", len(a), len(b)))
        for i, (x, y) in enumerate(zip(a, b)):
            walk(x, y, where + "[" + str(i) + "]", out)
    elif a != b:
        out.append((where, "value", a, b))


old, new = load(OLD), load(NEW)
diffs = []
walk(old, new, "", diffs)
print("parsed differences:", len(diffs))
for where, kind, a, b in diffs:
    print(" ", where, kind)
    if kind == "value" and isinstance(a, str) and "\n" not in a:
        print("    old:", repr(a))
        print("    new:", repr(b))

job = new["jobs"]["toolchain"]
print("steps:", len(job["steps"]), "permissions:", new.get("permissions"), "job permissions:", job.get("permissions"))
print("job keys:", sorted(job))
for i, st in enumerate(job["steps"]):
    print("  step", i, "uses" if "uses" in st else "run", st.get("uses", st.get("name")), "env:", sorted((st.get("env") or {})))
run = job["steps"][6]["run"]
lines = run.split("\n")
start = next(i for i, l in enumerate(lines) if l.startswith("case \"$WB_TOOL\""))
cut = "\n".join(lines[start:])
with open(OUT, "w") as f:
    f.write(cut)
own = open(OUT, "rb").read()
infra = open(INFRA_CUT, "rb").read()
print("own cut from run line", start + 1, ":", hashlib.sha256(own).hexdigest(), len(own), "bytes;",
      "equal to infra's new-lines-as-parsed.sh" if own == infra else "DIFFERENT from infra's cut")
print("sudo in the whole proposed file's run texts:")
for i, st in enumerate(job["steps"]):
    for l in (st.get("run") or "").split("\n"):
        if "sudo" in l:
            print("  step", i, ":", l.strip()[:110])

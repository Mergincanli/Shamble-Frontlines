# security-auditor, OPT-PATH reading, 2026-11-23: the proposed file with F-COUNT's one-token change (scratch copy
# only), parsed; its install-step cut compared with the cut the tries ran (cut-printf.sh); bash -n of the run text.
# Run: python3 -B /home/user/Shamble-Frontlines/.gate/ci-wasm-wt/.checkpoints/ci-wasm/security-auditor-opt-fcount.py
import hashlib
import subprocess
import yaml

NEW = "/home/user/Shamble-Frontlines/.gate/ci-wasm-wt/.gate/ci-wasm/proposed-2/toolchain.yml"
OUT = "/home/user/Shamble-Frontlines/.gate/ci-wasm-wt/.gate/ci-wasm/sa-opt-path/proposed-2-fcount.yml"
RUNTXT = "/home/user/Shamble-Frontlines/.gate/ci-wasm-wt/.gate/ci-wasm/sa-opt-path/fcount-step6-run.sh"
TRIED = "/home/user/Shamble-Frontlines/.gate/ci-wasm-wt/.gate/ci-wasm/sa-opt-path/work/cut-printf.sh"
OLD_LINE = '          [ "$(find "$@" -maxdepth 0 \\( -type f -o -type d \\) -user root ! -perm /022 | wc -l)" -eq "$#" ] \\\n'
NEW_LINE = '          [ "$(find "$@" -maxdepth 0 \\( -type f -o -type d \\) -user root ! -perm /022 -printf \'.\\n\' | wc -l)" -eq "$#" ] \\\n'

src = open(NEW, encoding="utf-8").read()
assert src.count(OLD_LINE) == 1, "the judge line is not in the proposed file once"
lineno = src[: src.index(OLD_LINE)].count("\n") + 1
out = src.replace(OLD_LINE, NEW_LINE)
open(OUT, "w", encoding="utf-8").write(out)
print("replaced line", lineno, "; variant", hashlib.sha256(out.encode()).hexdigest(), len(out.encode()), "bytes,",
      out.count("\n"), "lines")
print("new line:", NEW_LINE.rstrip("\n"))
a, b = yaml.safe_load(src), yaml.safe_load(out)
ja, jb = a["jobs"]["toolchain"], b["jobs"]["toolchain"]
same = [i for i in range(len(ja["steps"])) if ja["steps"][i] == jb["steps"][i]]
print("steps equal after parsing:", same, "; env equal:", ja["env"] == jb["env"], "; permissions:", b["permissions"])
run = jb["steps"][6]["run"]
open(RUNTXT, "w").write(run)
r = subprocess.run(["bash", "-n", RUNTXT], capture_output=True, text=True)
print("bash -n of the variant's step-6 run text: exit", r.returncode, r.stderr.strip())
lines = run.split("\n")
start = next(i for i, l in enumerate(lines) if l.startswith('case "$WB_TOOL"'))
cut = "\n".join(lines[start:])
print("variant's cut equals the tried cut-printf.sh:", cut == open(TRIED).read())

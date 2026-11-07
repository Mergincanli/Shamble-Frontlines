# U3c build B: mutation bites for check.mjs's new deps-step rules. Each mutation breaks one rule in a scratch copy of
# check.mjs; check.test.mjs (copied beside it) must then fail, and name the failing tests.
# Run: python3 -I <this> <spike dir> <scratch dir>
import pathlib
import re
import shutil
import subprocess
import sys

spike = pathlib.Path(sys.argv[1])
scratch = pathlib.Path(sys.argv[2])
scratch.mkdir(parents=True, exist_ok=True)
base = (spike / "check.mjs").read_text()
MUTATIONS = {
    "no tcpinfo shipped-deps rule": ('if (name !== "libc") problems.push(`npt-tcpinfo depends on ${name}`);', ";"),
    "no libc-present rule": ('if (!shippedDeps.includes("libc")) problems.push("npt-tcpinfo does not depend on libc");', ";"),
    "dev deps of tcpinfo count as shipped": (".filter(isShipped).map((d) => byId.get(d.pkg).name);", ".map((d) => byId.get(d.pkg).name);"),
    "only-npt-net rule off": ('if (name === "npt-net") continue;', 'if (name !== "__none__") continue;'),
    "literal grep only (allow(unsafe_code)": (
        r"const RELAX_UNSAFE = /\b(?:allow|expect|warn)\s*\(\s*(?:[\w:]+\s*,\s*)*unsafe_code\b/g;",
        r"const RELAX_UNSAFE = /allow\(unsafe_code/g;",
    ),
    "exceptions ignored (everything allowed)": ("if (n > 0 && !(crate in UNSAFE_EXCEPTIONS)) {", "if (false) {"),
    "manifest unsafe_code setting ignored": ('if (lines.some((l) => /\\bunsafe_code\\b/.test(l))) problems.push(`${f} sets unsafe_code`);', ";"),
    "workspace-lints inheritance not checked": ("if (!inheritsWorkspaceLints(lines))", "if (false)"),
    "crate-wide allow not checked": ("if (RELAX_UNSAFE_CRATE_WIDE.test(code))", "if (false)"),
    "tcpinfo's comments counted as attributes": ('const code = text.replace(/\\/\\/.*$/gm, "");', "const code = text;"),
    "tcpinfo allow count not checked": ("if (tcpinfoAllows !== TCPINFO_ALLOWS) {", "if (false) {"),
}
shutil.copy(spike / "check.test.mjs", scratch / "check.test.mjs")
for name, (old, new) in MUTATIONS.items():
    assert base.count(old) == 1, f"{name}: anchor not found once"
    (scratch / "check.mjs").write_text(base.replace(old, new, 1))
    r = subprocess.run(["node", "--test", str(scratch / "check.test.mjs")], capture_output=True, text=True)
    failed = re.findall(r"^not ok \d+ - (.*)$", r.stdout, re.M)
    print(f"{'BITES' if failed else 'MISSED'}  {name}: {len(failed)} failing: {failed}")
(scratch / "check.mjs").write_text(base)
r = subprocess.run(["node", "--test", str(scratch / "check.test.mjs")], capture_output=True, text=True)
print("unmutated copy:", re.search(r"^# pass (\d+)", r.stdout, re.M).group(1), "pass,", re.search(r"^# fail (\d+)", r.stdout, re.M).group(1), "fail")

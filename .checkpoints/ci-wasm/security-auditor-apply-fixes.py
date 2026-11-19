# security-auditor, part ci-text (2026-11-19): writes a scratch copy of the proposed toolchain.yml with the three
# replacements of the review (F-QUOTE line 172, F-OPT line 116, F-FLOOR lines 98-99), so they can be parsed and
# tried. Reads one file, writes one file; runs nothing.
# Usage: python3 -B -I .checkpoints/ci-wasm/security-auditor-apply-fixes.py <proposed.yml> <out.yml>
import pathlib
import sys

src = pathlib.Path(sys.argv[1]).read_text(encoding="utf-8").split("\n")
out = pathlib.Path(sys.argv[2])
assert out.name.endswith(".yml") and not out.exists()

F_QUOTE = [
    '              echo "== the type checks: the shipped files of the Worker and the page with the lib check, then with their tests; then the browser specs"',
]
F_OPT = [
    "          stat -c 'mode %a, owner %U:%G, %s bytes, %n' \"$WB_TOOL\" \"${WB_TOOL%/*}\" /opt",
    '          [ -z "$(find "${WB_TOOL%/*}" /opt -maxdepth 0 \\( ! -user root -o -perm /022 \\))" ] \\',
    '            || { echo "${WB_TOOL%/*} or /opt is not owned by root, or others than root may write in it"; exit 1; }',
]
F_FLOOR = [
    "          gh_version=$(gh --version | awk 'NR == 1 { print $3 }')",
    '          case "$gh_version" in "" | *[!0-9.]*) echo "gh printed no version of the form x.y.z: $gh_version"; exit 1 ;; esac',
    "          printf '%s\\n' 2.93.0 \"$gh_version\" | sort -V -C || { echo \"gh is older than 2.93.0\"; exit 1; }",
]

# Lines are 1-based in the review; replace from the bottom up so earlier numbers stay valid.
assert "the Worker's and the page's" in src[171]
src[171:172] = F_QUOTE
assert src[115].lstrip().startswith("stat -c 'mode %a, owner %U:%G, %s bytes, %n'")
src[115:116] = F_OPT
assert src[97].lstrip().startswith("printf '%s\\n' 2.93.0") and src[98].lstrip().startswith('|| { echo "gh is older')
src[97:99] = F_FLOOR
out.write_text("\n".join(src), encoding="utf-8")
print(f"wrote {out}")

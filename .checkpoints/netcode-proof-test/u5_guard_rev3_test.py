#!/usr/bin/env python3
"""Guard rev 3's refusal tests: no cargo call runs and no file in any worktree changes.

Part A runs the real guard (u5-fuzz-guard.sh) on argument and environment cases; every one must exit 64 before any
cargo call (the last ones stop at the toolchain check, after the config walk, because the nightly is not installed yet).
Part B runs a scratch copy of the guard whose dependency-check path comes from GUARD_TEST_DC and which stops right
after the dependency-check parse (it prints hashes_match's result), on scratch dependency-check files.
Usage: python3 -I u5_guard_rev3_test.py <scratch folder> <output file>
"""
import os
import subprocess
import sys

M = "/home/user/Shamble-Frontlines"
G = M + "/.checkpoints/netcode-proof-test/u5-fuzz-guard.sh"
DC = M + "/.checkpoints/netcode-proof-test/dependency-check.md"
W = M + "/.claude/worktrees/agent-a60a432d4564f4cc7"
T = M + "/.gate/shared-target"
GATE = M + "/.gate"
SCR = sys.argv[1]
OUT = sys.argv[2]
ZERO = "0" * 64
results = []


def run(guard, args, runner_env=False, extra=None):
    env = dict(os.environ)
    for k in ("CARGO_TARGET_DIR", "CARGO_NET_OFFLINE", "RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS",
              "CUSTOM_LIBFUZZER_PATH", "CUSTOM_LIBFUZZER_STD_CXX", "GUARD_TEST_DC", "GUARD_TEST_HASHES"):
        env.pop(k, None)
    if runner_env:
        env["CARGO_TARGET_DIR"] = T
        env["CARGO_NET_OFFLINE"] = "true"
    if extra:
        env.update(extra)
    p = subprocess.run(["bash", guard] + args, capture_output=True, text=True, env=env, cwd=SCR)
    lines = (p.stdout + p.stderr).strip().splitlines()
    return p.returncode, (lines[0] if lines else "")


def case(name, guard, args, want_rc, want_sub, **kw):
    rc, first = run(guard, args, **kw)
    ok = rc == want_rc and want_sub in first
    results.append(("PASS" if ok else "FAIL", name, rc, first[:170]))


C = GATE + "/u5/corpus-test"
AP = "-artifact_prefix=" + GATE + "/u5/art-test/"
GOOD_RUN = [W, "run", "input", C, "--", AP, "-runs=10000000", "-seed=1", "-max_len=65604",
            "-print_final_stats=1", "-detect_leaks=1"]

# Part A: the real guard.
case("A1 worktree outside .claude/worktrees", G, ["/tmp/x", "build"], 64, "the worktree must be under")
case("A2 worktree with ..", G, [M + "/.claude/worktrees/../x", "build"], 64, "holds '..'")
case("A3 E3 empty token", G, [W, "build", ""], 64, "an empty argument")
case("A4 E2 build with --", G, [W, "build", "--", "input"], 64, "build takes no '--'")
case("A5 GUARD-1 --fuzz-dir", G, [W, "build", "--fuzz-dir", "/tmp"], 64, "is not on the allow-list")
case("A6 E1 bare token after --", G, [W, "run", "input", C, "--", AP, "/tmp/x"], 64, "libFuzzer flag '/tmp/x'")
case("A7 E1 -jobs", G, [W, "run", "input", C, "--", AP, "-jobs=4"], 64, "libFuzzer flag '-jobs=4'")
case("A8 E1 -collect_data_flow", G, [W, "run", "input", C, "--", AP, "-collect_data_flow=/bin/sh"], 64,
     "libFuzzer flag '-collect_data_flow")
case("A9 E1 -features_dir", G, [W, "run", "input", C, "--", AP, "-features_dir=/tmp"], 64, "libFuzzer flag")
case("A10 E1 non-digit value", G, [W, "run", "input", C, "--", AP, "-runs=1e9"], 64, "the value must be")
case("A11 E1 empty value", G, [W, "run", "input", C, "--", AP, "-runs="], 64, "the value must be")
case("A12 E1 negative value", G, [W, "run", "input", C, "--", AP, "-runs=-1"], 64, "the value must be")
case("A13 E1 flag 0/1 only", G, [W, "run", "input", C, "--", AP, "-detect_leaks=2"], 64, "libFuzzer flag")
case("A14 E3 empty token after --", G, [W, "run", "input", C, "--", AP, ""], 64, "an empty argument")
case("A15 second -- after --", G, [W, "run", "input", C, "--", AP, "--"], 64, "libFuzzer flag '--'")
case("A16 run without -artifact_prefix", G, [W, "run", "input", C, "--", "-runs=5"], 64, "run needs -artifact_prefix")
case("A17 -exact_artifact_path outside .gate", G, [W, "run", "input", C, "--", AP, "-exact_artifact_path=/tmp/a"], 64,
     "is not under .gate/")
case("A18 corpus outside .gate", G, [W, "run", "input", "/tmp/c", "--", AP], 64, "is not an absolute path under")
case("A19 unknown target", G, [W, "build", "nosuch"], 64, "is not a [[bin]] name")
case("A20 build with two targets", G, [W, "build", "input", "ping"], 64, "build takes one target")
case("A21 sanitizer memory", G, [W, "build", "-s", "memory", "input"], 64, "sanitizer 'memory'")
case("A22 features other than bites", G, [W, "build", "--features", "x", "input"], 64, "features 'x'")
case("A23 subcommand alias b", G, [W, "b", "input"], 64, "only build or run")
case("A24 valid run, no runner environment", G, GOOD_RUN, 64, "CARGO_TARGET_DIR is")
case("A25 valid run, runner environment (passes the walk)", G, GOOD_RUN, 64,
     "toolchain nightly-2026-11-01 is not installed", runner_env=True)
case("A26 valid build -O -s address (passes the walk)", G, [W, "build", "-O", "-s", "address", "input"], 64,
     "toolchain nightly-2026-11-01 is not installed", runner_env=True)
case("A27 valid build --features=bites, all targets", G, [W, "build", "--features=bites"], 64,
     "toolchain nightly-2026-11-01 is not installed", runner_env=True)
case("A28 RUSTFLAGS set in the runner environment", G, GOOD_RUN, 64, "RUSTFLAGS is set", runner_env=True,
     extra={"RUSTFLAGS": "-Cfoo"})
case("A29 CARGO_NET_OFFLINE not true", G, GOOD_RUN, 64, "CARGO_NET_OFFLINE is", runner_env=True,
     extra={"CARGO_NET_OFFLINE": "false"})

# Part B: a scratch copy that reads GUARD_TEST_DC and stops after the parse.
src = open(G).read()
old_dc = "DC=$M/.checkpoints/netcode-proof-test/dependency-check.md\n"
assert src.count(old_dc) == 1
hook_at = "if [ $plant -eq 1 ]; then\n"
assert src.count(hook_at) == 1
hook = ('if [ -n "${GUARD_TEST_HASHES:-}" ]; then if hashes_match; then echo "MATCH ${#dc_paths[@]}"; '
        'else echo "DIFF${bad}"; fi; exit 0; fi\n')
test_guard = os.path.join(SCR, "u5-fuzz-guard.parse-test.sh")
with open(test_guard, "w") as f:
    f.write(src.replace(old_dc, 'DC="${GUARD_TEST_DC:?}"\n').replace(hook_at, hook + hook_at))
real = open(DC).read()
real_lines = real.splitlines()


def dc_case(name, text, want_rc, want_sub):
    path = os.path.join(SCR, "dc-" + name.split()[0] + ".md")
    with open(path, "w", newline="") as f:
        f.write(text)
    case(name, test_guard, [W], want_rc, want_sub, extra={"GUARD_TEST_DC": path, "GUARD_TEST_HASHES": "1"})


net = "spikes/netcode-proof/crates/net/Cargo.toml"
dc_case("B1 the real file", real, 0, "MATCH 8")
dc_case("B2 indented comment and spaces-only line", real + "  # note\n   \n", 0, "MATCH 8")
dc_case("B3 one space between hash and path", real + ZERO + " spikes/netcode-proof/x\n", 64, "is not '<sha256>")
dc_case("B4 a second fuzz/Cargo.toml line", real + ZERO + "  spikes/netcode-proof/fuzz/Cargo.toml\n", 64,
        "exactly one fuzz/Cargo.toml line (holds 2)")
dc_case("B5 an extra older line for one path", real + ZERO + "  " + net + "\n", 0, "MATCH 8")
dc_case("B6 that path's only line wrong",
        "\n".join(ZERO + "  " + net if l.endswith("  " + net) else l for l in real_lines) + "\n", 0,
        "DIFF " + net + " (afd0952b")
dc_case("B7 a listed file missing", real + ZERO + "  spikes/netcode-proof/crates/nosuch/Cargo.toml\n", 0,
        "DIFF spikes/netcode-proof/crates/nosuch/Cargo.toml (missing)")
dc_case("B8 a path with ..", real + ZERO + "  spikes/netcode-proof/../x\n", 64, "holds '..'")
dc_case("B9 CRLF line ends (the refusal's \\r splits the printed line)", real.replace("\n", "\r\n"), 64,
        "GUARD REFUSED: dependency-check.md line '3bf4d760")
dc_case("B10 no fuzz/Cargo.lock line",
        "\n".join(l for l in real_lines if not l.endswith("fuzz/Cargo.lock")) + "\n", 64,
        "exactly one fuzz/Cargo.lock line (holds 0)")
dc_case("B11 a path outside the spike", real + ZERO + "  legacy/index.html\n", 64, "is not '<sha256>")

passed = sum(1 for r in results if r[0] == "PASS")
lines = ["%s | %s | exit %s | %s" % r for r in results]
lines.append("%d of %d PASS" % (passed, len(results)))
text = "\n".join(lines) + "\n"
with open(OUT, "w") as f:
    f.write(text)
print(text, end="")
sys.exit(0 if passed == len(results) else 1)

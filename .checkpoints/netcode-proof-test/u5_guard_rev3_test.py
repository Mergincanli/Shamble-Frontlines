#!/usr/bin/env python3
"""Guard rev 4's tests (rev 3's file, extended): no cargo-fuzz call runs and no file in any worktree changes.

The nightly and cargo-fuzz are installed now, so a valid call on the real guard would reach cargo-fuzz. Hence:
Part A runs the real guard only on cases that refuse before the call (argument checks, or the runner's environment
missing or wrong).
Part B runs a scratch copy whose dependency-check path comes from GUARD_TEST_DC and which stops right after the
dependency-check parse (it prints hashes_match's result), on scratch dependency-check files.
Part C runs a scratch copy whose cargo-fuzz call line is replaced by a print and exit 0 (asserted below), with the
runner's environment: every check runs (the config walk, the hashes, both trees, the graph check, the version), then it
stops. GUARD_TEST_BAD_BLOB_PATH makes file_blob report a wrong blob for one path (R3-3's refusal).
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
              "CUSTOM_LIBFUZZER_PATH", "CUSTOM_LIBFUZZER_STD_CXX", "GUARD_TEST_DC", "GUARD_TEST_HASHES",
              "GUARD_TEST_BAD_BLOB_PATH"):
        env.pop(k, None)
    if runner_env:
        env["CARGO_TARGET_DIR"] = T
        env["CARGO_NET_OFFLINE"] = "true"
    if extra:
        env.update(extra)
    p = subprocess.run(["bash", guard] + args, capture_output=True, text=True, env=env, cwd=SCR)
    return p.returncode, (p.stdout + p.stderr).strip()


def case(name, guard, args, want_rc, want_sub, **kw):
    if guard == G and kw.get("runner_env") and not kw.get("extra"):
        sys.exit("refused: a real-guard case with the runner's environment and nothing set to refuse could reach the call")
    rc, text = run(guard, args, **kw)
    ok = rc == want_rc and want_sub in text
    lines = text.splitlines()
    shown = lines[-1] if lines else ""
    graph = [l for l in lines if l.startswith("GUARD GRAPH:")]
    if graph:
        shown = graph[0] + " || " + shown
    results.append(("PASS" if ok else "FAIL", name, rc, shown[:400]))


C = GATE + "/u5/corpus-test"
AP = "-artifact_prefix=" + GATE + "/u5/art-test/"
GOOD_RUN = [W, "run", "input", C, "--", AP, "-runs=10000000", "-seed=1", "-max_len=65604",
            "-print_final_stats=1", "-detect_leaks=1", "-malloc_limit_mb=0"]
BITE = "PM-0001-34/unchecked-index"

# Part A: the real guard, refusals before the call only.
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
case("A28 RUSTFLAGS set in the runner environment", G, GOOD_RUN, 64, "RUSTFLAGS is set", runner_env=True,
     extra={"RUSTFLAGS": "-Cfoo"})
case("A29 CARGO_NET_OFFLINE not true", G, GOOD_RUN, 64, "CARGO_NET_OFFLINE is", runner_env=True,
     extra={"CARGO_NET_OFFLINE": "false"})
case("A30 R3-1 --bite without --features bites", G, [W, "run", "input", C, "--bite", BITE, "--", AP], 64,
     "--bite needs --features bites")
case("A31 R3-1 --bite on build", G, [W, "build", "--features", "bites", "--bite", BITE, "input"], 64,
     "--bite is for run only")
case("A32 R3-1 bad bite id", G, [W, "run", "--features", "bites", "--bite", "x;id", "input", C, "--", AP], 64,
     "bite id 'x;id'")
case("A33 R3-1 two --bite", G, [W, "run", "--features", "bites", "--bite", BITE, "--bite", "PM-0001-34/b", "input",
                                C, "--", AP], 64, "one --bite only")
case("A34 R3-1 --bite with no id", G, [W, "run", "--bite"], 64, "'--bite' needs a bite id")
case("A35 R3-1 --bite with a dash value", G, [W, "run", "--features", "bites", "--bite", "--release", "input", C, "--",
                                              AP], 64, "bite id '--release'")
case("A36 R3-4 -rss_limit_mb=0", G, [W, "run", "input", C, "--", AP, "-rss_limit_mb=0"], 64, "not 0")
case("A37 R3-4 -timeout=0", G, [W, "run", "input", C, "--", AP, "-timeout=0"], 64, "not 0")
case("A38 R3-4 ten digits", G, [W, "run", "input", C, "--", AP, "-runs=1000000000"], 64, "1 to 9 digits")
case("A39 R3-4 nine digits pass the flag check", G, [W, "run", "input", C, "--", AP, "-runs=999999999",
                                                     "-timeout=25", "-rss_limit_mb=2048"], 64, "CARGO_TARGET_DIR is")

# Part B: a scratch copy that reads GUARD_TEST_DC and stops after the parse.
src = open(G).read()
old_dc = "DC=$M/.checkpoints/netcode-proof-test/dependency-check.md\n"
assert src.count(old_dc) == 1
hook_at = "if [ $plant -eq 1 ]; then\n"
assert src.count(hook_at) == 1
hook = ('if [ -n "${GUARD_TEST_HASHES:-}" ]; then if hashes_match; then echo "MATCH ${#dc_paths[@]}"; '
        'else echo "DIFF${bad}"; fi; exit 0; fi\n')
parse_guard = os.path.join(SCR, "u5-fuzz-guard.parse-test.sh")
with open(parse_guard, "w") as f:
    f.write(src.replace(old_dc, 'DC="${GUARD_TEST_DC:?}"\n').replace(hook_at, hook + hook_at))
real = open(DC).read()
real_lines = real.splitlines()


def dc_case(name, text, want_rc, want_sub):
    path = os.path.join(SCR, "dc-" + name.split()[0] + ".md")
    with open(path, "w", newline="") as f:
        f.write(text)
    case(name, parse_guard, [W], want_rc, want_sub, extra={"GUARD_TEST_DC": path, "GUARD_TEST_HASHES": "1"})


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
dc_case("B11 R3-2 a hash line outside the spike is skipped", real + ZERO + "  legacy/index.html\n", 0, "MATCH 8")
dc_case("B12 R3-2 a deleted spike file, absent", real + "deleted  spikes/netcode-proof/crates/gone/Cargo.toml\n", 0,
        "MATCH 9")
dc_case("B13 R3-2 a deleted-only spike file, present", real + "deleted  spikes/netcode-proof/crates/room/Cargo.toml\n",
        0, "DIFF spikes/netcode-proof/crates/room/Cargo.toml (")
dc_case("B14 R3-2 a deleted line outside the spike is skipped", real + "deleted  tools/package.json\n", 0, "MATCH 8")
dc_case("B15 a spike path with a space", real + ZERO + "  spikes/netcode-proof/a b\n", 64, "the path is not plain")
dc_case("B16 deleted with one space", real + "deleted spikes/netcode-proof/x\n", 64, "is not '<sha256>")

# Part C: a scratch copy that stops right before the cargo-fuzz call, run with the runner's environment.
call_line = '"${fe[@]}" cargo "+$NIGHTLY" fuzz "${pass[@]}"\n'
assert src.count(call_line) == 1
stop = 'echo "TEST STOP before the call: pass=${pass[*]} NPT_BITE=${bite:-unset} env=${fe[*]}"; exit 0\n'
blob_at = 'head_blob() { "${GE[@]}" git rev-parse --verify --quiet "HEAD:$1" 2>/dev/null; }\n'
assert src.count(blob_at) == 1
blob_hook = ('if [ -n "${GUARD_TEST_BAD_BLOB_PATH:-}" ]; then file_blob() { if [ "$1" = "$W/$GUARD_TEST_BAD_BLOB_PATH" ]; '
             'then echo 0000000000000000000000000000000000000000; else "${GE[@]}" git hash-object --no-filters -- "$1" '
             '2>/dev/null; fi; }; fi\n')
stop_src = src.replace(call_line, stop).replace(blob_at, blob_at + blob_hook)
assert stop_src.count('fuzz "${pass[@]}"') == 0 and stop_src.count("TEST STOP before the call") == 1
stop_guard = os.path.join(SCR, "u5-fuzz-guard.stop-test.sh")
with open(stop_guard, "w") as f:
    f.write(stop_src)
case("C1 valid run: every check passes, stops before the call", stop_guard, GOOD_RUN, 0, "TEST STOP before the call",
     runner_env=True)
case("C2 valid build -O -s address", stop_guard, [W, "build", "-O", "-s", "address", "input"], 0,
     "pass=build -O -s address input NPT_BITE=unset", runner_env=True)
case("C3 valid build --features=bites, all targets", stop_guard, [W, "build", "--features=bites"], 0,
     "pass=build --features=bites NPT_BITE=unset", runner_env=True)
case("C4 R3-1 a bite run: --bite dropped from the args, NPT_BITE set", stop_guard,
     [W, "run", "--features", "bites", "--bite", BITE, "input", C, "--", AP, "-runs=10", "-seed=7", "-max_len=65604",
      "-print_final_stats=1"], 0, "pass=run --features bites input " + C + " -- " + AP + " -runs=10 -seed=7 "
     "-max_len=65604 -print_final_stats=1 NPT_BITE=" + BITE, runner_env=True)
case("C5 R3-3 an unlisted path crate's manifest differs from HEAD", stop_guard, [W, "build", "input"], 64,
     "spikes/netcode-proof/crates/room/Cargo.toml is not listed in dependency-check.md and differs from the copy's HEAD",
     runner_env=True, extra={"GUARD_TEST_BAD_BLOB_PATH": "spikes/netcode-proof/crates/room/Cargo.toml"})
case("C6 R3-3 a listed path is checked by its hash line, not HEAD", stop_guard, [W, "build", "input"], 0,
     "TEST STOP before the call", runner_env=True,
     extra={"GUARD_TEST_BAD_BLOB_PATH": "spikes/netcode-proof/crates/net/Cargo.toml"})

passed = sum(1 for r in results if r[0] == "PASS")
lines = ["%s | %s | exit %s | %s" % r for r in results]
lines.append("%d of %d PASS" % (passed, len(results)))
text = "\n".join(lines) + "\n"
with open(OUT, "w") as f:
    f.write(text)
print(text, end="")
sys.exit(0 if passed == len(results) else 1)

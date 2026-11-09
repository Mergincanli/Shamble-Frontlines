# U4s build: the bites (u4-build-plan.md §5 U4s: grid floor-rounding (s2), the ring kept one short (s4); the self-test
# bites of check.mjs's two greps; each grep planted once in the real tree). Each bite is planted in place in the
# builder's worktree, one at a time; the test (or step) it targets is run alone; its failure is recorded by name; the
# file is restored and its sha256 checked against the original. Build rules (§7.1): CARGO_TARGET_DIR is the shared
# folder, every file under crates/ is touched before each build, and each run must show the expected number of tests.
# Run: python3 -I <this> <spike dir> [bite name ...]
import hashlib
import os
import pathlib
import re
import subprocess
import sys

SPIKE = pathlib.Path(sys.argv[1]).resolve()
ONLY = set(sys.argv[2:])
TARGET = "/home/user/Shamble-Frontlines/.gate/shared-target"
ENV = {**os.environ, "CARGO_TARGET_DIR": TARGET}
GRID = SPIKE / "crates/sim/src/grid.rs"
HISTORY = SPIKE / "crates/sim/src/history.rs"
TICKS = SPIKE / "crates/sim/src/ticks.rs"
ROOM = SPIKE / "crates/room/src/lib.rs"
CHECK = SPIKE / "check.mjs"

DERIVED = "} else if (!(isPath || /\\bTICK_HZ\\b/.test(init) || /\\bticks_for_ms\\s*\\(/.test(init))) {"

# (name, kind, file, old, new, filter, expected tests run, the failing test names must include, the output must say)
BITES = [
    ("grid floor-rounding (s2)", "cargo", GRID,
     "field.fit(scaled.round() as i64)", "field.fit(scaled.floor() as i64)",
     "s2_", 2, ["s2_from_then_to_returns_every_integer_of_every_field",
                "s2_to_errs_by_at_most_half_a_step_plus_one_ulp_over_the_half_steps_and_the_range_ends"],
     "came back from"),
    ("the ring kept one short (s4)", "cargo", HISTORY,
     "let capacity = capacity_ticks as usize;", "let capacity = capacity_ticks as usize - 1;",
     "s4_", 1, ["s4_the_hit_history_keeps_exactly_its_capacity_and_evicts_the_oldest"],
     "64 held after 64 pushes"),
    ("check: a function call counts as derived", "node", CHECK,
     DERIVED, DERIVED.replace("|| /\\bticks_for_ms", "|| /\\(/.test(init) || /\\bticks_for_ms"),
     "tick constants", 1, ["tick constants: TICK_HZ is set once"], "loop_catchup_at(30)"),
    ("check: TICK_HZ may be set anywhere first", "node", CHECK,
     "if (f.startsWith(TICK_HZ_HOME) && home === null) {", "if (home === null) {",
     "tick constants", 1, ["tick constants: TICK_HZ is set once"], "outside crates/sim/src/"),
    ("check: float literals missed", "node", CHECK,
     "const FLOAT_TOKEN =\n  /\\bf(?:32|64)\\b|", "const FLOAT_TOKEN =\n  /\\bf(?:32|64)\\b/g; const X =\n  /",
     "npt-room is float-free", 1, ["npt-room is float-free"], "1.5"),
    ("check: the tick grep unhooked from the deps step", "node", CHECK,
     "    ...tickConstantProblems(files, read),\n", "",
     "the deps step is depsStep", 1, ["the deps step is depsStep"], "tickConstantProblems's plant is not reported"),
    ("check: the float grep unhooked from the deps step", "node", CHECK,
     "    ...roomFloatProblems(files, read),\n", "",
     "the deps step is depsStep", 1, ["the deps step is depsStep"], "roomFloatProblems's plant is not reported"),
    ("real tree: a float in npt-room", "deps", ROOM,
     "mod-only-marker", None, None, None, None,
     "crates/room/src/lib.rs:7 has `f32` (2 float token(s)): npt-room is float-free"),
    ("real tree: LOOP_CATCHUP set from a literal", "deps", TICKS,
     "pub const LOOP_CATCHUP: u32 = loop_catchup_at(TICK_HZ);", "pub const LOOP_CATCHUP: u32 = 7;",
     None, None, None,
     "crates/sim/src/ticks.rs:36 sets LOOP_CATCHUP from `7`: a tick-counted value is a const fn of npt_sim::TICK_HZ"),
]


def sha(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()


def touch():
    for p in (SPIKE / "crates").rglob("*"):
        if p.is_file():
            os.utime(p)


def run(args):
    r = subprocess.run(args, cwd=SPIKE, env=ENV, capture_output=True, text=True)
    return r.returncode, r.stdout + r.stderr


originals = {p: sha(p) for p in (GRID, HISTORY, TICKS, ROOM, CHECK)}
for p, h in originals.items():
    print(f"original {p.relative_to(SPIKE)} sha256 {h}")
du = subprocess.run(["df", "-h", "/home/user"], capture_output=True, text=True).stdout.strip().splitlines()[-1]
print(f"df: {du}")

failures = 0
for name, kind, path, old, new, filt, expect_run, names, says in BITES:
    if ONLY and name not in ONLY:
        continue
    text = path.read_text()
    if kind == "deps" and new is None:
        planted = text + "\npub const SCALE: f32 = 0.5;\n"
    else:
        if text.count(old) != 1:
            print(f"BITES  {name}: the site is not unique ({text.count(old)} matches): NOT RUN")
            failures += 1
            continue
        planted = text.replace(old, new)
    path.write_text(planted)
    try:
        if kind == "cargo":
            touch()
            code, out = run(["cargo", "test", "--locked", "--offline", "-p", "npt-sim", "--test", "seam", "--", filt])
            ran = re.search(r"running (\d+) tests?", out)
            ran = int(ran.group(1)) if ran else None
            result = re.search(r"test result: \w+\. \d+ passed; \d+ failed[^\n]*", out)
            failed = re.findall(r"^test (\S+) \.\.\. FAILED$", out, re.M)
            ok = (code != 0 and ran == expect_run and len(failed) == expect_run
                  and all(n in failed for n in names) and says in out)
            print(f"BITES  {name}: filter {filt!r}; ran {ran} (expected {expect_run}); "
                  f"{result.group(0) if result else 'no result line'}; failed {failed}; says {says!r}: {says in out}")
            m = re.search(r"panicked at ([^\n]+)\n([^\n]+)", out)
            if m:
                print(f"       {m.group(1)}: {m.group(2)}")
        elif kind == "node":
            code, out = run(["node", "--test", "--test-name-pattern", filt, "check.test.mjs"])
            tests = re.search(r"^# tests (\d+)", out, re.M)
            fail = re.search(r"^# fail (\d+)", out, re.M)
            notok = re.findall(r"^not ok \d+ - (.+)$", out, re.M)
            tests, fail = (int(tests.group(1)) if tests else None), (int(fail.group(1)) if fail else None)
            ok = (code != 0 and tests == expect_run and fail == expect_run
                  and all(any(t.startswith(n) for t in notok) for n in names) and says in out)
            print(f"BITES  {name}: pattern {filt!r}; tests {tests} (expected {expect_run}); fail {fail}; "
                  f"not ok {notok}; says {says!r}: {says in out}")
        else:
            code, out = run(["node", "check.mjs", "--only", "deps"])
            ok = code != 0 and "FAIL  deps" in out and says in out
            line = next((l for l in out.splitlines() if "FAIL" in l), "no FAIL line")
            print(f"BITES  {name}: {line[:60]}...; says {says!r}: {says in out}")
        if not ok:
            failures += 1
            print("       NOT AS EXPECTED; output tail:\n" + "\n".join(out.splitlines()[-25:]))
    finally:
        path.write_text(text)
        h = sha(path)
        print(f"       restored {path.relative_to(SPIKE)}: sha256 {h} {'==' if h == originals[path] else '!='} original")
        if h != originals[path]:
            failures += 1

for p, h in originals.items():
    if sha(p) != h:
        print(f"MISMATCH after the run: {p}")
        failures += 1
print(f"bites done: {failures} not as expected")
sys.exit(1 if failures else 0)

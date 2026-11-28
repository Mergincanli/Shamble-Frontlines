#!/usr/bin/env python3
"""C1c review bites: each revert applied in the SCRATCH copy only, its tests run, the file restored.

Usage: python3 rbites.py <name>|control|all
Scratch: /home/user/Shamble-Frontlines/.gate/c1c-review-scratch/netcode-proof (never the worktree).
"""
import hashlib, os, subprocess, sys, time

SCR = "/home/user/Shamble-Frontlines/.gate/c1c-review-scratch/netcode-proof"
DELAY = os.path.join(SCR, "crates/net/src/l2/delay.rs")
TESTS = os.path.join(SCR, "crates/net/src/l2/delay_tests.rs")
TARGET = "/home/user/Shamble-Frontlines/.gate/u3v3-harness-check-target"
OUT = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u3c-fix3-review-c1c/rbites-run.txt"

T_FULL = "l2::delay::tests::a_full_budget_is_pending_until_it_frees_and_the_held_bytes_stay_bounded"
T_WAKE_CUR = "l2::delay::tests::an_acknowledgement_an_application_task_steps_that_reopens_the_up_room_wakes_the_pump"
T_WAKE_AB = "l2::delay::tests::an_acknowledgement_add_burst_steps_that_reopens_the_up_room_wakes_the_pump"
T_NORMAL = "l2::delay::tests::a_pump_that_ends_without_a_panic_never_gives_the_panic_error"
T_PANIC = "l2::delay::tests::a_pump_that_panics_gives_the_reader_an_error_never_a_clean_eof"

SHUT_NEW = """        if c.down_done {
            return Poll::Ready(Ok(()));
        }
        if c.pump_done {
            return Poll::Ready(c.failed.map_or(Ok(()), |k| Err(k.into())));
        }
"""
SHUT_OLD = """        if c.down_done || c.pump_done {
            return Poll::Ready(Ok(()));
        }
"""

BITES = {
    # name: [(file, old, new)], tests, expectation
    "R1-SPIN": ([(DELAY, "let again = c.released[DOWN] > 0 && !s_owes_write;",
                  "let again = { let _ = s_owes_write; c.released[DOWN] > 0 };")],
                [T_FULL], "fail at :792 (polls >= 2000)"),
    "WAKE-ADDBURST-ONLY": ([(DELAY, "        let w = c.wakers(got[UP], false);\n        let pump = if wake_pump {",
                             "        let w = c.wakers(got[UP], false);\n        let _ = wake_pump;\n        let pump = if got[DOWN] {")],
                           [T_WAKE_CUR, T_WAKE_AB], "add_burst test fails (woke 0), current() passes"),
    "PANIC-ALWAYS": ([(DELAY, "if std::thread::panicking() && c.failed.is_none() {", "if c.failed.is_none() {")],
                     [T_NORMAL, T_PANIC], "normal-end test fails (clean close: Other); panic test passes"),
    "PANIC-ALWAYS-DEADLINE-ONLY": ([(DELAY, "if std::thread::panicking() && c.failed.is_none() {", "if c.failed.is_none() {"),
                                   (TESTS, "    ends_without_the_panic_error(&mut ds, \"a clean close, then the pump's task cancelled\").await;\n",
                                    "    // review: clean-close half skipped\n")],
                                  [T_NORMAL], "normal-end test fails in its deadline half (Other)"),
    "SHUTDOWN-OK": ([(DELAY, SHUT_NEW, SHUT_OLD)], [T_PANIC], "panic test fails at the shutdown assert"),
    "control": ([], [T_FULL, T_WAKE_CUR, T_WAKE_AB, T_NORMAL, T_PANIC], "all pass"),
}


def sha(p):
    return hashlib.sha256(open(p, "rb").read()).hexdigest()


def run(name, log):
    edits, tests, expect = BITES[name]
    saved = {p: open(p).read() for p in (DELAY, TESTS)}
    h0 = {p: sha(p) for p in (DELAY, TESTS)}
    try:
        for p, old, new in edits:
            cur = open(p).read()
            n = cur.count(old)
            if n != 1:
                raise SystemExit(f"{name}: anchor found {n} times in {p}")
            open(p, "w").write(cur.replace(old, new))
        env = dict(os.environ, CARGO_TARGET_DIR=TARGET)
        cmd = ["cargo", "+1.99.1", "test", "--locked", "-p", "npt-net", "--lib", "--", "--exact", "--nocapture",
               "--test-threads", "1"] + tests
        t0 = time.time()
        r = subprocess.run(cmd, cwd=SCR, env=env, capture_output=True, text=True)
        dt = time.time() - t0
        out = r.stdout + r.stderr
        keep = [l for l in out.splitlines() if any(k in l for k in (
            "test l2::", "test result", "panicked", "pump ran", "left:", "right:", "woke", "lifted", "the reader got",
            "shutdown read", "Compiling npt-net", "Running unittests", "error", "warning: unused"))]
        log.write(f"\n=== {name} (expect: {expect}) exit={r.returncode} {dt:.1f}s\n")
        log.write("\n".join(keep) + "\n")
        log.flush()
        print(f"{name}: exit={r.returncode}")
        print("\n".join(keep))
    finally:
        for p, s in saved.items():
            open(p, "w").write(s)
        for p in (DELAY, TESTS):
            assert sha(p) == h0[p], f"restore failed for {p}"
        log.write(f"restored: {name} hashes ok\n")


def main():
    which = sys.argv[1]
    names = list(BITES) if which == "all" else [which]
    with open(OUT, "a") as log:
        log.write(f"\n##### {time.strftime('%Y-%m-%d %H:%M:%S')} run {which}\n")
        for n in names:
            run(n, log)


if __name__ == "__main__":
    main()

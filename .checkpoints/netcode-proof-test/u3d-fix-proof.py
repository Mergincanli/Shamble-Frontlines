# U3d fix pass proof (the fixer): bites for every new or changed run condition and test assert, the reviewers' S2-S6,
# X1, X2 re-run against the fixed code, and clean runs (the short self-test, the recorded chrt switch from outside and
# a process born under chrt, NPT_L2_SELFTEST_SECS's bounds with a non-UTF-8 value).
# Each bite: planted in place in the worktree, alone; a build after a touch of every .rs under crates/ (mtime only) and
# a df check (stop under 2 GB), with CARGO_TARGET_DIR the shared folder; the binary must be newer than the touch; its
# one test runs alone (the count checked: 1); the failure and its message recorded; the file restored and its sha256
# checked against the fixed state's.
# Run: python3 -I <this> <spike dir> clean|bites [id ...]
import hashlib
import json
import os
import pathlib
import re
import shutil
import subprocess
import sys
import time

SPIKE = pathlib.Path(sys.argv[1]).resolve()
MODE = sys.argv[2]
ONLY = set(sys.argv[3:])
NOTES = pathlib.Path(__file__).resolve().parent
TARGET = "/home/user/Shamble-Frontlines/.gate/shared-target"
L2 = SPIKE / "crates/net/src/l2"
TESTS, SOCKET = L2 / "socket_tests.rs", L2 / "socket.rs"
T = "l2::socket::tests::"
SELF = "l2_timing_self_test"
T6 = "the_cross_check_reads_delay_plus_lateness"
OUT = NOTES / f"u3d-fix-{MODE}-output.txt"
LINES = []


def say(s=""):
    print(s, flush=True)
    LINES.append(s)
    OUT.write_text("\n".join(LINES) + "\n")


def sha(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()


def df_ok():
    gb = shutil.disk_usage("/home/user").free / 1e9
    say(f"    df /home/user: {gb:.1f} GB free")
    if gb < 2:
        say("under 2 GB free: stop")
        sys.exit(3)


def build(release):
    df_ok()
    t0 = time.time()
    for p in (SPIKE / "crates").rglob("*.rs"):
        os.utime(p)
    env = dict(os.environ, CARGO_TARGET_DIR=TARGET)
    args = ["cargo", "+1.99.1", "test"] + (["--release"] if release else []) + \
        ["--locked", "-p", "npt-net", "--lib", "--no-run", "--message-format=json"]
    r = subprocess.run(args, cwd=SPIKE, env=env, capture_output=True, text=True)
    exe = [m["executable"] for m in map(json.loads, filter(None, r.stdout.splitlines()))
           if m.get("reason") == "compiler-artifact" and m.get("executable")
           and m["target"]["name"] == "npt_net" and m["profile"]["test"]]
    if r.returncode != 0 or len(exe) != 1:
        errs = [m["message"]["rendered"] for m in map(json.loads, filter(None, r.stdout.splitlines()))
                if m.get("reason") == "compiler-message" and m["message"]["level"] == "error"]
        say("    BUILD FAILED: " + " | ".join(errs)[:1500] + r.stderr[-500:])
        return None
    b = exe[0]
    fresh = os.stat(b).st_mtime >= t0
    say(f"    built {pathlib.Path(b).name} ({'release' if release else 'debug'}) in {time.time() - t0:.1f} s; "
        f"newer than the touch: {fresh}")
    assert fresh, "the binary is older than the touch: another copy's artifact?"
    return b


def run_env(extra):
    env = dict(os.environ)
    env.pop("CARGO_TARGET_DIR", None)
    env.update(extra)
    return env


def run(b, test, ignored, extra=None, timeout=None):
    args = [b, T + test, "--exact", "--nocapture"] + (["--ignored"] if ignored else [])
    t0 = time.time()
    try:
        r = subprocess.run(args, env=run_env(extra or {}), stdout=subprocess.PIPE, stderr=subprocess.STDOUT,
                           timeout=timeout)
        return r.returncode, r.stdout.decode(errors="replace"), time.time() - t0
    except subprocess.TimeoutExpired as e:
        return "timeout", (e.stdout or b"").decode(errors="replace"), time.time() - t0


def summarize(code, out):
    ran = re.search(r"running (\d+) tests?", out)
    result = re.search(r"test result: \w+\. (\d+) passed; (\d+) failed", out)
    panic = re.search(r"panicked at ([^\n]+):\n((?:.*\n){1,4})", out)
    msg = " | ".join(l.strip() for l in panic.group(2).splitlines() if l.strip())[:600] if panic else "(no panic)"
    before = out.split("panicked at")[0]
    prints = sum(1 for l in before.splitlines() if l.startswith("U3d self-test:") or l.startswith("  "))
    return ran.group(1) if ran else "?", result.group(0) if result else "no result", panic, msg, prints


def chrt_after(b, secs, label, extra=None):
    """The recorded switch from outside (M14): chrt -f -p 1 on npt-l2-release about 1 s into the run."""
    log = NOTES / f"u3d-fix-{label}.log"
    with open(log, "wb") as fh:
        p = subprocess.Popen([b, T + SELF, "--exact", "--nocapture", "--ignored"],
                             env=run_env(dict(extra or {}, NPT_L2_SELFTEST_SECS=str(secs))),
                             stdout=fh, stderr=subprocess.STDOUT)
        tid, end = None, time.time() + 10
        while tid is None and time.time() < end:
            for c in pathlib.Path(f"/proc/{p.pid}/task").glob("*/comm"):
                try:
                    if c.read_text().strip() == "npt-l2-release":
                        tid = c.parent.name
                except OSError:
                    pass
            time.sleep(0.01)
        time.sleep(1)
        before = subprocess.run(["chrt", "-p", str(tid)], capture_output=True, text=True).stdout.split("\n")
        sw = subprocess.run(["chrt", "-f", "-p", "1", str(tid)], capture_output=True, text=True)
        after = subprocess.run(["chrt", "-p", str(tid)], capture_output=True, text=True).stdout.split("\n")
        try:
            slack = pathlib.Path(f"/proc/{tid}/timerslack_ns").read_text().strip()
        except OSError as e:
            slack = f"unreadable ({e})"
        code = p.wait(timeout=secs + 60)
    out = log.read_bytes().decode(errors="replace")
    say(f"    release thread tid {tid}; before: {before[0].strip()}; chrt exit {sw.returncode} {sw.stderr.strip()}; "
        f"after: {after[0].strip()}; its slack read from outside after the switch: {slack}")
    return code, out


def show(label, code, out, want_fail, says):
    ran, result, panic, msg, prints = summarize(code, out)
    slack_line = next((l.strip() for l in out.splitlines() if l.startswith("  slack now")), "-")
    if want_fail:
        ok = ran == "1" and code == 101 and panic is not None and says in msg
        verdict = "BITES" if ok else "MISSED"
    else:
        ok = ran == "1" and code == 0 and panic is None
        verdict = "PASSES" if ok else "FAILED"
    say(f"  {verdict} {label}: exit {code}; ran {ran}; {result}; prints before the panic {prints}")
    say(f"    message: {msg}")
    say(f"    expected in it: {says!r}" if want_fail else "    (no failure expected)")
    if slack_line != "-":
        say(f"    {slack_line}")
    return verdict


FIXED = {p: sha(p) for p in (TESTS, SOCKET)}
say(f"# U3d fix pass proof, mode {MODE}, {time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime())}")
for p, h in FIXED.items():
    say(f"fixed state: {p.name} sha256 {h}")

if MODE == "clean":
    b = build(release=True)
    listed = subprocess.run([b, "--list"], capture_output=True, text=True, env=run_env({})).stdout
    ign = subprocess.run([b, "--ignored", "--list"], capture_output=True, text=True, env=run_env({})).stdout
    total = sum(1 for l in listed.splitlines() if l.endswith(": test"))
    names = [l[:-6] for l in ign.splitlines() if l.endswith(": test") and l.startswith(T)]
    say(f"  --list: {total} tests; --ignored --list under {T}: {names}")
    for t in ["the_task_poller_takes_lateness_where_the_task_runs", "the_fate_log_and_the_histogram_agree",
              "the_verdict_line_is_per_direction_and_inclusive", "nearest_rank_and_loss_band_on_fixed_inputs",
              "the_exact_excess_on_fixed_samples", T6, "a_held_lock_makes_both_directions_late"]:
        assert f"{T}{t}: test" in listed, t
    say("  T1-T7 each listed by name")
    results = []
    if not ONLY or "SHORT" in ONLY:
        code, out, secs = run(b, SELF, True, {"NPT_L2_SELFTEST_SECS": "5"}, timeout=120)
        (NOTES / "u3d-fix-selftest5.log").write_text(out)
        results.append(("SHORT", show("SHORT the self-test at 5 s (u3d-fix-selftest5.log)", code, out, False, "")))
    if not ONLY or "CHRT-AFTER" in ONLY:
        code, out = chrt_after(b, 6, "chrt-after")
        results.append(("CHRT-AFTER", show("CHRT-AFTER chrt -f -p 1 on the release thread ~1 s into 6 s "
                                           "(u3d-fix-chrt-after.log)", code, out, False, "")))
    if not ONLY or "CHRT-BORN" in ONLY:
        args = ["chrt", "-f", "1", b, T + SELF, "--exact", "--nocapture", "--ignored"]
        r = subprocess.run(args, env=run_env({"NPT_L2_SELFTEST_SECS": "3"}), stdout=subprocess.PIPE,
                           stderr=subprocess.STDOUT, timeout=120)
        out = r.stdout.decode(errors="replace")
        (NOTES / "u3d-fix-chrt-born.log").write_text(out)
        results.append(("CHRT-BORN", show("CHRT-BORN the whole process under chrt -f 1, 3 s (u3d-fix-chrt-born.log)",
                                          r.returncode, out, False, "")))
    if not ONLY or "ENV" in ONLY:
        for v in ["\udcff", "0", "601", "abc"]:
            code, out, secs = run(b, SELF, True, {"NPT_L2_SELFTEST_SECS": v}, timeout=20)
            inv = re.search(r"INVALID: NPT_L2_SELFTEST_SECS=[^;]*", out)
            started = "U3d self-test:" in out
            say(f"  ENV value {os.fsencode(v)!r}: exit {code} in {secs:.2f} s; {inv.group(0) if inv else 'no INVALID'}; "
                f"self-test started: {started}")
            results.append((f"ENV {os.fsencode(v)!r}", "INVALID" if code == 101 and inv and not started else "WRONG"))
        env = run_env({})
        env.pop("NPT_L2_SELFTEST_SECS", None)
        try:
            r = subprocess.run([b, T + SELF, "--exact", "--nocapture", "--ignored"], env=env,
                               stdout=subprocess.PIPE, stderr=subprocess.STDOUT, timeout=3)
            say(f"  ENV unset: exit {r.returncode} within 3 s (WRONG: the 100 s default should still run)")
            results.append(("ENV unset", "WRONG"))
        except subprocess.TimeoutExpired as e:
            out = (e.stdout or b"").decode(errors="replace")
            say(f"  ENV unset: still running at 3 s (the 100 s default), INVALID printed: {'INVALID' in out}")
            results.append(("ENV unset", "DEFAULT" if "INVALID" not in out else "WRONG"))
    say("summary: " + "; ".join(f"{i} {v}" for i, v in results))

if MODE == "bites":
    PRETOUCH_ZERO = ("Fate { ordinal: 0, dir: 0, kind: Kind::Delivered, overtaken: false, index: 0, t_in_ns: 0, "
                     "due_ns: 0, out_ns: 0, len: 0, errno: 0 }")
    U, S, C, E = "unit", "self", "self-chrt", "env"
    # (id, what, kind, test, [(file, old, new)], text the failure must carry)
    BITES = [
        # Item 5: T6's new asserts (the reviewers' X1 and X2, re-run).
        ("X1", "agree() always says disagree", U, T6,
         [(TESTS, 'if (e_would == "PASS") == p999_pass {', 'if false && (e_would == "PASS") == p999_pass {')],
         "E MISS and p99.9 FAIL"),
        ("X2", "the loss share over the Delivered count, not every fate", U, T6,
         [(TESTS, "count128 > 0 && (lo * count128..=hi * count128).contains(&lost),",
           "count128 > 0 && (lo * n128..=hi * n128).contains(&lost),")], "106 lost of 10,106"),
        # Item 6: samples() counts a Delivered fate released before it arrived; T6 asserts the count.
        ("E1", "an early fate not counted (taken as a delay of 0)", U, T6,
         [(TESTS, "                None => s.early += 1,", "                None => s.delay.push(0),")],
         "is counted"),
        ("T6b", "(the builder's T6b form) Lost fates taken as Delivered", U, T6,
         [(TESTS, "        if f.kind == Kind::Delivered {\n            // Only after the filter",
           "        if f.kind == Kind::Delivered || f.kind == Kind::Lost {\n            // Only after the filter")],
         "released before it arrived"),
        ("T6b2", "(the test-engineer's T6b form) Lost kept as delays through saturating_sub", U, T6,
         [(TESTS, "            match f.out_ns.checked_sub(f.t_in_ns) {\n                Some(d) => s.delay.push(d),\n"
                  "                None => s.early += 1,\n            }",
           "            s.delay.push(f.out_ns.saturating_sub(f.t_in_ns));"),
          (TESTS, "        if f.kind == Kind::Delivered {\n            // Only after the filter",
           "        if matches!(f.kind, Kind::Delivered | Kind::Lost) {\n            // Only after the filter")],
         "left == right"),
        # The self-test's run conditions (release, NPT_L2_SELFTEST_SECS=3).
        ("S2", "(re-run: order kept) the stop with no d + a + 500 ms wait", S, SELF,
         [(TESTS, "        std::thread::sleep(longest + Duration::from_millis(500));\n", "        let _ = longest;\n")],
         "every datagram accounted for"),
        ("S3", "(re-run) the loss band at twice P1's ppm", S, SELF,
         [(TESTS, "let (lo, hi) = loss_band(sm[d].fates, ppm);", "let (lo, hi) = loss_band(sm[d].fates, 2 * ppm);")],
         "outside P1's band"),
        ("S4", "(re-run) the fate log's capacity n (too small)", S, SELF,
         [(TESTS, "    let cap = (2 * n + 1_000) as usize;", "    let cap = n as usize;")], "the fate log filled"),
        ("S5", "(re-run) the pre-touch's drain removed (log full from the start)", S, SELF,
         [(TESTS, "    s.drain_fate_log(|_| {});\n    let rt = runtime(4);", "    let rt = runtime(4);")],
         "the fate log filled"),
        ("S5b", "one pre-touch entry left in the log after the drain", S, SELF,
         [(TESTS, "    s.drain_fate_log(|_| {});\n    let rt = runtime(4);",
           "    s.drain_fate_log(|_| {});\n    let _ = s.shared.lock().im.fates.push(" + PRETOUCH_ZERO + ");\n"
           "    let rt = runtime(4);")], "the fate log holds"),
        ("S6", "(re-run) the pre-touch skipped", S, SELF,
         [(TESTS, "        while k < cap && st.im.fates.push(zero) {", "        while false && k < cap && st.im.fates.push(zero) {")],
         "pre-touched with 0 of"),
        ("S7", "the release thread leaves its slack at the default (start 50,000 ns)", S, SELF,
         [(SOCKET, "thread::set_own_slack(t, sh.cfg.timer_slack_ns)", "thread::set_own_slack(t, 0)")],
         "the slack read back 50000 ns at start"),
        ("S9", "the header-time read writes 50,000 ns (now 50,000, start 1)", S, SELF,
         [(SOCKET, "let write = st.slack_write.take().unwrap_or(0);", "let write = st.slack_write.take().unwrap_or(50_000);")],
         "1 ns at start and 50000 ns now"),
        ("E2", "every live delay under 40 ms read as released before it arrived", S, SELF,
         [(TESTS, "            match f.out_ns.checked_sub(f.t_in_ns) {", "            match f.out_ns.checked_sub(f.t_in_ns + 40 * MS) {")],
         "released before they arrived"),
        # The slack condition under the recorded switch still checks the start value (S7 + chrt from outside).
        ("S7c", "S7 with chrt -f -p 1 from outside ~1 s into 6 s (start 50,000 under SCHED_FIFO)", C, SELF,
         [(SOCKET, "thread::set_own_slack(t, sh.cfg.timer_slack_ns)", "thread::set_own_slack(t, 0)")],
         "the slack read back 50000 ns at start"),
        # Item 4: a non-UTF-8 value taken as the default again (the old Err(_) arm).
        ("U4", "a non-UTF-8 NPT_L2_SELFTEST_SECS taken as the default (Err(_))", E, SELF,
         [(TESTS, "        Err(std::env::VarError::NotPresent) => 100u64,", "        Err(_) => 100u64,")], ""),
    ]
    originals = {p: p.read_bytes() for p in FIXED}
    results = []
    for bid, what, kind, test, edits, says in BITES:
        if ONLY and bid not in ONLY:
            continue
        say(f"{bid}: {what} ({test})")
        planted = {}
        for path, old, new in edits:
            src = planted.get(path, originals[path].decode())
            assert src.count(old) == 1, f"{bid}: anchor found {src.count(old)} times in {path.name}"
            planted[path] = src.replace(old, new, 1)
        try:
            for path, src in planted.items():
                path.write_text(src)
            b = build(release=kind != U)
            if b is None:
                verdict = "DID NOT COMPILE"
            elif kind == U:
                code, out, _ = run(b, test, False, timeout=300)
                (NOTES / f"u3d-fix-bite-{bid}.log").write_text(out)
                verdict = show(bid, code, out, True, says)
            elif kind == S:
                code, out, _ = run(b, test, True, {"NPT_L2_SELFTEST_SECS": "3"}, timeout=300)
                (NOTES / f"u3d-fix-bite-{bid}.log").write_text(out)
                verdict = show(bid, code, out, True, says)
            elif kind == C:
                code, out = chrt_after(b, 6, f"bite-{bid}")
                verdict = show(bid, code, out, True, says)
            else:
                code, out, secs = run(b, test, True, {"NPT_L2_SELFTEST_SECS": "\udcff"}, timeout=5)
                started = "U3d self-test:" in out or code == "timeout"
                say(f"  {'BITES' if code == 'timeout' else 'MISSED'} {bid}: value b'\\xff': exit {code} after "
                    f"{secs:.1f} s (timeout = still running: the default taken); INVALID printed: {'INVALID' in out}")
                verdict = "BITES" if code == "timeout" and "INVALID" not in out else "MISSED"
        finally:
            for path in planted:
                path.write_bytes(originals[path])
        restored = {p.name: sha(p) == FIXED[p] for p in planted}
        say(f"    restored: {restored}")
        assert all(restored.values()), f"{bid}: not restored"
        results.append((bid, verdict))
    for p, h in FIXED.items():
        assert sha(p) == h, p
    say("summary: " + "; ".join(f"{b} {v}" for b, v in results))

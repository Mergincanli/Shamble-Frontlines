# U3d fix-pass review (test-engineer): does each fix (items 1-6 of the fix pass) come with a test or run condition that
# fails when it is reverted? Each revert or plant is made in place, alone; its one test runs alone; the result is
# recorded; the file is restored and its sha256 checked against the fixed state. Plants are in my own forms where the
# fixer already ran theirs. Build rules (u3d-plan.md §7.1, Mustafa's rule of 2026-11-09): CARGO_TARGET_DIR is the
# shared folder; every .rs under crates/ is touched (mtime only) before each build; df before each build (stop under
# 2 GB); each run must show exactly one test, each release binary must list the same test count as the one before.
# Run: python3 -I <this> <spike dir> <group: clean|bites|runsh> [id ...]
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
GROUP = sys.argv[2]
ONLY = set(sys.argv[3:])
HERE = pathlib.Path(__file__).resolve().parent
OUTF = HERE / "u3d-fix-review-bites-output.txt"
TARGET = "/home/user/Shamble-Frontlines/.gate/shared-target"
L2 = SPIKE / "crates/net/src/l2"
TESTS, SOCKET, STATS = L2 / "socket_tests.rs", L2 / "socket.rs", L2 / "stats.rs"
RUNSH = SPIKE / "results/u3d-timing/run.sh"
T = "l2::socket::tests::"
SELF = "l2_timing_self_test"
T6 = "the_cross_check_reads_delay_plus_lateness"
# The fixed state (the fixer's final sha256, re-read at my baseline).
WANT = {
    TESTS: "ca17a9f6f6be03224c8921ef21f1731c1c0d86f3161c7bf7add0afb6b032b2b9",
    SOCKET: "7ad7cb22e4316c9f98f4d3abedc36fcab8ddea312239dd5e0b95f2c0856e76f9",
    STATS: "27dfd3266a0647b28cd642a062ffa45c7caac8da31a55ec4e62f9983187e9e22",
    RUNSH: "f4a73a5b2b558a50a9a2348695fb8049c44d8b927c1469abffca85f8254adfe3",
}
LISTED = {"count": None}


def log(s=""):
    print(s, flush=True)
    with OUTF.open("a") as f:
        f.write(s + "\n")


def sha(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()


def df_ok():
    gb = shutil.disk_usage("/home/user").free / 1e9
    log(f"       df /home/user: {gb:.1f} GB free")
    if gb < 2:
        sys.exit("under 2 GB free: stop")


def touch_rs():
    for p in (SPIKE / "crates").rglob("*.rs"):
        os.utime(p)
    return time.time()


def cargo_env():
    return dict(os.environ, CARGO_TARGET_DIR=TARGET)


def build_release():
    df_ok()
    t = touch_rs()
    r = subprocess.run(["cargo", "+1.99.1", "test", "--release", "--locked", "-p", "npt-net", "--lib", "--no-run",
                        "--message-format=json"], cwd=SPIKE, env=cargo_env(), capture_output=True, text=True)
    if r.returncode != 0:
        return None, "build failed: " + "\n".join(l for l in r.stderr.splitlines() if "error" in l)[:1500]
    exe = [m["executable"] for m in map(json.loads, filter(None, r.stdout.splitlines()))
           if m.get("reason") == "compiler-artifact" and m.get("executable") and m["target"]["name"] == "npt_net"
           and m["profile"]["test"]]
    assert len(exe) == 1, exe
    b = exe[0]
    fresh = os.stat(b).st_mtime > t
    lst = subprocess.run([b, "--list"], capture_output=True, text=True).stdout
    n = sum(1 for l in lst.splitlines() if l.endswith(": test"))
    ign = subprocess.run([b, "--ignored", "--list"], capture_output=True, text=True).stdout
    has_self = f"{T}{SELF}: test" in ign.splitlines()
    prev = LISTED["count"]
    LISTED["count"] = n
    log(f"       release binary {pathlib.Path(b).name} newer than the touch: {fresh}; lists {n} tests"
        f" (before: {prev}); ignored lists the self-test: {has_self}")
    assert fresh and has_self and (prev is None or prev == n)
    return b, ""


def prints_before_panic(out):
    before = out.split("panicked at")[0] if "panicked at" in out else out
    return sum(1 for l in before.splitlines() if l.startswith("U3d self-test:") or l.startswith("  "))


def panic_msg(out):
    m = re.search(r"panicked at ([^\n]+):\n((?:.*\n){1,4})", out)
    if not m:
        return "-", "(no panic)"
    return m.group(1), " | ".join(l.strip() for l in m.group(2).splitlines() if l.strip())[:600]


def run_self(b, secs, mode="plain", tag=""):
    env = dict(os.environ, NPT_L2_SELFTEST_SECS=str(secs))
    args = [b, "--ignored", "--nocapture", "--exact", T + SELF]
    extra = ""
    if mode == "born":
        args = ["chrt", "-f", "1"] + args
    p = subprocess.Popen(args, cwd=SPIKE, env=env, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
    if mode == "chrt_after":
        tid = None
        for _ in range(300):
            for c in pathlib.Path(f"/proc/{p.pid}/task").glob("*/comm"):
                try:
                    if c.read_text().strip() == "npt-l2-release":
                        tid = c.parent.name
                except OSError:
                    pass
            if tid:
                break
            time.sleep(0.01)
        time.sleep(1)
        before = subprocess.run(["chrt", "-p", tid], capture_output=True, text=True).stdout.strip().replace("\n", "; ")
        ch = subprocess.run(["chrt", "-f", "-p", "1", tid], capture_output=True, text=True)
        after = subprocess.run(["chrt", "-p", tid], capture_output=True, text=True).stdout.strip().replace("\n", "; ")
        try:
            sl = pathlib.Path(f"/proc/{tid}/timerslack_ns").read_text().strip()
        except OSError as e:
            sl = f"unreadable ({e})"
        extra = f"release tid {tid}; before: {before}; chrt rc {ch.returncode}; after: {after}; slack read from outside {sl}"
    out, _ = p.communicate()
    (HERE / f"u3d-fix-review-{tag}.log").write_text(out + f"\nexit {p.returncode}\n")
    return p.returncode, out, extra


def run_env(b, val, timeout):
    env = {k.encode(): v.encode() for k, v in os.environ.items() if k != "NPT_L2_SELFTEST_SECS"}
    if val is not None:
        env[b"NPT_L2_SELFTEST_SECS"] = val
    t0 = time.time()
    try:
        r = subprocess.run([b, "--ignored", "--nocapture", "--exact", T + SELF], cwd=SPIKE, env=env,
                           capture_output=True, timeout=timeout)
        code, out = r.returncode, r.stdout.decode(errors="replace") + r.stderr.decode(errors="replace")
    except subprocess.TimeoutExpired as e:
        code = "still running (killed at timeout)"
        out = (e.stdout or b"").decode(errors="replace") + (e.stderr or b"").decode(errors="replace")
    dt = time.time() - t0
    inv = re.search(r"INVALID: NPT_L2_SELFTEST_SECS=[^\n]*", out)
    return code, dt, inv.group(0)[:160] if inv else "no INVALID", prints_before_panic(out)


def run_unit(test):
    df_ok()
    t = touch_rs()
    r = subprocess.run(["cargo", "+1.99.1", "test", "--locked", "-p", "npt-net", "--lib", "--", T + test, "--exact"],
                       cwd=SPIKE, env=cargo_env(), stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
    out = r.stdout
    m = re.search(r"Running unittests src/lib.rs \(([^)]+)\)", out)
    fresh = None
    if m:
        p = pathlib.Path(m.group(1))
        p = p if p.is_absolute() else SPIKE / p
        fresh = os.stat(p).st_mtime > t
    return r.returncode, out, fresh


def plant(edits):
    originals = {p: p.read_bytes() for p, _, _ in edits}
    planted = {}
    for path, old, new in edits:
        src = planted.get(path, originals[path].decode())
        assert src.count(old) == 1, f"anchor found {src.count(old)} times in {path.name}: {old[:60]!r}"
        planted[path] = src.replace(old, new, 1)
    for path, src in planted.items():
        path.write_text(src)
    return originals


def restore(originals):
    for path, data in originals.items():
        path.write_bytes(data)
    ok = {p.name: sha(p) == WANT[p] for p in originals}
    log(f"       restored, sha256 == fixed state: {ok}")
    assert all(ok.values())


# ---- Anchors (the fixed text) and the old / planted text.
NEW_SLACK = """    assert!(
        slack.iter().all(|v| slack_ok.contains(v)),
        "the slack read back {} ns at start and {} ns now under {sched}, outside {slack_ok:?}: {INVALID}",
        slack[0],
        slack[1]
    );
"""
OLD_SLACK = """    assert_eq!(
        value(&h, "l2_timer_slack_start_ns"),
        u64::from(!rt_policy),
        "the slack read back at start under {sched}: {INVALID}"
    );
"""
STARTED = "    let _ = started.send(Ok(()));\n    let mut batch: Vec<Entry<Packet>> = Vec::with_capacity(BATCH);\n"
STARTED_P = ("    let _ = started.send(Ok(()));\n    if let Some(t) = tid {\n        let _ = thread::set_own_slack(t, 50_000);\n"
             "    }\n    let mut batch: Vec<Entry<Packet>> = Vec::with_capacity(BATCH);\n")
DRAIN = "    s.drain_fate_log(|_| {});\n    let rt = runtime(4);\n"
DRAIN_LEFT = """    s.drain_fate_log(|_| {});
    {
        let left = Fate {
            ordinal: 0,
            dir: DOWN as u8,
            kind: Kind::Delivered,
            overtaken: false,
            index: 0,
            t_in_ns: 0,
            due_ns: 0,
            out_ns: 0,
            len: 0,
            errno: 0,
        };
        assert!(s.shared.lock().im.fates.push(left));
    }
    let rt = runtime(4);
"""
FULL_EARLY = """    // The log complete before the loss band, which counts the log's own fates.
    assert_eq!(
        value(&r, names::FATE_LOG_FULL),
        0,
        "the fate log filled: {INVALID}"
    );
"""
FATES_N = """        assert_eq!(
            sm[d].fates, n,
            "{dir}: the fate log holds {} fates of the {n} sent: {INVALID}",
            sm[d].fates
        );
"""
POLLER_END = """        "the poller's count: {INVALID}"
    );
"""
FULL_OLD = """    assert_eq!(
        value(&r, names::FATE_LOG_FULL),
        0,
        "the fate log filled: {INVALID}"
    );
"""
NOT_UNICODE = """        Err(std::env::VarError::NotPresent) => 100u64,
        Err(std::env::VarError::NotUnicode(v)) => panic!(
            "INVALID: NPT_L2_SELFTEST_SECS={v:?} is not UTF-8, so not a whole number of seconds in 1-600; the run is \\
             INVALID, not a lateness verdict"
        ),
"""
EARLY_MATCH = """            match f.out_ns.checked_sub(f.t_in_ns) {
                Some(d) => s.delay.push(d),
                None => s.early += 1,
            }
"""
OLD_PANIC = """            let d = f.out_ns.checked_sub(f.t_in_ns);
            s.delay
                .push(d.unwrap_or_else(|| panic!("a fate released before it arrived: {f:?}")));
"""

# (id, what, kind, edits, must fail? (None: informative), what the failure must say)
BITES = [
    ("F1-R", "item 1 reverted (start-only slack assert, 0 under RT) + chrt -f -p 1 after start", "chrt_after",
     [(TESTS, NEW_SLACK, OLD_SLACK)], True, "the slack read back at start under SCHED_FIFO"),
    ("F1-P", "release thread raises its slack to 50,000 after the start read (now != 1)", "self",
     [(SOCKET, STARTED, STARTED_P)], True, "1 ns at start and 50000 ns now under SCHED_OTHER"),
    ("F2-A", "one DOWN entry left in the fate log after the pre-touch's drain (S5b, my form)", "self",
     [(TESTS, DRAIN, DRAIN_LEFT)], True, "down: the fate log holds 30001 fates of the 30000 sent"),
    ("F2-R", "item 2 reverted (fate_log_full after the loop, no fates == n) + the same DOWN entry", "self",
     [(TESTS, DRAIN, DRAIN_LEFT), (TESTS, FULL_EARLY, ""), (TESTS, FATES_N, ""),
      (TESTS, POLLER_END, POLLER_END + FULL_OLD)], None, ""),
    ("F3-P", "pre-touch stops at half the capacity", "self",
     [(TESTS, "        while k < cap && st.im.fates.push(zero) {", "        while k < cap / 2 && st.im.fates.push(zero) {")],
     True, "pre-touched with 30500 of its 61000 entries"),
    ("F4-R", "item 4 reverted (Err(_) => 100) + NPT_L2_SELFTEST_SECS=\\xff", "env",
     [(TESTS, NOT_UNICODE, "        Err(_) => 100u64,\n")], None, ""),
    ("F5-X1", "agree() always says disagree", "unit",
     [(TESTS, 'if (e_would == "PASS") == p999_pass {', 'if false && (e_would == "PASS") == p999_pass {')],
     True, "E MISS and p99.9 FAIL"),
    ("F5-X2", "loss share over the Delivered (n128) instead of every fate", "unit",
     [(TESTS, "count128 > 0 && (lo * count128..=hi * count128).contains(&lost),",
       "count128 > 0 && (lo * n128..=hi * n128).contains(&lost),")], True, "106 lost of 10,106"),
    ("F6-R", "samples() reverted to the old panic on an early fate", "unit",
     [(TESTS, EARLY_MATCH, OLD_PANIC)], True, "a fate released before it arrived: Fate"),
    ("F6-P", "an early fate dropped, not counted", "unit",
     [(TESTS, "                None => s.early += 1,\n", "                None => {}\n")], True,
     "down: the one fate released before it arrived is counted"),
    ("F6-S", "delays under 25 ms read as early (self-test)", "self",
     [(TESTS, "            match f.out_ns.checked_sub(f.t_in_ns) {",
       "            match f.out_ns.checked_sub(f.t_in_ns + 25 * MS) {")], True,
     "Delivered fates released before they arrived"),
]


def start_check():
    for p, h in WANT.items():
        got = sha(p)
        log(f"start {p.name} sha256 {got[:16]}... {'== fixed state' if got == h else '!= FIXED STATE: stop'}")
        assert got == h


def group_clean():
    log("=== clean: the fixed code, release")
    b, err = build_release()
    assert b, err
    log(f"       binary sha256 {sha(pathlib.Path(b))[:16]}...")
    for t in ["the_task_poller_takes_lateness_where_the_task_runs", "the_fate_log_and_the_histogram_agree",
              "the_verdict_line_is_per_direction_and_inclusive", "nearest_rank_and_loss_band_on_fixed_inputs",
              "the_exact_excess_on_fixed_samples", T6, "a_held_lock_makes_both_directions_late"]:
        lst = subprocess.run([b, "--list"], capture_output=True, text=True).stdout.splitlines()
        log(f"       listed {t}: {T + t + ': test' in lst}")
    for tag, secs, mode in [("clean-3s", 3, "plain"), ("chrt-after", 6, "chrt_after"), ("chrt-born", 3, "born")]:
        code, out, extra = run_self(b, secs, mode, tag)
        at, msg = panic_msg(out)
        sl = [l.strip() for l in out.splitlines() if l.startswith("  slack now")]
        log(f"RUN {tag} ({secs} s, {mode}): exit {code}; prints {prints_before_panic(out)}; {extra}")
        log(f"       {sl[0] if sl else 'no slack line'}")
        log(f"       panic at {at}: {msg}")
        for l in out.splitlines():
            if "E with L0 = 0, the judged form" in l and "bar" in l:
                log("       " + l.strip())
    for val, to, label in [(b"\xff", 20, "\\xff"), (None, 3, "unset"), (b"0", 20, "0"), (b"601", 20, "601"),
                           (b"abc", 20, "abc")]:
        code, dt, inv, pr = run_env(b, val, to)
        log(f"ENV {label}: exit {code} after {dt:.2f} s; {inv}; self-test prints {pr}")


def group_bites():
    for bid, what, kind, edits, must_fail, says in BITES:
        if ONLY and bid not in ONLY:
            continue
        log(f"=== {bid}: {what} ({kind})")
        originals = None
        try:
            originals = plant(edits)
            if kind == "unit":
                code, out, fresh = run_unit(T6)
                ran = re.search(r"running (\d+) tests?", out)
                res = re.search(r"test result: \w+\. (\d+) passed; (\d+) failed", out)
                at, msg = panic_msg(out)
                compiled = "error[" not in out and "could not compile" not in out
                one = ran is not None and ran.group(1) == "1"
                failed = res is not None and res.group(2) == "1" and f"{T}{T6} ... FAILED" in out
                passed = res is not None and res.group(1) == "1" and res.group(2) == "0"
                log(f"       debug test binary newer than the touch: {fresh}; ran {ran.group(1) if ran else '?'};"
                    f" {'compiled' if compiled else 'DID NOT COMPILE'}; {res.group(0) if res else 'no result'}")
                log(f"       panic at {at}: {msg}")
                if not compiled:
                    log("       " + "\n       ".join(l for l in out.splitlines() if l.startswith("error"))[:1500])
                ok_fail = compiled and one and failed and says in out and fresh
                verdict = ("BITES" if ok_fail else "MISSED") if must_fail else ("PASSED (informative)" if passed else "FAILED (informative)")
            else:
                b, err = build_release()
                if not b:
                    log("       " + err)
                    verdict = "DID NOT COMPILE"
                elif kind == "env":
                    code, dt, inv, pr = run_env(b, b"\xff", 5)
                    log(f"       \\xff: exit {code} after {dt:.2f} s; {inv}; self-test prints {pr}")
                    verdict = "INVALID (informative)" if "INVALID" in inv else "RUNS THE DEFAULT (informative)"
                else:
                    mode = "chrt_after" if kind == "chrt_after" else "plain"
                    secs = 6 if kind == "chrt_after" else 3
                    code, out, extra = run_self(b, secs, mode, bid)
                    at, msg = panic_msg(out)
                    ran = re.search(r"running (\d+) tests?", out)
                    one = ran is not None and ran.group(1) == "1"
                    sl = [l.strip() for l in out.splitlines() if l.startswith("  slack now")]
                    log(f"       ran {ran.group(1) if ran else '?'}; exit {code}; prints before the panic "
                        f"{prints_before_panic(out)}; {extra}")
                    log(f"       {sl[0] if sl else 'no slack line'}")
                    log(f"       panic at {at}: {msg}")
                    if must_fail:
                        verdict = "BITES" if (one and code != 0 and says in out) else "MISSED"
                    else:
                        verdict = "PASSED (informative)" if code == 0 else "FAILED (informative)"
        finally:
            if originals:
                restore(originals)
        log(f"RESULT {bid}: {verdict}")


def group_runsh():
    nonrs = [p for p in (SPIKE / "crates").rglob("*") if p.is_file() and p.suffix != ".rs"]
    rs = list((SPIKE / "crates").rglob("*.rs"))
    for bid, edits in [("SH-fixed", []), ("SH-R", [(RUNSH, '  find "$SPIKE/crates" -type f -exec touch {} +',
                                                   '  find "$SPIKE/crates" -type f -name \'*.rs\' -exec touch {} +')])]:
        if ONLY and bid not in ONLY:
            continue
        log(f"=== {bid}: run.sh check mode ({'item 6 reverted: -name *.rs' if edits else 'fixed'})")
        originals = plant(edits) if edits else None
        try:
            df_ok()
            touch_rs()
            time.sleep(1.2)
            t0 = time.time()
            env = dict(os.environ, CARGO_TARGET_DIR=TARGET, U3D_CHECK="1", NPT_L2_SELFTEST_SECS="2", NPT_L2_SECS="2",
                       ROUNDS="1")
            before = set((SPIKE / "results/u3d-timing").iterdir())
            r = subprocess.run(["bash", str(RUNSH)], cwd=SPIKE, env=env, capture_output=True, text=True)
            new = sorted(set((SPIKE / "results/u3d-timing").iterdir()) - before)
            old_nonrs = sum(1 for p in nonrs if p.stat().st_mtime < t0)
            old_rs = sum(1 for p in rs if p.stat().st_mtime < t0)
            log(f"       run.sh exit {r.returncode}; non-.rs files under crates older than T0: {old_nonrs} of {len(nonrs)};"
                f" .rs older than T0: {old_rs} of {len(rs)}; new records {[p.name for p in new]}")
            for p in new:
                dest = HERE / f"u3d-fix-review-runsh-{bid}-{p.name}"
                txt = p.read_text()
                exits = re.findall(r"exit[_ ]status[^\n]*|^exit [^\n]*", txt, re.M)
                log(f"       record exits: {exits[:6]}; moved to {dest.name}")
                os.replace(p, dest)
            left = sorted(x.name for x in (SPIKE / "results/u3d-timing").iterdir())
            log(f"       results/u3d-timing now holds {left}")
            if r.returncode != 0:
                log("       stderr: " + r.stderr[-800:])
        finally:
            if originals:
                restore(originals)


start_check()
{"clean": group_clean, "bites": group_bites, "runsh": group_runsh}[GROUP]()
for p, h in WANT.items():
    assert sha(p) == h, p
log(f"end: every file == fixed state ({GROUP})")

# U3d build proof (test-engineer): the bites of T1-T7 (u3d-plan.md §4) plus the ones the plan missed (the self-test's
# own run conditions, the premises of T1, T2 and T7). Written independently of the builder's u3d-bites.py: several
# bites use other forms of the same defect. Each bite is planted in place in the worktree, alone; the one test it
# targets runs alone; the failure (or its absence) is recorded with its message; the file is restored and its sha256
# checked against the builder's. Build rules (u3d-plan.md §7.1): CARGO_TARGET_DIR is the shared folder; every file
# under crates/ is touched (mtime only) before each build; df before each build (stop under 2 GB); each run must show
# exactly one test.
# Run: python3 -I <this> <spike dir> [bite id ...]
import hashlib
import os
import pathlib
import re
import shutil
import subprocess
import sys

SPIKE = pathlib.Path(sys.argv[1]).resolve()
ONLY = set(sys.argv[2:])
TARGET = "/home/user/Shamble-Frontlines/.gate/shared-target"
L2 = SPIKE / "crates/net/src/l2"
TESTS, SOCKET, STATS = L2 / "socket_tests.rs", L2 / "socket.rs", L2 / "stats.rs"
T = "l2::socket::tests::"
SELF = "l2_timing_self_test"
# The builder's sha256 (u3d-build.md, u3d-bites-output.txt), which every restore must give back.
WANT = {
    TESTS: "312949530016f993a1330983626b5a803acb8042db609a2cdc39f57e2fa90625",
    SOCKET: "7ad7cb22e4316c9f98f4d3abedc36fcab8ddea312239dd5e0b95f2c0856e76f9",
    STATS: "27dfd3266a0647b28cd642a062ffa45c7caac8da31a55ec4e62f9983187e9e22",
}

PARK = """    rt.spawn_blocking(move || {
        let waker = Waker::from(Arc::new(Unpark(std::thread::current())));
        let mut cx = Context::from_waker(&waker);
        let mut poller = std::pin::pin!(poller);
        loop {
            match poller.as_mut().poll(&mut cx) {
                Poll::Ready(p) => break p,
                Poll::Pending => std::thread::park(),
            }
        }
    })
}"""

# Hazen's interpolated quantile (q × n − 0.5), another common rule than the builder's (n − 1) × q.
HAZEN = """    let h = (sorted.len() as f64 * num as f64 / den as f64 - 0.5).max(0.0);
    let lo = (h.floor() as usize).min(sorted.len() - 1);
    let hi = (lo + 1).min(sorted.len() - 1);
    (sorted[lo] as f64 + (h - lo as f64) * (sorted[hi] as f64 - sorted[lo] as f64)) as u64"""

DELIVERED_BLOCK = """        if f.kind == Kind::Delivered {
            // Only after the filter: a dropped fate has out 0, and `0 − t_in` would underflow (R-F5).
            let d = f.out_ns.checked_sub(f.t_in_ns);
            s.delay
                .push(d.unwrap_or_else(|| panic!("a fate released before it arrived: {f:?}")));
        }"""

U = "unit"
S = "self"
# (id, what, kind, test, [(file, old, new), ...], must the test fail?, what its failure must say)
BITES = [
    # T1: planted inside start_poller, so each reaches the self-test's poller too (gap 1).
    ("T1a", "poller on a blocking-pool thread with a parked-thread waker", U, "the_task_poller_takes_lateness_where_the_task_runs",
     [(TESTS, "    rt.spawn(poller)\n}", PARK)], True, "inbound lateness"),
    ("T1b", "poller under Handle::block_on on its own thread", U, "the_task_poller_takes_lateness_where_the_task_runs",
     [(TESTS, "    rt.spawn(poller)\n}", "    let h = rt.clone();\n    rt.spawn_blocking(move || h.block_on(poller))\n}")],
     True, "inbound lateness"),
    ("T1x", "(extra) the blocker lets go at the drain instead of drain + 110 ms", U, "the_task_poller_takes_lateness_where_the_task_runs",
     [(TESTS, "Some(first) if Instant::now() >= first + Duration::from_millis(110) => {", "Some(_first) => {")],
     True, "inbound lateness"),
    # T2
    ("T2a", "L taken as out - t_in", U, "the_fate_log_and_the_histogram_agree",
     [(TESTS, "s.late.push(f.out_ns.saturating_sub(f.due_ns));", "s.late.push(f.out_ns.saturating_sub(f.t_in_ns));")],
     True, "the fate log's E"),
    ("T2b", "Lost counted in the E set (in_e_set)", U, "the_fate_log_and_the_histogram_agree",
     [(TESTS, "matches!(kind, Kind::Delivered | Kind::SendError | Kind::WouldBlock)",
       "matches!(kind, Kind::Delivered | Kind::SendError | Kind::WouldBlock | Kind::Lost)")], True, "the E set"),
    ("T2x", "(extra) drive's accounting wait removed (gap 5)", U, "the_fate_log_and_the_histogram_agree",
     [(TESTS, "    while !accounted(&r) && Instant::now() < end {", "    while false && !accounted(&r) && Instant::now() < end {")],
     None, ""),
    # T3
    ("T3a", "< for <=", U, "the_verdict_line_is_per_direction_and_inclusive",
     [(TESTS, "if e_ns <= BAR_NS {", "if e_ns < BAR_NS {")], True, "assertion"),
    ("T3b", "pooled mean", U, "the_verdict_line_is_per_direction_and_inclusive",
     [(TESTS, "    e.map(would_one)\n", "    [would_one((e[0] + e[1]) / 2); 2]\n")], True, "assertion"),
    # T4
    ("T4a", "floor rank (saturating_div)", U, "nearest_rank_and_loss_band_on_fixed_inputs",
     [(TESTS, "        .div_ceil(u128::from(den))\n", "        .saturating_div(u128::from(den))\n")], True, "assertion"),
    ("T4b", "interpolated quantile (Hazen, q n - 0.5)", U, "nearest_rank_and_loss_band_on_fixed_inputs",
     [(TESTS, "    sorted[nearest_rank(sorted.len(), num, den) - 1]", HAZEN)], True, "the 35,640th"),
    ("T4c", "band (0, n)", U, "nearest_rank_and_loss_band_on_fixed_inputs",
     [(TESTS, "    (centre.saturating_sub(half) as u64, (centre + half) as u64)", "    let _ = (centre, half);\n    (0, n as u64)")],
     True, "assertion"),
    ("T4d", "sd rounded up", U, "nearest_rank_and_loss_band_on_fixed_inputs",
     [(TESTS, "    let half = 5 * variance.isqrt();", "    let r = variance.isqrt();\n    let half = 5 * (r + u128::from(r * r < variance));")],
     True, "assertion"),
    ("T4e", "sd in f64 from the integer variance", U, "nearest_rank_and_loss_band_on_fixed_inputs",
     [(TESTS, "    let half = 5 * variance.isqrt();", "    let half = (5.0 * (variance as f64).sqrt()) as u128;")], True, "assertion"),
    ("T4f", "sd from f64 p (the plan's ±497 form)", U, "nearest_rank_and_loss_band_on_fixed_inputs",
     [(TESTS, "    let half = 5 * variance.isqrt();",
       "    let _ = variance;\n    let q = p as f64 / 1e6;\n    let half = (5.0 * (n as f64 * q * (1.0 - q)).sqrt()) as u128;")], True, "assertion"),
    # T5
    ("T5a", "L0 swapped", U, "the_exact_excess_on_fixed_samples",
     [(TESTS, "        at_p50: excess(late, p50),\n        at_zero: excess(late, 0),",
       "        at_p50: excess(late, 0),\n        at_zero: excess(late, p50),")], True, "assertion"),
    ("T5b", "divided by the over count", U, "the_exact_excess_on_fixed_samples",
     [(TESTS, "    (total / late.len() as u128) as u64",
       "    (total / late.iter().filter(|&&l| l > c).count().max(1) as u128) as u64")], True, "assertion"),
    ("T5c", "L0 = the mean", U, "the_exact_excess_on_fixed_samples",
     [(TESTS, "    let p50 = quantile(late, 1, 2);",
       "    let p50 = (late.iter().map(|&l| u128::from(l)).sum::<u128>() / late.len().max(1) as u128) as u64;")],
     True, "assertion"),
    ("T5d", "allowance not subtracted above the threshold", U, "the_exact_excess_on_fixed_samples",
     [(TESTS, "    let total: u128 = late.iter().map(|&l| u128::from(l.saturating_sub(c))).sum();",
       "    let total: u128 = late.iter().filter(|&&l| l > c).map(|&l| u128::from(l - l0)).sum();")], True, "assertion"),
    # T6
    ("T6a", "delay = due - t_in", U, "the_cross_check_reads_delay_plus_lateness",
     [(TESTS, "let d = f.out_ns.checked_sub(f.t_in_ns);", "let d = f.due_ns.checked_sub(f.t_in_ns);")], True, "assertion"),
    ("T6b", "Lost entries kept as delays (saturating, no panic)", U, "the_cross_check_reads_delay_plus_lateness",
     [(TESTS, DELIVERED_BLOCK,
       "        if matches!(f.kind, Kind::Delivered | Kind::Lost) {\n            s.delay.push(f.out_ns.saturating_sub(f.t_in_ns));\n        }")],
     True, "assertion"),
    ("T6c", "count over Delivered", U, "the_cross_check_reads_delay_plus_lateness",
     [(TESTS, "        s.fates += 1;\n", "        s.fates += u64::from(f.kind == Kind::Delivered);\n")], True, "assertion"),
    ("T6d", "band rule off by a term (p001 at d - 0.9a)", U, "the_cross_check_reads_delay_plus_lateness",
     [(TESTS, "        p001: at(d - a),", "        p001: at(d - 9 * a / 10),")], True, "assertion"),
    ("T6e", "band rule off by a term (mean within 10 %)", U, "the_cross_check_reads_delay_plus_lateness",
     [(TESTS, "        mean: (d - d / 20, d + d / 20),", "        mean: (d - d / 10, d + d / 10),")], True, "assertion"),
    # T7
    ("T7a", "down's release stamp = its due", U, "a_held_lock_makes_both_directions_late",
     [(SOCKET, "results.push((kind, errno, sh.now_ns()));", "results.push((kind, errno, e.due_ns));")], True, "down: due at"),
    ("T7b", "up's release stamp = its due", U, "a_held_lock_makes_both_directions_late",
     [(SOCKET, "st.im.finish(&e, Kind::Delivered, now, 0);", "st.im.finish(&e, Kind::Delivered, e.due_ns, 0);")],
     True, "up: due at"),
    ("T7c", "summarize skips the log buckets", U, "a_held_lock_makes_both_directions_late",
     [(STATS, "        if b < first {\n            continue;\n        }", "        if b < first || b >= LINEAR {\n            continue;\n        }")],
     True, "samples at least"),
    ("T7x", "(extra) the 'held' window without the lock", U, "a_held_lock_makes_both_directions_late",
     [(TESTS, "        let guard = s.shared.lock();\n        let t0 = s.now_ns();", "        let t0 = s.now_ns();"),
      (TESTS, "        drop(guard);\n        held = (t0, t1);", "        held = (t0, t1);")], True, "the release stamp does not see the stall"),
    # Print-only helpers' coverage (informative: a pass shows a branch no test reaches).
    ("X1", "(extra) agree() always says disagree", U, "the_cross_check_reads_delay_plus_lateness",
     [(TESTS, 'if (e_would == "PASS") == p999_pass {', 'if false && (e_would == "PASS") == p999_pass {')], None, ""),
    ("X2", "(extra) the loss share over the Delivered count, not every fate", U, "the_cross_check_reads_delay_plus_lateness",
     [(TESTS, "count128 > 0 && (lo * count128..=hi * count128).contains(&lost),",
       "count128 > 0 && (lo * n128..=hi * n128).contains(&lost),")], None, ""),
    # The self-test's run conditions (plan §3.1), in release, NPT_L2_SELFTEST_SECS=3: each must make the run INVALID
    # with every print first, except S6, which the plan does not make a condition.
    ("S1", "the pacers at 9,800 per second", S, SELF,
     [(TESTS, "    let rate = 10_000u64;\n    let n = rate * secs;\n    let profile = Profile::p1();",
       "    let rate = 9_800u64;\n    let n = rate * secs;\n    let profile = Profile::p1();")], True, "the rate fell"),
    ("S2", "the stop with no d + a + 500 ms wait", S, SELF,
     [(TESTS, "        std::thread::sleep(longest + Duration::from_millis(500));\n", "        let _ = longest;\n")],
     True, "INVALID"),
    ("S3", "the loss band at twice P1's ppm", S, SELF,
     [(TESTS, "let (lo, hi) = loss_band(sm[d].fates, ppm);", "let (lo, hi) = loss_band(sm[d].fates, 2 * ppm);")],
     True, "outside P1's band"),
    ("S4", "the fate log's capacity n (too small)", S, SELF,
     [(TESTS, "    let cap = (2 * n + 1_000) as usize;", "    let cap = n as usize;")], True, "INVALID"),
    ("S5", "the pre-touch's drain removed (log full from the start)", S, SELF,
     [(TESTS, "    s.drain_fate_log(|_| {});\n    let rt = runtime(4);", "    let rt = runtime(4);")], True, "INVALID"),
    ("S6", "the pre-touch skipped (no page touched first)", S, SELF,
     [(TESTS, "        while k < cap && st.im.fates.push(zero) {", "        while false && k < cap && st.im.fates.push(zero) {")],
     None, ""),
    ("S7", "the release thread leaves its slack at the default", S, SELF,
     [(SOCKET, "thread::set_own_slack(t, sh.cfg.timer_slack_ns)", "thread::set_own_slack(t, 0)")], True, "the slack read back"),
    ("S8", "the poller counts one per batch", S, SELF,
     [(TESTS, "                        got += n as u64;", "                        got += (n as u64).min(1);")], True, "the poller's count"),
]


def sha(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()


def df_ok():
    u = shutil.disk_usage("/home/user")
    gb = u.free / 1e9
    print(f"       df /home/user: {gb:.1f} GB free")
    if gb < 2:
        sys.exit("under 2 GB free: stop")


def touch():
    for p in (SPIKE / "crates").rglob("*"):
        if p.is_file():
            os.utime(p)


def run(kind, test, exact=True):
    df_ok()
    touch()
    env = dict(os.environ, CARGO_TARGET_DIR=TARGET)
    args = ["cargo", "+1.99.1", "test", "--locked", "-p", "npt-net", "--lib"]
    if kind == S:
        env["NPT_L2_SELFTEST_SECS"] = "3"
        args = ["cargo", "+1.99.1", "test", "--release", "--locked", "-p", "npt-net", "--lib"]
        tail = ["--", T + test, "--exact", "--ignored", "--nocapture"]
    else:
        tail = ["--", T + test] + (["--exact"] if exact else [])
    r = subprocess.run(args + tail, cwd=SPIKE, env=env, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
    return r.returncode, r.stdout


for p, h in WANT.items():
    got = sha(p)
    print(f"start {p.name} sha256 {got} {'== builder' if got == h else '!= BUILDER: stop'}")
    assert got == h
originals = {p: p.read_bytes() for p in WANT}
results = []
for bid, what, kind, test, edits, must_fail, says in BITES:
    if ONLY and bid not in ONLY:
        continue
    planted = {}
    for path, old, new in edits:
        src = planted.get(path, originals[path].decode())
        assert src.count(old) == 1, f"{bid}: anchor found {src.count(old)} times in {path.name}"
        planted[path] = src.replace(old, new, 1)
    try:
        for path, src in planted.items():
            path.write_text(src)
        code, out = run(kind, test)
    finally:
        for path in planted:
            path.write_bytes(originals[path])
    restored = {p.name: sha(p) == WANT[p] for p in planted}
    ran = re.search(r"running (\d+) tests?", out)
    result = re.search(r"test result: \w+\. (\d+) passed; (\d+) failed", out)
    panic = re.search(r"panicked at ([^\n]+):\n((?:.*\n){1,4})", out)
    msg = " | ".join(l.strip() for l in panic.group(2).splitlines() if l.strip())[:500] if panic else "(no panic)"
    compiled = "error[" not in out and "could not compile" not in out
    one = ran is not None and ran.group(1) == "1"
    failed = result is not None and result.group(2) == "1" and f"{T}{test} ... FAILED" in out
    passed = result is not None and result.group(1) == "1" and result.group(2) == "0"
    # Prints before the panic (self-test only): lines the test printed before "panicked at".
    before = out.split("panicked at")[0] if "panicked at" in out else out
    prints = sum(1 for l in before.splitlines() if l.startswith("U3d self-test:") or (l.startswith("  ") and kind == S))
    if must_fail is None:
        verdict = "FAILED (informative)" if failed else ("PASSED (informative)" if passed else "UNCLEAR")
    else:
        verdict = "BITES" if (compiled and one and failed and says in out) else "MISSED"
    results.append((bid, verdict))
    print(f"{verdict} {bid} {what}: {test}; ran {ran.group(1) if ran else '?'}; "
          f"{'compiled' if compiled else 'DID NOT COMPILE'}; {result.group(0) if result else 'no result'}; "
          f"says {says!r}: {says in out if says else '-'}")
    print(f"       at {panic.group(1) if panic else '-'}: {msg}")
    if kind == S:
        (pathlib.Path(__file__).parent / f"u3d-test-bites-{bid}.log").write_text(out)
        print(f"       self-test prints before the panic: {prints} (cargo's own indented lines included)")
        for l in before.splitlines():
            if "would" in l and "E with L0 = 0" in l and "bar" in l:
                print("       " + l.strip())
    if not compiled:
        print("       " + "\n       ".join(l for l in out.splitlines() if l.startswith("error"))[:1500])
    print(f"       restored: {restored}")
    assert all(restored.values()), f"{bid}: not restored"
    sys.stdout.flush()
for p, h in WANT.items():
    assert sha(p) == h, p
print("summary: " + "; ".join(f"{b} {v}" for b, v in results))

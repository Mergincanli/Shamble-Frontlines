# U3d build: the bites of T1-T7 (u3d-plan.md §4). Each bite is planted once in the builder's worktree, the one test it
# targets is run alone (debug, as check.mjs runs it), its failure is recorded with its panic message, and the file is
# restored and its sha256 checked against the original. Build rules (§7.1): CARGO_TARGET_DIR is the shared folder,
# every .rs under crates/ is touched before each build, and each run must show exactly one test, failing by name.
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
L2 = SPIKE / "crates/net/src/l2"
TESTS, SOCKET, STATS = L2 / "socket_tests.rs", L2 / "socket.rs", L2 / "stats.rs"
T = "l2::socket::tests::"

PARK = """    rt.spawn_blocking(move || {
        let mut poller = std::pin::pin!(poller);
        let waker = Waker::from(Arc::new(Unpark(std::thread::current())));
        let mut cx = Context::from_waker(&waker);
        loop {
            if let Poll::Ready(p) = poller.as_mut().poll(&mut cx) {
                return p;
            }
            std::thread::park();
        }
    })
}"""

INTERP = """    let h = (sorted.len() - 1) as f64 * num as f64 / den as f64;
    let lo = h.floor() as usize;
    let hi = (lo + 1).min(sorted.len() - 1);
    (sorted[lo] as f64 + (h - lo as f64) * (sorted[hi] - sorted[lo]) as f64) as u64"""

# (name, file, old, new, test, what the failure must say)
BITES = [
    ("T1a poller on its own thread, parked waker", TESTS, "    rt.spawn(poller)\n}", PARK,
     "the_task_poller_takes_lateness_where_the_task_runs", "inbound lateness"),
    ("T1b poller under block_on", TESTS, "    rt.spawn(poller)\n}",
     "    let h = rt.clone();\n    rt.spawn_blocking(move || h.block_on(poller))\n}",
     "the_task_poller_takes_lateness_where_the_task_runs", "inbound lateness"),
    ("T2a L = out - t_in", TESTS, "s.late.push(f.out_ns.saturating_sub(f.due_ns));",
     "s.late.push(f.out_ns.saturating_sub(f.t_in_ns));",
     "the_fate_log_and_the_histogram_agree", "the fate log's E"),
    ("T2b lost fates not filtered out", TESTS, "        if in_e_set(f.kind) {\n            s.late.push",
     "        if in_e_set(f.kind) || f.kind == Kind::Lost {\n            s.late.push",
     "the_fate_log_and_the_histogram_agree", "the E set"),
    ("T3a < for <=", TESTS, "if e_ns <= BAR_NS {", "if e_ns < BAR_NS {",
     "the_verdict_line_is_per_direction_and_inclusive", "assertion"),
    ("T3b pooled mean", TESTS, "    e.map(would_one)\n", "    [would_one((e[0] + e[1]) / 2); 2]\n",
     "the_verdict_line_is_per_direction_and_inclusive", "assertion"),
    ("T4a floor rank", TESTS, "        .div_ceil(u128::from(den))\n", "        .checked_div(u128::from(den))\n        .unwrap()\n",
     "nearest_rank_and_loss_band_on_fixed_inputs", "not the 999th"),
    ("T4b interpolated quantile", TESTS, "    sorted[nearest_rank(sorted.len(), num, den) - 1]", INTERP,
     "nearest_rank_and_loss_band_on_fixed_inputs", "the 1,000th"),
    ("T4c band (0, n)", TESTS, "    (centre.saturating_sub(half) as u64, (centre + half) as u64)",
     "    let _ = (centre, half);\n    (0, n as u64)",
     "nearest_rank_and_loss_band_on_fixed_inputs", "assertion"),
    ("T4d sd rounded up", TESTS, "    let half = 5 * variance.isqrt();",
     "    let half = 5 * (variance.isqrt() + u128::from(variance.isqrt().pow(2) < variance));",
     "nearest_rank_and_loss_band_on_fixed_inputs", "assertion"),
    ("T4e sd in f64", TESTS, "    let half = 5 * variance.isqrt();",
     "    let _ = variance;\n    let half = (5.0 * (n as f64 * p as f64 / 1e6 * (1.0 - p as f64 / 1e6)).sqrt()) as u128;",
     "nearest_rank_and_loss_band_on_fixed_inputs", "assertion"),
    ("T5a L0 swapped", TESTS, "        at_p50: excess(late, p50),\n        at_zero: excess(late, 0),",
     "        at_p50: excess(late, 0),\n        at_zero: excess(late, p50),",
     "the_exact_excess_on_fixed_samples", "assertion"),
    ("T5b divided by the over count", TESTS, "    (total / late.len() as u128) as u64",
     "    (total / late.iter().filter(|&&l| l > c).count().max(1) as u128) as u64",
     "the_exact_excess_on_fixed_samples", "assertion"),
    ("T5c L0 = the mean", TESTS, "        at_p50: excess(late, p50),",
     "        at_p50: excess(late, (late.iter().map(|&l| u128::from(l)).sum::<u128>() / late.len().max(1) as u128) as u64),",
     "the_exact_excess_on_fixed_samples", "assertion"),
    ("T5d allowance not subtracted above the threshold", TESTS,
     "    let total: u128 = late.iter().map(|&l| u128::from(l.saturating_sub(c))).sum();",
     "    let total: u128 = late.iter().filter(|&&l| l > c).map(|&l| u128::from(l - l0)).sum();",
     "the_exact_excess_on_fixed_samples", "assertion"),
    ("T6a delay = due - t_in", TESTS, "let d = f.out_ns.checked_sub(f.t_in_ns);", "let d = f.due_ns.checked_sub(f.t_in_ns);",
     "the_cross_check_reads_delay_plus_lateness", "assertion"),
    ("T6b lost entries kept", TESTS, "        if f.kind == Kind::Delivered {\n            // Only after the filter",
     "        if f.kind == Kind::Delivered || f.kind == Kind::Lost {\n            // Only after the filter",
     "the_cross_check_reads_delay_plus_lateness", "released before it arrived"),
    ("T6c count over Delivered", TESTS, "        s.fates += 1;\n", "        s.fates += u64::from(f.kind == Kind::Delivered);\n",
     "the_cross_check_reads_delay_plus_lateness", "assertion"),
    ("T6d band rule off by a term (p90 at d + 0.9a)", TESTS, "        p90: at(d + 4 * a / 5),", "        p90: at(d + 9 * a / 10),",
     "the_cross_check_reads_delay_plus_lateness", "assertion"),
    ("T7a down's release stamp = its due", SOCKET, "results.push((kind, errno, sh.now_ns()));",
     "results.push((kind, errno, e.due_ns));",
     "a_held_lock_makes_both_directions_late", "down: due at"),
    ("T7b up's release stamp = its due", SOCKET, "st.im.finish(&e, Kind::Delivered, now, 0);",
     "st.im.finish(&e, Kind::Delivered, e.due_ns, 0);",
     "a_held_lock_makes_both_directions_late", "up: due at"),
    ("T7c summarize skips the log buckets", STATS, "        if b < first {\n            continue;\n        }",
     "        if b < first || b >= LINEAR {\n            continue;\n        }",
     "a_held_lock_makes_both_directions_late", "samples at least"),
]


def sha(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()


def touch():
    for p in (SPIKE / "crates").rglob("*.rs"):
        os.utime(p)


def run(test, exact=True):
    touch()
    env = dict(os.environ, CARGO_TARGET_DIR=TARGET)
    args = ["cargo", "+1.99.1", "test", "--locked", "-p", "npt-net", "--lib", "--", T + test]
    r = subprocess.run(args + (["--exact"] if exact else []), cwd=SPIKE, env=env, capture_output=True, text=True)
    return r.returncode, r.stdout + r.stderr


originals = {p: (p.read_bytes(), sha(p)) for p in (TESTS, SOCKET, STATS)}
for p, (_, h) in originals.items():
    print(f"original {p.name} sha256 {h}")
missed = 0
for name, path, old, new, test, says in BITES:
    if ONLY and name.split()[0] not in ONLY:
        continue
    text, h0 = originals[path]
    src = text.decode()
    assert src.count(old) == 1, f"{name}: anchor not found exactly once in {path.name}"
    path.write_text(src.replace(old, new, 1))
    try:
        code, out = run(test)
    finally:
        path.write_bytes(text)
    h1 = sha(path)
    ran = re.search(r"running (\d+) tests?", out)
    result = re.search(r"test result: \w+\. (\d+) passed; (\d+) failed", out)
    panic = re.search(r"panicked at ([^\n]+):\n((?:.*\n){1,4})", out)
    msg = " | ".join(l.strip() for l in panic.group(2).splitlines() if l.strip())[:600] if panic else "(no panic)"
    compiled = "error[" not in out
    one = ran is not None and ran.group(1) == "1"
    failed = result is not None and result.group(2) == "1" and f"{T}{test} ... FAILED" in out
    bites = compiled and one and failed and says in out
    if not bites:
        missed += 1
    print(f"{'BITES ' if bites else 'MISSED'} {name}: {T}{test}; ran {ran.group(1) if ran else '?'} test(s); "
          f"{'compiled' if compiled else 'DID NOT COMPILE'}; result {result.group(0) if result else '?'}; "
          f"says {says!r}: {says in out}")
    print(f"       at {panic.group(1) if panic else '-'}: {msg}")
    if not compiled:
        print("       " + "\n       ".join(l for l in out.splitlines() if l.startswith("error"))[:1500])
    print(f"       restored {path.name}: sha256 {h1} {'== original' if h1 == h0 else '!= ORIGINAL'}")
    assert h1 == h0, f"{path.name} not restored"
# The restored copy: every test the bites targeted passes again.
code, out = run("", exact=False)
print("restored copy: socket tests", re.search(r"test result: \w+\. \d+ passed; \d+ failed[^\n]*", out).group(0)
      if "test result" in out else out[-500:])
print(f"missed: {missed}")

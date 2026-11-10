# U4c test-engineer review: in-place plants in npt-l1, one at a time (u4-build-plan.md §5 U4c's bites, then bites the
# plan missed). Each plant: check the three files' sha256 against the build's, apply exact-match edits (each anchor
# must occur exactly the expected number of times), add a stderr marker in L1Net::new so the run shows the planted
# build ran, wait until no other copy's cargo/rustc runs, touch every file under crates/ and keep touching every 2 s
# during the run (plan §7.1), run npt-l1's tests, restore the original bytes, check the sha256 again, report.
# Backups of the three files go to the session scratchpad once (a fallback if a run is cut mid-plant).
# Run: python3 -I <this> <worktree> <plant> [<plant> ...]   (plant names: see PLANTS; "clean" = the marker only)
import hashlib
import os
import pathlib
import re
import subprocess
import sys
import threading
import time

WT = pathlib.Path(sys.argv[1]).resolve()
NAMES = sys.argv[2:]
SPIKE = WT / "spikes/netcode-proof"
CRATES = SPIKE / "crates"
LIB = CRATES / "l1/src/lib.rs"
UNIT = CRATES / "l1/src/tests.rs"
INTEG = CRATES / "l1/tests/l1.rs"
SHA = {
    LIB: "1647e75d8b5bea591c76c2b2fccd22b68f77d6892774d5ae2e24a32d99ad4711",
    UNIT: "c21a362f460a7fcda89e5890c0c78ebcaa427f83b3f4538c74c7b745805f18b6",
    INTEG: "639ab9c5e1e7398124f5b82d1283bb4192c111a4984636231c5851c05fb906a6",
}
LOGS = pathlib.Path("/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4c-review-test")
BACKUP = pathlib.Path("/tmp/claude-0/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/scratchpad")
ENV = {
    **os.environ,
    "CARGO_TARGET_DIR": "/home/user/Shamble-Frontlines/.gate/shared-target",
    "CARGO_NET_OFFLINE": "true",
}
MARK = "U4C-PLANT-MARKER"
MARKER = (LIB, "pub fn new(seed: u64) -> L1Net {", 'pub fn new(seed: u64) -> L1Net {\n        eprintln!("' + MARK + '");', 1)

# name: (features, [(file, old, new, count)], expectation in words)
PLANTS = {
    "clean": ("", [], "control: nothing planted but the marker; all pass"),
    "clean-bites": ("bites", [], "control, bites build: nothing planted but the marker; all pass"),
    # §5 U4c's eight bites, planted directly in the measured build's model (not through with_model_bite).
    "separate-spaces": ("", [
        (LIB, "let space = usize::from(bitten!(self.bite, SeparateSpaces));", "let space = 1usize;", 1),
        (LIB, "let space = usize::from(bitten!(l.model.bite, SeparateSpaces));", "let space = 1usize;", 1),
    ], "L-c1 fails (falls back to the PTO)"),
    "no-pto-doubling": ("", [
        (LIB, "base.saturating_mul(1u64.checked_shl(self.pto_count[d]).unwrap_or(u64::MAX))", "base", 1),
    ], "L-c3 fails"),
    "acks-in-bursts": ("", [
        (LIB, "let lost = l.profile.in_burst(t);", "let lost = false;", 1),
    ], "L-c3 fails"),
    "every-packet-sampled": ("", [
        (LIB, "let every_sampled = bitten!(l.model.bite, EveryPacketSampled);", "let every_sampled = true;", 1),
    ], "L-c7 fails"),
    "stream-sample-in-order": ("bites", [
        (UNIT, "lc7(L1Net::new(seed)).unwrap();",
         'lc7(L1Net::with_model_bite(seed, "npt-l1/stream-sample-in-order").unwrap()).unwrap();', 1),
    ], "L-c7 fails (the bite's own code, the L-c7 #[test] switched to it: its defect needs bites-only state)"),
    "one-draw-per-message": ("", [
        (LIB, "let shared = if bitten!(self.bite, OneDrawPerMessage) {", "let shared = if true {", 1),
    ], "L-c4 fails"),
    "one-draw-per-message-keep-plants": ("", [
        (LIB, "let shared = if bitten!(self.bite, OneDrawPerMessage) {", "let shared = if true {", 1),
        (LIB, "Some(x) => x,", "Some(x) => x || forced,", 1),
    ], "L-c4 fails on its statistical half (link 1), the planted loss kept"),
    "events-at-polls": ("", [
        (LIB, "if bitten!(self.bite, EventsAtPolls) {", "if true {", 2),
    ], "L-c5 fails"),
    "sends-before-events": ("", [
        (LIB, "if bitten!(self.bite, SendsBeforeEvents) {", "if true {", 2),
    ], "L-c6 fails"),
    # Bites the plan does not name.
    "next-event-none": ("", [
        (LIB, "self.net.borrow_mut().next_event()", "self.net.borrow_mut().next_event().and(None)", 1),
    ], "does L-c5 fail on a next_event_ns that always says None?"),
    "next-event-no-timers": ("", [
        (LIB, "let event = self.events.peek().map(|e| e.0.at_ns);",
         "let event = self.events.iter().filter(|e| !matches!(e.0.what, What::Timer { .. }) && !self.stale(&e.0))"
         ".map(|e| e.0.at_ns).min();", 1),
    ], "does anything fail on a next_event_ns blind to the loss and probe timers?"),
    "next-event-no-ready": ("", [
        (LIB, ".filter_map(|q| q.peek())", ".filter_map(|q| q.peek().filter(|_| false))", 1),
    ], "does anything fail on a next_event_ns blind to what waits in an inbox?"),
    "no-packet-threshold": ("", [
        (LIB, "const PACKET_THRESHOLD: u64 = 3;", "const PACKET_THRESHOLD: u64 = 1_000_000;", 1),
    ], "does anything fail without kPacketThreshold?"),
    "packet-threshold-2": ("", [
        (LIB, "const PACKET_THRESHOLD: u64 = 3;", "const PACKET_THRESHOLD: u64 = 2;", 1),
    ], "does anything fail with kPacketThreshold 2?"),
    "time-threshold-srtt-only": ("", [
        (LIB, "(self.smoothed_ns().max(self.latest) * 9 / 8)", "(self.smoothed_ns() * 9 / 8)", 1),
    ], "does anything fail with 9/8 x smoothed RTT, latest RTT ignored?"),
    "acks-lost-at-profile-rate": ("", [
        (LIB, "let lost = l.profile.in_burst(t);",
         "let lost = l.profile.in_burst(t) || below(w[0], 1_000_000) < u64::from(l.model.loss_ppm);", 1),
    ], "does anything fail when acknowledgements are lost outside bursts too?"),
    "acks-share-datagram-flow": ("", [
        (LIB, "take(&mut l.flows[back as usize][usize::from(ACK_CODE)])", "take(&mut l.flows[back as usize][0])", 1),
    ], "L-c8's unit half (the v1 pins) should fail: datagram fates move"),
    "event-order-link-first": ("", [
        (LIB, "fn rank(&self) -> (u64, u64, u8, u32, Dir, usize, u64) {",
         "fn rank(&self) -> (u64, u32, u64, u8, Dir, usize, u64) {", 1),
        (LIB, "self.at_ns, self.key, self.order, self.link, self.dir, self.space, self.tie,",
         "self.at_ns, self.link, self.key, self.order, self.dir, self.space, self.tie,", 1),
    ], "does L-c6 tell (time, fate index) from (time, link, fate index)?"),
    "pto-from-oldest-unacked": ("", [
        (LIB, "sp.last_sent.map(|(at, key)| (at.saturating_add(pto), key))",
         "sp.unacked.values().find(|s| s.retransmittable).map(|s| (s.sent_ns.saturating_add(pto), s.index))", 1),
    ], "does anything fail when the PTO counts from the oldest unacknowledged stream packet, not the last send?"),
    "rfc-a7-sampling": ("", [
        (LIB, "} else if a.eligible {", "} else if true {", 1),
    ], "L-c7 should fail: RFC 9002 A.7's rule samples whenever the largest is newly acknowledged"),
}


def sha(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()


def check_all(when):
    bad = [f"{p.name} {sha(p)[:16]}" for p in SHA if sha(p) != SHA[p]]
    if bad:
        sys.exit(f"ABORT {when}: files differ from the build: {bad}")


def touch_all():
    for root, _, files in os.walk(CRATES):
        for f in files:
            os.utime(os.path.join(root, f), None)


def procs():
    """(pid, ppid, comm, cmdline) of every cargo, rustc or rustdoc process."""
    out = []
    for d in os.listdir("/proc"):
        if not d.isdigit():
            continue
        try:
            comm = open(f"/proc/{d}/comm").read().strip()
            if comm not in ("cargo", "rustc", "rustdoc"):
                continue
            stat = open(f"/proc/{d}/stat").read()
            ppid = int(stat.rsplit(")", 1)[1].split()[1])
            cmd = open(f"/proc/{d}/cmdline").read().replace("\0", " ")[:160]
            out.append((int(d), ppid, comm, cmd))
        except OSError:
            continue
    return out


def ancestors(pid):
    seen = []
    while pid > 1:
        seen.append(pid)
        try:
            stat = open(f"/proc/{pid}/stat").read()
            pid = int(stat.rsplit(")", 1)[1].split()[1])
        except OSError:
            break
    return seen


def foreign(mine):
    return [p for p in procs() if mine is None or mine not in ancestors(p[0])]


def wait_idle(limit_s=900):
    t0 = time.time()
    while foreign(None):
        if time.time() - t0 > limit_s:
            return False
        time.sleep(5)
    return True


def apply(edits):
    texts = {}
    for f, old, new, count in edits:
        t = texts.get(f, f.read_text())
        n = t.count(old)
        if n != count:
            sys.exit(f"ABORT: {f.name}: {old!r} occurs {n} times, not {count}")
        texts[f] = t.replace(old, new)
    for f, t in texts.items():
        f.write_text(t)
    return list(texts)


def run(name):
    features, edits, expect = PLANTS[name]
    check_all(f"before {name}")
    originals = {p: p.read_bytes() for p in SHA}
    df = subprocess.run(["df", "-h", "/home/user"], capture_output=True, text=True).stdout.strip().splitlines()[-1]
    idle = wait_idle()
    cmd = ["cargo", "test", "--locked", "-p", "npt-l1", "--no-fail-fast"]
    if features:
        cmd += ["--features", features]
    cmd += ["--", "--nocapture"]
    seen = []
    stop = threading.Event()
    try:
        changed = apply([MARKER] + edits)
        touch_all()
        start = time.strftime("%H:%M:%SZ", time.gmtime())
        proc = subprocess.Popen(cmd, cwd=SPIKE, env=ENV, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)

        def toucher():
            while not stop.is_set():
                touch_all()
                for p in foreign(proc.pid):
                    seen.append(p)
                stop.wait(2)

        th = threading.Thread(target=toucher)
        th.start()
        out, _ = proc.communicate()
        stop.set()
        th.join()
        end = time.strftime("%H:%M:%SZ", time.gmtime())
    finally:
        stop.set()
        for p, b in originals.items():
            if p.read_bytes() != b:
                p.write_bytes(b)
        touch_all()
    check_all(f"after {name}")
    LOGS.mkdir(exist_ok=True)
    (LOGS / f"{name}.log").write_text(" ".join(cmd) + "\n" + out)
    lines = out.splitlines()
    compiled = [l.strip() for l in lines if l.strip().startswith("Compiling npt-l1")]
    marks = out.count(MARK)
    results = []
    binary = None
    for l in lines:
        s = l.strip()
        if s.startswith("Running ") or s.startswith("Doc-tests"):
            binary = s.split("(")[0].replace("Running ", "").strip()
        m = re.match(r"test result: (\w+)\. (\d+) passed; (\d+) failed; (\d+) ignored", s)
        if m:
            results.append(f"{binary}: {m.group(1)} {m.group(2)} passed {m.group(3)} failed {m.group(4)} ignored")
    failed = sorted({m.group(1) for m in re.finditer(r"^---- (\S+) stdout ----", out, re.M)})
    msgs = []
    for i, l in enumerate(lines):
        if "panicked at" in l and i + 1 < len(lines):
            msgs.append(lines[i + 1].strip()[:300])
    foreign_cmds = sorted({f"{p[2]} {p[3][:100]}" for p in seen})
    print(f"== {name} [{start}-{end}; {df}; idle at start {idle}; features {features or 'none'}]")
    print(f"   expect: {expect}")
    print(f"   planted in: {[p.name for p in changed]}; compiled: {compiled}; marker lines {marks}")
    for r in results:
        print(f"   {r}")
    print(f"   failed: {failed}")
    for m in msgs:
        print(f"   msg: {m}")
    print(f"   foreign cargo/rustc during the run: {foreign_cmds if foreign_cmds else 'none'}")
    print(f"   restored: lib.rs {sha(LIB)[:16]}, tests.rs {sha(UNIT)[:16]}, l1.rs {sha(INTEG)[:16]}")


if __name__ == "__main__":
    for n in NAMES:
        if n not in PLANTS:
            sys.exit(f"no plant {n}: {sorted(PLANTS)}")
    check_all("start")
    for p in SHA:
        b = BACKUP / f"u4c-backup-{p.parent.name}-{p.name}"
        if not b.exists():
            b.write_bytes(p.read_bytes())
        if hashlib.sha256(b.read_bytes()).hexdigest() != SHA[p]:
            sys.exit(f"ABORT: backup {b} differs")
    for n in NAMES:
        run(n)

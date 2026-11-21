# U4c test-engineer review: try to reproduce check.mjs run R3's one npt-net lib failure (name lost) under CPU
# contention. Builds the npt-net lib test binary in the U4c copy under plan §7.1 (wait for no cargo/rustc/rustdoc,
# touch every file under crates/ before and every 2 s during the build), takes its path from cargo's
# "Executable unittests" line and copies it at once to .gate/u4c-review-test/ (the shared target's file has one name
# for every copy, so another copy's build replaces it: seen at 15:30Z). The copy must name this worktree in its debug
# info and no other worktree (else ABORT: not this copy's build). Then ROUNDS rounds of K copies of that binary at
# once (no cargo involved), pinned with taskset to the CPU list CPUS (so the contention stays on those CPUs and other
# lanes' runs keep the rest), naming every failing test and the lines after each panic. Any cargo/rustc/rustdoc seen
# during a round is recorded. Full outputs: u4c-review-test/load-<label>-<round>-<copy>.log.
# Run: python3 -I -u <this> <worktree> <label> <rounds> <k> <cpus, e.g. 0 or 0,1>
import hashlib
import os
import pathlib
import re
import shutil
import subprocess
import sys
import threading
import time

WT = pathlib.Path(sys.argv[1]).resolve()
LABEL = sys.argv[2]
ROUNDS = int(sys.argv[3])
K = int(sys.argv[4])
CPUS = sys.argv[5]
SPIKE = WT / "spikes/netcode-proof"
CRATES = SPIKE / "crates"
LOGS = pathlib.Path("/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4c-review-test")
BIN_DIR = pathlib.Path("/home/user/Shamble-Frontlines/.gate/u4c-review-test")
ENV = {
    **os.environ,
    "CARGO_TARGET_DIR": "/home/user/Shamble-Frontlines/.gate/shared-target",
    "CARGO_NET_OFFLINE": "true",
}


def touch_all():
    for root, _, files in os.walk(CRATES):
        for f in files:
            os.utime(os.path.join(root, f), None)


def procs():
    out = []
    for d in os.listdir("/proc"):
        if not d.isdigit():
            continue
        try:
            comm = open(f"/proc/{d}/comm").read().strip()
            if comm in ("cargo", "rustc", "rustdoc"):
                out.append((int(d), comm, open(f"/proc/{d}/cmdline").read().replace("\0", " ")[:140]))
        except OSError:
            continue
    return out


def ancestors(pid):
    seen = []
    while pid > 1:
        seen.append(pid)
        try:
            pid = int(open(f"/proc/{pid}/stat").read().rsplit(")", 1)[1].split()[1])
        except OSError:
            break
    return seen


def wait_idle(quiet_s=5):
    t0 = time.time()
    quiet = None
    while True:
        if procs():
            quiet = None
        elif quiet is None:
            quiet = time.time()
        if quiet is not None and time.time() - quiet >= quiet_s:
            return time.time() - t0
        if time.time() - t0 > 1800:
            sys.exit("ABORT: other cargo kept running for 30 minutes")
        time.sleep(1)


def sha(p):
    return hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()


LOGS.mkdir(exist_ok=True)
BIN_DIR.mkdir(parents=True, exist_ok=True)
waited = wait_idle()
touch_all()
build = subprocess.Popen(["cargo", "test", "--locked", "-p", "npt-net", "--lib", "--no-run"], cwd=SPIKE, env=ENV,
                         stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
stop = threading.Event()
foreign = set()


def toucher(owner):
    while not stop.is_set():
        touch_all()
        for p in procs():
            if owner is None or owner.pid not in ancestors(p[0]):
                foreign.add(f"{p[1]} {p[2]}")
        stop.wait(2)


th = threading.Thread(target=toucher, args=(build,))
th.start()
bout, _ = build.communicate()
stop.set()
th.join()
m = re.search(r"Executable unittests src/lib\.rs \((\S+)\)", bout)
compiled = [l.strip().split(" (")[0] for l in bout.splitlines() if l.strip().startswith("Compiling")]
print(f"build [waited {waited:.0f} s] exit {build.returncode}; compiled: {compiled}; foreign: {sorted(foreign) or 'none'}")
if not m or build.returncode != 0:
    print(bout[-3000:])
    sys.exit("ABORT: no test binary")
binary = BIN_DIR / "npt_net-lib-test"
shutil.copyfile(m.group(1), binary)
os.chmod(binary, 0o755)
data = binary.read_bytes()
trees = sorted(set(re.findall(rb"/home/user/Shamble-Frontlines/\.claude/worktrees/[A-Za-z0-9_-]+", data)))
mine = str(WT).encode()
if trees != [mine]:
    sys.exit(f"ABORT: the binary names {trees}, not only this worktree: another copy's build")
h0 = sha(binary)
print(f"binary copied from {m.group(1)} to {binary}; sha256 {h0}; worktrees named in it: {[t.decode() for t in trees]}")
for r in range(1, ROUNDS + 1):
    foreign = set()
    stop = threading.Event()
    th = threading.Thread(target=toucher, args=(None,))
    load = open("/proc/loadavg").read().split()[:3]
    t0 = time.time()
    start = time.strftime("%H:%M:%SZ", time.gmtime())
    ps = [subprocess.Popen(["taskset", "-c", CPUS, str(binary)], cwd=CRATES / "net", stdout=subprocess.PIPE,
                           stderr=subprocess.STDOUT, text=True) for _ in range(K)]
    th.start()
    outs = [p.communicate()[0] for p in ps]
    stop.set()
    th.join()
    secs = time.time() - t0
    print(f"== {LABEL} round {r}/{ROUNDS} x{K} on CPUs {CPUS} [{start}, {secs:.1f} s, load before {load}]; binary "
          f"{'same' if sha(binary) == h0 else 'CHANGED'}; foreign cargo/rustc: {len(foreign)} kinds")
    for i, (p, out) in enumerate(zip(ps, outs), 1):
        (LOGS / f"load-{LABEL}-{r}-{i}.log").write_text(out)
        res = [l.strip() for l in out.splitlines() if l.strip().startswith("test result:")]
        failed = [mm.group(1) for mm in re.finditer(r"^---- (\S+) stdout ----", out, re.M)]
        print(f"   copy {i}: exit {p.returncode} {res[0][:110] if res else 'NO RESULT'}; failed: {failed}")
        lines = out.splitlines()
        for j, l in enumerate(lines):
            if "panicked at" in l:
                print(f"     {l.strip()[:220]}")
                for k in lines[j + 1:j + 4]:
                    print(f"       {k.strip()[:260]}")

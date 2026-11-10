# U4c test-engineer review: repeat one cargo test command N times in the U4c copy under plan §7.1 (wait for 20 s with
# no cargo/rustc, touch every file under crates/ before and every 2 s during each run, record any other copy's
# cargo/rustc seen during it), to root-cause a test that failed once. Prints each run's result lines, failing tests
# and the lines after each panic; full output to u4c-review-test/repeat-<label>-<i>.log.
# Run: python3 -I <this> <worktree> <label> <n> <cargo args...>
import os
import pathlib
import re
import subprocess
import sys
import threading
import time

WT = pathlib.Path(sys.argv[1]).resolve()
LABEL = sys.argv[2]
N = int(sys.argv[3])
ARGS = sys.argv[4:]
SPIKE = WT / "spikes/netcode-proof"
CRATES = SPIKE / "crates"
LOGS = pathlib.Path("/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4c-review-test")
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


def wait_idle():
    t0 = time.time()
    quiet = None
    while True:
        if procs():
            quiet = None
        elif quiet is None:
            quiet = time.time()
        if quiet is not None and time.time() - quiet >= 20:
            return time.time() - t0
        if time.time() - t0 > 540:
            sys.exit("ABORT: other cargo kept running for 9 minutes")
        time.sleep(2)


LOGS.mkdir(exist_ok=True)
for i in range(1, N + 1):
    waited = wait_idle()
    touch_all()
    start = time.strftime("%H:%M:%SZ", time.gmtime())
    cmd = ["cargo", "test", "--locked"] + ARGS
    proc = subprocess.Popen(cmd, cwd=SPIKE, env=ENV, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
    seen = set()
    stop = threading.Event()

    def toucher():
        while not stop.is_set():
            touch_all()
            for p in procs():
                if proc.pid not in ancestors(p[0]):
                    seen.add(f"{p[1]} {p[2]}")
            stop.wait(2)

    th = threading.Thread(target=toucher)
    th.start()
    out, _ = proc.communicate()
    stop.set()
    th.join()
    end = time.strftime("%H:%M:%SZ", time.gmtime())
    (LOGS / f"repeat-{LABEL}-{i}.log").write_text(" ".join(cmd) + "\n" + out)
    lines = out.splitlines()
    res = [l.strip() for l in lines if l.strip().startswith("test result:")]
    failed = [m.group(1) for m in re.finditer(r"^---- (\S+) stdout ----", out, re.M)]
    print(f"== {LABEL} {i}/{N} [{start}-{end}; waited {waited:.0f} s] exit {proc.returncode}")
    for r in res:
        print(f"   {r[:200]}")
    print(f"   failed: {failed}")
    for j, l in enumerate(lines):
        if "panicked at" in l:
            print(f"   {l.strip()[:200]}")
            for k in lines[j + 1:j + 4]:
                print(f"     {k.strip()[:240]}")
    print(f"   foreign cargo/rustc during the run: {sorted(seen) if seen else 'none'}")

# U4c test-engineer review: one counted check.mjs run in the U4c copy under plan §7.1: wait until no cargo, rustc or
# rustdoc process runs, touch every file under crates/, keep touching every 2 s during the run, and record any other
# copy's cargo/rustc seen during it (a run with any is not reported as counted).
# Run: python3 -I <this> <worktree>
import os
import pathlib
import subprocess
import sys
import threading
import time

WT = pathlib.Path(sys.argv[1]).resolve()
CRATES = WT / "spikes/netcode-proof/crates"
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


t0 = time.time()
quiet_since = None
while True:
    if procs():
        quiet_since = None
    elif quiet_since is None:
        quiet_since = time.time()
    if quiet_since is not None and time.time() - quiet_since >= 20:
        break
    if time.time() - t0 > 540:
        sys.exit("ABORT: other cargo kept running for 9 minutes")
    time.sleep(2)
waited = time.time() - t0
df = subprocess.run(["df", "-h", "/home/user"], capture_output=True, text=True).stdout.strip().splitlines()[-1]
touch_all()
start = time.strftime("%H:%M:%SZ", time.gmtime())
proc = subprocess.Popen(["node", "spikes/netcode-proof/check.mjs"], cwd=WT, env=ENV, stdout=subprocess.PIPE,
                        stderr=subprocess.STDOUT, text=True)
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
print(f"check.mjs in {os.getcwd() if False else WT} [{start}-{end}; waited {waited:.0f} s for 20 s idle; {df}]")
for line in out.splitlines():
    print(line[:260])
print(f"exit {proc.returncode}; foreign cargo/rustc during the run: {sorted(seen) if seen else 'none'}")

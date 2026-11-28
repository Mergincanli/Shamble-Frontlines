# U4d build proof (test-engineer): the probe file again (u4d-test-bites.py's PROBE_SRC, read from that script so the
# source is the same), because run 1's filter kept only lines that start with "PROBE-" and the test harness prints each
# test's first line after its "test <name> ... " prefix. Same mechanics: created in place in U4d's copy, run once
# (shared target, offline, touched before and every 2 s during the run, df, this copy's npt-sim compiled), deleted, and
# every crates/sim file checked against its reference sha256 before and after.
# Run: python3 -I <this> <spike dir>
import hashlib
import os
import pathlib
import re
import shutil
import subprocess
import sys
import threading
import time

SPIKE = pathlib.Path(sys.argv[1]).resolve()
TARGET = "/home/user/Shamble-Frontlines/.gate/shared-target"
SIM = SPIKE / "crates/sim"
PROBE = SIM / "tests/te_probe.rs"
HERE = pathlib.Path(__file__).resolve().parent
text = (HERE / "u4d-test-bites.py").read_text()
start = text.index("PROBE_SRC = r'''") + len("PROBE_SRC = r'''")
PROBE_SRC = text[start:text.index("'''", start)]
REF = {}
for line in (HERE / "u4d-review-test.md").read_text().splitlines():
    m = re.match(r"- ((?:src|tests)/\w+\.rs) ([0-9a-f]{64})$", line)
    if m:
        REF[m.group(1)] = m.group(2)
assert len(REF) == 15, f"{len(REF)} reference sha256 read"


def sha(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()


def check_all(when):
    bad = [f for f, h in REF.items() if sha(SIM / f) != h]
    print(f"{when}: every crates/sim file at its reference sha256: {not bad and not PROBE.exists()} {bad if bad else ''}")
    assert not bad and not PROBE.exists()


def touch():
    for p in (SPIKE / "crates").rglob("*"):
        if p.is_file():
            os.utime(p)


def busy():
    for args in (["pgrep", "-x", "cargo"], ["pgrep", "-x", "rustc"], ["pgrep", "-x", "rustdoc"],
                 ["pgrep", "-f", "clippy-driver"], ["pgrep", "-f", "cargo-clippy"]):
        if subprocess.run(args, stdout=subprocess.DEVNULL).returncode == 0:
            return True
    return False


def toucher(stop):
    while not stop.wait(2.0):
        touch()


check_all("start")
gb = shutil.disk_usage("/home/user").free / 1e9
print(f"df /home/user: {gb:.1f} GB free")
if gb < 2:
    sys.exit("under 2 GB free: stop")
idle, waited = 0, 0
while idle < 4 and waited < 600:
    idle = 0 if busy() else idle + 1
    time.sleep(1)
    waited += 1
print(f"waited {waited} s for idle ({'idle' if idle >= 4 else 'NOT idle'})")
env = dict(os.environ, CARGO_TARGET_DIR=TARGET, CARGO_NET_OFFLINE="true")
args = ["cargo", "test", "--locked", "-p", "npt-sim", "--test", "te_probe", "--", "--nocapture", "--test-threads", "1"]
try:
    PROBE.write_text(PROBE_SRC)
    touch()
    stop = threading.Event()
    th = threading.Thread(target=toucher, args=(stop,))
    th.start()
    t0 = time.time()
    try:
        r = subprocess.run(args, cwd=SPIKE, env=env, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
    finally:
        stop.set()
        th.join()
finally:
    PROBE.unlink()
out = r.stdout
print(f"ran {time.time() - t0:.0f} s; exit {r.returncode}; this copy's npt-sim compiled: "
      f"{f'Compiling npt-sim v0.0.0 ({SIM})' in out}; {re.findall(r'test result: [^\n]+', out)}")
for line in out.splitlines():
    if "PROBE-" in line:
        print(line[line.index("PROBE-"):][:1500])
for line in out.splitlines():
    if line.startswith("test probe_"):
        print(line[:120])
check_all("end")

# U4d build proof (test-engineer), second pass: runs u4d-test-bites.py did not hold. E9b plants a cooldown refusal that
# writes a Kill event into the hashed event list (run 1's E9 wrote a Shot event, which the test's own shots_of check
# caught before the twins' event-list check ran); E13 plants a NaN into the aim's yaw before the own-copy snap (phase
# 2), where the snap turns it into 0.0, to see whether D-4's per-tick finite assert (which reads the hashed state only)
# or carry 9's record test sees it; C1 runs the clean D-6 with its print, for the builder's numbers.
# Same rules and mechanics as u4d-test-bites.py (in place, alone, restore from memory, sha256 against the reference,
# shared target, offline, touch before and every 2 s during the run, df, this copy's npt-sim compiled).
# Run: python3 -I <this> <spike dir> [id ...]
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
ONLY = set(sys.argv[2:])
TARGET = "/home/user/Shamble-Frontlines/.gate/shared-target"
SIM = SPIKE / "crates/sim"
WORLD = SIM / "src/world.rs"
REF = {
    "src/greybox.rs": "2d75cb0b6949680c98463817046b15610c48454968d845e1ce65c414504f35e5",
    "src/grid.rs": "a0e24c08224e5dd412ad78e8caca870d92c6f2e03e36765bea5261ffcc999484",
    "src/hash.rs": "4663b2a8d7c771d05ac33d9af8a9c1bcb951f7d35ce34440717d8699cce45f06",
    "src/history.rs": "0c9012ace74fa3bf449d7aab6de363ab76071e0e4457ac26d2d901f29fee9cc7",
    "src/lib.rs": "70eb8262e18f6461c81db8a7a9eb17710fbe209ab8e800ea18ede9448f01ba6f",
    "src/replay.rs": "8c566b8ae73dd9b946de0b3bad73ce12c7348f25d8fe5bba1655efbe25e2c3ff",
    "src/rng.rs": "d6b61bbe47569943af598ce34b549cfc7297cefc27652cdd91414557bfc2cb7d",
    "src/scene.rs": "4d8a04b5769cbaff76dac41c60be0364a78d1a797167886ea78a73e8362b2d56",
    "src/seam.rs": "a176cd0d8868e070a2591483efe1d3de661adba173d1e4bd32bf6e2f731a410a",
    "src/ticks.rs": "5d852f1a81f72e958a89f5489bce5d556da92d096e86aebc4bd130e447dece5b",
    "src/trajectory.rs": "2c0968b3ce8fef64991c2d2962193fdb821b8d117b52c7761c31dc5cd2c602ef",
    "src/world.rs": "57b0cf9c212c8033b810dd724d4b61c017fdc9d06fa9801171326a6b605b476d",
    "tests/replay.rs": "a547314c1ea9593ee62bb079fb8d4f351b06ceb347e715224a30df0322b6b636",
    "tests/seam.rs": "d91b8c0c9fe872a0322126ecc6d6a72ba218bb63e3c24995abae177955da163b",
    "tests/world.rs": "41ffc20ff978ad9a1fbdfc2c6f9c427f3e45bd9a0e4629b227ae6158431409ca",
}
YAW_OLD = "        let yaw = libm::atan2f(d[1], d[0]);\n"
YAW_NEW = ("        let yaw = if authority && self.elapsed == 150 && c.slot == 0 {\n            0.0 / 0.0\n"
           "        } else {\n            libm::atan2f(d[1], d[0])\n        };\n")
CD_OLD = "                self.reports.cooldown += 1;\n"
CD_NEW = ("                self.reports.cooldown += 1;\n"
          "                self.events.push(SimEvent::Kill { killer: c.slot, victim: c.slot });\n")
D4 = "d4_the_state_holds_no_non_finite_value_on_any_tick"
D6 = "d6_the_stress_scene_meets_pm_0001_2_on_every_tick"
C9 = "carry9_each_record_holds_its_pre_snap_value_and_the_wire_grids_integer_of_it"
CD = "d10_a_fire_press_in_the_cooldown_leaves_no_trace"
# (id, what, (test file, test name), edits, must it fail?, what the failure must say)
BITES = [
    ("E9b", "D-10: a cooldown refusal writes a Kill event into the hashed event list", ("world", CD),
     [(WORLD, CD_OLD, CD_NEW)], True, "step 11: the event lists"),
    ("E13", "D-4: slot 0's aim yaw = 0.0 / 0.0 before the own-copy snap at elapsed 150 (phase 2)", ("world", D4),
     [(WORLD, YAW_OLD, YAW_NEW)], None, ""),
    ("E13-c9", "the same NaN: does carry 9's record test see the raw NaN?", ("world", C9),
     [(WORLD, YAW_OLD, YAW_NEW)], None, ""),
    ("C1", "the clean D-6, its print (the builder's 683.62 shots/s, 1.00 portal changes/s)", ("world", D6), [], False, ""),
]


def sha(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()


def check_all(when):
    bad = [f for f, h in REF.items() if sha(SIM / f) != h]
    extra = (SIM / "tests/te_probe.rs").exists()
    print(f"{when}: every crates/sim file at its reference sha256: {not bad and not extra} {bad if bad else ''}")
    assert not bad and not extra


def df_ok():
    gb = shutil.disk_usage("/home/user").free / 1e9
    print(f"       df /home/user: {gb:.1f} GB free")
    if gb < 2:
        sys.exit("under 2 GB free: stop")


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


def wait_idle(need=4, limit=600):
    idle, waited = 0, 0
    while idle < need and waited < limit:
        idle = 0 if busy() else idle + 1
        time.sleep(1)
        waited += 1
    return waited, idle >= need


def toucher(stop):
    while not stop.wait(2.0):
        touch()


def run(target):
    df_ok()
    waited, idle = wait_idle()
    env = dict(os.environ, CARGO_TARGET_DIR=TARGET, CARGO_NET_OFFLINE="true")
    args = ["cargo", "test", "--locked", "-p", "npt-sim", "--test", target[0], "--", target[1], "--exact", "--nocapture"]
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
    print(f"       waited {waited} s for idle ({'idle' if idle else 'NOT idle: ran beside another cargo'}); "
          f"ran {time.time() - t0:.0f} s; another cargo running at the end: {busy()}")
    return r.returncode, r.stdout


check_all("start")
original = WORLD.read_bytes()
results = []
for bid, what, target, edits, must_fail, says in BITES:
    if ONLY and bid not in ONLY:
        continue
    src = original.decode()
    for path, old, new in edits:
        assert src.count(old) == 1, f"{bid}: anchor found {src.count(old)} times"
        src = src.replace(old, new, 1)
    print(f"{bid}: {what}")
    try:
        if edits:
            WORLD.write_text(src)
        code, out = run(target)
    finally:
        WORLD.write_bytes(original)
    restored = sha(WORLD) == REF["src/world.rs"]
    mine = f"Compiling npt-sim v0.0.0 ({SIM})" in out
    compiled = "error[" not in out and "could not compile" not in out
    ran = re.findall(r"running (\d+) tests?", out)
    result = re.findall(r"test result: \w+\. (\d+) passed; (\d+) failed", out)
    name = target[1]
    failed = f"test {name} ... FAILED" in out
    passed = f"test {name} ... ok" in out
    panic = re.search(r"panicked at ([^\n]+):\n((?:.*\n){1,6})", out)
    msg = " | ".join(l.strip() for l in panic.group(2).splitlines() if l.strip())[:700] if panic else "(no panic)"
    if must_fail is True:
        verdict = "BITES" if (compiled and mine and failed and says in out) else "MISSED"
    elif must_fail is False:
        verdict = "PASSES" if (compiled and mine and passed) else "FAILED"
    else:
        verdict = "FAILED (informative)" if failed else ("PASSED (not caught)" if passed else "UNCLEAR")
    print(f"{verdict} {bid}: {name}; this copy's npt-sim compiled: {mine}; {'compiled' if compiled else 'DID NOT COMPILE'}; "
          f"running {ran}; results {result}")
    print(f"       at {panic.group(1) if panic else '-'}: {msg}")
    for l in out.splitlines():
        if l.startswith("D-6:"):
            print("       " + l)
    if not compiled:
        print("       " + "\n       ".join(l for l in out.splitlines() if l.startswith("error"))[:1500])
    print(f"       restored: {restored}")
    results.append((bid, verdict))
    assert restored, f"{bid}: not restored"
    sys.stdout.flush()
check_all("end")
print("summary: " + "; ".join(f"{b} {v}" for b, v in results))

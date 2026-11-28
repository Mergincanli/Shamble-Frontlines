# U4c review round 2 (test-engineer): one more plant through r2-plant.py's machinery (loaded by path; its own checks,
# hashes, quiet wait, touch loop, restore and sha256 apply). N1: the acks-in-bursts defect planted directly in the
# model, run on the BITES build, so the bite tests run on top of it: bite_no_pto_doubling_fails_lc3 must now FAIL
# (its L-c3 run trips "no probe timeout fired", another check than its own; the new fails() must reject that), while
# bite_acks_in_bursts_fails_lc3 still passes (that is its own check).
# Run: python3 -I -u <this> <worktree> n1-acks-never-lost-bites
import importlib.util
import pathlib
import sys

HERE = pathlib.Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("r2_plant", HERE / "r2-plant.py")
r2 = importlib.util.module_from_spec(spec)
spec.loader.exec_module(r2)  # its __main__ block does not run
m = r2.m
m.PLANTS["n1-acks-never-lost-bites"] = ("bites", [
    (m.LIB, "let lost = l.profile.in_burst(t);", "let lost = false;", 1),
], "bites build: bite_no_pto_doubling_fails_lc3 FAILS on fails()'s check; bite_acks_in_bursts_fails_lc3 passes")
names = sys.argv[2:]
for n in names:
    if n not in m.PLANTS:
        sys.exit(f"no plant {n}")
m.check_all("start")
for n in names:
    m.run(n)

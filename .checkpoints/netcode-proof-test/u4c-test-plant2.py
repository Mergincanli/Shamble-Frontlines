# U4c test-engineer review, resumed run: more in-place plants (the RFC 9002 constants and the estimator's update
# order), through u4c-test-plant.py's own machinery (sha256 check before and after, the marker, wait for no other
# cargo/rustc, touch before and every 2 s, restore the bytes). Loads that script by its path and adds to its PLANTS.
# Run: python3 -I -u <this> <worktree> <plant> [<plant> ...]
import importlib.util
import pathlib
import sys

HERE = pathlib.Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("u4c_plant", HERE / "u4c-test-plant.py")
m = importlib.util.module_from_spec(spec)
spec.loader.exec_module(m)  # its __main__ block does not run: __name__ is "u4c_plant"
LIB = m.LIB
m.PLANTS.update({
    "max-ack-delay-0": ("", [
        (LIB, "const MAX_ACK_DELAY_NS: u64 = 25 * MS;", "const MAX_ACK_DELAY_NS: u64 = 0;", 1),
    ], "L-c2 (mean about 127.6 ms) and L-c9 (1,024 ms, 26 ms) should fail"),
    "initial-rtt-300": ("", [
        (LIB, "const INITIAL_RTT_NS: u64 = 333 * MS;", "const INITIAL_RTT_NS: u64 = 300 * MS;", 1),
    ], "L-c9 should fail (first PTO 925 ms, not 1,024 ms)"),
    "granularity-0": ("", [
        (LIB, "const GRANULARITY_NS: u64 = MS;", "const GRANULARITY_NS: u64 = 0;", 1),
    ], "L-c9 should fail (kGranularity floors both timers)"),
    "max-packet-1100": ("", [
        (LIB, "const MAX_PACKET: usize = 1_200;", "const MAX_PACKET: usize = 1_100;", 1),
    ], "does anything pin 1,200 B exactly (3,000 B is 3 pieces at 1,000 to 1,499 B)?"),
    "max-packet-1500": ("", [
        (LIB, "const MAX_PACKET: usize = 1_200;", "const MAX_PACKET: usize = 1_500;", 1),
    ], "L-c4 should fail (3,000 B in 2 pieces)"),
    "var-from-new-smoothed": ("", [
        (LIB, "self.var = (3 * self.var + sm.abs_diff(s)) / 4;",
         "self.var = (3 * self.var + ((7 * sm + s) / 8).abs_diff(s)) / 4;", 1),
    ], "the estimator test should fail (RFC 9002 §5.3 updates rttvar from the old smoothed RTT)"),
})
WT = sys.argv[1]
NAMES = sys.argv[2:]
for n in NAMES:
    if n not in m.PLANTS:
        sys.exit(f"no plant {n}")
m.check_all("start")
for n in NAMES:
    m.run(n)

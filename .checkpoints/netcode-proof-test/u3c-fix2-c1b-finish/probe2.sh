#!/bin/bash
# Probe 2's timings on this tree: 5 runs, each a fresh process (debug). Usage: probe2.sh <label> <out file>
# The probe test (probe2.rs) must be appended to delay_tests.rs first, and removed after.
set -u
cd /home/user/Shamble-Frontlines/.claude/worktrees/wf_10e244da-89a-1/spikes/netcode-proof || exit 1
export CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/u3v3-harness-check-target
cargo +1.99.1 test --locked -p npt-net --lib --no-run >/dev/null 2>&1 || { echo "build failed"; exit 1; }
for i in 1 2 3 4 5; do
  cargo +1.99.1 test --locked -p npt-net --lib -- --exact l2::delay::tests::zz_probe2_second_record_goes_out_soon_after_its_release --nocapture 2>&1 \
    | grep -E "PROBE2|test result|panicked" | sed "s/^/$1 run $i: /"
done >> "$2"

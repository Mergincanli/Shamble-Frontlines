#!/bin/bash
# Probe 2's timings: 5 runs on the given tree (debug), each a fresh process. Usage: probe2.sh <label> <out file>
set -u
cd /home/user/Shamble-Frontlines/.claude/worktrees/wf_ac5e4575-9da-1/spikes/netcode-proof || exit 1
export CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/u3v3-harness-check-target
cargo +1.99.1 test --locked -p npt-net --lib --no-run >/dev/null 2>&1 || { echo "build failed"; exit 1; }
for i in 1 2 3 4 5; do
  cargo +1.99.1 test --locked -p npt-net --lib -- --exact l2::delay::tests::zz_probe2_second_record_goes_out_soon_after_its_release --nocapture 2>&1 \
    | grep -E "PROBE2|test result" | sed "s/^/$1 run $i: /"
done >> "$2"

#!/usr/bin/env bash
# TE2-5 (b) (advisor-u4j-r2.md B2.3): the bites build's workspace tests, as check.mjs's test step runs them
# (check.mjs:1496: `cargo test --locked --workspace --all-features`, from spikes/netcode-proof) but with --no-fail-fast,
# ten times, each through main-run.sh (the lane lock held exclusively, an idle wait, 4 GB free, the 2 s touch loop, the
# shared target, offline), each run's whole output in its own log under .gate/te2-5/. A run's result is its exit code
# and the test runner's own summary lines ("test result: ..."), read from its log afterwards.
# Usage: bash te2-5-runs.sh
set -u
M=/home/user/Shamble-Frontlines
mkdir -p "$M/.gate/te2-5"
for i in 1 2 3 4 5 6 7 8 9 10; do
  bash "$M/.checkpoints/netcode-proof-test/main-run.sh" "$M/.gate/te2-5/run$i.log" \
    cargo test --locked --workspace --all-features --no-fail-fast
  echo "run $i: exit $? at $(date -u +%T) UTC"
done

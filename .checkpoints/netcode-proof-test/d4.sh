#!/usr/bin/env bash
# D4 (advisor-u4r-r2-full.md step 14): r5's command (u4r-build.md:1448-1458; .gate/u4r/c5-r5.log) run in the main
# folder's own cache, with CARGO_TARGET_DIR unset. First npt-lab's release build (with -v, so the Compiling/Fresh npt-*
# lines reach the log). Then the same 15 timed 60-s runs: SS P0, P2, P3 and B1, and ES P3, seeds 1 to 3, with r5's
# marks, settings, run ids and set name. Only three things change: the binary's path (this cache), --commit (HEAD;
# spikes/ equal to f1f131b) and --out (.gate/u4r-d4/r5/). One exclusive cache-run.sh call; its log is
# .gate/u4r-d4/d4.log.
# Usage: bash d4.sh
set -u
M=/home/user/Shamble-Frontlines
N=$M/.checkpoints/netcode-proof-test
BIN=$M/.gate/netcode-proof-test/target/release/npt-lab
OUT=$M/.gate/u4r-d4/r5
C=$(cd "$M" && git rev-parse HEAD)
if [ -e "$M/.gate/u4r-d4" ]; then
  echo "STOP: $M/.gate/u4r-d4 exists already; D4's output is never overwritten"
  exit 2
fi
args=(cargo build --release --locked -v -p npt-lab)
for p in P0 P2 P3 B1; do
  marks=PM-0001-2,PM-0001-17,PM-0001-57
  if [ "$p" = B1 ]; then marks=$marks,PM-0001-18; fi
  for s in 1 2 3; do
    args+=(:: bash -c "time $BIN scenario --run r5-ss-$p-s$s --set u4r-r5 --date 2026-11-14 --commit $C --out $OUT/ss-$p-s$s --scene SS --profile $p --seed $s --tick-start 63736 --warmup-s 10 --measured-s 50 --marks $marks --suite r5")
  done
done
for s in 1 2 3; do
  args+=(:: bash -c "time $BIN scenario --run r5-es-P3-s$s --set u4r-r5 --date 2026-11-14 --commit $C --out $OUT/es-P3-s$s --scene ES --profile P3 --seed $s --warmup-s 10 --measured-s 50 --marks PM-0001-2 --suite r5")
done
exec bash "$N/cache-run.sh" "$M/.gate/u4r-d4/d4.log" "${args[@]}"

#!/usr/bin/env bash
# TE2-5 (c), TE-M3's first step (advisor-u4d2-r1.md, Block 5: "Its first step is the panic: ... a loop of the bites
# build's workspace tests under load ..., now watching this test by name"). A diagnosis only: no source changes, no
# retry into green. Builds npt-net's library tests once in the bites build (`--all-features`, as check.mjs's bites test
# step), then runs that one test binary whole (every npt-net test, libtest's default threads, the neighbours that load
# it in check.mjs) N times in each of P parallel loops (P > 1 adds load), from crates/net. Each run's whole output goes
# to .gate/te-m3/run-<loop>-<i>.log; the first run that exits non-zero stops every loop, and its log is named, with
# libtest's own summary line and failures list. Run it through main-run.sh (the lane lock held exclusively):
#   bash .checkpoints/netcode-proof-test/main-run.sh /home/user/Shamble-Frontlines/.gate/te-m3/loop.log \
#     bash /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/te_m3_loop.sh <N> <P>
set -u
OUT=/home/user/Shamble-Frontlines/.gate/te-m3
N="${1:-20}"
P="${2:-1}"
mkdir -p "$OUT"
build="$OUT/build.log"
cargo test --locked -p npt-net --lib --all-features --no-run > "$build" 2>&1
rc=$?
echo "build exit $rc ($build)"
[ $rc -eq 0 ] || exit $rc
BIN=$(sed -n 's/^ *Executable unittests src\/lib\.rs (\(.*\))$/\1/p' "$build" | tail -1)
case "$BIN" in
  /*) ;;
  *) BIN="$(pwd)/$BIN" ;;
esac
[ -x "$BIN" ] || { echo "STOP: no test binary found in $build (got: $BIN)"; exit 2; }
echo "binary $BIN; $N runs in each of $P loops; start $(date -u +%T) UTC"
stop="$OUT/stop"
rm -f "$stop"
loop() {
  local p=$1 i
  for i in $(seq 1 "$N"); do
    [ -e "$stop" ] && return 0
    local f="$OUT/run-$p-$i.log"
    (cd crates/net && "$BIN") > "$f" 2>&1
    local r=$?
    echo "$(date -u +%T) loop $p run $i exit $r: $(grep -E '^test result:' "$f" | tail -1)"
    if [ $r -ne 0 ]; then
      echo "$p-$i" > "$stop"
      echo "FAILED: loop $p run $i, exit $r, log $f; libtest's failures list:"
      sed -n '/^failures:$/,/^test result:/p' "$f" | grep -E '^    ' | head -20
      return $r
    fi
  done
}
pids=()
for p in $(seq 1 "$P"); do
  loop "$p" &
  pids+=($!)
done
status=0
for pid in "${pids[@]}"; do
  wait "$pid"
  r=$?
  [ $status -eq 0 ] && [ $r -ne 0 ] && status=$r
done
echo "end $(date -u +%T) UTC; first failure: $(cat "$stop" 2>/dev/null || echo none)"
exit $status

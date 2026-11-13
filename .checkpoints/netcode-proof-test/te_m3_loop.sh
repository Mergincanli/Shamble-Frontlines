#!/usr/bin/env bash
# TE2-5 (c), TE-M3's first step (advisor-u4d2-r1.md, Block 5: "Its first step is the panic: ... a loop of the bites
# build's workspace tests under load ..., now watching this test by name"). A diagnosis only: no source changes, no
# retry into green. Builds npt-net's library tests once in the bites build (`--all-features`, as check.mjs's bites test
# step), then runs that test binary N times in each of P parallel loops, from crates/net, while H busy loops load the
# CPUs (H = 0: none). MODE `all` runs the whole binary (every npt-net test, libtest's default threads: the neighbours
# that load it in check.mjs); MODE `one` runs TE-M3's test alone with --nocapture, so each run's own line (the pump's
# runs during the wait, those at or after the deadline, S's shutdown calls, the end's lateness) is kept. Each run's
# whole output goes to .gate/te-m3/<tag>-<loop>-<i>.log with the CPUs' steal time over the run (/proc/stat, jiffies);
# the first run that exits non-zero stops every loop and is named, with libtest's own summary line and failures list.
# Run it through main-run.sh (the lane lock held exclusively):
#   bash .checkpoints/netcode-proof-test/main-run.sh /home/user/Shamble-Frontlines/.gate/te-m3/<tag>.log \
#     bash /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/te_m3_loop.sh <tag> <all|one> <N> <P> <H>
set -u
OUT=/home/user/Shamble-Frontlines/.gate/te-m3
TEST=l2::delay::tests::a_shutdown_s_never_completes_ends_at_its_deadline_without_spinning
TAG="${1:?tag}"
MODE="${2:?all or one}"
N="${3:-20}"
P="${4:-1}"
H="${5:-0}"
case "$MODE" in all | one) ;; *) echo "STOP: MODE is all or one (got: $MODE)"; exit 2 ;; esac
mkdir -p "$OUT"
build="$OUT/$TAG-build.log"
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
steal() { awk '/^cpu /{print $9}' /proc/stat; }
echo "binary $BIN; mode $MODE; $N runs in each of $P loops; $H busy loops; steal $(steal); start $(date -u +%T) UTC"
stop="$OUT/$TAG-stop"
rm -f "$stop"
hogs=()
for h in $(seq 1 "$H"); do
  (while :; do :; done) &
  hogs+=($!)
done
trap 'for x in "${hogs[@]}"; do kill "$x" 2>/dev/null; done' EXIT
loop() {
  local p=$1 i
  for i in $(seq 1 "$N"); do
    [ -e "$stop" ] && return 0
    local f="$OUT/$TAG-$p-$i.log" s0 s1 r
    s0=$(steal)
    if [ "$MODE" = all ]; then
      (cd crates/net && "$BIN") > "$f" 2>&1
    else
      (cd crates/net && "$BIN" --exact "$TEST" --nocapture --test-threads=1) > "$f" 2>&1
    fi
    r=$?
    s1=$(steal)
    echo "$(date -u +%T) loop $p run $i exit $r steal $((s1 - s0)): $(grep -E '^test result:' "$f" | tail -1)"
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
echo "end $(date -u +%T) UTC; steal $(steal); first failure: $(cat "$stop" 2>/dev/null || echo none)"
exit $status

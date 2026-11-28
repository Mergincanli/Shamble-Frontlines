#!/usr/bin/env bash
# U3C-HELD-1's loop (u3c-held-1-plan-draft.md steps 1 and 3; the advisor's plan review advisor-plan-review-full.json,
# C1-C3, HELD-Q1, HELD-Q10), in te_m3_loop.sh's form. A diagnosis or evidence run only: never a retry into green.
# Builds the workspace's tests once in the bites build, exactly check.mjs's bites command (`cargo test --locked
# --workspace --all-features --no-run`; never `-p npt-net`, which builds another binary), takes npt-net's library test
# binary by its one `Executable unittests src/lib.rs (...npt_net-<16 hex>)` line and stops (exit 2) unless it is
# npt_net-ed8673eeeaf4db12, then runs it N times in each of P parallel loops, from crates/net, while H busy loops load
# the CPUs (H = 0: none). MODE `all` runs the whole binary (libtest's default threads: the load check.mjs gives it);
# MODE `one` runs TEST alone with --nocapture. TEST is G-NC2 unless a 6th argument names another test. Each run's
# whole output goes to .gate/u3c-held-1/<tag>-<loop>-<i>.log; each run's line carries libtest's summary line and
# TEST's own result line; the first run that exits non-zero stops every loop and is named, with libtest's failures list
# and every failing test's whole `---- <name> stdout ----` block (C3; MODE one: the run's last 80 lines).
# Run it through main-run.sh (the lane lock held exclusively), from the main folder:
#   bash /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/main-run.sh <log> \
#     bash /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u3c-held-1/u3c_held_loop.sh <tag> <all|one> <N> <P> <H> [TEST]
set -u
OUT=/home/user/Shamble-Frontlines/.gate/u3c-held-1
WANT=npt_net-ed8673eeeaf4db12
TAG="${1:?tag}"
MODE="${2:?all or one}"
N="${3:-20}"
P="${4:-1}"
H="${5:-0}"
TEST="${6:-l2::delay::tests::under_loss_the_held_bytes_stay_within_64_kb_and_the_front_records_acknowledged_prefix}"
case "$MODE" in all | one) ;; *) echo "STOP: MODE is all or one (got: $MODE)"; exit 2 ;; esac
mkdir -p "$OUT"
build="$OUT/$TAG-build.log"
cargo test --locked --workspace --all-features --no-run > "$build" 2>&1
rc=$?
echo "build exit $rc ($build)"
[ $rc -eq 0 ] || exit $rc
BINS=$(sed -n 's/^ *Executable unittests src\/lib\.rs (\(.*\/npt_net-[0-9a-f]\{16\}\))$/\1/p' "$build")
[ "$(printf '%s\n' "$BINS" | grep -c .)" -eq 1 ] || { echo "STOP: not exactly one npt-net library test binary in $build (got: $BINS)"; exit 2; }
BIN="$BINS"
case "$BIN" in
  /*) ;;
  *) BIN="$(pwd)/$BIN" ;;
esac
[ "$(basename "$BIN")" = "$WANT" ] || { echo "STOP: npt-net's library test binary is not $WANT (got: $BIN)"; exit 2; }
[ -x "$BIN" ] || { echo "STOP: $BIN is not executable"; exit 2; }
sha256sum crates/net/src/l2/delay.rs crates/net/src/l2/delay_tests.rs "$BIN"
grep -E "^ *Compiling npt-net " "$build" | sed 's/^ *//'
steal() { awk '/^cpu /{print $9}' /proc/stat; }
echo "binary $BIN; mode $MODE; test $TEST; $N runs in each of $P loops; $H busy loops; steal $(steal); start $(date -u +%T) UTC"
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
    echo "$(date -u +%T) loop $p run $i exit $r steal $((s1 - s0)): $(grep -E '^test result:' "$f" | tail -1); $(grep -F "test $TEST ... " "$f" | tail -1)"
    if [ $r -ne 0 ]; then
      echo "$p-$i" > "$stop"
      echo "FAILED: loop $p run $i, exit $r, log $f; libtest's failures list:"
      sed -n '/^failures:$/,/^test result:/p' "$f" | grep -E '^    ' | head -40
      if [ "$MODE" = all ]; then
        echo "every failing test's stdout block:"
        sed -n '/^---- .* stdout ----$/,/^failures:$/p' "$f"
      else
        echo "the run's last 80 lines:"
        tail -80 "$f"
      fi
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

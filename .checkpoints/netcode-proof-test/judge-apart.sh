#!/usr/bin/env bash
# Step 17's judge measured apart (advisor-u4r-r2-full.md:23; RR3-L6 in advisor-u4r-r1-full.md; advisor-u4r-sets-r1-full.md
# Q2 and next steps 3 and 5: whole set folders too). One measured run's kept logs, or a whole set folder's, are judged
# by the set's own npt-judge and marks.json copies, into a scratch folder, twice. Each call is wrapped in peak.py and
# holds the lane lock exclusively, so no set runs meanwhile.
#   (1) With the set's marks (default --only PM-0001-2,PM-0001-17,PM-0001-57; B1's set adds PM-0001-18): the judge's
#       wall time and peak memory on those bytes.
#   (2) With --only PM-0001-30: the time and memory of reading the logs alone. PM-0001-30 has no pre: rule, so
#       JDG-ONLY-1's interim rule holds. No -30 run is in these logs, so the judging is trivial, and npt-judge reads
#       every log under --logs before it judges (main.rs:99; lib.rs:37-46).
# read_log's share is (2)'s wall time over (1)'s. The verdict files are scratch and never counted. The bytes (`du -sb`)
# go in the log, so P0's judge memory and time can be projected from its bytes.
# Usage: bash judge-apart.sh <set folder, e.g. u4r-P3-0f066338dd8d> [<run, e.g. l1-p3-s1> | ""] [<marks list>]
#        (no run, or "": the whole set folder)
set -u
M=/home/user/Shamble-Frontlines
N=$M/.checkpoints/netcode-proof-test
set_id="$1"
run="${2:-}"
only="${3:-PM-0001-2,PM-0001-17,PM-0001-57}"
BIN=$M/.gate/netcode-proof-test/bin/$set_id
LOGS=$M/.gate/netcode-proof-test/runs/$set_id${run:+/$run}
OUT=$M/.gate/u4r-judge-apart/$set_id-${run:-all}
LOG=$OUT.log
if [ ! -x "$BIN/npt-judge" ] || [ ! -f "$BIN/marks.json" ] || [ ! -d "$LOGS" ]; then
  echo "STOP: missing $BIN/npt-judge, $BIN/marks.json or $LOGS"
  exit 2
fi
if [ -e "$OUT" ]; then
  echo "STOP: $OUT exists already; a measurement is never overwritten"
  exit 2
fi
mkdir -p "$OUT"
{
  echo "judge-apart: $set_id ${run:-(the whole set folder)} at $(date -u +%T) UTC; uptime: $(uptime)"
  echo "npt-judge sha256 $(sha256sum "$BIN/npt-judge" | cut -d' ' -f1); marks.json sha256 $(sha256sum "$BIN/marks.json" | cut -d' ' -f1)"
  echo "the logs: $(find "$LOGS" -name '*.jsonl' -printf '%s\n' | awk '{s+=$1} END {print s}') B in $(find "$LOGS" -name '*.jsonl' | wc -l) .jsonl files"
  find "$LOGS" -name '*.jsonl' -printf '  %P %s\n' | sort
  echo "=== (1) the set's marks: --only $only"
  python3 -I "$N/peak.py" flock -x "$M/.gate/shared-target/.lane.lock" "$BIN/npt-judge" --marks "$BIN/marks.json" --logs "$LOGS" --only "$only" --out "$OUT/full.json"
  echo "=== (1) exit $?"
  echo "=== (2) reading alone (--only PM-0001-30)"
  python3 -I "$N/peak.py" flock -x "$M/.gate/shared-target/.lane.lock" "$BIN/npt-judge" --marks "$BIN/marks.json" --logs "$LOGS" --only PM-0001-30 --out "$OUT/read.json"
  echo "=== (2) exit $?"
  echo "end $(date -u +%T) UTC; uptime: $(uptime)"
} > "$LOG" 2>&1
cat "$LOG"

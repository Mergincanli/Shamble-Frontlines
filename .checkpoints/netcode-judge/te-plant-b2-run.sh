#!/bin/bash
# Runs each named plant in the scratch copy, records which judge tests fail. Usage: te-plant-b-run.sh plant...
N=/home/user/Shamble-Frontlines/.gate/netcode-judge/.checkpoints/netcode-judge
W=/home/user/Shamble-Frontlines/.gate/netcode-judge/.gate/te-plant-b
OUT=$W/plant-results-2.txt
for p in "$@"; do
  node $N/te-plant-b2.mjs "$p" || { echo "$p: PLANT FAILED TO APPLY" >> $OUT; continue; }
  res=$(cd $W/netcode-proof && CARGO_TARGET_DIR=$W/target cargo test -q -p npt-judge --locked --no-fail-fast 2>&1)
  fails=$(echo "$res" | grep -E "^    [a-z0-9_:]+$" | sort -u | tr '\n' ' ')
  sums=$(echo "$res" | grep -E "^test result" | sed 's/; 0 ignored.*//' | tr '\n' '|')
  if echo "$res" | grep -q "error\[E"; then echo "$p: COMPILE ERROR" >> $OUT; echo "$res" | grep -A5 "error\[E" | head -20 >> $OUT; continue; fi
  echo "$p: failing=[$fails] $sums" >> $OUT
done
node $N/te-plant-b2.mjs reset

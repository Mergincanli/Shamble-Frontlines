#!/usr/bin/env bash
# PM-0002-19: 20 generator runs on one commit give 20 identical sets of files. Run from the repository root:
#   bash .checkpoints/cs1-types/runs20.sh > .checkpoints/cs1-types/runs20-results.txt 2>&1
# Each run writes every generated file again (write mode, default test threads) and hashes them all.
set -u
gen=packages/types/src/generated
out=.gate/cs1-types/runs20; mkdir -p "$out"
echo "commit $(git rev-parse HEAD), $(date -u +%Y-%m-%dT%H:%M:%SZ); tree clean before: [$(git status --short -- crates packages | tr '\n' ' ')]"
failed=0
for n in $(seq 1 20); do
  SHAMBLE_WRITE_TYPES=1 cargo test --workspace --locked > "$out/run-$n.txt" 2>&1 || { failed=$((failed + 1)); echo "run $n: cargo test failed"; }
  find "$gen" -type f | LC_ALL=C sort | xargs sha256sum > "$out/set-$n.txt"
  echo "run $n: $(wc -l < "$out/set-$n.txt") files, set sha256 $(sha256sum < "$out/set-$n.txt" | cut -c1-64)"
done
echo "distinct sets: $(for n in $(seq 1 20); do sha256sum < "$out/set-$n.txt"; done | sort -u | wc -l) (wanted: 1); failed runs: $failed (wanted: 0)"
echo "the one set:"; sed 's/^/  /' "$out/set-1.txt"
if git diff --exit-code -- "$gen" > /dev/null && [ -z "$(git status --short -- "$gen")" ]; then echo "git: no difference from the committed files, no new file"; else echo "git: the generated folder DIFFERS from the commit"; failed=$((failed + 1)); fi
[ "$failed" -eq 0 ]

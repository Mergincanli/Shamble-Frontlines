#!/usr/bin/env bash
# The planted defects of change-set 1 (PM-0002-15, -16, -17), run once in a throwaway worktree and never committed.
# Run from the repository root:  bash .checkpoints/cs1-types/plant.sh > .checkpoints/cs1-types/plant-results.txt 2>&1
# It needs the root's node_modules (npm ci --ignore-scripts) and reuses the root's target folder for the registry crates.
# Each defect: plant, run, print the command, its exit code and the failing lines, put the file back.
set -u
root=$(pwd)
wt="$root/.gate/cs1-types/plant"
logs="$root/.gate/cs1-types/plant-logs"
[ -e "$wt" ] && { echo "$wt exists: remove it first (git worktree remove .gate/cs1-types/plant)"; exit 2; }
mkdir -p "$logs"
git worktree add --detach "$wt" HEAD > "$logs/worktree.txt" 2>&1 || { cat "$logs/worktree.txt"; exit 2; }
export CARGO_TARGET_DIR="$root/target"
cd "$wt" || exit 2
echo "worktree at $(git rev-parse HEAD), $(date -u +%Y-%m-%dT%H:%M:%SZ)"
verdicts=0; wrong=0

# run <label> <want: pass|fail> <pattern the output must hold, or -> <command...>
run() {
  local label=$1 want=$2 pattern=$3; shift 3
  local log="$logs/$label.txt"
  "$@" > "$log" 2>&1; local code=$?
  echo; echo "## $label"; echo "command: $*"; echo "exit code: $code (wanted: $want)"
  local ok=yes
  if [ "$want" = pass ] && [ "$code" -ne 0 ]; then ok=no; fi
  if [ "$want" = fail ] && [ "$code" -eq 0 ]; then ok=no; fi
  if [ "$pattern" != - ]; then
    if grep -qE -- "$pattern" "$log"; then grep -E -- "$pattern" "$log" | head -4 | cut -c1-220 | sed 's/^/  | /'; else ok=no; echo "  the output does not hold: $pattern"; fi
  fi
  grep -E '^test result|^test .* FAILED|^failures:$|error TS[0-9]+' "$log" | sort -u | head -8 | cut -c1-220 | sed 's/^/  | /'
  verdicts=$((verdicts + 1)); [ "$ok" = yes ] || wrong=$((wrong + 1))
  echo "as wanted: $ok"
}
# The copies live outside the worktree: a stray file in a generated folder fails the export tests (run 1 showed it).
kept() { echo "$logs/kept/$(echo "$1" | tr / _)"; }
keep() { mkdir -p "$logs/kept" && cp "$1" "$(kept "$1")"; }       # before a plant
back() { cp "$(kept "$1")" "$1" && rm "$(kept "$1")"; }           # after it
edit() { python3 - "$@" <<'EOF'
import sys
path, old, new, which = sys.argv[1:5]
text = open(path).read()
count = text.count(old)
if count == 0: sys.exit(f"plant failed: {old!r} is not in {path}")
i = text.rindex(old) if which == "last" else text.index(old)
open(path, "w").write(text[:i] + new + text[i + len(old):])
EOF
}
tsc() { node "$root/node_modules/typescript/bin/tsc" -p "$1"; }
vitest_types() { ( cd packages/types && node "$root/node_modules/vitest/vitest.mjs" run ); }
python3 - "$root" <<'EOF'
import json, sys
root = sys.argv[1]
base = json.load(open(f"{root}/.gate/cs1-types/tsconfig.json"))
base["include"] = ["./plant/packages/types/src"]
json.dump(base, open(f"{root}/.gate/cs1-types/tsconfig.plant.json", "w"), indent=2)
shipped = dict(base); shipped["compilerOptions"] = dict(base["compilerOptions"]); shipped["compilerOptions"].pop("skipLibCheck", None)
shipped["exclude"] = ["./plant/packages/types/src/**/*.test.ts"]
json.dump(shipped, open(f"{root}/.gate/cs1-types/tsconfig.plant.shipped.json", "w"), indent=2)
EOF
cfg="$root/.gate/cs1-types/tsconfig.plant.json"; cfg_shipped="$root/.gate/cs1-types/tsconfig.plant.shipped.json"
messages=crates/protocol/src/messages.rs
layout=crates/client-endpoint/src/layout.rs
gen=packages/types/src/generated

run "0a-clean-protocol" pass - cargo test -p protocol --locked
run "0b-clean-client-endpoint" pass - cargo test -p client-endpoint --locked
run "0c-clean-tsc" pass - tsc "$cfg"

# (1) PM-0002-15: a field renamed in an exported Rust type, no regeneration.
keep $messages; edit $messages "    pub group: u8," "    pub group_index: u8," last
run "1-rename-without-regenerating" fail 'generated/protocol/Snapshot\.ts is not what the Rust types give' cargo test -p protocol --locked
back $messages

# (2) PM-0002-15: a hand edit to a file ts-rs wrote.
keep $gen/protocol/Join.ts; edit $gen/protocol/Join.ts "protocol_version" "protocol_versiom" first
run "2-hand-edit-of-Join.ts" fail 'generated/protocol/Join\.ts is not what the Rust types give' cargo test -p protocol --locked
back $gen/protocol/Join.ts

# (2b) PM-0002-15: a hand edit to the file our own generator wrote; the hash line is untouched.
keep $gen/block/layout.ts; edit $gen/block/layout.ts "layout_hash: { offset: 4," "layout_hash: { offset: 5," first
run "2b-hand-edit-of-layout.ts" fail 'generated/block/layout\.ts is not what the layout table gives' cargo test -p client-endpoint --locked
back $gen/block/layout.ts

# (2c) PM-0002-15: a file added by hand to a generated folder, one per generator.
printf 'export type Extra = number;\n' > $gen/protocol/Extra.ts
run "2c-extra-file-in-generated-protocol" fail 'generated/protocol/Extra\.ts is not a generated file' cargo test -p protocol --locked
rm $gen/protocol/Extra.ts
printf 'export const extra = 1;\n' > $gen/block/extra.ts
run "2d-extra-file-in-generated-block" fail 'generated/block must hold layout\.ts and nothing else' cargo test -p client-endpoint --locked
rm $gen/block/extra.ts

# (3) The control: the same rename, regenerated. Rust passes; the shipped TypeScript passes; the package's own test, which
# builds a Snapshot, fails the type check until it follows the rename.
keep $messages; keep $gen/protocol/Snapshot.ts; keep packages/types/src/index.test.ts
edit $messages "    pub group: u8," "    pub group_index: u8," last
run "3a-rename-and-regenerate" pass - env SHAMBLE_WRITE_TYPES=1 cargo test -p protocol --locked
run "3b-plain-run-after" pass - cargo test -p protocol --locked
echo "  files changed by the regeneration: $(git status --short -- $gen | tr '\n' ' ')"
run "3c-tsc-shipped-sources" pass - tsc "$cfg_shipped"
run "3d-tsc-with-the-stale-test" fail 'index\.test\.ts.*error TS' tsc "$cfg"
edit packages/types/src/index.test.ts "    group: ack.group," "    group_index: ack.group," first
run "3e-tsc-after-the-test-follows" pass - tsc "$cfg"
run "3f-vitest" pass 'Tests +[0-9]+ passed' vitest_types
back $messages; back $gen/protocol/Snapshot.ts; back packages/types/src/index.test.ts

# (4) PM-0002-16: a layout change in Rust (a header field added, the stride grown), no regeneration.
keep $layout
edit $layout 'Section { name: "header", stride: 8, fields: &[' 'Section { name: "header", stride: 12, fields: &[' first
edit $layout '        Field { name: "layout_hash", ty: ElementType::U32, offset: 4 },' '        Field { name: "layout_hash", ty: ElementType::U32, offset: 4 },
        Field { name: "soldier_count", ty: ElementType::U32, offset: 8 },' first
run "4-layout-change-without-regenerating" fail 'generated/block/layout\.ts is not what the layout table gives' cargo test -p client-endpoint --locked
echo "  LAYOUT_HASH lines in the failing message (the committed file's, then the changed table's):"
grep -E '^export const LAYOUT_HASH' "$logs/4-layout-change-without-regenerating.txt" | sed 's/^/  | /'
echo "  tests that failed: $(grep -E '^test .* FAILED' "$logs/4-layout-change-without-regenerating.txt" | tr '\n' ' ')"
back $layout

# PM-0002-17's bites: syntax that is not erasable, in the generated folder, read by the same config.
printf 'export class Bite {\n  constructor(public readonly x: number) {}\n}\n' > $gen/bite-parameter-property.ts
run "17a-parameter-property" fail 'bite-parameter-property\.ts.*error TS' tsc "$cfg"
rm $gen/bite-parameter-property.ts
printf 'export enum Bite {\n  A,\n}\n' > $gen/bite-enum.ts
run "17b-enum" fail 'bite-enum\.ts.*error TS' tsc "$cfg"
rm $gen/bite-enum.ts
printf 'export declare enum Ambient {\n  A,\n}\n' > $gen/bite-declare-enum.ts
run "17c-declare-enum-recorded" pass - tsc "$cfg"   # recorded, not judged: "wanted: pass" only says what this run expects
rm $gen/bite-declare-enum.ts

run "9-clean-again-protocol" pass - cargo test -p protocol --locked
run "9-clean-again-client-endpoint" pass - cargo test -p client-endpoint --locked
echo; echo "## the worktree after the runs"
echo "git status --short: [$(git status --short | tr '\n' ' ')] (empty means every file is back)"
# A failed export test leaves its scratch folder to be looked at; the planted runs' folders go, file by file.
for d in /tmp/shamble-types-protocol-*; do [ -d "$d" ] && rm "${d:?}"/*.ts && rmdir "${d:?}"; done
cd "$root" || exit 2
if git worktree remove .gate/cs1-types/plant; then echo "worktree removed (no force)"; else echo "worktree NOT removed"; wrong=$((wrong + 1)); fi
echo; echo "runs: $verdicts, not as wanted: $wrong"
[ "$wrong" -eq 0 ]

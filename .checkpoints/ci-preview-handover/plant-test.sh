#!/usr/bin/bash
# Plant test for the drafted guard step (guard.sh, extracted from the draft workflow). Each case gets its own fresh
# folder made with mktemp; the real build is copied in, one fault is planted, the guard runs with GITHUB_WORKSPACE set
# to that folder. Expected: the real build passes (exit 0); every plant fails (exit 1). Usage: plant-test.sh <build dir>
set -u
here=$(dirname "$0"); guard="$here/guard.sh"; build="$1"; base=$(mktemp -d "$here/cases.XXXXXX")
case_dir() { local c; c=$(mktemp -d "$base/c.XXXXXX"); mkdir "$c/.gate"; cp -r "$build" "$c/.gate/dist-production"; echo "$c"; }
check() { local out rc; out=$(GITHUB_WORKSPACE="$1" /usr/bin/bash --noprofile --norc -euo pipefail "$guard" 2>&1); rc=$?
  printf '%-34s exit %s | %s\n' "$2" "$rc" "$(printf '%s' "$out" | tr '\n' ' ' | cut -c1-90)"; }
c=$(case_dir); check "$c" "the real build"
c=$(case_dir); ln -s /etc/hostname "$c/.gate/dist-production/assets/planted.js"; check "$c" "plant: a symbolic link"
c=$(case_dir); mkfifo "$c/.gate/dist-production/planted"; check "$c" "plant: a fifo"
c=$(mktemp -d "$base/c.XXXXXX"); mkdir -p "$c/.gate/dist-production"; check "$c" "plant: an empty build"
c=$(mktemp -d "$base/c.XXXXXX"); mkdir "$c/.gate"; cp -r "$build" "$c/elsewhere"; ln -s "$c/elsewhere" "$c/.gate/dist-production"; check "$c" "plant: the build folder a link"
c=$(mktemp -d "$base/c.XXXXXX"); mkdir -p "$c/elsewhere"; cp -r "$build" "$c/elsewhere/dist-production"; ln -s "$c/elsewhere" "$c/.gate"; check "$c" "plant: .gate a link"
echo "cases left in $base (scratch)"

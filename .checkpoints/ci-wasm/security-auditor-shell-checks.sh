#!/usr/bin/env bash
# security-auditor, part ci-text: local checks of single shell lines of the proposed toolchain.yml (2026-11-19).
# Runs nothing fetched, no network, no sudo. Work files go under .gate/ci-wasm/security/ (made fresh each run).
# Run from the repository's root: bash .checkpoints/ci-wasm/security-auditor-shell-checks.sh
set -u
W="$PWD/.gate/ci-wasm/security/work"
case "$W" in */.gate/ci-wasm/security/work) ;; *) echo "unexpected work path"; exit 2 ;; esac
[ ! -e "$W" ] || { echo "remove $W first"; exit 2; }
mkdir -p "$W"
STEP=(bash --noprofile --norc -eo pipefail -c)   # GitHub's `shell: bash`
N='wasm-bindgen-0.2.130-x86_64-unknown-linux-musl'

echo "== T1 the gh floor (line 98): sort -V -C, exit code per printed version"
for v in 2.103.3 2.93.0 2.92.0 2.100.0 2.93.1 "" DEV 2.93.0-rc1 2.92.0-1 v2.93.0 3.0.0; do
  printf '%s\n' 2.93.0 "$(printf 'gh version %s (2026-01-01)\nhttps://x\n' "$v" | awk 'NR == 1 { print $3 }')" | sort -V -C
  echo "  printed '$v' -> exit $?"
done

echo "== T2 the sed on curl's -w line (line 86)"
for u in 'https://release-assets.githubusercontent.com/github-production-release-asset/1/abc?sp=r&sig=SECRET&jwt=X' \
         'https://github.com/a/b/releases/download/0.2.130/f.tar.gz' 'https://h/p?a=1?b=2'; do
  printf 'final address: %s\n' "$u" | sed 's/?.*$/?(query left out)/'
done

echo "== T3 a failing curl piped to sed under -eo pipefail (lines 84-86): does the step stop?"
"${STEP[@]}" 'fakecurl() { echo "final address: https://h/p?sig=1"; return 22; }; fakecurl | sed "s/?.*$/?(query left out)/"; echo "  NOT STOPPED"'
echo "  step exit $?"

echo "== T4 the awk test on tar -tvzf (line 109), on crafted archives"
mk() { rm -rf "$W/src" && mkdir -p "$W/src/$N" && cd "$W/src" && for f in LICENSE-APACHE LICENSE-MIT README.md wasm-bindgen-test-runner wasm2es6js; do echo x > "$N/$f"; done; }
awkcheck() { tar -tvzf "$1" | awk '{ print } substr($1, 1, 1) !~ /^[-d]$/ || NF != 6 || $3 > 67108864 { bad = 1 } END { exit bad }' >/dev/null; echo "  $2 -> exit $?"; }
mk; echo prog > "$N/wasm-bindgen"; tar -czf "$W/ok.tgz" "$N"; cd - >/dev/null; awkcheck "$W/ok.tgz" "plain archive"
mk; ln -s /bin/sh "$N/wasm-bindgen"; tar -czf "$W/sym.tgz" "$N"; cd - >/dev/null; awkcheck "$W/sym.tgz" "symlink member"
mk; echo prog > "$N/wasm-bindgen"; ln "$N/wasm-bindgen" "$N/hard"; tar -czf "$W/hard.tgz" "$N"; cd - >/dev/null; awkcheck "$W/hard.tgz" "hard link member"
mk; echo prog > "$N/wasm-bindgen"; echo y > "$N/a b"; tar -czf "$W/space.tgz" "$N"; cd - >/dev/null; awkcheck "$W/space.tgz" "name with a space"
mk; truncate -s 70M "$N/wasm-bindgen"; tar -czf "$W/big.tgz" "$N"; cd - >/dev/null; awkcheck "$W/big.tgz" "70 MiB member"
mk; echo prog > "$N/wasm-bindgen"; mkfifo "$N/fifo"; tar -czf "$W/fifo.tgz" "$N"; cd - >/dev/null; awkcheck "$W/fifo.tgz" "fifo member"
tar -tvzf "$W/ok.tgz" | head -2 | sed 's/^/  sample: /'

echo "== T5 extraction of one named member with --strip-components=1 (lines 111-113)"
mkdir "$W/x1"; tar -xzf "$W/ok.tgz" -C "$W/x1" --no-same-owner --no-same-permissions --strip-components=1 "$N/wasm-bindgen"
echo "  ok.tgz -> [$(ls -A "$W/x1")]"; [ "$(ls -A "$W/x1")" = wasm-bindgen ] && echo "  ls -A compare passes"
mk; mkdir "$N/wasm-bindgen"; echo e > "$N/wasm-bindgen/evil"; tar -czf "$W/dirprog.tgz" "$N"; cd - >/dev/null
mkdir "$W/x2"; tar -xzf "$W/dirprog.tgz" -C "$W/x2" --strip-components=1 "$N/wasm-bindgen"
echo "  a folder named wasm-bindgen -> [$(ls -A "$W/x2")] (ls -A compare would pass; [ -f ] after install refuses)"; find "$W/x2" | sed "s|$W/||; s/^/    /"

echo "== T6 the drift loop (lines 77-82) on the real module, a broken copy, and a copy with the two bytes values swapped"
drift() { "${STEP[@]}" '
  WB_VERSION=0.2.130 WB_NAME='"$N"' WB_ARCHIVE_BYTES=11860397 WB_ARCHIVE_SHA256=9687f3bb551b9f66f37ed32a40d7913a00f7962db653054e56857bc306c3227f
  WB_PROGRAM_BYTES=8170784 WB_PROGRAM_SHA256=ca4df773020dae87d92b9eadb2b85412bea7a9cf6318ab9999dd119fd9f46931
  url="https://github.com/wasm-bindgen/wasm-bindgen/releases/download/$WB_VERSION/$WB_NAME.tar.gz"
  for line in "  version: \"$WB_VERSION\"," "    url: \"$url\"," "    bytes: $WB_ARCHIVE_BYTES," \
      "    sha256: \"$WB_ARCHIVE_SHA256\"," "    member: \"$WB_NAME/wasm-bindgen\"," \
      "    bytes: $WB_PROGRAM_BYTES," "    sha256: \"$WB_PROGRAM_SHA256\","; do
    grep -qxF -- "$line" "$1" || { echo "  lacks: $line"; exit 1; }
  done' drift "$1"; echo "  $2 -> exit $?"; }
M=packages/client/scripts/wasm-bindgen-tool.mjs
drift "$M" "the real module"
sed 's/    bytes: 8170784,/    bytes: 8170785,/' "$M" > "$W/broken.mjs"; drift "$W/broken.mjs" "program bytes changed"
sed 's/    bytes: 8170784,/    bytes: TMPX,/; s/    bytes: 11860397,/    bytes: 8170784,/; s/    bytes: TMPX,/    bytes: 11860397,/' "$M" > "$W/swapped.mjs"
grep -n "bytes:" "$W/swapped.mjs" | sed 's/^/    /'; drift "$W/swapped.mjs" "archive and program bytes swapped (fails later in build.mjs)"

echo "== T7 install -D: the mode of the folder it makes, by umask (container root, scratch only)"
echo prog > "$W/prog"
for m in 0022 0002 0000; do ( umask "$m"; install -D -m 0555 "$W/prog" "$W/opt-$m/wasm-bindgen-0.2.130/wasm-bindgen" ); \
  echo "  umask $m -> folder $(stat -c %a "$W/opt-$m/wasm-bindgen-0.2.130"), file $(stat -c %a "$W/opt-$m/wasm-bindgen-0.2.130/wasm-bindgen")"; done

echo "== T8 the plain-file test (line 115) on a link and on a file"
ln -s "$W/prog" "$W/link"
for f in "$W/prog" "$W/link"; do [ -f "$f" ] && [ ! -L "$f" ] && echo "  ${f##*/}: passes" || echo "  ${f##*/}: refused"; done
echo "== done"

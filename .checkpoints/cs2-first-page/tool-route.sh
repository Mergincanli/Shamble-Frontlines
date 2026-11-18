#!/usr/bin/env bash
# The route by which the wasm-bindgen command-line tool reaches this container (plan section 7, item 5a), for the one
# file Mustafa said yes to on 2026-11-18 (TOOL-SOURCE (a)). It stops at the first difference and runs nothing of the
# tool. Run from the repository's root:  bash .checkpoints/cs2-first-page/tool-route.sh
set -euo pipefail
URL='https://github.com/wasm-bindgen/wasm-bindgen/releases/download/0.2.130/wasm-bindgen-0.2.130-x86_64-unknown-linux-musl.tar.gz'
ARCHIVE_BYTES=11860397
ARCHIVE_SHA256=9687f3bb551b9f66f37ed32a40d7913a00f7962db653054e56857bc306c3227f
TOP='wasm-bindgen-0.2.130-x86_64-unknown-linux-musl'
PROGRAM_BYTES=8170784
PROGRAM_SHA256=ca4df773020dae87d92b9eadb2b85412bea7a9cf6318ab9999dd119fd9f46931
DOWNLOAD=.gate/cs2-first-page/tool-download
TOOLS=.gate/tools
OUT="$TOOLS/wasm-bindgen-0.2.130"
stop() { echo "STOP: $1 (nothing of the tool was run; tell Mustafa)"; exit 1; }

date -u
echo "== 1. a new folder, and the download"
for d in .gate .gate/cs2-first-page; do [ -d "$d" ] && [ ! -L "$d" ] || stop "$d is missing or a link"; done
[ ! -e "$DOWNLOAD" ] || stop "$DOWNLOAD exists already"
mkdir "$DOWNLOAD"
FILE="$DOWNLOAD/wasm-bindgen-0.2.130-x86_64-unknown-linux-musl.tar.gz"
curl --fail -L --max-redirs 2 --proto '=https' --proto-redir '=https' --max-filesize 12000000 -sS -o "$FILE" \
  -w 'final address: %{url_effective}\nstatus: %{http_code}, redirects: %{num_redirects}\n' "$URL" | sed -E 's/\?.*$/?(query left out)/'

echo "== 2. size, then sha256, before any tar call"
bytes=$(stat -c %s "$FILE"); echo "size: $bytes"
[ "$bytes" = "$ARCHIVE_BYTES" ] || stop "the archive is $bytes bytes, not $ARCHIVE_BYTES"
printf '%s  %s\n' "$ARCHIVE_SHA256" "$FILE" | sha256sum -c - || stop "the archive's sha256 is not $ARCHIVE_SHA256"

echo "== 3. the member list"
tar -tzf "$FILE" > "$DOWNLOAD/members.txt"
tar -tvzf "$FILE" > "$DOWNLOAD/members-long.txt"
cat "$DOWNLOAD/members-long.txt"
python3 -I - "$DOWNLOAD/members.txt" "$DOWNLOAD/members-long.txt" "$TOP" <<'PY' || stop "the member list is not the expected one"
import sys
names = open(sys.argv[1], encoding="utf-8").read().splitlines()
long = open(sys.argv[2], encoding="utf-8").read().splitlines()
top = sys.argv[3]
want = {top + "/"} | {f"{top}/{n}" for n in ("README.md", "LICENSE-MIT", "LICENSE-APACHE", "wasm-bindgen", "wasm-bindgen-test-runner", "wasm2es6js")}
problems = []
if len(names) != len(set(names)): problems.append("a name appears twice")
if set(names) != want: problems.append(f"names differ: extra {sorted(set(names) - want)}, missing {sorted(want - set(names))}")
if len(long) != len(names): problems.append("the two listings differ in length")
for line in long:
    kind, size, name = line[0], int(line.split()[2]), line.split()[-1]
    if name.endswith("/"):
        if kind != "d": problems.append(f"{name}: not a folder")
    elif kind != "-": problems.append(f"{name}: not a regular file (type {kind!r})")
    if size > 64 * 1024 * 1024: problems.append(f"{name}: {size} bytes is above 64 MiB")
    if " -> " in line or " link to " in line: problems.append(f"{name}: a link")
for p in problems: print("  " + p)
print(f"members: {len(names)} (the folder and six files)" if not problems else "member list refused")
sys.exit(1 if problems else 0)
PY

echo "== 4. only the program, into a new empty folder"
if [ -e "$TOOLS" ]; then [ -d "$TOOLS" ] && [ ! -L "$TOOLS" ] || stop "$TOOLS is not a plain folder"; else mkdir "$TOOLS"; fi
[ ! -e "$OUT" ] || stop "$OUT exists already"
mkdir "$OUT"
tar -xzf "$FILE" -C "$OUT" --no-same-owner --no-same-permissions --strip-components=1 "$TOP/wasm-bindgen"
[ "$(ls -A "$OUT")" = "wasm-bindgen" ] || stop "$OUT holds something else than the one program"

echo "== 5. the program, before any run"
PROGRAM="$OUT/wasm-bindgen"
[ -f "$PROGRAM" ] && [ ! -L "$PROGRAM" ] || stop "the program is not a regular file"
pbytes=$(stat -c %s "$PROGRAM"); echo "size: $pbytes"
[ "$pbytes" = "$PROGRAM_BYTES" ] || stop "the program is $pbytes bytes, not his $PROGRAM_BYTES"
printf '%s  %s\n' "$PROGRAM_SHA256" "$PROGRAM" | sha256sum -c - || stop "the program's sha256 is not his $PROGRAM_SHA256"
chmod 0555 "$PROGRAM" "$OUT"
readelf -h "$PROGRAM" | grep -E 'Class:|Machine:|Type:' || stop "readelf could not read the program's header"
readelf -h "$PROGRAM" | grep -q 'Class:.*ELF64' || stop "not an ELF64 file"
readelf -h "$PROGRAM" | grep -q 'Machine:.*X86-64' || stop "not an x86-64 file"
if readelf -l "$PROGRAM" | grep -q 'INTERP'; then stop "the program asks for an interpreter: it is not static"; fi
echo "no interpreter: a static program"
stat -c 'mode %a, owner %u:%g, %n' "$PROGRAM" "$OUT"
echo "== done: the program is in place, checked and not run"
date -u

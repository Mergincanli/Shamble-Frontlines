#!/usr/bin/env bash
# security-auditor, OPT-PATH reading of #64's proposed lines (ci-edit-2.diff), 2026-11-23.
# Runs the proposal's own install-and-check lines (own-cut.sh: cut from the parsed proposed-2/toolchain.yml by
# security-auditor-opt-yaml.py, equal byte for byte to infra-engineer's cut) on scratch folders, for the cases its 50
# tries did not cover. As root (this container's user), no network, nothing fetched. Every write goes under the
# literal work folder below; nothing is made under the real /.
# Run: bash /home/user/Shamble-Frontlines/.gate/ci-wasm-wt/.checkpoints/ci-wasm/security-auditor-opt-tries.sh
set -u
S=/home/user/Shamble-Frontlines/.gate/ci-wasm-wt/.gate/ci-wasm/sa-opt-path
W=/home/user/Shamble-Frontlines/.gate/ci-wasm-wt/.gate/ci-wasm/sa-opt-path/work
ACLPY=/home/user/Shamble-Frontlines/.gate/ci-wasm-wt/.checkpoints/ci-wasm/security-auditor-opt-acl.py
[ "$(id -u)" = 0 ] || { echo "run as root"; exit 2; }
CUT="$S/own-cut.sh"
[ "$(sha256sum < "$CUT" | cut -c1-64)" = bead1cd5bd9c12dbddfd61225e9d726971287f56091871152fedd548606d11fc ] \
  || { echo "own-cut.sh is not the proposal's cut"; exit 2; }
rm -rf -- /home/user/Shamble-Frontlines/.gate/ci-wasm-wt/.gate/ci-wasm/sa-opt-path/work
mkdir -m 0755 -- /home/user/Shamble-Frontlines/.gate/ci-wasm-wt/.gate/ci-wasm/sa-opt-path/work
acl() { python3 -B "$ACLPY" "$@"; }

# The judge as I propose it (one line per good entry, whatever its name), made from the cut, one change only.
q="'"
old_j='! -perm /022 | wc -l)"'
new_j="! -perm /022 -printf ${q}.\\n${q} | wc -l)\""
body=$(cat "$CUT")
[ "$(printf '%s' "$body" | grep -cF -- "$old_j")" = 1 ] || { echo "the judge is not in the cut once"; exit 2; }
printf '%s\n' "${body/"$old_j"/"$new_j"}" > "$W/cut-printf.sh"
echo "== the replacement judge (cut-printf.sh) differs from the cut only here:"
diff "$CUT" "$W/cut-printf.sh" | sed 's/^/   /'
sed -n '5,11p' "$CUT" > "$W/check-part.sh"     # from the plain-file line to the sha256 line (after the install)
sed -n '6p;8,9p' "$CUT" > "$W/judge-only.sh"   # the walk and the judge alone
sed -n '6p' "$CUT" > "$W/walk.sh"; printf '%s\n' 'printf "    [%s]\n" "$@"; echo "    $# entries"' >> "$W/walk.sh"
mkdir -m 0755 -- "$W/fake-chown" "$W/fake-mkdir"
printf '%s\n' '#!/bin/sh' 'if [ "$1" = -n ] && [ "$2" = chown ]; then echo "stand-in sudo: chown refused" >&2; exit 1; fi' \
  'exec /usr/bin/sudo "$@"' > "$W/fake-chown/sudo"
printf '%s\n' '#!/bin/sh' 'if [ "$1" = -n ] && [ "$2" = mkdir ]; then echo "sudo: a password is required" >&2; exit 1; fi' \
  'exec /usr/bin/sudo "$@"' > "$W/fake-mkdir/sudo"
chmod 0755 "$W/fake-chown/sudo" "$W/fake-mkdir/sudo"

PROG_SRC=/usr/bin/echo
BYTES=$(stat -c %s "$PROG_SRC"); SHA=$(sha256sum < "$PROG_SRC" | cut -c1-64)
MSG_JUDGE="or a folder above it is missing or a link, is not owned by root, or others than root may write in it"
MSG_FRESH="exists already or could not be made: the tool needs a fresh folder made here as root"
MSG_PLAIN="is not a plain file"
pass=0; fail=0

mk() {   # <case folder>: the extracted stand-in program in dl/x, and a stand-in parent "root" (755 root:root)
  mkdir -m 0755 -- "$1" "$1/dl" "$1/dl/x" "$1/root"
  cp -- "$PROG_SRC" "$1/dl/x/wasm-bindgen"
  dl="$1/dl"; WB_TOOL="$1/root/wasm-bindgen-0.2.130/wasm-bindgen"
}

try() {  # <case> <shell e|pf> <lines file> <want exit> <want text> [text that must NOT appear]
  local name=$1 sh=$2 file=$3 want=$4 text=$5 nottext=${6:-} out rc verdict
  local -a shell
  case "$sh" in e) shell=(bash -e) ;; pf) shell=(bash --noprofile --norc -eo pipefail) ;; esac
  out=$(cd "$W" && env -i PATH="${FAKE:+$FAKE:}/usr/sbin:/usr/bin:/sbin:/bin" HOME=/root LANG=C.UTF-8 dl="$dl" \
    WB_TOOL="$WB_TOOL" WB_PROGRAM_BYTES="$BYTES" WB_PROGRAM_SHA256="$SHA" "${shell[@]}" "$file" 2>&1); rc=$?
  if [ "$rc" = "$want" ] && { [ -z "$text" ] || printf '%s\n' "$out" | grep -qF -- "$text"; } \
     && { [ -z "$nottext" ] || ! printf '%s\n' "$out" | grep -qF -- "$nottext"; }; then
    verdict=PASS; pass=$((pass + 1)); else verdict=FAIL; fail=$((fail + 1)); fi
  printf '%s %-66s %-3s exit %s (want %s)\n' "$verdict" "$name" "$sh" "$rc" "$want"
  printf '%s\n' "$out" | sed "s|$W|W|g" | { if [ "${SHOW:-}" = 1 ]; then cat; else tail -n 1; fi; } | sed 's/^/      | /'
  return 0
}

echo "== the walk alone, on the WB_TOOL value of the file and on odd values (bash -e; 2 s limit)"
walk() {
  local out rc; echo "-- $1"
  out=$(env -i PATH=/usr/bin:/bin WB_TOOL="$2" timeout 2 bash -e "$W/walk.sh" 2>&1); rc=$?
  printf '%s\n' "$out" | head -n 12; echo "    exit $rc"
}
walk "the file's value" /wasm-bindgen-0.2.130/wasm-bindgen
walk "a double slash at the start" //wasm-bindgen-0.2.130/wasm-bindgen
walk "a double slash inside" /a//b/wasm-bindgen
walk "a .. inside" /opt/../wasm-bindgen-0.2.130/wasm-bindgen
walk "a trailing slash" /wasm-bindgen-0.2.130/wasm-bindgen/
walk "a space" "/wasm bindgen/wasm-bindgen"
walk "a newline in the folder's name" $'/wasm\nbindgen/wasm-bindgen'
walk "the root alone" /
walk "a program directly under /" /wasm-bindgen
walk "relative: without the case line the walk never ends" wasm-bindgen-0.2.130/wasm-bindgen
echo "-- mkdir of the empty folder name that \"\${WB_TOOL%/*}\" gives for /x or /: refused, nothing made"
mkdir -m 0755 "" 2>&1 | sed 's/^/    /'

for sh in e pf; do
  echo "== shell $sh"
  c="$W/b1-clean-$sh"; mk "$c"; try "B1 control: clean run" "$sh" "$CUT" 0 ": OK"
  c="$W/b2-dslash-$sh"; mk "$c"; WB_TOOL="$c/root//wasm-bindgen-0.2.130/wasm-bindgen"
  SHOW=1 try "B2 a double slash inside WB_TOOL (root/ and root both judged)" "$sh" "$CUT" 0 ": OK"
  c="$W/b3-dotdot-$sh"; mk "$c"; mkdir -m 0755 "$c/side"; WB_TOOL="$c/side/../root/wasm-bindgen-0.2.130/wasm-bindgen"
  try "B3 a .. inside WB_TOOL, no link" "$sh" "$CUT" 0 ": OK"
  c="$W/b4-link-dotdot-$sh"; mk "$c"; mkdir -m 0755 "$c/other" "$c/other/deep" "$c/other/root"
  ln -s "$c/other/deep" "$c/lnk"; WB_TOOL="$c/lnk/../root/wasm-bindgen-0.2.130/wasm-bindgen"
  try "B4 a link, then .. (the link is judged as its own prefix)" "$sh" "$CUT" 1 "$MSG_JUDGE"
  c="$W/b5-trailing-$sh"; mk "$c"; WB_TOOL="$c/root/wasm-bindgen-0.2.130/"
  try "B5 a trailing slash" "$sh" "$CUT" 1 "$MSG_PLAIN"
  c="$W/b6-space-$sh"; mk "$c"; mkdir -m 0755 "$c/ro ot"; WB_TOOL="$c/ro ot/wasm-bindgen-0.2.130/wasm-bindgen"
  try "B6 a space in a folder name" "$sh" "$CUT" 0 ": OK"
  c="$W/b7-nl-prog-$sh"; mk "$c"; chmod 0775 "$c/root"; WB_TOOL="$c/root/wasm-bindgen-0.2.130/wasm"$'\n'"bindgen"
  SHOW=1 try "B7 newline in the program's name + a parent at 775: judge fooled" "$sh" "$CUT" 1 "FAILED open or read" "$MSG_JUDGE"
  c="$W/b7p-nl-prog-$sh"; mk "$c"; chmod 0775 "$c/root"; WB_TOOL="$c/root/wasm-bindgen-0.2.130/wasm"$'\n'"bindgen"
  try "B7p the same, judge with -printf (replacement): refused there" "$sh" "$W/cut-printf.sh" 1 "$MSG_JUDGE"
  c="$W/b7b-nl-folder-$sh"; mk "$c"; WB_TOOL="$c/root/wasm"$'\n'"bindgen/wasm-bindgen"
  try "B7b newline in the folder's name, all good: count too high" "$sh" "$CUT" 1 "$MSG_JUDGE"
  c="$W/b7c-nl-folder-$sh"; mk "$c"; WB_TOOL="$c/root/wasm"$'\n'"bindgen/wasm-bindgen"
  try "B7c the same with -printf: judge passes, sha256 line refuses" "$sh" "$W/cut-printf.sh" 1 "FAILED open or read" "$MSG_JUDGE"
  c="$W/b7d-clean-printf-$sh"; mk "$c"; try "B7d control: clean run with the -printf judge" "$sh" "$W/cut-printf.sh" 0 ": OK"
  c="$W/b8-fifo-$sh"; mk "$c"; mkdir -m 0755 "$c/root/wasm-bindgen-0.2.130"; mkfifo -m 0555 "$WB_TOOL"
  try "B8 a fifo where the program should be, the judge alone" "$sh" "$W/judge-only.sh" 1 "$MSG_JUDGE"
  c="$W/b9-setgid-$sh"; mk "$c"; chown root:1001 "$c/root"; chmod 2755 "$c/root"
  SHOW=1 try "B9 a setgid parent of group 1001 (chown takes the group back)" "$sh" "$CUT" 0 ": OK"
  c="$W/b10-sticky-$sh"; mk "$c"; chmod 1755 "$c/root"; try "B10 a sticky parent 1755" "$sh" "$CUT" 0 ": OK"
  c="$W/b10b-sticky-$sh"; mk "$c"; chmod 1777 "$c/root"; try "B10b a sticky parent 1777 (like /tmp)" "$sh" "$CUT" 1 "$MSG_JUDGE"
  c="$W/b11-setuid-$sh"; mk "$c"; mkdir -m 0755 "$c/root/wasm-bindgen-0.2.130"
  install -o root -g root -m 4555 "$c/dl/x/wasm-bindgen" "$WB_TOOL"
  SHOW=1 try "B11 the program setuid 4555: no write route, passes, stat shows it" "$sh" "$W/check-part.sh" 0 "mode 4555"
  c="$W/b12-chown-$sh"; mk "$c"; FAKE="$W/fake-chown" try "B12 sudo refuses chown" "$sh" "$CUT" 1 "$MSG_FRESH"
  [ -e "$WB_TOOL" ] && echo "      | the program WAS installed" || echo "      | the program was not installed"
  c="$W/b13-mkdir-$sh"; mk "$c"; FAKE="$W/fake-mkdir" try "B13 sudo refuses mkdir" "$sh" "$CUT" 1 "$MSG_FRESH"
  [ -e "${WB_TOOL%/*}" ] && echo "      | the folder WAS made" || echo "      | no folder was made"
  c="$W/b14-default-acl-$sh"; mk "$c"; acl set "$c/root" default "u::rwx,u:1001:rwx,g::r-x,m::rwx,o::r-x"
  try "B14 a default ACL giving uid 1001 rwx on the parent" "$sh" "$CUT" 0 ": OK"
  acl show "${WB_TOOL%/*}" access; acl show "$WB_TOOL" access
  c="$W/b15-access-acl-$sh"; mk "$c"; acl set "$c/root" access "u::rwx,u:1001:rwx,g::r-x,m::rwx,o::r-x"
  try "B15 an access ACL giving uid 1001 write on the parent" "$sh" "$CUT" 1 "$MSG_JUDGE"
  c="$W/b15b-masked-acl-$sh"; mk "$c"; acl set "$c/root" access "u::rwx,u:1001:rwx,g::r-x,m::r-x,o::r-x"
  try "B15b the same entry, mask r-x (no write in effect)" "$sh" "$CUT" 0 ": OK"
done

echo "== one entry at a time: today's judge (cb476b1:118, nothing printed = accept) against the new one (count = 1)"
M="$W/matrix"; mkdir -m 0755 "$M"
mkdir -m 0755 "$M/d755"; mkdir -m 0775 "$M/d775"; mkdir -m 0777 "$M/d777"; mkdir -m 0757 "$M/d757"
mkdir "$M/d1777" "$M/d1755" "$M/d2755"; chmod 1777 "$M/d1777"; chmod 1755 "$M/d1755"; chmod 2755 "$M/d2755"
mkdir -m 0755 "$M/d755-u1001" "$M/d755-g1001"; mkdir -m 0775 "$M/d775-g1001"
chown 1001:1001 "$M/d755-u1001"; chown root:1001 "$M/d755-g1001" "$M/d775-g1001"
install -m 0555 "$PROG_SRC" "$M/f555"; install -m 4555 "$PROG_SRC" "$M/f4555"; install -m 0575 "$PROG_SRC" "$M/f575"
install -o 1001 -g 1001 -m 0555 "$PROG_SRC" "$M/f555-u1001"
ln -s "$M/d755" "$M/link-to-d755"; ln -s "$M/f555" "$M/link-to-f555"; mkfifo -m 0644 "$M/fifo644"
mkdir -m 0755 "$M/acl-w" "$M/acl-masked"
acl set "$M/acl-w" access "u::rwx,u:1001:rwx,g::r-x,m::rwx,o::r-x" > /dev/null
acl set "$M/acl-masked" access "u::rwx,u:1001:rwx,g::r-x,m::r-x,o::r-x" > /dev/null
viol=0
for n in d755 d775 d777 d757 d1777 d1755 d2755 d755-u1001 d755-g1001 d775-g1001 f555 f4555 f575 f555-u1001 \
         link-to-d755 link-to-f555 fifo644 acl-w acl-masked missing; do
  p="$M/$n"
  if [ -z "$(find "$p" -maxdepth 0 \( ! -user root -o -perm /022 \) 2>/dev/null)" ]; then o=accept; else o=refuse; fi
  if [ "$(find "$p" -maxdepth 0 \( -type f -o -type d \) -user root ! -perm /022 2>/dev/null | wc -l)" -eq 1 ]; then n2=accept; else n2=refuse; fi
  [ "$n2" = accept ] && [ "$o" = refuse ] && viol=$((viol + 1))
  printf '  %-14s mode %-5s today %-6s new %-6s\n' "$n" "$(stat -c %a "$p" 2>/dev/null || echo -)" "$o" "$n2"
done
echo "  entries the new judge accepts and today's refuses: $viol"

echo "== find's own exit status inside the judge (pipefail on)"
set -o pipefail
set -- "$M/d755" "$M/missing"
cnt=$(find "$@" -maxdepth 0 \( -type f -o -type d \) -user root ! -perm /022 2>/dev/null | wc -l; exit "${PIPESTATUS[0]}"); st=$?
echo "  2 entries, one missing: count $cnt, find's status $st (not seen by the [ ]): $([ "$cnt" -eq "$#" ] && echo passes || echo refused)"
set +o pipefail
echo "== $pass PASS, $fail FAIL"

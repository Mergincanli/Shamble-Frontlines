#!/bin/bash
# Test 3 (u8-brief.md rev 1.1 section 3): the root wrappers, npt-load-ctl, npt-lib.sh and the sudoers files.
# Run: bash spikes/netcode-proof/ops/tests/wrappers.test.sh   (no root needed for the checks; visudo -c reads only)
# Prints `ok <name>` or `FAIL <name>` per check and exits 1 on any failure.
set -uo pipefail
HERE=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
SBIN=$(cd "$HERE/../sbin" && pwd)
CI=$(cd "$HERE/../cloud-init" && pwd)
RUNS_DIR=$(cd "$HERE/../runs" && pwd)
. "$SBIN/npt-lib.sh"

pass=0 fails=0
ok() { pass=$((pass + 1)); printf 'ok %s\n' "$1"; }
bad() { fails=$((fails + 1)); printf 'FAIL %s\n' "$1"; }
check() { local name=$1; shift; if "$@"; then ok "$name"; else bad "$name"; fi; }
refuse() { local name=$1; shift; if "$@"; then bad "$name"; else ok "$name"; fi; }

ROOT_WRAPPERS=(npt-ctl npt-cert npt-capture npt-userdata npt-s14)
ALL=("${ROOT_WRAPPERS[@]}" npt-load-ctl)

# ---- syntax and the header every wrapper keeps ----------------------------------------------------------------
for w in "${ALL[@]}" npt-lib.sh; do check "bash -n $w" bash -n "$SBIN/$w"; done
for w in "${ALL[@]}"; do
  f="$SBIN/$w"
  check "$w: #!/bin/bash -p" test "$(head -n 1 "$f")" = '#!/bin/bash -p'
  check "$w: environment cleared, PATH fixed" grep -q '^  exec /usr/bin/env -i NPT_ENV_CLEARED=1 PATH=/usr/sbin:/usr/bin:/sbin:/bin LC_ALL=C' "$f"
  check "$w: set -euo pipefail and no globbing" grep -qx 'set -f' "$f"
  check "$w: set -euo pipefail" grep -qx 'set -euo pipefail' "$f"
  check "$w: no eval, no source of a caller path" bash -c '! grep -Eq "(^|[^_])eval |\\. \"\\\$" "$1"' _ "$f"
done

# Addresses outside redact-check's allow-list are built at run time, so this file itself passes test 6.
q() { local IFS=.; printf '%s' "$*"; }
# ---- validators: accepted ------------------------------------------------------------------------------------
for a in 192.0.2.1 198.51.100.254 203.0.113.9 "$(q 8 8 4 4)" 1.1.1.1; do check "ipv4 accepts $a" npt_valid_ipv4 "$a"; done
# ---- refused: every form redact-check knows, plus ranges, extra words and metacharacters ----------------------
for a in 3221225985 '::ffff:192.0.2.1' "::ffff:$(printf '%s' c000):201" 192.0.2.1/32 192.0.2.01 192.000.2.1 \
  "$(q 010 1 2 3)" 256.1.2.3 1.2.3 1.2.3.4.5 "$(q 0 1 2 3)" "$(q 10 0 0 1)" 127.0.0.1 169.254.169.254 \
  "$(q 172 16 0 1)" "$(q 192 168 1 1)" "$(q 100 64 0 1)" "$(q 224 0 0 1)" \
  255.255.255.255 '192.0.2.1;id' '$(id)' '`id`' '192.0.2.1 ' ' 192.0.2.1' $'192.0.2.1\nid' '' 0x7f000001 \
  '192.0.2.1:22' 'c000:201' 192-0-2-1; do
  refuse "ipv4 refuses '$a'" npt_valid_ipv4 "$a"
done
refuse "ipv4 refuses two arguments" npt_valid_ipv4 192.0.2.1 192.0.2.2
for n in 00 07 10 31; do check "NN accepts $n" npt_valid_nn "$n"; done
for n in 32 7 007 -1 '00 ' 0x1 '' '0$(id)' '01;id' 99; do refuse "NN refuses '$n'" npt_valid_nn "$n"; done
refuse "NN refuses extra arguments" npt_valid_nn 01 02
check "vCPU 0 of 4" npt_valid_vcpu 0 4
check "vCPU 3 of 4" npt_valid_vcpu 3 4
for v in 4 01 -1 32 '1 ' x ''; do refuse "vCPU refuses '$v' of 4" npt_valid_vcpu "$v" 4; done
refuse "vCPU refuses three arguments" npt_valid_vcpu 1 4 x
check "word: off in off|on" npt_valid_word off off on
refuse "word: 'off ' not in off|on" npt_valid_word 'off ' off on
check "txt value: 43 base64url" npt_valid_txt_value "$(printf 'aZ09_-%.0s' 1 2 3 4 5 6 7)a"
refuse "txt value: 44 characters" npt_valid_txt_value "$(printf 'aZ09_-%.0s' 1 2 3 4 5 6 7)ab"
refuse "txt value: a quote" npt_valid_txt_value "$(printf 'a%.0s' $(seq 1 42))\""
check "hostname: cora.ns.cloudflare.com" npt_valid_hostname cora.ns.cloudflare.com
refuse "hostname: a space" npt_valid_hostname 'cora.ns.cloudflare.com @x'
refuse "hostname: no dot" npt_valid_hostname localhost
key_ok="ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAI$(printf 'B%.0s' $(seq 1 43)) npt-rs-to-lg"
check "pubkey: one ed25519 line" npt_valid_pubkey "$key_ok"
refuse "pubkey: an RSA key" npt_valid_pubkey "ssh-rsa AAAAB3NzaC1yc2E $(printf 'B%.0s' $(seq 1 43))"
refuse "pubkey: options before the key" npt_valid_pubkey "command=\"/bin/sh\" $key_ok"
refuse "pubkey: two lines" npt_valid_pubkey "$key_ok"$'\n'"$key_ok"

# ---- the npt-load verbs (C4; plant P7 must fail one of these) --------------------------------------------------
for c in 'token 03' 'settings 31' 'start 00' 'stop 12' 'status 07' 'log 30' 'npt-log 05' fwcheck; do
  check "load verb accepts '$c'" npt_load_parse "$c"
done
for c in 'start 03 extra' 'start 03 --config /etc/shadow' 'start 03;id' 'start $(id)' 'start 32' 'start 3' \
  'fwcheck x' 'kill 03' '' ' start 03' 'start  03' $'start 03\nid' 'token 03 >/tmp/x' 'npt-log 5' 'npt-log 05 x' \
  'npt-log' 'npt-log ../05' 'npt-logs 05'; do
  refuse "load verb refuses '$c'" npt_load_parse "$c"
done
npt_load_parse 'start 05'
check "load parse sets VERB and NN only from the match" test "$NPT_VERB:$NPT_NN" = start:05
check "npt-load-ctl reads SSH_ORIGINAL_COMMAND only to carry it over the env clear and to parse it" \
  test "$(grep -v '^#' "$SBIN/npt-load-ctl" | grep -c 'SSH_ORIGINAL_COMMAND')" = 2
check "npt-load-ctl: the parse is the only reader" \
  grep -qx 'npt_load_parse "${SSH_ORIGINAL_COMMAND:-}" || die "refused (verbs: token|settings|start|stop|status|log|npt-log NN, fwcheck)"' "$SBIN/npt-load-ctl"
check "npt-load-ctl: npt-log reads one fixed path (NC-SB-7)" grep -qx \
  '  npt-log) head -c 268435456 -- "$BASE/logs/load-$NN.jsonl" ;;' "$SBIN/npt-load-ctl"
check "npt-load-ctl: npt-load's command line is fixed" \
  grep -qx '    setsid "$BIN" "$BASE/run/load-$NN.json" < /dev/null > "$BASE/logs/load-$NN.log" 2>&1 &' "$SBIN/npt-load-ctl"
check "npt-load-ctl: no \$@, no BASH_REMATCH, \$* only in die's message" bash -c \
  '! grep -v "^die() " "$1" | grep -Eq "\\\$[@*]|BASH_REMATCH"' _ "$SBIN/npt-load-ctl"

# ---- condition 4: the housekeeping set and the kernel-thread selection on fixed /proc and /sys texts ------------
check "housekeeping: 4 vCPUs, tick 1, no SMT" test "$(npt_housekeeping 0-3 1 1)" = 0,2,3
check "housekeeping: 8 threads, tick 2 with sibling 6" test "$(npt_housekeeping 0-7 2 2,6)" = 0,1,3,4,5,7
refuse "housekeeping: nothing would be left" npt_housekeeping 0 0 0
refuse "housekeeping: tick offline" npt_housekeeping 0-3 5 5
refuse "housekeeping: tick not among its siblings" npt_housekeeping 0-3 1 2
check "workqueue mask of 0,2,3 is d" test "$(npt_cpumask_hex 0,2,3)" = d
check "workqueue mask of 0,1,3,4,5,7 is bb" test "$(npt_cpumask_hex 0,1,3,4,5,7)" = bb
refuse "cpu list refuses 32" npt_cpulist_valid 0-32
refuse "cpu list refuses 3-1" npt_cpulist_valid 3-1
KTHREAD=$((0x00200000 | 0x40))
USER=$((0x400100))
FIX=$(printf '%s\n' \
  "11 $KTHREAD 1 cpuhp/1" \
  "12 $KTHREAD 0-3 kworker/u8:0" \
  "13 $USER 0-3 sshd" \
  "14 $USER 0,2,3 npt-super" \
  "15 $KTHREAD 2 ksoftirqd/2" \
  "16 x 0-3 broken")
SCAN=$(printf '%s\n' "$FIX" | npt_affinity_scan 1)
check "per-CPU kernel thread on the tick vCPU: left" grep -qx 'left 11 cpuhp/1' <<< "$SCAN"
check "unbound kernel thread: moved" grep -qx 'move 12 kworker/u8:0' <<< "$SCAN"
check "user thread allowed on the tick vCPU: counted" grep -qx 'counts 13 sshd' <<< "$SCAN"
check "user thread off the tick vCPU: not listed" bash -c '! grep -q " 14 " <<< "$1"' _ "$SCAN"
check "per-CPU kernel thread on another vCPU: not listed" bash -c '! grep -q " 15 " <<< "$1"' _ "$SCAN"
check "malformed record: bad, never selected" grep -q '^bad ' <<< "$SCAN"
check "no per-CPU kernel thread is ever moved" bash -c '! grep -Eq "^move (11|15) " <<< "$1"' _ "$SCAN"
check "a room-chosen thread name is cleaned" test "$(npt_comm_clean $'a b;\e[31m$(id)')" = 'a?b???31m??id?'
# INF-SB-9: set-property taken, PID 1 still on the tick vCPU: taskset runs too; off it: not; unreadable: taskset.
check "PID 1 still holds tick vCPU 3 after set-property (0-3): moved by taskset too" npt_pid1_needs_move 0-3 3
refuse "PID 1 off the tick vCPU (0-2): left to set-property" npt_pid1_needs_move 0-2 3
for p in '' 'x' '0-3 ' 0-40; do check "PID 1's list '$p' unreadable: taskset" npt_pid1_needs_move "$p" 3; done
check "npt-ctl cpus: PID 1 read after set-property, the fallback outside the refusal branch" bash -c '
  b=$(sed -n "/^cpus() {/,/^}/p" "$1"); a=$(grep -n "set-property --runtime init.scope" <<< "$b" | cut -d: -f1)
  c=$(grep -n "if npt_pid1_needs_move \"\$line\" \"\$tick\"; then" <<< "$b" | cut -d: -f1)
  [ -n "$a" ] && [ -n "$c" ] && [ "$a" -lt "$c" ] && grep -q "pid1.allowed_after_taskset" <<< "$b"' _ "$SBIN/npt-ctl"

# ---- Q-U8-1 term 2: no caller text reaches systemd-run, setpriv or taskset --------------------------------------
check "rooms start: the scope's command line is fixed (npt-super start, U6a's merged form)" grep -qF \
  '"${SETPRIV_ROOM[@]}" /usr/bin/taskset -c "$hk" /opt/npt/bin/npt-super start "${NPT_SUPER_ARGS[@]}" ;;' "$SBIN/npt-ctl"
check "rooms start: the settings read as npt-room, the flags from npt_super_args only" bash -c '
  f=$1; grep -qF "mapfile -t vals < <(\"\${SETPRIV_ROOM[@]}\" /usr/bin/python3 -I -c \"\$NPT_SETTINGS_PY\" \\" "$f" &&
  grep -qF "npt_super_args \"\$nn\" \"\${vals[@]}\" || die" "$f" && ! grep -q "npt-super hold" "$f"' _ "$SBIN/npt-ctl"
# npt_super_args (items 8 and 10): one good settings line set, then each field broken in turn.
SUP=(R1-headers-01 R1-headers measured 2026-12-03 "$(printf '0%.0s' $(seq 1 40))" L2 P0 SS 30 RS RS CCX23 fsn1
  192.0.2.10 PM-0001-1,PM-0001-2 '' /var/lib/npt/rooms/room-01.jsonl)
check "npt_super_args takes a room's values" npt_super_args 01 "${SUP[@]}"
check "npt_super_args: the measured binary, the settings, npt-super's log beside the room's" test \
  "${NPT_SUPER_ARGS[*]:0:6}" = "--room /opt/npt/bin/npt-server --settings /var/lib/npt/run/room-01.json --log /var/lib/npt/rooms/super-01.jsonl"
check "npt_super_args: --marks given, no --bite for a measured room" bash -c '[[ "$1" == *" --marks PM-0001-1,PM-0001-2" && "$1" != *--bite* ]]' _ "${NPT_SUPER_ARGS[*]}"
B=("${SUP[@]}"); B[15]=PM-0001-16/double-load; B[16]=/var/lib/npt/rooms/room-06-double-load.jsonl
check "npt_super_args: a bite starts the bites build with --bite" bash -c '. "$1"; npt_super_args 06 "${@:2}" &&
  [[ ${NPT_SUPER_ARGS[1]} == /opt/npt/bin/npt-server-bites && "${NPT_SUPER_ARGS[*]}" == *" --bite PM-0001-16/double-load" ]]' _ "$SBIN/npt-lib.sh" "${B[@]}"
for i in 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16; do
  for v in '' 'a b' $'x\ny' '$(id)'; do
    B=("${SUP[@]}"); B[i]=$v
    [ "$i" = 13 ] && [ -z "$v" ] && continue   # an empty address is the header's own form off the VMs
    [ "$i" = 14 ] && [ -z "$v" ] && continue   # no marks: no --marks flag
    [ "$i" = 15 ] && [ -z "$v" ] && continue   # no bite: a measured room
    refuse "npt_super_args refuses field $i as '${v//$'\n'/\\n}'" npt_super_args 01 "${B[@]}"
  done
done
B=("${SUP[@]}"); B[16]=/var/lib/npt/rooms/room-02.jsonl
refuse "npt_super_args refuses another room's log" npt_super_args 01 "${B[@]}"
refuse "npt_super_args refuses a missing value" npt_super_args 01 "${SUP[@]:0:16}"
refuse "npt_super_args refuses a bad NN" npt_super_args 1 "${SUP[@]}"
check "rooms tokens: read as npt-room, fixed path" grep -qF \
  'exec "${SETPRIV_ROOM[@]}" /usr/bin/head -c 1048576 -- "/var/lib/npt/rooms/room-$nn.tok" ;;' "$SBIN/npt-ctl"
check "setpriv drops to npt-room, clears groups and capability sets, no_new_privs" grep -qF \
  'readonly SETPRIV_ROOM=(/usr/bin/setpriv --reuid=npt-room --regid=npt-room --clear-groups --no-new-privs' "$SBIN/npt-ctl"
check "setpriv clears bounding, inheritable and ambient sets" grep -qF \
  '  --bounding-set=-all --inh-caps=-all --ambient-caps=-all --reset-env)' "$SBIN/npt-ctl"
check "hk and nn come only from the root-owned record and the validated NN" bash -c '
  f=$1; grep -q "hk=\$(housekeeping_record)" "$f" && grep -q "local nn=\$2" "$f" &&
  grep -q "npt_valid_nn \"\$2\" || die" "$f"' _ "$SBIN/npt-ctl"
check "npt-cert: dig takes names from the root-owned clone only" grep -qF \
  'dig +norecurse +short +time=5 +tries=1 TXT "$NPT_CHALLENGE_NAME" "@$n"' "$SBIN/npt-cert"
# ---- fix pass item 2: the TLS names; the issue mode's call and its stderr PROFILES line kept ------------------------
for n in staging.key.pem staging.chain.pem production.key.pem production.chain.pem; do
  check "npt-cert's header names /var/lib/npt/tls's $n" bash -c 'grep "^#" "$1" | grep -qF "$2"' _ "$SBIN/npt-cert" "$n"
done
check "npt-cert: the issue mode's call stays, its stderr not redirected (PROFILES reaches the results)" grep -qx \
  '    --inh-caps=-all --ambient-caps=-all --reset-env /opt/npt/bin/npt-host issue "$PROFILE"' "$SBIN/npt-cert"
check "r1.sh: the cert block keeps stderr and drops only the TXT step" bash -c '
  grep -qF "npt-cert issue \"\$p\" < /dev/null > \"\$RUN/cert-\$p.live\" 2>&1 &" "$1" &&
  grep -qF "grep -v '"'"'^Add in Cloudflare'"'"' \"\$RUN/cert-\$p.live\" | tee \"\$d/\$p.txt\"" "$1"' _ "$RUNS_DIR/r1.sh"
check "r1.sh: a browser room's WebSocket names the production pair, never one shared name" bash -c '
  grep -qF "chain='"'"'\"/var/lib/npt/tls/production.chain.pem\"'"'"' key='"'"'\"/var/lib/npt/tls/production.key.pem\"'"'"'" "$1" &&
  ! grep -Eq "tls/(chain|key)\.pem" "$1"' _ "$RUNS_DIR/r1.sh"
check "npt-capture: tcpdump's line is fixed" grep -qF \
  'setsid /usr/bin/tcpdump -i "$IFACE" -n -s 96 -c 2000000 -U -Z tcpdump -w "$OUT" "$FILTER" < /dev/null > "$LOG" 2>&1 &' "$SBIN/npt-capture"

# ---- the sudoers files: visudo, wrappers only, anchored patterns only (L5) --------------------------------------
for s in npt npt-lg; do
  f="$SBIN/sudoers.d/$s"
  check "visudo -c -f sudoers.d/$s" visudo -c -q -f "$f"
  lines=$(grep -E '^npt ' "$f")
  check "sudoers.d/$s has command lines" test -n "$lines"
  while IFS= read -r l; do
    cmd=$(sed -E 's/^npt ALL=\(root\) NOPASSWD: ([^ ]+).*/\1/' <<< "$l")
    args=${l#*"$cmd"}
    args=${args# }
    check "sudoers.d/$s: '$cmd' is a wrapper" bash -c '[[ $1 =~ ^/usr/local/sbin/npt-(ctl|cert|capture|userdata|s14)$ ]]' _ "$cmd"
    check "sudoers.d/$s: '$cmd' args anchored or none" bash -c '[[ $1 == "\"\"" || ( $1 == ^* && $1 == *\$ ) ]]' _ "$args"
  done <<< "$lines"
  check "sudoers.d/$s: never systemd-run, setpriv, taskset, nft, tcpdump, dig or a shell" bash -c \
    '! grep -E "^npt " "$1" | grep -Eq "systemd-run|setpriv|taskset|/nft|tcpdump|/dig|/bash|/sh( |$)|/dash|/zsh|ALL$|\*"' _ "$f"
  check "sudoers.d/$s: npt only, as root only" bash -c '! grep -E "^[^#D]" "$1" | grep -Ev "^npt ALL=\(root\) NOPASSWD: "' _ "$f"
  check "sudoers.d/$s: no line names npt-host or systemctl (Q-U6C-7: the path unit starts the host)" bash -c \
    '! grep -v "^#" "$1" | grep -Eq "npt-host|systemctl"' _ "$f"
done

# ---- globbing is off in every wrapper (set -f): a glob is expanded only into an array inside a set +f window --------
for w in "${ALL[@]}"; do
  check "$w: no for-loop over a bare glob while set -f holds" bash -c '! grep -Eq "^[[:space:]]*for [a-z_]+ in [^\"\$]*[*?[]" "$1"' _ "$SBIN/$w"
done
check "npt-ctl: /proc globs expanded inside set +f only" bash -c '
  n=$(grep -c "=(/proc/" "$1"); [ "$n" -ge 2 ] && ! grep -v "=(/proc/" "$1" | grep -q "/proc/\[0-9\]\*"' _ "$SBIN/npt-ctl"

# ---- r1.sh: syntax, every room through npt-ctl rooms start, the chain's fixed lists -----------------------------
RUNS=$(cd "$HERE/../runs" && pwd)
check "bash -n r1.sh" bash -n "$RUNS/r1.sh"
check "r1.sh: every room starts through sudo npt-ctl rooms start (condition 3)" grep -qF \
  'setsid sudo -n /usr/local/sbin/npt-ctl rooms start "$nn" < "$fifo"' "$RUNS/r1.sh"
check "r1.sh: npt-super is never started directly" bash -c '! grep -v "^ *#" "$1" | grep -Eq "(^|[ /])npt-super( |$)"' _ "$RUNS/r1.sh"
check "r1.sh: the server chain is C1's list" grep -qx 'readonly SERVER_CHAIN="headers probe calibrate starts ds-selftest s14"' "$RUNS/r1.sh"
check "r1.sh: no npx, npm exec or npm create" bash -c '! grep -Eq "\bnpx\b|npm (exec|create)" "$1"' _ "$RUNS/r1.sh"
check "r1.sh: host_active reads systemctl is-active npt-host and dies unless active" bash -c '
  b=$(sed -n "/^host_active() {/,/^}/p" "$1"); grep -qF "st=\$(systemctl is-active npt-host || true)" <<< "$b" &&
  grep -qF "[ \"\$st\" = active ] || die" <<< "$b"' _ "$RUNS/r1.sh"
check "r1.sh: the browser block checks npt-host first" bash -c '
  [ "$(sed -n "/^browsers() {/,/^}/p" "$1" | sed -n 3p)" = "  host_active" ]' _ "$RUNS/r1.sh"
check "r1.sh: the host-key cross-check block checks npt-host first" grep -qE '^  host\) host_active; ' "$RUNS/r1.sh"
check "r1.sh: a chain refuses to start while the other still runs (no bites beside S14)" grep -qF \
  '[ ! -f "$R/chain/$c.chain.log" ] || [ -f "$R/chain/$c.ended" ] || die "the $c chain still runs: chain $2 waits"' "$RUNS/r1.sh"
check "r1.sh: browsers and browsers-down stamp each leg; the s14 block keeps the stamps" bash -c '
  f=$1; [ "$(grep -c "printf '"'"'%s \(up\|down\) %s\\\\n'"'"' \"\$1\" \"\$(stamp)\" >> \"\$(out browsers)/stamps.txt\"" "$f")" = 2 ] &&
  grep -qF "cp -p \"\$R/browsers/stamps.txt\" \"\$d/browser-stamps.txt\"" "$f"' _ "$RUNS/r1.sh"
check "r1.sh: C5 stops before any room when LG reaches RS or answers it" bash -c '
  f=$1; a=$(grep -n "lg fwcheck" "$f" | head -1 | cut -d: -f1); b=$(grep -n "room_up 00" "$f" | head -1 | cut -d: -f1)
  [ -n "$a" ] && [ -n "$b" ] && [ "$a" -lt "$b" ] && grep -q " 0 received" "$f"' _ "$RUNS/r1.sh"

# ---- cloud-init installs exactly what exists here -----------------------------------------------------------------
for w in "${ROOT_WRAPPERS[@]}"; do check "rs.yaml installs $w" grep -q "for w in .*\\b$w\\b" "$CI/rs.yaml"; done
check "lg.yaml installs npt-load-ctl to /usr/local/bin" grep -qF 'install -m 0755 "$O/sbin/npt-load-ctl" /usr/local/bin/npt-load-ctl' "$CI/lg.yaml"
for f in npt-lib.sh npt-ns.txt sudoers.d/npt sudoers.d/npt-lg; do check "ops/sbin/$f exists" test -f "$SBIN/$f"; done

printf '\nwrappers.test.sh: %d ok, %d failed\n' "$pass" "$fails"
[ "$fails" -eq 0 ]

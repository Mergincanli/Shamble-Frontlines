# npt-lib.sh: validators and pure computations for U8's wrappers (u8-brief.md rev 1.1 section 1 Block 3).
# No side effects: sourcing defines constants and functions only, reads no file and runs no command, so the container
# tests (ops/tests/wrappers.test.sh) source it as the wrappers do. Installed root-owned, 0644, at
# /usr/local/lib/npt/npt-lib.sh. Every validator takes exactly one argument and refuses anything else.

readonly NPT_OCTET='(25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9]?[0-9])'
readonly NPT_PF_KTHREAD=$((0x00200000))
readonly NPT_MAX_CPUS=32
# Q-U8-3: the QUIC block. One interval element with a 15-minute timeout, so the block cannot stay on.
readonly NPT_QUIC_OFF_RULE='add element inet npt quic_off { 30001-30032 timeout 15m }'
readonly NPT_QUIC_ON_RULE='flush set inet npt quic_off'
readonly NPT_DNS_NAME='rs.nt.shamblefrontlines.com'
readonly NPT_CHALLENGE_NAME='_acme-challenge.rs.nt.shamblefrontlines.com'

# A public IPv4 in plain dotted form: no leading zero, no CIDR, no mapped or decimal form, no special range
# (unspecified, private, shared, loopback, link-local, multicast, reserved, broadcast).
npt_valid_ipv4() {
  [ "$#" -eq 1 ] || return 1
  local re="^${NPT_OCTET}\\.${NPT_OCTET}\\.${NPT_OCTET}\\.${NPT_OCTET}\$"
  [[ $1 =~ $re ]] || return 1
  local a=${BASH_REMATCH[1]} b=${BASH_REMATCH[2]}
  ((a == 0 || a == 10 || a == 127 || a >= 224)) && return 1
  ((a == 100 && b >= 64 && b <= 127)) && return 1
  ((a == 169 && b == 254)) && return 1
  ((a == 172 && b >= 16 && b <= 31)) && return 1
  ((a == 192 && b == 168)) && return 1
  return 0
}

# A room slot NN, 00-31 (two digits; settings path /var/lib/npt/run/room-NN.json).
npt_valid_nn() {
  [ "$#" -eq 1 ] && [[ $1 =~ ^(0[0-9]|[12][0-9]|3[01])$ ]]
}

# A vCPU index below the online count (two arguments: the index, then the count the wrapper read).
npt_valid_vcpu() {
  [ "$#" -eq 2 ] && [[ $1 =~ ^([0-9]|[12][0-9]|3[01])$ ]] && [[ $2 =~ ^[1-9][0-9]?$ ]] && ((10#$1 < 10#$2))
}

# One word from a fixed list: npt_valid_word <word> <allowed>...
npt_valid_word() {
  [ "$#" -ge 2 ] || return 1
  local w=$1 x
  shift
  for x in "$@"; do [ "$w" = "$x" ] && return 0; done
  return 1
}

# A kernel cpu list ("0-3,8"), every CPU below NPT_MAX_CPUS.
npt_cpulist_valid() {
  [ "$#" -eq 1 ] && [[ $1 =~ ^[0-9]{1,2}(-[0-9]{1,2})?(,[0-9]{1,2}(-[0-9]{1,2})?)*$ ]] || return 1
  local part lo hi
  local IFS=,
  for part in $1; do
    lo=${part%-*} hi=${part#*-}
    ((10#$lo <= 10#$hi && 10#$hi < NPT_MAX_CPUS)) || return 1
  done
}

# Expands a valid cpu list to its CPUs, ascending, space-separated.
npt_cpulist_expand() {
  npt_cpulist_valid "$1" || return 1
  local part lo hi c out=() seen=()
  local IFS=,
  for part in $1; do
    lo=$((10#${part%-*})) hi=$((10#${part#*-}))
    for ((c = lo; c <= hi; c++)); do seen[c]=1; done
  done
  for c in "${!seen[@]}"; do out+=("$c"); done
  IFS=' '
  printf '%s\n' "${out[*]}"
}

# True if a valid cpu list holds the CPU.
npt_cpulist_has() {
  local c
  for c in $(npt_cpulist_expand "$1"); do [ "$c" = "$2" ] && return 0; done
  return 1
}

# The number of CPUs in a valid cpu list.
npt_cpulist_count() {
  local n=0 c
  for c in $(npt_cpulist_expand "$1"); do n=$((n + 1)); done
  printf '%s\n' "$n"
}

# Condition 4's housekeeping set: every online CPU but the tick vCPU and its SMT siblings.
# npt_housekeeping <online list> <tick vCPU> <the tick's thread_siblings_list>; prints a comma list, or fails when the
# tick is offline, not among its own siblings, or nothing would be left.
npt_housekeeping() {
  [ "$#" -eq 3 ] || return 1
  npt_cpulist_valid "$1" && npt_cpulist_valid "$3" || return 1
  npt_cpulist_has "$1" "$2" && npt_cpulist_has "$3" "$2" || return 1
  local c out=()
  for c in $(npt_cpulist_expand "$1"); do
    npt_cpulist_has "$3" "$c" || out+=("$c")
  done
  [ "${#out[@]}" -gt 0 ] || return 1
  local IFS=,
  printf '%s\n' "${out[*]}"
}

# INF-SB-9: PID 1's threads still need taskset when its Cpus_allowed_list (read after set-property) holds the tick
# vCPU or does not read as a cpu list. npt_pid1_needs_move <allowed list> <tick vCPU>.
npt_pid1_needs_move() {
  [ "$#" -eq 2 ] || return 0
  ! npt_cpulist_valid "$1" || npt_cpulist_has "$1" "$2"
}

# The unbound workqueue mask (/sys/devices/virtual/workqueue/cpumask) for a valid cpu list, in hex.
npt_cpumask_hex() {
  npt_cpulist_valid "$1" || return 1
  local m=0 c
  for c in $(npt_cpulist_expand "$1"); do m=$((m | (1 << c))); done
  printf '%x\n' "$m"
}

# A thread name as the records print it: at most 32 characters of [A-Za-z0-9_.:/-], every other character `?`
# (a room can name its threads, so a name never reaches a record raw).
npt_comm_clean() {
  local s=${1:0:32}
  printf '%s\n' "${s//[^A-Za-z0-9_.:\/-]/?}"
}

# Condition 4's selection, on records read from /proc by `npt-ctl cpus` (one per thread, on stdin):
#   <tid> <flags from /proc/<pid>/task/<tid>/stat> <Cpus_allowed_list> <clean name>
# Prints, for the tick vCPU given:
#   move <tid> <name>    a kernel thread (PF_KTHREAD) allowed on more than one CPU: moved to the housekeeping set
#   left <tid> <name>    a kernel thread bound to the tick vCPU only: per-CPU, left, and excluded from `alone`
#   counts <tid> <name>  any other thread whose allowed list holds the tick vCPU: it counts against `alone`
# A malformed record is printed as `bad <line>` and never selected.
npt_affinity_scan() {
  [ "$#" -eq 1 ] || return 1
  local tick=$1 tid flags allowed name n
  while read -r tid flags allowed name; do
    if ! [[ $tid =~ ^[0-9]{1,9}$ && $flags =~ ^[0-9]{1,12}$ ]] || ! npt_cpulist_valid "$allowed"; then
      printf 'bad %s\n' "$(npt_comm_clean "$tid $flags $allowed")"
      continue
    fi
    n=$(npt_cpulist_count "$allowed")
    if ((10#$flags & NPT_PF_KTHREAD)); then
      if ((n > 1)); then
        printf 'move %s %s\n' "$tid" "$name"
      elif [ "$allowed" = "$tick" ]; then
        printf 'left %s %s\n' "$tid" "$name"
      fi
    elif npt_cpulist_has "$allowed" "$tick"; then
      printf 'counts %s %s\n' "$tid" "$name"
    fi
  done
}

# npt-load-ctl's fixed verbs (C4), parsed from SSH_ORIGINAL_COMMAND: `<verb> NN` or `fwcheck`. Sets NPT_VERB and
# NPT_NN; refuses every other text, extra words included.
npt_load_parse() {
  NPT_VERB='' NPT_NN=''
  [ "$#" -eq 1 ] || return 1
  if [[ $1 =~ ^(token|settings|start|stop|status|log|npt-log)\ (0[0-9]|[12][0-9]|3[01])$ ]]; then
    NPT_VERB=${BASH_REMATCH[1]} NPT_NN=${BASH_REMATCH[2]}
    return 0
  fi
  if [ "$1" = fwcheck ]; then
    NPT_VERB=fwcheck
    return 0
  fi
  return 1
}

# The RS-to-LG public key, one line: ed25519 only (a 51-byte blob is 68 base64 characters), an optional plain comment.
npt_valid_pubkey() {
  [ "$#" -eq 1 ] && [[ $1 =~ ^ssh-ed25519\ AAAAC3NzaC1lZDI1NTE5AAAAI[A-Za-z0-9+/]{43}(\ [A-Za-z0-9@._-]{1,64})?$ ]]
}

# The DNS-01 TXT value instant-acme gives (KeyAuthorization::dns_value: base64url of a SHA-256, 43 characters).
npt_valid_txt_value() {
  [ "$#" -eq 1 ] && [[ $1 =~ ^[A-Za-z0-9_-]{43}$ ]]
}

# A DNS host name in lower case (the zone's authoritative servers, read from the root-owned clone).
npt_valid_hostname() {
  [ "$#" -eq 1 ] && [[ $1 =~ ^[a-z0-9]([a-z0-9-]{0,61}[a-z0-9])?(\.[a-z0-9]([a-z0-9-]{0,61}[a-z0-9])?)+$ ]]
}

# A certificate serial or a SHA-256 in hex, as the issue mode prints them.
npt_valid_hex() {
  [ "$#" -eq 1 ] && [[ $1 =~ ^[0-9a-f]{2,128}$ ]]
}

# npt-super's `start` flags from one room's settings (fix pass items 8 and 10; U6a's merged npt-super,
# crates/super/src/main.rs:54-139). `npt-ctl rooms start` reads the settings file as npt-room with NPT_SETTINGS_PY,
# which prints one value per line in NPT_SUPER_FIELDS' order (marks joined by commas, a null bite empty), and passes
# them here: each is checked against an anchored pattern, so npt-super's log takes the room's run and the same eight
# header values (set, role, layer, profile, date, scene, tick_hz, bite) from that one file; the judge makes a run
# INVALID when its logs differ on any of them (NR-U6A-1). Sets NPT_SUPER_ARGS; the room binary is the bites build only
# for a bite, and npt-super's log is the room log's name with `super-` for `room-`.
readonly NPT_SUPER_FIELDS='run set role date commit layer profile scene tick_hz machine_role machine_name machine_plan machine_location machine_address marks bite log'
readonly NPT_SETTINGS_PY='import json, sys
raw = open(sys.argv[1], "rb").read(65537)
s = json.loads(raw) if len(raw) <= 65536 else sys.exit(2)
out = []
for k in sys.argv[2].split():
    v = s.get(k)
    if k == "marks" and isinstance(v, list) and all(isinstance(m, str) for m in v):
        v = ",".join(v)
    elif k == "bite" and v is None:
        v = ""
    if isinstance(v, bool) or not isinstance(v, (str, int)) or "\n" in str(v) or "\r" in str(v):
        sys.exit(2)
    out.append(str(v))
sys.stdout.write("".join(x + "\n" for x in out))'
npt_super_args() {
  NPT_SUPER_ARGS=()
  [ "$#" -eq 18 ] && npt_valid_nn "$1" || return 1
  local nn=$1 id='^[A-Za-z0-9][A-Za-z0-9._-]{0,63}$' w='^[A-Za-z0-9-]{1,16}$' mark='PM-[0-9]{4}-[0-9]{1,3}'
  [[ $2 =~ $id && $3 =~ $id && $4 =~ ^[a-z][a-z-]{0,31}$ && $5 =~ ^[0-9]{4}-[0-9]{2}-[0-9]{2}$ ]] || return 1
  [[ $6 =~ ^[0-9a-f]{40}$ && $7 =~ ^(L1|L2|real)$ && $8 =~ ^(P[0-4]|B1|real-path)$ && $9 =~ ^(SS|ES|CO|TC)$ ]] || return 1
  [[ ${10} =~ ^[1-9][0-9]{0,2}$ && ${11} =~ ^[A-Z]{1,8}$ && ${12} =~ $w && ${13} =~ $w && ${14} =~ ^[a-z0-9-]{1,16}$ ]] || return 1
  [ -z "${15}" ] || npt_valid_ipv4 "${15}" || return 1
  [[ ${16} =~ ^($mark(,$mark){0,15})?$ && ${17} =~ ^($mark/[a-z0-9-]{1,32})?$ ]] || return 1
  [[ ${18} =~ ^/var/lib/npt/rooms/room-$nn(-[a-z0-9-]{1,32})?\.jsonl$ ]] || return 1
  local bin=/opt/npt/bin/npt-server
  [ -z "${17}" ] || bin=/opt/npt/bin/npt-server-bites
  NPT_SUPER_ARGS=(--room "$bin" --settings "/var/lib/npt/run/room-$nn.json" --log "${18/\/room-//super-}"
    --run "$2" --set "$3" --role "$4" --date "$5" --commit "$6" --layer "$7" --profile "$8" --scene "$9"
    --tick-hz "${10}" --machine-role "${11}" --machine-name "${12}" --machine-plan "${13}"
    --machine-location "${14}" --machine-address "${15}")
  [ -z "${16}" ] || NPT_SUPER_ARGS+=(--marks "${16}")
  [ -z "${17}" ] || NPT_SUPER_ARGS+=(--bite "${17}")
}

# The metrics_src element command for validated LG addresses (npt-ctl peers).
npt_metrics_rule() {
  [ "$#" -ge 1 ] && [ "$#" -le 4 ] || return 1
  local a
  for a in "$@"; do npt_valid_ipv4 "$a" || return 1; done
  local IFS=,
  local list="$*"
  printf 'add element inet npt metrics_src { %s }\n' "${list//,/, }"
}

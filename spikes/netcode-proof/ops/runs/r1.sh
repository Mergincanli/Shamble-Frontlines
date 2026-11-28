#!/bin/bash
# r1.sh <block> [<argument>]: RS's run script for R1 (u8-brief.md rev 1.1 section 1 Block 5; C1, C3, C5; conditions
# 3 and 4; Q-U8-5). It runs as npt on RS from the root-owned clone, one fixed block per call:
#   bash /opt/npt/clone/spikes/netcode-proof/ops/runs/r1.sh <block>
# Every result goes under /var/lib/npt/results/r1/<block>/. Every room starts through `sudo npt-ctl rooms start NN`
# (`npt-super start`, its hold form, condition 3), its stdin held by a holder process this script owns and closed to
# end it: npt-super ends by its stdin closing, never by a kill (a log with no footer makes the run INVALID).
# C1: Manual mode prompts every laptop command and no ssh allow exists, so while Mustafa is away the laptop runs
# nothing: `chain server` starts the server blocks on RS detached from the SSH session (they survive the disconnect,
# hold each room's stdin, write a done or stopped marker per block); `chain-status` reads them when he is back.
# Blocks:
#   bringup                  C-U8-1's facts and THP, taskset, netem, the login checks, C5's firewall proof, npt-ctl
#                            cpus, then one npt-super start whose `alone` must read 1 (condition 4)
#   cert-staging|cert-production            starts the order and prints the TXT step (him present)
#   cert-staging-wait|cert-production-wait  waits for the order's end and records the ledger line
#   headers probe calibrate starts ds-selftest s14 bites   the server blocks (normally through a chain)
#   host                     the HTTPS host-key cross-check's RS half: stops unless npt-host is active (Q-U6C-7)
#   dist-accept              his word on a dist that differs from the expected list (SEC-SB-5), recorded
#   lg-boot <epoch s>        LG's creation, for LG's delete-by in the chain's deadline (INF-SB-12)
#   browsers <room> | browsers-down <room>  a browser leg's room up or down (rooms: R1.md's slot table)
#   tokens NN                a room's token file to stdout (the laptop's browser scripts; never a log)
#   collect                  npt-judge's summaries and the raw tarball
#   chain server|bites | chain-status
# Interfaces still to align against the merged code (each in one home here): U6b's and U6c's room settings fields
# (room_settings; U6a's are its merged struct's), npt-load's settings (load_settings: U6c's C1 (c)) and the probe's
# records (probe_presence: Q-U6C-5's names). npt-super's flags are U6a's merged `start` (npt-lib.sh npt_super_args).
set -euo pipefail
umask 022
export PATH=/usr/sbin:/usr/bin:/sbin:/bin LC_ALL=C
readonly SELF=/opt/npt/clone/spikes/netcode-proof/ops/runs/r1.sh
readonly SPIKE=/opt/npt/clone/spikes/netcode-proof
readonly R=/var/lib/npt/results/r1 RUN=/var/lib/npt/run ROOMS=/var/lib/npt/rooms BIN=/opt/npt/bin HASH=/var/lib/npt/hash
readonly B=/var/lib/npt/bringup STATE=/var/lib/npt/state EXPECTED_DIST=$SPIKE/ops/runs/r1-dist.sha256
readonly TICK=3 TICK_HZ=30 LOCATION=fsn1 # LOCATION: a Stop B fill, fsn1 or nbg1 (INF-SB-3)
readonly SERVER_CHAIN="headers probe calibrate starts ds-selftest s14"
readonly BITES_CHAIN="bites"
# The blocks C1 skips under alone 0: those whose judged work runs on the tick vCPU (NC-R2-5, advisor-u8-r2.md section 2).
readonly TIMING_BLOCKS="headers ds-selftest bites"
# The probe (U6c's probe mode in room 02, UDP 30003: advisor-u6c-brief.md C1 (b)): six legs, each its warm-up, its
# window (1,000,000 a direction at 10,000 a second: 100 s), and about 12 s of tail and set-up (the profile's longest
# one-way delay plus 1 s, then the next leg's bind): 6 x 122 = 732 s planned. LG sends for that plus 60 s; the block
# waits for the probe's own exit, in npt-super's log, up to that plus 120 s, then for LG's own end (NC-R2-1).
readonly PROBE_LEGS="P0 P1 P2 P3 P4 B1" PROBE_PACKETS=1000000 PROBE_WARMUP_S=10 PROBE_RATE=10000 PROBE_LEG_EXTRA_S=12
readonly PROBE_TOTAL_S=$((6 * (PROBE_WARMUP_S + PROBE_PACKETS / PROBE_RATE + PROBE_LEG_EXTRA_S)))
# S14's three 950 s runs with set-up (R1.md section 1's 0.80 h), and the bites' start after RS's boot (12:57 - 09:30).
readonly S14_S=2880 BITES_AT_S=12420
# Calibrate's LG run; its room is held and outlives it (NC-R2-3).
readonly CALIBRATE_S=300
die() { printf 'r1.sh: %s\n' "$*" >&2; exit 2; }
[ "$(id -un)" = npt ] || die "run as npt"
. /usr/local/lib/npt/npt-lib.sh

# ---- small helpers ---------------------------------------------------------------------------------------------
out() { mkdir -p "$R/$1"; printf '%s\n' "$R/$1"; }
readf() { cat "$1" 2> /dev/null || echo unreadable; }
stamp() { date -u +%Y-%m-%dT%H:%M:%SZ; }
stat_cpu() { { printf 'utc %s\nuptime %s\n' "$(stamp)" "$(cat /proc/uptime)"; grep '^cpu' /proc/stat; } > "$1"; }
rs_ip() {
  local a
  a=$(ip -4 -o addr show dev eth0 scope global | awk '{print $4}' | cut -d/ -f1 | head -n 1)
  npt_valid_ipv4 "$a" || die "RS's own IPv4 is unreadable"
  printf '%s\n' "$a"
}
lg_ip() {
  local a
  a=$(sed -n 's/^lg=\([^ ]*\).*/\1/p' "$STATE/peers" 2> /dev/null)
  npt_valid_ipv4 "$a" || die "no LG address: run npt-ctl peers first"
  printf '%s\n' "$a"
}
# lg <verb>: one npt-load-ctl call on LG; LG_BOUND=<s> bounds the whole call (load_ready's polls: NC-R2-2).
lg() {
  ${LG_BOUND:+timeout "$LG_BOUND"} ssh -i "$HOME/.ssh/npt_lg" -o IdentitiesOnly=yes -o BatchMode=yes -o StrictHostKeyChecking=yes \
    -o ConnectTimeout=10 "npt-load@$(lg_ip)" "$1"
}
deadline_ok() {
  # Each VM's delete-by is its creation plus 4 h (L1); RS's boot stands for its creation. No block starts within
  # 20 minutes of it (INF-SB-12: the probe alone runs 14 minutes, 17 at its bounds), and an LG block also not within 20 minutes of LG's
  # (`r1.sh lg-boot`, from PROMPT-R1.md section 2 step 9; unrecorded, LG is taken as made 15 minutes before RS).
  local boot now lg
  boot=$(date -d "$(uptime -s)" +%s)
  now=$(date +%s)
  lg=$(tr -dc '0-9' < "$R/bringup/lg-boot.txt" 2> /dev/null || true)
  [ -n "$lg" ] || lg=$((boot - 900))
  ((now < boot + 4 * 3600 - 1200)) || return 1
  case ${1:-} in headers | probe | calibrate | ds-selftest) ((now < lg + 4 * 3600 - 1200)) ;; esac
}
# NC-SB-10: a block that opens rooms brings them down on every exit (its EXIT trap); a re-run is a new attempt, -aN in
# its run id and log names (create_new logs); a block marked done is never re-run unless its folder holds an INVALID
# file (ADR 0001:1582: a failing run is never re-run). Every attempt is kept and judged.
OPENED=()
ATT=''
LGK=()
down_all() {
  local n
  for n in "${OPENED[@]}"; do room_down "$n" || true; done
  # NC-R2-6: a block that died after LG's start keeps LG's copies (LGK: its dir and NN) before a re-run's start
  # deletes LG's npt-log; LG is stopped first, its abort path.
  [ "${#LGK[@]}" = 0 ] || { lg "stop ${LGK[1]}" || true; lg_keep "${LGK[@]}"; }
}
block_start() {
  local b=$1 n=1
  if [ -f "$R/chain/$b.done" ] && ! compgen -G "$R/$b/INVALID*" > /dev/null; then
    die "$b is done: a run is never re-run unless INVALID (ADR 0001:1582)"
  fi
  mkdir -p "$R/$b"
  [ ! -f "$R/$b/attempts" ] || n=$(($(wc -l < "$R/$b/attempts") + 1))
  printf 'attempt %s %s\n' "$n" "$(stamp)" >> "$R/$b/attempts"
  [ "$n" = 1 ] || ATT="-a$n"
  trap down_all EXIT
}
# busy <chain> <block>: why the block may not start now (a room's stdin holder alive; another chain running), else
# nothing. S14 opens no room and runs beside the browser legs by design (advisor-u8-stopb.md section 4).
busy() {
  local h p c
  for h in "$RUN"/room-*.holder; do
    [ -f "$h" ] && [ "$2" != s14 ] || continue
    p=$(tr -dc '0-9' < "$h")
    [ -z "$p" ] || [ ! -d "/proc/$p" ] || { printf 'room %s is up\n' "$(basename "$h" .holder)"; return 0; }
  done
  for c in server bites; do
    [ "$c" = "$1" ] || [ ! -f "$R/chain/$c.chain.log" ] || [ -f "$R/chain/$c.ended" ] || { printf 'the %s chain runs\n' "$c"; return 0; }
  done
}
# The page host (rs.yaml's npt-host.path and npt-host.service, Q-U6C-7): no browser leg and no HTTPS host-key
# cross-check runs unless systemd reads npt-host as active, and bring-up found the served dist equal to the expected
# list, or he gave his word on the differing files (`r1.sh dist-accept`: SEC-SB-5).
host_active() {
  local st
  [ -f "$R/bringup/dist.txt" ] || die "bring-up has not compared the served dist (SEC-SB-5)"
  [ ! -f "$R/bringup/DIST-DIFFERS" ] || [ -f "$R/bringup/DIST-ACCEPTED" ] \
    || die "the served dist differs from the expected list (bringup/DIST-DIFFERS): no page is served without his word"
  ! systemctl is-failed --quiet npt-host.path || die "npt-host.path failed (the host restarted past its start limit: INF-SB-6)"
  st=$(systemctl is-active npt-host || true)
  [ "$st" = active ] || die "npt-host is not active ($st): no browser leg and no host-key cross-check; this comes to Mustafa"
}
# dist_check <dir>: RS's dist.sha256 against the expected list committed at X (ops/runs/r1-dist.sha256, filled at Stop
# B). It never stops bring-up (the server chain still runs: a room faces only his /32 and LG); on any difference, or an
# empty list, DIST-DIFFERS names the files for him and the advisor.
dist_check() {
  python3 -I - "$EXPECTED_DIST" "$B/dist.sha256" "$1" << 'PY'
import os, sys
def read(p):
    out = {}
    for line in open(p, encoding="utf-8"):
        h, _, name = line.strip().partition("  ")
        if h and not h.startswith("#"):
            out[name] = h
    return out
want, got, d = read(sys.argv[1]), read(sys.argv[2]), sys.argv[3]
diff = sorted(n for n in set(want) | set(got) if want.get(n) != got.get(n)) if want else ["(the expected list is empty: Stop B fills it)"]
if diff:
    open(os.path.join(d, "DIST-DIFFERS"), "w").write("".join(f"{n}\n" for n in diff))
note = f"dist: DIFFERS in {len(diff)} file(s), named in DIST-DIFFERS" if diff else f"dist: equal to the expected list ({len(want)} files)"
open(os.path.join(d, "dist.txt"), "w").write(note + "\n")
print(note)
PY
}
timing_invalid() {
  local d=$1
  if [ -f "$R/bringup/TIMING-INVALID" ]; then
    cp "$R/bringup/alone-tasks.txt" "$d/INVALID-alone.txt"
    printf 'INVALID: bring-up read alone 0 (condition 4); the task list is beside this file and goes to the advisor\n'
  fi
}

# sudo_exact <sudo -l output> <sudoers file>: npt's sudo list holds exactly the file's `npt ALL=(root) NOPASSWD:`
# command lines, each as root with no password, nothing more and nothing less (SEC-SB-8); prints any difference.
sudo_exact() {
  python3 -I - "$1" "$2" << 'PY'
import sys
got, on = [], False
for line in open(sys.argv[1], encoding="utf-8"):
    if " may run the following commands" in line:
        on = True
    elif on and line[:1] in (" ", "\t") and line.strip():
        s, tag = line.strip(), "(root) NOPASSWD: "
        parts = s[len(tag):].split(", /") if s.startswith(tag) else [s]
        got += [f"{tag}{parts[0]}"] + [f"{tag}/{p}" for p in parts[1:]] if s.startswith(tag) else parts
want = [f"(root) NOPASSWD: {l.split('NOPASSWD: ', 1)[1].strip()}" for l in open(sys.argv[2], encoding="utf-8")
        if l.startswith("npt ALL=(root) NOPASSWD: ")]
diff = [f"extra: {c}" for c in got if c not in want] + [f"missing: {c}" for c in want if c not in got]
print("\n".join(diff) or f"exactly the {len(want)} command lines")
sys.exit(1 if diff or len(got) != len(want) else 0)
PY
}
# alone_min <room log>: the least `alone` over the log's affinity events.
alone_min() { logq "$1" affinity alone | sort -n | head -n 1; }
# One npt-log event field per line: logq <log> <event> <field> (`-`: each such event's fields, one JSON object a line).
logq() {
  python3 -I - "$1" "$2" "$3" << 'PY'
import json, sys
path, ev, field = sys.argv[1:4]
try:
    for line in open(path, encoding="utf-8"):
        try:
            r = json.loads(line)
        except ValueError:
            continue
        if r.get("k") == "event" and r.get("ev") == ev and (field == "-" or field in r.get("f", {})):
            print(json.dumps(r["f"]) if field == "-" else r["f"][field])
except OSError:
    pass
PY
}

# ---- room settings and rooms (the one home for the settings' field names) -----------------------------------------
# json_obj s:<key>=<text> | l:<key>=<a,b,…> | j:<key>=<JSON> …: one JSON object in argument order (texts escaped by
# json.dumps; an `l` value is a comma list of texts, empty for []).
json_obj() {
  python3 -I -c 'import json, sys
o = {}
for a in sys.argv[1:]:
    k, v = a[2:].split("=", 1)
    o[k] = v if a[0] == "s" else [x for x in v.split(",") if x] if a[0] == "l" else json.loads(v)
print(json.dumps(o))' "$@"
}

# room_settings NN <kind> [<log suffix>]: writes /var/lib/npt/run/room-NN.json (npt writes, npt-room reads through
# its group) with only the fields the settings type accepts (it refuses unknown fields: SEC-U6AD-2), checked by item 8
# against the merged struct, crates/server/src/main.rs:77-147: U6a's fields with U6b's (the binds, `transport`, the
# WebSocket pair, `hash_path`, `ws_split_mark_s`) and U6c's (`mode`; the probe's four: C1 (b)); `bite` only in a bites
# build, so only in a bite room. Each start is its own run, R1-<set>-NN<suffix> (the judge groups logs by run:
# eval.rs:37-75). npt-ctl builds npt-super's flags from this same file (npt-lib.sh npt_super_args), so npt-super's log
# and the room's carry one run and the same set, role, layer, profile, date, scene, tick_hz and bite (item 10).
# U8's values for U6a's required fields it had not set: io_threads one per housekeeping vCPU; warmup_ticks 1,800
# (60 s at 30 Hz) where LG's clients join, else 0; first_cpu_model bring-up's read, empty at the first start.
# Kinds: alone, headers, probe, calibrate, start, ds, browser:<slots>, bite:<id>.
room_settings() {
  local nn=$1 kind=$2 sfx=${3:-}$ATT scene=SS clients=1 tick=$TICK layer=L2 profile=P0 transport=both mode=room
  local set role=measured marks='' bite='' mark=null warmup=0 dur=0 cpu='' hk io='' addr commit day s f args chain='""' key='""'
  # -16 ("every room start in every run": marks.json) in every kind but the probe and the bites (NC-SB-6); the probe
  # serves -3 only, C-U8-2, -4 and -6 being conditions, not marks (NC-SB-5).
  case $kind in
    alone) set=bringup marks=PM-0001-16 ;;
    start) set=starts marks=PM-0001-16 ;;
    headers) set=headers clients=48 warmup=1800 dur=300000 marks=PM-0001-1,PM-0001-2,PM-0006-11,PM-0001-16 ;;
    probe) set=probe mode=probe profile=P1 marks=PM-0001-3 ;;
    calibrate) set=calibrate clients=48 warmup=1800 marks=PM-0001-16 ;; # held: it outlives LG (NC-R2-3)
    ds) set=ds-selftest clients=48 warmup=1800 dur=420000 transport=ws profile=P2 mark=120 marks=PM-0001-16 ;;
    browser:*)
      # Only a browser room presents the production chain (INF-SB-1, NC-SB-11); every other kind serves the room's own
      # certificate (the empty pair, U6b's merged form: main.rs:119-122).
      set=browsers clients=${kind#browser:} tick=null marks=PM-0001-16
      chain='"/var/lib/npt/tls/production.chain.pem"' key='"/var/lib/npt/tls/production.key.pem"'
      ;;
    bite:*)
      set=bites role=bite bite=${kind#bite:}
      marks=${bite%%/*}
      ;;
    *) die "unknown room kind" ;;
  esac
  [[ $clients =~ ^[0-9]{1,2}$ ]] || die "bad client count"
  hk=$(sed -n 's/^housekeeping=//p' "$STATE/cpus" 2> /dev/null)
  npt_cpulist_valid "$hk" || die "run npt-ctl cpus first"
  [ "$tick" = null ] || io=$(npt_cpulist_expand "$hk")
  [ "$kind" = alone ] || cpu=$(sed -n 's/^model //p' "$R/bringup/facts.txt" 2> /dev/null | head -n 1 || true)
  # NC-SB-15: both ends trimmed, as npt-log's read_local trims the model it compares with.
  cpu=${cpu#"${cpu%%[![:space:]]*}"} cpu=${cpu%"${cpu##*[![:space:]]}"}
  addr=$(rs_ip)
  commit=$(tr -d '\n' < "$B/head.txt")
  day=$(date -u +%Y-%m-%d)
  s=$((10#$nn)) f="$RUN/room-$nn.json"
  args=(s:run="R1-$set-$nn$sfx" s:set="R1-$set" s:role="$role" l:marks="$marks" s:date="$day" s:commit="$commit"
    s:layer="$layer" s:profile="$profile" j:seed=1 s:scene="$scene" j:clients="$clients" s:cmd_timing=B
    j:tick_hz="$TICK_HZ" s:machine_role=RS s:machine_name=RS s:machine_plan=CCX23 s:machine_location="$LOCATION"
    s:machine_address="$addr" j:tick_vcpu="$tick" j:io_vcpus="[${io// /,}]" j:io_threads="$(npt_cpulist_count "$hk")"
    s:log="$ROOMS/room-$nn$sfx.jsonl" s:token_file="$ROOMS/room-$nn.tok" j:duration_ms="$dur" j:warmup_ticks="$warmup"
    s:first_cpu_model="$cpu" s:mode="$mode" s:transport="$transport" s:wt_bind="0.0.0.0:$((30001 + s))"
    s:ws_bind="0.0.0.0:$((31001 + s))" j:ws_cert_chain="$chain" j:ws_cert_key="$key"
    s:hash_path="$HASH/room-$nn.json" j:ws_split_mark_s="$mark")
  [ -z "$bite" ] || args+=(s:bite="$bite")
  if [ "$mode" = probe ]; then
    # U6c's probe mode (C1 (b)): LG's IPv4 is the one peer; the six legs, 1,000,000 a direction, 10 s warm-up.
    args+=(s:probe_peer_ipv4="$(lg_ip)" l:probe_legs="${PROBE_LEGS// /,}" j:probe_packets="$PROBE_PACKETS"
      j:probe_warmup_s="$PROBE_WARMUP_S")
  fi
  rm -f "$f"
  json_obj "${args[@]}" > "$f"
  chmod 0640 "$f"
}

# room_up NN [<log suffix>]: a holder keeps the room's stdin open; the room starts through npt-ctl; waits for OPEN
# (npt-super's room_start event in its log) up to 15 s.
room_up() {
  local nn=$1 sfx=${2:-}$ATT fifo="$RUN/room-$1.in" i
  OPENED+=("$nn")
  rm -f "$fifo"
  mkfifo -m 0600 "$fifo"
  setsid bash -c 'exec 3> "$1"; exec sleep infinity' _ "$fifo" < /dev/null > /dev/null 2>&1 &
  printf '%s\n' "$!" > "$RUN/room-$nn.holder"
  setsid sudo -n /usr/local/sbin/npt-ctl rooms start "$nn" < "$fifo" > "$RUN/room-$nn.out" 2>&1 &
  printf '%s\n' "$!" > "$RUN/room-$nn.pid"
  for i in $(seq 1 75); do
    if [ -n "$(logq "$ROOMS/super-$nn$sfx.jsonl" room_start spawn_to_open_ns)" ]; then
      # NC-SB-13: npt-super writes room_start for a start killed at its deadline or ended before OPEN too, then
      # room_exit; an open room has no room_exit a second later (read live once npt-super flushes: F28, U6b's merge).
      sleep 1
      [ -n "$(logq "$ROOMS/super-$nn$sfx.jsonl" room_exit nonzero)" ] || return 0
      room_down "$nn" || true
      die "room $nn ended at its start (room_exit right after room_start; its output: $RUN/room-$nn.out)"
    fi
    sleep 0.2
  done
  room_down "$nn" || true
  die "room $nn did not open within 15 s (its output: $RUN/room-$nn.out)"
}

# room_down NN: closes the room's stdin (npt-super ends and kills its room), then waits up to 15 s.
room_down() {
  local nn=$1 holder pid i
  holder=$(tr -dc '0-9' < "$RUN/room-$nn.holder" 2> /dev/null) || true
  pid=$(tr -dc '0-9' < "$RUN/room-$nn.pid" 2> /dev/null) || true
  [ -n "$holder" ] && kill "$holder" 2> /dev/null || true
  for i in $(seq 1 75); do [ -n "$pid" ] && [ -d "/proc/$pid" ] || break; sleep 0.2; done
  rm -f "$RUN/room-$nn.in" "$RUN/room-$nn.holder" "$RUN/room-$nn.pid"
  [ -z "$pid" ] || [ ! -d "/proc/$pid" ]
}

# load_settings NN <mode> <transport> <clients> <duration s>: npt-load's settings (U6c's C1 (c); the one home). Modes
# run, calibrate and probe. The run's fields (run, set, marks, date, commit, seed, and the header's role, layer,
# profile, scene and tick_hz, so LG's log agrees with the room's: NC-SB-7) come from the room's own settings file. pin_hex is the leaf the clients pin: the room's hash file's value_hex whenever the room serves its
# own certificate (every LG room: INF-SB-1; which hash file a WebSocket pin reads is checked at U6b's merge); the
# production order's SHA256 only on ws to a room that names the production chain; the probe pins nothing (it writes no
# hash file: C1 (b)), so its pin_hex is 64 zeros. npt-load ends at duration_s with DONE (its stdin is /dev/null under
# npt-load-ctl); `lg "stop NN"` stays the abort path.
load_settings() {
  python3 -I - "$1" "$2" "$3" "$4" "$5" "$(rs_ip)" "$(lg_ip)" "$RUN/room-$1.json" "$HASH/room-$1.json" "$R/cert/production.txt" "$LOCATION" << 'PY'
import json, re, sys
nn, mode, transport, clients, duration, rs, lg, room, hashf, cert, location = sys.argv[1:12]
r = json.load(open(room, encoding="utf-8"))
try:
    if mode == "probe":
        pin = "0" * 64
    elif transport == "ws" and r.get("ws_cert_chain"):
        pin = re.search(r"Ledger: serial [0-9a-f]+, sha256 ([0-9a-f]{64})\.", open(cert, encoding="utf-8").read())[1]
    else:
        pin = json.load(open(hashf, encoding="utf-8"))["value_hex"]
except (OSError, ValueError, KeyError, TypeError):
    pin = ""
if mode not in ("run", "calibrate", "probe") or not re.fullmatch(r"[0-9a-f]{64}", str(pin)):
    sys.exit(f"r1.sh: no load settings for room {nn}: mode {mode}, or no pin on {transport}")
s = int(nn, 10)
print(json.dumps({"room": nn, "server": rs, "wt_port": 30001 + s, "ws_port": 31001 + s, "mode": mode,
    "transport": transport, "clients": int(clients), "duration_s": int(duration),
    "token_path": f"/var/lib/npt-load/tokens/room-{nn}.tok", "log_path": f"/var/lib/npt-load/logs/load-{nn}.jsonl",
    "run": r["run"], "set": r["set"], "marks": r["marks"], "date": r["date"], "commit": r["commit"],
    "role": r["role"], "layer": r["layer"], "profile": r["profile"], "scene": r["scene"], "tick_hz": r["tick_hz"],
    "machine": {"role": "LG", "name": "LG1", "plan": "CAX21", "location": location, "address": lg},
    "behaviour": "brawl", "seed": r["seed"], "pin_hex": pin}))
PY
}
load_up() { sudo -n /usr/local/sbin/npt-ctl rooms tokens "$1" | lg "token $1" && load_settings "$@" | lg "settings $1" && lg "start $1"; }
# load_ready NN <deadline, epoch s>: npt-load prints READY once its clients are up (C1 (c)); waits for it in LG's
# load log until the clock passes the deadline (a deadline, not a count of tries: NC-SB-14), each poll bounded by the
# time left, so none runs past it (NC-R2-2).
load_ready() {
  local got left
  while left=$(($2 - $(date +%s))); ((left > 0)); do
    got=$(LG_BOUND=$left lg "log $1" || true)
    grep -qx READY <<< "$got" && return 0
    sleep 1
  done
  return 1
}
# setting NN <field>: one value of the room's settings file.
setting() { python3 -I -c 'import json,sys; print(json.load(open(sys.argv[1]))[sys.argv[2]])' "$RUN/room-$1.json" "$2"; }
# lg_block <dir> NN <kind> <mode> <transport>: an LG-loaded room (NC-SB-2, Q1). The room ends at its own duration_ms
# while LG still runs (LG's duration is the room's plus 30 s), so no measured tick follows the load; the block waits
# for npt-super's room_exit, then LG's end. READY must come 10 s before the room's warm-up ends, else the room comes
# down first, before its first measured tick, then LG stops: its series are empty, a gap, so the run reads INVALID,
# never FAIL. A held room (duration_ms 0: calibrate, no mark, its summary over LG's whole run: NC-R2-3) outlives its LG
# instead: LG runs CALIBRATE_S, then the room comes down.
lg_block() {
  local d=$1 nn=$2 mode=$4 tr=$5 ms warm t0 i lgs
  room_settings "$nn" "$3"
  ms=$(setting "$nn" duration_ms)
  warm=$(($(setting "$nn" warmup_ticks) / TICK_HZ))
  lgs=$((ms ? ms / 1000 + 30 : CALIBRATE_S))
  room_up "$nn"
  t0=$(date +%s)
  load_up "$nn" "$mode" "$tr" 48 "$lgs"
  LGK=("$d" "$nn")
  if ! load_ready "$nn" $((t0 + warm - 10)); then
    printf 'INVALID: LG printed no READY %s s before the warm-up ended; the room down before its first measured tick, then LG stopped (Q1)\n' 10 | tee "$d/INVALID-ready.txt"
    room_down "$nn" || true
    lg "stop $nn" || true
    return 0
  fi
  for i in $(seq 1 $((ms ? ms / 1000 + 60 : 0))); do
    [ -z "$(logq "$ROOMS/super-$nn$ATT.jsonl" room_exit nonzero)" ] || break
    sleep 1
  done
  load_wait "$nn" $((ms ? 90 : lgs + 60))
  room_down "$nn" || true
}
load_wait() {
  local i
  for i in $(seq 1 "$2"); do lg "status $1" | grep -q stopped && return 0; sleep 1; done
  lg "stop $1" || true
}
# save_log <file> <dir> [<name>]: one npt-log into the results (NC-R2-1). A whole log, or one cut at a line end (the
# judge reads it as cut short: its run INVALID by the complete rule, eval.rs:575-582), is copied as it is. One cut
# inside a line keeps its whole lines as <name> and the whole file as <name>.raw; one empty or with no header line goes
# to <name>.empty; either is named in <dir>/cut.txt. npt-judge reads only *.jsonl, so no cut line, whoever wrote it,
# fails collect's whole-tree call. A missing file is skipped.
save_log() {
  [ -f "$1" ] || return 0
  python3 -I - "$1" "$2" "${3:-${1##*/}}" << 'PY'
import json, os, shutil, sys
src, d, name = sys.argv[1:4]
data, good, lines, head = open(src, "rb").read(), 0, 0, None
for line in data.split(b"\n")[:-1]:
    try:
        r = json.loads(line)
    except ValueError:
        break
    if not isinstance(r, dict):
        break
    head = r if head is None else head
    good, lines = good + len(line) + 1, lines + 1
dst = os.path.join(d, name)
if (head or {}).get("k") != "header":
    shutil.copyfile(src, dst + ".empty")
    note = f"{name}: empty or no header line: kept as {name}.empty"
elif good < len(data):
    open(dst, "wb").write(data[:good])
    shutil.copyfile(src, dst + ".raw")
    note = f"{name}: cut inside a line: its {lines} whole lines kept as {name}, the whole file as {name}.raw"
else:
    shutil.copy2(src, dst)
    sys.exit(0)
open(os.path.join(d, "cut.txt"), "a", encoding="utf-8").write(note + "\n")
print(note)
PY
}
# keep <dir> NN [<log suffix> [<leg>...]]: the room's and npt-super's logs, and each named leg's log (the probe's six).
# Every npt-log is named *.jsonl, the one extension npt-judge reads (crates/judge/src/lib.rs:18-34: NC-SB-1).
keep() {
  local d=$1 nn=$2 sfx=${3:-}$ATT leg
  save_log "$ROOMS/room-$nn$sfx.jsonl" "$d"
  save_log "$ROOMS/super-$nn$sfx.jsonl" "$d"
  for leg in "${@:4}"; do save_log "$ROOMS/room-$nn$sfx-$leg.jsonl" "$d"; done
}
# lg_keep <dir> NN: LG's stdout copy, and its npt-log through save_log (a stopped or unreachable LG gives a cut or
# empty copy: NC-R2-1).
lg_keep() {
  LGK=()
  lg "log $2" > "$1/load-$2$ATT.log" || true
  lg "npt-log $2" > "$RUN/load-$2.fetch" || true
  save_log "$RUN/load-$2.fetch" "$1" "load-$2$ATT.jsonl"
  rm -f "$RUN/load-$2.fetch"
}

# ---- the blocks ----------------------------------------------------------------------------------------------
bringup() {
  local d alone
  d=$(out bringup)
  [ -f "$B/build.done" ] || { tail -n 5 "$B/steps.log" 2> /dev/null; die "cloud-init's build is not done (see above)"; }
  cp -p "$B"/steps.log "$B"/bin.sha256 "$B"/dist.sha256 "$B"/head.txt "$B"/netem.txt "$d/"
  local hz
  hz=$( (grep -E '^CONFIG_HZ=' "/boot/config-$(uname -r)" 2> /dev/null || zcat /proc/config.gz 2> /dev/null | grep -E '^CONFIG_HZ=') | cut -d= -f2 || true)
  {
    # C-U8-1: U3-Q9's four facts and the THP mode, U3d's dry-run commands; each read, or `unreadable`, never guessed
    # (CONFIG_HZ never from `getconf CLK_TCK`, which is USER_HZ).
    printf 'kernel %s\n' "$(uname -r)"
    printf 'clocksource %s (available: %s)\n' "$(readf /sys/devices/system/clocksource/clocksource0/current_clocksource)" \
      "$(readf /sys/devices/system/clocksource/clocksource0/available_clocksource)"
    printf 'cpuidle %s governor %s\n' "$(readf /sys/devices/system/cpu/cpuidle/current_driver)" \
      "$(readf /sys/devices/system/cpu/cpuidle/current_governor)"
    printf 'config_hz %s\n' "${hz:-unreadable}"
    printf 'thp %s\n' "$(readf /sys/kernel/mm/transparent_hugepage/enabled)"
    printf 'nproc %s\nmodel %s\n' "$(nproc)" "$(grep -m1 '^model name' /proc/cpuinfo | cut -d: -f2- | sed 's/^ //')"
    for c in /sys/devices/system/cpu/cpu[0-9]*; do printf '%s siblings %s\n' "${c##*/}" "$(cat "$c/topology/thread_siblings_list")"; done
    # Q-U6A-5's term: util-linux's taskset on the image.
    printf 'taskset %s (%s)\n' "$(taskset --version)" "$(dpkg -S /usr/bin/taskset)"
    printf 'sch_netem %s (bring-up: %s)\n' "$(grep -q '^sch_netem ' /proc/modules && echo loaded || echo absent)" "$(cat "$B/netem.txt")"
    printf 'ipv6_fetch %s\n' "$(curl -6 -sS -o /dev/null -w '%{http_code}' --max-time 10 https://github.com 2>&1 || true)"
  } > "$d/facts.txt"
  cat "$d/facts.txt"
  # The login: npt in its own group only; sudo lists the wrappers only (cloud-init's default-user merge, F13).
  [ "$(id -nG)" = npt ] || die "npt is in groups beyond its own: $(id -nG)"
  # Through a pipe: sudo wraps its list at 80 columns unless stdout is a FIFO (display.c), and a regular file is not
  # (SEC-R2-2); pipefail keeps sudo's own failure.
  sudo -n -l | cat > "$d/sudo-l.txt"
  sudo_exact "$d/sudo-l.txt" "$SPIKE/ops/sbin/sudoers.d/npt" > "$d/sudo-exact.txt" \
    || die "npt's sudo list is not exactly sudoers.d/npt's command lines (SEC-SB-8): see sudo-exact.txt"
  # C5: both Hetzner firewalls in force, before any room opens. Prints no address outside the VM list.
  if ! lg fwcheck | tee "$d/c5-lg.txt" | grep -q '^fwcheck: blocked'; then
    touch "$d/STOP"
    die "C5: LG reached RS's port 22: fw-rs is not in force. No room opens; this comes to Mustafa"
  fi
  if ping -c 3 -W 2 "$(lg_ip)" > "$d/c5-rs.txt" 2>&1; then
    touch "$d/STOP"
    die "C5: LG answered RS's ICMP echo: fw-lg is not in force. No room opens; this comes to Mustafa"
  fi
  if ! grep -q ' 0 received' "$d/c5-rs.txt"; then
    touch "$d/STOP"
    die "C5: the ICMP check did not run (see c5-rs.txt): fw-lg is unproven. No room opens; this comes to Mustafa"
  fi
  printf 'C5: fw-rs and fw-lg in force (LG cannot reach RS port 22; LG does not answer RS)\n' | tee "$d/c5.txt"
  dist_check "$d"
  # Condition 4: the housekeeping set, then one npt-super start whose `alone` must read 1.
  sudo -n /usr/local/sbin/npt-ctl cpus "$TICK" > "$d/cpus.txt"
  room_settings 00 alone
  room_up 00
  sleep 2
  { printf 'utc %s\n' "$(stamp)"; grep -E '^(cpu|ctxt|procs_running)' /proc/stat; } > "$d/alone-stat.txt"
  room_down 00 || true
  # NC-SB-8: read once the room is down and its log closed (npt-server's log is buffered to its footer), the least
  # `alone` over both affinity events.
  alone=$(alone_min "$ROOMS/room-00$ATT.jsonl")
  keep "$d" 00
  if [ "$alone" = 1 ]; then
    printf 'alone 1: the tick vCPU %s is the room tick thread'"'"'s alone\n' "$TICK" | tee "$d/alone.txt"
  else
    { printf 'alone %s (want 1); every task whose allowed list holds the tick vCPU %s:\n' "${alone:-unread}" "$TICK"
      for t in /proc/[0-9]*/task/[0-9]*; do
        printf '%s %s %s %s\n' "${t##*/}" "$(cut -d')' -f2- "$t/stat" 2> /dev/null | awk '{print $7}')" \
          "$(sed -n 's/^Cpus_allowed_list:[[:space:]]*//p' "$t/status" 2> /dev/null)" "$(npt_comm_clean "$(cat "$t/comm" 2> /dev/null)")"
      done | npt_affinity_scan "$TICK" | grep -E '^(counts|move) ' || true
    } > "$d/alone-tasks.txt"
    touch "$d/TIMING-INVALID"
    cat "$d/alone-tasks.txt"
    printf 'alone 0: recorded. R1 goes on with the certificate, the browser legs and the ping; the chain does not run the timing blocks (%s): each is recorded not run with this list and comes to the advisor (C1)\n' "$TIMING_BLOCKS"
  fi
  touch "$d/done"
}

cert() {
  local p=$1 d i
  d=$(out cert)
  [ ! -f "$R/bringup/STOP" ] || die "bring-up stopped (C5)"
  if [ "$p" = staging ]; then
    # R1 reads and records both directories' profile lists (plan.md:971-972).
    for u in https://acme-staging-v02.api.letsencrypt.org/directory https://acme-v02.api.letsencrypt.org/directory; do
      printf '%s %s\n' "$u" "$(curl -sS --max-time 15 "$u" | python3 -I -c 'import json,sys; print(sorted(json.load(sys.stdin).get("meta", {}).get("profiles", {})))')"
    done > "$d/profiles.txt"
    cat "$d/profiles.txt"
  fi
  setsid sudo -n /usr/local/sbin/npt-cert issue "$p" < /dev/null > "$RUN/cert-$p.live" 2>&1 &
  printf '%s\n' "$!" > "$RUN/cert-$p.pid"
  for i in $(seq 1 120); do grep -q '^Add in Cloudflare' "$RUN/cert-$p.live" && break; sleep 1; done
  # The TXT step goes to the laptop's screen only: never into the results.
  grep -E '^(Add in Cloudflare|Polling|npt-cert)' "$RUN/cert-$p.live" || die "the order printed no TXT step in 2 minutes"
}

cert_wait() {
  local p=$1 d pid i
  d=$(out cert)
  pid=$(tr -dc '0-9' < "$RUN/cert-$p.pid")
  for i in $(seq 1 1860); do [ -d "/proc/$pid" ] || break; sleep 1; done
  # Everything but the TXT step is kept, the issue mode's stderr `PROFILES …` line among it (plan.md:971-972).
  grep -v '^Add in Cloudflare' "$RUN/cert-$p.live" | tee "$d/$p.txt"
  rm -f "$RUN/cert-$p.live" "$RUN/cert-$p.pid"
  grep -q 'Ledger: serial' "$d/$p.txt" || die "the $p order failed: the run stops and this comes to Mustafa"
}

headers() {
  local d
  d=$(out headers)
  timing_invalid "$d"
  lg_block "$d" 01 headers run wt
  lg_keep "$d" 01
  keep "$d" 01
}

probe() {
  local d i ended=''
  d=$(out probe)
  stat_cpu "$d/stat-before.txt"
  room_settings 02 probe
  room_up 02
  # The probe mints no token (C1 (b)): LG's probe mode gets an empty token file, never `rooms tokens 02`.
  printf '' | lg "token 02"
  load_settings 02 probe wt 1 $((PROBE_TOTAL_S + 60)) | lg "settings 02"
  lg "start 02"
  LGK=("$d" 02)
  # The probe's own exit (npt-super's room_exit event) ends the probe; then LG's own end, 60 s on, with its footer
  # (NC-R2-1): `lg stop` is load_wait's abort path only.
  for i in $(seq 1 $((PROBE_TOTAL_S + 120))); do
    ended=$(logq "$ROOMS/super-02$ATT.jsonl" room_exit nonzero | head -n 1)
    [ -n "$ended" ] && break
    sleep 1
  done
  [ -n "$ended" ] || ended="not seen within $((PROBE_TOTAL_S + 120)) s"
  printf 'probe exit: nonzero %s\n' "$ended" | tee "$d/exit.txt"
  room_down 02 || true
  stat_cpu "$d/stat-after.txt"
  load_wait 02 120
  lg_keep "$d" 02
  keep "$d" 02 '' $PROBE_LEGS
  probe_presence "$ROOMS" "02$ATT" "$(setting 02 run)" | tee "$d/presence.txt"
  ! grep -q INVALID "$d/presence.txt" || cp "$d/presence.txt" "$d/INVALID-presence.txt"
}

# Q-U8-5 and C-U8-2, -4, -6 with Q-U6C-5's names (C1 (d)): each of the six per-leg logs room-02-<leg>.jsonl must hold
# `leg_edge` at `ph` warmup and after, with `udp_row_found` 1 and `udp_drops_count` at both (equal at both on P1), and
# `cpu_stat` at both with `cpu_user_count`. A missing read makes that leg INVALID, never "unreadable counts as present";
# "a drop on the probe's P1 socket between the two reads makes ImpairSocket's self-test INVALID with its cause named, and
# that leg comes to the advisor before any L2 mark counts with its printed E, its max and the counters, never set aside
# and re-run as noise" (plan.md:1256). Per leg, after the verdicts, three numbers reported and never judged
# (advisor-u6c-r2.md:187-191): the drops' rise, the up span's excess in intervals, the rise less the excess.
# NC-SB-3: each leg is its own run, so each leg log's header must read run `<settings run>-<leg>`, profile the leg,
# process `probe`, layer `L2`, and the log must end in its footer (the judge makes a run whose logs disagree on profile
# INVALID: eval.rs:37-75). probe_presence <rooms dir> NN <settings run>.
probe_presence() {
  python3 -I - "$1" "$2" "$3" << 'PY'
import json, os, re, sys
rooms, nn, run = sys.argv[1:4]
# NC-SB-9, NC-U6C-12 (U6c's merged names, SERIES.md:920-935, :952): each direction's rate is l2_summary's
# probe_<d>_window_count over FP-1's probe_<d>_window_ns, 10,000 a second within 1 %; P1's two slack and policy reads
# (plan.md:1256's pair) are header.params' start-edge pair and ImpairSocket's end-edge pair (unread: u64::MAX, "" or
# "unknown"; a policy reads "<name> <priority>": socket.rs:319, :358-362), the policy's name the same at both.
RATE = 10000
P1_READS = ("probe_start_timer_slack_ns", "probe_start_sched_policy", "l2_timer_slack_ns", "l2_sched_policy")
POL = {"SCHED_OTHER": 0, "SCHED_FIFO": 1, "SCHED_RR": 2, "SCHED_BATCH": 3, "SCHED_IDLE": 5, "SCHED_DEADLINE": 6}
pol = lambda p: f"{POL[p.split()[0]]} {int(p.split()[1])}" if type(p) is str and re.fullmatch(r"[A-Z_]+ -?\d+", p) and p.split()[0] in POL else "-1 -1"
def rate(s, d):
    n, ns = (s.get(f"probe_{d}_{k}") for k in ("window_count", "window_ns"))
    return n * 1e9 / ns if type(n) is int and type(ns) is int and ns > 0 else None
reports, p1, unread = [], [], lambda v: "unread" if v is None else v
for leg in ["P0", "P1", "P2", "P3", "P4", "B1"]:
    seen = {"leg_edge": {}, "cpu_stat": {}}
    head = foot = summ = None
    try:
        for line in open(os.path.join(rooms, f"room-{nn}-{leg}.jsonl"), encoding="utf-8"):
            try:
                r = json.loads(line)
            except ValueError:
                continue
            if not isinstance(r, dict):
                continue
            head = r if head is None and r.get("k") == "header" else head
            foot = r if r.get("k") == "footer" else foot
            if r.get("k") == "event" and r.get("ev") in seen and isinstance(r.get("f"), dict):
                seen[r["ev"]].setdefault(r.get("ph"), r["f"])
            summ = r["f"] if summ is None and r.get("ev") == "l2_summary" and isinstance(r.get("f"), dict) else summ
    except OSError:
        print(f"{leg}: INVALID: its log room-{nn}-{leg}.jsonl is missing or unreadable")
        continue
    count = lambda f, k: f.get(k) if type(f.get(k)) is int else None
    edge, cpu, rates = seen["leg_edge"], seen["cpu_stat"], [rate(summ or {}, d) for d in ("up", "down")]
    s0, pol0, s1, pol1 = (((head or {}).get("params") or {}).get(k) for k in P1_READS)
    p1_read = all(type(x) is int and x != 2**64 - 1 for x in (s0, s1)) and all(type(x) is str and x not in ("", "unknown") for x in (pol0, pol1))
    if leg == "P1":  # ADV-U8H-1 (b): l2_summary's and the slack reads' integers; each policy as its number and priority
        vals = list((summ or {}).items()) + [(P1_READS[0], s0), (P1_READS[2], s1)]
        p1 = [f"P1 {k} {v}" for k, v in vals if re.fullmatch("[a-z0-9_]+", k) and type(v) is int]
        p1 += [f"P1 {k} {pol(v)}" for k, v in ((P1_READS[1], pol0), (P1_READS[3], pol1))] + [f"P1 non_integer_fields {len(vals) - len(p1)}"]
    w0, w1 = (count(edge.get(ph, {}), "udp_drops_count") for ph in ("warmup", "after"))
    span, pps, sent = (count(f, k) for f, k in ((summ or {}, "probe_up_window_ns"), ((head or {}).get("params") or {}, "probe_rate_pps"),
        ((head or {}).get("params") or {}, "probe_sent_count")))
    rise = None if None in (w0, w1) else w1 - w0
    excess = None if None in (span, pps, sent) else round(span * pps / 1e9) - sent
    reports.append(f"report {leg} (never judged): rise {unread(rise)}, up span excess {unread(excess)} intervals, "
                   f"rise - excess {unread(None if None in (rise, excess) else rise - excess)}")
    gone = [f"{ev} at {ph}" for ev in seen for ph in ("warmup", "after") if ph not in seen[ev]]
    want = {"run": f"{run}-{leg}", "profile": leg, "process": "probe", "layer": "L2"}
    bad = [f"{k} {(head or {}).get(k)!r} (want {v!r})" for k, v in want.items() if (head or {}).get(k) != v]
    if bad or foot is None:
        print(f"{leg}: INVALID: its header or footer: {'; '.join(bad) or 'no footer'}")
    elif gone:
        print(f"{leg}: INVALID: missing {', '.join(gone)}")
    elif any(edge[ph].get("udp_row_found") != 1 or count(edge[ph], "udp_drops_count") is None for ph in edge):
        print(f"{leg}: INVALID: the socket's /proc/net/udp row was not read at both edges")
    elif any(count(cpu[ph], "cpu_user_count") is None for ph in cpu):
        print(f"{leg}: INVALID: /proc/stat was not read at both edges")
    elif None in rates:
        print(f"{leg}: INVALID: a direction's window count or span was not read (C-U8-4's rate)")
    elif any(abs(r - RATE) > RATE / 100 for r in rates):
        print(f"{leg}: INVALID: the rate up {round(rates[0])}/s, down {round(rates[1])}/s, is not within 1 % of {RATE}/s (C-U8-4)")
    elif leg == "P1" and not p1_read:
        print(f"P1: INVALID: a slack or policy read ({', '.join(P1_READS)}) is missing from its header (C-U8-4)")
    elif leg == "P1" and pol0.split()[0] != pol1.split()[0]:
        print(f"P1: INVALID: the release thread's policy {pol0} at the window's start, {pol1} at the header read (C-U8-4)")
    elif leg == "P1" and edge["after"]["udp_drops_count"] != edge["warmup"]["udp_drops_count"]:
        d = edge["after"]["udp_drops_count"] - edge["warmup"]["udp_drops_count"]
        print(f"P1: INVALID: {d} drops on the probe's P1 socket between its reads (C-U8-6): to the advisor before any L2 mark counts")
    else:
        print(f"{leg}: present at both edges")
for r in reports + p1:
    print(r)
PY
}

calibrate() {
  local d
  d=$(out calibrate)
  lg_block "$d" 03 calibrate calibrate wt
  lg_keep "$d" 03
  keep "$d" 03
}

starts() {
  local d i sfx
  d=$(out starts)
  for i in $(seq -w 1 20); do
    sfx="-s$i"
    room_settings 04 start "$sfx"
    room_up 04 "$sfx"
    room_down 04 || true
    keep "$d" 04 "$sfx"
  done
  printf 'starts: %s\n' "$(cat "$ROOMS"/super-04-s*.jsonl | grep -c '"ev":"room_start"' || true)"
}

ds_selftest() {
  local d
  d=$(out ds-selftest)
  timing_invalid "$d"
  stat_cpu "$d/stat-before.txt"
  # LG's 48 WebSocket connections at P2 join within 50 s (READY: lg_block); the 1 min warm-up; the pool's mark at
  # OPEN + 120 s (settings `ws_split_mark_s`); the measured window the 5 minutes after it, to the room's end at 420 s,
  # while LG runs to 450 s (C-U8-3; NC-SB-2).
  lg_block "$d" 05 ds run ws
  stat_cpu "$d/stat-after.txt"
  lg_keep "$d" 05
  keep "$d" 05
  # ADV-U8H-1 (b): the room's ws_split fields (condition 10's read), integers only, for the pushed summaries.
  logq "$ROOMS/room-05$ATT.jsonl" ws_split - | python3 -I -c 'import json, re, sys
f = json.loads((sys.stdin.read().splitlines() or ["null"])[0])
ok = [f"{k} {v}" for k, v in f.items() if re.fullmatch("[a-z0-9_]+", k) and type(v) is int] if isinstance(f, dict) else []
print("\n".join(ok + [f"ws_split non_integer_fields {len(f) - len(ok)}"]) if isinstance(f, dict) else "no ws_split event in the room log")' > "$d/ws_split.txt"
}

# s14_run <dir>: one npt-s14 run copied into <dir>, then compare.py's per-run standing on RS (NC-SB-12).
s14_run() {
  local dir
  mkdir -p "$1"
  dir=$(sudo -n /usr/local/sbin/npt-s14 run | sed -n 's/^npt-s14: .* results in \(\/var\/lib\/npt\/s14\/[0-9TZ]*\)$/\1/p')
  [ -n "$dir" ] && [ -d "$dir" ] || die "npt-s14 gave no results folder"
  cp -rp "$dir"/. "$1/"
  python3 -I "$SPIKE/ops/runs/s14/compare.py" summary "$1" | tee "$1/summary.txt"
}
# A VOID or INVALID S14 runs once more the same day only if that ends 5 minutes before the bites' start (12:57, T0 +
# 3 h 27 min), else on R2's RS (a new folder, R1's kept); both are kept. A MISS needs the model and is never re-run.
s14() {
  local d boot
  d=$(out s14)
  s14_run "$d"
  if grep -Eq '; (VOID|INVALID): ' "$d/summary.txt"; then
    cp "$d/summary.txt" "$d/INVALID-s14.txt"
    boot=$(date -d "$(uptime -s)" +%s)
    if (($(date +%s) + S14_S + 300 <= boot + BITES_AT_S)); then
      s14_run "$d/rerun"
    else
      printf 'no time for a re-run before the bites: on R2'"'"'s RS, a new folder (R1'"'"'s kept)\n' | tee "$d/rerun-R2.txt"
    fi
  fi
  # The browser legs that ran beside S14 (section 4 of advisor-u8-stopb.md), on record beside its runs.
  cp -p "$R/browsers/stamps.txt" "$d/browser-stamps.txt" 2> /dev/null || true
}

bites() {
  local d id nn=06
  d=$(out bites)
  timing_invalid "$d"
  for id in PM-0001-1/shared-vcpu PM-0006-11/no-smt-map PM-0001-16/double-load; do
    room_settings "$nn" "bite:$id" "-${id##*/}"
    room_up "$nn" "-${id##*/}"
    sleep 10
    room_down "$nn" || true
    keep "$d" "$nn" "-${id##*/}"
  done
}

# R1.md's slot table: one room per mark and browser, a slot per attempt (C3; condition 2).
browser_room() {
  case $1 in
    smoke) printf '10 8\n' ;; cert-chrome) printf '11 20\n' ;; cert-edge) printf '12 20\n' ;;
    cert-firefox) printf '13 20\n' ;; cert-brave) printf '14 1\n' ;; fallback-chrome) printf '15 20\n' ;;
    fallback-edge) printf '16 20\n' ;; fallback-firefox) printf '17 20\n' ;; fallback-brave) printf '18 1\n' ;;
    mds) printf '19 3\n' ;; *) die "unknown browser room (R1.md's slot table)" ;;
  esac
}

readonly BROWSER_ROOMS="smoke cert-chrome cert-edge cert-firefox cert-brave fallback-chrome fallback-edge fallback-firefox fallback-brave mds"
browsers() {
  local nn slots
  host_active
  npt_valid_word "$1" $BROWSER_ROOMS || die "unknown browser room (R1.md's slot table)"
  read -r nn slots < <(browser_room "$1")
  room_settings "$nn" "browser:$slots"
  room_up "$nn"
  printf '%s up %s\n' "$1" "$(stamp)" >> "$(out browsers)/stamps.txt"
  printf 'browser room %s up: NN %s, UDP %s, TCP %s, %s slots\n' "$1" "$nn" $((30001 + 10#$nn)) $((31001 + 10#$nn)) "$slots"
}

browsers_down() {
  local nn slots d
  npt_valid_word "$1" $BROWSER_ROOMS || die "unknown browser room (R1.md's slot table)"
  read -r nn slots < <(browser_room "$1")
  d=$(out "browsers/$1")
  room_down "$nn" || true
  printf '%s down %s\n' "$1" "$(stamp)" >> "$(out browsers)/stamps.txt"
  keep "$d" "$nn"
  printf 'browser room %s down\n' "$1"
}

# collect: one npt-judge call over the whole tree first (pooled checks and bites need it). One malformed log fails
# that whole read (log/src/lib.rs:659; judge/src/lib.rs:36-46; save_log keeps cut lines out of it), so on its error
# each block folder is judged apart, and split.txt names the file the error names (ADV-SB-1).
collect() {
  local d b
  d=$(out collect)
  : > "$d/judge.txt"
  # NC-SB-8: the summaries open with C1's line when bring-up read alone 0.
  [ ! -f "$R/bringup/TIMING-INVALID" ] || printf 'C1: bring-up read alone 0; the timing blocks (%s) were not run (bringup/alone-tasks.txt)\n' "$TIMING_BLOCKS" | tee "$d/C1.txt" > "$d/judge.txt"
  if ! "$BIN/npt-judge" --marks "$SPIKE/marks/marks.json" --logs "$R" --out "$d/verdicts.json" >> "$d/judge.txt" 2>&1; then
    printf 'the whole-tree read failed; each block judged apart. %s\n' "$(grep -m 1 '^npt-judge: ' "$d/judge.txt" || true)" > "$d/split.txt"
    for b in "$R"/*/; do
      b=${b%/} && b=${b##*/}
      [ "$b" != collect ] || continue
      "$BIN/npt-judge" --marks "$SPIKE/marks/marks.json" --logs "$R/$b" --out "$d/verdicts-$b.json" > "$d/judge-$b.txt" 2>&1 || true
    done
    cat "$d/split.txt"
  fi
  tar -C "${R%/*}" -czf "${R%/*}/r1.tar.gz" "${R##*/}"
  sha256sum "${R%/*}/r1.tar.gz" | tee "$d/tarball.sha256"
}

chain_run() {
  local name=$1 list b why
  case $name in server) list=$SERVER_CHAIN ;; bites) list=$BITES_CHAIN ;; esac
  mkdir -p "$R/chain"
  # The chain's end is written on every exit (expanded now: the trap runs after this function's locals are gone).
  trap "printf 'chain $name ended %s\\n' \"\$(stamp)\" > \"$R/chain/$name.ended\"" EXIT
  for b in $list; do
    if [ -f "$R/bringup/STOP" ] || ! deadline_ok "$b"; then
      printf '%s stopped (bring-up STOP or a delete-by within 20 minutes) %s\n' "$b" "$(stamp)" > "$R/chain/$b.stopped"
      continue
    fi
    why=$(busy "$name" "$b")
    if [ -n "$why" ]; then
      printf '%s stopped: %s %s\n' "$b" "$why" "$(stamp)" > "$R/chain/$b.stopped"
      continue
    fi
    # NC-SB-8: under C1's alone 0 no timing block runs, so no log is made that could FAIL a mark C1 rules invalid;
    # probe (housekeeping vCPUs), calibrate, starts (-16 against 2 s) and s14 still run (NC-R2-5).
    if [ -f "$R/bringup/TIMING-INVALID" ] && npt_valid_word "$b" $TIMING_BLOCKS; then
      mkdir -p "$R/$b"
      cp -p "$R/bringup/alone-tasks.txt" "$R/$b/" 2> /dev/null || true
      printf 'not run: alone 0 (C1) %s\n' "$(stamp)" | tee "$R/$b/NOT-RUN.txt" > "$R/chain/$b.stopped"
      continue
    fi
    printf 'running %s\n' "$(stamp)" > "$R/chain/$b.running"
    if bash "$SELF" "$b" > "$R/chain/$b.log" 2>&1; then
      mv -f "$R/chain/$b.running" "$R/chain/$b.done"
    else
      mv -f "$R/chain/$b.running" "$R/chain/$b.stopped"
    fi
  done
}

[ "$#" -ge 1 ] || die "a block is needed"
case $1 in bringup | headers | probe | calibrate | starts | ds-selftest | s14 | bites) block_start "$1" ;; esac
case $1 in
  bringup) bringup ;;
  cert-staging) cert staging ;;
  cert-production) cert production ;;
  cert-staging-wait) cert_wait staging ;;
  cert-production-wait) cert_wait production ;;
  headers) headers ;;
  probe) probe ;;
  calibrate) calibrate ;;
  starts) starts ;;
  ds-selftest) ds_selftest ;;
  s14) s14 ;;
  bites) bites ;;
  host) host_active; printf 'npt-host active: fetch /npt/ssh-host-keys.txt over HTTPS now (PROMPT-R1.md section 3)\n' ;;
  lg-boot)
    # LG's creation for its delete-by (INF-SB-12): LG's boot in epoch seconds (its `grep btime /proc/stat`: PROMPT-R1.md 2.9).
    [ "$#" -eq 2 ] && [[ $2 =~ ^[0-9]{10}$ ]] && (($(date +%s) - $2 < 21600)) || die "lg-boot <LG's boot, epoch s, within 6 h>"
    printf '%s\n' "$2" > "$R/bringup/lg-boot.txt"
    ;;
  dist-accept)
    # His word, typed that day after he read the differing files (a security call, his: SEC-SB-5); never on an empty list.
    [ -f "$R/bringup/DIST-DIFFERS" ] || die "no difference to accept"
    ! grep -q 'the expected list is empty' "$R/bringup/DIST-DIFFERS" || die "the expected list is empty: Stop B's fill is missing"
    cat "$R/bringup/DIST-DIFFERS"
    printf 'accepted by his word %s\n' "$(stamp)" > "$R/bringup/DIST-ACCEPTED"
    ;;
  browsers) [ "$#" -eq 2 ] || die "browsers <room>"; browsers "$2" ;;
  browsers-down) [ "$#" -eq 2 ] || die "browsers-down <room>"; browsers_down "$2" ;;
  tokens) [ "$#" -eq 2 ] && npt_valid_nn "$2" || die "tokens NN"; exec sudo -n /usr/local/sbin/npt-ctl rooms tokens "$2" ;;
  collect) collect ;;
  chain)
    [ "$#" -eq 2 ] && npt_valid_word "$2" server bites || die "chain server|bites"
    [ -f "$R/bringup/done" ] || die "bring-up is not done"
    [ ! -f "$R/bringup/STOP" ] || die "bring-up stopped (C5)"
    mkdir -p "$R/chain"
    # One chain at a time: the bites never run beside S14 (advisor-u8-stopb.md section 4).
    for c in server bites; do
      [ ! -f "$R/chain/$c.chain.log" ] || [ -f "$R/chain/$c.ended" ] || die "the $c chain still runs: chain $2 waits"
    done
    rm -f "$R/chain/$2.ended"
    setsid nohup bash "$SELF" chain-run "$2" < /dev/null > "$R/chain/$2.chain.log" 2>&1 &
    printf 'chain %s started on RS (pid %s); it holds its rooms itself; read it with `r1.sh chain-status`\n' "$2" "$!"
    ;;
  chain-run) [ "$#" -eq 2 ] && npt_valid_word "$2" server bites || die "chain-run server|bites"; chain_run "$2" ;;
  chain-status)
    for b in $SERVER_CHAIN $BITES_CHAIN; do
      st=pending
      for m in done stopped running; do [ -f "$R/chain/$b.$m" ] && st=$m; done
      printf '%-12s %s\n' "$b" "$st"
    done
    ls "$R/chain/"*.ended 2> /dev/null || true
    ;;
  *) die "unknown block" ;;
esac

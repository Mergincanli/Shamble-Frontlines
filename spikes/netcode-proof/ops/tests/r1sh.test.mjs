// Test 7 (U8's fix pass, advisor-u6c-brief.md section 5 items 4, 5, 8 and 10): r1.sh's settings writers and the
// probe's presence check, run in bash on their real text (each function cut from r1.sh, beside stubs for the VM's
// reads), with fixture logs. Writes only under a temporary folder, and under NPT_R1_TREE when the caller sets it (3.1's
// results tree, kept for the main session's npt-judge run).
import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { existsSync, mkdirSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { test } from "node:test";
import { OPS, text, withTemp } from "./helpers.mjs";

const R1 = text("runs/r1.sh");
const LIB = join(OPS, "sbin", "npt-lib.sh");

/** One function of r1.sh, from `name() {` to its closing `}` at the line's start (or a one-line function). */
export function fn(name, body = R1) {
  const m = body.match(new RegExp(`^${name}\\(\\) \\{(?:\\n[\\s\\S]*?\\n\\}| .*; \\})\\n`, "m"));
  assert.ok(m, `r1.sh has ${name}()`);
  return m[0];
}
/** r1.sh's `readonly` lines naming any of `names`. */
function consts(...names) {
  return R1.split("\n").filter((l) => l.startsWith("readonly ") && names.some((n) => l.includes(`${n}=`))).join("\n");
}

const ADDR = { rs: "192.0.2.10", lg: "198.51.100.20" };
const COMMIT = "0123456789abcdef0123456789abcdef01234567";

/** The VM's folders and root-owned records under `dir` (the housekeeping set, LG's address, the commit). */
function prep(dir) {
  for (const d of ["state", "bringup", "run", "results/cert", "hash"]) mkdirSync(join(dir, d), { recursive: true });
  writeFileSync(join(dir, "state", "cpus"), "tick=3\nhousekeeping=0-2\n");
  writeFileSync(join(dir, "state", "peers"), `lg=${ADDR.lg}\n`);
  writeFileSync(join(dir, "bringup", "head.txt"), `${COMMIT}\n`);
}

/** Runs `script` in bash after r1.sh's constants, npt-lib.sh, stubs and the named functions; `dir` holds state. */
export function r1bash(dir, names, script) {
  prep(dir);
  const prelude = [
    "set -euo pipefail",
    `. "${LIB}"`,
    consts("TICK", "PROBE_LEGS", "PROBE_TOTAL_S", "CALIBRATE_S"),
    `STATE="${dir}/state" B="${dir}/bringup" RUN="${dir}/run" R="${dir}/results" HASH="${dir}/hash"`,
    "ROOMS=/var/lib/npt/rooms BIN=/opt/npt/bin ATT=''",
    "OPENED=() LGK=()",
    'die() { printf "die: %s\\n" "$*" >&2; exit 2; }',
    `rs_ip() { printf '%s\\n' ${ADDR.rs}; }`,
    `lg_ip() { printf '%s\\n' ${ADDR.lg}; }`,
    ...names.map((n) => fn(n)),
  ].join("\n");
  return execFileSync("bash", ["-c", `${prelude}\n${script}`], { encoding: "utf8", stdio: ["ignore", "pipe", "pipe"] });
}

/** One npt-log event line. */
const ev = (name, ph, f) => JSON.stringify({ k: "event", ev: name, ph, f });
const LEGS = ["P0", "P1", "P2", "P3", "P4", "B1"];
/** P1's two slack and policy reads in header.params, as U6c's merged probe writes them (SERIES.md:920-923, :952). */
const HEAD_READS = { probe_start_timer_slack_ns: 1, probe_start_sched_policy: "SCHED_FIFO 50", l2_timer_slack_ns: 1, l2_sched_policy: "SCHED_FIFO 50" };
/** A leg log's header as U6c writes it (NC-SB-3's interface line), with `o` over it. */
const legHead = (leg, o = {}) => JSON.stringify({ k: "header", run: `R1-probe-02-${leg}`, profile: leg, process: "probe", layer: "L2",
  params: { ...HEAD_READS, probe_rate_pps: 10000, probe_sent_count: 1_000_000 }, ...o });
/** A leg's events with both edges (drops `a` then `b`), less any `[event, ph]` in `drop`, its window (FP-1's counts and
 * spans: `up` records over `upNs`), its footer. */
function legLog(a = 7, b = 7, drop = [], up = 1_000_000, upNs = 100e9) {
  const lines = [];
  for (const [ph, drops] of [["warmup", a], ["after", b]]) {
    if (!drop.some(([e, p]) => e === "leg_edge" && p === ph)) {
      lines.push(ev("leg_edge", ph, { udp_row_found: 1, udp_drops_count: drops, snmp_udp_in_datagrams_count: 5 }));
    }
    if (!drop.some(([e, p]) => e === "cpu_stat" && p === ph)) {
      lines.push(ev("cpu_stat", ph, { cpu_user_count: 1000, cpu_system_count: 50, cpu3_user_count: 400 }));
    }
  }
  lines.push(ev("l2_summary", "after", { probe_up_window_count: up, probe_down_window_count: 1_000_000, probe_window_ns: 100e9,
    probe_up_window_ns: upNs, probe_down_window_ns: 100e9 }));
  lines.push(JSON.stringify({ k: "footer", records: lines.length + 1, ended: "clean" }));
  return `${lines.join("\n")}\n`;
}

test("item 4: the probe's settings carry U6c's probe fields; the block stores an empty token and waits on the probe's exit", () => {
  withTemp((dir) => {
    r1bash(dir, ["json_obj", "room_settings"], "room_settings 02 probe");
    const s = JSON.parse(readFileSync(join(dir, "run", "room-02.json"), "utf8"));
    assert.equal(s.mode, "probe");
    assert.equal(s.probe_peer_ipv4, ADDR.lg, "LG's IPv4");
    assert.deepEqual(s.probe_legs, LEGS);
    assert.equal(s.probe_packets, 1000000);
    assert.equal(s.probe_warmup_s, 10);
    const total = Number(r1bash(dir, [], 'printf "%s" "$PROBE_TOTAL_S"'));
    assert.equal(total, 6 * (10 + 100 + 12), "six legs at warm-up + window + tail and set-up");
  });
  const p = fn("probe").split("\n").filter((l) => !/^\s*#/.test(l)).join("\n");
  assert.match(p, /\n  printf '' \| lg "token 02"\n/, "an empty token for room 02");
  assert.doesNotMatch(p, /rooms tokens|load_up/, "never the room's token file: the probe mints none");
  assert.match(p, /load_settings 02 probe wt 1 \$\(\(PROBE_TOTAL_S \+ 60\)\) \| lg "settings 02"/, "LG: the total plus 60 s");
  assert.match(p, /for i in \$\(seq 1 \$\(\(PROBE_TOTAL_S \+ 120\)\)\); do\n {4}ended=\$\(logq "\$ROOMS\/super-02\$ATT\.jsonl" room_exit nonzero/);
  assert.match(p, /\n  room_down 02 \|\| true\n  stat_cpu "\$d\/stat-after\.txt"\n  load_wait 02 120\n  lg_keep "\$d" 02\n/, "then LG's end");
  assert.doesNotMatch(p, /lg "stop/, "lg stop is load_wait's abort path only (NC-R2-1)");
  assert.match(p, /keep "\$d" 02 '' \$PROBE_LEGS\n/, "keep copies the six per-leg logs");
  assert.match(p, /probe_presence "\$ROOMS" "02\$ATT" "\$\(setting 02 run\)" \| tee/);
});

test("item 4: probe_presence on fixture logs: present, a missing edge, a P1 drop, a missing log; ADV-U8H-1's P1 integers", () => {
  withTemp((dir) => {
    const rooms = join(dir, "rooms");
    mkdirSync(rooms);
    const write = (logs, heads = {}) => {
      for (const leg of LEGS) writeFileSync(join(rooms, `room-02-${leg}.jsonl`), `${legHead(leg, heads[leg])}\n${logs[leg] ?? legLog()}`);
    };
    const all = () => r1bash(dir, ["probe_presence"], `probe_presence "${rooms}" 02 R1-probe-02`).trim().split("\n");
    const run = () => all().filter((l) => /^\w+: /.test(l));
    write({});
    assert.deepEqual(run(), LEGS.map((l) => `${l}: present at both edges`));
    const ints = ["probe_up_window_count 1000000", "probe_down_window_count 1000000", "probe_window_ns 100000000000", "probe_up_window_ns 100000000000",
      "probe_down_window_ns 100000000000", "probe_start_timer_slack_ns 1", "l2_timer_slack_ns 1", "probe_start_sched_policy 1 50", "l2_sched_policy 1 50", "non_integer_fields 0"];
    assert.deepEqual(all().slice(12), ints.map((l) => `P1 ${l}`), "after the verdicts and reports: P1's l2_summary and its four reads, integers only");
    // advisor-u6c-r2.md:187-191: per leg, reported and never judged, the rise between the edges, the up span's excess in
    // intervals (probe_up_window_ns over 1 s / probe_rate_pps, less probe_sent_count) and the rise less the excess.
    write({ P1: legLog(7, 229, [], 1_000_000, 100_022_400_000) }, { P1: { params: { ...HEAD_READS, probe_rate_pps: 10000, probe_sent_count: 1_000_010 } } });
    const report = all().filter((l) => l.startsWith("report "));
    assert.equal(report.length, 6);
    assert.equal(report[0], "report P0 (never judged): rise 0, up span excess 0 intervals, rise - excess 0");
    assert.equal(report[1], "report P1 (never judged): rise 222, up span excess 214 intervals, rise - excess 8");
    assert.ok(report.every((l) => !l.includes("INVALID")), "never read by the block's INVALID check");
    // NC-SB-3: the settings' run on a leg, two legs sharing a run, a wrong profile, a missing footer: each INVALID.
    write({ P4: legLog().replace(/\{"k":"footer".*\n$/, "") }, { P0: { run: "R1-probe-02" }, P3: { run: "R1-probe-02-P2" }, B1: { profile: "P1" } });
    const heads = run();
    assert.match(heads[0], /^P0: INVALID: its header or footer: run 'R1-probe-02' \(want 'R1-probe-02-P0'\)/);
    assert.match(heads[3], /^P3: INVALID: its header or footer: run 'R1-probe-02-P2'/, "P3 shares P2's run");
    assert.match(heads[4], /^P4: INVALID: its header or footer: no footer/);
    assert.match(heads[5], /^B1: INVALID: its header or footer: profile 'P1' \(want 'B1'\)/);
    assert.deepEqual([heads[1], heads[2]], ["P1: present at both edges", "P2: present at both edges"]);
    write({ P3: legLog(7, 7, [["cpu_stat", "after"]]), P0: legLog(7, 7, [["leg_edge", "warmup"]]) });
    const missing = run();
    assert.equal(missing[0], "P0: INVALID: missing leg_edge at warmup");
    assert.equal(missing[3], "P3: INVALID: missing cpu_stat at after");
    assert.equal(missing.filter((l) => l.endsWith("present at both edges")).length, 4);
    write({ P1: legLog(7, 9), P2: legLog(7, 9) });
    const drop = run();
    assert.match(drop[1], /^P1: INVALID: 2 drops on the probe's P1 socket between its reads \(C-U8-6\)/);
    assert.equal(drop[2], "P2: present at both edges", "a drop counts against P1 only");
    write({ P1: legLog(7, 9).replace('"probe_window_ns":100000000000', '"probe_window_ns":"100 s","probe_ok":true') }, { P1: { params: { ...HEAD_READS, l2_sched_policy: "SCHED_UNKNOWN 0" } } });
    assert.deepEqual(all().slice(12).filter((l) => !ints.includes(l.slice(3))), ["P1 l2_sched_policy -1 -1", "P1 non_integer_fields 2"], "an INVALID P1 prints too");
    write({ B1: legLog().replace('"udp_row_found":1', '"udp_row_found":0') });
    assert.equal(run()[5], "B1: INVALID: the socket's /proc/net/udp row was not read at both edges");
    write({ P4: legLog().replaceAll('"cpu_user_count":1000', '"cpu_user_count":"1000"') });
    assert.equal(run()[4], "P4: INVALID: /proc/stat was not read at both edges");
    // NC-SB-9, NC-U6C-12: a count 2 % short; a span 2 % long at the exact count (the nominal probe_window_ns stays
    // 100 s, so only FP-1's per-direction span shows it); a direction's span unread.
    write({ P2: legLog(7, 7, [], 980_000), P4: legLog(7, 7, [], 1_000_000, 102e9), P0: legLog().replace(/,"probe_up_window_ns":[^,}]*/, "") });
    const rate = run();
    assert.equal(rate[2], "P2: INVALID: the rate up 9800/s, down 10000/s, is not within 1 % of 10000/s (C-U8-4)");
    assert.equal(rate[4], "P4: INVALID: the rate up 9804/s, down 10000/s, is not within 1 % of 10000/s (C-U8-4)");
    assert.equal(rate[0], "P0: INVALID: a direction's window count or span was not read (C-U8-4's rate)");
    assert.equal(rate[3], "P3: present at both edges", "an exact rate");
    // NC-U6C-12: P1's four reads in its header; each missing, an unread start pair (u64::MAX, ""), a policy changed.
    for (const field of Object.keys(HEAD_READS)) {
      const { [field]: _, ...rest } = HEAD_READS;
      write({}, { P1: { params: rest } });
      assert.match(run()[1], /^P1: INVALID: a slack or policy read \(probe_start_timer_slack_ns, probe_start_sched_policy, l2_timer_slack_ns, l2_sched_policy\) is missing from its header/, field);
    }
    for (const [k, unread] of [["probe_start_timer_slack_ns", "18446744073709551615"], ["l2_timer_slack_ns", "18446744073709551615"],
      ["probe_start_sched_policy", '""'], ["l2_sched_policy", '"unknown"']]) {
      writeFileSync(join(rooms, "room-02-P1.jsonl"), `${legHead("P1").replace(new RegExp(`"${k}":[^,}]*`), `"${k}":${unread}`)}\n${legLog()}`);
      assert.match(run()[1], /^P1: INVALID: a slack or policy read .* is missing from its header/, `${k} unread`);
    }
    write({}, { P1: { params: { ...HEAD_READS, l2_sched_policy: "SCHED_OTHER 0" } } });
    assert.equal(run()[1], "P1: INVALID: the release thread's policy SCHED_FIFO 50 at the window's start, SCHED_OTHER 0 at the header read (C-U8-4)");
    write({}, { P1: { params: { ...HEAD_READS, l2_sched_policy: "SCHED_FIFO 99" } } });
    assert.equal(run()[1], "P1: present at both edges", "the policy's name, not its priority");
    write({ P4: legLog(7, 7, [], 1_009_000) });
    assert.equal(run()[4], "P4: present at both edges", "0.9 % off: inside 1 %");
    write({});
    rmSync(join(rooms, "room-02-P2.jsonl"));
    assert.equal(run()[2], "P2: INVALID: its log room-02-P2.jsonl is missing or unreadable");
  });
});

const LOAD_FIELDS = ["room", "server", "wt_port", "ws_port", "mode", "transport", "clients", "duration_s", "token_path",
  "log_path", "run", "set", "marks", "date", "commit", "role", "layer", "profile", "scene", "tick_hz", "machine",
  "behaviour", "seed", "pin_hex"];

test("item 5: npt-load's settings carry the run's fields from the room's settings, LG's machine and the pinned leaf", () => {
  withTemp((dir) => {
    const room = { run: "R1-headers-01", set: "R1-headers", marks: ["PM-0001-1", "PM-0001-2"], date: "2026-12-03", commit: COMMIT, seed: 1,
      role: "measured", layer: "L2", profile: "P0", scene: "SS", tick_hz: 30 };
    const wtHash = "ab".repeat(32);
    const wsHash = "cd".repeat(32);
    prep(dir);
    for (const nn of ["01", "02", "05"]) writeFileSync(join(dir, "run", `room-${nn}.json`), JSON.stringify({ ...room, run: `R1-x-${nn}` }));
    writeFileSync(join(dir, "hash", "room-01.json"), JSON.stringify({ algorithm: "sha-256", value_hex: wtHash, validity_ns: 1 }));
    const load = (args) => JSON.parse(r1bash(dir, ["load_settings"], `load_settings ${args}`));
    const wt = load("01 run wt 48 300");
    assert.deepEqual(Object.keys(wt), LOAD_FIELDS);
    assert.deepEqual([wt.room, wt.server, wt.wt_port, wt.ws_port, wt.mode, wt.transport, wt.clients, wt.duration_s],
      ["01", ADDR.rs, 30002, 31002, "run", "wt", 48, 300]);
    assert.deepEqual([wt.run, wt.set, wt.marks, wt.date, wt.commit, wt.seed], ["R1-x-01", ...Object.values(room).slice(1, 6)]);
    assert.deepEqual(wt.machine, { role: "LG", name: "LG1", plan: "CAX21", location: "fsn1", address: ADDR.lg });
    assert.equal(wt.behaviour, "brawl");
    assert.equal(wt.pin_hex, wtHash, "wt: the room's hash file's value_hex");
    assert.throws(() => load("05 run ws 48 420"), /no pin on ws/, "ws to an own-certificate room without its hash file");
    writeFileSync(join(dir, "run", "room-12.json"), JSON.stringify({ ...room, ws_cert_chain: "/var/lib/npt/tls/production.chain.pem" }));
    assert.throws(() => load("12 run ws 1 60"), /no pin on ws/, "ws to the production chain without the order's line");
    writeFileSync(join(dir, "results", "cert", "production.txt"),
      `PROFILES classic,shortlived\nnpt-cert: issued (production, profile shortlived); egress closed. Ledger: serial 0a1b2c, sha256 ${wsHash}.\n`);
    assert.equal(load("12 run ws 1 60").pin_hex, wsHash, "ws to a room naming the production chain: the order's SHA256");
    assert.equal(load("02 probe wt 1 792").pin_hex, "0".repeat(64), "the probe writes no hash file and pins nothing");
    assert.throws(() => load("01 bots wt 48 300"), /no load settings/, "modes run, calibrate and probe only (C1 (c))");
  });
  // npt-load ends at duration_s: stdin stays /dev/null under npt-load-ctl; READY gates the DelayStream self-test's mark.
  assert.match(text("sbin/npt-load-ctl"), /\n {4}setsid "\$BIN" "\$BASE\/run\/load-\$NN\.json" < \/dev\/null > /);
  assert.match(fn("ds_selftest"), /\n  lg_block "\$d" 05 ds run ws\n/);
  assert.match(fn("load_ready"), /got=\$\(LG_BOUND=\$left lg "log \$1" \|\| true\)\n {4}grep -qx READY <<< "\$got" && return 0/);
  assert.doesNotMatch(R1, /load_up \d\d bots /, "no `bots` mode left");
});

test("ADV-U8H-1 (b): ds_selftest's ws_split.txt holds the ws_split event's integer fields, or one line saying there is none", () => {
  withTemp((dir) => {
    const stubs = "timing_invalid() { :; }; stat_cpu() { :; }; lg_block() { :; }; lg_keep() { :; }; keep() { :; }";
    const ws = (log) => (writeFileSync(join(dir, "room-05.jsonl"), log), r1bash(dir, ["out", "logq", "ds_selftest"], `ROOMS="${dir}"; ${stubs}; ds_selftest`),
      readFileSync(join(dir, "results", "ds-selftest", "ws_split.txt"), "utf8"));
    assert.equal(ws(`{\n${ev("ws_split", "measured", { ws_late_up_count: 9, ws_late_down_p99_ns: 800, ws_mode: "x" })}\n`), "ws_late_up_count 9\nws_late_down_p99_ns 800\nws_split non_integer_fields 1\n");
    assert.equal(ws(`${ev("ws_summary", "measured", { n: 1 })}\n`), "no ws_split event in the room log\n");
  });
});

// Item 8: every required field of the merged Settings; values of the types it takes (it refuses unknown fields:
// SEC-U6AD-2). The struct (crates/server/src/main.rs:77-147, U6a's fields with U6b's and U6c's) is read from the source
// with each field's Rust type; a field is optional when its type is `Option<…>` or `#[serde(default)]` stands above it.
const SERVER = readFileSync(join(OPS, "..", "crates", "server", "src", "main.rs"), "utf8");
const FIELDS = [];
let serdeDefault = false;
for (const l of SERVER.match(/\nstruct Settings \{\n([\s\S]*?)\n\}/)[1].split("\n")) {
  const f = l.match(/^ {4}([a-z_0-9]+): (.+),$/);
  if (f) FIELDS.push({ name: f[1], type: f[2], optional: serdeDefault || f[2].startsWith("Option<") });
  serdeDefault = f ? false : serdeDefault || /^ {4}#\[serde\(default\)\]$/.test(l);
}
const NAMES = FIELDS.map((f) => f.name);
/** A JSON value against a Rust type of the struct; a type not named here never fits, so it fails by name. */
const fits = (t, v) => (t.startsWith("Option<") ? v === null || fits(t.slice(7, -1), v)
  : t.startsWith("Vec<") ? Array.isArray(v) && v.every((x) => fits(t.slice(4, -1), x))
    : t === "String" ? typeof v === "string" : ["u32", "u64"].includes(t) && Number.isInteger(v) && v >= 0);
const U6B = ["wt_bind", "ws_bind", "transport", "ws_cert_chain", "ws_cert_key", "hash_path", "ws_split_mark_s"];
const U6C = ["mode", "probe_peer_ipv4", "probe_legs", "probe_packets", "probe_warmup_s"];
const KINDS = [["00", "alone"], ["01", "headers"], ["02", "probe"], ["03", "calibrate"], ["04", "start", "-s07"],
  ["05", "ds"], ["10", "browser:8"], ["06", "bite:PM-0001-16/double-load", "-double-load"]];
const settingsFor = (dir, [nn, kind, sfx = ""]) => {
  r1bash(dir, ["json_obj", "room_settings"], `room_settings ${nn} '${kind}' '${sfx}'`);
  return JSON.parse(readFileSync(join(dir, "run", `room-${nn}.json`), "utf8"));
};

test("item 8: the merged Settings holds 40 fields: U6a's 26 + `bite` + U6b's 7 + `cc` + U6c's 5", () => {
  assert.equal(FIELDS.length, 40, "U6a's 26 + `bite` + U6b's 7 + `cc` (U6b's E6) + U6c's 5 = 40: a struct change fails here");
  assert.deepEqual([...U6B, "cc", ...U6C, "bite"].filter((x) => !NAMES.includes(x)), [], "U6b's 7, `cc`, U6c's 5 and `bite` among the 40");
});

test("item 8: room_settings writes every required field of the merged Settings, values of the types it takes", () => {
  withTemp((dir) => {
    for (const k of KINDS) {
      const s = settingsFor(dir, k);
      const keys = Object.keys(s);
      assert.deepEqual(keys.filter((x) => !NAMES.includes(x)), [], `${k[1]}: no unknown field`);
      assert.deepEqual(FIELDS.filter((f) => !f.optional && f.name !== "bite" && !keys.includes(f.name)).map((f) => f.name), [], `${k[1]}: every required field`);
      const bad = FIELDS.filter((f) => f.name in s && !fits(f.type, s[f.name])).map((f) => `${f.name}: ${f.type} is not ${JSON.stringify(s[f.name])}`);
      assert.deepEqual(bad, [], `${k[1]}: each value of the type the struct takes`);
      assert.ok(!("cc" in s), `${k[1]}: no cc: R1 runs quinn's default, cubic (main.rs:127-129; PM-0001-24, the mark that selects it, is R3's)`);
      assert.equal("bite" in s, k[1].startsWith("bite:"), `${k[1]}: bite only in a bites build's room`);
      assert.equal(keys.some((x) => x.startsWith("probe_")), k[1] === "probe", `${k[1]}: the probe's four in the probe only`);
      assert.ok(!("super" in s) && !("log_path" in s) && !("token_path" in s), "npt-super's values ride on its flags");
      assert.equal(s.tick_hz, 30);
      assert.match(s.commit, /^[0-9a-f]{40}$/);
      assert.ok(["room", "probe"].includes(s.mode), "C1 (b): room or probe");
      assert.equal(s.tick_vcpu === null, s.io_vcpus.length === 0, "tick_vcpu and io_vcpus: both or neither");
      assert.ok(!s.io_vcpus.includes(s.tick_vcpu) && s.io_threads >= 1 && s.io_threads <= 64);
      const ms = { headers: 300000, ds: 420000 }[k[1]] ?? 0;
      assert.ok(s.clients >= 1 && s.clients <= 48 && s.duration_ms === ms, "measured LG rooms end at their duration; the rest held");
      assert.equal(s.log, `/var/lib/npt/rooms/room-${k[0]}${k[2] ?? ""}.jsonl`);
      assert.ok(Array.isArray(s.marks) && s.marks.every((m) => /^PM-\d{4}-\d{1,3}$/.test(m)));
      assert.equal(s.run, `R1-${s.set.slice(3)}-${k[0]}${k[2] ?? ""}`, "each start its own run");
    }
  });
  // npt-ctl starts U6a's `npt-super start`: every flag it can pass is one npt-super parses.
  const sup = readFileSync(join(OPS, "..", "crates", "super", "src", "main.rs"), "utf8");
  const known = new Set([...sup.slice(sup.indexOf("const HEADER_FLAGS"), sup.indexOf("#[cfg(test)]")).matchAll(/"(--[a-z-]+)"/g)].map((m) => m[1]));
  const lib = text("sbin/npt-lib.sh");
  const emitted = [...fn("npt_super_args", lib).matchAll(/(--[a-z-]+) "/g)].map((m) => m[1]);
  assert.deepEqual(emitted.filter((f) => !known.has(f)), [], "no flag npt-super refuses");
  for (const f of ["--room", "--settings", "--log", "--scene", "--tick-hz", "--run", "--set", "--role", "--date", "--commit",
    "--layer", "--profile", "--machine-role", "--machine-name", "--machine-plan", "--machine-location", "--machine-address"]) {
    assert.ok(emitted.includes(f), `npt-super's required ${f}`);
  }
});

test("item 10: npt-super's command line and the room's settings carry equal run fields, for every kind of room", () => {
  withTemp((dir) => {
    for (const k of KINDS) {
      const s = settingsFor(dir, k);
      // npt-ctl's own read (NPT_SETTINGS_PY, here without setpriv) and npt_super_args, on the file room_settings wrote.
      const out = r1bash(dir, [], [
        `mapfile -t vals < <(python3 -I -c "$NPT_SETTINGS_PY" "${dir}/run/room-${k[0]}.json" "$NPT_SUPER_FIELDS")`,
        `npt_super_args ${k[0]} "\${vals[@]}"`,
        'printf "%s\\n" "${NPT_SUPER_ARGS[@]}"',
      ].join("\n"));
      const argv = out.split("\n").slice(0, -1);
      const flag = {};
      for (let i = 0; i < argv.length; i += 2) flag[argv[i]] = argv[i + 1];
      const want = {
        "--run": s.run, "--set": s.set, "--role": s.role, "--layer": s.layer, "--profile": s.profile, "--date": s.date,
        "--scene": s.scene, "--tick-hz": String(s.tick_hz), "--commit": s.commit, "--machine-role": s.machine_role,
        "--machine-name": s.machine_name, "--machine-plan": s.machine_plan, "--machine-location": s.machine_location,
        "--machine-address": s.machine_address, "--settings": `/var/lib/npt/run/room-${k[0]}.json`,
        "--log": s.log.replace("/room-", "/super-"),
        "--room": s.bite ? "/opt/npt/bin/npt-server-bites" : "/opt/npt/bin/npt-server",
      };
      if (s.marks.length) want["--marks"] = s.marks.join(",");
      if (s.bite) want["--bite"] = s.bite;
      assert.deepEqual(flag, want, `${k[1]}: one source for the run and the eight header values`);
    }
  });
  // npt-super ends by its stdin closing (the holder's end), never by a kill: a log with no footer makes the run INVALID.
  const down = fn("room_down");
  assert.deepEqual(down.match(/\bkill\b[^\n]*/g), ['kill "$holder" 2> /dev/null || true'], "room_down kills the stdin holder only");
  const code = R1.split("\n").filter((l) => !/^\s*#/.test(l)).join("\n");
  assert.equal(code.match(/\bkill\b/g).length, 1, "no other kill in r1.sh");
  assert.doesNotMatch(code, /pkill|killall|systemctl (stop|kill)/);
});

// ---- fix pass 2 (advisor-u8-stopb.md section 7) ----------------------------------------------------------------
const JUDGE_RS = readFileSync(join(OPS, "..", "crates", "judge", "src", "lib.rs"), "utf8");

test("2.1 NC-SB-1: every npt-log path r1.sh, npt-lib.sh and lib.mjs write has the one extension npt-judge reads", () => {
  const ext = JUDGE_RS.match(/\} else if path\.extension\(\)\.is_some_and\(\|e\| e == "([a-z]+)"\) \{/)[1];
  const paths = [];
  withTemp((dir) => {
    for (const k of KINDS) {
      const s = settingsFor(dir, k);
      const flags = r1bash(dir, [], `mapfile -t vals < <(python3 -I -c "$NPT_SETTINGS_PY" "${dir}/run/room-${k[0]}.json" "$NPT_SUPER_FIELDS")
npt_super_args ${k[0]} "\${vals[@]}"; printf "%s\\n" "\${NPT_SUPER_ARGS[@]}"`).split("\n");
      paths.push(s.log, flags[flags.indexOf("--log") + 1]);
    }
    writeFileSync(join(dir, "hash", "room-01.json"), JSON.stringify({ value_hex: "ab".repeat(32) }));
    paths.push(JSON.parse(r1bash(dir, ["json_obj", "room_settings", "load_settings"], "room_settings 01 headers\nload_settings 01 run wt 48 330")).log_path);
  });
  for (const p of paths) assert.ok(p.endsWith(`.${ext}`), `${p}: npt-judge reads *.${ext} only`);
  assert.ok(fn("probe_presence").includes(`f"room-{nn}-{leg}.${ext}"`), "the probe's legs: the leg before the extension");
  const keep = fn("keep");
  for (const n of [`room-$nn$sfx.${ext}`, `super-$nn$sfx.${ext}`, `room-$nn$sfx-$leg.${ext}`]) assert.ok(keep.includes(n), `keep: ${n}`);
  assert.ok(text("browser/lib.mjs").includes(`writeFileSync(\`\${base}.${ext}\``), "the browser legs");
  for (const f of ["runs/r1.sh", "sbin/npt-lib.sh", "browser/lib.mjs", "browser/config.mjs"]) {
    assert.doesNotMatch(text(f), /(room|super)-[^\s"']*\.log\b|\.npt\.log/, `${f}: no npt-log left on .log`);
  }
});

test("2.2 INF-SB-1: only browser rooms name production.*; ds's WebSocket pin is the room's own hash file", () => {
  withTemp((dir) => {
    for (const k of KINDS) {
      const s = settingsFor(dir, k);
      const browser = k[1].startsWith("browser:");
      const body = readFileSync(join(dir, "run", `room-${k[0]}.json`), "utf8");
      assert.equal(body.includes("production."), browser, `${k[1]}: production.* only in a browser room`);
      assert.deepEqual([s.ws_cert_chain, s.ws_cert_key], browser
        ? ["/var/lib/npt/tls/production.chain.pem", "/var/lib/npt/tls/production.key.pem"] : ["", ""]);
    }
    writeFileSync(join(dir, "hash", "room-05.json"), JSON.stringify({ algorithm: "sha-256", value_hex: "5e".repeat(32), validity_ns: 1 }));
    const ds = r1bash(dir, ["json_obj", "room_settings", "load_settings"], "room_settings 05 ds\nload_settings 05 run ws 48 450");
    assert.equal(JSON.parse(ds).pin_hex, "5e".repeat(32), "no production file in the server chain");
  });
});

// lg_block run with every VM call stubbed, each call written to a trace; READY 0 (printed) or 1 (not).
const STUBS = (trace, ready) => `room_up() { echo "room_up $*" >> ${trace}; }
load_up() { echo "load_up $*" >> ${trace}; }
load_ready() { echo "load_ready $1 $(($2 - $(date +%s)))" >> ${trace}; return ${ready}; }
lg() { echo "lg $*" >> ${trace}; }
load_wait() { echo "load_wait $*" >> ${trace}; }
room_down() { echo "room_down $*" >> ${trace}; }
logq() { echo 0; }`;

test("2.3 NC-SB-2, Q1: each LG room ends 30 s before its LG; no READY 10 s before the warm-up's end brings it down", () => {
  withTemp((dir) => {
    for (const [nn, kind, mode, tr] of [["01", "headers", "run", "wt"], ["03", "calibrate", "calibrate", "wt"], ["05", "ds", "run", "ws"]]) {
      for (const ready of [0, 1]) {
        const trace = join(dir, `trace-${nn}-${ready}`);
        rmSync(join(dir, "results", nn), { recursive: true, force: true });
        mkdirSync(join(dir, "results", nn), { recursive: true });
        r1bash(dir, ["json_obj", "room_settings", "setting", "lg_block"], `${STUBS(trace, ready)}\nlg_block "${dir}/results/${nn}" ${nn} ${kind} ${mode} ${tr}`);
        const s = JSON.parse(readFileSync(join(dir, "run", `room-${nn}.json`), "utf8"));
        const calls = readFileSync(trace, "utf8").trim().split("\n");
        const lgS = Number(calls.find((c) => c.startsWith("load_up")).split(" ").at(-1));
        if (kind === "calibrate") {
          // NC-R2-3: no mark, its summary over LG's whole run: the room has no duration and outlives LG's 300 s.
          assert.ok(s.duration_ms === 0 && lgS === 300, "calibrate: a held room, LG 300 s");
          if (ready === 0) assert.equal(calls.at(-2), "load_wait 03 360", "calibrate: LG's whole run awaited, then the room down");
        } else {
          assert.ok(s.duration_ms > 0 && s.duration_ms / 1000 + 30 <= lgS, `${kind}: the room ends at least 30 s before LG`);
        }
        const by = Number(calls.find((c) => c.startsWith("load_ready")).split(" ").at(-1));
        const warm = s.warmup_ticks / s.tick_hz;
        assert.ok(warm === 60 && by <= warm - 10 && by >= warm - 12, `${kind}: READY due 10 s before the warm-up's end (${by} s)`);
        const invalid = readdirSync(join(dir, "results", nn)).includes("INVALID-ready.txt");
        if (ready === 0) {
          assert.deepEqual(calls.slice(-2).map((c) => c.split(" ")[0]), ["load_wait", "room_down"], `${kind}: LG's end, then down`);
          assert.ok(!invalid);
        } else {
          assert.deepEqual(calls.slice(-2), [`room_down ${nn}`, `lg stop ${nn}`], `${kind}: no READY: room down, then LG (NC-R2-2)`);
          assert.ok(invalid && !calls.some((c) => c.startsWith("load_wait")), `${kind}: before any measured tick`);
        }
      }
    }
    // NC-R2-2: each READY poll is bounded by the time left (a clock read per poll: 1000, 1001, …; deadline 1003).
    const polls = join(dir, "polls");
    const clock = `date() { local n; n=$(cat ${dir}/clock 2> /dev/null || echo 0); echo $((n + 1)) > ${dir}/clock; echo $((1000 + n)); }
sleep() { :; }; lg() { echo "\${LG_BOUND:-none} $1" >> ${polls}; [ "$READY" = 0 ] || echo READY; }`;
    assert.throws(() => r1bash(dir, ["load_ready"], `${clock}\nREADY=0 load_ready 03 1003`));
    assert.deepEqual(readFileSync(polls, "utf8").trim().split("\n"), ["3 log 03", "2 log 03", "1 log 03"], "the bound shrinks");
    rmSync(join(dir, "clock"));
    assert.equal(r1bash(dir, ["load_ready"], `${clock}\nREADY=1 load_ready 03 1003 && echo up`).trim(), "up");
    const lgCalls = r1bash(dir, ["lg"], 'timeout() { echo "timeout $1 $2"; }; ssh() { echo ssh; }; lg "log 03"; LG_BOUND=4 lg "log 03"');
    assert.deepEqual(lgCalls.trim().split("\n"), ["ssh", "timeout 4 ssh"], "unbounded, or timeout over the whole ssh call");
  });
});

test("2.9 SEC-SB-5: a differing or empty expected dist makes host and browsers refuse; the server chain may start", () => {
  withTemp((dir) => {
    prep(dir);
    const bring = join(dir, "results", "bringup");
    const served = `${"a".repeat(64)}  ./index.html\n${"b".repeat(64)}  ./npt_client_wasm_bg.wasm\n`;
    writeFileSync(join(dir, "bringup", "dist.sha256"), served);
    const run = (expected, after = "", failed = 1) => {
      rmSync(bring, { recursive: true, force: true });
      mkdirSync(bring, { recursive: true });
      writeFileSync(join(dir, "expected.sha256"), expected);
      const sys = `systemctl() { [ "$1" != is-failed ] || return ${failed}; echo active; }`;
      const script = `EXPECTED_DIST="${dir}/expected.sha256"\n${sys}\ndist_check "${bring}" > /dev/null\n${after}host_active && echo SERVED`;
      try {
        return r1bash(dir, ["dist_check", "host_active"], script).trim();
      } catch (e) {
        return String(e.stderr).trim();
      }
    };
    assert.equal(run(`# filled at Stop B\n${served}`), "SERVED", "equal lists: the page is served");
    assert.match(run(served, "", 0), /^die: npt-host\.path failed \(the host restarted past its start limit: INF-SB-6\)/, "2.17");
    assert.match(run(served.replace("b".repeat(64), "c".repeat(64))), /^die: the served dist differs from the expected list/);
    assert.equal(readFileSync(join(bring, "DIST-DIFFERS"), "utf8"), "./npt_client_wasm_bg.wasm\n", "the differing file named");
    assert.equal(run(served.replace("b".repeat(64), "c".repeat(64)), `touch "${bring}/DIST-ACCEPTED"\n`), "SERVED", "his word");
    assert.match(run(`${served}${"d".repeat(64)}  ./extra.js\n`), /differs/, "a file missing on RS");
    assert.match(run(text("runs/r1-dist.sha256").replace(/^(?!#).*\n/gm, "")), /differs/, "the comment alone: refused");
    assert.match(readFileSync(join(bring, "DIST-DIFFERS"), "utf8"), /the expected list is empty/);
  });
  assert.ok(fn("bringup").indexOf('dist_check "$d"') < fn("bringup").indexOf('touch "$d/done"'), "bring-up still ends done");
  assert.match(R1, /\n {2}dist-accept\)\n[^]*?die "the expected list is empty: Stop B's fill is missing"/, "no word accepts an empty list");
  assert.doesNotMatch(R1.match(/\n {2}chain\)\n[^]*?;;/)[0], /DIST/, "the server chain may start whatever the dist");
  assert.equal(fn("browsers").split("\n")[2], "  host_active", "every browsers call checks first");
});

// npt-judge's stand-in: like read_log (log/src/lib.rs:656-659, :722-726), a line that is not one JSON record, or an
// empty log, fails the whole read naming its file (exit 1); a log with no footer reads (its run INVALID, not modelled).
const JUDGE_STUB = `#!/bin/bash
while [ $# -gt 0 ]; do case $1 in --logs) L=$2 ;; --out) O=$2 ;; esac; shift 2; done
for f in $(find "$L" -name '*.jsonl' | sort); do
  msg=$(python3 -I -c 'import json, sys
lines = open(sys.argv[1], encoding="utf-8").read().splitlines() or sys.exit("empty log: no header")
for n, l in enumerate(lines, 1):
    try:
        json.loads(l)
    except ValueError:
        sys.exit(f"line {n}: not one JSON record")' "$f" 2>&1) || { echo "npt-judge: $f: $msg"; exit 1; }
done
echo '{"verdicts": []}' > "$O"
`;

test("2.10 NC-SB-8: alone read from the closed log (the min); under TIMING-INVALID the chain runs no timing block", () => {
  withTemp((dir) => {
    prep(dir);
    const log = join(dir, "room-00.jsonl");
    const aff = (a) => ev("affinity", "warmup", { alone: a, tick_vcpu: 3 });
    writeFileSync(log, `{"k":"header"}\n${aff(1)}\n${aff(0)}\n{"k":"footer","records":3,"ended":"clean"}\n`);
    assert.equal(r1bash(dir, ["logq", "alone_min"], `alone_min "${log}"`).trim(), "0", "both events read, the min");
    writeFileSync(log, `{"k":"header"}\n${aff(1)}\n${aff(1)}\n`);
    assert.equal(r1bash(dir, ["logq", "alone_min"], `alone_min "${log}"`).trim(), "1");
    const b = fn("bringup");
    assert.ok(b.indexOf("room_down 00 || true") < b.indexOf('alone=$(alone_min "$ROOMS/room-00$ATT.jsonl")'), "read after room_down");
    // The chains with a block stub that records what it ran.
    mkdirSync(join(dir, "results", "bringup"), { recursive: true });
    writeFileSync(join(dir, "results", "bringup", "TIMING-INVALID"), "");
    writeFileSync(join(dir, "results", "bringup", "alone-tasks.txt"), "alone 0 (want 1)\n");
    writeFileSync(join(dir, "block"), `echo "$1" >> "${dir}/ran"\n`);
    const pre = `${consts("SERVER_CHAIN", "BITES_CHAIN", "TIMING_BLOCKS")}\nSELF="${dir}/block"\ndeadline_ok() { true; }\nstamp() { echo T; }`;
    r1bash(dir, ["busy", "chain_run"], `${pre}\nchain_run server\nchain_run bites`);
    // NC-R2-5: the skip is exactly the blocks whose judged work runs on the tick vCPU.
    assert.deepEqual(readFileSync(join(dir, "ran"), "utf8").trim().split("\n"), ["probe", "calibrate", "starts", "s14"], "the rest run");
    assert.equal(readFileSync(join(dir, "results", "chain", "bites.ended"), "utf8"), "chain bites ended T\n", "the end on exit");
    assert.ok(!["probe", "starts"].some((t) => existsSync(join(dir, "results", t))), "no NOT-RUN record for a block that ran");
    assert.ok(!["probe", "starts"].some((b) => fn(b).includes("timing_invalid")), "and no INVALID-alone record either");
    for (const t of ["headers", "ds-selftest", "bites"]) {
      assert.match(readFileSync(join(dir, "results", t, "NOT-RUN.txt"), "utf8"), /^not run: alone 0 \(C1\)/, t);
      assert.ok(readdirSync(join(dir, "results", t)).includes("alone-tasks.txt"), `${t}: the task list beside it`);
      assert.match(readFileSync(join(dir, "results", "chain", `${t}.stopped`), "utf8"), /not run: alone 0/);
    }
    // collect's summaries open with C1's line.
    mkdirSync(join(dir, "bin"));
    writeFileSync(join(dir, "bin", "npt-judge"), JUDGE_STUB, { mode: 0o755 });
    r1bash(dir, ["out", "collect"], `${consts("TIMING_BLOCKS")}\nBIN="${dir}/bin" SPIKE="${dir}"\ncollect`);
    assert.match(readFileSync(join(dir, "results", "collect", "judge.txt"), "utf8"), /^C1: bring-up read alone 0; the timing blocks/);
  });
});

test("2.11 NC-SB-10, INF-SB-12: a trap per block; busy refusals; -aN in run id and log names; no re-run of a done block", () => {
  withTemp((dir) => {
    prep(dir);
    const trace = join(dir, "trace");
    const stub = `stamp() { echo T; }\nroom_down() { echo "down $1" >> ${trace}; }`;
    assert.throws(() => r1bash(dir, ["block_start", "down_all"], `${stub}\nblock_start headers\nOPENED+=(01)\nexit 3`));
    assert.equal(readFileSync(trace, "utf8"), "down 01\n", "the EXIT trap brought the block's room down");
    // -aN: the second attempt of a block names its run and logs.
    const second = r1bash(dir, ["block_start", "down_all", "json_obj", "room_settings"], `${stub}\nblock_start starts\nblock_start starts\nroom_settings 04 start -s01\necho "$ATT"`);
    assert.equal(second.trim(), "-a2");
    const s = JSON.parse(readFileSync(join(dir, "run", "room-04.json"), "utf8"));
    assert.deepEqual([s.run, s.log], ["R1-starts-04-s01-a2", "/var/lib/npt/rooms/room-04-s01-a2.jsonl"]);
    assert.match(readFileSync(join(dir, "results", "starts", "attempts"), "utf8"), /^attempt 1 T\nattempt 2 T\n$/);
    // A done block is refused unless its folder holds an INVALID file.
    mkdirSync(join(dir, "results", "chain"), { recursive: true });
    writeFileSync(join(dir, "results", "chain", "headers.done"), "");
    assert.throws(() => r1bash(dir, ["block_start", "down_all"], `${stub}\nblock_start headers`), /headers is done: a run is never re-run unless INVALID/);
    writeFileSync(join(dir, "results", "headers", "INVALID-ready.txt"), "");
    r1bash(dir, ["block_start", "down_all"], `${stub}\nblock_start headers`);
    // busy: a live holder refuses every block but s14; another chain running refuses too.
    writeFileSync(join(dir, "run", "room-05.holder"), `${process.pid}\n`);
    assert.equal(r1bash(dir, ["busy"], "busy server headers").trim(), "room room-05 is up");
    assert.equal(r1bash(dir, ["busy"], "busy server s14").trim(), "");
    rmSync(join(dir, "run", "room-05.holder"));
    writeFileSync(join(dir, "results", "chain", "bites.chain.log"), "");
    assert.equal(r1bash(dir, ["busy"], "busy server headers").trim(), "the bites chain runs");
    writeFileSync(join(dir, "results", "chain", "bites.ended"), "");
    assert.equal(r1bash(dir, ["busy"], "busy server headers").trim(), "");
  });
  const run = fn("chain_run");
  assert.match(run, /why=\$\(busy "\$name" "\$b"\)\n {4}if \[ -n "\$why" \]; then\n {6}printf '%s stopped: %s %s\\n'/, "recorded why");
  assert.match(run, /trap "printf 'chain \$name ended %s\\\\n'/, "a chain always writes its end");
  assert.match(fn("deadline_ok"), /4 \* 3600 - 1200\)\) \|\| return 1\n {2}case \$\{1:-\} in headers \| probe \| calibrate \| ds-selftest\) \(\(now < lg \+ 4 \* 3600 - 1200\)\)/);
  assert.match(R1, /^case \$1 in bringup \| headers \| probe \| calibrate \| starts \| ds-selftest \| s14 \| bites\) block_start "\$1" ;; esac$/m);
});

test("2.12 NC-SB-7: LG's npt-log header fields equal its room's; every LG block keeps LG's npt-log", () => {
  withTemp((dir) => {
    for (const [nn, kind, mode, tr] of [["01", "headers", "run", "wt"], ["02", "probe", "probe", "wt"], ["03", "calibrate", "calibrate", "wt"], ["05", "ds", "run", "ws"]]) {
      prep(dir);
      writeFileSync(join(dir, "hash", `room-${nn}.json`), JSON.stringify({ value_hex: "ab".repeat(32) }));
      const l = JSON.parse(r1bash(dir, ["json_obj", "room_settings", "load_settings"], `room_settings ${nn} ${kind}\nload_settings ${nn} ${mode} ${tr} 48 330`));
      const s = JSON.parse(readFileSync(join(dir, "run", `room-${nn}.json`), "utf8"));
      for (const k of ["run", "set", "role", "layer", "profile", "scene", "tick_hz", "date", "marks", "commit", "seed"]) {
        assert.deepEqual(l[k], s[k], `${kind}: ${k} from the room's settings`);
      }
      assert.equal(l.log_path, `/var/lib/npt-load/logs/load-${nn}.jsonl`);
    }
  });
  for (const [b, nn] of [["headers", "01"], ["probe", "02"], ["calibrate", "03"], ["ds_selftest", "05"]]) {
    assert.ok(fn(b).includes(`\n  lg_keep "$d" ${nn}\n`), `${b} keeps LG's npt-log`);
  }
  assert.match(fn("lg_keep"), /lg "npt-log \$2" > "\$RUN\/load-\$2\.fetch" \|\| true\n {2}save_log "\$RUN\/load-\$2\.fetch" "\$1" "load-\$2\$ATT\.jsonl"/);
  assert.deepEqual(R1.match(/lg "npt-log/g), ['lg "npt-log'], "one LG fetch, lg_keep's (NC-R2-1)");
  assert.match(text("sbin/npt-load-ctl"), /\n {4}rm -f -- "\$BASE\/logs\/load-\$NN\.jsonl" {3}# a re-run's fresh npt-log/);
});

test("2.14 NC-SB-12: a VOID or INVALID S14 runs once more only if it ends 5 minutes before the bites, else on R2", () => {
  withTemp((dir) => {
    prep(dir);
    const s14 = (standing, ago) => {
      rmSync(join(dir, "results", "s14"), { recursive: true, force: true });
      const stub = `${consts("S14_S")}\nstamp() { echo T; }\nBOOT=$(($(date +%s) - ${ago}))
uptime() { date -d "@$BOOT" '+%F %T'; }
s14_run() { mkdir -p "$1"; echo "$1" >> "${dir}/runs"; echo "run1: delivered 1 of 1; tcp_info {}; ${standing}: why" > "$1/summary.txt"; }`;
      rmSync(join(dir, "runs"), { force: true });
      r1bash(dir, ["out", "s14"], `${stub}\ns14`);
      return readFileSync(join(dir, "runs"), "utf8").trim().split("\n").map((p) => p.slice(dir.length));
    };
    assert.deepEqual(s14("VOID", 0), ["/results/s14", "/results/s14/rerun"], "time left: one more run, both kept");
    assert.ok(readdirSync(join(dir, "results", "s14")).includes("INVALID-s14.txt"));
    assert.deepEqual(s14("INVALID", 12000), ["/results/s14"], "too late: no re-run");
    assert.match(readFileSync(join(dir, "results", "s14", "rerun-R2.txt"), "utf8"), /on R2's RS, a new folder \(R1's kept\)/);
    assert.deepEqual(s14("judged", 0), ["/results/s14"], "a judged run is never re-run here (a MISS needs the model)");
  });
});

test("2.15 NC-SB-5, -6: the probe lists PM-0001-3 only; -16 in every kind but probe and bite", () => {
  withTemp((dir) => {
    for (const k of KINDS) {
      const { marks } = settingsFor(dir, k);
      if (k[1] === "probe") assert.deepEqual(marks, ["PM-0001-3"], "the probe: -3 only");
      else if (k[1].startsWith("bite:")) assert.deepEqual(marks, ["PM-0001-16"], "a bite: its own mark (double-load's is -16)");
      else assert.ok(marks.includes("PM-0001-16"), `${k[1]}: -16, every room start`);
    }
  });
  const plan = text("runs/R1.md");
  assert.match(plan, /\| L2 probe with ImpairSocket's self-test \(0 added hours\) \| 0\.20 \| -3; conditions C-U8-2, C-U8-4, C-U8-6 \|/);
  assert.match(plan, /\| Headers \| 0\.15 \| -1, -2, PM-0006-11, -16 \|/);
});

test("2.19 SEC-SB-8: a sudo list with one line beyond sudoers.d/npt's nine stops bring-up", () => {
  withTemp((dir) => {
    const sudoers = join(OPS, "sbin", "sudoers.d", "npt");
    const nine = readFileSync(sudoers, "utf8").split("\n").filter((l) => l.startsWith("npt ALL=(root) NOPASSWD: "))
      .map((l) => `    (root) NOPASSWD: ${l.split("NOPASSWD: ")[1]}`);
    assert.equal(nine.length, 9);
    const list = (lines) => {
      const f = join(dir, "sudo-l.txt");
      writeFileSync(f, `Matching Defaults entries for npt on rs:\n    env_reset, !setenv\n\nUser npt may run the following commands on rs:\n${lines.join("\n")}\n`);
      try {
        return { ok: true, out: r1bash(dir, ["sudo_exact"], `sudo_exact "${f}" "${sudoers}"`) };
      } catch (e) {
        return { ok: false, out: String(e.stdout) };
      }
    };
    assert.deepEqual(list(nine), { ok: true, out: "exactly the 9 command lines\n" });
    const extra = list([...nine, "    (root) NOPASSWD: /usr/bin/systemctl"]);
    assert.deepEqual(extra, { ok: false, out: "extra: (root) NOPASSWD: /usr/bin/systemctl\n" }, "one line beyond the nine");
    assert.equal(list(nine.slice(1)).ok, false, "one missing");
    assert.match(list([...nine.slice(0, 8), "    (ALL : ALL) ALL"]).out, /extra: \(ALL : ALL\) ALL/);
    assert.equal(list([`    (root) NOPASSWD: ${nine.map((l) => l.split("NOPASSWD: ")[1]).join(", ")}`]).ok, true, "one merged line read alike");
    // SEC-R2-2: the list as sudo 1.9.15p5 writes it to a regular file, lbuf.c's wrap at 80 columns (the break at the
    // last space that fits, continuations indented 8 and given 72): the peers and rooms lines split, so it must stop.
    const wrap = (line, cols = 80, indent = 8) => {
      const out = [];
      for (let cp = 0, have = cols; ; have = cols - indent) {
        let need = line.length - cp;
        let ep = -1;
        if (need > have) {
          ep = line.lastIndexOf(" ", cp + have - 1);
          if (ep < cp) ep = line.indexOf(" ", cp + have);
          if (ep >= 0) need = ep - cp;
        }
        out.push(`${cp ? " ".repeat(indent) : ""}${line.slice(cp, cp + need)}`);
        if (ep < 0) return out;
        for (cp = ep; line[cp] === " "; cp++);
      }
    };
    // The model against this container's sudo 1.9.15p5 (u8-build.md 3.3): its Defaults line (indent 4), into a file.
    const sp = String.raw`secure_path=/usr/local/sbin\:/usr/local/bin\:/usr/sbin\:/usr/bin\:/sbin\:/bin\:/snap/bin,`;
    assert.deepEqual(wrap(`    env_reset, mail_badpass, ${sp} use_pty`, 80, 4), ["    env_reset, mail_badpass,", `    ${sp}`, "    use_pty"]);
    const wrapped = nine.flatMap((l) => wrap(l));
    assert.equal(wrapped.length, 11, "two of the nine are over 80 columns: peers and rooms");
    assert.equal(list(wrapped).ok, false, "a wrapped list stops bring-up");
  });
  assert.match(fn("bringup"), /\n {2}sudo -n -l \| cat > "\$d\/sudo-l\.txt"\n/, "SEC-R2-2: the list through a pipe, never wrapped");
  assert.match(fn("bringup"), /sudo_exact "\$d\/sudo-l\.txt" "\$SPIKE\/ops\/sbin\/sudoers\.d\/npt" > "\$d\/sudo-exact\.txt" \\\n {4}\|\| die/);
});

test("2.20 NC-SB-13, -15: room_up refuses a start followed by room_exit; first_cpu_model trimmed at both ends", () => {
  withTemp((dir) => {
    prep(dir);
    const up = (exit) => r1bash(dir, ["room_up"], `mkfifo() { :; }\nsetsid() { :; }\nsleep() { :; }
room_down() { echo "down $1"; }\nlogq() { [ "$2" = room_start ] && echo 2100000000 || echo ${exit}; }\nroom_up 04 && echo OPEN`);
    assert.equal(up("").trim(), "OPEN", "no room_exit a second after room_start: open");
    assert.throws(() => up(1), /die: room 04 ended at its start/, "a deadline kill or an early exit");
    mkdirSync(join(dir, "results", "bringup"), { recursive: true });
    writeFileSync(join(dir, "results", "bringup", "facts.txt"), "model   AMD EPYC Processor  \t\n");
    r1bash(dir, ["json_obj", "room_settings"], "room_settings 01 headers");
    assert.equal(JSON.parse(readFileSync(join(dir, "run", "room-01.json"), "utf8")).first_cpu_model, "AMD EPYC Processor");
  });
});

test("2.1 ADV-SB-1: one cut log makes collect judge each block folder apart and name the file", () => {
  withTemp((dir) => {
    prep(dir);
    mkdirSync(join(dir, "bin"));
    // npt-judge's stand-in: like the real one, a line cut inside fails the whole read, naming its file (exit 1).
    writeFileSync(join(dir, "bin", "npt-judge"), JUDGE_STUB, { mode: 0o755 });
    const log = (cut) => `{"k":"header"}\n${cut ? '{"k":"foo' : '{"k":"footer","records":1,"ended":"clean"}\n'}`;
    for (const [b, f, cut] of [["headers", "room-01.jsonl", false], ["probe", "room-02-P0.jsonl", true], ["starts", "super-04-s01.jsonl", false]]) {
      mkdirSync(join(dir, "results", b), { recursive: true });
      writeFileSync(join(dir, "results", b, f), log(cut));
    }
    const run = () => r1bash(dir, ["out", "collect"], `BIN="${dir}/bin" SPIKE="${dir}"\ncollect`);
    run();
    const c = join(dir, "results", "collect");
    assert.match(readFileSync(join(c, "split.txt"), "utf8"), new RegExp(`npt-judge: ${dir}/results/probe/room-02-P0\\.jsonl: line 2`));
    for (const b of ["headers", "starts"]) assert.equal(readFileSync(join(c, `verdicts-${b}.json`), "utf8"), '{"verdicts": []}\n');
    assert.match(readFileSync(join(c, "judge-probe.txt"), "utf8"), /room-02-P0\.jsonl: line 2: not one JSON record/);
    writeFileSync(join(dir, "results", "probe", "room-02-P0.jsonl"), log(false));
    rmSync(c, { recursive: true });
    run();
    assert.ok(readFileSync(join(c, "verdicts.json"), "utf8").length > 0);
    assert.throws(() => readFileSync(join(c, "split.txt")), /ENOENT/, "no split when the whole tree reads");
  });
});

// ---- pass 3 (advisor-u8-r2.md section 3) -------------------------------------------------------------------------
// npt-judge's own PM-0001-4 fixture logs (npt-log's format), as the copies save_log is given.
const FIX4 = join(OPS, "..", "crates", "judge", "fixtures", "pm-0001-4", "base");
const LG2 = readFileSync(join(FIX4, "s30-2-lg.jsonl"), "utf8");

test("3.1 NC-R2-1: save_log keeps a whole log as it is, a cut line's whole lines beside .raw, an empty one as .empty", () => {
  withTemp((dir) => {
    const [src, d] = [join(dir, "src"), join(dir, "out")];
    mkdirSync(src);
    mkdirSync(d);
    const lines = LG2.split("\n").slice(0, -1);
    const cases = { "whole.jsonl": LG2, "line-end.jsonl": `${lines.slice(0, 3).join("\n")}\n`, "in-line.jsonl": LG2.slice(0, -20),
      "empty.jsonl": "", "no-header.jsonl": `${lines.slice(1).join("\n")}\n`, "in-header.jsonl": lines[0].slice(0, 40) };
    for (const [f, body] of Object.entries(cases)) writeFileSync(join(src, f), body);
    r1bash(dir, ["save_log"], Object.keys(cases).map((f) => `save_log "${join(src, f)}" "${d}"`).join("\n"));
    const got = (f) => readFileSync(join(d, f), "utf8");
    assert.equal(got("whole.jsonl"), LG2);
    assert.equal(got("line-end.jsonl"), cases["line-end.jsonl"], "cut at a line end: as it is (the judge reads it cut short)");
    const prefix = got("in-line.jsonl").split("\n");
    assert.deepEqual(prefix, [...lines.slice(0, 3), ""], "cut inside a line: the whole lines, ending at a newline");
    prefix.slice(0, -1).forEach((l) => JSON.parse(l));
    assert.equal(got("in-line.jsonl.raw"), cases["in-line.jsonl"], "the whole file beside it");
    for (const f of ["empty", "no-header", "in-header"]) assert.equal(got(`${f}.jsonl.empty`), cases[`${f}.jsonl`], f);
    assert.deepEqual(readdirSync(d).filter((f) => f.endsWith(".jsonl")).sort(), ["in-line.jsonl", "line-end.jsonl", "whole.jsonl"]);
    assert.deepEqual(got("cut.txt").split("\n").map((l) => l.split(":")[0]), ["in-line.jsonl", "empty.jsonl", "no-header.jsonl", "in-header.jsonl", ""]);
  });
});

test("3.1 NC-R2-1: the probe block waits for LG's end before any lg stop and keeps LG's cut copy through save_log", () => {
  withTemp((dir) => {
    const trace = join(dir, "trace");
    writeFileSync(join(dir, "cut"), LG2.slice(0, -20));
    const stubs = `out() { mkdir -p "$R/$1"; printf '%s\\n' "$R/$1"; }
timing_invalid() { :; }; stat_cpu() { :; }; room_settings() { :; }; load_settings() { :; }; probe_presence() { :; }
setting() { echo R1-probe-02; }; logq() { echo 0; }; sleep() { :; }
room_up() { echo "room_up $*" >> ${trace}; }; room_down() { echo "room_down $*" >> ${trace}; }
load_wait() { echo "load_wait $*" >> ${trace}; }
lg() { echo "lg $1" >> ${trace}; [ "$1" != "npt-log 02" ] || cat "${dir}/cut"; }`;
    r1bash(dir, ["save_log", "keep", "lg_keep", "probe"], `${stubs}\nprobe > /dev/null`);
    const calls = readFileSync(trace, "utf8").trim().split("\n");
    assert.ok(calls.indexOf("load_wait 02 120") > calls.indexOf("lg start 02"), "LG's own end awaited after the probe's exit");
    assert.ok(calls.indexOf("load_wait 02 120") < calls.indexOf("lg npt-log 02") && !calls.includes("lg stop 02"));
    const p = join(dir, "results", "probe");
    assert.equal(readFileSync(join(p, "load-02.jsonl"), "utf8"), `${LG2.split("\n").slice(0, 3).join("\n")}\n`, "the whole lines");
    assert.equal(readFileSync(join(p, "load-02.jsonl.raw"), "utf8"), LG2.slice(0, -20));
    assert.match(readFileSync(join(p, "cut.txt"), "utf8"), /^load-02\.jsonl: cut inside a line: its 3 whole lines kept/);
  });
});

test("3.1 NC-R2-1: a results tree with a cut and an empty LG copy reads whole; NPT_R1_TREE keeps it for npt-judge", () => {
  withTemp((dir) => {
    const tree = process.env.NPT_R1_TREE ?? join(dir, "tree");
    assert.ok(!existsSync(tree), "NPT_R1_TREE names a new folder");
    const [src, d, bad] = [join(dir, "src"), join(tree, "s30"), join(dir, "bad")];
    for (const f of [src, d, bad]) mkdirSync(f, { recursive: true });
    for (const f of readdirSync(FIX4)) writeFileSync(join(src, f), f === "s30-2-lg.jsonl" ? LG2.slice(0, -20) : readFileSync(join(FIX4, f)));
    writeFileSync(join(src, "load-07.jsonl"), "");
    r1bash(dir, ["save_log"], readdirSync(src).sort().map((f) => `save_log "${join(src, f)}" "${d}"`).join("\n"));
    writeFileSync(join(dir, "npt-judge"), JUDGE_STUB, { mode: 0o755 });
    const judge = (logs) => execFileSync(join(dir, "npt-judge"), ["--logs", logs, "--out", join(dir, "v.json")], { encoding: "utf8" });
    judge(tree);
    assert.deepEqual(readFileSync(join(d, "cut.txt"), "utf8").split("\n").map((l) => l.split(":")[0]), ["load-07.jsonl", "s30-2-lg.jsonl", ""]);
    const kept = readFileSync(join(d, "s30-2-lg.jsonl"), "utf8").trim().split("\n");
    assert.notEqual(JSON.parse(kept.at(-1)).k, "footer", "run s30-2's LG log has no footer: INVALID by the complete rule");
    writeFileSync(join(bad, "s30-2-lg.jsonl"), LG2.slice(0, -20));
    assert.throws(() => judge(bad), (e) => /s30-2-lg\.jsonl: line 4: not one JSON record/.test(e.stdout), "the copy as fetched fails");
  });
});

test("3.8 NC-R2-6: LG's stdout copy carries -aN; a block that dies after LG's start stops LG and keeps its copies", () => {
  withTemp((dir) => {
    const trace = join(dir, "trace");
    const stub = `stamp() { echo T; }\nroom_down() { :; }\nlg() { echo "lg $1" >> ${trace}; [ "$1" != "npt-log 03" ] || head -n 3 "${join(FIX4, "s30-1-lg.jsonl")}"; }`;
    const run = `block_start calibrate\nblock_start calibrate\nLGK=("$R/calibrate" 03)\ndie "a block dies"`;
    assert.throws(() => r1bash(dir, ["block_start", "down_all", "save_log", "lg_keep"], `${stub}\n${run}`), /a block dies/);
    assert.deepEqual(readFileSync(trace, "utf8").trim().split("\n"), ["lg stop 03", "lg log 03", "lg npt-log 03"], "on exit");
    const kept = readdirSync(join(dir, "results", "calibrate")).filter((f) => f.startsWith("load-")).sort();
    assert.deepEqual(kept, ["load-03-a2.jsonl", "load-03-a2.log"], "the second attempt's names: no copy overwritten");
  });
});

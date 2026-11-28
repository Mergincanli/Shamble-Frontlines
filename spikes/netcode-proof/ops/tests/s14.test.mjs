// S14's rig (u8-brief.md rev 1.1 section 1 Block 4): compare.py on fixture kernel runs and fixture KCMODEL lines, the
// burst recovery by advisor-u3q.md:130-133's (e) definition, and the schedule both scripts share with the knob.
import assert from "node:assert/strict";
import { execFileSync, spawnSync } from "node:child_process";
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { test } from "node:test";
import { OPS, text, withTemp } from "./helpers.mjs";

const PROBE = join(OPS, "runs", "s14", "probe.py");
const COMPARE = join(OPS, "runs", "s14", "compare.py");
const GAP = 33_333_333;
const N = 28500;
const BURSTS = Array.from({ length: 34 }, (_, k) => [(600 + 10 * k) * 1e9, 1e9]);
const BASE = 50_050_000;
const STALLS = [100, 400, 700, 1000, 1300, 1600];

const py = (...args) => execFileSync("python3", ["-I", ...args], { encoding: "utf8" });

/** One run's delays: steady stalls of length L at STALLS, and each burst's backlog delivered at return + rec. */
function delays(sentOf, L, rec, bursts) {
  const d = Array.from({ length: N }, () => BASE);
  for (const k of STALLS) d[k] = BASE + L;
  for (const [s, l] of bursts) {
    const ret = s + l;
    const k0 = Array.from({ length: N }, (_, k) => k).find((k) => sentOf(k) >= ret);
    for (let k = 0; k < N; k++) {
      const t = sentOf(k);
      if (t >= s && (k <= k0 || t < ret + rec)) d[k] = Math.max(BASE, ret + rec - t);
    }
  }
  return d;
}

function writeKernel(dir, { L = 133_400_000, rec = 1.1e9, rttUs = 100_100, retrans = 50, steady = 30 } = {}) {
  for (const run of [1, 2, 3]) {
    const first = 1e12 + run * 1e11;
    const late = 100_000;
    const sentAbs = (k) => first + k * GAP + late;
    const d = delays((k) => k * GAP + late, L, rec, BURSTS);
    writeFileSync(join(dir, `server-run${run}.json`), JSON.stringify({
      role: "server", messages: N, delivered: N, cc_server: "cubic", cc_errors: [],
      sent_ns: d.map((_, k) => sentAbs(k)), delay_ns: d,
    }));
    writeFileSync(join(dir, `client-run${run}.json`), JSON.stringify({
      role: "client", messages: N, first_due_ns: first, period_ns: GAP, cc_client: "cubic", cc_errors: [],
      send_late_ns: Array(N).fill(late),
      tcp_info_client: { rto_us: 300000, rtt_us: rttUs, rttvar_us: 500, snd_cwnd: 10, total_retrans: retrans, total_retrans_steady: steady },
    }));
    writeFileSync(join(dir, `bursts-run${run}.json`), JSON.stringify({ schedule: BURSTS, actual: BURSTS, tc_call_ns: [] }));
  }
}

function writeModel(dir, { L = 133_300_000, rec = 1.05e9, withBursts = true, seeds = 4 } = {}) {
  const lines = [];
  for (let seed = 1; seed <= seeds; seed++) {
    const d = delays((k) => k * GAP, L, rec, BURSTS);
    const late = d.map((x, k) => [k, x]).filter(([, x]) => x > BASE + 1_000_000);
    const m = { seed, rtt_ns: 100_100_000, n: N, p50_ns: BASE, late, segments: N, repairs: [6, 0, 0] };
    if (withBursts) m.bursts = BURSTS;
    lines.push(`KCMODEL ${JSON.stringify(m)}`);
  }
  writeFileSync(join(dir, "model.txt"), `${lines.join("\n")}\n`);
}

function judge(dir) {
  const out = py(COMPARE, "judge", dir);
  return { out, v: JSON.parse(readFileSync(join(dir, "verdict.json"), "utf8")) };
}

test("both scripts share the schedule, in the knob's form (start_ns:len_ns from the first message)", () => {
  const want = BURSTS.map(([s, l]) => `${s}:${l}`).join(",");
  assert.equal(py(PROBE, "schedule").trim(), want);
  assert.equal(py(COMPARE, "schedule").trim(), want);
  assert.equal(py(PROBE, "messages").trim(), String(N));
});

test("agreeing kernel and model: PASS on both halves; recovery exactly as defined", () => {
  withTemp((dir) => {
    writeKernel(dir);
    writeModel(dir);
    const { out, v } = judge(dir);
    assert.equal(v.verdict, "PASS", out);
    assert.equal(v.stall_half, "PASS");
    assert.equal(v.burst_half, "PASS");
    assert.equal(v.burst.kernel_p50_ns, 1.1e9, "return to the in-order delivery of the first message written at or after it");
    assert.equal(v.burst.model_p50_ns, 1.05e9);
    assert.equal(v.burst.kernel_n, 102);
    assert.equal(v.burst.model_n, 136);
    assert.equal(v.stall.kernel_n, 18, "steady episodes only (6 per run, 3 runs)");
    assert.equal(v.model_recoveries_approximate, 0);
    assert.match(py(COMPARE, "env", dir), /^NPT_KC_RTT_NS=100100000 NPT_KC_MESSAGES=28500 NPT_KC_RUNS=100 NPT_KC_BURSTS=600000000000:1000000000,/);
  });
});

test("2.5 NC-SB-4: one schedule in both files; at least 1,000 expected steady episodes and 101 recoveries pooled", () => {
  const probe = readFileSync(PROBE, "utf8");
  const compare = readFileSync(COMPARE, "utf8");
  const runs = Number(readFileSync(join(OPS, "sbin", "npt-s14"), "utf8").match(/ RUNS=(\d+) /)[1]);
  const steadyS = Number(probe.match(/^STEADY_S = (\d+)$/m)[1]);
  const steadyEnd = Number(compare.match(/^STEADY_END_NS = ([\d_]+)$/m)[1].replaceAll("_", ""));
  assert.equal(py(PROBE, "schedule").trim(), py(COMPARE, "schedule").trim(), "one schedule");
  assert.equal(steadyEnd, (steadyS - 1) * 1e9, "stalls over messages written before the first burst less 1 s");
  const steadyMsgs = Math.ceil(steadyEnd / GAP);
  const episodes = runs * steadyMsgs * 0.02;
  const recoveries = runs * py(PROBE, "schedule").trim().split(",").length;
  assert.ok(episodes >= 1000, `about ${Math.round(episodes)} expected steady episodes pooled (${runs} runs x ${steadyMsgs} x 2 %)`);
  assert.ok(recoveries >= 101, `${recoveries} recoveries pooled, so the burst p99 is a rank, not the maximum`);
});

test("model lines without the burst schedule: the burst half reads NOT-RUN, never PASS", () => {
  withTemp((dir) => {
    writeKernel(dir);
    writeModel(dir, { withBursts: false });
    const { out, v } = judge(dir);
    assert.equal(v.burst_half, "NOT-RUN");
    assert.equal(v.verdict, "PARTIAL", out);
    assert.match(out, /comes to the advisor|to the advisor before -18's WebSocket part counts/);
  });
});

test("a kernel recovery 24 % slower than the model's: MISS, netem wins", () => {
  withTemp((dir) => {
    writeKernel(dir, { rec: 1.3e9 });
    writeModel(dir);
    const { out, v } = judge(dir);
    assert.equal(v.burst_half, "MISS");
    assert.equal(v.verdict, "MISS");
    assert.match(out, /netem wins \(TCP-FREEZE\)/);
  });
});

test("a stall 15 % longer than the model's: MISS on the stall half", () => {
  withTemp((dir) => {
    writeKernel(dir, { L: 153_300_000 });
    writeModel(dir);
    assert.equal(judge(dir).v.stall_half, "MISS");
  });
});

test("tcpi_rtt of 80 ms (netem's 100 ms not in place): INVALID", () => {
  withTemp((dir) => {
    writeKernel(dir, { rttUs: 80_000 });
    writeModel(dir);
    const { out, v } = judge(dir);
    assert.equal(v.verdict, "INVALID", out);
  });
});

test("no retransmission: VOID; no model line: INVALID", () => {
  withTemp((dir) => {
    writeKernel(dir, { retrans: 0 });
    writeModel(dir);
    assert.equal(judge(dir).v.verdict, "VOID");
  });
  withTemp((dir) => {
    writeKernel(dir);
    assert.equal(judge(dir).v.verdict, "INVALID");
  });
});

test("2.14 NC-SB-12: steady episodes beyond the steady window's retransmissions read VOID, the bursts' notwithstanding", () => {
  withTemp((dir) => {
    writeKernel(dir, { retrans: 50, steady: 3 });
    writeModel(dir);
    const { out, v } = judge(dir);
    assert.equal(v.verdict, "VOID", out);
    assert.match(out, /6 steady episodes > 3 retransmissions in the steady window \(the noise guard\)/);
    assert.match(out, /re-run the same day only if it ends 5 minutes before the bites' start/);
    const summary = py(COMPARE, "summary", dir);
    assert.match(summary, /run1: .*VOID: 6 steady episodes > 3/);
    const grep = readFileSync(join(OPS, "runs", "r1.sh"), "utf8").match(/if grep -Eq '([^']+)' "\$d\/summary\.txt"; then/)[1];
    assert.match(summary, new RegExp(grep), "r1.sh's s14 block sees the VOID in RS's summary");
  });
  withTemp((dir) => {
    writeKernel(dir, { steady: null });
    writeModel(dir);
    assert.match(judge(dir).out, /INVALID \(the steady window's retransmissions were not read\)/);
  });
  assert.match(readFileSync(PROBE, "utf8"), /if i == steady_end:\n {12}# NC-SB-12[^\n]*\n {12}steady_retrans = struct\.unpack_from\("<24I", cs\.getsockopt/);
});

test("2.20 NC-SB-17: each direction's burst switch recorded; `actual` (the validity window) is the data direction's", () => {
  const p = readFileSync(PROBE, "utf8");
  assert.match(p, /LINKS = \[\("npt-s14c", "s14c", "loss 2%"\), \("npt-s14s", "s14s", ""\)\]/, "the data direction (client egress) first");
  assert.match(p, /actual\.append\(\[on\[0\]\[1\] - first, off\[0\]\[1\] - on\[0\]\[1\]\]\)\n {8}reverse\.append\(\[on\[1\]\[1\] - first/);
  assert.match(p, /"actual_reverse": reverse/);
  assert.match(readFileSync(COMPARE, "utf8"), /for \(s0, l0\), \(s1, l1\) in zip\(BURSTS, b\["actual"\]\):/, "the validity rule reads the data direction");
});

test("probe.py's bursts mode refuses a first-message stamp not within 10 s of now (no tc runs)", () => {
  const r = spawnSync("python3", ["-I", PROBE, "bursts", "1"], { encoding: "utf8" });
  assert.equal(r.status, 2);
  assert.match(r.stderr, /refused/);
  assert.equal(spawnSync("python3", ["-I", PROBE, "client", "x"], { encoding: "utf8" }).status, 2);
});

// S14-PUSH (a)'s second condition: R1's push carries the s14 folder whole, so what RS writes into it is pinned: its
// files, kernel.txt's sources, probe.py's JSON keys, its two addresses (compare.py's judge runs here, after the push).
test("S14-PUSH (a): the s14 folder's writers, kernel.txt's sources, probe.py's keys and its two addresses, pinned", () => {
  const go = (what) => `${what}: a change that puts anything more into the pushed s14 folder stops and goes to Mustafa first`;
  const only = (what, got, want) => assert.deepEqual([...new Set(got)].filter((x) => !want.includes(x)), [], go(what));
  const npt = text("sbin/npt-s14"), probe = text("runs/s14/probe.py"), r1 = text("runs/r1.sh"), cmp = text("runs/s14/compare.py");
  const kernel = npt.slice(npt.indexOf("\n{\n"), npt.indexOf("\nfor run in")) + npt.split("\n").filter((l) => l.includes('>> "$OUT/kernel.txt"')).join("\n");
  const block = r1.slice(r1.indexOf("\ns14_run() {"), r1.indexOf("\nbites() {")) + r1.match(/\nblock_start\(\) \{[^]*?\n\}\n/)[0], stamps = r1.split("\n").filter((l) => l.includes("stamps.txt") && !/^\s*#/.test(l));
  only("npt-s14's files", [...npt.matchAll(/"\$OUT\/([^"]*)"/g)].map((m) => m[1]), ["kernel.txt", "server-run$run.json", "client-run$run.json", "bursts-run$run.json"]);
  only("npt-s14's $OUT lines", npt.split("\n").map((l) => l.trim()).filter((l) => /\$\{?OUT\b|"\$PROBE" (?:server|client|bursts)/.test(l)), ['} > "$OUT/kernel.txt"',
    `printf 'loadavg_before_run%s %s\\n' "$run" "$(cat /proc/loadavg)" >> "$OUT/kernel.txt"`, `printf 'loadavg_after_run%s %s\\n' "$run" "$(cat /proc/loadavg)" >> "$OUT/kernel.txt"`,
    'ip netns exec "$NSS" "${SETPRIV_NPT[@]}" /usr/bin/python3 -I "$PROBE" server "$PORT" "$N" \\', '| head -c "$CAP" > "$OUT/server-run$run.json" &',
    'coproc CLIENT { exec ip netns exec "$NSC" "${SETPRIV_NPT[@]}" /usr/bin/python3 -I "$PROBE" client 198.51.100.2 "$PORT" "$N"; }',
    'python3 -I "$PROBE" bursts "${BASH_REMATCH[1]}" > "$OUT/bursts-run$run.json"', 'head -c "$CAP" <&"${CLIENT[0]}" > "$OUT/client-run$run.json"',
    '[ ! -e "$OUT" ] || die "$OUT exists already"', 'install -d -o root -g root -m 0755 "$OUT"', `printf 'npt-s14: %s runs done; results in %s\\n' "$RUNS" "$OUT"`]);
  const summary = cmp.slice(cmp.indexOf('elif cmd == "summary"'), cmp.indexOf('elif cmd == "judge"')), standing = cmp.slice(cmp.indexOf("def standing("), cmp.indexOf("def model_lines("));
  assert.deepEqual([summary.match(/\bprint\(/g).length, cmp.match(/^import \w+$/gm)], [1, ["import glob", "import json", "import math", "import os", "import sys"]], go("compare.py summary's one print"));
  only("compare.py summary's values (summary.txt on RS)", [...(summary + standing).matchAll(/\{([^{}]+)\}/g)].map((m) => m[1]), ["name", "s['delivered']", "s['messages']",
    "base", "len(eps)", "len(rec)", "lat99", "c['tcp_info_client']", "kind or 'judged'", "why", "rtt", "len(BURSTS)", "s1", "l1", "s0", "l0", "steady"]);
  only("kernel.txt's sources", [...kernel.matchAll(/\$\(([^)]*)\)/g)].map((m) => m[1]).concat(kernel.split("\n").map((l) => l.trim())
    .filter((l) => l && !/^(printf |for s in |tcp_dsack |done$|\{$|\} > )/.test(l))), ["uname -r", "date -u +%Y-%m-%dT%H:%M:%SZ", "cat /proc/loadavg",
    'ip netns exec "$NSC" cat "/proc/sys/net/ipv4/$s" 2> /dev/null || echo absent', 'tc -n "$NSC" qdisc show dev s14c', 'tc -n "$NSS" qdisc show dev s14s']);
  only("kernel.txt's variables", [...kernel.matchAll(/\$\{?([A-Za-z_]\w*)/g)].map((m) => m[1]), ["N", "RUNS", "s", "NSC", "NSS", "run", "OUT"]);
  assert.deepEqual([npt.match(/for s in /g).length, npt.match(/for s in ([^;]+); do/)[1].split(/[\s\\]+/)], [1, ["tcp_recovery", "tcp_early_retrans", "tcp_sack", "tcp_frto", "tcp_rto_min_us",
    "tcp_thin_linear_timeouts", "tcp_timestamps", "tcp_dsack", "tcp_congestion_control", "tcp_available_congestion_control"]], go("the listed net.ipv4 settings"));
  only("r1.sh's writes into the folder", block.split("\n").map((l) => l.trim()).filter((l) => /^[^#]*(?:>|\btee |\bcp |\bmv |\bln |\binstall |\bpython3 |\bnode |s14_run ")/.test(l)), [
    'python3 -I "$SPIKE/ops/runs/s14/compare.py" summary "$1" | tee "$1/summary.txt"', 'cp -rp "$dir"/. "$1/"', 's14_run "$d"', 's14_run "$d/rerun"',
    'cp "$d/summary.txt" "$d/INVALID-s14.txt"', `printf 'no time for a re-run before the bites: on R2'"'"'s RS, a new folder (R1'"'"'s kept)\\n' | tee "$d/rerun-R2.txt"`,
    'cp -p "$R/browsers/stamps.txt" "$d/browser-stamps.txt" 2> /dev/null || true', `printf 'attempt %s %s\\n' "$n" "$(stamp)" >> "$R/$b/attempts"`,
    'if [ -f "$R/chain/$b.done" ] && ! compgen -G "$R/$b/INVALID*" > /dev/null; then',
    `dir=$(sudo -n /usr/local/sbin/npt-s14 run | sed -n 's/^npt-s14: .* results in \\(\\/var\\/lib\\/npt\\/s14\\/[0-9TZ]*\\)$/\\1/p')`]);
  only("the stamps' writers", [...r1.match(/^stamp\(\) .*/gm), ...stamps], ["stamp() { date -u +%Y-%m-%dT%H:%M:%SZ; }", '  cp -p "$R/browsers/stamps.txt" "$d/browser-stamps.txt" 2> /dev/null || true',
    `  printf '%s up %s\\n' "$1" "$(stamp)" >> "$(out browsers)/stamps.txt"`, `  printf '%s down %s\\n' "$1" "$(stamp)" >> "$(out browsers)/stamps.txt"`]);
  only("probe.py's JSON keys", [...probe.matchAll(/(?<!env=\{)"(\w+)": /g), ...probe.match(/^COUNTERS = \{[^]*?^\}/m)[0].matchAll(/"([A-Z]\w+)"/g)].map((m) => m[1]), [
    "role", "port", "messages", "delivered", "cc_server", "cc_errors", "sent_ns", "delay_ns", "first_due_ns", "period_ns", "cc_client", "send_late_ns",
    "tcp_info_client", "rto_us", "rtt_us", "rttvar_us", "snd_cwnd", "total_retrans", "total_retrans_steady", "kernel_counters_before", "kernel_counters_after",
    "schedule", "actual", "actual_reverse", "tc_call_ns", "RetransSegs", "TCPFastRetrans", "TCPLossProbes", "TCPTimeouts", "TCPLostRetransmit"]);
  for (const [name, t] of Object.entries({ "npt-s14": npt, "probe.py": probe, "compare.py": cmp, "r1.sh's s14 block": block, "r1.sh's stamps lines": stamps.join("\n") })) {
    only(`${name}'s addresses, host and interface names`, t.match(/\b\d{1,3}(?:\.\d{1,3}){3}\b|\b(?:[0-9a-f]{2}[:-]){5}[0-9a-f]{2}\b|hostname|nodename|gethost\w*|getfqdn|\buname\b(?! -r\b)|platform\.node|getsockname|getpeername|getifaddrs|if_nameindex|ifconfig|\/sys\/class\/net|\/etc\/host\w*|\b(?:addr|link|neigh) show\b/gi) ?? [], ["198.51.100.1", "198.51.100.2"]);
  }
});

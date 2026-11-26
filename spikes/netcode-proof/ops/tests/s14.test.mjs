// S14's rig (u8-brief.md rev 1.1 section 1 Block 4): compare.py on fixture kernel runs and fixture KCMODEL lines, the
// burst recovery by advisor-u3q.md:130-133's (e) definition, and the schedule both scripts share with the knob.
import assert from "node:assert/strict";
import { execFileSync, spawnSync } from "node:child_process";
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { test } from "node:test";
import { OPS, withTemp } from "./helpers.mjs";

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

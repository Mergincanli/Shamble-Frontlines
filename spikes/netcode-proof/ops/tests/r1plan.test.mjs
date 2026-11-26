// Test 5 (u8-brief.md rev 1.1 section 3; C1, C3, C6): R1.md's arithmetic, recomputed from its own tables and the
// lines and rate it cites; and no block marked away needs a laptop-session command.
import assert from "node:assert/strict";
import { readFileSync, readdirSync, statSync } from "node:fs";
import { join } from "node:path";
import { test } from "node:test";
import { OPS, text } from "./helpers.mjs";

const plan = text("runs/R1.md");
const r1 = text("runs/r1.sh");

/** The rows of the first table after a heading line that starts with `heading`. */
function table(heading) {
  const start = plan.indexOf(heading);
  assert.ok(start >= 0, `no section ${heading}`);
  const lines = plan.slice(start).split("\n");
  const first = lines.findIndex((l) => l.startsWith("|"));
  const rows = [];
  for (const l of lines.slice(first + 2)) {
    if (!l.startsWith("|")) break;
    rows.push(l.split("|").slice(1, -1).map((c) => c.trim()));
  }
  return rows;
}
const num = (s) => Number(String(s).replace(/[*,]/g, "").replace(/^EUR /, ""));
const minutes = (hhmm) => {
  const [h, m] = hhmm.split(":").map(Number);
  return h * 60 + m;
};
const round2 = (x) => Math.round(x * 100) / 100;

const blocks = table("## 1. Blocks and hours");
const body = blocks.filter((r) => !r[0].startsWith("**"));
const planned = blocks.find((r) => r[0] === "**Planned**");

// S14's last part runs beside the browser legs (advisor-u8-stopb.md section 4): hours it adds to no span.
const overlap = Number(plan.match(/its last \*\*([\d.]+) h\*\* runs beside the browser legs/)[1]);
const s14 = num(body.find((r) => r[0] === "S14 veth and netem check")[1]);

test("the block hours sum to the planned total and stay within the 4 billed hours", () => {
  const sum = round2(body.reduce((a, r) => a + num(r[1]), 0) - overlap);
  assert.equal(sum, 3.55);
  assert.equal(num(planned[1]), sum);
  assert.deepEqual([s14, overlap], [0.8, 0.5], "S14 0.80 h, its last 0.50 h beside the browser legs");
  assert.ok((3 * (950 + 5)) / 3600 <= s14, "three 950 s runs and about 5 s of set-up each fit S14's row");
  assert.ok(sum <= 4, "within 4 h");
  assert.match(plan, /2\.95 less Safari's 0\.30 is 2\.65; plus -29's 0\.25 and PM-0006-25's 0\.20 is 3\.10; plus\nDelayStream's 0\.15 and S14's 0\.30 is \*\*3\.55\*\*/);
  assert.equal(round2(2.95 - 0.3 + 0.25 + 0.2 + 0.15 + 0.3), 3.55);
});

test("the cap equals the cap rule's arithmetic (plan.md:795-796)", () => {
  const plannedEur = 4 * 0.1386 + 4 * 0.0176;
  const capRaw = 5 * 0.1386 + 5 * 0.0176;
  assert.equal(round2(plannedEur), 0.62);
  assert.equal(Math.ceil(Math.round(capRaw * 1e6) / 1e5) / 10, 0.8);
  assert.match(plan, /= 0\.5544 \+ 0\.0704 = 0\.6248/);
  assert.match(plan, /= 0\.693 \+ 0\.088 = 0\.781, rounded up to the next 0\.10/);
  assert.match(plan, /\*\*Cap EUR 0\.80:\*\*/);
});

test("the timeline follows from T0 09:30 and the block hours", () => {
  const tl = table("**Thursday 3 December** (T0");
  const h = (pred) => body.filter((r) => pred(r[0])).reduce((a, r) => a + num(r[1]), 0);
  const spans = [
    ["cloud-init's build", h((n) => n.startsWith("Bring-up: cloud-init"))],
    ["Bring-up commands", h((n) => n.startsWith("Bring-up: first"))],
    ["Certificate", h((n) => n.startsWith("Certificate"))],
    ["Server chain, S14 aside", body.filter((r) => r[4] === "chain server" && !r[0].startsWith("S14")).reduce((a, r) => a + num(r[1]), 0)],
    ["S14, its last 0.50 h beside the browser legs", s14, -overlap],
    ["Browser blocks", body.filter((r) => r[4] === "laptop" && !/Bring-up|Certificate/.test(r[0])).reduce((a, r) => a + num(r[1]), 0)],
    ["Bites chain", h((n) => n === "First bites")],
  ];
  let t = minutes("09:30");
  for (const [name, hours, back = 0] of spans) {
    const row = tl.find((r) => r[0] === name);
    assert.ok(row, `timeline row ${name}`);
    assert.equal(minutes(row[1]), t, `${name} starts`);
    t += Math.round(hours * 60);
    assert.equal(minutes(row[2]), t, `${name} ends`);
    t += Math.round(back * 60);
  }
  assert.equal(minutes(tl.find((r) => r[0] === "Browser blocks")[1]), minutes("10:39") + 60, "the browser legs at 11:39, as before");
  assert.ok(t <= minutes("13:30"), "inside RS's delete-by (T0 + 4 h)");
});

test("his sittings: the step minutes sum to the stated total, about 2 h 40 min (C1)", () => {
  const steps = table("| Time | His step | Minutes |");
  const total = steps.reduce((a, r) => a + num(r[2]), 0);
  const sittings = [...plan.matchAll(/(\d\d:\d\d)-(\d\d:\d\d) \((\d+) min\)/g)];
  assert.equal(sittings.length, 3);
  let sum = 0;
  for (const [, a, b, m] of sittings) {
    assert.equal(minutes(b) - minutes(a), Number(m));
    sum += Number(m);
  }
  assert.equal(total, sum);
  assert.match(plan, new RegExp(`\\*\\*${sum} minutes, about 2 h 40 min\\*\\*`));
  assert.ok(Math.abs(sum - 160) <= 5, "C1's likely about 2 h 40 min");
});

test("no block marked away needs a laptop-session command (C1)", () => {
  const chain = r1.match(/^readonly SERVER_CHAIN="([^"]+)"$/m)[1].split(" ");
  const name = {
    Headers: "headers",
    "L2 probe with ImpairSocket's self-test (0 added hours)": "probe",
    Calibration: "calibrate",
    "Room starts": "starts",
    "DelayStream's loaded self-test": "ds-selftest",
    "S14 veth and netem check": "s14",
  };
  const away = body.filter((r) => r[3] === "away");
  for (const r of away) {
    assert.ok(["cloud-init", "chain server"].includes(r[4]), `${r[0]}: away, so started by cloud-init or the chain`);
    if (r[4] === "chain server") assert.ok(chain.includes(name[r[0]]), `${r[0]} is in r1.sh's SERVER_CHAIN`);
  }
  assert.deepEqual(away.filter((r) => r[4] === "chain server").map((r) => name[r[0]]), chain, "the chain is exactly the away blocks, in order");
  for (const r of body.filter((x) => x[4] === "laptop")) assert.equal(r[3], "present", `${r[0]}: a laptop command needs him`);
});

test("the slot table: a slot per attempt, its sum, and r1.sh's rooms (C3; condition 2)", () => {
  const rows = table("## 4. Slot table");
  const rooms = rows.filter((r) => !r[0].startsWith("**"));
  const sum = rooms.reduce((a, r) => a + num(r[2]), 0);
  assert.equal(sum, num(rows.find((r) => r[0] === "**Browser slots**")[2]));
  assert.equal(sum, 8 + 3 * 20 + 1 + 3 * 20 + 1 + 3);
  for (const [room, nn, slots] of rooms) {
    assert.match(r1, new RegExp(`${room}\\) printf '${nn} ${slots}\\\\n'`), `r1.sh's ${room}`);
  }
  assert.ok(plan.includes(
    'U8\'s briefs and R1\'s run plan each state in one line that their tests or runs hold no throwing or rejoining hostile\nclient and no client restarted mid-run, or bring C-U6-19 first. In R1 each browser leg (each browser on each\ntransport, the QUIC-blocked fallback, -36\'s three) joins its own slot once, or the room restarts between legs; the\ntoken file holds a slot for every leg. SA-F1-1 stays open until U6d merges."',
  ), "condition 2's line word for word");
  assert.match(plan, /\*\*R1's line:\*\* R1's runs hold no throwing or rejoining hostile client and no client restarted mid-run/);
});

test("the hash URL is per room, `/npt/cert-hash-NN.json`, and plan.md:995-996 is superseded beside :287 (fix pass item 3)", () => {
  assert.match(plan, /each attempt fetches `\/npt\/cert-hash-NN\.json` \(the page derives NN from its `room` query/);
  assert.match(text("browser/cert14d.mjs"), /the page fetches \/npt\/cert-hash-NN\.json \(NN from its `room` query\)/);
  assert.match(plan, /:287 \(RS's host key[^]*?\); :995-996 \(one hash file for every room/);
  const plain = ["/npt/cert-hash", ".json"].join("");
  const walk = (dir) => readdirSync(dir).flatMap((n) => {
    const p = join(dir, n);
    if (n === "node_modules" || p === join(OPS, "tests")) return [];
    return statSync(p).isDirectory() ? walk(p) : [p];
  });
  for (const f of walk(OPS)) assert.ok(!readFileSync(f, "utf8").includes(plain), `${f} names the plain hash URL`);
});

test("2.9 SEC-SB-5: the expected dist list is committed, compared at bring-up, and refuses the page on a difference", () => {
  assert.match(text("runs/r1-dist.sha256"), /^# R1's expected dist \(SEC-SB-5/);
  assert.match(plan, /`ops\/runs\/r1-dist\.sha256`, committed at X, filled at Stop B from two equal\n {2}builds/);
  assert.match(plan, /`r1\.sh host` and every `browsers` call refuse \(no page is served\)/);
  assert.match(plan, /the server chain still runs/);
  assert.match(text("operator/PROMPT-R1.md"), /then, and only then, `ssh -F <ssh> npt-rs r1\.sh dist-accept`/);
  assert.match(r1, /^ {2}dist_check "\$d"$/m);
});

test("2.16 the slack per VM, the six Primary IPv4s, the bring-up rule and the provider list, recomputed", () => {
  const tl = table("**Thursday 3 December** (T0");
  const slack = minutes("09:30") + 4 * 60 - minutes(tl.find((r) => r[0] === "Teardown")[2]);
  assert.match(plan, new RegExp(`RS's 13:30 less teardown's end 13:25 is \\*\\*${slack} minutes\\*\\* at worst`), "INF-SB-7");
  const [rs, lgs, kept] = [1, 5, 2];
  assert.match(plan, new RegExp(`RS plus five CAX21s, and six Primary IPv4s\\n {3}\\(the two kept, four made with the extra servers\\)`));
  assert.equal(rs + lgs, kept + 4, "INF-SB-5: one Primary IPv4 per server");
  const t45 = minutes("09:30") + 45;
  assert.equal(`${Math.floor(t45 / 60)}:${t45 % 60}`, "10:15");
  assert.match(plan, /no\n {2}build\.done by T0 \+ 45 min \(10:15\), he deletes LG then RS at once/);
  assert.equal(Math.round((0.16 + 0.62) * 100) / 100, 0.78, "a same-day retry inside the cap");
  for (const p of ["GitHub", "crates.io", "static.rust-lang.org", "nodejs.org", "registry.npmjs.org", "the Ubuntu archive"]) {
    assert.ok(plan.includes(p), `INF-SB-2's provider list: ${p}`);
  }
  assert.match(plan, /fw-rs 2, 2, 1, 1; fw-lg 1, 1: no Any, INF-SB-4/);
});

test("the dates recomputed from the cited lines and the rate (C6)", () => {
  const est = (lo, hi) => [lo * 1.35, hi * 1.5];
  const units = { MOVED: est(360, 720), U4k: [800, 1900], U6d: [2450, 2700], U6e: [2150, 2750], U6j: est(1650, 3050) };
  const rows = table("**The lines and the rate**");
  for (const [u, [lo, hi]] of Object.entries(units)) {
    const row = rows.find((r) => r[0] === u);
    const [a, b] = row[2].split("-").map(num);
    assert.equal(a, Math.round(lo / 10) * 10, `${u} low`);
    assert.equal(b, Math.round(hi / 10) * 10, `${u} high`);
  }
  const lo = Object.values(units).reduce((s, [a]) => s + a, 0);
  const hi = Object.values(units).reduce((s, [, b]) => s + b, 0);
  const mid = (lo + hi) / 2;
  const r10 = (x) => Math.round(x / 10) * 10;
  assert.match(plan, new RegExp(`\\*\\*${r10(lo).toLocaleString("en-US")}-${r10(hi).toLocaleString("en-US")}, middle ${r10(mid).toLocaleString("en-US")}\\*\\*`));
  const rate = 1175;
  const days = [lo, mid, hi].map((x) => (x / rate).toFixed(1));
  assert.match(plan, new RegExp(`${days[0]}, ${days[1]} and ${days[2]} lane days`));
  // Lane day 1 is Fri 4 Dec; a unit merges on the lane day its cumulative lines reach.
  const day = (lines) => 3 + Math.ceil(lines / rate);
  const through = (k, pick) => Object.values(units).slice(0, k).reduce((s, r) => s + pick(r), 0);
  const midOf = ([a, b]) => (a + b) / 2;
  const name = (d) => `${["Thu", "Fri", "Sat", "Sun", "Mon", "Tue", "Wed"][(d - 3) % 7]} ${d}`;
  assert.equal(name(day(through(4, midOf))), "Thu 10");
  assert.equal(name(day(through(4, ([, b]) => b))), "Fri 11");
  assert.equal(name(day(through(5, midOf))), "Sat 12");
  assert.equal(name(day(through(5, ([, b]) => b))), "Tue 15");
  assert.match(plan, /U6e\nmerges about Thu 10 \(Fri 11 at the top\), U6j about Sat 12 \(about Tue 15 at the top\)/);
  assert.equal(round2(1.72 + 1.56 + 3.34), 6.62);
  assert.equal(round2(1.9 + 1.8 + 3.6), 7.3);
  assert.match(plan, /planned EUR 6\.62 \(1\.72 \+ 1\.56 \+ 3\.34\), caps EUR 7\.30 \(1\.90 \+ 1\.80 \+ 3\.60\)/);
});

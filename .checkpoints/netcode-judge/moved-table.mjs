// Part a (MOVED): adds the `moved` labels and the two MARK-NOTES texts to spikes/netcode-proof/marks/marks.json.
//   node .checkpoints/netcode-judge/moved-table.mjs [--write]      (from the repository root; without --write: a dry run)
// The file is exactly JSON.stringify(x, null, 2) + "\n" (checked below), so nothing but the listed fields moves.
// Sources: .checkpoints/netcode-proof-test/replan-marks.md:7-99 (the table) and moved-block.md (the destinations);
// MARK-NOTES is read from replan.md:106-111 by line, never retyped. It refuses to run on any other marks.json.
import { createHash } from "node:crypto";
import { readFileSync, writeFileSync } from "node:fs";

const FILE = "spikes/netcode-proof/marks/marks.json";
const BEFORE = "c1935a5b996fb0aaff82b0b3c1c604255e1534f6ca7e8d2e55a9f472cd3b22a9";
const BY = "NPT-TRIM, 2026-11-21 (the re-plan, approved 2026-11-22)";
const sha = (t) => createHash("sha256").update(t, "utf8").digest("hex");
const fail = (m) => {
  console.error(`moved-table: ${m}`);
  process.exit(1);
};

// Where each piece goes, in words used more than once.
const BASELINE = "the security baseline before the first online playtest";
const LAG = "Slice 1 change-set 7 (Lag)";
const SHOOT = "Slice 1 change-set 8 (Shoot)";
const S3 = "Slice 3";
const SAFARI = ["Safari later", "Safari left the proof test (BROWSER-SET (a))"];
const HUMAN = ["Slice 3, with the human run", "the human run (R7) left the proof test"];
const CRATE = ["Slice 3's WebTransport crate choice", "the proof test carries one crate, web-transport-quinn"];
const R4 = "its run, R4, left the proof test";
const U4G = "client prediction left the proof test (U4g)";
const U4H = "lag compensation's judged rows left the proof test (U4h)";
const R7 = "it rode the human run (R7), which left the proof test";

/** Marks moved whole: id -> [to, why]. */
const WHOLE = {
  "PM-0001-25": [BASELINE, "its run, R4 (hostile input), left the proof test"],
  "PM-0001-32": [`Slice 1 change-set 6 (Look) for both backends; the browsers online at ${S3}; Safari later`, `${R7}; the proof test's browser client draws nothing`],
  "PM-0001-33": [S3, `it needs WebTransport's rtt() in the real client; ${R7}`],
  "PM-0001-34": ["the real protocol: change-set 3's codec first, then each message type as it is added", "its unit (U5, the decoder fuzzing) stopped"],
  "PM-0001-35": [S3, "its swap needs the second transport; its unit (U5) stopped"],
  "PM-0001-41": [LAG, U4G],
  "PM-0001-42": [LAG, `${U4G}; 48 scripted clients there`],
  "PM-0001-60": [`${LAG} for movement, change-set 8 (Shoot) for fire timing`, U4G],
  "PM-0001-43": [`${LAG} for movement, change-set 8 (Shoot) for shots; its RD part when RD is named`, R7],
  "PM-0001-44": [LAG, `the replay's cost is the real client's; ${R7}`],
  "PM-0001-45": [LAG, "interpolation left the proof test (U4g2's client half)"],
  "PM-0001-46": [`${LAG} for the delay, change-set 8 (Shoot) for respawns`, R7],
  "PM-0001-61": [SHOOT, "remote events left the proof test (U4g2's client half)"],
  "PM-0001-47": [LAG, "the clock estimate is the real client's; its L1 pass here (a4cac39) stays as a record"],
  "PM-0001-48": [S3, U4H],
  "PM-0001-49": [S3, U4H],
  "PM-0001-50": [S3, U4H],
  "PM-0001-51": [`${SHOOT} for hitscan; its lag-compensated parts at ${S3}`, U4H],
  "PM-0001-52": [`${SHOOT}; the browsers at ${S3}`, `${R7}; the proof test's browser client draws nothing`],
  "PM-0001-62": [SHOOT, U4G],
  "PM-0001-53": [S3, "it is the human run (R7), which left the proof test"],
  "PM-0001-54": ["Slice 1 change-set 9 (Throw)", "its unit (U4h2) left the proof test"],
  "PM-0001-55": ["the first slice that adds vehicles", "its unit (U4g) left the proof test"],
  "PM-0006-15": [BASELINE, R4],
  "PM-0006-26": [BASELINE, R4],
  "PM-0006-27": [BASELINE, R4],
};

/** Parts of kept marks: id -> { part name -> [to, why] }. */
const PARTS = {
  "PM-0001-1": { RD: ["when RD is named, with PM-0001-43's RD part", "the desktop's legs rode the human run (R7)"], MAC: SAFARI },
  "PM-0001-3": { "MP at P2": HUMAN },
  "PM-0001-14": { "MAC Safari": SAFARI },
  "PM-0001-19": { "ML Chrome": HUMAN },
  "PM-0001-28": {
    "wtransport: ML chrome": CRATE,
    "wtransport: ML edge": CRATE,
    "wtransport: ML firefox": CRATE,
    "wtransport: MAC safari": CRATE,
    "wtransport: control, wrong certificate hash": CRATE,
    "wtransport: ML* (rough, reported)": CRATE,
    "web-transport-quinn: MAC safari": SAFARI,
  },
  "PM-0001-29": { "MAC-b safari": SAFARI },
  "PM-0001-31": { "ML Chrome over WebSocket at P2": HUMAN },
  "PM-0001-36": { "MAC-b safari loopback": SAFARI, "MAC-b safari eu-path": SAFARI },
  "PM-0006-14": {
    "N=36": ["slice 6, on the real sim", "the N = 36 and N = 18 searches (R6) left the proof test"],
    "N=18": ["slice 6, on the real sim", "the N = 36 and N = 18 searches (R6) left the proof test"],
  },
  "PM-0006-25": { "MAC safari": SAFARI },
};

/** Checks and bites of kept marks: id -> { checks: { name -> [to, why] }, bites: { id -> [to, why] } }. */
const WITH_41 = [`${LAG}, with PM-0001-41`, "it reads the client's prediction, which left the proof test"];
const PIECES = {
  "PM-0001-58": {
    checks: { "0 corrections larger than 1 mm not traced to a late or lost command (PM-0001-41's count)": WITH_41 },
    bites: { "PM-0001-58/lt-tick-offset": WITH_41 },
  },
  "PM-0001-56": {
    checks: {
      "every listed item read once and written (10 items)": [
        "with their marks: items 5 to 10, and item 1's other crate and pick (Slice 3)",
        "MARK-NOTES: the record counts 4 items here",
      ],
    },
    bites: {},
  },
};

// MARK-NOTES, word for word from the re-plan (lines 106-111), split at its "(1)" and "(2)".
const replan = readFileSync(".checkpoints/netcode-proof-test/replan.md", "utf8").split("\n");
const notes = replan.slice(105, 111).join(" ").replace(/\s+/g, " ");
const m = notes.match(/\(1\) -7: (.+?\.) \(2\) (-56's record .+?\.) Lean: yes to both\.$/);
if (!m) fail("replan.md:106-111 is not the MARK-NOTES text this script reads");
const [, NOTE_7, NOTE_56] = m;
const TAG = "MARK-NOTES (Mustafa, 2026-11-22): ";
const OLD_7 =
  "The history is sized for the largest cap the test uses (-48's test cap, fixed in U4 from L1's largest honest rewind at P3 and P4) plus the margin, as data, recorded; the rule cap can only shrink it, so -7 needs no re-run (plan §6.2).";

const text = readFileSync(FILE, "utf8");
if (sha(text) !== BEFORE) fail(`${FILE} is not the file this table was written for (sha256 ${sha(text)})`);
const data = JSON.parse(text);
if (`${JSON.stringify(data, null, 2)}\n` !== text) fail("marks.json is not JSON.stringify(x, null, 2): refusing to rewrite it");

const moved = ([to, why]) => ({ to, why, by: BY });
/** The object again with `moved` right after `after` (so the label sits beside the name it belongs to). */
const withMoved = (obj, after, label) => {
  if ("moved" in obj) fail(`${obj.id ?? obj.name}: already moved`);
  const out = {};
  for (const [k, v] of Object.entries(obj)) {
    out[k] = v;
    if (k === after) out.moved = moved(label);
  }
  if (!("moved" in out)) fail(`${obj.id ?? obj.name}: no ${after} to put the label after`);
  return out;
};
const one = (list, pred, what) => {
  const at = list.map((x, i) => (pred(x) ? i : -1)).filter((i) => i >= 0);
  if (at.length !== 1) fail(`${what}: ${at.length} matches, not 1`);
  return at[0];
};

const counts = { marks: 0, parts: 0, checks: 0, bites: 0 };
const mark = (id) => one(data.marks, (x) => x.id === id, id);
for (const [id, label] of Object.entries(WHOLE)) {
  const i = mark(id);
  if (data.marks[i].verdict === "outside") fail(`${id} is outside: it reads MOVED by its kind`);
  data.marks[i] = withMoved(data.marks[i], "verdict", label);
  counts.marks++;
}
for (const [id, parts] of Object.entries(PARTS)) {
  const mk = data.marks[mark(id)];
  for (const [name, label] of Object.entries(parts)) {
    const i = one(mk.parts, (p) => p.name === name, `${id} part ${name}`);
    mk.parts[i] = withMoved(mk.parts[i], "name", label);
    counts.parts++;
  }
}
for (const [id, { checks, bites }] of Object.entries(PIECES)) {
  const mk = data.marks[mark(id)];
  for (const [name, label] of Object.entries(checks)) {
    const i = one(mk.checks, (c) => c.name === name, `${id} check ${name}`);
    mk.checks[i] = withMoved(mk.checks[i], "name", label);
    counts.checks++;
  }
  for (const [bid, label] of Object.entries(bites)) {
    const i = one(mk.bites, (b) => b.id === bid, `bite ${bid}`);
    mk.bites[i] = withMoved(mk.bites[i], "id", label);
    counts.bites++;
  }
}

// -56 (MARK-NOTES 2): the kept count, right after the moved 10-item check; its notes gain the sentence.
const m56 = data.marks[mark("PM-0001-56")];
const old56 = one(m56.checks, (c) => c.name === "every listed item read once and written (10 items)", "-56's count");
const { moved: _gone, name: _name, value: _value, ...shape } = m56.checks[old56];
m56.checks.splice(old56 + 1, 0, { name: "every kept item read once and written (4 items)", ...shape, value: 4 });
m56.notes = `${m56.notes} ${TAG}${NOTE_56}`;
// -7 (MARK-NOTES 1): the sentence on the history's size is replaced.
const m7 = data.marks[mark("PM-0001-7")];
if (m7.notes.split(OLD_7).length !== 2) fail("-7's notes do not hold the sentence MARK-NOTES replaces exactly once");
m7.notes = m7.notes.replace(OLD_7, `${TAG}${NOTE_7}`);

const out = `${JSON.stringify(data, null, 2)}\n`;
console.log(`labels: ${counts.marks} marks whole, ${counts.parts} parts, ${counts.checks} checks, ${counts.bites} bites; -56's kept check added; -7's and -56's notes`);
console.log(`sha256 before ${BEFORE}`);
console.log(`sha256 after  ${sha(out)}`);
if (process.argv.includes("--write")) {
  writeFileSync(FILE, out);
  console.log(`written: ${FILE}`);
} else {
  console.log("dry run: nothing written (pass --write)");
}

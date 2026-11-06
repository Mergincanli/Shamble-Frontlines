// Tests for check.mjs's rule checks: each dependency rule of plan §4.2 is broken once on a synthetic graph and must
// be caught; the clean graph must pass. Run: node --test spikes/netcode-proof/check.test.mjs

import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { test } from "node:test";
import { dependencyProblems, expectedMarks, marksProblems, referenceProblems } from "./check.mjs";

/** cargo metadata's shape, from edges { crate: [[dependency, kind]] } (kind null = normal, "dev", "build"). */
function meta(edges) {
  const names = new Set([...Object.keys(edges), ...Object.values(edges).flatMap((ds) => ds.map(([d]) => d))]);
  const id = (n) => `path+file:///w/${n}#0.0.0`;
  return {
    packages: [...names].map((n) => ({ id: id(n), name: n })),
    workspace_members: Object.keys(edges)
      .filter((n) => n.startsWith("npt-"))
      .map(id),
    resolve: {
      nodes: [...names].map((n) => ({
        id: id(n),
        deps: (edges[n] ?? []).map(([d, kind]) => ({ name: d, pkg: id(d), dep_kinds: [{ kind, target: null }] })),
      })),
    },
  };
}

const n = (...ds) => ds.map((d) => [d, null]);

function clean() {
  return {
    "npt-link": [],
    "npt-l1": n("npt-link"),
    "npt-sim": [],
    "npt-proto": n("serde"),
    "npt-room": n("npt-proto"),
    "npt-server": n("npt-link", "npt-proto", "npt-room", "npt-sim", "npt-net", "npt-log", "tokio"),
    "npt-net": n("npt-link", "tokio", "quinn", "wtransport", "tokio-tungstenite"),
    "npt-client": n("npt-proto", "npt-sim"),
    "npt-client-wasm": n("npt-client"),
    "npt-bots": n("npt-link", "npt-proto", "npt-client", "npt-net", "npt-log"),
    "npt-super": n("npt-log"),
    "npt-host": n("npt-log", "tokio"),
    "npt-log": n("serde", "serde_json"),
    "npt-judge": n("npt-log", "serde", "serde_json"),
    "npt-lab": n("npt-link", "npt-l1", "npt-sim", "npt-proto", "npt-room", "npt-server", "npt-client", "npt-log"),
    quinn: n("quinn-proto", "tokio"),
    wtransport: n("quinn"),
    "tokio-tungstenite": n("tungstenite", "tokio"),
    tokio: n("mio"),
  };
}

function problemsWith(change) {
  const edges = clean();
  change(edges);
  return dependencyProblems(meta(edges));
}

test("the clean graph passes", () => {
  assert.deepEqual(dependencyProblems(meta(clean())), []);
});

test("a leaf crate reaching an async runtime fails, even through another crate", () => {
  const p = problemsWith((e) => {
    e["npt-sim"] = n("helper");
    e.helper = n("tokio");
  });
  assert.ok(p.includes("leaf crate npt-sim depends on tokio"), p.join("\n"));
  assert.ok(p.includes("leaf crate npt-sim depends on mio"), "tokio's own parts are caught too");
});

test("a dev dependency does not ship, so it does not count", () => {
  assert.deepEqual(
    problemsWith((e) => {
      e["npt-room"] = [...n("npt-proto"), ["tokio", "dev"]];
    }),
    [],
  );
  const p = problemsWith((e) => {
    e["npt-room"] = [...n("npt-proto"), ["tokio", "build"]];
  });
  assert.ok(p.includes("leaf crate npt-room depends on tokio"), "a build dependency does count");
});

test("npt-client never depends on npt-net", () => {
  const p = problemsWith((e) => e["npt-client"].push(["npt-net", null]));
  assert.ok(p.includes("npt-client depends on npt-net"), p.join("\n"));
  assert.ok(p.includes("leaf crate npt-client depends on quinn"), "and through it the transports");
});

test("npt-super and npt-host keep away from the sim, room, protocol and client crates", () => {
  assert.ok(problemsWith((e) => e["npt-super"].push(["npt-proto", null])).includes("npt-super depends on npt-proto"));
  assert.ok(problemsWith((e) => e["npt-host"].push(["npt-client", null])).includes("npt-host depends on npt-client"));
  assert.ok(problemsWith((e) => e["npt-host"].push(["npt-room", null])).includes("npt-host depends on npt-room"));
});

test("npt-judge depends only on npt-log and serde", () => {
  assert.ok(problemsWith((e) => e["npt-judge"].push(["regex", null])).includes("npt-judge depends on regex"));
  const p = problemsWith((e) => e["npt-judge"].push(["npt-sim", null]));
  assert.ok(p.includes("npt-judge depends on npt-sim") && p.includes("npt-judge reaches npt-sim"), p.join("\n"));
  assert.deepEqual(
    problemsWith((e) => e["npt-judge"].push(["tempfile", "dev"])),
    [],
    "a test-only dependency is allowed",
  );
});

test("nothing depends on npt-lab, not even for tests", () => {
  const p = problemsWith((e) => e["npt-server"].push(["npt-lab", "dev"]));
  assert.ok(p.includes("npt-server depends on npt-lab"), p.join("\n"));
});

test("marks.json covers exactly the plan's marks, and every row it hashed is unchanged", () => {
  const a1 = "docs/adr/0001-multiplayer-netcode.md";
  const a4 = "docs/adr/0004-simulation-core.md";
  const a6 = "docs/adr/0006-infrastructure-operations.md";
  const r1 = "| PM-0001-1 | The rig is named | recorded | RS | x | y | z |";
  const r2 = "| PM-0001-2 | The load is the stress scene | 48 soldiers | RS | x | y | z |";
  const r67 = "| PM-0004-67 | The loop keeps time | p99 <= 2 ms | RS | x | y | z |";
  const r6 = (n) => `| PM-0006-${n} | row ${n} | t | RS | x | y | z |`;
  const files = {
    [a1]: `# ADR 0001\n\n| ID | Mark |\n|---|---|\n${r1}\n${r2}\n`,
    [a4]: `${r67}\r\n`,
    [a6]: Array.from({ length: 17 }, (_, i) => r6(i + 11)).join("\n"),
  };
  const sha = (s) => createHash("sha256").update(s, "utf8").digest("hex");
  const mark = (id, file, row) => ({ id, draft: { file, row_sha256: sha(row) } });
  const good = {
    marks: [
      mark("PM-0001-1", a1, r1),
      mark("PM-0001-2", a1, r2),
      mark("PM-0004-67", a4, r67),
      ...Array.from({ length: 17 }, (_, i) => mark(`PM-0006-${i + 11}`, a6, r6(i + 11))),
    ],
  };
  const read = (f) => files[f];
  assert.deepEqual(expectedMarks(files[a1]).slice(0, 3), ["PM-0001-1", "PM-0001-2", "PM-0004-67"]);
  assert.deepEqual(marksProblems(good, read), [], "a CRLF line end is not part of the row");
  const changed = { ...files, [a1]: files[a1].replace("48 soldiers", "47 soldiers") };
  assert.deepEqual(marksProblems(good, (f) => changed[f]), [
    `PM-0001-2: its row (${a1}:6) changed since marks.json took its hash; re-read it`,
  ]);
  const missing = { marks: good.marks.filter((m) => m.id !== "PM-0006-20") };
  assert.deepEqual(marksProblems(missing, read), ["PM-0006-20 is in marks.json 0 times, not once"]);
  const extra = { marks: [...good.marks, mark("PM-0002-1", a1, r1)] };
  assert.ok(marksProblems(extra, read).includes("PM-0002-1 is in marks.json but not in plan §6.2's marks"));
  const twice = { ...files, [a1]: `${files[a1]}${r2}\n` };
  assert.ok(marksProblems(good, (f) => twice[f]).includes(`PM-0001-2: 2 rows in ${a1}, not 1`));
});

test("only code outside spikes/ is read for references to it", () => {
  const files = [
    "shamble-core.js",
    "index.html",
    ".github/workflows/gate.yml",
    "docs/SPIKE_PLAN.md",
    ".checkpoints/netcode-proof-test.md",
    "CLAUDE.md",
    "spikes/netcode-proof/check.mjs",
  ];
  const text = {
    "shamble-core.js": "const a = 1;",
    "index.html": "<script src='spikes/netcode-proof/web/main.js'></script>",
    ".github/workflows/gate.yml": "run: node tools/gate.mjs",
    "docs/SPIKE_PLAN.md": "spikes/netcode-proof",
    ".checkpoints/netcode-proof-test.md": "spikes/netcode-proof",
    "CLAUDE.md": "throwaway spikes/ only",
    "spikes/netcode-proof/check.mjs": "spikes/",
  };
  assert.deepEqual(referenceProblems(files, (f) => text[f]), ["index.html references spikes/"]);
});

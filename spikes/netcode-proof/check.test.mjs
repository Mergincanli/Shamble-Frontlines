// Tests for check.mjs's rule checks: each dependency rule of plan §4.2 is broken once on a synthetic graph and must
// be caught; the clean graph must pass. Run: node --test spikes/netcode-proof/check.test.mjs

import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { test } from "node:test";
import { dependencyProblems, expectedMarks, lexRust, marksProblems, referenceProblems, unsafeProblems } from "./check.mjs";

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
    "npt-net": n("npt-link", "tokio", "quinn", "wtransport", "tokio-tungstenite", "npt-tcpinfo"),
    "npt-tcpinfo": [...n("libc"), ["socket2", "dev"]],
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

test("npt-tcpinfo ships with libc alone, a dev dependency aside", () => {
  assert.ok(problemsWith((e) => e["npt-tcpinfo"].push(["nix", null])).includes("npt-tcpinfo depends on nix"));
  assert.ok(problemsWith((e) => e["npt-tcpinfo"].push(["cc", "build"])).includes("npt-tcpinfo depends on cc"));
  const p = problemsWith((e) => {
    e["npt-tcpinfo"] = n("rustix");
  });
  assert.ok(p.includes("npt-tcpinfo depends on rustix") && p.includes("npt-tcpinfo does not depend on libc"), p.join("\n"));
  const gone = problemsWith((e) => {
    delete e["npt-tcpinfo"];
    e["npt-net"] = e["npt-net"].filter(([d]) => d !== "npt-tcpinfo");
  });
  assert.ok(gone.includes("npt-tcpinfo is not a workspace member"), gone.join("\n"));
});

test("only npt-net depends on npt-tcpinfo, in any kind; reaching it through npt-net is fine", () => {
  assert.ok(problemsWith((e) => e["npt-server"].push(["npt-tcpinfo", null])).includes("npt-server depends on npt-tcpinfo"));
  assert.ok(problemsWith((e) => e["npt-bots"].push(["npt-tcpinfo", "dev"])).includes("npt-bots depends on npt-tcpinfo"));
  assert.ok(problemsWith((e) => e["npt-lab"].push(["npt-tcpinfo", "build"])).includes("npt-lab depends on npt-tcpinfo"));
});

test("npt-net has no direct libc dependency; reaching it through npt-tcpinfo or tokio is fine", () => {
  assert.deepEqual(problemsWith((e) => e["npt-net"].push(["libc", null])), ["npt-net depends on libc directly"]);
  assert.deepEqual(problemsWith((e) => e["npt-net"].push(["libc", "build"])), ["npt-net depends on libc directly"]);
  assert.deepEqual(problemsWith((e) => e["npt-net"].push(["libc", "dev"])), [], "a dev dependency does not ship (C10)");
  assert.deepEqual(problemsWith((e) => (e.tokio = n("mio", "libc"))), [], "through tokio");
  const gone = problemsWith((e) => {
    delete e["npt-net"];
    e["npt-server"] = e["npt-server"].filter(([d]) => d !== "npt-net");
    e["npt-bots"] = e["npt-bots"].filter(([d]) => d !== "npt-net");
  });
  assert.ok(gone.includes("npt-net is not a workspace member"), gone.join("\n"));
});

/** The placed allow, as npt-tcpinfo writes it: a multi-line reason with a line continuation and brackets inside. */
const placed = (f, between = "") =>
  `#[allow(\n    unsafe_code,\n    reason = "one foreign call (TCP_INFO) [ADR 0002]; \\\n              no wrapper"\n)]\n` +
  `${between}pub fn ${f}(fd: BorrowedFd<'_>) -> io::Result<u32> {\n    Ok(0)\n}\n`;

/** A clean crate tree for unsafeProblems: the two exceptions and two ordinary crates. */
function cleanCrates() {
  return {
    "crates/client-wasm/Cargo.toml": '[package]\nname = "npt-client-wasm"\n\n[lints.rust]\nunsafe_code = "allow"\n',
    "crates/client-wasm/src/lib.rs": "pub fn create() {}\n",
    "crates/tcpinfo/Cargo.toml": '[package]\nname = "npt-tcpinfo"\n\n[lints]\nworkspace = true\n',
    "crates/tcpinfo/src/lib.rs":
      "//! The `#[allow(unsafe_code)]` sits on the two functions only (a doc comment, not an attribute).\n" +
      "/* A block comment /* nested */ naming #[allow(unsafe_code)] is a comment too. */\n" +
      "#![deny(clippy::undocumented_unsafe_blocks)]\nmod linux;\n",
    "crates/tcpinfo/src/linux.rs":
      `const URL: &str = "http://example.invalid/#[x]"; // a // inside a string is not a comment\n` +
      `const QUOTE: char = '"';\nconst RAW: &str = r#"a "quoted" ]] #[ bit"#;\n` +
      `fn life<'a>(s: &'a str) -> &'a str {\n    s\n}\n` +
      `${placed("srtt_us")}${placed("unacked_and_unsent", "#[inline]\n")}`,
    "crates/net/Cargo.toml": '[package]\nname = "npt-net"\n# unsafe_code = "allow" in a comment sets nothing\n\n[lints]\nworkspace = true\n',
    "crates/net/src/lib.rs": "//! npt-net stays free of unsafe code; the workspace denies unsafe_code.\n#![deny(unsafe_code)]\nmod l2;\n",
    "crates/net/src/l2/socket.rs": '#[cfg(test)]\n#[path = "socket_tests.rs"]\nmod tests;\n',
    "crates/net/src/l2/socket_tests.rs": 'const SERIES: &str = include_str!("../../../../marks/SERIES.md");\n',
    "crates/sim/Cargo.toml": '[package]\nname = "npt-sim"\n\n[lints]\nworkspace = true\n\n[dependencies]\n',
  };
}

function unsafeWith(change) {
  const files = cleanCrates();
  change(files);
  return unsafeProblems(Object.keys(files), (f) => files[f]);
}

/** npt-tcpinfo's two messages: a misplaced relaxing at `where` on `on`, and a function whose count is not 1. */
const misplaced = (where, on) =>
  `${where} relaxes unsafe_code on ${on}; npt-tcpinfo may only put #[allow(unsafe_code, reason = "...")] directly on ` +
  "srtt_us and unacked_and_unsent in crates/tcpinfo/src/linux.rs";
const count = (fn, k) => `npt-tcpinfo's pub fn ${fn} has ${k} #[allow(unsafe_code, reason = ...)] attributes, not 1`;

test("unsafe_code: the clean tree passes", () => {
  assert.deepEqual(unsafeWith(() => {}), []);
});

test("lexRust blanks comments and keeps literals, offsets and lines", () => {
  const src = `a /* x /* y */ z */ "s // t" 'q' '"' r#"u "v" w"# // c\nb'\\n' x<'a> /** d */ e`;
  const { code, mask } = lexRust(src);
  assert.equal(code.length, src.length);
  assert.equal(mask.length, src.length);
  assert.equal(code.split("\n").length, 2);
  assert.match(code, /^a\s+"s \/\/ t" 'q' '"' r#"u "v" w"#\s+\nb'\\n' x<'a>\s+e$/);
  assert.match(mask, /^a\s+\nb\s+x<'a>\s+e$/);
});

test("unsafe_code: an attribute relaxing it outside the two exceptions fails, in every form", () => {
  const planted = [
    "#[allow(unsafe_code)]\nfn f() {}\n",
    "#[expect(unsafe_code)]\nfn f() {}\n",
    "#[warn(unsafe_code)]\nfn f() {}\n",
    "#[allow(dead_code, unsafe_code)]\nfn f() {}\n",
    "#[allow(\n    clippy::all,\n    unsafe_code\n)]\nfn f() {}\n",
    "#![cfg_attr(test, allow(unsafe_code))]\n",
  ];
  for (const code of planted) {
    const p = unsafeWith((f) => {
      f["crates/net/src/planted.rs"] = code;
    });
    assert.deepEqual(p, ["crates/net/src/planted.rs relaxes unsafe_code (only npt-client-wasm and npt-tcpinfo may)"], code);
  }
});

test("unsafe_code: a manifest that sets it or drops the workspace lints fails", () => {
  const set = unsafeWith((f) => {
    f["crates/sim/Cargo.toml"] = '[package]\nname = "npt-sim"\n\n[lints.rust]\nunsafe_code = "allow"\n';
  });
  assert.deepEqual(set, [
    "crates/sim/Cargo.toml sets unsafe_code",
    "crates/sim/Cargo.toml does not inherit the workspace lints ([lints] workspace = true)",
  ]);
  const dropped = unsafeWith((f) => {
    f["crates/sim/Cargo.toml"] = '[package]\nname = "npt-sim"\nworkspace = true\n';
  });
  assert.deepEqual(dropped, ["crates/sim/Cargo.toml does not inherit the workspace lints ([lints] workspace = true)"]);
});

test("unsafe_code: npt-tcpinfo never relaxes it crate-wide, in another file, or by a missing allow", () => {
  const wide = unsafeWith((f) => {
    f["crates/tcpinfo/src/lib.rs"] = "#![allow(unsafe_code)]\nmod linux;\n";
  });
  assert.deepEqual(wide, [misplaced("crates/tcpinfo/src/lib.rs:1", "its whole module or crate (#!)")]);
  const third = unsafeWith((f) => {
    f["crates/tcpinfo/examples/probe.rs"] = "#[allow(unsafe_code)]\nfn main() {}\n";
  });
  assert.deepEqual(third, [misplaced("crates/tcpinfo/examples/probe.rs:1", "`fn main() {}`")]);
  const none = unsafeWith((f) => {
    f["crates/tcpinfo/src/linux.rs"] = "pub fn srtt_us() {}\n";
  });
  assert.deepEqual(none, [count("srtt_us", 0), count("unacked_and_unsent", 0)]);
  const stub = unsafeWith((f) => {
    f["crates/tcpinfo/src/linux.rs"] = placed("unacked_and_unsent");
    f["crates/tcpinfo/src/stub.rs"] = placed("srtt_us");
  });
  assert.deepEqual(stub, [misplaced("crates/tcpinfo/src/stub.rs:1", "`pub fn srtt_us(fd: BorrowedFd<'_>) -> io::Result<u32> {`"), count("srtt_us", 0)]);
});

test("unsafe_code: the critic's two mutations fail (an allow on `mod linux;`, an allow moved to a helper)", () => {
  // 1. A module-level outer allow, with one function's own allow removed: the count of attributes stays 2.
  const onMod = unsafeWith((f) => {
    f["crates/tcpinfo/src/lib.rs"] = '#[cfg(target_os = "linux")]\n#[allow(unsafe_code)]\nmod linux;\n';
    f["crates/tcpinfo/src/linux.rs"] = `pub fn srtt_us() {}\n${placed("unacked_and_unsent")}`;
  });
  assert.deepEqual(onMod, [misplaced("crates/tcpinfo/src/lib.rs:2", "`mod linux;`"), count("srtt_us", 0)]);
  // 2. The allow moved from srtt_us to a helper, in its exact form: the count stays 2.
  const moved = unsafeWith((f) => {
    const helper = placed("srtt_us").replace("pub fn srtt_us", "fn helper");
    f["crates/tcpinfo/src/linux.rs"] = `pub fn srtt_us() {}\n${placed("unacked_and_unsent")}${helper}`;
  });
  assert.deepEqual(moved, [misplaced("crates/tcpinfo/src/linux.rs:10", "`fn helper(fd: BorrowedFd<'_>) -> io::Result<u32> {`"), count("srtt_us", 0)]);
  // The same onto another public function.
  const other = unsafeWith((f) => {
    f["crates/tcpinfo/src/linux.rs"] = `${placed("unacked_and_unsent")}${placed("other")}pub fn srtt_us() {}\n`;
  });
  assert.deepEqual(other, [misplaced("crates/tcpinfo/src/linux.rs:9", "`pub fn other(fd: BorrowedFd<'_>) -> io::Result<u32> {`"), count("srtt_us", 0)]);
});

test("unsafe_code: on the two functions only the exact allow with its reason counts", () => {
  const forms = {
    "#[allow(unsafe_code)]\n": "no reason",
    '#[expect(unsafe_code, reason = "x")]\n': "expect",
    '#[allow(unsafe_code, dead_code, reason = "x")]\n': "another lint beside it",
    "#[cfg_attr(all(), allow(unsafe_code))]\n": "cfg_attr",
    '#[allow(unsafe_code, reason = r"x")]\n': "a raw-string reason",
  };
  for (const [attr, what] of Object.entries(forms)) {
    const p = unsafeWith((f) => {
      f["crates/tcpinfo/src/linux.rs"] = `${placed("unacked_and_unsent")}${attr}pub fn srtt_us() {}\n`;
    });
    assert.deepEqual(p, [misplaced("crates/tcpinfo/src/linux.rs:9", "`pub fn srtt_us() {}`"), count("srtt_us", 0)], what);
  }
  const inner = unsafeWith((f) => {
    f["crates/tcpinfo/src/linux.rs"] = `#![allow(unsafe_code)]\n${f["crates/tcpinfo/src/linux.rs"]}`;
  });
  assert.deepEqual(inner, [misplaced("crates/tcpinfo/src/linux.rs:1", "its whole module or crate (#!)")]);
  const twice = unsafeWith((f) => {
    f["crates/tcpinfo/src/linux.rs"] = `${placed("unacked_and_unsent")}${placed("srtt_us", placed("x").split("pub fn")[0])}`;
  });
  assert.deepEqual(twice, [count("srtt_us", 2)], "two allows stacked on one function");
});

test("unsafe_code: the lexer neither hides an attribute nor counts a comment", () => {
  // After a string holding `//` and a char literal holding `"`, the attribute is still code and still caught.
  const hidden = unsafeWith((f) => {
    f["crates/tcpinfo/src/linux.rs"] += `const A: &str = "x // y"; const B: char = '"';\n#[allow(unsafe_code)]\nfn f() {}\n`;
  });
  const next = cleanCrates()["crates/tcpinfo/src/linux.rs"].split("\n").length;
  assert.deepEqual(hidden, [misplaced(`crates/tcpinfo/src/linux.rs:${next + 1}`, "`fn f() {}`")]);
  // A mention inside a string is not an attribute, so it is reported, not counted as a placed allow.
  const inString = unsafeWith((f) => {
    f["crates/tcpinfo/src/linux.rs"] += 'const S: &str = "#[allow(unsafe_code)]";\n';
  });
  assert.deepEqual(inString, [
    "crates/tcpinfo/src/linux.rs names an unsafe_code relaxing outside an attribute (a string or a macro's text)",
  ]);
  // Inside a (nested) block comment it is nothing.
  assert.deepEqual(
    unsafeWith((f) => {
      f["crates/tcpinfo/src/linux.rs"] += "/* /* */ #[allow(unsafe_code)] fn f() {} */\n";
    }),
    [],
  );
});

test("unsafe_code: no #[path] or include! reaches outside its crate, the way npt-tcpinfo's file could be borrowed", () => {
  const planted = {
    '#[path = "../../tcpinfo/src/linux.rs"]\nmod k;\n': '#[path = "../../tcpinfo/src/linux.rs"]',
    '#[cfg_attr(unix, path = "../../tcpinfo/src/linux.rs")]\nmod k;\n': '#[path = "../../tcpinfo/src/linux.rs"]',
    '#[path = "/abs/linux.rs"]\nmod k;\n': '#[path = "/abs/linux.rs"]',
    'include!("../../tcpinfo/src/linux.rs");\n': 'include!("../../tcpinfo/src/linux.rs")',
    'include!("k.inc");\n': 'include!("k.inc")',
    'include!(concat!(env!("OUT_DIR"), "/k.rs"));\n': "include! with a computed path",
  };
  for (const [code, what] of Object.entries(planted)) {
    const p = unsafeWith((f) => {
      f["crates/room/src/k.rs"] = code;
    });
    assert.deepEqual(p, [`crates/room/src/k.rs:1 has ${what}, not a .rs file inside crates/room/`], code);
  }
  assert.deepEqual(
    unsafeWith((f) => {
      f["crates/room/src/k.rs"] = '#[path = "../src/sub/k2.rs"]\nmod k2;\ninclude!("./k3.rs");\n';
    }),
    [],
    "inside the crate is fine",
  );
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

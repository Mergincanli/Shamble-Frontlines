// Tests for check.mjs's rule checks: each dependency rule of plan §4.2 is broken once on a synthetic graph and must
// be caught; the clean graph must pass. Run: node --test spikes/netcode-proof/check.test.mjs

import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { mkdirSync, mkdtempSync, readFileSync, readdirSync, readlinkSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, relative } from "node:path";
import { test } from "node:test";
import { fileURLToPath } from "node:url";
import {
  BITE_ID,
  STEPS,
  WASM_CRATES,
  biteHomeProblems,
  cargoConfigProblems,
  cargoCounts,
  crateFiles,
  dependencyProblems,
  depsStep,
  exceptionLintProblems,
  expectedMarks,
  layoutProblems,
  lexRust,
  manifestKeyProblems,
  marksProblems,
  NOT_TICK_COUNTS,
  referenceProblems,
  roomFloatProblems,
  SIM_DEPENDENCIES,
  TICK_NAMED_SKIP,
  tickConstantProblems,
  unsafeProblems,
  workspaceLintProblems,
} from "./check.mjs";

const HERE = dirname(fileURLToPath(import.meta.url));

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
    "npt-sim": [...n("libm", "rand_chacha", "rand_core"), ["proptest", "dev"]],
    rand_chacha: n("rand_core", "ppv-lite86"),
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

test("npt-sim ships with libm, rand_chacha and rand_core alone, a dev dependency aside (U4s, the arch review's F4)", () => {
  assert.deepEqual(SIM_DEPENDENCIES, ["libm", "rand_chacha", "rand_core"]);
  assert.deepEqual(problemsWith((e) => e["npt-sim"].push(["serde", null])), ["npt-sim depends on serde"]);
  assert.deepEqual(problemsWith((e) => e["npt-sim"].push(["cc", "build"])), ["npt-sim depends on cc"], "a build dependency ships");
  const above = problemsWith((e) => e["npt-sim"].push(["npt-proto", null]));
  assert.ok(above.includes("npt-sim depends on npt-proto"), above.join("\n"));
  assert.deepEqual(problemsWith((e) => e["npt-sim"].push(["tempfile", "dev"])), [], "a dev dependency does not ship (C10)");
  assert.deepEqual(problemsWith((e) => (e["npt-sim"] = n("libm"))), [], "a subset of the list is fine");
  // The list is of direct dependencies: what they reach (rand_chacha's ppv-lite86) is theirs.
  assert.deepEqual(problemsWith((e) => (e.libm = n("helper"))), []);
  // A missing npt-sim fails, as the other named members do, and is said once (it is a leaf crate too: the rust
  // fix-review's nit).
  const gone = problemsWith((e) => {
    delete e["npt-sim"];
    for (const k of Object.keys(e)) e[k] = e[k].filter(([d]) => d !== "npt-sim");
  });
  assert.deepEqual(gone, ["npt-sim is not a workspace member"]);
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

/**
 * The placed allow, as npt-tcpinfo writes it: a multi-line reason with a line continuation and brackets inside, on a
 * function whose body holds an unsafe block (where it may).
 */
const placed = (f, between = "") =>
  `#[allow(\n    unsafe_code,\n    reason = "one foreign call (TCP_INFO) [ADR 0002]; \\\n              no wrapper"\n)]\n` +
  `${between}pub fn ${f}(fd: BorrowedFd<'_>) -> io::Result<u32> {\n    unsafe { Ok(0) }\n}\n`;
/** npt-tcpinfo's crate-level deny of the two clippy unsafe lints, as lib.rs's first line. */
const DENY = "#![deny(clippy::undocumented_unsafe_blocks, clippy::multiple_unsafe_ops_per_block)]\n";

/** A clean crate tree for unsafeProblems: the two exceptions and two ordinary crates. */
function cleanCrates() {
  return {
    "crates/client-wasm/Cargo.toml": '[package]\nname = "npt-client-wasm"\n\n[lints.rust]\nunsafe_code = "allow"\n',
    "crates/client-wasm/src/lib.rs": "pub fn create() {}\n",
    "crates/tcpinfo/Cargo.toml": '[package]\nname = "npt-tcpinfo"\n\n[lints.rust]\nunsafe_code = "deny"\n',
    "crates/tcpinfo/src/lib.rs":
      "//! The `#[allow(unsafe_code)]` sits on the two functions only (a doc comment, not an attribute).\n" +
      "/* A block comment /* nested */ naming #[allow(unsafe_code)] is a comment too. */\n" +
      "#![deny(\n    clippy::undocumented_unsafe_blocks,\n    clippy::multiple_unsafe_ops_per_block\n)]\nmod linux;\n",
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
/** npt-tcpinfo's message for `unsafe` (or `global_asm`) outside the two functions' bodies. */
const outside = (where, what = "unsafe") =>
  `${where} has \`${what}\` outside the bodies of srtt_us and unacked_and_unsent in crates/tcpinfo/src/linux.rs`;

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
    // Review S1's routes rustc 1.99.1 accepts: a comment inside the list (one holding `)` too), raw identifiers.
    "#[allow(dead_code, /* x */ unsafe_code)]\nfn f() {}\n",
    "#[allow(dead_code, /* ) */ unsafe_code)]\nfn f() {}\n",
    "#[allow(dead_code, // )\n    unsafe_code)]\nfn f() {}\n",
    "#[allow(r#unsafe_code)]\nfn f() {}\n",
    "#[r#allow(unsafe_code)]\nfn f() {}\n",
    "#[allow(r#dead_code, unsafe_code)]\nfn f() {}\n",
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
  // The two exceptions' manifests set their own level (the workspace forbids it): exceptionLintProblems reads those.
  assert.deepEqual(
    unsafeWith((f) => (f["crates/tcpinfo/Cargo.toml"] = '[package]\nname = "npt-tcpinfo"\n\n[lints.rust]\nunsafe_code = "deny"\n')),
    [],
  );
});

test("a crate's `[lints] workspace = true` inside a multi-line string inherits nothing, basic or literal (review C2c N2)", () => {
  // The reviewer's plant: npt-sim's two lines moved into a string under [package.metadata.npt]; cargo then gave rustc no
  // lint flag at all (a shebang-desynced unsafe block compiled) while the deps step passed.
  const dropped = ["crates/sim/Cargo.toml does not inherit the workspace lints ([lints] workspace = true)"];
  for (const q of ['"""', "'''"]) {
    const p = unsafeWith((f) => {
      f["crates/sim/Cargo.toml"] = f["crates/sim/Cargo.toml"].replace("[lints]\nworkspace = true\n", `[package.metadata.npt]\nnote = ${q}\n[lints]\nworkspace = true\n${q}\n`);
    });
    assert.deepEqual(p, dropped, q);
  }
  // Read as cargo reads it: a comment after either line and CRLF line ends still inherit.
  assert.deepEqual(unsafeWith((f) => (f["crates/sim/Cargo.toml"] = '[package]\r\nname = "npt-sim"\r\n\r\n[lints] # all\r\nworkspace = true # inherited\r\n')), []);
  // The name scan reads past a `#` inside a string (no comment there), as cargo does.
  assert.deepEqual(unsafeWith((f) => (f["crates/sim/Cargo.toml"] += '[package.metadata.npt]\nnote = { a = "#", unsafe_code = 1 }\n')), [
    "crates/sim/Cargo.toml sets unsafe_code",
  ]);
});

test("unsafe_code: npt-tcpinfo never relaxes it crate-wide, in another file, or by a missing allow", () => {
  const wide = unsafeWith((f) => {
    f["crates/tcpinfo/src/lib.rs"] = `#![allow(unsafe_code)]\n${DENY}mod linux;\n`;
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
  assert.deepEqual(stub, [
    misplaced("crates/tcpinfo/src/stub.rs:1", "`pub fn srtt_us(fd: BorrowedFd<'_>) -> io::Result<u32> {`"),
    outside("crates/tcpinfo/src/stub.rs:7"),
    count("srtt_us", 0),
  ]);
});

test("unsafe_code: the critic's two mutations fail (an allow on `mod linux;`, an allow moved to a helper)", () => {
  // 1. A module-level outer allow, with one function's own allow removed: the count of attributes stays 2.
  const onMod = unsafeWith((f) => {
    f["crates/tcpinfo/src/lib.rs"] = `${DENY}#[cfg(target_os = "linux")]\n#[allow(unsafe_code)]\nmod linux;\n`;
    f["crates/tcpinfo/src/linux.rs"] = `pub fn srtt_us() {}\n${placed("unacked_and_unsent")}`;
  });
  assert.deepEqual(onMod, [misplaced("crates/tcpinfo/src/lib.rs:3", "`mod linux;`"), count("srtt_us", 0)]);
  // 2. The allow moved from srtt_us to a helper, in its exact form: the count stays 2.
  const moved = unsafeWith((f) => {
    const helper = placed("srtt_us").replace("pub fn srtt_us", "fn helper");
    f["crates/tcpinfo/src/linux.rs"] = `pub fn srtt_us() {}\n${placed("unacked_and_unsent")}${helper}`;
  });
  assert.deepEqual(moved, [
    misplaced("crates/tcpinfo/src/linux.rs:10", "`fn helper(fd: BorrowedFd<'_>) -> io::Result<u32> {`"),
    outside("crates/tcpinfo/src/linux.rs:16"),
    count("srtt_us", 0),
  ]);
  // The same onto another public function.
  const other = unsafeWith((f) => {
    f["crates/tcpinfo/src/linux.rs"] = `${placed("unacked_and_unsent")}${placed("other")}pub fn srtt_us() {}\n`;
  });
  assert.deepEqual(other, [
    misplaced("crates/tcpinfo/src/linux.rs:9", "`pub fn other(fd: BorrowedFd<'_>) -> io::Result<u32> {`"),
    outside("crates/tcpinfo/src/linux.rs:15"),
    count("srtt_us", 0),
  ]);
});

test("unsafe_code: on the two functions only the exact allow with its reason counts", () => {
  const forms = {
    "#[allow(unsafe_code)]\n": "no reason",
    '#[allow(unsafe_code, reason = "")]\n': "an empty reason (review S5 (c))",
    '#[allow(unsafe_code, reason = " \\n \\\n ")]\n': "a reason of escapes and blanks",
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
  // A #[path] whose value a macro supplies (rustc 1.99.1 compiles both forms), or a raw string, is not read: refused.
  for (const code of [
    'macro_rules! m {\n    ($p:literal) => {\n        #[path = $p]\n        mod k;\n    };\n}\nm!("../../tcpinfo/src/linux.rs");\n',
    'macro_rules! m {\n    ($p:expr) => {\n        #[path = $p]\n        mod k;\n    };\n}\nm!("../../tcpinfo/src/linux.rs");\n',
    '#[path = r"../../tcpinfo/src/linux.rs"]\nmod k;\n',
  ]) {
    const p = unsafeWith((f) => {
      f["crates/room/src/k.rs"] = code;
    });
    const line = code.startsWith("#") ? 1 : 3;
    assert.deepEqual(p, [`crates/room/src/k.rs:${line} has a #[path] whose value is not a plain string literal, not a .rs file inside crates/room/`], code);
  }
});

/** One unsafe block, as rustc 1.99.1 compiles it once unsafe_code is relaxed. */
const UNSAFE_FN = "pub fn f() -> u8 {\n    let x = 1u8;\n    unsafe { *(&raw const x) }\n}\n";

test("unsafe code outside the two exception crates fails however the lint was relaxed (review S1)", () => {
  const relaxedElsewhere = {
    // Relaxed by RUSTFLAGS=-Aunsafe_code, --cap-lints warn or a cargo config: no attribute to see, the block still shows.
    "rustflags or --cap-lints": UNSAFE_FN,
    // An allow built by a macro from a fragment: rustc 1.99.1 compiles it; the attribute names no lint.
    "a macro-built allow": `macro_rules! lax {\n    ($l:ident) => {\n        #[allow($l)]\n        ${UNSAFE_FN}    };\n}\nlax!(unsafe_code);\n`,
    "an unsafe attribute": '#[unsafe(no_mangle)]\npub extern "C" fn k() {}\n',
    "global_asm! under a macro-built allow": "macro_rules! lax {\n    ($l:ident) => {\n        #[allow($l)]\n        mod m {\n            core::arch::global_asm!(\"\");\n        }\n    };\n}\nlax!(unsafe_code);\n",
  };
  const firstLine = { "rustflags or --cap-lints": 3, "a macro-built allow": 6, "an unsafe attribute": 1, "global_asm! under a macro-built allow": 5 };
  for (const [what, code] of Object.entries(relaxedElsewhere)) {
    const p = unsafeWith((f) => {
      f["crates/room/src/evil.rs"] = code;
    });
    const n = code.match(/\bunsafe\b|\bglobal_asm\b/g).length;
    assert.deepEqual(
      p,
      [`crates/room/src/evil.rs:${firstLine[what]} has unsafe code (${n} \`unsafe\` or \`global_asm\`; only npt-client-wasm and npt-tcpinfo may)`],
      what,
    );
  }
  // Not unsafe code: the word in a comment, a string or a doc, a raw identifier, the lint's own name in a deny.
  const harmless = '#![deny(unsafe_code)]\n// unsafe { }\n/// unsafe\nconst S: &str = "unsafe { }";\nfn r#unsafe() {}\n';
  assert.deepEqual(unsafeWith((f) => (f["crates/room/src/ok.rs"] = harmless)), []);
  // npt-client-wasm's manifest relaxes the lint for the whole crate, so its unsafe code is its own.
  assert.deepEqual(unsafeWith((f) => (f["crates/client-wasm/src/glue.rs"] = UNSAFE_FN)), []);
});

test("lexRust reads a char literal as one code point, so an emoji cannot hide code from the scan", () => {
  // Read unit by unit, the emoji's closing quote would start `','`, a char literal, and the `"` after it a string
  // running to the next `"`, so the unsafe block between would be blanked.
  const src = `const A: [char; 2] = ['\u{1F600}','"'];\n${UNSAFE_FN}const B: &str = "";\n`;
  const { code, mask } = lexRust(src);
  assert.equal(mask.length, src.length);
  assert.match(mask, /unsafe \{ \*\(&raw const x\) \}/, "the block after the literals stays code");
  assert.match(code, /'\u{1F600}'/u);
  const p = unsafeWith((f) => {
    f["crates/room/src/emoji.rs"] = src;
  });
  assert.deepEqual(p, ["crates/room/src/emoji.rs:4 has unsafe code (1 `unsafe` or `global_asm`; only npt-client-wasm and npt-tcpinfo may)"]);
});

test("npt-tcpinfo: unsafe only inside srtt_us and unacked_and_unsent, unsafe_code named only by their allows", () => {
  const linux = cleanCrates()["crates/tcpinfo/src/linux.rs"];
  const at = linux.split("\n").length;
  // An unsafe block in a helper, its allow built by a macro (rustc compiles it; no attribute names the lint).
  const lax = unsafeWith((f) => {
    f["crates/tcpinfo/src/linux.rs"] += `macro_rules! lax {\n    ($l:ident) => {\n        #[allow($l)]\n        ${UNSAFE_FN}    };\n}\nlax!(unsafe_code);\n`;
  });
  assert.deepEqual(lax, [
    "crates/tcpinfo/src/linux.rs:" + (at + 9) + " names unsafe_code outside an attribute (a macro can build the attribute from it)",
    outside(`crates/tcpinfo/src/linux.rs:${at + 5}`),
  ]);
  // In the tests, an example or the stub: outside the two functions.
  for (const file of ["crates/tcpinfo/src/tests.rs", "crates/tcpinfo/examples/probe.rs", "crates/tcpinfo/src/stub.rs"]) {
    assert.deepEqual(unsafeWith((f) => (f[file] = UNSAFE_FN)), [outside(`${file}:3`)], file);
  }
  assert.deepEqual(
    unsafeWith((f) => (f["crates/tcpinfo/src/linux.rs"] += 'core::arch::global_asm!("");\n')),
    [outside(`crates/tcpinfo/src/linux.rs:${at}`, "global_asm")],
  );
  // `pub unsafe fn` puts the keyword outside the body (and the allow no longer sits on `pub fn srtt_us`).
  const unsafeFn = unsafeWith((f) => {
    f["crates/tcpinfo/src/linux.rs"] = placed("unacked_and_unsent") + placed("srtt_us").replace("pub fn", "pub unsafe fn");
  });
  assert.deepEqual(unsafeFn, [
    misplaced("crates/tcpinfo/src/linux.rs:9", "`pub unsafe fn srtt_us(fd: BorrowedFd<'_>) -> io::Result<u32> {`"),
    outside("crates/tcpinfo/src/linux.rs:14"),
    outside("crates/tcpinfo/src/linux.rs:15"),
    count("srtt_us", 0),
  ]);
  // A comment inside the list does not hide a misplaced allow (review S1).
  const comment = unsafeWith((f) => (f["crates/tcpinfo/src/linux.rs"] += "#[allow(dead_code, /* ) */ unsafe_code)]\nfn h() {}\n"));
  assert.deepEqual(comment, [misplaced(`crates/tcpinfo/src/linux.rs:${at}`, "`fn h() {}`")]);
});

test("npt-tcpinfo: the clippy unsafe lints stay denied at the crate root and relaxed nowhere (review S1)", () => {
  const named = (where, name) =>
    `${where} names clippy's ${name} outside crates/tcpinfo/src/lib.rs's crate-level deny (relaxing it would let an ` +
    "unsafe block go without its SAFETY comment or hold more than one unsafe operation)";
  const linux = cleanCrates()["crates/tcpinfo/src/linux.rs"];
  const at = linux.split("\n").length;
  for (const [attr, name] of [
    ["#[allow(clippy::undocumented_unsafe_blocks)]", "undocumented_unsafe_blocks"],
    ["#[expect(clippy::multiple_unsafe_ops_per_block)]", "multiple_unsafe_ops_per_block"],
    ["#[allow(clippy::restriction)]", "restriction"],
    ["#[allow(clippy::r#undocumented_unsafe_blocks)]", "undocumented_unsafe_blocks"],
    ["#[cfg_attr(all(), allow(clippy::restriction))]", "restriction"],
  ]) {
    const p = unsafeWith((f) => (f["crates/tcpinfo/src/linux.rs"] += `${attr}\nfn h() {}\n`));
    assert.deepEqual(p, [named(`crates/tcpinfo/src/linux.rs:${at}`, name)], attr);
  }
  // A macro's argument naming it (the attribute built inside the macro).
  const viaMacro = unsafeWith((f) => (f["crates/tcpinfo/src/tests.rs"] = "lax!(clippy::multiple_unsafe_ops_per_block);\n"));
  assert.deepEqual(viaMacro, [named("crates/tcpinfo/src/tests.rs:1", "multiple_unsafe_ops_per_block")]);
  // The crate-level deny missing, partial, behind cfg_attr, or not at the crate root.
  const missing = "crates/tcpinfo/src/lib.rs does not deny clippy::undocumented_unsafe_blocks and clippy::multiple_unsafe_ops_per_block at the crate root";
  assert.deepEqual(unsafeWith((f) => (f["crates/tcpinfo/src/lib.rs"] = "mod linux;\n")), [missing]);
  assert.deepEqual(unsafeWith((f) => delete f["crates/tcpinfo/src/lib.rs"]), [missing]);
  assert.deepEqual(
    unsafeWith((f) => (f["crates/tcpinfo/src/lib.rs"] = "#![deny(clippy::undocumented_unsafe_blocks)]\nmod linux;\n")),
    [named("crates/tcpinfo/src/lib.rs:1", "undocumented_unsafe_blocks"), missing],
  );
  assert.deepEqual(
    unsafeWith((f) => (f["crates/tcpinfo/src/lib.rs"] = `#![cfg_attr(any(), ${DENY.slice(3, -3)})]\nmod linux;\n`)),
    [named("crates/tcpinfo/src/lib.rs:1", "undocumented_unsafe_blocks"), named("crates/tcpinfo/src/lib.rs:1", "multiple_unsafe_ops_per_block"), missing],
  );
  assert.deepEqual(
    unsafeWith((f) => (f["crates/tcpinfo/src/lib.rs"] = `mod linux;\nfn f() {\n    ${DENY}}\n`)),
    [named("crates/tcpinfo/src/lib.rs:3", "undocumented_unsafe_blocks"), named("crates/tcpinfo/src/lib.rs:3", "multiple_unsafe_ops_per_block"), missing],
  );
  assert.deepEqual(unsafeWith((f) => (f["crates/tcpinfo/src/lib.rs"] = `${DENY.replace("deny", "forbid")}mod linux;\n`)), [], "forbid is fine");
});

/** lintTables' refusal of a lint table header it does not read (review C2b F1). */
const lintTable = (where, header, prefix = "") =>
  `${typeof where === "number" ? `Cargo.toml:${where}` : where} has the lint table ${header}, which this check does not read ` +
  `(only [${prefix}lints] and [${prefix}lints.<tool>], with bare names: not a lint's own table, a quoted part or an array of tables)`;
/** lintTables' refusal of a lint name that is not lowercase with `_` (review C2b F1). */
const lintName = (where, name, table) =>
  `${where} names the lint ${name} in [${table}] in a form this check does not read (cargo reads \`-\` as \`_\`: only lowercase names with \`_\` are read)`;
const TCPINFO_MANIFEST = "crates/tcpinfo/Cargo.toml";
/**
 * exceptionLintProblems on the spike's own manifests, npt-tcpinfo's passed through `change`; `lineOf(text)` is the
 * 1-based line of that exact line in the changed manifest.
 */
function tcpinfoLints(change) {
  const ws = readFileSync(join(HERE, "Cargo.toml"), "utf8");
  const real = {
    [TCPINFO_MANIFEST]: readFileSync(join(HERE, TCPINFO_MANIFEST), "utf8"),
    "crates/client-wasm/Cargo.toml": readFileSync(join(HERE, "crates/client-wasm/Cargo.toml"), "utf8"),
  };
  const text = change(real[TCPINFO_MANIFEST]);
  const lineOf = (needle) => text.split("\n").findIndex((l) => l === needle) + 1;
  return { problems: exceptionLintProblems(ws, (p) => (p === TCPINFO_MANIFEST ? text : real[p])), lineOf };
}

test("the workspace manifest forbids unsafe_code, and nothing else will do (review F-D, C2R-1, C2R-3)", () => {
  const real = readFileSync(join(HERE, "Cargo.toml"), "utf8");
  assert.deepEqual(workspaceLintProblems(real), [], "the spike's own Cargo.toml");
  const none = 'Cargo.toml has no [workspace.lints.rust] unsafe_code = "forbid" for its crates to inherit';
  const line = real.split("\n").findIndex((l) => l.startsWith("unsafe_code")) + 1;
  const other = (n) => `Cargo.toml:${n} names unsafe_code other than as "forbid" in [workspace.lints.rust]`;
  const quoted = (n, table) => `Cargo.toml:${n} has a key in [${table}] that is quoted or dotted, which this check does not read`;
  const level = (v) => workspaceLintProblems(real.replace('unsafe_code = "forbid"', `unsafe_code = ${v}`));
  // deny keeps every crate's lint but drops the compiler backstop: a crate's allow would compile again.
  for (const v of ['"deny"', "'deny'", '{ level = "deny", priority = -1 }', '"allow"', '"warn"', '{ level = "allow", priority = 1 }']) {
    assert.deepEqual(level(v), [other(line), none], v);
  }
  assert.deepEqual(workspaceLintProblems(real.replace('unsafe_code = "forbid"', "")), [none], "the line removed");
  for (const v of ["'forbid'", '{ level = "forbid", priority = -1 }', '"forbid" # a comment']) assert.deepEqual(level(v), [], v);
  const t = (body) => `[workspace]\nmembers = []\n${body}`;
  assert.deepEqual(workspaceLintProblems(t('[workspace.lints.rust]\n# unsafe_code = "allow" in a comment\nunsafe_code = "forbid"\n')), []);
  assert.deepEqual(workspaceLintProblems(t('[workspace.lints.clippy]\nunsafe_code = "forbid"\n')), [other(4), none], "another table");
  assert.deepEqual(
    workspaceLintProblems(t('[workspace.lints]\nrust.unsafe_code = "forbid"\n')),
    [quoted(4, "workspace.lints"), other(4), none],
    "a dotted key (read closed)",
  );
  // A copy of the right lines inside a multi-line string, the real one relaxed below it.
  const decoy = t('[workspace.metadata]\nnote = """\n[workspace.lints.rust]\nunsafe_code = "forbid"\n"""\n[workspace.lints.rust]\nunsafe_code = "allow"\n');
  assert.deepEqual(workspaceLintProblems(decoy), [other(9), none]);
  // The critic's E (review C2R-3): the real key spelt in a quoted key's escapes, which cargo decodes to unsafe_code
  // (cargo check -v passed --allow=unsafe_code), and the decoy in a string: the header and the key are read outside
  // strings, so the decoy is none, and the quoted key is refused.
  const escaped = `${real.replace('unsafe_code = "forbid"', '"unsafe\\u005fcode" = "allow"')}[workspace.metadata.note]\ntext = """\n[workspace.lints.rust]\nunsafe_code = "forbid"\n"""\n`;
  assert.deepEqual(workspaceLintProblems(escaped), [quoted(line, "workspace.lints.rust"), none]);
  // A table name with a quoted part (cargo reads `[workspace."lints".rust]` as the same table) is refused as a header
  // (review C2b F1), and its entry is not read as the forbid.
  const head = real.split("\n").findIndex((l) => l.startsWith("[workspace.lints.rust]")) + 1;
  assert.deepEqual(workspaceLintProblems(real.replace("[workspace.lints.rust]", '[workspace."lints".rust]')), [
    lintTable(head, '[workspace."lints".rust]', "workspace."),
    other(line),
    none,
  ]);
});

test("a lint's own sub-table, a quoted header and an array of lint tables are refused, not skipped (review C2b F1)", () => {
  // The reviewer's plant: both sub-tables with level "allow" relaxed tcpinfo's unsafe_code deny and the clippy deny
  // (cargo 1.99.1 passed --deny=clippy::iter_over_hash_type, then --allow=clippy::iter-over-hash-type) while deps passed.
  const plant =
    '[lints.rust.unsafe-code]\nlevel = "allow"\npriority = 1\n\n[lints.clippy.iter-over-hash-type]\nlevel = "allow"\npriority = 1\n';
  const { problems, lineOf } = tcpinfoLints((t) => `${t}\n${plant}`);
  assert.deepEqual(problems, [
    lintTable(`${TCPINFO_MANIFEST}:${lineOf("[lints.rust.unsafe-code]")}`, "[lints.rust.unsafe-code]"),
    lintTable(`${TCPINFO_MANIFEST}:${lineOf("[lints.clippy.iter-over-hash-type]")}`, "[lints.clippy.iter-over-hash-type]"),
  ]);
  for (const header of ["[lints.clippy.iter_over_hash_type]", '[lints."clippy"]', "[ 'lints' . clippy ]", "[[lints.clippy]]", "[lints.Clippy]"]) {
    const r = tcpinfoLints((t) => `${t}\n${header}\nfoo = "allow"\n`);
    assert.deepEqual(r.problems, [lintTable(`${TCPINFO_MANIFEST}:${r.lineOf(header)}`, header)], header);
  }
  // The workspace's own tables, the same way.
  const t = (body) => `[workspace]\nmembers = []\n[workspace.lints.rust]\nunsafe_code = "forbid"\n${body}`;
  assert.deepEqual(workspaceLintProblems(t('[workspace.lints.clippy.iter_over_hash_type]\nlevel = "allow"\n')), [
    lintTable(5, "[workspace.lints.clippy.iter_over_hash_type]", "workspace."),
  ]);
  // Not a lint table: [lints] in the workspace manifest, a table merely named like one, a string or a comment.
  assert.deepEqual(workspaceLintProblems(t('[workspace.metadata.lints]\nx = 1\n[workspace.lintsx]\ny = 2\n# [workspace.lints.clippy.x]\nz = "[workspace.lints.clippy.x]"\n')), []);
});

test("a lint name with `-` or capitals is refused in a lint table, as cargo reads `-` as `_` (review C2b F1)", () => {
  // A hyphen twin beside the real key: cargo passes both (`--deny=unsafe_code --allow=unsafe-code`), and the later one
  // relaxes the deny (the reviewer's L1b: an unsafe block compiled).
  for (const [table, line] of [
    ["lints.rust", 'unsafe-code = { level = "allow", priority = 1 }'],
    ["lints.clippy", 'iter-over-hash-type = "allow"'],
    ["lints.clippy", 'Iter_Over_Hash_Type = "allow"'],
  ]) {
    const { problems, lineOf } = tcpinfoLints((t) => t.replace(`[${table}]\n`, `[${table}]\n${line}\n`));
    const name = line.split(" ")[0];
    assert.deepEqual(problems, [lintName(`${TCPINFO_MANIFEST}:${lineOf(line)}`, name, table)], line);
  }
  // In the workspace's table, the line scan reports the hyphen spelling too (it names the lint, other than as forbid).
  const real = readFileSync(join(HERE, "Cargo.toml"), "utf8");
  const changed = real.replace('unsafe_code = "forbid"\n', 'unsafe_code = "forbid"\nunsafe-code = "allow"\n');
  const at = changed.split("\n").indexOf('unsafe-code = "allow"') + 1;
  assert.deepEqual(workspaceLintProblems(changed), [
    lintName(`Cargo.toml:${at}`, "unsafe-code", "workspace.lints.rust"),
    `Cargo.toml:${at} names unsafe_code other than as "forbid" in [workspace.lints.rust]`,
  ]);
});

test("lints given as a dotted key or an inline table under another header are refused, quoted parts included (review C2b F1)", () => {
  const dotted = (n) => `Cargo.toml:${n} gives lints as a dotted key or an inline table, which this check does not read`;
  const t = (body) => `[workspace]\nmembers = []\n${body}[workspace.lints.rust]\nunsafe_code = "forbid"\n`;
  for (const body of [
    'lints.clippy.iter-over-hash-type = "allow"\n',
    '"lints".clippy.iter_over_hash_type = "allow"\n',
    "lints . 'clippy' = { iter_over_hash_type = \"allow\" }\n",
    'lints = { clippy = { iter_over_hash_type = "allow" } }\n',
  ]) {
    assert.deepEqual(workspaceLintProblems(t(body)), [dotted(3)], body);
  }
  // At the root, `workspace` as a dotted key or an inline table.
  assert.deepEqual(workspaceLintProblems(`workspace.lints.clippy.x = "allow"\n${t("")}`), [dotted(1)]);
  assert.deepEqual(workspaceLintProblems(`"workspace" = { lints = {} }\n${t("")}`), [dotted(1)]);
  // A `[...]` line inside a value that runs on is no header: the dotted key after the array is still [workspace]'s.
  assert.deepEqual(workspaceLintProblems(t('x = [\n  [1]\n]\nlints.clippy.x = "allow"\n')), [dotted(6)]);
  // A crate's: at its root (quoted or not), or in [lints] beside `workspace = true`.
  const { problems, lineOf } = tcpinfoLints((x) => `"lints".clippy.iter-over-hash-type = { level = "allow", priority = 1 }\n${x}`);
  assert.deepEqual(problems, [
    `${TCPINFO_MANIFEST}:${lineOf('"lints".clippy.iter-over-hash-type = { level = "allow", priority = 1 }')} gives lints as a dotted key or an inline table, which this check does not read`,
  ]);
  // Neither a value that names lints nor another table's `lints` key is a lint.
  assert.deepEqual(workspaceLintProblems(t('[workspace.metadata]\nlints = "lints.rust.unsafe_code"\nnote = { lints = 1 }\n')), []);
});

test("one byte order mark before line 1 is read past, as cargo 1.99.1 reads it (review C2c N4)", () => {
  const BOM = String.fromCharCode(0xfeff);
  // The reviewer's plant: a lint table of a tool the workspace does not name (rustdoc), as a dotted key on line 1 after
  // a BOM; cargo metadata accepted it, and the deps step had passed.
  const plant = 'lints.rustdoc.broken-intra-doc-links = "allow"';
  assert.deepEqual(tcpinfoLints((t) => `${BOM}${plant}\n${t}`).problems, [
    `${TCPINFO_MANIFEST}:1 gives lints as a dotted key or an inline table, which this check does not read`,
  ]);
  // A key of a lint table on line 1 is read as written too: at the workspace manifest's root, a dotted key.
  assert.deepEqual(workspaceLintProblems(`${BOM}workspace.lints.rust.unsafe_code = "forbid"\n[workspace]\nmembers = []\n`), [
    "Cargo.toml:1 gives lints as a dotted key or an inline table, which this check does not read",
    'Cargo.toml:1 names unsafe_code other than as "forbid" in [workspace.lints.rust]',
    'Cargo.toml has no [workspace.lints.rust] unsafe_code = "forbid" for its crates to inherit',
  ]);
  // No false positive: the spike's manifests with a BOM read as they do without.
  const ws = readFileSync(join(HERE, "Cargo.toml"), "utf8");
  assert.deepEqual(workspaceLintProblems(`${BOM}${ws}`), []);
  assert.deepEqual(tcpinfoLints((t) => `${BOM}${t}`).problems, []);
  assert.deepEqual(unsafeWith((f) => (f["crates/sim/Cargo.toml"] = `${BOM}${f["crates/sim/Cargo.toml"]}`)), []);
  assert.deepEqual(manifestKeyProblems([{ path: "x", text: `${BOM}[package]\nname = "x"\n` }]), []);
});

test("a manifest naming unsafe-code (cargo's spelling of unsafe_code) is caught by the line scans too (review C2b F1, L2)", () => {
  // The workspace manifest: a line naming the lint in any table but as the forbid entry.
  const t = (body) => `[workspace]\nmembers = []\n[workspace.lints.rust]\nunsafe_code = "forbid"\n${body}`;
  assert.deepEqual(workspaceLintProblems(t("[workspace.metadata]\nunsafe-code = 1\n")), [
    'Cargo.toml:6 names unsafe_code other than as "forbid" in [workspace.lints.rust]',
  ]);
  // Any other crate's manifest (cargo refuses own lint tables beside `workspace = true`; the scan reports it anyway).
  assert.deepEqual(
    unsafeWith((f) => (f["crates/sim/Cargo.toml"] = '[package]\nname = "npt-sim"\n\n[lints]\nworkspace = true\n\n[lints.rust.unsafe-code]\nlevel = "allow"\n')),
    ["crates/sim/Cargo.toml sets unsafe_code"],
  );
});

test("the two exceptions hold the workspace's other lints in their own tables, and their own level (review C2R-1)", () => {
  const ws = readFileSync(join(HERE, "Cargo.toml"), "utf8");
  const T = "crates/tcpinfo/Cargo.toml";
  const W = "crates/client-wasm/Cargo.toml";
  const real = { [T]: readFileSync(join(HERE, T), "utf8"), [W]: readFileSync(join(HERE, W), "utf8") };
  const read = (changed) => (p) => {
    if (!(p in real)) throw new Error(`no ${p}`);
    return changed[p] ?? real[p];
  };
  assert.deepEqual(exceptionLintProblems(ws, read({})), [], "the spike's own manifests");
  const tcp = (from, to) => exceptionLintProblems(ws, read({ [T]: real[T].replace(from, to) }));
  assert.deepEqual(tcp('iter_over_hash_type = "deny"\n', ""), [`${T} has no [lints.clippy] iter_over_hash_type: the workspace's is "deny"`]);
  assert.deepEqual(tcp('disallowed_types = "deny"', 'disallowed_types = "warn"'), [
    `${T} sets [lints.clippy] disallowed_types = "warn", not the workspace's "deny"`,
  ]);
  assert.deepEqual(tcp("[lints.clippy]\n", '[lints.clippy]\nundocumented_unsafe_blocks = "allow"\n'), [
    `${T} sets [lints.clippy] undocumented_unsafe_blocks = "allow", which the workspace does not`,
  ]);
  assert.deepEqual(tcp('unsafe_code = "deny"\n', 'unsafe_code = "deny"\nunused = "allow"\n'), [`${T} sets [lints.rust] unused = "allow", which the workspace does not`]);
  // Its own level: deny, which its two #[allow]s can relax; forbid would refuse them, allow or warn relax the crate.
  for (const [v, shown] of [['"forbid"', '"forbid"'], ['"allow"', '"allow"'], ['"warn"', '"warn"'], ['{ level = "deny" }', '{level="deny"}']]) {
    assert.deepEqual(tcp('unsafe_code = "deny"', `unsafe_code = ${v}`), [`${T} sets [lints.rust] unsafe_code = ${shown}, not "deny"`], v);
  }
  assert.deepEqual(tcp('unsafe_code = "deny"', "unsafe_code = 'deny'"), [], "a literal string is the same value");
  assert.deepEqual(tcp('unsafe_code = "deny"\n', ""), [`${T} sets [lints.rust] unsafe_code = nothing, not "deny"`]);
  // The workspace's lints inherited instead (u3c-plan.md 1.8 (a)'s old form): forbid there would refuse the allows.
  const inherited = exceptionLintProblems(ws, read({ [T]: '[package]\nname = "npt-tcpinfo"\n\n[lints]\nworkspace = true\n' }));
  assert.ok(inherited.includes(`${T} sets [lints.rust] unsafe_code = nothing, not "deny"`), inherited.join("\n"));
  assert.ok(inherited.includes(`${T} has no [lints.clippy] disallowed_types: the workspace's is "deny"`), inherited.join("\n"));
  // Read closed: a quoted key, and a copy of the dropped line inside a string, which is none.
  const at = real[T].split("\n").findIndex((l) => l.startsWith("disallowed_types")) + 1;
  assert.deepEqual(tcp('disallowed_types = "deny"', '"disallowed_types" = "deny"'), [
    `${T}:${at} has a key in [lints.clippy] that is quoted or dotted, which this check does not read`,
    `${T} has no [lints.clippy] disallowed_types: the workspace's is "deny"`,
  ]);
  const decoy = exceptionLintProblems(
    ws,
    read({ [T]: `${real[T].replace('disallowed_types = "deny"\n', "")}[package.metadata]\nnote = """\n[lints.clippy]\ndisallowed_types = "deny"\n"""\n` }),
  );
  assert.deepEqual(decoy, [`${T} has no [lints.clippy] disallowed_types: the workspace's is "deny"`]);
  // npt-client-wasm: allow, and the same other lints.
  assert.deepEqual(exceptionLintProblems(ws, read({ [W]: real[W].replace('unsafe_code = "allow"', 'unsafe_code = "deny"') })), [
    `${W} sets [lints.rust] unsafe_code = "deny", not "allow"`,
  ]);
  assert.deepEqual(exceptionLintProblems(ws, read({ [W]: real[W].replace('disallowed_methods = "deny"\n', "") })), [
    `${W} has no [lints.clippy] disallowed_methods: the workspace's is "deny"`,
  ]);
  // A lint the workspace adds must be added to both.
  const more = ws.replace('iter_over_hash_type = "deny"\n', 'iter_over_hash_type = "deny"\nundocumented_unsafe_blocks = "deny"\n');
  assert.deepEqual(exceptionLintProblems(more, read({})), [
    `${W} has no [lints.clippy] undocumented_unsafe_blocks: the workspace's is "deny"`,
    `${T} has no [lints.clippy] undocumented_unsafe_blocks: the workspace's is "deny"`,
  ]);
  assert.deepEqual(exceptionLintProblems(ws, () => {
    throw new Error("absent");
  }), [`${W} cannot be read`, `${T} cannot be read`]);
});

/** cargo metadata's shape for layoutProblems: members by folder, each with its targets ([kind, name, src, edition?]). */
function layoutMeta(members) {
  const packages = Object.entries(members).map(([dir, targets]) => ({
    id: `path+file:///w/${dir}#0.0.0`,
    name: `npt-${dir.split("/").pop() || "root"}`,
    manifest_path: `/w/${dir ? `${dir}/` : ""}Cargo.toml`,
    targets: targets.map(([kind, name, src, edition = "2024"]) => ({
      kind: [kind],
      name,
      src_path: src.startsWith("/") ? src : `/w/${dir ? `${dir}/` : ""}${src}`,
      edition,
    })),
  }));
  return { workspace_root: "/w", workspace_members: packages.map((p) => p.id), packages };
}

test("every member lives in crates/ and builds only .rs files in its own folder, in edition 2024 (review F-E, S5 (c))", () => {
  const clean = {
    "crates/room": [["lib", "npt_room", "src/lib.rs"], ["test", "rooms", "tests/rooms.rs"]],
    "crates/tcpinfo": [["lib", "npt_tcpinfo", "src/lib.rs"], ["example", "siocoutq_probe", "examples/siocoutq_probe.rs"]],
    "crates/bots": [["bin", "npt-bots", "src/bin/npt-bots.rs"], ["custom-build", "build-script-build", "build.rs"]],
  };
  assert.deepEqual(layoutProblems(layoutMeta(clean)), []);
  const withTarget = (dir, target) => layoutProblems(layoutMeta({ ...clean, [dir]: [...clean[dir], target] }));
  for (const [kind, src] of [
    ["lib", "../tcpinfo/src/linux.rs"],
    ["bin", "src/../../tcpinfo/src/linux.rs"],
    ["example", "/elsewhere/x.rs"],
    ["test", "../room-other/tests/t.rs"],
    ["bench", "../../benches/b.rs"],
    ["custom-build", "../tcpinfo/build.rs"],
  ]) {
    const p = withTarget("crates/room", [kind, "planted", src]);
    assert.equal(p.length, 1, `${kind} ${src}: ${p.join("\n")}`);
    assert.match(p[0], new RegExp(`^npt-room's ${kind} target planted compiles .*, outside its folder crates/room/$`));
  }
  assert.deepEqual(withTarget("crates/room", ["lib", "planted", "src/lib.txt"]), [
    "npt-room's lib target planted compiles crates/room/src/lib.txt, not a .rs file the scan reads",
  ]);
  assert.deepEqual(withTarget("crates/room", ["lib", "planted", "src/x.rs", "2021"]), ["npt-room's lib target planted is edition 2021, not 2024"]);
  // A proc-macro member: its expansions are another crate's macro code, which rustc 1.99.1 does not lint even under
  // --forbid unsafe_code (probe: an `unsafe` built from strings compiled in the caller).
  assert.deepEqual(withTarget("crates/room", ["proc-macro", "planted", "src/pm.rs"]), [
    "npt-room's proc-macro target planted is a proc-macro crate, whose expansions the lint and the scan cannot see",
  ]);
  // A path dependency outside the workspace is code nobody scans; a registry package is a dependency like any other.
  const outside = layoutMeta(clean);
  outside.packages.push({ id: "path+file:///x/evil#0.0.0", name: "evil", manifest_path: "/x/evil/Cargo.toml", source: null, targets: [] });
  outside.packages.push({ id: "registry+x#libc@0.2.191", name: "libc", manifest_path: "/r/libc/Cargo.toml", source: "registry+x", targets: [] });
  assert.deepEqual(layoutProblems(outside), ["evil (../x/evil/Cargo.toml) is a path dependency outside the workspace, which the scan does not read"]);
  // A member outside crates/: the workspace root itself, or another folder.
  for (const dir of ["", "tools/extra"]) {
    const p = layoutProblems(layoutMeta({ ...clean, [dir]: [["lib", "x", "src/lib.rs"]] }));
    const manifest = dir ? `${dir}/Cargo.toml` : "Cargo.toml";
    assert.deepEqual(p, [`npt-${dir.split("/").pop() || "root"}'s manifest is ${manifest}, not crates/<folder>/Cargo.toml (the unsafe_code scan reads crates/ only)`]);
  }
});

test("rustflags, cargo configs and aliases that would relax a lint are refused (review S1, F-D)", () => {
  assert.deepEqual(cargoConfigProblems({ PATH: "/bin", RUSTFLAGS: "-C target-cpu=native -D warnings" }, []), []);
  for (const [k, v, flag] of [
    ["RUSTFLAGS", "-Aunsafe_code", "-Aunsafe_code"],
    ["RUSTFLAGS", "-C opt-level=1 --cap-lints=warn", "--cap-lints=warn"],
    ["RUSTFLAGS", "--allow unsafe_code", "--allow"],
    ["CARGO_ENCODED_RUSTFLAGS", "-C\x1fopt-level=1\x1f--force-warn\x1funsafe_code", "--force-warn"],
    ["CARGO_BUILD_RUSTFLAGS", "-Zcrate-attr=allow(unsafe_code)", "-Zcrate-attr=allow(unsafe_code)"],
    ["CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS", "-W unsafe_code", "-W"],
    ["RUSTFLAGS", "@flags.txt", "@flags.txt"],
  ]) {
    assert.deepEqual(cargoConfigProblems({ [k]: v }, []), [`${k} in the environment passes ${JSON.stringify(flag)} to rustc: a flag that can relax unsafe_code`], `${k}=${v}`);
  }
  assert.deepEqual(cargoConfigProblems({ CARGO_ALIAS_CLIPPY: "check" }, []), ["CARGO_ALIAS_CLIPPY in the environment replaces `cargo clippy` with an alias"]);
  // Config files: the spike's own (a target-dir) passes; each planted rustflags or alias fails.
  const own = { path: "spike/.cargo/config.toml", text: readFileSync(join(HERE, ".cargo", "config.toml"), "utf8") };
  assert.deepEqual(cargoConfigProblems({}, [own]), []);
  const config = (text) => cargoConfigProblems({}, [{ path: "c.toml", text }]);
  assert.deepEqual(config('[build]\nrustflags = [\n    "-C", # a comment\n    "link-arg=-Wl,-z,relro",\n]\n[env]\nX = "rustflags = -A"\n'), []);
  for (const [text, flag] of [
    ['[build]\nrustflags = ["-A", "unsafe_code"]\n', "-A"],
    ["[target.'cfg(unix)']\nrustflags = \"-C opt-level=1 --cap-lints warn\"\n", "--cap-lints"],
    ['build.rustflags = ["\\u002DAunsafe_code"]\n', "-Aunsafe_code"],
    ['[build]\n"rustflags" = [\'--force-warn=unsafe_code\']\n', "--force-warn=unsafe_code"],
    ['[target.x86_64-unknown-linux-gnu]\nrustflags = """\n-W unsafe_code"""\n', "-W"],
  ]) {
    assert.ok(
      config(text).some((p) => p.startsWith(`c.toml's rustflags passes ${JSON.stringify(flag)} to rustc`)),
      `${text}: ${config(text).join("\n")}`,
    );
  }
  assert.deepEqual(config("[build]\nrustflags = 3\n"), ["c.toml has a rustflags value this check cannot read"]);
  assert.deepEqual(config('[alias]\nclippy = "check"\n'), ["c.toml has a key `clippy`: an alias that replaces `cargo clippy`"]);
  assert.deepEqual(config('alias.fmt = "version"\n'), ["c.toml has a key `fmt`: an alias that replaces `cargo fmt`"]);
  assert.deepEqual(config('alias = { "clippy" = ["check"] }\n'), ["c.toml has a key `clippy`: an alias that replaces `cargo clippy`"]);
});

test("the crate walk reads every folder and reports symbolic links it does not follow (review F-E)", (t) => {
  const tmp = mkdtempSync(join(tmpdir(), "npt-check-"));
  try {
    const crates = join(tmp, "crates");
    for (const f of ["room/Cargo.toml", "room/src/lib.rs", "room/src/target/mod.rs", "room/src/.hidden/x.rs", "tcpinfo/src/linux.rs", "room/README.md"]) {
      mkdirSync(dirname(join(crates, f)), { recursive: true });
      writeFileSync(join(crates, f), "\n");
    }
    let linked = true;
    try {
      symlinkSync("../../tcpinfo/src/linux.rs", join(crates, "room/src/k.rs"));
      symlinkSync("../../tcpinfo/src", join(crates, "room/src/borrowed"), "dir");
    } catch {
      linked = false;
    }
    const { files, links } = crateFiles(crates);
    // Node 22's walk may also list a linked folder's files (as crates/room/src/borrowed/...); a link is refused anyway.
    const own = files.filter((f) => !f.startsWith("crates/room/src/borrowed/"));
    assert.deepEqual(own, [
      "crates/room/Cargo.toml",
      "crates/room/src/.hidden/x.rs",
      "crates/room/src/lib.rs",
      "crates/room/src/target/mod.rs",
      "crates/tcpinfo/src/linux.rs",
    ]);
    if (!linked) t.diagnostic("symbolic links cannot be made here: the link half is not exercised");
    else assert.deepEqual(links, ["crates/room/src/borrowed", "crates/room/src/k.rs"]);
  } finally {
    rmSync(tmp, { recursive: true, force: true });
  }
});

test("another rustc, a workspace wrapper, a config include and a test runner are refused; this container's environment is not (review C2R-4, C2R-7, C2R-8)", () => {
  // This container's cargo environment (2026-11-08: CARGO_HTTP_CAINFO, RUSTUP_HOME, RUST_BACKTRACE), the target folder
  // every unit passes, rustup's toolchain variable and sccache as RUSTC_WRAPPER: none of it is refused.
  const normal = {
    PATH: "/usr/local/bin:/usr/bin",
    HOME: "/root",
    CARGO_HTTP_CAINFO: "/root/.ccr/ca-bundle.crt",
    RUSTUP_HOME: "/root/.rustup",
    RUST_BACKTRACE: "1",
    CARGO_TARGET_DIR: "/home/user/Shamble-Frontlines/.gate/u3v3-model-target",
    RUSTUP_TOOLCHAIN: "1.99.1",
    RUSTC_WRAPPER: "sccache",
    CARGO_BUILD_RUSTC_WRAPPER: "/usr/bin/sccache",
    // `rustup run 1.99.1` adds these; git's own (2026-11-08, names as found here) are not cargo's.
    CARGO_HOME: "/root/.cargo",
    RUST_RECURSION_COUNT: "1",
    GIT_CONFIG_COUNT: "1",
    GIT_CONFIG_KEY_0: "credential.interactive",
    GIT_CONFIG_VALUE_0: "never",
    GIT_TERMINAL_PROMPT: "0",
  };
  assert.deepEqual(cargoConfigProblems(normal, []), []);
  const config = (text) => cargoConfigProblems({}, [{ path: "c.toml", text }]);
  assert.deepEqual(config('[build]\nrustc-wrapper = "sccache"\ntarget-dir = "t"\n[target.x86_64-unknown-linux-gnu]\nlinker = "cc"\n'), []);
  // C2R-8: another compiler, or a wrapper around the workspace crates' rustc calls.
  for (const k of ["RUSTC", "CARGO_BUILD_RUSTC", "RUSTC_WORKSPACE_WRAPPER", "CARGO_BUILD_RUSTC_WORKSPACE_WRAPPER"]) {
    assert.deepEqual(cargoConfigProblems({ ...normal, [k]: "/opt/other/rustc" }, []), [
      `${k} in the environment replaces rustc or wraps the workspace's rustc calls (only RUSTC_WRAPPER may wrap them)`,
    ]);
  }
  for (const [text, key] of [
    ['[build]\nrustc = "/opt/other/rustc"\n', "rustc"],
    ['build.rustc-workspace-wrapper = "w"\n', "rustc-workspace-wrapper"],
    ['[build]\n"rustc" = "r"\n', "rustc"],
  ]) {
    assert.deepEqual(config(text), [`c.toml has a key \`${key}\`: it replaces rustc or wraps the workspace's rustc calls`], text);
  }
  // C2R-4 (the critic's F): an include of a file this check never reads, which cargo 1.99.1 loads on stable.
  for (const text of ['include = ["extra.toml"]\n[build]\ntarget-dir = "t"\n', 'include = "extra.toml"\n', '[[include]]\npath = "extra.toml"\n']) {
    assert.deepEqual(config(text), ["c.toml has a key `include`: it loads a config file this check does not read"], text);
  }
  // The table's name quoted, which cargo 1.99.1 loads as [[include]] (review C2c N2's reader list: a probe's alias from
  // the included file ran), in escapes too; a line inside an array that only looks like one is refused as well.
  for (const text of ['[["include"]]\npath = "extra.toml"\n', "[[ 'include' ]]\npath = \"extra.toml\"\n", '[["incl\\u0075de"]]\npath = "x"\n', 'x = [\n[["include"]]\n]\n']) {
    assert.deepEqual(config(text), ["c.toml has a key `include`: it loads a config file this check does not read"], text);
  }
  assert.deepEqual(config('[build]\njobs = 1\n[env]\nX = "[[include]]"\n["included"]\nx = 1\n'), [], "a string, and another name");
  // C2R-7: a runner `true` makes cargo test exit 0 with no test run.
  assert.deepEqual(cargoConfigProblems({ ...normal, CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUNNER: "true" }, []), [
    "CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUNNER in the environment runs the test binaries in cargo's place",
  ]);
  for (const text of ['[target.x86_64-unknown-linux-gnu]\nrunner = "true"\n', "[target.'cfg(unix)']\nrunner = [\"true\"]\n", 'target.x86_64-unknown-linux-gnu.runner = "true"\n']) {
    assert.deepEqual(config(text), ["c.toml has a key `runner`: it runs the test binaries in cargo's place"], text);
  }
});

test("RUSTC_BOOTSTRAP, cargo's channel override, and a manifest's cargo-features or rustflags key, are refused; the spike's own manifests are not (review C2b F5, C2c N3)", () => {
  // The reviewer's bypass: RUSTC_BOOTSTRAP=1, `cargo-features = ["profile-rustflags"]` and a profile's
  // `rustflags = ["--cap-lints", "warn"]` compiled a planted allow and unsafe block under the forbid, no warning.
  const bootstrap = "RUSTC_BOOTSTRAP in the environment unlocks unstable cargo and rustc features (a manifest's profile rustflags)";
  for (const v of ["1", "npt_room", "-1", ""]) assert.deepEqual(cargoConfigProblems({ PATH: "/bin", RUSTC_BOOTSTRAP: v }, []), [bootstrap], v);
  // Cargo's own test knob (review C2c N3): set to nightly, stable cargo 1.99.1 accepted `cargo-features` (cargo metadata
  // exit 0; 101 without it). Refused whatever its value; a name that merely resembles it is not.
  const channel =
    "__CARGO_TEST_CHANNEL_OVERRIDE_DO_NOT_USE_THIS in the environment sets the release channel cargo reads (nightly " +
    "unlocks its unstable features: a manifest's cargo-features)";
  for (const v of ["nightly", "dev", "stable", ""]) {
    assert.deepEqual(cargoConfigProblems({ PATH: "/bin", __CARGO_TEST_CHANNEL_OVERRIDE_DO_NOT_USE_THIS: v }, []), [channel], v);
  }
  assert.deepEqual(cargoConfigProblems({ __CARGO_TEST_CHANNEL_OVERRIDE: "nightly", CARGO_TEST_CHANNEL: "nightly", constructor: "x" }, []), []);
  const features = (p) => `${p} has a key \`cargo-features\`: it unlocks unstable manifest keys (a profile's rustflags among them)`;
  const flags = (p) =>
    `${p} has a key \`rustflags\`: a manifest's rustflags reach rustc (\`--cap-lints warn\` there turns the workspace's forbid into a warning)`;
  const attack = 'cargo-features = ["profile-rustflags"]\n[workspace]\nmembers = []\n[profile.dev]\nrustflags = ["--cap-lints", "warn"]\n';
  assert.deepEqual(manifestKeyProblems([{ path: "Cargo.toml", text: attack }]), [features("Cargo.toml"), flags("Cargo.toml")]);
  // Any rustflags key, in any table or form (a member's manifest included), whatever its value.
  for (const text of ["[profile.dev.package.npt-room]\nrustflags = []\n", 'profile.release.rustflags = ["-C", "opt-level=1"]\n', '[profile]\ndev = { "rustflags" = [] }\n']) {
    assert.deepEqual(manifestKeyProblems([{ path: "crates/room/Cargo.toml", text }]), [flags("crates/room/Cargo.toml")], text);
  }
  assert.deepEqual(manifestKeyProblems([{ path: "crates/room/Cargo.toml", text: "'cargo-features' = []\n[package]\n" }]), [features("crates/room/Cargo.toml")]);
  // A string or a comment that names them is no key.
  assert.deepEqual(manifestKeyProblems([{ path: "x", text: '[package]\ndescription = "rustflags = 1; cargo-features = []"\n# rustflags = ["-A"]\n' }]), []);
  // No false positive: the spike's workspace manifest and every crate manifest under crates/ (what depsStep reads).
  const own = [{ path: "Cargo.toml", text: readFileSync(join(HERE, "Cargo.toml"), "utf8") }];
  for (const f of crateFiles(join(HERE, "crates")).files.filter((f) => f.endsWith("/Cargo.toml"))) own.push({ path: f, text: readFileSync(join(HERE, f), "utf8") });
  assert.ok(own.length >= 17, `${own.length} manifests`);
  assert.deepEqual(manifestKeyProblems(own), []);
});

/** The tick grep's message for a tick-count name (not a derived one) set from `init` at `at` (u4-build-plan.md §4.3). */
const fromLiteral = (at, name, init) =>
  `${at} sets ${name} from \`${init}\`: a tick-counted value is a const fn of npt_sim::TICK_HZ, never a literal ` +
  "(u4-build-plan.md §4.3)";
/** Its message for a derived name set outside npt-sim's sources other than by a path to npt-sim's (plan §4.6). */
const notAPath = (at, name, init) =>
  `${at} sets ${name} from \`${init}\` outside crates/sim/src/: there it is a path to npt_sim::${name}, never derived ` +
  "again (one home: plan §4.6)";
/** Its message for a derived name in npt-sim's sources that is not its own function at TICK_HZ (§4.3; rust R4). */
const notOwnFn = (at, name, init) =>
  `${at} sets ${name} from \`${init}\`: in npt-sim it is its own function at the tick rate, ` +
  `\`${name.toLowerCase()}_at(TICK_HZ)\`, which s1 pins at 30 and 60 Hz (u4-build-plan.md §4.3)`;
/** The float grep's message for its first token in `at` (u4-build-plan.md §4.11). */
const roomFloat = (at, token, n = 1) => `${at} has \`${token}\` (${n} float token(s)): npt-room is float-free (u4-build-plan.md §4.11)`;
/** The bite-id grep's message for `id` planted at `sites` (each crate's first file:line; u4-build-plan.md §4.14). */
const biteTwice = (id, ...sites) =>
  `${id} is planted in ${sites.length} crates, ${sites.join(" and ")}: a bite's id lives in the one crate whose code it ` +
  "plants (u4-build-plan.md §4.14)";

/**
 * A spike folder for depsStep (the workspace manifest, the two exceptions, npt-room and a cargo config) and cargo
 * metadata's shape for it, from clean()'s graph: every member at crates/<name without npt->/ with one lib target there.
 * `plant` changes the files, `change` the graph.
 */
function depsFixture(t, plant = () => {}, change = () => {}) {
  const dir = mkdtempSync(join(tmpdir(), "npt-deps-"));
  t.after(() => rmSync(dir, { recursive: true, force: true }));
  const files = {
    "Cargo.toml": '[workspace]\nmembers = ["crates/*"]\n\n[workspace.lints.rust]\nunsafe_code = "forbid"\n\n[workspace.lints.clippy]\ndisallowed_types = "deny"\n',
    "crates/client-wasm/Cargo.toml": '[package]\nname = "npt-client-wasm"\n\n[lints.rust]\nunsafe_code = "allow"\n\n[lints.clippy]\ndisallowed_types = "deny"\n',
    "crates/tcpinfo/Cargo.toml": '[package]\nname = "npt-tcpinfo"\n\n[lints.rust]\nunsafe_code = "deny"\n\n[lints.clippy]\ndisallowed_types = "deny"\n',
    "crates/tcpinfo/src/lib.rs": `${DENY}mod linux;\n`,
    "crates/tcpinfo/src/linux.rs": `${placed("srtt_us")}${placed("unacked_and_unsent")}`,
    "crates/room/Cargo.toml": '[package]\nname = "npt-room"\n\n[lints]\nworkspace = true\n',
    "crates/room/src/lib.rs": "pub fn f() {}\n",
    ".cargo/config.toml": '[build]\ntarget-dir = "t"\n',
  };
  plant(files);
  for (const [f, text] of Object.entries(files)) {
    mkdirSync(dirname(join(dir, f)), { recursive: true });
    writeFileSync(join(dir, f), text);
  }
  const edges = clean();
  change(edges);
  const m = meta(edges);
  m.workspace_root = dir;
  for (const p of m.packages) {
    if (!m.workspace_members.includes(p.id)) {
      Object.assign(p, { source: "registry+https://example.invalid/index", targets: [] });
      continue;
    }
    const folder = join(dir, "crates", p.name.slice(4));
    Object.assign(p, { source: null, manifest_path: join(folder, "Cargo.toml") });
    p.targets = [{ kind: ["lib"], name: p.name, src_path: join(folder, "src", "lib.rs"), edition: "2024" }];
  }
  return { dir, meta: m, env: { CARGO_HOME: join(dir, "no-cargo-home") } };
}

test("the deps step is depsStep, and it calls every check: one plant per check, each reported (review C2R-5)", (t) => {
  assert.equal(STEPS.deps, depsStep, "check.mjs's deps step runs this function");
  const ok = depsFixture(t);
  const clean = depsStep(ok.dir, ok.env, () => ({ ok: true, meta: ok.meta }));
  assert.equal(clean.ok, true, clean.out);
  assert.match(clean.out, /^held: .* \(6 crate files and 1 cargo config file\(s\) read\)$/);
  const bad = depsFixture(
    t,
    (f) => {
      f["Cargo.toml"] = `cargo-features = ["profile-rustflags"]\n${f["Cargo.toml"].replace('"forbid"', '"deny"')}`;
      // A member's manifest too (review C2c N1: dropping depsStep's member-manifest loop had passed every test).
      f["crates/room/Cargo.toml"] += '\n[profile.dev]\nrustflags = ["--cap-lints", "warn"]\n';
      f["crates/tcpinfo/Cargo.toml"] = f["crates/tcpinfo/Cargo.toml"].replace('disallowed_types = "deny"\n', "");
      f[".cargo/config.toml"] = `include = ["extra.toml"]\n${f[".cargo/config.toml"]}`;
      f["crates/room/src/evil.rs"] = UNSAFE_FN;
      // U4s's three greps (u4-build-plan.md §4.3, §4.11, §4.14).
      f["crates/room/src/ticks.rs"] = "pub const LOOP_CATCHUP: u32 = 7;\npub fn g() -> f64 {\n    0.0\n}\n";
      f["crates/sim/src/grid.rs"] = 'pub const BITES: [&str; 1] = ["PM-0001-17/client-rounding"];\n';
      f["crates/client/src/lib.rs"] = 'pub const BITES: [&str; 1] = ["PM-0001-17/client-rounding"];\n';
    },
    (e) => (e["npt-sim"] = n("tokio")),
  );
  bad.meta.packages.find((p) => p.name === "npt-room").targets[0].src_path = join(bad.dir, "crates", "tcpinfo", "src", "linux.rs");
  let linked = true;
  try {
    symlinkSync("lib.rs", join(bad.dir, "crates", "room", "src", "k.rs"));
  } catch {
    linked = false;
  }
  const r = depsStep(bad.dir, bad.env, () => ({ ok: true, meta: bad.meta }));
  assert.equal(r.ok, false);
  const lines = r.out.split("\n");
  for (const [check, line] of [
    ["dependencyProblems", "leaf crate npt-sim depends on tokio"],
    ["workspaceLintProblems", 'Cargo.toml has no [workspace.lints.rust] unsafe_code = "forbid" for its crates to inherit'],
    ["exceptionLintProblems", `crates/tcpinfo/Cargo.toml has no [lints.clippy] disallowed_types: the workspace's is "deny"`],
    ["manifestKeyProblems", "Cargo.toml has a key `cargo-features`: it unlocks unstable manifest keys (a profile's rustflags among them)"],
    [
      "manifestKeyProblems (a member)",
      "crates/room/Cargo.toml has a key `rustflags`: a manifest's rustflags reach rustc (`--cap-lints warn` there turns the workspace's forbid into a warning)",
    ],
    ["layoutProblems", "npt-room's lib target npt-room compiles crates/tcpinfo/src/linux.rs, outside its folder crates/room/"],
    ["cargoConfigProblems", `${join(bad.dir, ".cargo", "config.toml")} has a key \`include\`: it loads a config file this check does not read`],
    ["unsafeProblems", "crates/room/src/evil.rs:3 has unsafe code (1 `unsafe` or `global_asm`; only npt-client-wasm and npt-tcpinfo may)"],
    ["tickConstantProblems", notAPath("crates/room/src/ticks.rs:1", "LOOP_CATCHUP", "7")],
    ["roomFloatProblems", roomFloat("crates/room/src/ticks.rs:2", "f64", 2)],
    ["biteHomeProblems", biteTwice("PM-0001-17/client-rounding", "crates/client/src/lib.rs:1", "crates/sim/src/grid.rs:1")],
  ]) {
    assert.ok(lines.includes(line), `${check}'s plant is not reported:\n${r.out}`);
  }
  if (!linked) t.diagnostic("symbolic links cannot be made here: the link refusal is not exercised");
  else assert.ok(lines.includes("crates/room/src/k.rs is a symbolic link: the scan does not follow it, and rustc would"), r.out);
  // The environment it is given is the one checked (process.env when check.mjs runs it).
  const runner = depsStep(ok.dir, { ...ok.env, CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUNNER: "true" }, () => ({ ok: true, meta: ok.meta }));
  assert.equal(runner.out, "CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUNNER in the environment runs the test binaries in cargo's place");
  // And so is its CARGO_HOME (review C2b F3): a config there that must be refused is read from the CARGO_HOME given, not
  // from this process's ~/.cargo.
  const home = join(ok.dir, "cargo-home");
  mkdirSync(home);
  writeFileSync(join(home, "config.toml"), '[build]\nrustc = "/opt/other/rustc"\n');
  const homed = depsStep(ok.dir, { ...ok.env, CARGO_HOME: home }, () => ({ ok: true, meta: ok.meta }));
  assert.equal(homed.out, `${join(home, "config.toml")} has a key \`rustc\`: it replaces rustc or wraps the workspace's rustc calls`);
  // cargo metadata failing fails the step with its output.
  assert.deepEqual(depsStep(ok.dir, ok.env, () => ({ ok: false, out: "error: no Cargo.lock" })), { ok: false, out: "error: no Cargo.lock" });
});

test("the test step counts and names a test that reported a skip (review C2R-9)", () => {
  const out =
    "running 2 tests\ntest a ... ok\ntest b ... ok\n\ntest result: ok. 2 passed; 0 failed; 0 ignored; 0 measured\n" +
    "test result: ok. 5 passed; 0 failed; 1 ignored; 0 measured\n" +
    "SKIPPED an_mptcp_socket_is_not_tcp_for_either_reading: no MPTCP socket on this kernel (Protocol not available (os error 92))\n";
  assert.equal(cargoCounts(out), "7 tests passed, 0 failed, 1 reported a skip (an_mptcp_socket_is_not_tcp_for_either_reading)");
  assert.equal(cargoCounts("test result: ok. 3 passed; 0 failed; 0 ignored"), "3 tests passed, 0 failed, 0 reported a skip");
  // The line npt-tcpinfo's tests write is the one counted here.
  const rs = readFileSync(join(HERE, "crates", "tcpinfo", "src", "tests.rs"), "utf8");
  assert.ok(rs.includes('format!("SKIPPED {test}: {why}\\n")'), "crates/tcpinfo/src/tests.rs writes `SKIPPED <test>: <why>`");
  // And the real MPTCP test hands mptcp_case the process's stderr, the one writer past libtest's capture (review C2b F4):
  // given io::sink() or a buffer, its skip would reach nobody, and on a host with MPTCP no Rust test can tell (the unit
  // test of the skip path writes to a Vec). Its whole body is pinned, spaces aside.
  const body = rs.match(/#\[test\]\nfn an_mptcp_socket_is_not_tcp_for_either_reading\(\) \{\n([\s\S]*?)\n\}\n/)?.[1];
  assert.equal(
    body?.replace(/\s+/g, "").replace(/,\)/g, ")"),
    "mptcp_case(||Socket::new(Domain::IPV4,Type::STREAM,Some(Protocol::MPTCP)),&mutstd::io::stderr());",
    `the real MPTCP test's body:\n${body}`,
  );
});

test("the root run's provenance hides only its own new logs (review C2R-10; Python's own test)", () => {
  const script = join(HERE, "results", "u3c-kernel-check", "test_siocoutq_root_check.py");
  const r = spawnSync("python3", ["-I", "-B", script], { encoding: "utf8" });
  assert.equal(r.status, 0, `${r.error ?? ""}${r.stdout}${r.stderr}`);
  assert.match(r.stderr, /^Ran [1-9]\d* tests? in .*\n\nOK\n?$/m, r.stderr);
});

/** Every file under `dir` (its .git included), as its relative path and the SHA-256 of its bytes, or a link's target. */
function snapshot(dir) {
  const out = {};
  for (const e of readdirSync(dir, { recursive: true, withFileTypes: true })) {
    const p = join(e.parentPath, e.name);
    if (e.isSymbolicLink()) out[relative(dir, p)] = `-> ${readlinkSync(p)}`;
    else if (e.isFile()) out[relative(dir, p)] = createHash("sha256").update(readFileSync(p)).digest("hex");
  }
  return out;
}

test("the provenance test's git stays in its scratch repositories when GIT_DIR names another (review C2b F2)", (t) => {
  // A git hook or `git rebase --exec` exports GIT_DIR and GIT_INDEX_FILE: with them inherited, the Python test's
  // fixture commit landed on that repository's branch (the reviewer's reproduction dropped its keep.txt).
  const victim = mkdtempSync(join(tmpdir(), "npt-victim-"));
  const hooks = mkdtempSync(join(tmpdir(), "npt-no-hooks-"));
  t.after(() => {
    rmSync(victim, { recursive: true, force: true });
    rmSync(hooks, { recursive: true, force: true });
  });
  const env = Object.fromEntries(Object.entries(process.env).filter(([k]) => !k.startsWith("GIT_")));
  const git = (...args) => {
    const config = [`core.hooksPath=${hooks}`, "commit.gpgsign=false", "user.name=test", "user.email=test@example.invalid"];
    const r = spawnSync("git", [...config.flatMap((c) => ["-c", c]), ...args], { cwd: victim, env, encoding: "utf8" });
    assert.equal(r.status, 0, `git ${args.join(" ")}: ${r.stderr}`);
  };
  writeFileSync(join(victim, "keep.txt"), "kept\n");
  git("init", "-q");
  git("add", "-A");
  git("commit", "-q", "-m", "victim");
  const before = snapshot(victim);
  assert.ok(Object.keys(before).includes("keep.txt") && Object.keys(before).some((f) => f.startsWith(".git")));
  const script = join(HERE, "results", "u3c-kernel-check", "test_siocoutq_root_check.py");
  const exported = { ...process.env, GIT_DIR: join(victim, ".git"), GIT_INDEX_FILE: join(victim, ".git", "index") };
  const r = spawnSync("python3", ["-I", "-B", script], { encoding: "utf8", env: exported });
  assert.equal(r.status, 0, `${r.error ?? ""}${r.stdout}${r.stderr}`);
  assert.match(r.stderr, /^Ran [1-9]\d* tests? in .*\n\nOK\n?$/m, r.stderr);
  assert.deepEqual(snapshot(victim), before, "the repository GIT_DIR names is unchanged, byte for byte");
  // Its git runs no hook and signs no commit, whatever the user's git config says: a ~/.gitconfig with a hooks folder
  // (a post-commit hook that leaves a mark) and commit.gpgsign = true (a signing that cannot work here).
  const home = mkdtempSync(join(tmpdir(), "npt-home-"));
  t.after(() => rmSync(home, { recursive: true, force: true }));
  mkdirSync(join(home, "hooks"));
  const mark = join(home, "hook-ran");
  writeFileSync(join(home, "hooks", "post-commit"), `#!/bin/sh\necho ran >> '${mark}'\n`, { mode: 0o755 });
  writeFileSync(join(home, ".gitconfig"), `[core]\n\thooksPath = ${join(home, "hooks")}\n[commit]\n\tgpgsign = true\n[gpg]\n\tprogram = false\n`);
  const configured = spawnSync("python3", ["-I", "-B", script], { encoding: "utf8", env: { ...env, HOME: home, XDG_CONFIG_HOME: join(home, "xdg") } });
  assert.equal(configured.status, 0, `${configured.error ?? ""}${configured.stdout}${configured.stderr}`);
  assert.deepEqual(readdirSync(home).sort(), [".gitconfig", "hooks"], "no hook ran");
});

test("npt-tcpinfo's non-Linux stub is linted for wasm32 with the leaf crates (review F-N)", () => {
  assert.ok(WASM_CRATES.includes("npt-tcpinfo"));
  for (const leaf of ["npt-link", "npt-l1", "npt-sim", "npt-proto", "npt-room", "npt-client", "npt-log"]) assert.ok(WASM_CRATES.includes(leaf), leaf);
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

/**
 * The tick grep's clean tree: npt-sim's constants each its own function at TICK_HZ, and outside it uses, asserts,
 * paths, tick counts derived from TICK_HZ or ticks_for_ms, CMD_CATCHUP and npt-net's own HANG_TICKS (U4s, §4.3).
 */
const TICK_CLEAN = {
  "crates/sim/src/ticks.rs":
    "pub const TICK_HZ: u32 = 30;\n" +
    "pub const LOOP_CATCHUP: u32 = loop_catchup_at(TICK_HZ);\n" +
    "pub const FULL_STATE_ACK_TICKS: u32 = full_state_ack_ticks_at(TICK_HZ);\n" +
    "pub const ACK_RING_TICKS: u32 = ack_ring_ticks_at( TICK_HZ );\n" +
    "pub const SETTINGS_STEP_TICKS: u32 =\n    settings_step_ticks_at(TICK_HZ);\n" +
    "// const LOOP_CATCHUP: u32 = 7; in a comment sets nothing\n" +
    'const NOTE: &str = "const ACK_RING_TICKS: u32 = 32;";\n' +
    "const MAX_TICK_HZ: u32 = 1_000;\nconst XYZ: [u32; 3] = [1, 2, 3];\n" +
    "const _: () = assert!(LOOP_CATCHUP == loop_catchup_at(TICK_HZ), \"x\");\n" +
    "pub const fn loop_catchup_at(tick_hz: u32) -> u32 {\n    tick_hz / 4\n}\n",
  "crates/room/src/lib.rs":
    "use npt_sim::LOOP_CATCHUP;\nconst TICK_HZ: u32 = npt_sim::TICK_HZ;\n" +
    "impl Ring {\n    const ACK_RING_TICKS: u32 = npt_sim :: ACK_RING_TICKS;\n}\n" +
    "impl InputBuffer {\n    pub const CMD_CATCHUP: usize = npt_proto::Limits::REDUNDANCY_CAP;\n}\n" +
    "const EVENT_ACK_TICKS: u32 = npt_sim::ticks_for_ms(2_000);\nconst WINDOW_TICKS: u32 = 2 * npt_sim::TICK_HZ;\n" +
    "const ALIAS_TICKS: u32 = npt_sim::ACK_RING_TICKS;\nconst MOST_CATCHUP: usize = InputBuffer::CMD_CATCHUP;\n",
  "crates/sim/tests/seam.rs": "fn s1() {\n    assert_eq!(LOOP_CATCHUP, 7);\n    assert_eq!(loop_catchup_at(60), 15);\n}\n",
  "crates/net/src/l2/delay_tests.rs": "fn c() {\n    const HANG_TICKS: u64 = 900;\n}\n",
};
const tickRun = (change = () => {}) => {
  const files = { ...TICK_CLEAN };
  change(files);
  return tickConstantProblems(Object.keys(files).sort(), (f) => files[f]);
};

test("tick constants: TICK_HZ is set once, in npt-sim, and a derived name elsewhere is only a path to it, in any form (U4s, §4.3)", () => {
  const run = tickRun;
  assert.deepEqual(run(), [], "the clean tree, uses and asserts and paths to npt-sim's constants included");
  const room = "crates/room/src/lib.rs";
  const at = `${room}:${TICK_CLEAN[room].split("\n").length}`;
  for (const [plant, want] of [
    ["const LOOP_CATCHUP: u32 = 7;\n", notAPath(at, "LOOP_CATCHUP", "7")],
    ["pub static ACK_RING_TICKS: u32 = 32;\n", notAPath(at, "ACK_RING_TICKS", "32")],
    ["static mut LOOP_CATCHUP: u32 = 30 / 4;\n", notAPath(at, "LOOP_CATCHUP", "30 / 4")],
    ["const SETTINGS_STEP_TICKS: u32 = SEVEN;\n", notAPath(at, "SETTINGS_STEP_TICKS", "SEVEN")],
    ["const LOOP_CATCHUP: u32 = npt_sim::loop_catchup_at(30);\n", notAPath(at, "LOOP_CATCHUP", "npt_sim::loop_catchup_at(30)")],
    // Derived again outside npt-sim, even from TICK_HZ itself (the arch review's F6): one home.
    ["const LOOP_CATCHUP: u32 = npt_sim::TICK_HZ / 4;\n", notAPath(at, "LOOP_CATCHUP", "npt_sim::TICK_HZ / 4")],
    ["const FULL_STATE_ACK_TICKS: u32 = npt_sim::ticks_for_ms(2_000);\n", notAPath(at, "FULL_STATE_ACK_TICKS", "npt_sim::ticks_for_ms(2_000)")],
    ["impl X {\n    pub const FULL_STATE_ACK_TICKS: u32 = 60;\n}\n", notAPath(at.replace(/\d+$/, (k) => +k + 1), "FULL_STATE_ACK_TICKS", "60")],
    ["const ACK_RING_TICKS: u32 =\n    32;\n", notAPath(at, "ACK_RING_TICKS", "32")],
    [
      "const TICK_HZ: u32 = 60;\n",
      `${at} sets TICK_HZ from \`60\` outside crates/sim/src/: npt_sim::TICK_HZ is the one tick constant (plan §4.6)`,
    ],
  ]) {
    assert.deepEqual(run((f) => (f[room] += plant)), [want], plant);
  }
  // In npt-net too: the name rule skips it, these names never.
  assert.deepEqual(
    run((f) => (f["crates/net/src/l2/delay_tests.rs"] += "const LOOP_CATCHUP: u32 = 7;\n")),
    [notAPath("crates/net/src/l2/delay_tests.rs:4", "LOOP_CATCHUP", "7")],
  );
  // A second TICK_HZ inside npt-sim (files are read in sorted order, so grid.rs's is the first), and one in npt-client.
  assert.deepEqual(
    run((f) => (f["crates/sim/src/grid.rs"] = "const TICK_HZ: u32 = 60;\n")),
    [
      "crates/sim/src/ticks.rs:1 sets TICK_HZ from `30` a second time (the first: crates/sim/src/grid.rs:1): npt_sim::TICK_HZ is the one tick constant (plan §4.6)",
    ],
  );
  assert.deepEqual(
    run((f) => (f["crates/client/src/lib.rs"] = "pub const TICK_HZ: u32 = 60;\n")),
    ["crates/client/src/lib.rs:1 sets TICK_HZ from `60` outside crates/sim/src/: npt_sim::TICK_HZ is the one tick constant (plan §4.6)"],
  );
  // A derived constant at a literal rate in npt-sim's own tests is refused too: they are outside its sources.
  assert.deepEqual(
    run((f) => (f["crates/sim/tests/seam.rs"] += "const LOOP_CATCHUP: u32 = loop_catchup_at(60);\n")),
    [notAPath("crates/sim/tests/seam.rs:5", "LOOP_CATCHUP", "loop_catchup_at(60)")],
  );
});

test("tick constants: in npt-sim each derived name is its own function at TICK_HZ; no other tick-count name is a literal (U4s fix pass, §4.3, plan §4.6)", () => {
  const ticks = "crates/sim/src/ticks.rs";
  const swap = (from, to) => (f) => {
    assert.ok(f[ticks].includes(from), from);
    f[ticks] = f[ticks].replace(from, to);
  };
  const loop = "loop_catchup_at(TICK_HZ)";
  // The rust review's R4 and its kin: each gives 7 at 30 Hz, and only its own function gives 15 at 60.
  for (const init of ["7 * TICK_HZ / 30", "TICK_HZ / 4", "7", "ticks_for_ms(233)", "loop_catchup_at(30)", "crate::LOOP_CATCHUP"]) {
    assert.deepEqual(tickRun(swap(loop, init)), [notOwnFn(`${ticks}:2`, "LOOP_CATCHUP", init)], init);
  }
  // FULL_STATE_ACK_TICKS's own function, not the ms form it computes (s1 pins full_state_ack_ticks_at at both rates).
  assert.deepEqual(
    tickRun(swap("full_state_ack_ticks_at(TICK_HZ)", "ticks_for_ms(2_000)")),
    [notOwnFn(`${ticks}:3`, "FULL_STATE_ACK_TICKS", "ticks_for_ms(2_000)")],
  );
  // A tick-count name in npt-sim must join TICK_DERIVED, whatever it is set from.
  assert.deepEqual(tickRun((f) => (f["crates/sim/src/grid.rs"] = "pub const RESEND_TICKS: u32 = ticks_for_ms(500);\n")), [
    "crates/sim/src/grid.rs:1 sets RESEND_TICKS, a tick count by its name, in npt-sim outside TICK_DERIVED: it joins " +
      "TICK_DERIVED as `resend_ticks_at(TICK_HZ)`, its function pinned in s1 (u4-build-plan.md §4.3)",
  ]);
  // Outside npt-sim, every other name of a tick count (`*_TICKS`, `*CATCHUP*`) is never a literal, in any form.
  const room = "crates/room/src/lib.rs";
  const at = `${room}:${TICK_CLEAN[room].split("\n").length}`;
  for (const [plant, name, init] of [
    ["const LOOP_CATCHUP_ALIAS: u32 = 7;\n", "LOOP_CATCHUP_ALIAS", "7"],
    ["const FAST_CATCHUP: u32 = 30 / 4;\n", "FAST_CATCHUP", "30 / 4"],
    ["const STALL_TICKS: u32 = SEVEN;\n", "STALL_TICKS", "SEVEN"],
    ["pub static RESEND_TICKS: u64 = 60;\n", "RESEND_TICKS", "60"],
    ["static mut HOLD_TICKS: u32 = npt_sim::ticks_for_ms_at(60, 500);\n", "HOLD_TICKS", "npt_sim::ticks_for_ms_at(60, 500)"],
    ["const RING_TICKS: [u32; 2] = [32, 64];\n", "RING_TICKS", "[32, 64]"],
    ["impl Q {\n    const EVENT_CAP_TICKS: u32 =\n        90;\n}\n", "EVENT_CAP_TICKS", "90"],
  ]) {
    const line = plant.startsWith("impl") ? at.replace(/\d+$/, (k) => +k + 1) : at;
    assert.deepEqual(tickRun((f) => (f[room] += plant)), [fromLiteral(line, name, init)], plant);
  }
  // CMD_CATCHUP is the redundancy cap, not a tick count (plan §4.6); npt-net's L2 harness keeps its own 30 Hz.
  assert.deepEqual(tickRun((f) => (f[room] += "const CMD_CATCHUP: usize = 8;\n")), []);
  assert.deepEqual(tickRun((f) => (f["crates/net/src/l2/delay_tests.rs"] += "const SEND_TICKS: u64 = 30;\n")), []);
  assert.deepEqual(Object.keys(NOT_TICK_COUNTS), ["CMD_CATCHUP"]);
  assert.deepEqual(TICK_NAMED_SKIP, ["crates/net/"]);
});

test("npt-room is float-free: f32, f64 and float literals in crates/room/src fail; prose, strings, ranges and integers pass (U4s, §4.11)", () => {
  const clean = {
    "crates/room/src/lib.rs":
      "//! No f32 or f64 here: 1.5 in a comment is prose.\n" +
      'const S: &str = "f32 1.5 2e3";\n' +
      "fn f(t: (u8, u8), v: &[u32]) -> u32 {\n    let r = 0..5;\n    let s = 0..=5;\n    let x = t.0 + t.1;\n" +
      "    let m = 1.max(2) + 2_u32.pow(3);\n    let h = 0x1e3 + 0xff_f32 + 0b1;\n    let buf32 = 7u32 + 1_000i64 as u32;\n" +
      "    v[r.start] + s.start() + u32::from(x) + m + h + buf32\n}\n",
    "crates/sim/src/lib.rs": "pub fn f() -> f32 {\n    1.5\n}\n",
    "crates/room/tests/t.rs": "const X: f64 = 1.0;\n",
    "crates/room/Cargo.toml": '[package]\nname = "npt-room"\ndescription = "f32 1.5"\n',
  };
  const run = (change = () => {}) => {
    const files = { ...clean };
    change(files);
    return roomFloatProblems(Object.keys(files).sort(), (f) => files[f]);
  };
  assert.deepEqual(run(), [], "the clean tree: other crates, room's tests and its manifest are not read");
  const file = "crates/room/src/interest.rs";
  for (const [plant, token] of [
    ["fn g() -> f32 {\n    0\n}\n", "f32"],
    ["use core::f64::consts::PI;\n", "f64"],
    ["const A: u64 = 1.5 as u64;\n", "1.5"],
    ["const B: u32 = 1. as u32;\n", "1."],
    ["const C: u32 = 1e3 as u32;\n", "1e3"],
    ["const D: u32 = 2.5e-3 as u32;\n", "2.5e-3"],
    ["const E: u32 = 2f64 as u32;\n", "2f64"],
    ["const F: u32 = 1_000.0_f32 as u32;\n", "1_000.0_f32"],
  ]) {
    assert.deepEqual(run((f) => (f[file] = plant)), [roomFloat(`${file}:1`, token)], plant);
  }
  assert.deepEqual(run((f) => (f[file] = "\n\nfn g(x: f32) -> f64 {\n    f64::from(x) * 0.5\n}\n")), [roomFloat(`${file}:3`, "f32", 4)]);
});

/**
 * The bite-id grep's clean tree, as the spike plants its ids (u4-build-plan.md §4.14): -17's in npt-client, -41's in
 * npt-sim's own-copy snap, npt-l1's -3 ids, npt-lab's probe tests running one of them inside `#[cfg(test)] mod tests`,
 * tests naming every id, and prose and longer strings that name them.
 */
const BITE_CLEAN = {
  "crates/client/src/lib.rs":
    "/// PM-0001-17/client-rounding's site is the client's call into the grid.\n" +
    '#[cfg(feature = "bites")]\npub const BITES: [&str; 1] = ["PM-0001-17/client-rounding"];\n',
  "crates/client/tests/bites.rs": 'const OTHERS: [&str; 2] = ["PM-0001-41/server-quant-differs", "PM-0001-3/clock-seed"];\n',
  "crates/l1/src/lib.rs": 'const BITE_IDS: [(&str, u8); 2] = [\n    ("PM-0001-3/rtt-each-way", 0),\n    ("PM-0001-3/clock-seed", 1),\n];\n',
  "crates/lab/src/probe.rs":
    "pub fn run() {}\n\n#[cfg(test)]\n#[allow(clippy::unwrap_used)]\nmod tests {\n    #[test]\n    fn a_bite_runs() {\n" +
    '        let s = Some("PM-0001-3/rtt-each-way");\n        let r = r#"PM-0001-3/clock-seed"#;\n    }\n}\n',
  "crates/room/src/lib.rs":
    'const NOTE: &str = "see PM-0001-17/client-rounding";\nconst QUOTED: &str = "\\"PM-0001-41/server-quant-differs\\"";\n',
  "crates/sim/src/grid.rs": "//! The grid's one defect; -17 and -41 plant it, and their ids live with their sites.\npub struct Config;\n",
  "crates/sim/src/lib.rs": '#[cfg(feature = "bites")]\npub const OWN_COPY_SNAP_BITES: [&str; 1] = ["PM-0001-41/server-quant-differs"];\n',
  "crates/sim/tests/seam.rs": 'const OTHERS: [&str; 2] = ["PM-0001-17/client-rounding", "PM-0001-3/rtt-each-way"];\n',
};

test("a bite id lives in one crate's sources: one id in two crates fails, naming both file:line (the rust fix-review's G1, §4.14)", () => {
  const run = (change = () => {}) => {
    const files = { ...BITE_CLEAN };
    change(files);
    return biteHomeProblems(Object.keys(files).sort(), (f) => files[f]);
  };
  assert.ok(BITE_ID.test("PM-0001-17/client-rounding") && !BITE_ID.test("PM-0001-17") && !BITE_ID.test("see PM-0001-3/x"));
  // G1a: every bite id marks.json names is one BITE_ID matches (upper case and dots included).
  const marks = JSON.parse(readFileSync(join(HERE, "marks", "marks.json"), "utf8"));
  const ids = marks.marks.flatMap((m) => (m.bites ?? []).map((b) => b.id));
  assert.ok(ids.length > 100, `marks.json's bite ids read: ${ids.length}`);
  assert.deepEqual(
    ids.filter((id) => !BITE_ID.test(id)),
    [],
    "every marks.json bite id is a BITE_ID",
  );
  assert.deepEqual(run(), [], "the clean tree: tests, a #[cfg(test)] module, prose and longer strings name ids freely");
  // G1a: an upper-case id (a busy-wait bite planned for room and sim code) in two crates fails too.
  assert.deepEqual(
    run((f) => {
      f["crates/room/src/lib.rs"] += 'const B: &str = "PM-0001-7/B-S";\n';
      f["crates/sim/src/grid.rs"] += 'const B: &str = "PM-0001-7/B-S";\n';
    }),
    [biteTwice("PM-0001-7/B-S", "crates/room/src/lib.rs:3", "crates/sim/src/grid.rs:3")],
  );
  // The arch review's F1 come back: -17's id in npt-sim's grid beside npt-client's.
  const grid = "crates/sim/src/grid.rs";
  assert.deepEqual(run((f) => (f[grid] += '#[cfg(feature = "bites")]\npub const BITES: [&str; 1] = ["PM-0001-17/client-rounding"];\n')), [
    biteTwice("PM-0001-17/client-rounding", "crates/client/src/lib.rs:3", `${grid}:4`),
  ]);
  // A second site inside the one crate is that crate's affair.
  assert.deepEqual(run((f) => (f["crates/client/src/endpoint.rs"] = 'const ID: &str = "PM-0001-17/client-rounding";\n')), []);
  // Outside its test module npt-lab names npt-l1's id as a source does; a raw string counts; three crates are named.
  assert.deepEqual(run((f) => (f["crates/lab/src/main.rs"] = 'const DEFAULT: &str = "PM-0001-3/rtt-each-way";\n')), [
    biteTwice("PM-0001-3/rtt-each-way", "crates/l1/src/lib.rs:2", "crates/lab/src/main.rs:1"),
  ]);
  assert.deepEqual(
    run((f) => {
      f["crates/client/src/lib.rs"] += 'const SNAP: &str = r#"PM-0001-41/server-quant-differs"#;\n';
      f["crates/room/src/lib.rs"] += 'const S: &str = "PM-0001-41/server-quant-differs";\n';
    }),
    [biteTwice("PM-0001-41/server-quant-differs", "crates/client/src/lib.rs:4", "crates/room/src/lib.rs:3", "crates/sim/src/lib.rs:2")],
  );
  // Only an inline `#[cfg(test)] mod` is skipped: a function under it is read, which fails closed.
  assert.deepEqual(run((f) => (f["crates/lab/src/probe.rs"] += '#[cfg(test)]\nfn id() -> &\'static str {\n    "PM-0001-3/clock-seed"\n}\n')), [
    biteTwice("PM-0001-3/clock-seed", "crates/l1/src/lib.rs:3", "crates/lab/src/probe.rs:14"),
  ]);
});

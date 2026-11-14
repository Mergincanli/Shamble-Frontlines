// Tests for check.mjs's rule checks: each dependency rule of plan §4.2 is broken once on a synthetic graph and must
// be caught; the clean graph must pass. Run: node --test spikes/netcode-proof/check.test.mjs

import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { mkdirSync, mkdtempSync, readFileSync, readdirSync, readlinkSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { homedir, tmpdir } from "node:os";
import { dirname, join, relative } from "node:path";
import { test } from "node:test";
import { fileURLToPath } from "node:url";
import {
  BITE_ID,
  CHANNEL_ONLY,
  CLIPPY_TWINS,
  FAILED_STEP_DIR,
  H2_VECTOR,
  HASH_MAP_ARM,
  HASH_MAP_FIELD,
  HASH_WASM_UNSAFE_FILE,
  LEAF,
  LOADER,
  READ_BACK,
  RECORDINGS_MANIFEST,
  SIM_BUILDS,
  STD_SIN_SCAN,
  STEPS,
  UNSAFE_EXCEPTIONS,
  WASM_BITES,
  WASM_BUILDS,
  WASM_CRATES,
  WASM_STEP_DIR,
  biteHomeProblems,
  buildFlagProblems,
  buildInputProblems,
  builtArtifact,
  cargoConfigDirs,
  cargoConfigProblems,
  cargoCounts,
  clippyTwinProblems,
  crateFiles,
  dependencyProblems,
  depsStep,
  docBothBuilds,
  exceptionLintProblems,
  expectedMarks,
  failedRun,
  failingTests,
  layoutProblems,
  lexRust,
  linkUseProblems,
  lintBothBuilds,
  main,
  manifestKeyProblems,
  marksProblems,
  NOT_TICK_COUNTS,
  readBackCheck,
  readComparison,
  readScan,
  referenceProblems,
  roomFloatProblems,
  run,
  rustflagSources,
  SIM_DEPENDENCIES,
  simFeatureCheck,
  simSubtree,
  TICK_NAMED_SKIP,
  testBothBuilds,
  tickConstantProblems,
  treeArgs,
  unsafeProblems,
  wasmStep,
  workspaceLintProblems,
} from "./check.mjs";
import {
  BEGIN,
  HASH_WASM_EXPORTS,
  LINKER_EXPORTS,
  LOADER_EXPORTS,
  STEP,
  exportProblems,
  instantiate,
  moduleProblems,
  replay,
} from "./crates/hash-wasm/loader.mjs";

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
    "npt-hash-wasm": n("npt-sim"),
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

/** npt-hash-wasm's seven exports' signatures, as crates/hash-wasm/src/lib.rs writes them. */
const EXPORT_SIGS = {
  begin: "(bite: u32, arm_tick: u32) -> u32",
  feed: "(word: u32)",
  step: "() -> u32",
  hash_lo: "() -> u32",
  hash_hi: "() -> u32",
  dump_len: "() -> u32",
  dump_word: "(i: u32) -> u32",
};
/** One npt-hash-wasm export as its lib.rs writes it: the placed allow (a multi-line reason), its no_mangle, its body. */
const exported = (name, sig = EXPORT_SIGS[name], between = "") =>
  `#[allow(\n    unsafe_code,\n    reason = "the third exception (ADR 0002:264-265): this listed name; \\\n              integers only"\n)]\n` +
  `${between}#[unsafe(no_mangle)]\npub extern "C" fn ${name}${sig} {\n    run()\n}\n`;
/** begin's first line in the fixtures (its body follows): where U4d2 round 1's FX-3 plants an `unsafe` block. */
const BEGIN_HEAD = `pub extern "C" fn begin${EXPORT_SIGS.begin} {\n`;
/** npt-hash-wasm's clean lib.rs: a doc naming what it may not do, the seven exports, a helper. */
const HASH_WASM_LIB =
  "//! The seven exports; no `unsafe {}` block and no other #[allow(unsafe_code)] (a doc comment, not code).\n" +
  `${HASH_WASM_EXPORTS.map((n) => exported(n)).join("")}fn run() -> u32 {\n    0\n}\n`;
/** npt-hash-wasm's manifest as the fixtures give it: its own tables, unsafe_code denied. */
const HASH_WASM_MANIFEST = '[package]\nname = "npt-hash-wasm"\n\n[lints.rust]\nunsafe_code = "deny"\n';

/** A clean crate tree for unsafeProblems: the three exceptions and two ordinary crates. */
function cleanCrates() {
  return {
    "crates/hash-wasm/Cargo.toml": HASH_WASM_MANIFEST,
    [HASH_WASM_UNSAFE_FILE]: HASH_WASM_LIB,
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

test("unsafe_code: an attribute relaxing it outside the three exceptions fails, in every form", () => {
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
    assert.deepEqual(p, ["crates/net/src/planted.rs relaxes unsafe_code (only npt-client-wasm, npt-tcpinfo and npt-hash-wasm may)"], code);
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
  // The three exceptions' manifests set their own level (the workspace forbids it): exceptionLintProblems reads those.
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
    // The macro forms' attribute holds a metavariable too (carry 85 (c) as advisor-u4r-r1 widened it: FP-6 item 8).
    const held = code.startsWith("#") ? [] : [metaHeld(`crates/room/src/k.rs:${line}`)];
    assert.deepEqual(p, [...held, `crates/room/src/k.rs:${line} has a #[path] whose value is not a plain string literal, not a .rs file inside crates/room/`], code);
  }
});

/** One unsafe block, as rustc 1.99.1 compiles it once unsafe_code is relaxed. */
const UNSAFE_FN = "pub fn f() -> u8 {\n    let x = 1u8;\n    unsafe { *(&raw const x) }\n}\n";

/** Carry 85 (c)'s refusal of an attribute holding a metavariable (advisor-u4r-r1, FP-6 item 8: SEC-U4R-5), at `where`. */
function metaHeld(where) {
  return `${where} has an attribute holding a metavariable (\`$\` between \`#[\` or \`#![\` and its \`]\`): a macro could build a #[path] or an allow from a fragment that this scan does not read`;
}

test("unsafe code outside the three exception crates fails however the lint was relaxed (review S1)", () => {
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
    // A macro-built allow's attribute holds a metavariable: refused as well (FP-6 item 8).
    const held = what.includes("macro-built allow") ? [metaHeld("crates/room/src/evil.rs:3")] : [];
    assert.deepEqual(
      p,
      [`crates/room/src/evil.rs:${firstLine[what]} has unsafe code (${n} \`unsafe\` or \`global_asm\`; only npt-client-wasm, npt-tcpinfo and npt-hash-wasm may)`, ...held],
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
  assert.deepEqual(p, ["crates/room/src/emoji.rs:4 has unsafe code (1 `unsafe` or `global_asm`; only npt-client-wasm, npt-tcpinfo and npt-hash-wasm may)"]);
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
    // Its attribute holds a metavariable (FP-6 item 8).
    metaHeld(`crates/tcpinfo/src/linux.rs:${at + 2}`),
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
    "crates/hash-wasm/Cargo.toml": readFileSync(join(HERE, "crates/hash-wasm/Cargo.toml"), "utf8"),
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

test("the three exceptions hold the workspace's other lints in their own tables, and their own level (review C2R-1; U4d2)", () => {
  const ws = readFileSync(join(HERE, "Cargo.toml"), "utf8");
  const T = "crates/tcpinfo/Cargo.toml";
  const W = "crates/client-wasm/Cargo.toml";
  const H = "crates/hash-wasm/Cargo.toml";
  const real = { [T]: readFileSync(join(HERE, T), "utf8"), [W]: readFileSync(join(HERE, W), "utf8"), [H]: readFileSync(join(HERE, H), "utf8") };
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
  // A lint the workspace adds must be added to all three.
  const more = ws.replace('iter_over_hash_type = "deny"\n', 'iter_over_hash_type = "deny"\nundocumented_unsafe_blocks = "deny"\n');
  assert.deepEqual(exceptionLintProblems(more, read({})), [
    `${W} has no [lints.clippy] undocumented_unsafe_blocks: the workspace's is "deny"`,
    `${T} has no [lints.clippy] undocumented_unsafe_blocks: the workspace's is "deny"`,
    `${H} has no [lints.clippy] undocumented_unsafe_blocks: the workspace's is "deny"`,
  ]);
  assert.deepEqual(exceptionLintProblems(ws, () => {
    throw new Error("absent");
  }), [`${W} cannot be read`, `${T} cannot be read`, `${H} cannot be read`]);
  // npt-hash-wasm (the third, U4d2): deny, as npt-tcpinfo; its manifest at "allow" or "forbid" is refused by name, and
  // so is a clippy key the workspace's tables do not hold, or one it drops.
  const hw = (from, to) => exceptionLintProblems(ws, read({ [H]: real[H].replace(from, to) }));
  for (const v of ['"allow"', '"forbid"', '"warn"']) assert.deepEqual(hw('unsafe_code = "deny"', `unsafe_code = ${v}`), [`${H} sets [lints.rust] unsafe_code = ${v}, not "deny"`], v);
  assert.deepEqual(hw('iter_over_hash_type = "deny"\n', ""), [`${H} has no [lints.clippy] iter_over_hash_type: the workspace's is "deny"`]);
  assert.deepEqual(hw("[lints.clippy]\n", '[lints.clippy]\nundocumented_unsafe_blocks = "allow"\n'), [
    `${H} sets [lints.clippy] undocumented_unsafe_blocks = "allow", which the workspace does not`,
  ]);
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
  // A target table passes; its `linker` key no longer does (carry 85 (a): the test below).
  assert.deepEqual(config('[build]\nrustc-wrapper = "sccache"\ntarget-dir = "t"\n[target.x86_64-unknown-linux-gnu]\nrustflags = []\n'), []);
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

test("carry 85 (a), (b) and (d): a linker for any target and a profile outside the manifests are refused, naming the source and value; CARGO_HOME is read as cargo reads it; this container's environment and the spike's own config pass", () => {
  // (a) A linker: an environment variable whose name ends in _LINKER, or a `linker` key in any table or form.
  assert.deepEqual(cargoConfigProblems({ PATH: "/bin", CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER: "/opt/ld" }, []), [
    'CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER in the environment sets a linker ("/opt/ld"): its arguments can rewrite the target_features section the Wasm loader reads',
  ]);
  const config = (text) => cargoConfigProblems({}, [{ path: "c.toml", text }]);
  const linker = (v) => `c.toml has a key \`linker\` (${v}): it sets a linker, whose arguments can rewrite the target_features section the Wasm loader reads`;
  assert.deepEqual(config('[target.wasm32-unknown-unknown]\nlinker = "rust-lld"\n'), [linker('"rust-lld"')]);
  assert.deepEqual(config('target.x86_64-unknown-linux-gnu.linker = "cc"\n'), [linker('"cc"')]);
  assert.deepEqual(config('[host]\n"linker" = "cc"\n'), [linker('"cc"')]);
  // (b) A profile: CARGO_PROFILE_* in the environment; a `profile` table, quoted table, dotted key or key in a config.
  assert.deepEqual(cargoConfigProblems({ CARGO_PROFILE_RELEASE_OVERFLOW_CHECKS: "false" }, []), [
    "CARGO_PROFILE_RELEASE_OVERFLOW_CHECKS in the environment sets a profile key (\"false\"): it can drop npt-sim's -C overflow-checks=on, which plan §4.5 asks for in every profile",
  ]);
  // Each refusal names its line and text (advisor-u4r-r1, FP-6 item 9: SEC-U4R-6).
  const profile = (line, said) =>
    `c.toml:${line} has a profile key or table (${JSON.stringify(said)}): it can drop npt-sim's -C overflow-checks=on, which plan §4.5 asks for in every profile`;
  for (const text of [
    "[profile.release.package.npt-sim]\noverflow-checks = false\n",
    "profile.release.overflow-checks = false\n",
    '["profile".dev]\nopt-level = 1\n',
    "profile = { release = { debug = true } }\n",
  ]) {
    assert.deepEqual(config(text), [profile(1, text.split("\n")[0])], text);
  }
  assert.deepEqual(config('[build]\njobs = 2\n\n[profile.dev]\nopt-level = 1\n'), [profile(4, "[profile.dev]")], "its line");
  // Neither name in a string value or inside another key's name.
  assert.deepEqual(config('[build]\ntarget-dir = "profile/linker"\nprofiles = 1\nlinkers = 2\n'), []);
  // This container's environment and the spike's own config, as the deps step reads them, pass.
  assert.deepEqual(cargoConfigProblems(process.env, [{ path: ".cargo/config.toml", text: readFileSync(join(HERE, ".cargo", "config.toml"), "utf8") }]), []);
  // (d) CARGO_HOME as cargo's home crate reads it: an empty one as unset, a relative one against the folder cargo runs
  // in, read through check.mjs's own function (cargo never runs with these values: u4r-brief.md section 4 rule 7).
  const start = join(HERE, "crates", "lab");
  const home = (env) => cargoConfigDirs(start, env).at(-1);
  assert.equal(home({}), join(homedir(), ".cargo"));
  assert.equal(home({ CARGO_HOME: "" }), join(homedir(), ".cargo"), "an empty CARGO_HOME is unset");
  assert.equal(home({ CARGO_HOME: "../h" }), join(HERE, "crates", "h"), "a relative CARGO_HOME, against the folder cargo runs in");
  assert.equal(home({ CARGO_HOME: "/opt/cargo" }), "/opt/cargo");
  assert.ok(cargoConfigDirs(start, {}).includes(join(HERE, ".cargo")), "and every folder above it");
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
/** The clippy.toml comparison's message for the first differing lines (U4a; u4-build-plan.md §2, Drift). */
const clippyDrift = (copyAt, homeAt) =>
  `${copyAt} differs from ${homeAt} below their headers: crates/proto/clippy.toml is crates/sim/clippy.toml's list, ` +
  "line for line (u4-build-plan.md §2, plan §4.5)";
/** The npt-link grep's message for `shown` at `at` (U4a; u4s-carries.md carry 3). */
const linkUse = (at, shown) =>
  `${at} names \`${shown}\` from npt_link: npt-proto and npt-client use npt-link for Channel only (catalogue item 2; ` +
  "u4-build-plan.md §4.1)";
/** buildInputProblems' messages (the advisor's U4d2 round-1 ruling, FX-1): a source holding flags; a build target set. */
const holds = (where, flags) =>
  `${where} holds ${JSON.stringify(flags)}: no rustflags source may hold a flag (cargo uses one source and drops the ` +
  "rest, so the read-back could not be given what rustc gets: the advisor's U4d2 round-1 ruling, FX-1)";
const targetEnv = (value) =>
  `CARGO_BUILD_TARGET in the environment sets the build target (${JSON.stringify(value)}): the native builds take ` +
  "cargo's default target, the x86-64 one the read-back and the features comparison read";
const targetKey = (path) =>
  `${path} has a key \`target\` (build.target): it sets the build target, and the native builds take cargo's default ` +
  "target, the x86-64 one the read-back and the features comparison read";

/** rustc 1.99.1's read-back of the target features per target in this container (`rustc --print cfg`; notes F1). */
const CFG_BASE = {
  "x86_64-unknown-linux-gnu": ["fxsr", "sse", "sse2"],
  "wasm32-unknown-unknown": ["bulk-memory", "multivalue", "mutable-globals", "nontrapping-fptoint", "reference-types", "sign-ext"],
};
/**
 * rustc 1.99.1's read-back given `flags` for `target`, as the advisor's U4d2 brief check ran it (A15; notes F21), as a
 * runner result: a target feature of this target's is added or removed; one of the other target's only warns; a CPU
 * name of this target's sets its list (haswell and native add avx, avx2 and fma on x86-64; bleeding-edge adds simd128
 * and relaxed-simd on wasm32); a CPU name of the other target's empties wasm32's list and cuts x86-64's to sse, sse2.
 */
function cfgModel(target, flags) {
  const x86 = target.startsWith("x86_64");
  const own = x86 ? ["fma", "sse2", "sse3", "avx"] : ["simd128", "relaxed-simd", "sign-ext"];
  let features = [...CFG_BASE[target]];
  const options = [];
  for (let i = 0; i < flags.length; i++) {
    if (flags[i] === "-C" || flags[i] === "--codegen") options.push(flags[++i] ?? "");
    else if (flags[i].startsWith("--codegen=")) options.push(flags[i].slice(10));
    else if (flags[i].startsWith("-C")) options.push(flags[i].slice(2));
  }
  for (const o of options) {
    const [name, value = ""] = [o.split("=")[0].replace(/_/g, "-"), o.split("=").slice(1).join("=")];
    if (name === "target-cpu") {
      if (["haswell", "native"].includes(value)) features = x86 ? [...CFG_BASE[target], "avx", "avx2", "fma"] : [];
      else if (value === "x86-64") features = x86 ? [...CFG_BASE[target]] : [];
      else if (["bleeding-edge", "mvp"].includes(value)) features = x86 ? ["sse", "sse2"] : value === "mvp" ? ["mutable-globals"] : [...CFG_BASE[target], "relaxed-simd", "simd128"];
    } else if (name === "target-feature") {
      for (const item of value.split(",")) {
        const f = item.replace(/^[+-]/, "");
        if (!own.includes(f)) continue;
        features = item.startsWith("-") ? features.filter((x) => x !== f) : [...new Set([...features, f])];
      }
    }
  }
  const stdout = `debug_assertions\npanic="${x86 ? "unwind" : "abort"}"\n${features.sort().map((f) => `target_feature="${f}"`).join("\n")}\n`;
  return { ok: true, status: 0, stdout, out: stdout };
}
/**
 * One build's `cargo tree` output in treeArgs's form: the package, a dependency outside npt-sim's subtree (getrandom),
 * npt-sim's subtree as the head's lock gives it (notes F4), then another package at npt-sim's depth; `extra` gives a
 * package other features.
 */
const treeText = (pkg, bites, extra = {}) =>
  `${[
    `0${pkg} v0.0.0 (/w/crates/${pkg.slice(4)}) [${bites ? "bites" : ""}]`,
    `1getrandom v0.2.17 [${extra.getrandom ?? ""}]`,
    `1npt-sim v0.0.0 (/w/crates/sim) [${extra["npt-sim"] ?? (bites ? "bites" : "")}]`,
    `2libm v0.2.16 [${extra.libm ?? "arch,default"}]`,
    "2rand_chacha v0.10.0 []",
    "3ppv-lite86 v0.2.21 [simd]",
    `4zerocopy v0.8.63 [${extra.zerocopy ?? "simd"}]`,
    "3rand_core v0.10.1 []",
    "2rand_core v0.10.1 []",
    "1serde v1.0.228 [std]",
  ].join("\n")}\n`;
/**
 * A runner for depsStep's `cargo tree` and `rustc` calls: each tree from `trees["<pkg>[ bites]"]` (text), else
 * treeText's; each read-back from cfgModel; `calls` records every call.
 */
function depsRunner({ trees = {}, cfg = cfgModel } = {}) {
  const calls = [];
  const ok = (stdout) => ({ ok: true, status: 0, stdout, out: stdout });
  const runner = (cmd, args, cwd) => {
    calls.push({ line: `${cmd} ${args.join(" ")}`, cwd });
    if (cmd === "rustc" && args[0] === "--version") return ok("rustc 1.99.1 (58cf8f9c3 2026-10-13)\n");
    if (cmd === "rustc" && args[0] === "--print") return cfg(args[3], args.slice(4));
    if (cmd === "cargo" && args[0] === "tree") {
      const key = `${args[args.indexOf("-p") + 1]}${args.includes("bites") ? " bites" : ""}`;
      return ok(trees[key] ?? treeText(key.split(" ")[0], args.includes("bites")));
    }
    throw new Error(`depsRunner has no answer for ${cmd} ${args.join(" ")}`);
  };
  return { runner, calls };
}

/**
 * A spike folder for depsStep (the workspace manifest, the three exceptions, npt-room and a cargo config) and cargo
 * metadata's shape for it, from clean()'s graph: every member at crates/<name without npt->/ with one lib target there,
 * npt-sim's package naming its three dependencies; and a runner for its `cargo tree` and `rustc` calls (depsRunner).
 * `plant` changes the files, `change` the graph.
 */
function depsFixture(t, plant = () => {}, change = () => {}, answers = {}) {
  const dir = mkdtempSync(join(tmpdir(), "npt-deps-"));
  t.after(() => rmSync(dir, { recursive: true, force: true }));
  const files = {
    "Cargo.toml": '[workspace]\nmembers = ["crates/*"]\n\n[workspace.lints.rust]\nunsafe_code = "forbid"\n\n[workspace.lints.clippy]\ndisallowed_types = "deny"\n',
    "crates/client-wasm/Cargo.toml": '[package]\nname = "npt-client-wasm"\n\n[lints.rust]\nunsafe_code = "allow"\n\n[lints.clippy]\ndisallowed_types = "deny"\n',
    "crates/tcpinfo/Cargo.toml": '[package]\nname = "npt-tcpinfo"\n\n[lints.rust]\nunsafe_code = "deny"\n\n[lints.clippy]\ndisallowed_types = "deny"\n',
    "crates/tcpinfo/src/lib.rs": `${DENY}mod linux;\n`,
    "crates/tcpinfo/src/linux.rs": `${placed("srtt_us")}${placed("unacked_and_unsent")}`,
    "crates/hash-wasm/Cargo.toml": `${HASH_WASM_MANIFEST}\n[lints.clippy]\ndisallowed_types = "deny"\n`,
    [HASH_WASM_UNSAFE_FILE]: HASH_WASM_LIB,
    "crates/room/Cargo.toml": '[package]\nname = "npt-room"\n\n[lints]\nworkspace = true\n',
    "crates/room/src/lib.rs": "pub fn f() {}\n",
    ".cargo/config.toml": '[build]\ntarget-dir = "t"\n',
    // U4a's clippy.toml pair (u4-build-plan.md §2, Drift): headers of different lengths, one list below them.
    "crates/sim/clippy.toml": "# npt-sim's bans.\n# Its header.\n\ndisallowed-types = []\n",
    "crates/proto/clippy.toml": "# npt-proto shares them.\n\ndisallowed-types = []\n",
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
  const sim = m.packages.find((p) => p.name === "npt-sim");
  if (sim) sim.dependencies = [...SIM_DEPENDENCIES.map((d) => ({ name: d, kind: null })), { name: "proptest", kind: "dev" }];
  return { dir, meta: m, env: { CARGO_HOME: join(dir, "no-cargo-home") }, ...depsRunner(answers) };
}

test("the deps step is depsStep, and it calls every check: one plant per check, each reported (review C2R-5)", (t) => {
  assert.equal(STEPS.deps, depsStep, "check.mjs's deps step runs this function");
  const ok = depsFixture(t);
  const clean = depsStep(ok.dir, ok.env, () => ({ ok: true, meta: ok.meta }), ok.runner);
  assert.equal(clean.ok, true, clean.out);
  assert.match(clean.out, /^held: .* \(8 crate files and 1 cargo config file\(s\) read; rustc 1\.99\.1 .*'s read-back, given no rustflags: .*; npt-sim's features, one set in npt-server, npt-lab, npt-hash-wasm: .*\)$/);
  // The read-back and the features comparison run through the runner it is given, from the spike's folder.
  assert.deepEqual(
    ok.calls.map((c) => c.line),
    [
      "rustc --version",
      ...READ_BACK.map((b) => `rustc --print cfg --target ${b.target}`),
      ...[false, true].flatMap((bites) => SIM_BUILDS.map((b) => `cargo ${treeArgs(b, bites).join(" ")}`)),
    ],
  );
  assert.ok(ok.calls.every((c) => c.cwd === ok.dir));
  const bad = depsFixture(
    t,
    (f) => {
      f["Cargo.toml"] = `cargo-features = ["profile-rustflags"]\n${f["Cargo.toml"].replace('"forbid"', '"deny"')}`;
      // A member's manifest too (review C2c N1: dropping depsStep's member-manifest loop had passed every test).
      f["crates/room/Cargo.toml"] += '\n[profile.dev]\nrustflags = ["--cap-lints", "warn"]\n';
      f["crates/tcpinfo/Cargo.toml"] = f["crates/tcpinfo/Cargo.toml"].replace('disallowed_types = "deny"\n', "");
      // U4d2's build-flag check: a refused flag in a config's rustflags (the name scan; the read-back lists fma).
      f[".cargo/config.toml"] = `include = ["extra.toml"]\n${f[".cargo/config.toml"]}rustflags = ["-C", "target-feature=+fma"]\n`;
      f["crates/room/src/evil.rs"] = UNSAFE_FN;
      // U4s's three greps (u4-build-plan.md §4.3, §4.11, §4.14).
      f["crates/room/src/ticks.rs"] = "pub const LOOP_CATCHUP: u32 = 7;\npub fn g() -> f64 {\n    0.0\n}\n";
      f["crates/sim/src/grid.rs"] = 'pub const BITES: [&str; 1] = ["PM-0001-17/client-rounding"];\n';
      f["crates/client/src/lib.rs"] = 'pub const BITES: [&str; 1] = ["PM-0001-17/client-rounding"];\n';
      // U4a's two (u4-build-plan.md §2's Drift; u4s-carries.md carry 3).
      f["crates/proto/clippy.toml"] += "disallowed-methods = []\n";
      f["crates/proto/src/lib.rs"] = "use npt_link::Link;\n";
      // U4d2's npt-hash-wasm rules: an unsafe block in its lib.rs, in a new fn and (U4d2 round 1, FX-3) inside a listed
      // export's body, where that export's own allow lets rustc compile it.
      f[HASH_WASM_UNSAFE_FILE] = `${HASH_WASM_LIB.replace(BEGIN_HEAD, `${BEGIN_HEAD}    unsafe {}\n`)}fn stray() {\n    unsafe {}\n}\n`;
    },
    (e) => {
      e["npt-sim"] = n("tokio");
      // U4d2: npt-hash-wasm ships npt-sim only.
      e["npt-hash-wasm"] = n("npt-sim", "serde");
    },
    // U4d2's features comparison: the server build's libm with one feature more.
    { trees: { "npt-server": treeText("npt-server", false, { libm: "arch,default,std" }) } },
  );
  bad.meta.packages.find((p) => p.name === "npt-room").targets[0].src_path = join(bad.dir, "crates", "tcpinfo", "src", "linux.rs");
  bad.meta.packages.find((p) => p.name === "npt-proto").dependencies = [{ name: "npt-link", rename: "link" }];
  let linked = true;
  try {
    symlinkSync("lib.rs", join(bad.dir, "crates", "room", "src", "k.rs"));
  } catch {
    linked = false;
  }
  const r = depsStep(bad.dir, bad.env, () => ({ ok: true, meta: bad.meta }), bad.runner);
  assert.equal(r.ok, false);
  const lines = r.out.split("\n");
  // The new fn's block follows the fixture's lib.rs, one line longer for the block in begin's body.
  const strayAt = `${HASH_WASM_UNSAFE_FILE}:${HASH_WASM_LIB.split("\n").length + 2}`;
  const bodyAt = `${HASH_WASM_UNSAFE_FILE}:${HASH_WASM_LIB.split("\n").indexOf(BEGIN_HEAD.trimEnd()) + 2}`;
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
    ["unsafeProblems", "crates/room/src/evil.rs:3 has unsafe code (1 `unsafe` or `global_asm`; only npt-client-wasm, npt-tcpinfo and npt-hash-wasm may)"],
    ["tickConstantProblems", notAPath("crates/room/src/ticks.rs:1", "LOOP_CATCHUP", "7")],
    ["roomFloatProblems", roomFloat("crates/room/src/ticks.rs:2", "f64", 2)],
    ["biteHomeProblems", biteTwice("PM-0001-17/client-rounding", "crates/client/src/lib.rs:1", "crates/sim/src/grid.rs:1")],
    ["clippyTwinProblems", clippyDrift("crates/proto/clippy.toml:4", "crates/sim/clippy.toml:4 (its end)")],
    ["linkUseProblems", linkUse("crates/proto/src/lib.rs:1", "npt_link::Link")],
    ["dependencyProblems (a renamed npt-link)", "npt-proto renames its npt-link dependency to link"],
    // U4d2's checks (u4d2-brief.md 2.4 (b), (c), (d)).
    [
      "buildFlagProblems",
      `${join(bad.dir, ".cargo", "config.toml")}'s rustflags passes "-C target-feature=+fma": it enables fma, which plan.md §4.5's build flags refuse`,
    ],
    ["buildInputProblems (U4d2 round 1, FX-1)", holds(`${join(bad.dir, ".cargo", "config.toml")}'s rustflags`, "-C target-feature=+fma")],
    [
      "readBackCheck",
      'rustc\'s read-back for x86_64-unknown-linux-gnu (given "-C target-feature=+fma") lists target_feature="fma", which plan.md §4.5\'s build flags refuse',
    ],
    [
      "simFeatureCheck",
      "npt-sim's libm v0.2.16 resolves [arch, default, std] in the npt-server (x86_64-unknown-linux-gnu, measured) build and [arch, default] in the npt-lab (x86_64-unknown-linux-gnu, measured) build",
    ],
    [
      "unsafeProblems (npt-hash-wasm)",
      `${strayAt} has \`unsafe\` outside an attribute; npt-hash-wasm's only unsafe is the #[unsafe(no_mangle)] of its listed exports (${HASH_WASM_EXPORTS.join(", ")}) in ${HASH_WASM_UNSAFE_FILE}`,
    ],
    [
      "unsafeProblems (npt-hash-wasm, inside an export's body: U4d2 round 1, FX-3)",
      `${bodyAt} has \`unsafe\` outside an attribute; npt-hash-wasm's only unsafe is the #[unsafe(no_mangle)] of its listed exports (${HASH_WASM_EXPORTS.join(", ")}) in ${HASH_WASM_UNSAFE_FILE}`,
    ],
    ["dependencyProblems (npt-hash-wasm)", "npt-hash-wasm depends on serde"],
  ]) {
    assert.ok(lines.includes(line), `${check}'s plant is not reported:\n${r.out}`);
  }
  if (!linked) t.diagnostic("symbolic links cannot be made here: the link refusal is not exercised");
  else assert.ok(lines.includes("crates/room/src/k.rs is a symbolic link: the scan does not follow it, and rustc would"), r.out);
  // The environment it is given is the one checked (process.env when check.mjs runs it).
  const runner = depsStep(ok.dir, { ...ok.env, CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUNNER: "true" }, () => ({ ok: true, meta: ok.meta }), ok.runner);
  assert.equal(runner.out, "CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUNNER in the environment runs the test binaries in cargo's place");
  // And so is its CARGO_HOME (review C2b F3): a config there that must be refused is read from the CARGO_HOME given, not
  // from this process's ~/.cargo.
  const home = join(ok.dir, "cargo-home");
  mkdirSync(home);
  writeFileSync(join(home, "config.toml"), '[build]\nrustc = "/opt/other/rustc"\n');
  const homed = depsStep(ok.dir, { ...ok.env, CARGO_HOME: home }, () => ({ ok: true, meta: ok.meta }), ok.runner);
  assert.equal(homed.out, `${join(home, "config.toml")} has a key \`rustc\`: it replaces rustc or wraps the workspace's rustc calls`);
  // The environment's rustflags reach the name scan and the read-back (U4d2): RUSTFLAGS="-C target-cpu=native".
  const native = depsStep(ok.dir, { ...ok.env, RUSTFLAGS: "-C target-cpu=native" }, () => ({ ok: true, meta: ok.meta }), ok.runner);
  assert.ok(native.out.split("\n").includes('RUSTFLAGS in the environment passes "-C target-cpu=native": plan.md §4.5\'s build flags refuse target-cpu=native'), native.out);
  // cargo metadata failing fails the step with its output.
  assert.deepEqual(depsStep(ok.dir, ok.env, () => ({ ok: false, out: "error: no Cargo.lock" }), ok.runner), { ok: false, out: "error: no Cargo.lock" });
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

test("npt-proto's clippy.toml is npt-sim's below its header: one changed line fails, naming both (U4a, u4-build-plan.md §2's Drift)", () => {
  assert.deepEqual(CLIPPY_TWINS, [["crates/sim/clippy.toml", "crates/proto/clippy.toml"]]);
  // The spike's own two files hold.
  const real = (f) => readFileSync(join(HERE, f), "utf8");
  assert.deepEqual(clippyTwinProblems(real), []);
  const sim = real("crates/sim/clippy.toml");
  const proto = real("crates/proto/clippy.toml");
  const twins = (p, s = sim) => clippyTwinProblems((f) => (f.endsWith("proto/clippy.toml") ? p : s));
  // a9's self-test bite: one line of proto's copy changed (a ban dropped from the list).
  const simHead = sim.split("\n").findIndex((l) => l.startsWith("disallowed-types"));
  const protoHead = proto.split("\n").findIndex((l) => l.startsWith("disallowed-types"));
  const dropped = proto.replace('    "f32::sin", "f64::sin",\n', '    "f32::sin",\n');
  const at = proto.split("\n").findIndex((l) => l === '    "f32::sin", "f64::sin",') + 1;
  assert.notEqual(dropped, proto);
  assert.deepEqual(twins(dropped), [
    clippyDrift(`crates/proto/clippy.toml:${at}`, `crates/sim/clippy.toml:${at - protoHead + simHead}`),
  ]);
  // Below the header a comment counts; a longer or shorter header does not; a line added at the end is found there.
  assert.equal(twins(proto.replace("disallowed-methods", "# note\ndisallowed-methods")).length, 1);
  assert.deepEqual(twins(`# one more header line\n${proto}`), []);
  assert.equal(twins(`${proto}extra = 1\n`)[0]?.includes("(its end)"), true);
  // A missing file is a problem, not a pass.
  const missing = clippyTwinProblems((f) => {
    if (f.endsWith("proto/clippy.toml")) throw new Error("ENOENT");
    return sim;
  });
  assert.equal(missing.length, 1);
  assert.match(missing[0], /^crates\/proto\/clippy\.toml is missing/);
});

test("npt-proto's and npt-client's sources name only Channel from npt_link, by file and line (U4a, u4s-carries.md carry 3)", () => {
  assert.deepEqual(CHANNEL_ONLY, ["crates/proto/src/", "crates/client/src/"]);
  const files = { "crates/proto/src/msg.rs": "", "crates/client/src/lib.rs": "", "crates/room/src/lib.rs": "use npt_link::Link;\n" };
  const run = (text, f = "crates/proto/src/msg.rs") => linkUseProblems(Object.keys({ ...files, [f]: 1 }), (g) => (g === f ? text : files[g]));
  // The real tree holds.
  const { files: real } = crateFiles(join(HERE, "crates"));
  assert.deepEqual(linkUseProblems(real, (f) => readFileSync(join(HERE, f), "utf8")), []);
  // Channel, whichever way it is named; prose and strings may name anything; another crate's sources are not read.
  for (const ok of [
    "use npt_link::Channel;\n",
    "use npt_link :: Channel as Ch;\n",
    "use npt_link::{Channel};\nuse npt_link::{ Channel as C, };\n",
    "fn f(c: npt_link::Channel) -> bool { c == npt_link::Channel::Datagram }\n",
    "// npt_link::Link is not ours\nconst S: &str = \"npt_link::Loopback\";\n",
  ]) {
    assert.deepEqual(run(ok), [], ok);
  }
  assert.deepEqual(run("use npt_link::Link;\n", "crates/room/src/lib.rs"), []);
  // Anything else fails, named by file and line, in either crate.
  for (const [bad, line, shown] of [
    ["use npt_link::Link;\n", 1, "npt_link::Link"],
    ["\nuse npt_link::{Channel, Link};\n", 2, "npt_link::{Channel, Link}"],
    ["use npt_link::*;\n", 1, "npt_link::*"],
    ["use npt_link;\n", 1, "npt_link"],
    ["extern crate npt_link as l;\n", 1, "npt_link as l"],
    ["fn f() { let _ = npt_link::Loopback::pair(); }\n", 1, "npt_link::Loopback::pair()"],
    ["use npt_link::ChannelSet;\n", 1, "npt_link::ChannelSet"],
  ]) {
    assert.deepEqual(run(bad), [linkUse(`crates/proto/src/msg.rs:${line}`, shown)], bad);
    assert.deepEqual(run(bad, "crates/client/src/lib.rs"), [linkUse(`crates/client/src/lib.rs:${line}`, shown)], bad);
  }
  // A renamed dependency would hide the name from the grep: refused in npt-proto's and npt-client's manifests only.
  const renamed = (crate) => {
    const m = meta(clean());
    m.packages.find((p) => p.name === crate).dependencies = [{ name: "npt-link", rename: "link" }];
    return dependencyProblems(m);
  };
  assert.deepEqual(renamed("npt-proto"), ["npt-proto renames its npt-link dependency to link"]);
  assert.deepEqual(renamed("npt-client"), ["npt-client renames its npt-link dependency to link"]);
  assert.deepEqual(renamed("npt-room"), []);
});

/** A runner that records each cargo call and answers from `results` (then success). */
function recorder(results = []) {
  const calls = [];
  const runner = (cmd, args, cwd, env) => {
    calls.push({ cmd, args, cwd, env });
    return results.shift() ?? { ok: true, out: `ran ${args[0]}\n`, stdout: "", status: 0 };
  };
  return { calls, runner };
}

test("each cargo step runs what it says: fmt, clippy and its wasm32 twin, doc and test, both builds, -D warnings (U4a, the rust fix2 review's nits)", () => {
  const steps = (name) => {
    const r = recorder();
    const out = STEPS[name](r.runner);
    return { out, calls: r.calls.map((c) => `${c.cmd} ${c.args.join(" ")}`), env: r.calls.map((c) => c.env) };
  };
  assert.deepEqual(steps("fmt").calls, ["cargo fmt --all --check"]);
  assert.deepEqual(steps("clippy").calls, [
    "cargo clippy --locked --workspace --all-targets -- -D warnings",
    "cargo clippy --locked --workspace --all-targets --all-features -- -D warnings",
  ]);
  const wasm = `--target wasm32-unknown-unknown ${WASM_CRATES.map((c) => `-p ${c}`).join(" ")}`;
  assert.deepEqual(steps("clippy-wasm32").calls, [
    `cargo clippy --locked ${wasm} -- -D warnings`,
    `cargo clippy --locked ${wasm} --all-features -- -D warnings`,
  ]);
  assert.deepEqual(steps("test").calls, ["cargo test --locked --workspace", "cargo test --locked --workspace --all-features"]);
  // doc: both builds on the host, then both for wasm32 (npt-tcpinfo's non-Linux stub documented too), rustdoc's
  // warnings as errors through RUSTDOCFLAGS, whatever the environment held, and no CARGO_ENCODED_RUSTDOCFLAGS.
  assert.equal(STEPS.doc, docBothBuilds);
  assert.equal(STEPS.test, testBothBuilds);
  const r = recorder();
  const doc = docBothBuilds(r.runner, { PATH: "/bin", RUSTDOCFLAGS: "--cap-lints=allow", CARGO_ENCODED_RUSTDOCFLAGS: "--cap-lints=allow" });
  assert.equal(doc.ok, true);
  assert.deepEqual(
    r.calls.map((c) => `${c.cmd} ${c.args.join(" ")}`),
    [
      "cargo doc --locked --no-deps --workspace",
      "cargo doc --locked --no-deps --workspace --all-features",
      `cargo doc --locked --no-deps ${wasm}`,
      `cargo doc --locked --no-deps ${wasm} --all-features`,
    ],
  );
  assert.ok(WASM_CRATES.includes("npt-tcpinfo"));
  for (const c of r.calls) {
    assert.equal(c.env.RUSTDOCFLAGS, "-D warnings");
    assert.equal(Object.hasOwn(c.env, "CARGO_ENCODED_RUSTDOCFLAGS"), false);
    assert.equal(c.env.PATH, "/bin");
  }
  assert.match(lastLine(doc.out), /^measured build and bites build both documented, host and wasm32, rustdoc warnings as errors$/);
  assert.match(lastLine(steps("clippy").out.out), /^measured build and bites build both linted$/);
});

const lastLine = (text) => text.trimEnd().split("\n").at(-1);
const cargoTail = "error: this URL is not a hyperlink\n --> crates/room/src/lib.rs:7:27\nerror: could not document `npt-room`\nwarning: build failed, waiting for other jobs to finish...\n";
const failed = (out = cargoTail) => ({ ok: false, out, stdout: "", status: 101 });

test("a failed build's summary names the build and its first error, not cargo's last line (U4a, the rust fix2 review's nits)", () => {
  // The measured build's rustdoc failing: its first error, and no further run.
  let r = recorder([failed()]);
  let s = docBothBuilds(r.runner, {});
  assert.equal(s.ok, false);
  assert.equal(r.calls.length, 1);
  assert.equal(lastLine(s.out), "the measured build's rustdoc failed: error: this URL is not a hyperlink");
  // Only the bites build's clippy failing: never "both linted".
  r = recorder([{ ok: true, out: "measured ok\n" }, failed("error[E0425]: cannot find value `x` in this scope\nerror: could not compile\n")]);
  s = lintBothBuilds(["--workspace"], r.runner);
  assert.equal(s.ok, false);
  assert.equal(lastLine(s.out), "the bites build's clippy (the measured build's passed) failed: error[E0425]: cannot find value `x` in this scope");
  assert.ok(!s.out.includes("both linted"), s.out);
  assert.ok(s.out.startsWith("measured ok\n"), "the measured build's output is kept");
  // The measured build's clippy, then each of doc's later runs, and the tests.
  r = recorder([failed()]);
  assert.equal(lastLine(lintBothBuilds([], r.runner).out), "the measured build's clippy failed: error: this URL is not a hyperlink");
  for (const [at, what] of [
    [1, "the bites build's rustdoc"],
    [2, "the measured build's wasm32 rustdoc"],
    [3, "the bites build's wasm32 rustdoc"],
  ]) {
    r = recorder([...Array(at).fill({ ok: true, out: "" }), failed()]);
    assert.equal(lastLine(docBothBuilds(r.runner, {}).out), `${what} failed: error: this URL is not a hyperlink`);
    assert.equal(r.calls.length, at + 1);
  }
  const testOut = "test result: FAILED. 3 passed; 1 failed; 0 ignored\nerror: test failed, to rerun pass `-p npt-proto --test wire`\n";
  r = recorder([failed(testOut)]);
  assert.equal(
    lastLine(testBothBuilds(r.runner).out),
    "the measured build's tests (3 tests passed, 1 failed, 0 reported a skip) failed: error: test failed, to rerun pass `-p npt-proto --test wire`",
  );
  // Output with no error line says so.
  assert.equal(lastLine(failedRun(failed("warning: build failed\n"), "x").out), "x failed: its output has no error line (the lines above)");
});

/** The last `n` lines of a text: what main prints of a failed step (its last 40). */
const tail = (text, n = 40) => text.trimEnd().split("\n").slice(-n).join("\n");

/** hashWasmProblems' messages (U4d2). */
const HW_LIST = `its listed exports (${HASH_WASM_EXPORTS.join(", ")}) in ${HASH_WASM_UNSAFE_FILE}`;
const hwMisplaced = (where, on) =>
  `${where} relaxes unsafe_code on ${on}; npt-hash-wasm may only put #[allow(unsafe_code, reason = "...")] directly on ${HW_LIST}`;
const hwCount = (fn, k) => `npt-hash-wasm's export ${fn} has ${k} #[allow(unsafe_code, reason = ...)] attributes, not 1`;
const hwUnsafe = (where, what, how) => `${where} has \`${what}\` ${how}; npt-hash-wasm's only unsafe is the #[unsafe(no_mangle)] of ${HW_LIST}`;
const hwSig = (where, fn, what) =>
  `${where} exports ${fn} with ${what}: npt-hash-wasm's exports take and give u32 or i32 only (the advisor's U4D2-Q7 ruling)`;

test("npt-hash-wasm, the third exception: one allow on each listed export, its no_mangle the only unsafe, u32 or i32 only (U4d2, 2.4 (d) and (e))", () => {
  const F = HASH_WASM_UNSAFE_FILE;
  const at = HASH_WASM_LIB.split("\n").length;
  const lineOf = (text, start) => text.split("\n").findIndex((l) => l.startsWith(start)) + 1;
  const hw = (change) => unsafeWith(change);
  assert.deepEqual(hw(() => {}), [], "the clean fixture");
  // The real crate holds (its seven exports as built).
  const real = crateFiles(join(HERE, "crates")).files;
  assert.ok(real.includes(F) && real.includes("crates/hash-wasm/Cargo.toml"));
  assert.deepEqual(unsafeProblems(real, (f) => readFileSync(join(HERE, f), "utf8")), [], "the spike's own crates");
  // plan :706's bite: a stray `unsafe {}` block, refused by file:line (rustc refuses it too, under the crate's deny).
  assert.deepEqual(hw((f) => (f[F] += "fn stray() {\n    unsafe {}\n}\n")), [hwUnsafe(`${F}:${at + 1}`, "unsafe", "outside an attribute")]);
  // An #[unsafe(no_mangle)] export not on the list, with its allow: the allow is misplaced, its unsafe refused.
  const extra = 'pub extern "C" fn extra() -> u32 {';
  assert.deepEqual(hw((f) => (f[F] += exported("extra", "() -> u32"))), [
    hwMisplaced(`${F}:${at}`, `\`${extra}\``),
    hwUnsafe(`${F}:${at + 5}`, "unsafe", `in #[unsafe(no_mangle)] on \`${extra}\``),
  ]);
  // A listed export with no allow, or two (a second stacked on step).
  const bare = `#[unsafe(no_mangle)]\npub extern "C" fn begin${EXPORT_SIGS.begin} {\n    run()\n}\n`;
  assert.deepEqual(hw((f) => (f[F] = HASH_WASM_LIB.replace(exported("begin"), bare))), [hwCount("begin", 0)]);
  assert.deepEqual(hw((f) => (f[F] = HASH_WASM_LIB.replace(exported("step"), exported("step", EXPORT_SIGS.step, '#[allow(unsafe_code, reason = "twice")]\n')))), [
    hwCount("step", 2),
  ]);
  // An allow on a helper; an inner allow; cfg_attr, expect or warn on an export; unsafe_code named outside an attribute.
  assert.deepEqual(hw((f) => (f[F] += '#[allow(unsafe_code, reason = "a helper")]\nfn helper() {}\n')), [hwMisplaced(`${F}:${at}`, "`fn helper() {}`")]);
  assert.deepEqual(hw((f) => (f[F] = `#![allow(unsafe_code)]\n${HASH_WASM_LIB}`)), [hwMisplaced(`${F}:1`, "its whole module or crate (#!)")]);
  const beginLine = '`pub extern "C" fn begin(bite: u32, arm_tick: u32) -> u32 {`';
  for (const attr of ["#[cfg_attr(all(), allow(unsafe_code))]\n", '#[expect(unsafe_code, reason = "x")]\n', '#[warn(unsafe_code, reason = "x")]\n', "#[allow(unsafe_code)]\n"]) {
    const p = hw((f) => (f[F] = HASH_WASM_LIB.replace(exported("begin"), `${attr}${bare}`)));
    assert.deepEqual(p, [hwMisplaced(`${F}:2`, beginLine), hwCount("begin", 0)], attr);
  }
  assert.deepEqual(hw((f) => (f[F] += "lax!(unsafe_code);\n")), [`${F}:${at} names unsafe_code outside an attribute (a macro can build the attribute from it)`]);
  // Any other unsafe: an unsafe fn, an unsafe extern block, export_name or link_section in place of no_mangle, global_asm!.
  assert.deepEqual(hw((f) => (f[F] += "pub unsafe fn f() {}\n")), [hwUnsafe(`${F}:${at}`, "unsafe", "outside an attribute")]);
  assert.deepEqual(hw((f) => (f[F] += 'unsafe extern "C" {\n    fn now() -> u32;\n}\n')), [hwUnsafe(`${F}:${at}`, "unsafe", "outside an attribute")]);
  for (const other of ['unsafe(export_name = "begin2")', 'unsafe(link_section = ".x")']) {
    const p = hw((f) => (f[F] = HASH_WASM_LIB.replace("#[unsafe(no_mangle)]\npub extern \"C\" fn begin", `#[${other}]\npub extern "C" fn begin`)));
    assert.deepEqual(p, [hwUnsafe(`${F}:7`, "unsafe", `in #[${other}] on ${beginLine}`)], other);
  }
  assert.deepEqual(hw((f) => (f[F] += 'core::arch::global_asm!("");\n')), [hwUnsafe(`${F}:${at}`, "global_asm", "outside an attribute")]);
  // The written invariant's second clause (the advisor's U4D2-Q7 ruling): u32 or i32 in and out.
  const feedAt = `${F}:${lineOf(HASH_WASM_LIB, 'pub extern "C" fn feed')}`;
  assert.deepEqual(hw((f) => (f[F] = HASH_WASM_LIB.replace(exported("feed"), exported("feed", "(word: f64)")))), [hwSig(feedAt, "feed", "parameter word: f64")]);
  const loAt = `${F}:${lineOf(HASH_WASM_LIB, 'pub extern "C" fn hash_lo')}`;
  assert.deepEqual(hw((f) => (f[F] = HASH_WASM_LIB.replace(exported("hash_lo"), exported("hash_lo", "() -> u64")))), [hwSig(loAt, "hash_lo", "result u64")]);
  assert.deepEqual(hw((f) => (f[F] = HASH_WASM_LIB.replace(exported("step"), exported("step", "() -> i32")))), [], "i32 is lowered as u32 is");
  // Outside its own file, even a placed allow on a listed name is misplaced.
  assert.deepEqual(hw((f) => (f["crates/hash-wasm/src/other.rs"] = exported("hash_hi"))), [
    hwMisplaced("crates/hash-wasm/src/other.rs:1", '`pub extern "C" fn hash_hi() -> u32 {`'),
    hwUnsafe("crates/hash-wasm/src/other.rs:6", "unsafe", 'in #[unsafe(no_mangle)] on `pub extern "C" fn hash_hi() -> u32 {`'),
  ]);
  // Its dependencies: npt-sim only; nothing depends on it, in any kind; it is a leaf crate and the third exception.
  assert.deepEqual(problemsWith((e) => (e["npt-hash-wasm"] = n("npt-sim", "serde"))), ["npt-hash-wasm depends on serde"]);
  assert.deepEqual(problemsWith((e) => (e["npt-hash-wasm"] = [])), ["npt-hash-wasm does not depend on npt-sim"]);
  assert.deepEqual(problemsWith((e) => e["npt-lab"].push(["npt-hash-wasm", "dev"])), ["npt-lab depends on npt-hash-wasm"]);
  assert.deepEqual(problemsWith((e) => (e["npt-hash-wasm"] = [...n("npt-sim"), ["proptest", "dev"]])), [], "a dev dependency does not ship");
  assert.ok(LEAF.includes("npt-hash-wasm") && WASM_CRATES.includes("npt-hash-wasm"));
  assert.deepEqual(UNSAFE_EXCEPTIONS, { "crates/client-wasm/": "npt-client-wasm", "crates/tcpinfo/": "npt-tcpinfo", "crates/hash-wasm/": "npt-hash-wasm" });
});

test("npt-hash-wasm: an `unsafe` block inside a listed export's body is refused at its line, where that export's own allow lets rustc compile it (U4d2 round 1, FX-3: RR-M1, SEC-U4D2-2, TE-M2)", () => {
  const F = HASH_WASM_UNSAFE_FILE;
  // TP4's form (r1-te-tp4-body-unsafe.log: rustc, clippy -D warnings and the unit tests accept it; the scan alone
  // refuses it), and a bare block, as the first line of each listed export's body.
  for (const name of HASH_WASM_EXPORTS) {
    const head = `pub extern "C" fn ${name}${EXPORT_SIGS[name]} {\n`;
    for (const block of ["    unsafe {}\n", "    let w = unsafe { core::ptr::read_volatile(&0u32) };\n"]) {
      const planted = HASH_WASM_LIB.replace(head, `${head}${block}`);
      assert.notEqual(planted, HASH_WASM_LIB, name);
      const line = planted.split("\n").indexOf(block.trimEnd()) + 1;
      assert.deepEqual(unsafeWith((f) => (f[F] = planted)), [hwUnsafe(`${F}:${line}`, "unsafe", "outside an attribute")], `${name}: ${block}`);
    }
  }
  // Deeper in a body: inside a nested block, a closure and an unsafe fn item declared within it.
  for (const inner of ["    if true {\n        unsafe {}\n    }\n", "    let f = || unsafe { 0 };\n", "    unsafe fn g() {}\n"]) {
    const planted = HASH_WASM_LIB.replace(BEGIN_HEAD, `${BEGIN_HEAD}${inner}`);
    const line = planted.split("\n").findIndex((l, i) => i > 0 && l.includes("unsafe") && !l.includes("no_mangle") && !l.includes("unsafe_code")) + 1;
    assert.deepEqual(unsafeWith((f) => (f[F] = planted)), [hwUnsafe(`${F}:${line}`, "unsafe", "outside an attribute")], inner);
  }
});

test("npt-tcpinfo and npt-hash-wasm, the deny-level exceptions, have no build script and name `include` only as a macro call: V4's two forms are refused by name (U4d2 round 1, FX-6: SEC-U4D2-4)", () => {
  const F = HASH_WASM_UNSAFE_FILE;
  const why =
    "code a build script writes, brought in by a renamed include!, would sit inside a listed function's allow, where rustc " +
    "compiles unsafe code the scan never reads (SEC-U4D2-4)";
  // V4's first form (r1-sec-scan.log; sec-probe-scan.mjs): a build.rs writing `\x75nsafe` code into OUT_DIR, which the
  // scan cannot see in its string.
  const buildRs =
    'fn main() {\n    let out = std::env::var("OUT_DIR").unwrap();\n' +
    '    let code = "{ let _ = \\x75nsafe { core::ptr::read_volatile(&0u32) }; }";\n' +
    '    std::fs::write(format!("{out}/gen.rs"), code).unwrap();\n}\n';
  for (const crate of ["hash-wasm", "tcpinfo"]) {
    assert.deepEqual(unsafeWith((f) => (f[`crates/${crate}/build.rs`] = buildRs)), [`crates/${crate}/build.rs is a build script in npt-${crate}, which may have none: ${why}`]);
  }
  // Its second: `use std::include as inc;`, and `inc!(concat!(env!("OUT_DIR"), "/gen.rs"));` in begin's body, which the
  // include! rule cannot see by its name; and a raw identifier's form in npt-tcpinfo.
  const includeAt = (where, name) =>
    `${where} names \`include\` other than directly before \`!\` and an opening bracket, as an include!, include_str! or ` +
    `include_bytes! call is written (a renamed include!, as \`use std::include as inc;\`, or one a macro builds, as ` +
    `\`m!(include!)\`, hides the call from the include! rule), which ${name} may not: ${why}`;
  const renamed = `use std::include as inc;\n${HASH_WASM_LIB.replace(BEGIN_HEAD, `${BEGIN_HEAD}    inc!(concat!(env!("OUT_DIR"), "/gen.rs"));\n`)}`;
  assert.deepEqual(unsafeWith((f) => (f[F] = renamed)), [includeAt(`${F}:1`, "npt-hash-wasm")]);
  assert.deepEqual(unsafeWith((f) => (f["crates/tcpinfo/src/lib.rs"] += "use core::r#include as i;\n")), [includeAt("crates/tcpinfo/src/lib.rs:8", "npt-tcpinfo")]);
  // The macros by their own names are pathProblems' (an in-crate .rs literal passes); `include` in prose, a string or
  // another crate is not this rule's (the workspace's forbid makes rustc refuse unsafe code there).
  const calls = 'include!("k.rs");\nconst V: &str = include_str!("../x.txt");\nconst B: &[u8] = include_bytes!("../y.bin");\n// include\nconst S: &str = "include";\n';
  assert.deepEqual(unsafeWith((f) => (f[F] = `${HASH_WASM_LIB}${calls}`)), []);
  assert.deepEqual(unsafeWith((f) => (f["crates/net/src/lib.rs"] += "use std::include as inc;\n")), []);
  // Carry 85 (c): `include` must be directly before `!` and an opening bracket: `m!(include!)` (the bare name handed to
  // a macro, which can build the call) and `include ! (...)` are refused; `include!{...}` and `include![...]` pass.
  assert.deepEqual(unsafeWith((f) => (f[F] = `${HASH_WASM_LIB}m!(include!);\n`)), [includeAt(`${F}:${HASH_WASM_LIB.split("\n").length}`, "npt-hash-wasm")]);
  assert.deepEqual(unsafeWith((f) => (f["crates/tcpinfo/src/lib.rs"] += 'const K: &str = include ! ("k.rs");\n')), [includeAt("crates/tcpinfo/src/lib.rs:8", "npt-tcpinfo")]);
  assert.deepEqual(unsafeWith((f) => (f[F] = `${HASH_WASM_LIB}include!{"k.rs"}\ninclude!["k.rs"];\n`)), []);
  // Carry 85 (c), every crate, as advisor-u4r-r1 widened it (FP-6 item 8: SEC-U4R-5): an attribute holding a
  // metavariable anywhere in its text, either form; none in the spike.
  const metaAt = (where) => `${where} has an attribute holding a metavariable (\`$\` between \`#[\` or \`#![\` and its \`]\`): a macro could build a #[path] or an allow from a fragment that this scan does not read`;
  const meta = "macro_rules! m {\n    ($p:meta) => {\n        #[$p]\n        mod x;\n    };\n}\nmacro_rules! n {\n    ($p:meta) => {\n        # ! [ $p ]\n    };\n}\n";
  assert.deepEqual(unsafeWith((f) => (f["crates/net/src/lib.rs"] += meta)), [metaAt("crates/net/src/lib.rs:6"), metaAt("crates/net/src/lib.rs:12")]);
  const inner = "macro_rules! k {\n    ($a:meta) => {\n        #[cfg_attr(all(), $a)]\n        mod y;\n    };\n}\n";
  assert.deepEqual(unsafeWith((f) => (f["crates/net/src/lib.rs"] += inner)), [metaAt("crates/net/src/lib.rs:6")]);
  // A `build` key in the manifest, in either form cargo reads; a custom-build target in cargo metadata.
  const manifestKey = (crate) => `crates/${crate}/Cargo.toml has a key \`build\`: a build script in npt-${crate}, which may have none: ${why}`;
  assert.deepEqual(unsafeWith((f) => (f["crates/hash-wasm/Cargo.toml"] = HASH_WASM_MANIFEST.replace("\n\n[lints.rust]", '\nbuild = "gen.rs"\n\n[lints.rust]'))), [manifestKey("hash-wasm")]);
  assert.deepEqual(unsafeWith((f) => (f["crates/tcpinfo/Cargo.toml"] = `package.build = true\n${f["crates/tcpinfo/Cargo.toml"]}`)), [manifestKey("tcpinfo")]);
  for (const crate of ["hash-wasm", "tcpinfo"]) {
    const m = layoutMeta({ [`crates/${crate}`]: [["lib", `npt_${crate}`, "src/lib.rs"], ["custom-build", "build-script-build", "build.rs"]] });
    assert.deepEqual(layoutProblems(m), [`npt-${crate}'s custom-build target build-script-build is a build script in npt-${crate}, which may have none: ${why}`]);
  }
  assert.deepEqual(layoutProblems(layoutMeta({ "crates/bots": [["bin", "npt-bots", "src/main.rs"], ["custom-build", "build-script-build", "build.rs"]] })), [], "another crate's build script");
  // The spike's own crates have neither.
  const real = crateFiles(join(HERE, "crates")).files;
  assert.deepEqual(unsafeProblems(real, (f) => readFileSync(join(HERE, f), "utf8")), []);
});

/** The target_features entries rustc 1.99.1's release builds of npt-hash-wasm carry (the rust seat's wasminfo run). */
const BUILT_FEATURES = ["+bulk-memory", "+bulk-memory-opt", "+call-indirect-overlong", "+multivalue", "+mutable-globals", "+nontrapping-fptoint", "+reference-types", "+sign-ext"];
/** LEB128, as WebAssembly's binary format writes an unsigned integer. */
const leb128 = (n) => {
  const out = [];
  do {
    let b = n & 0x7f;
    n >>>= 7;
    if (n) b |= 0x80;
    out.push(b);
  } while (n);
  return out;
};
const wasmName = (s) => [...leb128(Buffer.byteLength(s)), ...Buffer.from(s)];
/** A target_features section's body: a count, then each entry's prefix byte and name. */
const featureBody = (entries) => [...leb128(entries.length), ...entries.flatMap((f) => [f.charCodeAt(0), ...wasmName(f.slice(1))])];

/**
 * A hand-assembled WebAssembly module, a few hundred bytes: each of `names` exported, a function () -> i32 returning 0,
 * but `memory`, a memory of one page; `kinds` exports a name as another kind (`global`, an i32 global; `function`, for
 * `memory` too, and then the module has no memory); `imports` imports one function per [module, field], first;
 * `features` is its target_features section's entries (two of BUILT_FEATURES by default: wasm-ld writes one), and
 * `sections`, when given, its target_features sections' bodies in place of that one ([] for none).
 */
function wasmModule(names, { imports = [], kinds = {}, features = ["+mutable-globals", "+sign-ext"], sections } = {}) {
  const section = (id, body) => [id, ...leb128(body.length), ...body];
  const kindOf = (x) => kinds[x] ?? (x === "memory" ? "memory" : "function");
  const fns = names.filter((x) => kindOf(x) === "function");
  const globals = names.filter((x) => kindOf(x) === "global");
  const body = [0, 0x41, 0, 0x0b];
  const exported = (x) => {
    const k = kindOf(x);
    if (k === "memory") return [...wasmName(x), 0x02, 0];
    if (k === "global") return [...wasmName(x), 0x03, ...leb128(globals.indexOf(x))];
    return [...wasmName(x), 0x00, ...leb128(imports.length + fns.indexOf(x))];
  };
  return new Uint8Array([
    0x00, 0x61, 0x73, 0x6d, 1, 0, 0, 0,
    ...section(1, [1, 0x60, 0, 1, 0x7f]),
    ...(imports.length ? section(2, [...leb128(imports.length), ...imports.flatMap(([m, f]) => [...wasmName(m), ...wasmName(f), 0x00, 0])]) : []),
    ...section(3, [...leb128(fns.length), ...fns.map(() => 0)]),
    ...(names.some((x) => kindOf(x) === "memory") ? section(5, [1, 0x00, 1]) : []),
    ...(globals.length ? section(6, [...leb128(globals.length), ...globals.flatMap(() => [0x7f, 0x00, 0x41, 0, 0x0b])]) : []),
    ...section(7, [...leb128(names.length), ...names.flatMap(exported)]),
    ...section(10, [...leb128(fns.length), ...fns.flatMap(() => [...leb128(body.length), ...body])]),
    ...(sections ?? [featureBody(features)]).flatMap((b) => section(0, [...wasmName("target_features"), ...b])),
  ]);
}

test("the loader's list is the seven exports and wasm-ld's memory; an export more is refused by name, and a module that imports does not instantiate (U4d2, 2.2, 2.4 (d))", () => {
  assert.deepEqual(HASH_WASM_EXPORTS, ["begin", "feed", "step", "hash_lo", "hash_hi", "dump_len", "dump_word"]);
  assert.deepEqual(LINKER_EXPORTS, { memory: "memory" });
  assert.deepEqual(LOADER_EXPORTS, { ...Object.fromEntries(HASH_WASM_EXPORTS.map((x) => [x, "function"])), memory: "memory" });
  assert.deepEqual([BEGIN, STEP], [{ BEGUN: 0, REFUSED: 1 }, { NEED_MORE: 0, STEPPED: 1, DONE: 2, FAILED: 3 }]);
  const listed = [...HASH_WASM_EXPORTS, "memory"];
  assert.ok(wasmModule([...listed, "extra"]).length < 256, "a few hundred bytes at most, built here");
  assert.deepEqual(exportProblems(wasmModule(listed)), [], "the clean module");
  assert.deepEqual(exportProblems(wasmModule([...listed, "extra"])), ['the module exports "extra" (a function), which is not on the loader\'s list']);
  assert.deepEqual(exportProblems(wasmModule(listed.filter((x) => x !== "dump_word"))), ['the module does not export "dump_word"']);
  assert.match(exportProblems(new Uint8Array([0, 1, 2, 3]))[0], /^not a WebAssembly module: /);
  assert.deepEqual(moduleProblems(new WebAssembly.Module(wasmModule(listed))), []);
  // Refused before instantiating; and a module that imports a function is refused, naming it (FX-4's form since U4d2's
  // round 1: every import refused by moduleProblems, not left to an empty import object).
  assert.throws(() => instantiate(new WebAssembly.Module(wasmModule([...listed, "extra"]))), /^Error: refused: the module exports "extra"/);
  assert.throws(
    () => instantiate(new WebAssembly.Module(wasmModule(listed, { imports: [["env", "now"]] }))),
    /^Error: refused: the module imports "env\.now" \(a function\): the loader gives a module nothing to import$/,
  );
  assert.equal(typeof instantiate(new WebAssembly.Module(wasmModule(listed))).begin, "function");
});

test("the loader reads the module's target_features section: simd128 or relaxed-simd used or required is refused, and so is a module with none, two or one that does not parse; the built modules' list passes (U4d2 round 1, FX-2)", () => {
  const listed = [...HASH_WASM_EXPORTS, "memory"];
  const problems = (opts) => moduleProblems(new WebAssembly.Module(wasmModule(listed, opts)));
  // The built modules' list (both builds), and the two features named disallowed (`-`), pass.
  assert.deepEqual(problems({ features: BUILT_FEATURES }), []);
  assert.deepEqual(problems({ features: [...BUILT_FEATURES, "-simd128", "-relaxed-simd"] }), []);
  // Used (`+`) or required (`=`): refused by name.
  const lists = (f) => `the module's target_features section lists ${f}, which plan.md §4.5 leaves off (simd128 and relaxed-simd)`;
  for (const f of ["+simd128", "+relaxed-simd", "=simd128", "=relaxed-simd"]) assert.deepEqual(problems({ features: [...BUILT_FEATURES, f] }), [lists(f)], f);
  // sec-probe-tf2's module's list (r1-sec-tf2.log: one safe #[target_feature(enable = "relaxed-simd")] fn): both named.
  assert.deepEqual(problems({ features: ["+relaxed-simd", "+simd128", ...BUILT_FEATURES] }), [lists("+relaxed-simd"), lists("+simd128")]);
  assert.throws(() => instantiate(new WebAssembly.Module(wasmModule(listed, { features: ["+simd128"] }))), /^Error: refused: the module's target_features section lists \+simd128,/);
  // None, or two (wasm-ld writes one; a module stripped of it says nothing of its features).
  const sections = (k) => `the module has ${k} target_features sections, not one, so its target features cannot be read (wasm-ld writes one)`;
  assert.deepEqual(problems({ sections: [] }), [sections(0)]);
  assert.deepEqual(problems({ sections: [featureBody(BUILT_FEATURES), featureBody(BUILT_FEATURES)] }), [sections(2)]);
  // One that does not parse: a name cut short, a count cut short, an entry missing, a prefix of another byte, a byte
  // after the last entry, a name that is no UTF-8.
  const parse = (why) => `the module's target_features section does not parse: ${why}`;
  const whole = featureBody(["+sign-ext"]);
  assert.deepEqual(problems({ sections: [whole.slice(0, -2)] }), [parse("entry 1's name runs past its end")]);
  assert.deepEqual(problems({ sections: [[0x80]] }), [parse("it ends inside its entry count")]);
  assert.deepEqual(problems({ sections: [[2, ...whole.slice(1)]] }), [parse("it ends before entry 2 of 2")]);
  assert.deepEqual(problems({ sections: [[1, 0x2a, ...wasmName("simd128")]] }), [parse("entry 1's prefix is byte 42, not +, - or =")]);
  assert.deepEqual(problems({ sections: [[...whole, 0]] }), [parse("1 byte(s) follow its last entry")]);
  const notUtf8 = problems({ sections: [[1, 0x2b, 2, 0xc3, 0x28]] });
  assert.equal(notUtf8.length, 1, notUtf8.join("\n"));
  assert.ok(notUtf8[0].startsWith(parse("")), notUtf8[0]);
  // exportProblems (the bytes' path) reads it too.
  assert.deepEqual(exportProblems(wasmModule(listed, { sections: [] })), [sections(0)]);
});

test("the loader refuses every import by module and name before instantiating, Object.prototype's names included (U4d2 round 1, FX-4: SEC-U4D2-1)", () => {
  const listed = [...HASH_WASM_EXPORTS, "memory"];
  const imports = (m, f) => `the module imports ${JSON.stringify(`${m}.${f}`)} (a function): the loader gives a module nothing to import`;
  // sec-probe-imports' modules (r1-sec-imports.log): given `{}`, constructor.constructor (JS's Function),
  // __proto__.valueOf and constructor.keys had linked with moduleProblems []; env.now had failed only by chance.
  for (const [m, f] of [["constructor", "constructor"], ["__proto__", "valueOf"], ["constructor", "keys"], ["env", "now"]]) {
    const mod = new WebAssembly.Module(wasmModule(listed, { imports: [[m, f]] }));
    assert.deepEqual(moduleProblems(mod), [imports(m, f)], `${m}.${f}`);
    assert.throws(() => instantiate(mod), (e) => e.message === `refused: ${imports(m, f)}`, `${m}.${f}`);
  }
  assert.deepEqual(moduleProblems(new WebAssembly.Module(wasmModule(listed, { imports: [["env", "now"], ["constructor", "constructor"]] }))), [
    imports("env", "now"),
    imports("constructor", "constructor"),
  ]);
  assert.equal(typeof instantiate(new WebAssembly.Module(wasmModule(listed))).begin, "function", "a module with no import instantiates");
});

test("the loader refuses a listed name exported as another kind: begin as a global, memory as a function (U4d2 round 1, FX-5: SEC-U4D2-8, TE-L1)", () => {
  const listed = [...HASH_WASM_EXPORTS, "memory"];
  const problems = (kinds) => moduleProblems(new WebAssembly.Module(wasmModule(listed, { kinds })));
  assert.deepEqual(problems({ begin: "global" }), ['the module exports "begin" as a global, not a function']);
  assert.deepEqual(problems({ memory: "function" }), ['the module exports "memory" as a function, not a memory']);
  assert.throws(() => instantiate(new WebAssembly.Module(wasmModule(listed, { kinds: { begin: "global" } }))), /^Error: refused: the module exports "begin" as a global, not a function$/);
});

test("the loader streams a recording in chunks, numbers its ticks from the first, and names the recording and the last tick on a trap or a failure (U4d2, 2.2)", (t) => {
  const dir = mkdtempSync(join(tmpdir(), "npt-loader-"));
  t.after(() => rmSync(dir, { recursive: true, force: true }));
  const path = join(dir, "r.rec");
  // 200,003 bytes: 50,001 words, the last zero-padded, so more than one 64 KiB chunk.
  writeFileSync(path, Buffer.alloc(200_003, 1));
  const fake = (steps, trapAt = Infinity) => {
    const fed = [];
    let feeds = 0;
    let n = 0;
    return {
      fed,
      exports: {
        begin: () => 0,
        feed: (w) => {
          feeds++;
          if (fed.length < 2 || feeds === 50_001) fed.push(w >>> 0);
        },
        step: () => {
          if (feeds === 0) return STEP.NEED_MORE;
          if (n >= trapAt) throw new WebAssembly.RuntimeError("unreachable");
          if (n < steps && feeds >= (n + 1) * 10_000) return (n++, STEP.STEPPED);
          return feeds === 50_001 ? (n < steps ? STEP.NEED_MORE : STEP.DONE) : STEP.NEED_MORE;
        },
        hash_hi: () => -1,
        hash_lo: () => n,
        dump_len: () => 2,
        dump_word: (i) => (i ? -2 : 7),
      },
    };
  };
  const ok = fake(5);
  const r = replay(ok.exports, { path, name: "TC-s3", firstTick: 0xffff_fffe, dumpAt: 0 });
  assert.equal(r.ok, true, r.error);
  assert.deepEqual(r.lines, [
    "h TC-s3 4294967294 ffffffff00000001",
    "h TC-s3 4294967295 ffffffff00000002",
    "h TC-s3 0 ffffffff00000003",
    "h TC-s3 1 ffffffff00000004",
    "h TC-s3 2 ffffffff00000005",
    "stepped 5",
  ]);
  assert.equal(r.dump, "dump TC-s3 0\nhash ffffffff00000003\n00000007\nfffffffe\n");
  assert.deepEqual(ok.fed, [0x01010101, 0x01010101, 0x00010101], "little-endian words, the last zero-padded");
  // A trap, a FAILED status, a file that ends inside a tick, a dump tick never stepped, a refused begin.
  const trap = replay(fake(5, 3).exports, { path, name: "SS-s1", firstTick: 10 });
  assert.equal(trap.error, "SS-s1: the Wasm leg trapped after tick 12: unreachable");
  const failedStep = fake(5);
  failedStep.exports.step = () => STEP.FAILED;
  assert.equal(replay(failedStep.exports, { path, name: "SS-s1", firstTick: 0 }).error, "SS-s1: step() returned FAILED after no tick");
  assert.equal(replay(fake(9).exports, { path, name: "ES-s2", firstTick: 0 }).error, "ES-s2: the file ends inside a tick, after tick 4");
  assert.equal(replay(fake(5).exports, { path, name: "ES-s2", firstTick: 0, dumpAt: 99 }).error, "ES-s2: no tick 99 was stepped, so no dump was taken (after tick 4)");
  const refused = fake(5);
  refused.exports.begin = () => 1;
  assert.match(replay(refused.exports, { path, name: "SS-s1", firstTick: 0, bite: 1, arm: 5 }).error, /^SS-s1: begin\(1, 5\) returned 1, not BEGUN/);
  assert.equal(trap.lines.at(-1), `failed ${trap.error}`);
});

test("the build flags: +fma, target-cpu=native, +simd128 and +relaxed-simd refused in every form and source; -fma, +sse2 and target-cpu=x86-64 pass the name scan (U4d2, 2.4 (c) (i))", () => {
  const scan = (env, configs = []) => buildFlagProblems(rustflagSources(env, configs));
  const forms = (o) => [["-C", o], [`-C${o}`], ["--codegen", o], [`--codegen=${o}`]];
  const sources = [
    ["RUSTFLAGS", " "],
    ["CARGO_ENCODED_RUSTFLAGS", "\x1f"],
    ["CARGO_BUILD_RUSTFLAGS", " "],
    ["CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUSTFLAGS", " "],
    ["CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS", " "],
  ];
  const cases = [];
  for (const f of ["fma", "simd128", "relaxed-simd"]) {
    for (const o of [`target-feature=+${f}`, `target-feature=+sse2,+${f}`, `target_feature=+${f}`, `target-feature=-${f},+${f}`, `target-feature=${f}`]) {
      cases.push([o, `it enables ${f}, which plan.md §4.5's build flags refuse`]);
    }
  }
  for (const o of ["target-cpu=native", "target_cpu=native"]) cases.push([o, "plan.md §4.5's build flags refuse target-cpu=native"]);
  for (const [o, why] of cases) {
    for (const tokens of forms(o)) {
      for (const [k, sep] of sources) {
        assert.deepEqual(scan({ PATH: "/bin", [k]: tokens.join(sep) }), [`${k} in the environment passes ${JSON.stringify(`-C ${o}`)}: ${why}`], `${k}=${tokens}`);
      }
      // A cargo config's rustflags, as an array and as a string, in [build] and in a target table.
      for (const text of [`[build]\nrustflags = ${JSON.stringify(tokens)}\n`, `[target.wasm32-unknown-unknown]\nrustflags = ${JSON.stringify(tokens.join(" "))}\n`]) {
        assert.deepEqual(scan({}, [{ path: "c.toml", text }]), [`c.toml's rustflags passes ${JSON.stringify(`-C ${o}`)}: ${why}`], text);
      }
    }
  }
  // The three controls: no refused flag is named.
  for (const flags of ["-C target-feature=-fma", "-C target-feature=+sse2", "-C target-cpu=x86-64", "-C opt-level=3 -D warnings"]) {
    assert.deepEqual(scan({ RUSTFLAGS: flags }), [], flags);
  }
  // Each source is read: the sources' names and flags as rustc gets them.
  assert.deepEqual(rustflagSources({ RUSTFLAGS: " -C  a=1 ", CARGO_ENCODED_RUSTFLAGS: "-C b=2\x1f-C\x1fc=3", HOME: "/root" }, [{ path: "c.toml", text: 'rustflags = "-C d=4"\n[x]\nrustflags = ["-C", "e=5"]\n' }]), [
    { where: "RUSTFLAGS in the environment", flags: ["-C", "a=1"] },
    { where: "CARGO_ENCODED_RUSTFLAGS in the environment", flags: ["-C b=2", "-C", "c=3"] },
    { where: "c.toml's rustflags", flags: ["-C", "d=4"] },
    { where: "c.toml's rustflags", flags: ["-C", "e=5"] },
  ]);
});

/** readBackCheck's refusal of a read-back that lists none of a target's own features. */
const nonVacuous = (target, present, given) =>
  `rustc's read-back for ${target} (${given}) lists no target_feature="${present}": it read nothing a build for it has (a CPU ` +
  "name meant for the other target, or a parse that reads nothing), so it proves nothing";
const listsRefused = (target, f, given = "given no rustflags") =>
  `rustc's read-back for ${target} (${given}) lists target_feature="${f}", which plan.md §4.5's build flags refuse`;

test("rustc's read-back fails the step on fma (x86-64) or simd128 and relaxed-simd (wasm32), on a list with none of its own features, and on a failed run; every rustflag of every source reaches it (U4d2, 2.4 (c) (ii))", () => {
  const X = "x86_64-unknown-linux-gnu";
  const W = "wasm32-unknown-unknown";
  const check = (env, cfg = cfgModel, configs = []) => {
    const r = depsRunner({ cfg });
    return { ...readBackCheck(rustflagSources(env, configs), r.runner, "/w"), calls: r.calls };
  };
  assert.deepEqual(READ_BACK, [
    { target: X, refused: ["fma"], present: "sse2" },
    { target: W, refused: ["simd128", "relaxed-simd"], present: "sign-ext" },
  ]);
  const clean = check({});
  assert.deepEqual(clean.problems, []);
  assert.equal(
    clean.line,
    `rustc 1.99.1 (58cf8f9c3 2026-10-13)'s read-back, given no rustflags: ${X} fxsr, sse, sse2; ${W} bulk-memory, multivalue, mutable-globals, nontrapping-fptoint, reference-types, sign-ext`,
  );
  // Recorded read-backs that list a refused feature, each alone.
  const plus = (target, f) => (t, flags) => {
    const r = cfgModel(t, flags);
    return t === target ? { ...r, stdout: `${r.stdout}target_feature="${f}"\n`, out: `${r.stdout}target_feature="${f}"\n` } : r;
  };
  assert.deepEqual(check({}, plus(X, "fma")).problems, [listsRefused(X, "fma")]);
  assert.deepEqual(check({}, plus(W, "simd128")).problems, [listsRefused(W, "simd128")]);
  assert.deepEqual(check({}, plus(W, "relaxed-simd")).problems, [listsRefused(W, "relaxed-simd")]);
  // A read-back missing sse2 or sign-ext; one that lists nothing; one that exits non-zero.
  const minus = (target, f) => (t, flags) => {
    const r = cfgModel(t, flags);
    const stdout = t === target ? r.stdout.replace(`target_feature="${f}"\n`, "") : r.stdout;
    return { ...r, stdout, out: stdout };
  };
  assert.deepEqual(check({}, minus(X, "sse2")).problems, [nonVacuous(X, "sse2", "given no rustflags")]);
  assert.deepEqual(check({}, minus(W, "sign-ext")).problems, [nonVacuous(W, "sign-ext", "given no rustflags")]);
  assert.deepEqual(check({}, () => ({ ok: true, status: 0, stdout: "", out: "" })).problems, [nonVacuous(X, "sse2", "given no rustflags"), nonVacuous(W, "sign-ext", "given no rustflags")]);
  const broken = (t, flags) => (t === W ? { ok: false, status: 1, stdout: "", out: "error: Error loading target specification\n" } : cfgModel(t, flags));
  assert.deepEqual(check({}, broken).problems, [`rustc --print cfg --target ${W} (given no rustflags) failed: error: Error loading target specification`]);
  // The live plants' flags (2.7), through rustc's behaviour as the advisor's check ran it (A15; cfgModel): a CPU name
  // the name scan cannot see.
  assert.deepEqual(check({ RUSTFLAGS: "-C target-cpu=haswell" }).problems, [listsRefused(X, "fma", 'given "-C target-cpu=haswell"'), nonVacuous(W, "sign-ext", 'given "-C target-cpu=haswell"')]);
  assert.deepEqual(check({ CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUSTFLAGS: "-C target-cpu=bleeding-edge" }).problems, [
    listsRefused(W, "simd128", 'given "-C target-cpu=bleeding-edge"'),
    listsRefused(W, "relaxed-simd", 'given "-C target-cpu=bleeding-edge"'),
  ]);
  // The three controls through the whole check: -fma and +sse2 pass; target-cpu=x86-64 fails on wasm32's read-back.
  for (const f of ["-C target-feature=-fma", "-C target-feature=+sse2"]) {
    assert.deepEqual([...buildFlagProblems(rustflagSources({ RUSTFLAGS: f }, [])), ...check({ RUSTFLAGS: f }).problems], [], f);
  }
  assert.deepEqual(buildFlagProblems(rustflagSources({ RUSTFLAGS: "-C target-cpu=x86-64" }, [])), []);
  assert.deepEqual(check({ RUSTFLAGS: "-C target-cpu=x86-64" }).problems, [nonVacuous(W, "sign-ext", 'given "-C target-cpu=x86-64"')]);
  // The argv pin: four sources, each a distinct harmless flag; every one is in each rustc call, which runs where told.
  const env = {
    RUSTFLAGS: "-C debuginfo=0",
    CARGO_ENCODED_RUSTFLAGS: "-C\x1fopt-level=1",
    CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS: "-C codegen-units=1",
  };
  const pinned = check(env, cfgModel, [{ path: "c.toml", text: '[build]\nrustflags = ["-C", "overflow-checks=on"]\n' }]);
  assert.deepEqual(pinned.problems, []);
  const cfgCalls = pinned.calls.filter((c) => c.line.startsWith("rustc --print cfg"));
  assert.deepEqual(cfgCalls.map((c) => c.line.split(" ").slice(0, 5).join(" ")), [`rustc --print cfg --target ${X}`, `rustc --print cfg --target ${W}`]);
  for (const c of cfgCalls) {
    for (const f of ["debuginfo=0", "opt-level=1", "codegen-units=1", "overflow-checks=on"]) assert.ok(c.line.includes(`-C ${f}`), `${f} in ${c.line}`);
  }
  assert.ok(pinned.calls.every((c) => c.cwd === "/w"));
});

test("no rustflags source may hold a flag and no build target may be set, so the read-back is given what cargo gives rustc; RR-M2's negation pair fails naming both sources (U4d2 round 1, FX-1)", (t) => {
  // RR-M2's pair (r1-rr-superset.log): cargo used CARGO_ENCODED_RUSTFLAGS alone, so the negation in RUSTFLAGS hid
  // bleeding-edge's simd128 and relaxed-simd from a read-back given both; each source is refused by name.
  const pair = { CARGO_ENCODED_RUSTFLAGS: "-Ctarget-cpu=bleeding-edge", RUSTFLAGS: "-Ctarget-feature=-simd128,-relaxed-simd" };
  const pairLines = [holds("CARGO_ENCODED_RUSTFLAGS in the environment", "-Ctarget-cpu=bleeding-edge"), holds("RUSTFLAGS in the environment", "-Ctarget-feature=-simd128,-relaxed-simd")];
  assert.deepEqual(buildInputProblems(pair, []), pairLines);
  // SEC-U4D2-5: flags the name rule does not know, each refused here (and passed by buildFlagProblems).
  for (const flags of ['--cfg feature="bites"', "-C overflow-checks=off", "-C llvm-args=-fp-contract=fast"]) {
    assert.deepEqual(buildInputProblems({ PATH: "/bin", RUSTFLAGS: flags }, []), [holds("RUSTFLAGS in the environment", flags)], flags);
    assert.deepEqual(buildFlagProblems(rustflagSources({ RUSTFLAGS: flags }, [])), [], `${flags}: the name rule alone passes it`);
  }
  // Every source form: CARGO_BUILD_RUSTFLAGS, a target's variable, a config's rustflags in [build] and in a target table.
  assert.deepEqual(buildInputProblems({ CARGO_BUILD_RUSTFLAGS: "-C opt-level=1" }, []), [holds("CARGO_BUILD_RUSTFLAGS in the environment", "-C opt-level=1")]);
  assert.deepEqual(buildInputProblems({ CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUSTFLAGS: "-C debuginfo=0" }, []), [
    holds("CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUSTFLAGS in the environment", "-C debuginfo=0"),
  ]);
  const config = (text) => buildInputProblems({}, [{ path: "c.toml", text }]);
  assert.deepEqual(config('[build]\nrustflags = ["-C", "overflow-checks=off"]\n'), [holds("c.toml's rustflags", "-C overflow-checks=off")]);
  assert.deepEqual(config('[target.x86_64-unknown-linux-gnu]\nrustflags = "-C debuginfo=0"\n'), [holds("c.toml's rustflags", "-C debuginfo=0")]);
  // SEC-U4D2-6: a build target set by the environment (whatever its value) or by a config's build.target, in any form.
  assert.deepEqual(buildInputProblems({ CARGO_BUILD_TARGET: "wasm32-unknown-unknown" }, []), [targetEnv("wasm32-unknown-unknown")]);
  assert.deepEqual(buildInputProblems({ CARGO_BUILD_TARGET: "" }, []), [targetEnv("")]);
  for (const text of ['[build]\ntarget = "aarch64-unknown-linux-gnu"\n', 'build.target = ["x86_64-unknown-linux-musl"]\n', 'build = { target = "x" }\n']) {
    assert.deepEqual(config(text), [targetKey("c.toml")], text);
  }
  // None of it: this container's cargo environment, empty sources, the spike's own config, a target table, target-dir.
  const normal = { PATH: "/usr/bin", HOME: "/root", CARGO_HTTP_CAINFO: "/root/.ccr/ca-bundle.crt", RUSTUP_HOME: "/root/.rustup", RUST_BACKTRACE: "1" };
  assert.deepEqual(buildInputProblems({ ...normal, CARGO_TARGET_DIR: "/t", CARGO_NET_OFFLINE: "true", RUSTFLAGS: "", CARGO_ENCODED_RUSTFLAGS: "" }, []), []);
  const own = { path: "spike/.cargo/config.toml", text: readFileSync(join(HERE, ".cargo", "config.toml"), "utf8") };
  assert.deepEqual(buildInputProblems(normal, [own]), []);
  assert.deepEqual(config('[target.x86_64-unknown-linux-gnu]\nlinker = "cc"\n[build]\ntarget-dir = "t"\nrustflags = []\n[env]\nX = "target = y"\n'), []);
  // Through depsStep (review C2R-5's rule: a check unhooked from the step fails a test): the pair fails the step.
  const f = depsFixture(t);
  const r = depsStep(f.dir, { ...f.env, ...pair }, () => ({ ok: true, meta: f.meta }), f.runner);
  assert.equal(r.ok, false);
  for (const line of pairLines) assert.ok(r.out.split("\n").includes(line), r.out);
});

test("npt-sim's resolved features are one set in the server, npt-lab and npt-hash-wasm builds, like for like; a difference anywhere in its subtree fails (U4d2, 2.4 (b), PM-0004-74)", () => {
  const sim = {
    id: "sim",
    name: "npt-sim",
    dependencies: [...SIM_DEPENDENCIES.map((d) => ({ name: d, kind: null })), { name: "proptest", kind: "dev" }],
  };
  const meta = { packages: [sim], workspace_members: ["sim"] };
  const check = (trees) => {
    const r = depsRunner({ trees });
    return { ...simFeatureCheck(meta, r.runner, "/w"), calls: r.calls };
  };
  const subtree = "libm [arch, default], rand_chacha [], ppv-lite86 [simd], zerocopy [simd], rand_core []";
  const clean = check({});
  assert.deepEqual(clean.problems, []);
  assert.equal(clean.line, `npt-sim's features, one set in npt-server, npt-lab, npt-hash-wasm: measured: npt-sim [], ${subtree}; bites: npt-sim [bites], ${subtree}`);
  // The pairing pinned: three measured trees, then three bites trees, each build's package and target; a measured list
  // is never paired with a bites list (npt-sim's own `bites` differs between the sets and passes).
  assert.deepEqual(SIM_BUILDS, [
    { pkg: "npt-server", target: "x86_64-unknown-linux-gnu" },
    { pkg: "npt-lab", target: "x86_64-unknown-linux-gnu" },
    { pkg: "npt-hash-wasm", target: "wasm32-unknown-unknown" },
  ]);
  assert.deepEqual(
    clean.calls.map((c) => c.line),
    [false, true].flatMap((bites) =>
      SIM_BUILDS.map((b) => `cargo tree --locked -e normal --target ${b.target} -p ${b.pkg}${bites ? " --features bites" : ""} --no-dedupe --prefix depth --format {p} [{f}]`),
    ),
  );
  const S = "npt-server (x86_64-unknown-linux-gnu, measured)";
  const L = "npt-lab (x86_64-unknown-linux-gnu, measured)";
  const HW = "npt-hash-wasm (wasm32-unknown-unknown, measured)";
  // PM-0004-74's bite: the server build's libm with one feature more, named with both builds and the features.
  assert.deepEqual(check({ "npt-server": treeText("npt-server", false, { libm: "arch,default,std" }) }).problems, [
    `npt-sim's libm v0.2.16 resolves [arch, default, std] in the ${S} build and [arch, default] in the ${L} build`,
    `npt-sim's libm v0.2.16 resolves [arch, default, std] in the ${S} build and [arch, default] in the ${HW} build`,
  ]);
  // A transitive dependency only (zerocopy, below rand_chacha through ppv-lite86), in the bites set: a walk of npt-sim's
  // direct dependencies alone would pass it. Every pair is compared, so each build it sets apart is named.
  assert.deepEqual(check({ "npt-hash-wasm bites": treeText("npt-hash-wasm", true, { zerocopy: "simd,std" }) }).problems, [
    "npt-sim's zerocopy v0.8.63 resolves [simd] in the npt-server (x86_64-unknown-linux-gnu, bites) build and [simd, std] in the npt-hash-wasm (wasm32-unknown-unknown, bites) build",
    "npt-sim's zerocopy v0.8.63 resolves [simd] in the npt-lab (x86_64-unknown-linux-gnu, bites) build and [simd, std] in the npt-hash-wasm (wasm32-unknown-unknown, bites) build",
  ]);
  // ADR 0004:1184's control: a feature on a package outside npt-sim's subtree passes.
  assert.deepEqual(check({ "npt-server": treeText("npt-server", false, { getrandom: "std" }) }).problems, []);
  // npt-sim's own features differing within one set fail (2.7's live plant: hash-wasm's npt-sim with `bites`).
  assert.deepEqual(check({ "npt-hash-wasm": treeText("npt-hash-wasm", false, { "npt-sim": "bites" }) }).problems, [
    `npt-sim's npt-sim v0.0.0 resolves [] in the ${S} build and [bites] in the ${HW} build`,
    `npt-sim's npt-sim v0.0.0 resolves [] in the ${L} build and [bites] in the ${HW} build`,
  ]);
  // Not vacuous: a tree with no npt-sim subtree, or one missing a dependency npt-sim's manifest names, fails.
  const noSim = "0npt-lab v0.0.0 (/w/crates/lab) []\n1serde v1.0.228 [std]\n";
  assert.deepEqual(check({ "npt-lab": noSim }).problems, [`the ${L} build's tree holds no npt-sim subtree, so its features prove nothing`]);
  const noLibm = treeText("npt-hash-wasm", false).replace("2libm v0.2.16 [arch,default]\n", "");
  assert.deepEqual(check({ "npt-hash-wasm": noLibm }).problems, [
    `the ${HW} build's npt-sim subtree lacks libm, which crates/sim/Cargo.toml names`,
    `npt-sim's libm v0.2.16 resolves [arch, default] in the ${S} build and is not in its npt-sim subtree in the ${HW} build`,
    `npt-sim's libm v0.2.16 resolves [arch, default] in the ${L} build and is not in its npt-sim subtree in the ${HW} build`,
  ]);
  // A cargo tree that fails is a problem, not a pass.
  const r = depsRunner();
  const failing = (cmd, args, cwd) => (args.includes("npt-lab") ? { ok: false, status: 101, stdout: "", out: "error: no Cargo.lock\n" } : r.runner(cmd, args, cwd));
  assert.deepEqual(simFeatureCheck(meta, failing, "/w").problems, [`cargo tree for the ${L} build failed: error: no Cargo.lock`, "cargo tree for the npt-lab (x86_64-unknown-linux-gnu, bites) build failed: error: no Cargo.lock"]);
  // simSubtree reads the first npt-sim to the next line at its depth or above.
  assert.deepEqual([...simSubtree(treeText("npt-server", false)).keys()], ["npt-sim v0.0.0", "libm v0.2.16", "rand_chacha v0.10.0", "ppv-lite86 v0.2.21", "zerocopy v0.8.63", "rand_core v0.10.1"]);
  assert.equal(simSubtree(noSim), null);
});

test("the test step names every failing test from libtest's failures: list beside its summary, in the lines main prints (carry 65; j-r2 B2.3)", () => {
  // libtest's stdout for a failing binary: the panics' failures: block, then the names' one.
  const libtest = (names) =>
    `running ${names.length + 1} tests\ntest a ... ok\n${names.map((x) => `test ${x} ... FAILED`).join("\n")}\n\nfailures:\n\n` +
    `${names.map((x) => `---- ${x} stdout ----\n\nthread '${x}' panicked at src/lib.rs:1:1:\nboom\n`).join("\n")}\n\nfailures:\n` +
    `${names.map((x) => `    ${x}`).join("\n")}\n\ntest result: FAILED. 1 passed; ${names.length} failed; 0 ignored; 0 measured; 0 filtered out\n\n`;
  // `run` joins stdout before stderr, and stderr's Compiling and Running lines push the names out of main's 40.
  const stderr = `${Array.from({ length: 60 }, (_, i) => `   Compiling c${i} v0.0.0`).join("\n")}\n     Running unittests src/lib.rs (x)\nerror: test failed, to rerun pass \`-p npt-lab --lib\`\n`;
  const failing = (names) => ({ ok: false, status: 101, stdout: libtest(names), out: `${libtest(names)}${stderr}` });
  assert.ok(!tail(failing(["tests::h4_one"]).out).includes("    tests::h4_one"), "without the line, main's 40 lines miss the name");
  // One failing test.
  let s = testBothBuilds(recorder([failing(["tests::h4_one"])]).runner);
  assert.ok(tail(s.out).includes("the measured build's failing tests (libtest's failures: list): tests::h4_one"), tail(s.out));
  assert.equal(lastLine(s.out), "the measured build's tests (1 tests passed, 1 failed, 0 reported a skip) failed: error: test failed, to rerun pass `-p npt-lab --lib`");
  // Two failing tests in one test binary, both named.
  s = testBothBuilds(recorder([failing(["hash::tests::h5_a", "probe::tests::h7_b"])]).runner);
  assert.ok(tail(s.out).includes("the measured build's failing tests (libtest's failures: list): hash::tests::h5_a, probe::tests::h7_b"), tail(s.out));
  // A failure in the bites build only, named with the build.
  const passed = { ok: true, status: 0, stdout: "test result: ok. 5 passed; 0 failed; 0 ignored\n", out: "test result: ok. 5 passed; 0 failed; 0 ignored\n" };
  s = testBothBuilds(recorder([passed, failing(["hash::tests::h6_the_bites_build"])]).runner);
  assert.ok(tail(s.out).includes("the bites build's failing tests (libtest's failures: list): hash::tests::h6_the_bites_build"), tail(s.out));
  assert.match(lastLine(s.out), /^the bites build's tests \(measured build: 5 tests passed, 0 failed, 0 reported a skip; bites build: 1 tests passed, 1 failed, 0 reported a skip\) failed: /);
  // A failed run with no failures: list (a build error) says so; failingTests reads only the names' list.
  s = testBothBuilds(recorder([{ ok: false, status: 101, stdout: "", out: "error[E0425]: cannot find value `x`\n" }]).runner);
  assert.ok(tail(s.out).includes("the measured build: no failing test in libtest's failures: list (a build or harness error: the lines above)"));
  assert.deepEqual(failingTests(libtest(["x::y", "x::z"])), ["x::y", "x::z"]);
  assert.deepEqual(failingTests("test result: ok. 1 passed; 0 failed\n"), []);
});

test("a failed step's whole output is kept in its file under the checkout's .gate/, named in its FAIL lines, and removed when the step next runs (U4d2 round 1, FX-10: TE-L4)", (t) => {
  assert.equal(FAILED_STEP_DIR, join(HERE, "..", "..", ".gate", "netcode-proof-test", "failed-steps"));
  const dir = mkdtempSync(join(tmpdir(), "npt-failed-"));
  t.after(() => rmSync(dir, { recursive: true, force: true }));
  // A failed test step as testBothBuilds returns it: the panic, then 60 lines of cargo's stderr, then the summary, so
  // main's 40 lines miss the panic (TE-M3's case: r1-te-final-check.log).
  const panic = "thread 'l2::delay::tests::a_shutdown' panicked at crates/net/src/l2/delay_tests.rs:2590:5:\nassertion `left == right` failed\n  left: 2\n right: 1";
  const out =
    `running 1 test\ntest l2::delay::tests::a_shutdown ... FAILED\n\nfailures:\n\n---- l2::delay::tests::a_shutdown stdout ----\n${panic}\n\n` +
    `failures:\n    l2::delay::tests::a_shutdown\n\ntest result: FAILED. 0 passed; 1 failed\n` +
    `${Array.from({ length: 60 }, (_, i) => `   Compiling c${i} v0.0.0`).join("\n")}\nthe bites build's tests (x) failed: error: test failed`;
  const printed = [];
  assert.equal(main(["--only", "test"], { test: () => ({ ok: false, out }) }, (l) => printed.push(l), dir), 1);
  const file = join(dir, "test.log");
  assert.ok(!printed.join("\n").includes("panicked at"), "main's 40 lines miss the panic");
  assert.equal(readFileSync(file, "utf8"), `${out}\n`, "the file holds the whole output, the panic included");
  assert.ok(printed.includes(`      the step's whole output (${out.split("\n").length} lines): ${file}`), printed.join("\n"));
  assert.equal(printed.at(-1), "check: 1 step(s) failed");
  // The next run of the step removes it first (it passes now); another step's file stays as it was.
  writeFileSync(join(dir, "deps.log"), "an older run's\n");
  assert.equal(main(["--only", "test"], { test: () => ({ ok: true, out: "measured build: 1 tests passed" }) }, () => {}, dir), 0);
  assert.deepEqual(readdirSync(dir), ["deps.log"]);
});

/** The wasm step's folders and copies, as it lays them out under WASM_STEP_DIR. */
const WS = {
  bin: join(WASM_STEP_DIR, "bin"),
  rec: join(WASM_STEP_DIR, "recordings"),
  meas: join(WASM_STEP_DIR, "measured"),
  hm: join(WASM_STEP_DIR, "hash-map"),
  sin: join(WASM_STEP_DIR, "std-sin"),
};
const LAB = join(WS.bin, "npt-lab");
const LAB_BITES = join(WS.bin, "npt-lab-bites");
const WASM_M = join(WS.bin, "npt-hash-wasm.wasm");
const WASM_B = join(WS.bin, "npt-hash-wasm-bites.wasm");
const RECS = [
  { name: "SS-s1", first: 0 },
  { name: "ES-s2", first: 1_000_000 },
  { name: "TC-s3", first: 4_294_962_796 },
];
const COUNTS3 = "SS-s1 9000, ES-s2 9000, TC-s3 9000";
/** A compare_text line (npt-lab's): equal, or a first mismatch at `rec` `tick` with `field` (or none). */
const cmpLine = (a, b, { compared = 27_000, mismatching = 0, rec, tick, field, counts = COUNTS3 } = {}) =>
  mismatching === 0
    ? `${a} against ${b}: ${compared} ticks compared, 0 mismatching, equal (${counts})`
    : `${a} against ${b}: ${compared} ticks compared, ${mismatching} mismatching; first mismatch at entry 7: ${rec} tick ${tick} hash ` +
      `00000000000000aa against ${rec} tick ${tick} hash 00000000000000bb; ${field ? `field ${field} (word 9): 1 and 2` : "no field (no dump at that tick)"} (${counts})`;
const NOT_CATCHABLE_SCAN = "# std-sin's scan\nnot catchable at this site on x86-64 and wasm32\nscanned on: the test's tree\nlibc: ldd (test) 2.39\n";
/** A scan with two catches, listed out of the manifest's order (ES-s2 comes before TC-s3 there, so ES-s2's is armed). */
const CATCH_SCAN = "catch TC-s3 4294962800 vehicle[0].pos.y\ncatch ES-s2 1000123 vehicle[3].pos.y\nlibc: ldd (Ubuntu GLIBC 2.39-0ubuntu8.6) 2.39\n";
/** A scan whose first catch in the manifest's order is on SS-s1 before HASH_MAP_ARM, as the committed scan's is. */
const CATCH_SCAN_SS = "catch ES-s2 1000123 vehicle[3].pos.y\ncatch SS-s1 2 vehicle[8].pos.y\nlibc: ldd (Ubuntu GLIBC 2.39-0ubuntu8.6) 2.39\n";

/**
 * A recorded wasm step: a runner that answers each command as cargo, npt-lab and the loader do (an equal run, the
 * hash-map bite caught natively and not catchable on wasm32, std-sin by `scan`: caught and re-shown at its first catch
 * in the manifest's order), `over[kind]` replacing one answer (`ms`: the call's time, 1,500 by default), and an
 * in-memory io (the manifest `manifestPath` holding `recs`, and the scan data, read; the files each command writes
 * known to exist). `options` goes to wasmStep as its fifth argument.
 */
function wasmWorld({ over = {}, scan = NOT_CATCHABLE_SCAN, recs = RECS, manifestPath = RECORDINGS_MANIFEST, options } = {}) {
  const calls = [];
  const files = new Set();
  const copies = [];
  const manifest = { recordings: recs.map((r) => ({ name: r.name, first_tick: r.first, ticks: 9_000 })) };
  const all = { compared: 9_000 * recs.length, counts: recs.map((r) => `${r.name} 9000`).join(", ") };
  const order = recs.map((r) => r.name);
  const sc = readScan(scan)
    .catches.filter((c) => order.includes(c.recording))
    .sort((x, y) => order.indexOf(x.recording) - order.indexOf(y.recording))[0] ?? { recording: "ES-s2", tick: 1_000_123, field: "vehicle[3].pos.y" };
  const sinAt = { compared: 9_000, mismatching: 8_000, rec: sc.recording, tick: sc.tick, field: sc.field, counts: `${sc.recording} 9000` };
  const arg = (args, k) => (args.includes(k) ? args[args.indexOf(k) + 1] : null);
  const kindOf = (cmd, args) => {
    if (cmd === "cargo") return `build ${args.includes("npt-hash-wasm") ? "wasm" : "lab"}${args.includes("bites") ? " bites" : ""}`;
    if (cmd === process.execPath) {
      const out = arg(args, "--out");
      if (out.startsWith(WS.hm)) return `hash-map wasm ${out.endsWith("wasm-1.lines") ? 1 : 2}`;
      if (out.startsWith(WS.sin)) return "std-sin wasm";
      if (arg(args, "--dump-at") === "end") return `h2 ${arg(args, "--name")}`;
      return `${arg(args, "--dump-at") ? "wasm dump" : "wasm"} ${arg(args, "--name")}`;
    }
    if (["bites", "vectors"].includes(args[1])) return args[1];
    const where = arg(args, "--out") ?? arg(args, "--ours");
    const area = where.startsWith(WS.hm) ? "hash-map " : where.startsWith(WS.sin) ? "std-sin " : "";
    if (args[1] !== "compare") return `${area}legs`;
    if (arg(args, "--theirs-name") === "std-sin native leg 1") return "std-sin re-show";
    if (area === "hash-map ") return `hash-map compare ${arg(args, "--theirs-name") === "Wasm leg 2" ? "ww" : "nw"}`;
    return `${area}compare${!area && args.includes("--theirs-dump") ? " field" : ""}`;
  };
  const artifact = (name, kind, path) =>
    `${JSON.stringify({ reason: "compiler-artifact", target: { name, kind: [kind] }, executable: kind === "bin" ? path : null, filenames: [path] })}\n`;
  const answer = (kind, args) => {
    const name = arg(args, "--name");
    const said = {
      "build lab": { stdout: artifact("npt-lab", "bin", "/t/release/npt-lab") },
      "build lab bites": { stdout: artifact("npt-lab", "bin", "/t/release/npt-lab") },
      "build wasm": { stdout: artifact("npt_hash_wasm", "cdylib", "/t/wasm32-unknown-unknown/release/npt_hash_wasm.wasm") },
      "build wasm bites": { stdout: artifact("npt_hash_wasm", "cdylib", "/t/wasm32-unknown-unknown/release/npt_hash_wasm.wasm") },
      legs: {
        stdout:
          "wrote the header-only recordings vector-SS, vector-ES, vector-CO, vector-TC (seed 1, first tick 0, no tick) for h2\n" +
          `${recs.map((r) => `regenerated ${r.name}: 100 bytes, sha256 ab, as the manifest's\n`).join("")}${cmpLine("native leg 1", "native leg 2", all)}\n`,
        writes: [...recs.map((r) => join(WS.rec, `${r.name}.rec`)), ...["SS", "ES", "CO", "TC"].map((s) => join(WS.rec, `vector-${s}.rec`))],
      },
      compare: { stdout: `${cmpLine("native leg 1", "Wasm leg 1", all)}\n` },
      vectors: { stdout: "h2: CO's wasm32 dump is the committed vector (260 words, digest b432ac332054c77b); every scene's initial dump and hash are equal natively and on wasm32 (SS 6488, ES 1944, CO 260, TC 340)\n" },
      bites: { stdout: "1 PM-0004-21/hash-map\n2 PM-0004-21/std-sin\n3 PM-0001-41/server-speed-1pct\n4 PM-0001-41/server-quant-differs\n" },
      "hash-map legs": {
        status: 1,
        stdout:
          "leg 1 order digest 1111111111111111 (PM-0004-21/hash-map armed at SS-s1 tick 1234)\nleg 2 order digest 2222222222222222 (PM-0004-21/hash-map armed at SS-s1 tick 1234)\n" +
          `${cmpLine("native leg 1", "native leg 2", { compared: 9_000, mismatching: 7_766, rec: "SS-s1", tick: 1234, field: HASH_MAP_FIELD, counts: "SS-s1 9000" })}\n`,
      },
      "hash-map compare nw": {
        status: 1,
        stdout:
          `native leg 1 digest 1111111111111111 (${HASH_MAP_FIELD} at words [6488, 6489] of its dump at SS-s1 tick 1234)\n` +
          `Wasm leg 1 digest 3333333333333333 (${HASH_MAP_FIELD} at words [6488, 6489] of its dump at SS-s1 tick 1234)\n` +
          `${cmpLine("native leg 1", "Wasm leg 1", { compared: 9_000, mismatching: 7_766, rec: "SS-s1", tick: 1234, field: HASH_MAP_FIELD, counts: "SS-s1 9000" })}\n`,
      },
      "hash-map compare ww": {
        stdout:
          `Wasm leg 1 digest 3333333333333333 (${HASH_MAP_FIELD} at words [6488, 6489] of its dump at SS-s1 tick 1234)\n` +
          `Wasm leg 2 digest 3333333333333333 (${HASH_MAP_FIELD} at words [6488, 6489] of its dump at SS-s1 tick 1234)\n` +
          `${cmpLine("Wasm leg 1", "Wasm leg 2", { compared: 9_000, counts: "SS-s1 9000" })}\n`,
      },
      "std-sin legs": { stdout: `${cmpLine("native leg 1", "native leg 2", { compared: 9_000, counts: `${sc.recording} 9000` })}\n` },
      "std-sin compare": { status: 1, stdout: `${cmpLine("native leg 1", "Wasm leg 1", sinAt)}\n` },
      "std-sin re-show": { status: 1, stdout: `${cmpLine("clean native leg 1", "std-sin native leg 1", sinAt)}\n` },
    }[kind];
    if (said) return said;
    // The loader's runs.
    const armed = args.includes("--bite") ? `, bite ${arg(args, "--bite")} armed at tick ${arg(args, "--arm")}` : "";
    const dumped = args.includes("--dump-at") ? `, its dump at ${arg(args, "--dump-at")} written` : "";
    const ticks = kind.startsWith("h2 ") ? 0 : 9_000;
    return { stdout: `wasm32 leg: ${name}: ${ticks} ticks stepped${armed}${dumped}\n`, writes: [arg(args, "--out")] };
  };
  const io = {
    reset: () => files.clear(),
    mkdir: () => {},
    copy: (from, to) => copies.push([from, to]),
    exists: (p) => files.has(p),
    read: (p) => {
      if (p === manifestPath) return JSON.stringify(manifest);
      if (p === STD_SIN_SCAN) return scan;
      throw new Error(`ENOENT ${p}`);
    },
    libc: () => "glibc 2.39 (Node's report)",
  };
  let clock = 0;
  // `limit` is the call's timeout (U4d2 round 1, FX-7); an answer with `timedOut` reports the run stopped by it.
  const runner = (cmd, args, cwd, env, limit) => {
    const kind = kindOf(cmd, args);
    calls.push({ kind, cwd, limit, line: `${cmd === process.execPath ? "node" : cmd} ${args.join(" ")}` });
    const base = answer(kind, args);
    const r = over[kind] ? { ...base, ...over[kind](base, args) } : base;
    for (const w of r.writes ?? []) files.add(w);
    clock += r.ms ?? 1_500;
    const status = r.timedOut ? null : (r.status ?? 0);
    return { ok: status === 0, status, stdout: r.stdout ?? "", out: `${r.stdout ?? ""}${r.stderr ?? ""}`, timedOut: Boolean(r.timedOut) };
  };
  const result = wasmStep(runner, io, () => clock, undefined, options);
  return { ...result, calls, copies, kinds: calls.map((c) => c.kind) };
}

test("the wasm step's builds, legs and comparisons, in order; a clean run passes and says each pair, h2, each bite and each call's seconds (U4d2, 2.4 (a) and (g))", () => {
  const w = wasmWorld();
  assert.equal(w.ok, true, w.out);
  const loader = (wasm, r, out, extra = "") => `node ${LOADER} --wasm ${wasm} --rec ${join(WS.rec, `${r.name}.rec`)} --name ${r.name} --first-tick ${r.first} --out ${out}${extra}`;
  const manifestArgs = `--manifest ${RECORDINGS_MANIFEST} --dir ${WS.rec}`;
  const hmBite = `--bite ${WASM_BITES.hashMap} --arm ${HASH_MAP_ARM} --on SS-s1`;
  const hmCompare = (ours, theirs, a, b) =>
    `${LAB_BITES} hash compare --ours ${join(WS.hm, `${ours}.lines`)} --theirs ${join(WS.hm, `${theirs}.lines`)} ${manifestArgs} --ours-dump ${join(WS.hm, `${ours}.dump`)} --theirs-dump ${join(WS.hm, `${theirs}.dump`)} --digest-field ${HASH_MAP_FIELD} ${hmBite} --ours-name ${a} --theirs-name ${b}`;
  assert.deepEqual(
    w.calls.map((c) => c.line),
    [
      ...WASM_BUILDS.map((b) => `cargo ${b.args.join(" ")}`),
      `${LAB} hash ${manifestArgs} --out ${WS.meas}`,
      ...RECS.map((r) => loader(WASM_M, r, join(WS.meas, `wasm-${r.name}.lines`))),
      `${LAB} hash compare --ours ${join(WS.meas, "native-1.lines")} --theirs ${RECS.map((r) => join(WS.meas, `wasm-${r.name}.lines`)).join(",")} ${manifestArgs} --ours-name native leg 1 --theirs-name Wasm leg 1`,
      ...["SS", "ES", "CO", "TC"].map((s) => loader(WASM_M, { name: `vector-${s}`, first: 0 }, join(WS.meas, `vector-${s}.lines`), ` --dump-at end --dump-out ${join(WS.meas, `vector-${s}.dump`)}`)),
      `${LAB} hash vectors --dir ${WS.rec} --dumps ${WS.meas} --committed ${H2_VECTOR}`,
      `${LAB_BITES} hash bites`,
      `${LAB_BITES} hash ${manifestArgs} --out ${WS.hm} --only SS-s1 --bite ${WASM_BITES.hashMap} --arm ${HASH_MAP_ARM} --dump-at ${HASH_MAP_ARM}`,
      ...[1, 2].map((k) => loader(WASM_B, RECS[0], join(WS.hm, `wasm-${k}.lines`), ` --bite 1 --arm ${HASH_MAP_ARM} --dump-at ${HASH_MAP_ARM} --dump-out ${join(WS.hm, `wasm-${k}.dump`)}`)),
      hmCompare("native-1", "wasm-1", "native leg 1", "Wasm leg 1"),
      hmCompare("wasm-1", "wasm-2", "Wasm leg 1", "Wasm leg 2"),
    ],
  );
  assert.ok(w.calls.every((c) => c.cwd === HERE), "every call runs from the spike's folder");
  // The four release builds, each copied out of cargo's own path before the next replaces it.
  assert.deepEqual(w.copies, [
    ["/t/release/npt-lab", LAB],
    ["/t/release/npt-lab", LAB_BITES],
    ["/t/wasm32-unknown-unknown/release/npt_hash_wasm.wasm", WASM_M],
    ["/t/wasm32-unknown-unknown/release/npt_hash_wasm.wasm", WASM_B],
  ]);
  assert.deepEqual(
    WASM_BUILDS.map((b) => b.args.join(" ")),
    [
      "build --locked --release -p npt-lab --message-format=json",
      "build --locked --release -p npt-lab --features bites --message-format=json",
      "build --locked --release --target wasm32-unknown-unknown -p npt-hash-wasm --message-format=json",
      "build --locked --release --target wasm32-unknown-unknown -p npt-hash-wasm --features bites --message-format=json",
    ],
  );
  const last = lastLine(w.out);
  for (const said of [
    `native leg 1 against native leg 2: 27000 ticks compared, 0 mismatching, equal (${COUNTS3})`,
    `native leg 1 against Wasm leg 1: 27000 ticks compared, 0 mismatching, equal (${COUNTS3})`,
    "h2 equal",
    `hash-map: native leg 1 - native leg 2 caught at SS-s1 tick 1234, ${HASH_MAP_FIELD} (digests 1111111111111111, 2222222222222222)`,
    `native leg 1 - Wasm leg 1 caught at SS-s1 tick 1234, ${HASH_MAP_FIELD} (digests 1111111111111111, 3333333333333333)`,
    "Wasm leg 1 - Wasm leg 2 not exercised (digests equal, 3333333333333333); not catchable on wasm32 (its two Wasm digests are equal)",
    "std-sin: not catchable at this site on x86-64 and wasm32 (the scan: scanned on: the test's tree; libc: ldd (test) 2.39)",
    "seconds: build npt-lab 1.5, build npt-lab-bites 1.5,",
  ]) {
    assert.ok(last.includes(said), `${said}\nin: ${last}`);
  }
  assert.equal((last.match(/ 1\.5(?:,|$)/g) ?? []).length, w.calls.length, "each timed call's seconds");
  assert.deepEqual(STEPS.wasm.length, 0, "STEPS.wasm(runner) defaults its runner");
  assert.deepEqual(Object.keys(STEPS), ["fmt", "clippy", "clippy-wasm32", "doc", "test", "deps", "wasm", "refs", "marks", "self", "redact"]);
  // The std-sin catch path: armed at the scan's first catch in the manifest's order, native leg 1 and Wasm leg 1, then
  // the re-show (carry 83 (c)): the hash-map half's native leg 1, clean before its arming tick, against the bitten
  // native leg, its field named by compare's own clean replay against the bitten leg's dump.
  const c = wasmWorld({ scan: CATCH_SCAN_SS });
  assert.equal(c.ok, true, c.out);
  const sinArgs = "--bite PM-0004-21/std-sin --arm 2";
  assert.deepEqual(c.calls.slice(-4).map((x) => x.line), [
    `${LAB_BITES} hash ${manifestArgs} --out ${WS.sin} --only SS-s1 ${sinArgs} --dump-at 2`,
    loader(WASM_B, RECS[0], join(WS.sin, "wasm-1.lines"), ` --bite 2 --arm 2 --dump-at 2 --dump-out ${join(WS.sin, "wasm-1.dump")}`),
    `${LAB_BITES} hash compare --ours ${join(WS.sin, "native-1.lines")} --theirs ${join(WS.sin, "wasm-1.lines")} ${manifestArgs} --ours-dump ${join(WS.sin, "native-1.dump")} --theirs-dump ${join(WS.sin, "wasm-1.dump")} ${sinArgs} --on SS-s1 --ours-name native leg 1 --theirs-name Wasm leg 1`,
    `${LAB_BITES} hash compare --ours ${join(WS.hm, "native-1.lines")} --theirs ${join(WS.sin, "native-1.lines")} ${manifestArgs} --theirs-dump ${join(WS.sin, "native-1.dump")} --ours-name clean native leg 1 --theirs-name std-sin native leg 1`,
  ]);
  assert.ok(
    lastLine(c.out).includes("std-sin: caught at SS-s1 tick 2, vehicle[8].pos.y, as the scan found; re-shown against a clean bites-build native leg (the scan's other catches: ES-s2 tick 1000123)"),
    lastLine(c.out),
  );
  // Carry 83 (c)'s failing cases: a catch not on the manifest's first recording before HASH_MAP_ARM (no clean leg
  // covers it), and a re-show whose first difference is not the scan's.
  const notFirst = "std-sin: the scan's catch ES-s2 tick 1000123 is not on the manifest's first recording (SS-s1) before the hash-map bite's arming tick 1234";
  for (const scan of [CATCH_SCAN, "catch SS-s1 1234 vehicle[8].pos.y\n"]) {
    const w2 = wasmWorld({ scan });
    assert.equal(w2.ok, false, w2.out);
    assert.ok(tail(w2.out).includes(scan === CATCH_SCAN ? notFirst : "the scan's catch SS-s1 tick 1234 is not on the manifest's first recording"), tail(w2.out));
    assert.ok(!w2.kinds.includes("std-sin re-show"), "no re-show runs");
  }
  const off = wasmWorld({
    scan: CATCH_SCAN_SS,
    over: { "std-sin re-show": () => ({ status: 1, stdout: `${cmpLine("clean native leg 1", "std-sin native leg 1", { compared: 9_000, mismatching: 9, rec: "SS-s1", tick: 3, field: "vehicle[8].pos.y", counts: "SS-s1 9000" })}\n` }) },
  });
  assert.equal(off.ok, false, off.out);
  assert.ok(tail(off.out).includes("std-sin: the re-show (the bitten native leg against a clean bites-build native leg) first differs at SS-s1 tick 3, field vehicle[8].pos.y, not at the scan's SS-s1 tick 2, vehicle[8].pos.y"), tail(off.out));
});

test("the wasm step fails on every outcome that is not an equal, counted run, naming what failed within the lines main prints (U4d2, 2.4 (a))", () => {
  const fails = (world, ...said) => {
    assert.equal(world.ok, false, world.out);
    for (const s of said) assert.ok(tail(world.out).includes(s), `${s}\nnot in main's 40 lines:\n${tail(world.out)}`);
  };
  // A native-against-Wasm mismatch: the loader again with that tick's dump, the compare again, which names the field.
  const mismatch = (field) => ({
    compare: () => ({ status: 1, stdout: `${cmpLine("native leg 1", "Wasm leg 1", { mismatching: 3, rec: "ES-s2", tick: 1_000_500 })}\n` }),
    "compare field": () => ({ status: 1, stdout: `${cmpLine("native leg 1", "Wasm leg 1", { mismatching: 3, rec: "ES-s2", tick: 1_000_500, field })}\n` }),
  });
  const nw = wasmWorld({ over: mismatch("soldier[3].pos.x") });
  fails(nw, "native leg 1 against Wasm leg 1: 3 mismatching; first mismatch at ES-s2 tick 1000500", "field soldier[3].pos.x (word 9)");
  assert.deepEqual(nw.kinds.slice(9, 11), ["wasm dump ES-s2", "compare field"]);
  assert.ok(nw.calls[9].line.endsWith(`--dump-at 1000500 --dump-out ${join(WS.meas, "wasm-ES-s2-1000500.dump")}`));
  // Leg 1 against leg 2.
  fails(
    wasmWorld({ over: { legs: (b) => ({ status: 1, stdout: b.stdout.replace(cmpLine("native leg 1", "native leg 2"), cmpLine("native leg 1", "native leg 2", { mismatching: 1, rec: "TC-s3", tick: 4_294_967_000, field: "soldier[0].hp" })) }) } }),
    "native leg 1 against native leg 2: 1 mismatching; first mismatch at TC-s3 tick 4294967000",
    "field soldier[0].hp",
  );
  // A SHA-256 or length that differs at the regeneration: the recording named; h2 still runs; the legs on it do not.
  const regen = wasmWorld({
    over: {
      legs: () => ({
        status: 1,
        stdout: "wrote the header-only recordings vector-SS, vector-ES, vector-CO, vector-TC (seed 1, first tick 0, no tick) for h2\n",
        stderr: "npt-lab hash: ES-s2: regenerated 16700849 bytes with sha256 aa; the manifest has 16700849 bytes with sha256 bb\n",
        writes: [join(WS.rec, "SS-s1.rec"), ...["SS", "ES", "CO", "TC"].map((s) => join(WS.rec, `vector-${s}.rec`))],
      }),
    },
  });
  fails(regen, "the recordings manifest: ES-s2: regenerated 16700849 bytes with sha256 aa; the manifest has 16700849 bytes with sha256 bb");
  assert.ok(regen.kinds.includes("wasm SS-s1") && !regen.kinds.includes("wasm ES-s2") && regen.kinds.includes("vectors"));
  // A trap, and a FAILED status: the recording and the last tick stepped.
  fails(
    wasmWorld({ over: { "wasm TC-s3": () => ({ status: 1, stdout: "", stderr: "loader: TC-s3: the Wasm leg trapped after tick 4294962900: unreachable\n" }) } }),
    "Wasm leg 1: loader: TC-s3: the Wasm leg trapped after tick 4294962900: unreachable",
  );
  fails(
    wasmWorld({ over: { "wasm SS-s1": () => ({ status: 1, stdout: "", stderr: "loader: SS-s1: step() returned FAILED after tick 41\n" }) } }),
    "Wasm leg 1: loader: SS-s1: step() returned FAILED after tick 41",
  );
  // 0 ticks compared; a short leg (the recording, both counts).
  fails(wasmWorld({ over: { compare: () => ({ stdout: "native leg 1 against Wasm leg 1: 0 ticks compared (nothing to compare) ()\n" }) } }), "native leg 1 against Wasm leg 1: 0 ticks compared, not the manifest's 27000");
  fails(wasmWorld({ over: { "wasm ES-s2": (b) => ({ stdout: b.stdout.replace("9000 ticks", "8999 ticks") }) } }), "Wasm leg 1: ES-s2 stepped 8999 ticks, the manifest's 9000");
  fails(
    wasmWorld({ over: { compare: () => ({ status: 1, stdout: `${cmpLine("native leg 1", "Wasm leg 1", { compared: 26_999, mismatching: 1, rec: "x", tick: 1, counts: "SS-s1 9000, ES-s2 8999, TC-s3 9000" })}\n` }) } }),
    "native leg 1 against Wasm leg 1: ES-s2 has 8999 ticks compared, the manifest's 9000",
  );
  // An h2 vector word that differs: its index and field.
  fails(
    wasmWorld({
      over: {
        vectors: () => ({
          status: 1,
          stdout: "h2: CO's wasm32 dump differs from the committed vector at word 17 (soldier[2].pos.x): 3f800000 committed, 3f800001 on wasm32\nh2: the vectors differ (the lines above)\n",
        }),
      },
    }),
    "h2: CO's wasm32 dump differs from the committed vector at word 17 (soldier[2].pos.x)",
  );
  // The loader's refusal of an export more, by name.
  fails(
    wasmWorld({ over: { "wasm SS-s1": () => ({ status: 1, stdout: "", stderr: `loader: ${WASM_M}: refused: the module exports "extra" (a function), which is not on the loader's list\n` }) } }),
    'refused: the module exports "extra" (a function)',
  );
  // A clean-half leg carrying an order digest or a bite: a bites build's output given to the measured half.
  fails(wasmWorld({ over: { legs: (b) => ({ stdout: `leg 1 order digest 1234 (x)\n${b.stdout}` }) } }), "a clean-half native leg wrote an order digest");
  fails(wasmWorld({ over: { "wasm ES-s2": (b) => ({ stdout: b.stdout.replace("stepped", "stepped, bite 1 armed at tick 1234") }) } }), "Wasm leg 1: ES-s2's leg carries a bite");
  // A build that fails, or that names no artifact.
  fails(wasmWorld({ over: { "build wasm bites": () => ({ status: 101, stdout: "", stderr: "error[E0425]: cannot find value `x`\nerror: could not compile\n" }) } }), "the release build of npt-hash-wasm (wasm32, bites) failed: error: could not compile");
  fails(wasmWorld({ over: { "build lab": () => ({ stdout: "" }) } }), "the release build of npt-lab (x86-64, measured) names no bin artifact in cargo's JSON messages");
});

test("the wasm step's calls but its four cargo builds share one 300 s deadline from the first of them, each given what is left; a call stopped by it fails the step naming the call (carry 84 (a); before it U4d2 round 1, FX-7: INF-4)", () => {
  const w = wasmWorld();
  assert.equal(w.ok, true, w.out);
  // The fake clock moves 1,500 ms a call: the i-th call after the builds is given 300,000 - 1,500 i ms.
  const timedCalls = w.calls.filter((c) => !c.kind.startsWith("build "));
  timedCalls.forEach((c, i) => assert.equal(c.limit, 300_000 - 1_500 * i, `${c.kind}: ${c.limit}`));
  assert.equal(w.calls.filter((c) => c.limit === 0).length, 4, "the four cargo builds alone have no limit");
  const stopped = (label, left) =>
    `${label}: still running at the step's deadline (300 s shared by every call but its cargo builds, from the first of them; ${left} s were left for it), so it was stopped with every process it started`;
  // A Wasm leg that hangs: the step fails naming it, within main's 40 lines.
  const hung = wasmWorld({ over: { "wasm ES-s2": () => ({ timedOut: true, stdout: "" }) } });
  assert.equal(hung.ok, false, hung.out);
  assert.ok(tail(hung.out).split("\n").includes(stopped("Wasm ES-s2", "297.0")), tail(hung.out));
});

test("carry 84 (c), the shared deadline with a fake runner and clock: a first call that takes the whole limit is stopped, and the next calls are not started, each named as skipped", () => {
  // `npt-lab hash`, the first call after the builds, runs the whole 300 s and is stopped (it had regenerated the
  // recordings first).
  const w = wasmWorld({ over: { legs: (b) => ({ timedOut: true, ms: 300_000, stdout: "", writes: b.writes }) } });
  assert.equal(w.ok, false, w.out);
  assert.deepEqual(w.kinds, ["build lab", "build lab bites", "build wasm", "build wasm bites", "legs"], "nothing starts after the stopped call");
  const said = w.out.split("\n");
  assert.ok(said.some((l) => l.startsWith("npt-lab hash: still running at the step's deadline") && l.includes("300.0 s were left for it")), w.out);
  for (const label of ["Wasm SS-s1", "Wasm ES-s2", "Wasm TC-s3", "npt-lab bites"]) {
    assert.ok(
      said.includes(`${label}: not started (skipped): npt-lab hash was stopped at the step's deadline (300 s shared by every call but its cargo builds, from the first of them)`),
      `${label}\n${w.out}`,
    );
  }
  // A call that ends exactly at the deadline is not stopped, but the time is spent: the next is skipped as such.
  const spent = wasmWorld({ over: { legs: (b) => ({ ...b, ms: 300_000 }) } });
  assert.equal(spent.ok, false, spent.out);
  assert.ok(spent.out.split("\n").includes("Wasm SS-s1: not started (skipped): the step's deadline had passed (300 s shared by every call but its cargo builds, from the first of them)"), spent.out);
  assert.equal(spent.kinds.filter((k) => !k.startsWith("build ")).length, 1, "only the first call ran");
});

test("the shared deadline on a fractional clock (wasmStep's default is performance.now: advisor-u4r-r1, FP-6 item 11) gives every call a whole limit, as spawnSync takes it", () => {
  // `npt-lab hash` takes 1,500.4 ms, so the time left for every later call is fractional.
  const w = wasmWorld({ over: { legs: (b) => ({ ...b, ms: 1_500.4 }) } });
  assert.equal(w.ok, true, w.out);
  const timedCalls = w.calls.filter((c) => !c.kind.startsWith("build "));
  for (const c of timedCalls) assert.ok(Number.isInteger(c.limit) && c.limit > 0, `${c.kind}: ${c.limit}`);
  assert.equal(timedCalls[1].limit, 298_500, "300,000 - 1,500.4, rounded up");
});

test("carry 84 (b) and (c): run() stops a timed call with every process it started (a child that starts a grandchild and sleeps, limit 200 ms)", () => {
  // A /bin/sh child, which starts far inside the limit (advisor-u4r-r1, FP-6 item 7: T-3; a node child took 203.7-204.0
  // ms in all, so one slow start would fail a check that is never retried).
  const child = "echo child $$; sleep 30 & echo grandchild $!; wait";
  const t0 = Date.now();
  const r = run("/bin/sh", ["-c", child], HERE, process.env, 200);
  const ms = Date.now() - t0;
  assert.equal(r.timedOut, true, `run() reports the timeout: ${JSON.stringify(r)}`);
  assert.ok(ms < 10_000, `run() returned after ${ms} ms, not the child's 30 s`);
  const pids = ["child", "grandchild"].map((k) => Number(r.out.match(new RegExp(`^${k} (\\d+)$`, "m"))?.[1]));
  assert.ok(pids.every((p) => p > 0), `both started and said their pid: ${r.out}`);
  // Left means still running: a process gone, or a zombie (dead, its exit status not yet collected by its new parent),
  // is not left. SIGKILL is delivered at once, but /proc may show the state a moment later.
  const running = (pid) => {
    try {
      return readFileSync(`/proc/${pid}/stat`, "utf8").split(") ")[1][0] !== "Z";
    } catch {
      return false;
    }
  };
  const until = Date.now() + 2_000;
  while (pids.some(running) && Date.now() < until);
  assert.deepEqual(pids.map(running), [false, false], `child ${pids[0]}, grandchild ${pids[1]}`);
});

test("carry 83 (b): the clean half requires exit 0 from npt-lab hash and from its compare, whatever their lines say", () => {
  const legs = wasmWorld({ over: { legs: (b) => ({ ...b, status: 1 }) } });
  assert.equal(legs.ok, false, legs.out);
  assert.ok(tail(legs.out).split("\n").includes("npt-lab hash: exit 1, not 0 (the clean half requires exit 0 as well as its lines)"), tail(legs.out));
  const compare = wasmWorld({ over: { compare: (b) => ({ ...b, status: 1 }) } });
  assert.equal(compare.ok, false, compare.out);
  assert.ok(
    tail(compare.out).split("\n").includes("npt-lab hash compare (native leg 1 against Wasm leg 1): exit 1, not 0 (the clean half requires exit 0 as well as its lines)"),
    tail(compare.out),
  );
  // Their lines alone said equal: only the exit code fails each.
  for (const w of [legs, compare]) assert.equal(w.out.split("\n").filter((l) => /exit 1, not 0/.test(l)).length, 1, w.out);
});

test("wasmStep's two options (the advisor's U4r build ruling, point 1 (4)): a non-default manifest reaches every npt-lab hash call and the loader's recordings; the bites half off starts no bites-build call", () => {
  const DET = "crates/lab/det-recordings.json";
  const recs = [
    { name: "SS-P2-s1", first: 0 },
    { name: "ES-P3-s2", first: 1_000_000 },
  ];
  const w = wasmWorld({ recs, manifestPath: DET, options: { manifest: DET, bitesHalf: false } });
  assert.equal(w.ok, true, w.out);
  const hashCalls = w.calls.filter((c) => c.line.startsWith(`${LAB} hash`) && !c.line.startsWith(`${LAB} hash vectors`));
  assert.deepEqual(hashCalls.map((c) => c.kind), ["legs", "compare"]);
  for (const c of hashCalls) assert.ok(c.line.includes(` --manifest ${DET} --dir ${WS.rec}`) && !c.line.includes(RECORDINGS_MANIFEST), c.line);
  const wasmLegs = w.calls.filter((c) => c.kind.startsWith("wasm "));
  assert.deepEqual(
    wasmLegs.map((c) => c.line),
    recs.map((r) => `node ${LOADER} --wasm ${WASM_M} --rec ${join(WS.rec, `${r.name}.rec`)} --name ${r.name} --first-tick ${r.first} --out ${join(WS.meas, `wasm-${r.name}.lines`)}`),
  );
  assert.ok(w.calls.every((c) => !c.line.includes("--features bites") && !c.line.startsWith(LAB_BITES) && !c.line.includes(WASM_B)), "no bites-build call");
  assert.deepEqual(w.kinds.filter((k) => k.startsWith("build ")), ["build lab", "build wasm"]);
  const last = lastLine(w.out);
  assert.ok(last.includes("native leg 1 against Wasm leg 1: 18000 ticks compared, 0 mismatching, equal (SS-P2-s1 9000, ES-P3-s2 9000)"), last);
  assert.ok(last.includes("hash-map: not run: the bites half is off (wasmStep's bitesHalf option); std-sin: not run: the bites half is off (wasmStep's bitesHalf option)"), last);
  // The defaults: check.mjs's own manifest and the bites half.
  const d = wasmWorld();
  assert.ok(d.calls.some((c) => c.line.startsWith(LAB_BITES)) && d.calls.every((c) => !c.line.includes(DET)));
});

test("the wasm step judges PM-0004-21's two bites: each caught at its tick and field, or not exercised; anything else fails (U4d2, 2.4 (a) item 6)", () => {
  const fails = (world, ...said) => {
    assert.equal(world.ok, false, world.out);
    for (const s of said) assert.ok(tail(world.out).includes(s), `${s}\nnot in:\n${tail(world.out)}`);
  };
  const nwAt = (o) => () => ({
    status: 1,
    stdout:
      `native leg 1 digest 1111111111111111 (${HASH_MAP_FIELD} at words [6488, 6489] of its dump at SS-s1 tick 1234)\n` +
      `Wasm leg 1 digest 3333333333333333 (${HASH_MAP_FIELD} at words [6488, 6489] of its dump at SS-s1 tick 1234)\n` +
      `${cmpLine("native leg 1", "Wasm leg 1", { compared: 9_000, counts: "SS-s1 9000", ...o })}\n`,
  });
  // hash-map: a catch at another tick, at another field, and a miss (digests differ, no tick does): each fails.
  fails(wasmWorld({ over: { "hash-map compare nw": nwAt({ mismatching: 3, rec: "SS-s1", tick: 1235, field: HASH_MAP_FIELD }) } }), "the first mismatch is SS-s1 tick 1235, field bite[0].hash_map_order, not SS-s1 tick 1234");
  fails(wasmWorld({ over: { "hash-map compare nw": nwAt({ mismatching: 3, rec: "SS-s1", tick: 1234, field: "soldier[0].alive" }) } }), "the first mismatch is SS-s1 tick 1234, field soldier[0].alive, not SS-s1 tick 1234");
  fails(wasmWorld({ over: { "hash-map compare nw": nwAt({}) } }), "hash-map, native leg 1 against Wasm leg 1: digests 1111111111111111 and 3333333333333333 differ, but the first mismatch is none");
  // Digests equal, yet a tick differs: fails.
  fails(
    wasmWorld({ over: { "hash-map compare ww": (b) => ({ status: 1, stdout: b.stdout.replace(cmpLine("Wasm leg 1", "Wasm leg 2", { compared: 9_000, counts: "SS-s1 9000" }), cmpLine("Wasm leg 1", "Wasm leg 2", { compared: 9_000, mismatching: 2, rec: "SS-s1", tick: 5000, field: "soldier[1].pos.x", counts: "SS-s1 9000" })) }) } }),
    "hash-map, Wasm leg 1 against Wasm leg 2: digests equal (3333333333333333: not exercised), yet 2 ticks differ",
  );
  // No pair exercised (every pair's digests equal): fails.
  const same = (a, b) => (base) => ({ status: 0, stdout: base.stdout.replace(/digest \w{16}/g, "digest 4444444444444444").replace(/^(.* against .*): .*$/m, `${cmpLine(a, b, { compared: 9_000, counts: "SS-s1 9000" })}`) });
  fails(
    wasmWorld({ over: { "hash-map legs": same("native leg 1", "native leg 2"), "hash-map compare nw": same("native leg 1", "Wasm leg 1"), "hash-map compare ww": same("Wasm leg 1", "Wasm leg 2") } }),
    "hash-map: no pair was exercised (every pair's digests equal)",
  );
  // The native digests equal (not exercised) while native and Wasm differ and are caught: one pair exercised passes.
  const oneExercised = wasmWorld({
    over: {
      "hash-map legs": same("native leg 1", "native leg 2"),
      "hash-map compare nw": (b) => ({ stdout: b.stdout.replace("native leg 1 digest 1111111111111111", "native leg 1 digest 4444444444444444") }),
    },
  });
  assert.equal(oneExercised.ok, true, oneExercised.out);
  assert.ok(lastLine(oneExercised.out).includes("native leg 1 - native leg 2 not exercised (digests equal, 4444444444444444); native leg 1 - Wasm leg 1 caught"));
  // std-sin, armed at the scan's catch: caught at its tick and field passes (the pin test); at another tick, another
  // field, or a miss fails, naming the libc the step ran on beside the scan's.
  const sinAt = (o) => () => ({ status: 1, stdout: `${cmpLine("native leg 1", "Wasm leg 1", { compared: 9_000, counts: "ES-s2 9000", ...o })}\n` });
  const libc = "this step ran on glibc 2.39 (Node's report), the scan on libc: ldd (Ubuntu GLIBC 2.39-0ubuntu8.6) 2.39";
  fails(wasmWorld({ scan: CATCH_SCAN, over: { "std-sin compare": sinAt({ mismatching: 9, rec: "ES-s2", tick: 1_000_124, field: "vehicle[3].pos.y" }) } }), "NOT caught at the scan's ES-s2 tick 1000123, vehicle[3].pos.y: first mismatch ES-s2 tick 1000124", libc);
  fails(wasmWorld({ scan: CATCH_SCAN, over: { "std-sin compare": sinAt({ mismatching: 9, rec: "ES-s2", tick: 1_000_123, field: "vehicle[3].pos.x" }) } }), "field vehicle[3].pos.x", libc);
  fails(wasmWorld({ scan: CATCH_SCAN, over: { "std-sin compare": sinAt({}) } }), "NOT caught at the scan's ES-s2 tick 1000123, vehicle[3].pos.y: first mismatch none", libc);
  // The miss says why a CPU or libc change can cause it (U4d2 round 1, FX-9: INF-3): glibc's sinf is chosen at load.
  fails(
    wasmWorld({ scan: CATCH_SCAN, over: { "std-sin compare": sinAt({}) } }),
    `${libc}; glibc picks its sinf by the CPU's features when it loads (an IFUNC), so on another CPU or libc the scan is re-run there (u4d2-brief.md 2.4 (a) item 6; INF-3)`,
  );
  // The scan data: "not catchable" passes and is printed (the pin test); a file with neither fails.
  fails(wasmWorld({ scan: "scanned on: nothing\n" }), `${STD_SIN_SCAN} records neither a catch on a manifest recording nor "not catchable at this site on x86-64 and wasm32"`);
  assert.deepEqual(readScan(CATCH_SCAN), {
    catches: [
      { recording: "TC-s3", tick: 4_294_962_800, field: "vehicle[0].pos.y" },
      { recording: "ES-s2", tick: 1_000_123, field: "vehicle[3].pos.y" },
    ],
    notCatchable: false,
    record: ["libc: ldd (Ubuntu GLIBC 2.39-0ubuntu8.6) 2.39"],
  });
  // The two ids are named by check.mjs; their indexes come from `npt-lab hash bites`, never a copy of World::BITES.
  assert.deepEqual(WASM_BITES, { hashMap: "PM-0004-21/hash-map", stdSin: "PM-0004-21/std-sin" });
  fails(wasmWorld({ over: { bites: () => ({ stdout: "1 PM-0004-21/hash-map\n" }) } }), "npt-lab hash bites lists no PM-0004-21/std-sin (World::BITES)");
  const reordered = wasmWorld({ over: { bites: () => ({ stdout: "1 PM-0001-41/server-speed-1pct\n2 PM-0004-21/std-sin\n3 PM-0004-21/hash-map\n" }) } });
  assert.ok(reordered.calls.find((c) => c.kind === "hash-map wasm 1").line.includes("--bite 3 --arm 1234"));
});

test("readComparison and builtArtifact read npt-lab's lines and cargo's JSON as they are written (U4d2)", () => {
  const c = readComparison(`x\n${cmpLine("native leg 1", "Wasm leg 1", { mismatching: 4, rec: "SS-s1", tick: 12, field: "soldier[0].pos.x" })}\n`, "native leg 1", "Wasm leg 1");
  assert.deepEqual({ ...c, line: undefined, counts: [...c.counts] }, {
    line: undefined,
    compared: 27_000,
    mismatching: 4,
    equal: false,
    first: { recording: "SS-s1", tick: 12 },
    at: "SS-s1 tick 12 hash 00000000000000aa against SS-s1 tick 12 hash 00000000000000bb; field soldier[0].pos.x (word 9): 1 and 2",
    field: "soldier[0].pos.x",
    counts: [["SS-s1", 9000], ["ES-s2", 9000], ["TC-s3", 9000]],
  });
  assert.equal(readComparison("nothing\n", "a", "b"), null);
  const end = readComparison("a against b: 9 ticks compared, 1 mismatching; first mismatch at entry 8: SS-s1 tick 8 hash 01 against its end; the lists step other ticks or one ends (SS-s1 9)\n", "a", "b");
  assert.deepEqual([end.first, end.field, end.mismatching], [null, null, 1]);
  const json = `${JSON.stringify({ reason: "compiler-message" })}\n${JSON.stringify({ reason: "compiler-artifact", target: { name: "npt_lab", kind: ["lib"] }, filenames: ["/l.rlib"] })}\n${JSON.stringify({ reason: "compiler-artifact", target: { name: "npt-lab", kind: ["bin"] }, executable: "/t/npt-lab" })}\n`;
  assert.equal(builtArtifact(json, { target: "npt-lab", kind: "bin" }), "/t/npt-lab");
  assert.equal(builtArtifact(json, { target: "npt_hash_wasm", kind: "cdylib" }), null);
});

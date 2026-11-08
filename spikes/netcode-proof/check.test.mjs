// Tests for check.mjs's rule checks: each dependency rule of plan §4.2 is broken once on a synthetic graph and must
// be caught; the clean graph must pass. Run: node --test spikes/netcode-proof/check.test.mjs

import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { test } from "node:test";
import { fileURLToPath } from "node:url";
import {
  WASM_CRATES,
  cargoConfigProblems,
  crateFiles,
  dependencyProblems,
  expectedMarks,
  layoutProblems,
  lexRust,
  marksProblems,
  referenceProblems,
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
    "crates/tcpinfo/Cargo.toml": '[package]\nname = "npt-tcpinfo"\n\n[lints]\nworkspace = true\n',
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

test("the workspace manifest denies unsafe_code, and nothing else will do (review F-D)", () => {
  const real = readFileSync(join(HERE, "Cargo.toml"), "utf8");
  assert.deepEqual(workspaceLintProblems(real), [], "the spike's own Cargo.toml");
  const none = 'Cargo.toml has no [workspace.lints.rust] unsafe_code = "deny" (or "forbid") for its crates to inherit';
  const line = real.split("\n").findIndex((l) => l.startsWith("unsafe_code")) + 1;
  const other = (n) => `Cargo.toml:${n} names unsafe_code other than as "deny" or "forbid" in [workspace.lints.rust]`;
  for (const level of ['"allow"', '"warn"', "'allow'", '{ level = "allow", priority = 1 }']) {
    assert.deepEqual(workspaceLintProblems(real.replace('unsafe_code = "deny"', `unsafe_code = ${level}`)), [other(line), none], level);
  }
  assert.deepEqual(workspaceLintProblems(real.replace('unsafe_code = "deny"', "")), [none], "the line removed");
  for (const ok of ['"forbid"', "'deny'", '{ level = "deny", priority = -1 }']) {
    assert.deepEqual(workspaceLintProblems(real.replace('unsafe_code = "deny"', `unsafe_code = ${ok}`)), [], ok);
  }
  const t = (body) => `[workspace]\nmembers = []\n${body}`;
  assert.deepEqual(workspaceLintProblems(t('[workspace.lints.rust]\n# unsafe_code = "allow" in a comment\nunsafe_code = "deny"\n')), []);
  assert.deepEqual(workspaceLintProblems(t('[workspace.lints.clippy]\nunsafe_code = "deny"\n')), [other(4), none], "another table");
  assert.deepEqual(workspaceLintProblems(t('[workspace.lints]\nrust.unsafe_code = "deny"\n')), [other(4), none], "a dotted key (read closed)");
  // A copy of the right lines inside a multi-line string, the real one relaxed below it.
  const decoy = t('[workspace.metadata]\nnote = """\n[workspace.lints.rust]\nunsafe_code = "deny"\n"""\n[workspace.lints.rust]\nunsafe_code = "allow"\n');
  assert.ok(workspaceLintProblems(decoy).includes(other(9)), workspaceLintProblems(decoy).join("\n"));
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

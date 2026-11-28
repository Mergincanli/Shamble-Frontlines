// The security seat's round-2 probe of FX-3 and FX-6 (u4d2-r2-review-security.md): the copy's unsafeProblems on the
// real crate files with npt-hash-wasm's lib.rs (or npt-tcpinfo's, or a manifest) swapped, or a file added, in memory
// only. Each case prints its problem count and lines, and whether that matches what the case expects. Writes nothing.
// Run from the copy's spikes/netcode-proof.
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { pathToFileURL } from "node:url";

const here = process.cwd();
const { unsafeProblems, crateFiles } = await import(pathToFileURL(join(here, "check.mjs")).href);
const LIB = "crates/hash-wasm/src/lib.rs";
const TCP = "crates/tcpinfo/src/lib.rs";
const real = crateFiles(join(here, "crates")).files;
const disk = (f) => readFileSync(join(here, f), "utf8");
const lib = disk(LIB);
const BEGIN_BODY = "    let Some(replayer) = replayer_for(bite, arm_tick) else {\n";
if (!lib.includes(BEGIN_BODY)) throw new Error("anchor not found in begin's body");
const inBegin = (line) => lib.replace(BEGIN_BODY, `${line}\n${BEGIN_BODY}`);
const beginLine = lib.slice(0, lib.indexOf(BEGIN_BODY)).split("\n").length;
let unexpected = 0;

function run(label, expect, { swap = {}, extra = {} } = {}) {
  const files = [...new Set([...real, ...Object.keys(extra)])].sort();
  const read = (f) => (Object.hasOwn(swap, f) ? swap[f] : Object.hasOwn(extra, f) ? extra[f] : disk(f));
  const p = unsafeProblems(files, read);
  const ok = expect === "refused" ? p.length > 0 : p.length === 0;
  if (!ok) unexpected++;
  console.log(`--- ${label}: ${p.length} problem(s) [expected ${expect}: ${ok ? "as expected" : "UNEXPECTED"}]`);
  for (const l of p) console.log(`    ${l.slice(0, 240)}`);
}

const buildRs =
  'fn main() {\n    let out = std::env::var("OUT_DIR").unwrap();\n' +
  '    let code = "{ let _ = \\x75nsafe { core::ptr::read_volatile(&0u32) }; }";\n' +
  '    std::fs::write(format!("{out}/gen.rs"), code).unwrap();\n}\n';
const splitter = 'macro_rules! m {\n    ($($t:tt)*) => {\n        $($t)* ("../../../gen.rs")\n    };\n}\n';

run("R0 the real files", "passed");
// FX-3: round 1's V1, and the same block in dump_word's body.
run(`FX-3 V1 an unsafe block in begin's body (expected at lib.rs:${beginLine})`, "refused", {
  swap: { [LIB]: inBegin("    let _ = unsafe { core::ptr::read_volatile(&bite) };") },
});
const DW = 'pub extern "C" fn dump_word(i: u32) -> u32 {\n';
if (lib.includes(DW)) run("FX-3 an unsafe block in dump_word's body", "refused", { swap: { [LIB]: lib.replace(DW, `${DW}    let _ = unsafe { core::ptr::read_volatile(&i) };\n`) } });
// FX-6: round 1's V4 whole, then each half.
run("FX-6 V4 whole: build.rs + `use std::include as inc;` + inc! in begin's body", "refused", {
  swap: { [LIB]: `use std::include as inc;\n${inBegin('    inc!(concat!(env!("OUT_DIR"), "/gen.rs"));')}` },
  extra: { "crates/hash-wasm/build.rs": buildRs },
});
run("FX-6 V4's first half: build.rs alone", "refused", { extra: { "crates/hash-wasm/build.rs": buildRs } });
run("FX-6 V4's second half: the rename alone", "refused", { swap: { [LIB]: `use std::include as inc;\n${lib}` } });
// Near forms the rule should refuse.
run("near: `use std::{include as inc};`", "refused", { swap: { [LIB]: `use std::{include as inc};\n${lib}` } });
run("near: `use core::include as inc;` in npt-tcpinfo", "refused", { swap: { [TCP]: `${disk(TCP)}use core::include as inc;\n` } });
run("near: m!(include) (the name as a macro argument)", "refused", {
  swap: { [LIB]: `macro_rules! m {\n    ($i:ident) => {\n        $i!("../../../gen.rs")\n    };\n}\n${inBegin("    m!(include);")}` },
});
run("near: `std::include!(\"../../gen.rs\")`", "refused", { swap: { [LIB]: inBegin('    std::include!("../../gen.rs");') } });
run("near: `r#include!(\"../../gen.rs\")`", "refused", { swap: { [LIB]: inBegin('    r#include!("../../gen.rs");') } });
run("near: `include!(r\"../../gen.rs\")` (a raw string)", "refused", { swap: { [LIB]: inBegin('    include!(r"../../gen.rs");') } });
run("near: Cargo.toml `\"build\" = \"gen.rs\"` (a quoted key)", "refused", {
  swap: { "crates/hash-wasm/Cargo.toml": disk("crates/hash-wasm/Cargo.toml").replace("[package]\n", '[package]\n"build" = "gen.rs"\n') },
});
run("near: Cargo.toml `package = { ..., build = ... }`-style inline key in npt-tcpinfo", "refused", {
  swap: { "crates/tcpinfo/Cargo.toml": `${disk("crates/tcpinfo/Cargo.toml")}\n[badges]\nx = { build = "y" }\n` },
});
// Near forms the rule lets through (each with no build script).
run("LETS THROUGH? `include!` handed to a macro that adds the path: m!(include!) in begin's body", "passed", {
  swap: { [LIB]: `${splitter}${inBegin("    m!(include!);")}` },
});
run("LETS THROUGH? the same with a space: m!(include !)", "passed", { swap: { [LIB]: `${splitter}${inBegin("    m!(include !);")}` } });
run("LETS THROUGH? the same in npt-tcpinfo (a crate-level item)", "passed", { swap: { [TCP]: `${disk(TCP)}${splitter}m!(include!);\n` } });
run("LETS THROUGH? an escaped literal: include!(\"\\x2e\\x2e/\\x2e\\x2e/\\x2e\\x2e/gen.rs\") (C2R-2, review's)", "passed", {
  swap: { [LIB]: inBegin('    include!("\\x2e\\x2e/\\x2e\\x2e/\\x2e\\x2e/gen.rs");') },
});
run("LETS THROUGH? an escaped #[path] (C2R-2, review's)", "passed", { swap: { [LIB]: `${lib}#[path = "\\x2e\\x2e/\\x2e\\x2e/\\x2e\\x2e/gen.rs"]\nmod g;\n` } });
console.log(`=== ${unexpected ? `${unexpected} UNEXPECTED` : "every case as expected"}`);
process.exitCode = unexpected ? 1 : 0;

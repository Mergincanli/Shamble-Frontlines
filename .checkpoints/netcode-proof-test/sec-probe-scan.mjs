// The security seat's read-only probe of U4d2's unsafe and signature scan (u4d2-review-security.md). It imports the
// copy's check.mjs from the working folder (the runner's: the copy's spikes/netcode-proof), reads the real crate files,
// swaps npt-hash-wasm's lib.rs (or adds a file) in memory only, and prints unsafeProblems' answer for each variant.
// It writes nothing.
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { pathToFileURL } from "node:url";

const here = process.cwd();
const { unsafeProblems, crateFiles, buildFlagProblems, rustflagSources } = await import(pathToFileURL(join(here, "check.mjs")).href);
const LIB = "crates/hash-wasm/src/lib.rs";
const real = crateFiles(join(here, "crates")).files;
const disk = (f) => readFileSync(join(here, f), "utf8");
const lib = disk(LIB);

const BEGIN_BODY = "    let Some(replayer) = replayer_for(bite, arm_tick) else {\n";
const FEED_FN = 'pub extern "C" fn feed(word: u32) {\n';

function run(label, { libText = lib, extra = {} } = {}) {
  const files = [...new Set([...real, ...Object.keys(extra)])].sort();
  const read = (f) => (f === LIB ? libText : Object.hasOwn(extra, f) ? extra[f] : disk(f));
  const p = unsafeProblems(files, read);
  console.log(`--- ${label}: ${p.length} problem(s)`);
  for (const l of p) console.log(`    ${l}`);
}

const must = (text, find) => {
  if (!text.includes(find)) throw new Error(`anchor not found: ${find}`);
  return text;
};

run("V0 the real crate files");
// V1: an unsafe block inside a listed export's body, where the export's own allow lets rustc accept it.
run("V1 unsafe block inside begin's body", {
  libText: must(lib, BEGIN_BODY).replace(BEGIN_BODY, `    let _ = unsafe { core::ptr::read_volatile(&bite) };\n${BEGIN_BODY}`),
});
// V2: the signature scan reads the token, not the type: a module-local rename of f64 as u32.
run("V2 `use core::primitive::f64 as u32;` then feed(word: u32) in a module", {
  libText: `${lib}\nmod shadow {\n    use core::primitive::f64 as u32;\n    pub fn unused(_x: u32) {}\n}\n`.replace(
    must(lib, FEED_FN) && FEED_FN,
    FEED_FN,
  ),
});
run("V2b the same, the export itself inside the shadowing module", {
  libText: lib.replace(
    /#\[allow\(\n    unsafe_code,\n    reason = "[^"]*"\n\)\]\n#\[unsafe\(no_mangle\)\]\npub extern "C" fn feed\(word: u32\) \{\n[\s\S]*?\n\}\n/,
    (m) => `mod shadow {\n    use core::primitive::f64 as u32;\n${m.replace(/^/gm, "    ")}}\n`,
  ),
});
run("V3 `type u32 = f64;` at the crate root", { libText: `${lib}\n#[allow(non_camel_case_types)]\ntype u32 = f64;\n` });
// V4: pathProblems' known limit C2R-2 inside npt-hash-wasm: include! renamed, a build.rs that writes code from escapes.
const buildRs =
  'fn main() {\n    let out = std::env::var("OUT_DIR").unwrap();\n' +
  '    let code = "{ let _ = \\x75nsafe { core::ptr::read_volatile(&0u32) }; }";\n' +
  '    std::fs::write(format!("{out}/gen.rs"), code).unwrap();\n}\n';
run("V4 build.rs writing \\x75nsafe code, begin's body `inc!(concat!(env!(\"OUT_DIR\"), \"/gen.rs\"))`", {
  libText: `use std::include as inc;\n${must(lib, BEGIN_BODY).replace(BEGIN_BODY, `    inc!(concat!(env!("OUT_DIR"), "/gen.rs"));\n${BEGIN_BODY}`)}`,
  extra: { "crates/hash-wasm/build.rs": buildRs },
});
run("V4b include! by its own name with a computed path (the scan's rule)", {
  libText: must(lib, BEGIN_BODY).replace(BEGIN_BODY, `    include!(concat!(env!("OUT_DIR"), "/gen.rs"));\n${BEGIN_BODY}`),
});
// V5: the C2R-1 shebang desync, trying to hide an unsafe block inside an export body.
const shebang = `#!x"\n${lib}`;
run("V5 a shebang line holding one quote, nothing else", { libText: shebang });
// V6-V9: forms the scan should refuse.
run("V6 cfg_attr(all(), unsafe(no_mangle)) on begin", {
  libText: lib.replace('#[unsafe(no_mangle)]\npub extern "C" fn begin', '#[cfg_attr(all(), unsafe(no_mangle))]\npub extern "C" fn begin'),
});
run("V7 #[unsafe(\\n    no_mangle\\n)] over lines (a lawful form)", {
  libText: lib.replace('#[unsafe(no_mangle)]\npub extern "C" fn begin', '#[unsafe(\n    no_mangle\n)]\npub extern "C" fn begin'),
});
run("V8 macro_rules body holding unsafe", { libText: `${lib}\nmacro_rules! m {\n    () => {\n        unsafe {}\n    };\n}\n` });
run("V9 #[unsafe(no_mangle)] pub static begin", {
  libText: `${lib}\n#[allow(unsafe_code, reason = "x")]\n#[unsafe(no_mangle)]\npub static hash_lo2: u32 = 0;\n`,
});
run("V10 a second listed export in a module (same name)", {
  libText: `${lib}\nmod again {\n    #[allow(unsafe_code, reason = "x")]\n    #[unsafe(no_mangle)]\n    pub extern "C" fn step() -> u32 {\n        0\n    }\n}\n`,
});
run("V11 `-> !` on dump_word", {
  libText: lib.replace('pub extern "C" fn dump_word(i: u32) -> u32 {', 'pub extern "C" fn dump_word(i: u32) -> ! {'),
});
run("V12 a multi-line signature, f32 on the second line", {
  libText: lib.replace('pub extern "C" fn dump_word(i: u32) -> u32 {', 'pub extern "C" fn dump_word(\n    i: f32,\n) -> u32 {'),
});
run("V13 a raw-identifier parameter type `r#u32`? (not Rust: just the token reader)", {
  libText: lib.replace('pub extern "C" fn dump_word(i: u32) -> u32 {', 'pub extern "C" fn dump_word(i: core::primitive::u32) -> u32 {'),
});

// The flag scan's reach: which flags the name rule reads (the read-back is separate).
const flagCases = {
  "-C llvm-args=-enable-unsafe-fp-math": ["-C", "llvm-args=-enable-unsafe-fp-math"],
  "--cfg feature=\"bites\"": ["--cfg", 'feature="bites"'],
  "-C overflow-checks=off": ["-C", "overflow-checks=off"],
  "-C target-cpu=haswell": ["-C", "target-cpu=haswell"],
  "-Ctarget-feature=+avx512f": ["-Ctarget-feature=+avx512f"],
  "-Ctarget_feature=+fma": ["-Ctarget_feature=+fma"],
  "--codegen=target-feature=+sse2,+fma": ["--codegen=target-feature=+sse2,+fma"],
};
console.log("--- buildFlagProblems per flag (empty: the name rule passes it)");
for (const [label, flags] of Object.entries(flagCases)) {
  const p = buildFlagProblems([{ where: "probe", flags }]);
  console.log(`    ${label}: ${p.length ? p.join(" | ") : "(passes the name rule)"}`);
}
const env = { CARGO_BUILD_TARGET: "i686-unknown-linux-gnu", CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER: "/bin/true" };
console.log(`--- rustflagSources of env with CARGO_BUILD_TARGET and a linker: ${JSON.stringify(rustflagSources(env, []))}`);

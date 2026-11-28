// The security seat's round-2 probe of FX-1 (u4d2-r2-review-security.md): the copy's buildInputProblems (with
// buildFlagProblems beside it) on every rustflags form round 1 probed (r1-sec-scan.log's name-rule cases), in every
// environment and config source form, and the build-target forms. In memory only: it writes nothing. Run from the
// copy's spikes/netcode-proof (the runner's folder).
import { join } from "node:path";
import { pathToFileURL } from "node:url";

const { buildInputProblems, buildFlagProblems, rustflagSources, cargoConfigProblems } = await import(
  pathToFileURL(join(process.cwd(), "check.mjs")).href
);

let bad = 0;
const show = (label, env, configs, expectRefused) => {
  const p = buildInputProblems(env, configs);
  const names = buildFlagProblems(rustflagSources(env, configs));
  const other = cargoConfigProblems(env, configs);
  const ok = expectRefused ? p.length > 0 : p.length === 0;
  if (!ok) bad++;
  console.log(`${ok ? "as expected" : "UNEXPECTED"}  ${label}: buildInputProblems ${p.length}; name rule ${names.length}; cargoConfigProblems ${other.length}`);
  for (const l of p) console.log(`    FX-1: ${l.slice(0, 170)}`);
};

// r1-sec-scan.log's name-rule cases, as round 1 gave them.
const cases = {
  "-C llvm-args=-enable-unsafe-fp-math": ["-C", "llvm-args=-enable-unsafe-fp-math"],
  '--cfg feature="bites"': ["--cfg", 'feature="bites"'],
  "-C overflow-checks=off": ["-C", "overflow-checks=off"],
  "-C target-cpu=haswell": ["-C", "target-cpu=haswell"],
  "-Ctarget-feature=+avx512f": ["-Ctarget-feature=+avx512f"],
  "-Ctarget_feature=+fma": ["-Ctarget_feature=+fma"],
  "--codegen=target-feature=+sse2,+fma": ["--codegen=target-feature=+sse2,+fma"],
  "-C link-arg=--no-check-features": ["-C", "link-arg=--no-check-features"],
  "-C linker=/bin/true": ["-C", "linker=/bin/true"],
};
const envNames = [
  "RUSTFLAGS",
  "CARGO_ENCODED_RUSTFLAGS",
  "CARGO_BUILD_RUSTFLAGS",
  "CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS",
  "CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUSTFLAGS",
  "CARGO_PROFILE_RELEASE_RUSTFLAGS",
];
console.log("--- each round-1 flag in each environment source");
for (const [label, flags] of Object.entries(cases)) {
  for (const k of envNames) {
    const v = k === "CARGO_ENCODED_RUSTFLAGS" ? flags.join("\x1f") : flags.join(" ");
    show(`${k}=${label}`, { [k]: v }, [], true);
  }
}
console.log("--- each round-1 flag in each config form");
const q = (flags) => `[${flags.map((f) => JSON.stringify(f)).join(", ")}]`;
for (const [label, flags] of Object.entries(cases)) {
  const forms = {
    "[build] array": `[build]\nrustflags = ${q(flags)}\n`,
    "[build] string": `[build]\nrustflags = ${JSON.stringify(flags.join(" "))}\n`,
    "[target.<triple>]": `[target.wasm32-unknown-unknown]\nrustflags = ${q(flags)}\n`,
    "[target.'cfg(...)']": `[target.'cfg(target_arch = "wasm32")']\nrustflags = ${q(flags)}\n`,
    "dotted build.rustflags": `build.rustflags = ${q(flags)}\n`,
    "inline table": `build = { rustflags = ${q(flags)} }\n`,
    'quoted "rustflags"': `[build]\n"rustflags" = ${q(flags)}\n`,
    "escaped quoted key": `[build]\n"rust\\u0066lags" = ${q(flags)}\n`,
    "literal 'rustflags'": `[build]\n'rustflags' = ${q(flags)}\n`,
    "[profile.release] rustflags": `[profile.release]\nrustflags = ${q(flags)}\n`,
  };
  for (const [form, text] of Object.entries(forms)) show(`config ${form}: ${label}`, {}, [{ path: "c.toml", text }], true);
}
console.log("--- RR-M2's pair (r1-rr-superset.log's first environment)");
show("pair", { CARGO_ENCODED_RUSTFLAGS: "-Ctarget-cpu=bleeding-edge", RUSTFLAGS: "-Ctarget-feature=-simd128,-relaxed-simd" }, [], true);
console.log("--- the build target");
for (const v of ["x86_64-unknown-linux-gnu", "wasm32-unknown-unknown", "", " "]) show(`CARGO_BUILD_TARGET=${JSON.stringify(v)}`, { CARGO_BUILD_TARGET: v }, [], true);
const targetForms = {
  "[build] target": '[build]\ntarget = "wasm32-unknown-unknown"\n',
  "[build] target array": '[build]\ntarget = ["x86_64-unknown-linux-gnu", "wasm32-unknown-unknown"]\n',
  "dotted build.target": 'build.target = "i686-unknown-linux-gnu"\n',
  "inline table": 'build = { target = "x" }\n',
  'quoted "target"': '[build]\n"target" = "x"\n',
  "literal 'target'": "[build]\n'target' = 'x'\n",
  "indented, no spaces": '[build]\n    target="x"\n',
  "escaped quoted key": '[build]\n"t\\u0061rget" = "x"\n',
  "a target path (a .json spec)": '[build]\ntarget = "my-spec.json"\n',
};
for (const [form, text] of Object.entries(targetForms)) show(`config ${form}`, {}, [{ path: "c.toml", text }], true);
console.log("--- clean: nothing refused");
show("an empty environment", {}, [], false);
show("this container's variables, empty RUSTFLAGS and CARGO_ENCODED_RUSTFLAGS", { PATH: "/usr/bin", HOME: "/root", RUSTUP_HOME: "/root/.rustup", CARGO_HTTP_CAINFO: "/x", RUST_BACKTRACE: "1", RUSTFLAGS: "", CARGO_ENCODED_RUSTFLAGS: "" }, [], false);
show("RUSTFLAGS of spaces only", { RUSTFLAGS: "   " }, [], false);
show("config target-dir and a [target.<triple>] linker", {}, [{ path: "c.toml", text: '[build]\ntarget-dir = "t"\n[target.x86_64-unknown-linux-gnu]\nlinker = "cc"\n' }], false);
show("config rustflags = []", {}, [{ path: "c.toml", text: "[build]\nrustflags = []\n" }], false);
// Values JavaScript's \s splits away that cargo (split(' ') then Rust's trim) would hand rustc as an argument.
console.log("--- whitespace edges (JS \\s against cargo's split(' ') and Rust's trim)");
for (const [label, v] of [["U+FEFF", "﻿"], ["U+00A0", " "], ["U+0085", "\u0085"], ["tab", "\t"]]) {
  const p = buildInputProblems({ RUSTFLAGS: v }, []);
  console.log(`    RUSTFLAGS=${label}: buildInputProblems ${p.length}`);
}
console.log(`--- ${bad ? `${bad} UNEXPECTED` : "every case as expected"}`);
process.exitCode = bad ? 1 : 0;

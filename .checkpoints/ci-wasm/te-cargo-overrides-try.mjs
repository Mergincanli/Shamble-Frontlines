// Scratch (test-engineer, CI-WASM followups, block 3): the proposed cases for the build's refusal of cargo settings,
// run against the pattern as build.mjs:44-47 has it today, and against five mutants that each case must catch.
// Reads the pattern's text from build.mjs itself, so a later edit there shows here. No environment value is read.
import { readFileSync } from "node:fs";

const source = readFileSync(new URL("../../packages/client/scripts/build.mjs", import.meta.url), "utf8");
const m = /const CARGO_OVERRIDES = new RegExp\(\s*"([^"]*)" \+\s*"([^"]*)",?\s*\);/.exec(source);
if (m === null) throw new Error("pattern not found in build.mjs");
const text = m[1] + m[2];

const refused = [
  "RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS", "RUSTC", "RUSTC_WRAPPER", "RUSTC_WORKSPACE_WRAPPER", "RUSTUP_TOOLCHAIN",
  "CARGO_BUILD_RUSTFLAGS", "CARGO_BUILD_TARGET", "CARGO_PROFILE_RELEASE_OVERFLOW_CHECKS", "CARGO_PROFILE_RELEASE_OPT_LEVEL",
  "CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUSTFLAGS", "CARGO_TARGET_DIR_X", "CARGO_BUILD_TARGET_DIRX", "WASM_BINDGEN_KEEP_DEBUG",
  "RUST_LOG",
];
const allowed = [
  "CARGO_TARGET_DIR", "CARGO_BUILD_TARGET_DIR", "CARGO_HOME", "RUSTUP_HOME", "RUSTUP_AUTO_INSTALL", "HOME", "PATH", "USER",
  "CI", "LANG", "PROBE_IP", "npm_config_update_notifier", "SHAMBLE_WASM_BINDGEN", "RUSTFLAGS_X", "XRUSTFLAGS",
];
const env = Object.fromEntries([...refused, ...allowed].map((name) => [name, "1"]));
const run = (pattern) => Object.keys(env).filter((name) => pattern.test(name));
const same = (a, b) => JSON.stringify(a) === JSON.stringify(b);

const today = run(new RegExp(text));
console.log(`pattern today: ${same(today, refused) ? "PASS" : "FAIL"} (${today.length} refused of ${refused.length + allowed.length})`);
if (!same(today, refused)) console.log(`  got: ${today.join(", ")}`);

const mutants = {
  "no negative lookahead": text.replace("(?!CARGO_(BUILD_)?TARGET_DIR$)", ""),
  "lookahead without $": text.replace("TARGET_DIR$)", "TARGET_DIR)"),
  "no CARGO_PROFILE family": text.replace("CARGO_(BUILD|PROFILE|TARGET)_.*", "CARGO_(BUILD|TARGET)_.*"),
  "no ^ anchor": text.replace(/^\^/, ""),
  "no final $ anchor": text.replace(/\$$/, ""),
  "no RUSTC_WORKSPACE_WRAPPER": text.replace("RUSTC_(WORKSPACE_)?WRAPPER", "RUSTC_WRAPPER"),
  "no WASM_BINDGEN family": text.replace("|WASM_BINDGEN_.*", ""),
};
for (const [name, mutant] of Object.entries(mutants)) {
  if (mutant === text) {
    console.log(`mutant "${name}": NOT APPLIED (the text did not change)`);
    continue;
  }
  console.log(`mutant "${name}": ${same(run(new RegExp(mutant)), refused) ? "SURVIVED (the cases miss it)" : "caught"}`);
}

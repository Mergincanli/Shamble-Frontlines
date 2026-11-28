// security-auditor's U4r probe (read only): imports check.mjs's pure scan functions from the read copy given as
// argv[2] (main does not run on import: check.mjs's last lines), and reports what unsafeProblems and
// cargoConfigProblems say of a few planted texts held in memory, then what cargoConfigProblems and buildInputProblems
// say of this container's own environment and the cargo configs cargo would read from the spike. Writes nothing; runs
// no cargo.
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { pathToFileURL } from "node:url";

const spike = process.argv[2];
const check = await import(pathToFileURL(join(spike, "check.mjs")).href);
const { files } = check.crateFiles(join(spike, "crates"));
const disk = (f) => readFileSync(join(spike, f), "utf8");
const LIB = "crates/hash-wasm/src/lib.rs";
const scan = (extra) => check.unsafeProblems(files, (f) => (f === LIB ? disk(f) + extra : disk(f)));

const base = scan("");
console.log(`baseline: ${base.length} problems`);
const forms = {
  "#[$a] (the carry's form)": 'macro_rules! m { ($a:meta) => { #[$a] mod g; }; }\nm!(path = "../../../gen.rs");\n',
  "#[cfg_attr(all(), $a)]": 'macro_rules! m { ($a:meta) => { #[cfg_attr(all(), $a)] mod g; }; }\nm!(path = "../../../gen.rs");\n',
  "#[cfg_attr(all(), $($t)*)]": 'macro_rules! m { ($($t:tt)*) => { #[cfg_attr(all(), $($t)*)] mod g; }; }\nm!(path = "../../../gen.rs");\n',
  "plain #[path] (control)": '#[path = "../../../gen.rs"] mod g;\n',
};
for (const [name, text] of Object.entries(forms)) {
  const p = scan(text).filter((x) => !base.includes(x));
  console.log(`${name}: ${p.length} new problem(s)${p.length ? `: ${p.join(" | ")}` : ""}`);
}

const cfg = (text) => check.cargoConfigProblems({ PATH: "/bin" }, [{ path: "c.toml", text }]);
for (const text of [
  "[profile.release.package.npt-sim]\noverflow-checks = false\n",
  "[ profile . release ]\noverflow-checks = false\n",
  "['profile'.release]\noverflow-checks = false\n",
  '["\\u0070rofile".release]\noverflow-checks = false\n',
  "[target.wasm32-unknown-unknown]\n'linker' = \"x\"\n",
]) {
  console.log(`config ${JSON.stringify(text)}: ${JSON.stringify(cfg(text))}`);
}

// This container: the environment as it is, and the config files cargo would read from the spike.
const configs = [];
for (const dir of new Set(check.cargoConfigDirs(spike, process.env))) {
  for (const name of ["config.toml", "config"]) {
    const path = join(dir, name);
    if (existsSync(path)) configs.push({ path, text: readFileSync(path, "utf8") });
  }
}
console.log(`container: ${configs.length} cargo config file(s): ${configs.map((c) => c.path).join(", ") || "none"}`);
console.log(`container cargoConfigProblems: ${JSON.stringify(check.cargoConfigProblems(process.env, configs))}`);
console.log(`container buildInputProblems: ${JSON.stringify(check.buildInputProblems(process.env, configs))}`);

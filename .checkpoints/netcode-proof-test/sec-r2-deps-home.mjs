// The security seat's round-2 probe (u4d2-r2-review-security.md): the copy's depsStep, as STEPS.deps runs it, with
// one change to the environment object it reads its cargo configs from: CARGO_HOME pointed at a probe folder holding
// one config.toml (sec-r2-home/<case>/), so cargoConfigFiles reads a real file. cargo metadata, cargo tree and rustc
// still run with this process's own environment (depsStep's metadata and runner use run's default env). It prints the
// step's verdict and each problem line. Writes nothing. Run from the copy's spikes/netcode-proof.
import { join } from "node:path";
import { pathToFileURL } from "node:url";

const { depsStep } = await import(pathToFileURL(join(process.cwd(), "check.mjs")).href);
for (const dir of process.argv.slice(2)) {
  const r = depsStep(process.cwd(), { ...process.env, CARGO_HOME: dir });
  const counted = r.out.match(/\((\d+) crate files and (\d+) cargo config file\(s\) read/);
  console.log(`--- CARGO_HOME=${dir}: deps ${r.ok ? "PASS" : "FAIL"}${counted ? ` (${counted[2]} config file(s) read)` : ""}`);
  if (!r.ok) for (const l of r.out.split("\n")) console.log(`    ${l.slice(0, 220)}`);
}

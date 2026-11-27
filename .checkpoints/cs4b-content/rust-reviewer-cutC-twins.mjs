// rust-reviewer, cut C: the build's own twin rules on the scratch twins, with the allow-list as it is and with the
// two proposed entries. Usage: node rust-reviewer-cutC-twins.mjs <build-checks.mjs> <release.wasm> <debug.wasm>
import { readFileSync } from "node:fs";
import { pathToFileURL } from "node:url";
const [checks, release, debug] = process.argv.slice(2);
const { twinProblems } = await import(pathToFileURL(checks).href);
const proposed = [{ routine: "C::round", why: "-" }, { routine: "C::__multi3", why: "-" }];
for (const [label, allowed] of [["today's list", []], ["with the two entries", proposed]]) {
  const lines = [...twinProblems(readFileSync(release), "the release twin", allowed), ...twinProblems(readFileSync(debug), "the dev-profile twin", allowed)];
  console.log(`${label}: ${lines.length} lines`); for (const line of lines) console.log(`  ${line}`);
}

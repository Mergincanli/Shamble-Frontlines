// Scratch (test-engineer, CI-WASM followups, block 4): does the proposed type check of the browser specs bite?
// Works on a copy of packages/e2e under .gate/ci-wasm/te-types-plant/ (never on the repository's files): for each
// plant it edits one file of the copy, runs tsc with the proposed flags, prints whether tsc failed and on which line,
// then puts the file back. The clean copy must pass first.
import { execFileSync } from "node:child_process";
import { readdirSync, readFileSync, writeFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const repo = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../..");
const copy = path.join(repo, ".gate/ci-wasm/te-types-plant");
const shim = path.join(repo, ".checkpoints/ci-wasm/te-node-shim.d.ts");
const flags = [
  "--ignoreConfig", "--noEmit", "--strict", "--noUncheckedIndexedAccess", "--exactOptionalPropertyTypes", "--noImplicitOverride",
  "--noFallthroughCasesInSwitch", "--noImplicitReturns", "--isolatedModules", "--verbatimModuleSyntax", "--erasableSyntaxOnly",
  "--allowImportingTsExtensions", "--module", "preserve", "--target", "es2023", "--lib", "es2023,dom,dom.iterable", "--skipLibCheck",
];
const specs = (dir) =>
  readdirSync(dir, { withFileTypes: true }).flatMap((e) => (e.isDirectory() ? specs(path.join(dir, e.name)) : e.name.endsWith(".ts") ? [path.join(dir, e.name)] : []));
const files = [shim, path.join(copy, "playwright.config.ts"), ...specs(path.join(copy, "tests")).sort()];

function tsc() {
  try {
    execFileSync(process.execPath, [path.join(repo, "node_modules/typescript/bin/tsc"), ...flags, ...files], { cwd: repo, encoding: "utf8" });
    return { failed: false, lines: [] };
  } catch (error) {
    const out = String(error.stdout ?? "");
    return { failed: true, lines: out.split("\n").filter((l) => l.includes("error TS")).map((l) => l.replace(copy + "/", "")) };
  }
}

const plants = [
  ["a misspelt field read with a default, which passes at run time (undefined ?? 0 is 0)", "tests/skeleton/skeleton.spec.ts",
    "expect(data.worker?.undetached).toBe(0);", "expect(data.worker?.undetachd ?? 0).toBe(0);"],
  ["the report's field changes type in report.ts", "tests/skeleton/report.ts", "  blocks: number;", "  blocks: string;"],
  ["a wrong option type in the CDP call", "tests/skeleton/allocation.spec.ts", "samplingInterval: 64,", 'samplingInterval: "64",'],
  ["a wrong value in the config", "playwright.config.ts", "retries: 0,", 'retries: "0",'],
  ["a page-side name that does not exist", "tests/skeleton/isolation.spec.ts", "() => crossOriginIsolated", "() => crossOriginIsolatd"],
  ["an import that no longer exists", "tests/skeleton/clock.spec.ts", 'import { needsBuild, report, type Report } from "./report.ts";',
    'import { needsBuild, report, type Reports } from "./report.ts";'],
];

const clean = tsc();
console.log(`clean copy: ${clean.failed ? "FAILED" : "passes"}${clean.failed ? "\n  " + clean.lines.join("\n  ") : ""}`);
for (const [what, file, from, to] of plants) {
  const full = path.join(copy, file);
  const original = readFileSync(full, "utf8");
  if (!original.includes(from)) {
    console.log(`plant "${what}": NOT APPLIED (text not found)`);
    continue;
  }
  writeFileSync(full, original.replace(from, to));
  try {
    const result = tsc();
    console.log(`plant "${what}": ${result.failed ? "caught" : "PASSED UNSEEN"}${result.failed ? ` -> ${result.lines[0]}` : ""}`);
  } finally {
    writeFileSync(full, original);
  }
}
console.log(`after the plants: ${tsc().failed ? "the copy FAILS (not restored?)" : "the copy passes again"}`);

// security-auditor scratch (read-only, writes nothing): U7 Block 0a's binding check of web/'s two npm files
// (advisor-u7-brief.md C10 items (1), (2), (4), (6); security-u7-deps.md).
// Use: node web-lock-check.mjs <spike dir>                      checks <spike>/web/package.json and package-lock.json
//      node web-lock-check.mjs <spike dir> --model [--plant N]  checks the modelled files (self-test; plants 1-6 FAIL)
// Exit 0 when every check passes, 1 otherwise. Prints one PASS/FAIL line per check.
import { existsSync, readFileSync, readdirSync, statSync } from "node:fs";
import { join } from "node:path";
import { pathToFileURL } from "node:url";

const TS = "6.0.3";
const RESOLVED = "https://registry.npmjs.org/typescript/-/typescript-6.0.3.tgz";
// The registry's dist.integrity, read 2026-11-23 and equal to the tarball's own sha512 (security-u7-deps.md).
const INTEGRITY = "sha512-y2TvuxSZPDyQakkFRPZHKFm+KKVqIisdg9/CZwm9ftvKXLP8NRWj38/ODjNbr43SsoXqNuAisEf1GdCxqWcdBw==";
const ENTRY_KEYS = ["version", "resolved", "integrity", "dev", "license", "bin", "engines"];

const args = process.argv.slice(2);
const spike = args[0];
if (!spike) throw new Error("name the spike folder (spikes/netcode-proof)");
const web = join(spike, "web");
const model = args.includes("--model");
const plant = model && args.includes("--plant") ? Number(args[args.indexOf("--plant") + 1]) : 0;

let pkgText;
let lockText;
if (model) {
  const pkg = { name: "npt-web", private: true, type: "module", devDependencies: { typescript: TS } };
  const lock = {
    name: "npt-web",
    lockfileVersion: 3,
    requires: true,
    packages: {
      "": { name: "npt-web", devDependencies: { typescript: TS } },
      "node_modules/typescript": {
        version: TS, resolved: RESOLVED, integrity: INTEGRITY, dev: true, license: "Apache-2.0",
        bin: { tsc: "bin/tsc", tsserver: "bin/tsserver" }, engines: { node: ">=14.17" },
      },
    },
  };
  if (plant === 1) lock.packages["node_modules/typescript"].hasInstallScript = true;
  if (plant === 2) lock.packages["node_modules/typescript"].integrity = INTEGRITY.replace("y2T", "y2U");
  if (plant === 3) pkg.scripts = { postinstall: "node x.js" };
  if (plant === 4) lock.packages["node_modules/left-pad"] = { version: "1.3.0" };
  if (plant === 5) lock.packages["node_modules/typescript"].resolved = RESOLVED.replace("registry.npmjs.org", "registry.example.org");
  if (plant === 6) pkg.devDependencies.typescript = "^6.0.3";
  pkgText = JSON.stringify(pkg, null, 2) + "\n";
  lockText = JSON.stringify(lock, null, 2) + "\n";
} else {
  pkgText = readFileSync(join(web, "package.json"), "utf8");
  lockText = readFileSync(join(web, "package-lock.json"), "utf8");
}

let fails = 0;
const check = (id, ok, what) => {
  if (!ok) fails++;
  console.log(`${ok ? "PASS" : "FAIL"} ${id} ${what}`);
};
const sameKeys = (o, keys) => o && typeof o === "object" && Object.keys(o).sort().join() === [...keys].sort().join();
const subsetKeys = (o, keys) => o && typeof o === "object" && Object.keys(o).every((k) => keys.includes(k));
const exactDeps = (o) => sameKeys(o, ["typescript"]) && o.typescript === TS;

const pkg = JSON.parse(pkgText);
check("1a", sameKeys(pkg, ["name", "private", "type", "devDependencies"]), "package.json keys exactly name, private, type, devDependencies");
check("1b", typeof pkg.name === "string" && /^[a-z0-9][a-z0-9._-]{0,213}$/.test(pkg.name) && pkg.name !== "typescript", "name is a plain npm name");
check("1c", pkg.private === true && pkg.type === "module", '"private": true and "type": "module"');
check("1d", exactDeps(pkg.devDependencies), `devDependencies exactly {"typescript": "${TS}"}`);

const lock = JSON.parse(lockText);
const pk = lock.packages ?? {};
const root = pk[""];
const ts = pk["node_modules/typescript"];
check("2a", sameKeys(lock, ["name", "lockfileVersion", "requires", "packages"]) && lock.lockfileVersion === 3 && lock.requires === true && lock.name === pkg.name, "lockfileVersion 3, requires true, name as package.json");
check("2b", sameKeys(pk, ["", "node_modules/typescript"]), "packages exactly the root and node_modules/typescript");
check("2c", sameKeys(root, ["name", "devDependencies"]) && root.name === pkg.name && exactDeps(root.devDependencies), "root entry: name and devDependencies typescript 6.0.3 only");
check("2d", subsetKeys(ts, ENTRY_KEYS) && ts.version === TS && ts.dev === true, `typescript entry ${TS}, dev, keys within ${ENTRY_KEYS.join(" ")} (no hasInstallScript, no dependencies)`);
check("2e", ts?.resolved === RESOLVED, `resolved ${RESOLVED}`);
check("2f", ts?.integrity === INTEGRITY, "integrity equals the registry's sha512 read on 2026-11-23");

// Item (4): no .npmrc anywhere under the spike (by name only; never read), no npm-shrinkwrap.json in web/.
const SKIP = new Set([".git", "node_modules", "target", ".gate"]);
const found = [];
const walk = (p) => {
  for (const n of readdirSync(p)) {
    const q = join(p, n);
    if (SKIP.has(n)) continue;
    if (n === ".npmrc") found.push(q);
    if (statSync(q).isDirectory()) walk(q);
  }
};
walk(spike);
check("4a", found.length === 0, `no .npmrc under ${spike}${found.length ? ": " + found.join(", ") : ""}`);
check("4b", !existsSync(join(web, "npm-shrinkwrap.json")), "no web/npm-shrinkwrap.json (npm would read it before the lock)");

// Item (6): the leak scanner check.mjs's redact step runs (F26's fix), on both texts.
const { scanText } = await import(pathToFileURL(join(spike, "ops", "operator", "redact-check.mjs")).href);
const hits = [...scanText(pkgText), ...scanText(lockText)];
check("6", hits.length === 0, `redact-check scanText: ${hits.length} finding(s)${hits.length ? " " + JSON.stringify(hits) : ""}`);

console.log(`${model ? `model${plant ? ` plant ${plant}` : ""}` : web}: ${fails ? `${fails} FAIL` : "all PASS"}`);
process.exitCode = fails ? 1 : 0;

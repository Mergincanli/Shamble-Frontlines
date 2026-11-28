// helpers.mjs: shared test code. The tests run over tsc's output (C7): NPT_WEB_OUT names that folder, else
// <checkout>/.gate/netcode-proof-test/web-check (tsconfig.json's outDir). Nothing here writes under web/.
import assert from "node:assert/strict";
import { existsSync, mkdirSync, mkdtempSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

export const WEB = resolve(dirname(fileURLToPath(import.meta.url)), "..");
export const ROOT = resolve(WEB, "..", "..", "..");
export const GATE = join(ROOT, ".gate", "netcode-proof-test");
export const OUT = resolve(process.env.NPT_WEB_OUT || join(GATE, "web-check"));

/** One of tsc's emitted modules; a missing one fails the test (never a skip). */
export async function load(name) {
  const f = join(OUT, name);
  if (!existsSync(f)) throw new Error(`no ${name} in tsc's output folder: run tsc -p web first`);
  return import(pathToFileURL(f).href);
}

/** A fresh scratch folder under .gate/netcode-proof-test/web-test/. */
export function scratch(tag) {
  const base = join(GATE, "web-test");
  mkdirSync(base, { recursive: true });
  return mkdtempSync(join(base, `${tag}-`));
}

/** Each line is npt-log's `event` or `second` form exactly (crates/log/src/lib.rs:195-233; C1), with no token or URL. */
export function checkLine(line, tok) {
  const r = JSON.parse(line);
  const name = /^[a-z][a-z0-9_]*$/;
  if (r.k === "event") {
    assert.deepEqual(Object.keys(r), ["k", "ev", "ph", "ns", "f"]);
    assert.match(r.ev, name);
    assert.ok(Number.isSafeInteger(r.ns) && r.ns >= 0);
    for (const [k, v] of Object.entries(r.f)) assert.ok(name.test(k) && Number.isSafeInteger(v), k);
  } else {
    assert.equal(r.k, "second");
    assert.deepEqual(Object.keys(r), ["k", "sec", "ph", "v"]);
    assert.ok(Number.isSafeInteger(r.sec) && r.sec >= 0);
    for (const [k, v] of Object.entries(r.v)) assert.ok(name.test(k) && Number.isSafeInteger(v), k);
  }
  assert.equal(r.ph, "measured");
  assert.equal(line.includes(tok.slice(5, 15)), false);
  assert.equal(line.includes("nptj_"), false);
  assert.equal(/https?:|wss?:/.test(line), false);
}

/** A slot token in U6a's form (`nptj_` + 22 base64url), built at run time so no token literal sits in the tree. */
export function token(seed = "Ab3") {
  return ["nptj", "_", seed.repeat(8).slice(0, 21), "A"].join("");
}

// build.test.mjs: Block T1 (u7-brief.md rev 1.1): index.html under the CSP, build.mjs's dist and its checks, web/'s
// npm rule (advisor-u8-brief.md C2 as security-u7-deps.md item (4) applies it).
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync, readdirSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { test } from "node:test";
import {
  DIST_NAMES, GATE as BUILD_GATE, WEB as BUILD_WEB, assemble, distProblems, encodedRustFlags, homePaths, outDirFor,
  parseArgs, wasmBindgenOk,
} from "../build.mjs";
import { GATE, WEB, scratch } from "./helpers.mjs";

const html = readFileSync(join(WEB, "index.html"), "utf8");

test("index.html: no inline script, no inline style, the three elements the scripts read", () => {
  const scripts = [...html.matchAll(/<script\b([^>]*)>([\s\S]*?)<\/script>/gi)];
  assert.equal(scripts.length, 1);
  assert.match(scripts[0][1], /\btype="module"/);
  assert.match(scripts[0][1], /\bsrc="main\.js"/);
  assert.equal(scripts[0][2].trim(), "");
  assert.doesNotMatch(html, /<style\b/i);
  assert.doesNotMatch(html, /\sstyle\s*=/i);
  assert.doesNotMatch(html, /\son[a-z]+\s*=/i);
  assert.doesNotMatch(html, /javascript:/i);
  for (const id of ["status", "version", "counters"]) assert.match(html, new RegExp(`id="${id}"`));
});

test("the dist is the seven names U6c's host takes", () => {
  assert.deepEqual([...DIST_NAMES], [
    "index.html", "main.js", "worker.js", "lib.js", "hooks.js", "npt_client_wasm.js", "npt_client_wasm_bg.wasm",
  ]);
});

function fakeInputs(dir) {
  const js = join(dir, "js");
  const bg = join(dir, "bindgen");
  mkdirSync(js);
  mkdirSync(bg);
  for (const n of ["main.js", "worker.js", "lib.js", "hooks.js"]) writeFileSync(join(js, n), `// ${n}\n`);
  writeFileSync(join(bg, "npt_client_wasm.js"), "// glue\n");
  writeFileSync(join(bg, "npt_client_wasm_bg.wasm"), Buffer.from([0, 0x61, 0x73, 0x6d, 1, 0, 0, 0]));
  writeFileSync(join(js, "extra.js"), "// never copied\n");
  return { jsDir: js, bindgenDir: bg };
}

test("assemble: a fresh folder with exactly the seven files, each SHA-256 right", () => {
  const dir = scratch("assemble");
  const inputs = fakeInputs(dir);
  const out = join(dir, "dist");
  mkdirSync(out);
  writeFileSync(join(out, "stale.txt"), "old\n");
  const sums = assemble({ out, index: join(WEB, "index.html"), ...inputs });
  assert.deepEqual(readdirSync(out).sort(), [...DIST_NAMES].sort());
  assert.deepEqual(sums.map((s) => s.name), [...DIST_NAMES]);
  for (const s of sums) {
    assert.equal(s.sha256, createHash("sha256").update(readFileSync(join(out, s.name))).digest("hex"));
  }
});

test("assemble: a hooks file replaces tsc's; a missing input or a folder outside .gate/ is refused", () => {
  const dir = scratch("assemble-hooks");
  const inputs = fakeInputs(dir);
  const hooks = join(dir, "other-hooks.js");
  writeFileSync(hooks, "// other\n");
  const out = join(dir, "dist");
  assemble({ out, index: join(WEB, "index.html"), ...inputs, hooks });
  assert.equal(readFileSync(join(out, "hooks.js"), "utf8"), "// other\n");
  assert.throws(() => assemble({ out, index: join(dir, "absent.html"), ...inputs }), /missing index\.html/);
  assert.throws(() => assemble({ out: join(WEB, "test"), index: join(WEB, "index.html"), ...inputs }), /web\/dist or under \.gate/);
});

test("outDirFor: web/dist, the bites folder under .gate/ (C13), --out only under .gate/netcode-proof-test/", () => {
  assert.equal(BUILD_WEB, WEB);
  assert.equal(BUILD_GATE, GATE);
  assert.equal(outDirFor({}), join(WEB, "dist"));
  assert.equal(outDirFor({ bites: true }), join(GATE, "dist-bites"));
  assert.equal(outDirFor({ out: join(GATE, "t5", "dist") }), join(GATE, "t5", "dist"));
  assert.throws(() => outDirFor({ out: join(WEB, "dist2") }), /under \.gate/);
  assert.throws(() => outDirFor({ out: GATE }), /under \.gate/);
  assert.throws(() => outDirFor({ out: join(GATE, "..", "x") }), /under \.gate/);
  assert.deepEqual(parseArgs([]), { bites: false, out: null });
  assert.deepEqual(parseArgs(["--bites"]), { bites: true, out: null });
  assert.throws(() => parseArgs(["--bites", "--out", "x"]), /own folder/);
  assert.throws(() => parseArgs(["--fast"]), /unknown argument/);
});

test("wasm-bindgen: exactly 0.2.130 passes", () => {
  assert.equal(wasmBindgenOk("wasm-bindgen 0.2.130\n"), true);
  assert.equal(wasmBindgenOk("wasm-bindgen 0.2.129"), false);
  assert.equal(wasmBindgenOk("wasm-bindgen 0.2.1300"), false);
  assert.equal(wasmBindgenOk(""), false);
});

test("the remap flags name CARGO_HOME, the workspace and the target, one per field", () => {
  const f = encodedRustFlags("/h/.cargo", "/w/spike", "/w/.gate/t").split("\x1f");
  assert.deepEqual(f, [
    "--remap-path-prefix=/h/.cargo=/cargo-home",
    "--remap-path-prefix=/w/spike=/npt",
    "--remap-path-prefix=/w/.gate/t=/npt-target",
  ]);
});

test("the dist scan: innerHTML in any of the seven fails; a bite id fails the measured dist only (U7R-TE-9)", () => {
  const clean = DIST_NAMES.map((n) => [n, Buffer.from(`// ${n}: textContent only\n`)]);
  assert.deepEqual(distProblems(clean, false), []);
  const withHtml = clean.map(([n, b]) => [n, n === "npt_client_wasm.js" ? Buffer.from("x.innerHTML = y") : b]);
  assert.deepEqual(distProblems(withHtml, true), ["npt_client_wasm.js holds innerHTML"]);
  const withBite = clean.map(([n, b]) => [n, n === "hooks.js" ? Buffer.from('const B = ["PM-0001-36/read-once"];') : b]);
  assert.deepEqual(distProblems(withBite, false), ["hooks.js holds a bite id in the measured dist"]);
  assert.deepEqual(distProblems(withBite, true), []);
});

test("the Wasm scan finds absolute home paths and passes remapped ones", () => {
  const bad = Buffer.from("\0\0panicked at /home/someone/.cargo/registry/src/x/lib.rs\0", "latin1");
  assert.deepEqual(homePaths(bad), ["/home/"]);
  assert.deepEqual(homePaths(Buffer.from("C:\\Users\\me\\x.rs")), ["C:\\Users\\"]);
  assert.deepEqual(homePaths(Buffer.from("/opt/w/spike/crates/a.rs"), ["/opt/w/spike"]), ["/opt/w/spike"]);
  assert.deepEqual(homePaths(Buffer.from("\0/cargo-home/registry/src/x.rs /npt/crates/client/src/lib.rs\0")), []);
});

test("web/ holds no .npmrc and no npm-shrinkwrap.json (C2; security-u7-deps.md item (4))", () => {
  const found = [];
  const walk = (d) => {
    for (const n of readdirSync(d)) {
      if (n === "node_modules") continue;
      const p = join(d, n);
      if (n === ".npmrc" || n === "npm-shrinkwrap.json") found.push(p);
      if (statSync(p).isDirectory()) walk(p);
    }
  };
  walk(WEB);
  assert.deepEqual(found, []);
  assert.equal(existsSync(join(WEB, "dist")), false, "a web/dist folder fails check.mjs's redact step (N-U7D-1)");
});

// security-auditor scratch (read-only, writes nothing): models Cargo.lock after U7's Block 0b, on top of the U6b and
// U6c models of ../security-u6c/lockmodel.mjs (replayed here unchanged, since that script exports nothing). Prints
// hashes, counts and the added lines only.
// Use: node lockmodel-u7.mjs
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";

const LOCK = new URL("../../../spikes/netcode-proof/Cargo.lock", import.meta.url);
const before = readFileSync(LOCK, "utf8");
const sha = (t) => createHash("sha256").update(t).digest("hex");
const REG = 'source = "registry+https://github.com/rust-lang/crates.io-index"';
function parse(text) {
  const [head, ...rest] = text.split("\n[[package]]\n");
  return { head, blocks: rest.map((b) => {
    const body = b.replace(/\n+$/, "");
    return { name: body.match(/^name = "([^"]+)"/m)[1], version: body.match(/^version = "([^"]+)"/m)[1], body };
  }) };
}
const emit = ({ head, blocks }) => head + blocks.map((b) => "\n[[package]]\n" + b.body + "\n").join("");
const cmpVer = (a, b) => {
  const pa = a.split(/[.+-]/).map(Number), pb = b.split(/[.+-]/).map(Number);
  for (let i = 0; i < 3; i++) if (pa[i] !== pb[i]) return pa[i] - pb[i];
  return 0;
};
const order = (a, b) => (a.name < b.name ? -1 : a.name > b.name ? 1 : 0) || cmpVer(a.version, b.version);
const copy = (l) => ({ head: l.head, blocks: l.blocks.map((b) => ({ ...b })) });
const find = (l, name, version) => {
  const b = l.blocks.find((x) => x.name === name && (!version || x.version === version));
  if (!b) throw new Error(`no ${name} ${version ?? ""}`);
  return b;
};
const edit = (b, fn) => { const lines = b.body.split("\n"); fn(lines); b.body = lines.join("\n"); };
const dropLine = (b, line) => edit(b, (ls) => { const i = ls.indexOf(line); if (i < 0) throw new Error(`${b.name}: no ${line}`); ls.splice(i, 1); });
const replaceLine = (b, from, to) => edit(b, (ls) => { const i = ls.indexOf(from); if (i < 0) throw new Error(`${b.name}: no ${from}`); ls[i] = to; });
const setDeps = (b, deps) => {
  const lines = b.body.split("\n");
  const i = lines.indexOf("dependencies = [");
  b.body = [...(i < 0 ? lines : lines.slice(0, i)), ...(deps.length ? ["dependencies = [", ...deps.map((d) => ` "${d}",`), "]"] : [])].join("\n");
};
const numstat = (aText, zText) => {
  const cnt = (t) => { const m = new Map(); for (const l of t.split("\n")) m.set(l, (m.get(l) ?? 0) + 1); return m; };
  const a = cnt(aText), z = cnt(zText);
  let add = 0, rem = 0; const added = [];
  for (const [l, n] of z) if (n > (a.get(l) ?? 0)) { add += n - (a.get(l) ?? 0); added.push(l); }
  for (const [l, n] of a) if (n > (z.get(l) ?? 0)) rem += n - (z.get(l) ?? 0);
  return { add, rem, added };
};

const lock = parse(before);
if (emit(lock) !== before) throw new Error("parse/emit is not an identity on the real lock");
console.log(`real lock: sha256 ${sha(before)}, ${lock.blocks.length} packages`);

// U6b's Block 0 (security-u6b-deps.md section 4), as lockmodel.mjs:66-78.
const u6b = copy(lock);
for (const [n, v] of [["aws-lc-rs", "1.19.0"], ["aws-lc-sys", "0.47.0"], ["cmake", "0.1.58"], ["dunce", "1.0.5"], ["fs_extra", "1.3.0"], ["jobserver", "0.1.35"], ["pkg-config", "0.3.34"]]) {
  find(u6b, n, v);
  u6b.blocks = u6b.blocks.filter((b) => !(b.name === n && b.version === v));
}
for (const n of ["quinn-proto", "rustls", "rustls-webpki"]) dropLine(find(u6b, n), ' "aws-lc-rs",');
dropLine(find(u6b, "cc"), ' "jobserver",');
dropLine(find(u6b, "cc"), ' "libc",');
replaceLine(find(u6b, "npt-server"), ' "npt-sim",', ' "npt-sim",\n "rcgen",');
const u6bText = emit(u6b);
console.log(`U6b model: sha256 ${sha(u6bText)}, ${u6b.blocks.length} packages`);

// U6c's Block 0 (security-u6c-deps.md item 7), as lockmodel.mjs:80-122.
const u6c = copy(u6b);
const NEW = [
  ["async-trait", "0.1.92", "82f6aeea286b8eb4dd3431a1be1b59d290ace00f5bfd8e2a159bc2a05e2c1667", ["proc-macro2", "quote", "syn 3.0.9"]],
  ["atomic-waker", "1.1.2", "1505bd5d3d116872e7271a6d4e16d81d0c8570876c8de68093a09ac269d8aac0", []],
  ["base64", "0.23.1", "ac07cdecf99051d9a5238b80f35af32cdeba5b336e55d957b318b50137e18da5", []],
  ["fnv", "1.0.7", "3f9eec918d3f24069decb9af1554cad7c880e2da24a9afd88aca000531ab82c1", []],
  ["h2", "0.4.20", "7d29020232d6aa3fb1daca64c1127cf662cf97f254ae16c18c05b8ab635fc118", ["atomic-waker", "bytes", "fnv", "futures-core", "futures-sink", "http", "indexmap", "slab", "tokio", "tokio-util", "tracing"]],
  ["http-body", "1.1.0", "ca2a8f2913ee65f60facd6a5905613afaa448497a0230cc41ce022d93290bc2c", ["bytes", "http"]],
  ["http-body-util", "0.1.5", "23169fe34a5fbcdd3f3862e78fb9b6fccd5f02a6dc6f732547005d45631ce71c", ["bytes", "futures-core", "http", "http-body", "pin-project-lite"]],
  ["httpdate", "1.0.3", "df3b46402a9d5adb4c86a0cf463f42e19994e3ee891101b1841f30a545cb49a9", []],
  ["hyper", "1.12.0", "2c3e324da4c95177d6291d4c8730197c0d1822f8a9766814a4a44fa5ab797c9c", ["atomic-waker", "bytes", "futures-channel", "futures-core", "h2", "http", "http-body", "httparse", "itoa", "pin-project-lite", "smallvec", "tokio", "want"]],
  ["hyper-rustls", "0.27.10", "dfa8e654703247911e29c23fbeaa261834bd9bb74efba2f9acddc37bfb127f53", ["http", "hyper", "hyper-util", "rustls", "rustls-native-certs", "rustls-platform-verifier", "tokio", "tokio-rustls", "tower-service"]],
  ["hyper-util", "0.1.21", "ddc03d96684f9226b8a787cdb71488417b53ab5ea8fdb1dac946cb9431cc8bff", ["bytes", "futures-channel", "futures-util", "http", "http-body", "httparse", "hyper", "libc", "pin-project-lite", "socket2", "tokio", "tower-service", "tracing"]],
  ["instant-acme", "0.8.5", "9f05ad37c421b962354c358d347d4a6130151df9407978372d3ad7f0c8f71a64", ["async-trait", "base64 0.22.1", "bytes", "http", "http-body", "http-body-util", "httpdate", "hyper", "hyper-rustls", "hyper-util", "rcgen", "ring", "rustls", "rustls-pki-types", "serde", "serde_json", "thiserror", "tokio"]],
  ["pem", "4.0.0", "d354a98a3d1251555de99e8fdd8afda05573c31b82f59063a7b0a29b5527f120", ["base64 0.23.1", "serde_core"]],
  ["tokio-util", "0.7.20", "e464cf451ba96ebfc6f9b6542f17ee8b8956e33f1e40d9690624e59d7a7f8a4b", ["bytes", "futures-core", "futures-sink", "libc", "pin-project-lite", "tokio"]],
  ["tower-service", "0.3.3", "8df9b6e13f2d32c91b9bd719c00d1958837bc7dec474d94952798cc8e69eeec3", []],
  ["try-lock", "0.2.5", "e421abadd41a4225275504ea4d6566923418b7f05506fbc9c0fe86ba7396114b", []],
  ["want", "0.3.2", "ec4cdd0dd910afe868b7ef477227d8d538b46b3075031afee8a9f2acb0a2ed0b", ["try-lock"]],
];
for (const [name, version, cksum, deps] of NEW) {
  const body = [`name = "${name}"`, `version = "${version}"`, REG, `checksum = "${cksum}"`];
  if (deps.length) body.push("dependencies = [", ...deps.map((d) => ` "${d}",`), "]");
  u6c.blocks.push({ name, version, body: body.join("\n") });
}
replaceLine(find(u6c, "pem", "3.0.6"), ' "base64",', ' "base64 0.22.1",');
replaceLine(find(u6c, "sfv"), ' "base64",', ' "base64 0.22.1",');
replaceLine(find(u6c, "wtransport"), ' "pem",', ' "pem 3.0.6",');
replaceLine(find(u6c, "rcgen"), ' "ring",', ' "pem 4.0.0",\n "ring",');
setDeps(find(u6c, "npt-host"), ["bytes", "http", "http-body-util", "httparse", "instant-acme", "npt-log", "rcgen", "serde", "serde_json", "sha2", "tokio", "tokio-rustls"]);
setDeps(find(u6c, "npt-bots"), ["npt-client", "npt-link", "npt-log", "npt-net", "npt-proto", "npt-server", "npt-sim", "serde", "serde_json", "tokio"]);
u6c.blocks.sort(order);
const u6cText = emit(u6c);
console.log(`U6c model: sha256 ${sha(u6cText)}, ${u6c.blocks.length} packages, ${u6cText.split("\n").length - 1} lines`);

// U7's Block 0b (advisor-u7-brief.md C6): npt-client-wasm's array gains npt-room, npt-server, npt-sim, wasm-bindgen.
const u7 = copy(u6c);
const DEPS = ["npt-client", "npt-room", "npt-server", "npt-sim", "wasm-bindgen"];
for (const d of DEPS) {
  if (u7.blocks.filter((b) => b.name === d).length !== 1) throw new Error(`${d}: not exactly one version, so the array would name "${d} <version>"`);
}
setDeps(find(u7, "npt-client-wasm"), [...DEPS].sort());
const u7Text = emit(u7);
const ns = numstat(u6cText, u7Text);
console.log(`U7 model: sha256 ${sha(u7Text)}, ${u7.blocks.length} packages, ${u7Text.split("\n").length - 1} lines`);
console.log(`U7 vs U6c (multiset numstat): ${ns.add} added, ${ns.rem} removed: ${JSON.stringify(ns.added)}`);
console.log(`npt-client-wasm block after:\n${find(u7, "npt-client-wasm").body}`);
console.log(`wasm-bindgen block (unchanged): ${find(u7, "wasm-bindgen").body === find(u6c, "wasm-bindgen").body}`);

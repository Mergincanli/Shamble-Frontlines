// security-auditor scratch (read-only): fetches one published .crate from static.crates.io into memory, checks its
// sha256, and prints the files whose path matches a pattern (or lists every file). Nothing is written to disk.
// Use: NODE_USE_ENV_PROXY=1 node crate-read.mjs <name> <version> [path regex] [--list]
import { createHash } from "node:crypto";
import { gunzipSync } from "node:zlib";

const [name, version, pattern, flag] = process.argv.slice(2);
if (!name || !version) throw new Error("usage: crate-read.mjs <name> <version> [regex] [--list]");
const url = `https://static.crates.io/crates/${name}/${name}-${version}.crate`;
const res = await fetch(url);
if (!res.ok) throw new Error(`${url}: HTTP ${res.status}`);
const gz = Buffer.from(await res.arrayBuffer());
console.log(`# ${url} sha256 ${createHash("sha256").update(gz).digest("hex")} (${gz.length} bytes)`);
const tar = gunzipSync(gz);
const re = pattern && pattern !== "--list" ? new RegExp(pattern) : null;
const list = flag === "--list" || pattern === "--list";
let longName = null;
for (let off = 0; off + 512 <= tar.length; ) {
  const h = tar.subarray(off, off + 512);
  if (h.every((b) => b === 0)) break;
  let path = h.subarray(0, 100).toString("utf8").replace(/\0.*$/s, "");
  const prefix = h.subarray(345, 500).toString("utf8").replace(/\0.*$/s, "");
  if (prefix) path = `${prefix}/${path}`;
  const size = parseInt(h.subarray(124, 136).toString("utf8").replace(/\0.*$/s, "").trim() || "0", 8);
  const type = String.fromCharCode(h[156] || 48);
  const body = tar.subarray(off + 512, off + 512 + size);
  off += 512 + Math.ceil(size / 512) * 512;
  if (type === "L") {
    longName = body.toString("utf8").replace(/\0.*$/s, "");
    continue;
  }
  if (longName) {
    path = longName;
    longName = null;
  }
  if (type !== "0" && type !== "\0") continue;
  if (list) console.log(`${size}\t${path}`);
  else if (re && re.test(path)) {
    console.log(`===== ${path} (${size} bytes)`);
    const lines = body.toString("utf8").split("\n");
    lines.forEach((l, i) => console.log(`${String(i + 1).padStart(5)}  ${l}`));
  }
}

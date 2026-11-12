// rust-reviewer scratch (U4d2 round 1), read only: prints a .wasm file's size, exports, imports and custom sections,
// with its target_features section decoded (prefix + name), so the review can see the exact export list and whether any
// SIMD feature is used. Usage: node u4d2-review-rust-wasminfo.mjs <file.wasm>...
import { readFileSync } from "node:fs";
import { gzipSync } from "node:zlib";

function features(buf) {
  const b = new Uint8Array(buf);
  let i = 0;
  const leb = () => {
    let r = 0;
    let s = 0;
    for (;;) {
      const x = b[i++];
      r |= (x & 0x7f) << s;
      if (!(x & 0x80)) return r;
      s += 7;
    }
  };
  const n = leb();
  const out = [];
  for (let k = 0; k < n; k++) {
    const p = String.fromCharCode(b[i++]);
    const len = leb();
    out.push(p + new TextDecoder().decode(b.slice(i, i + len)));
    i += len;
  }
  return out;
}

for (const path of process.argv.slice(2)) {
  const bytes = readFileSync(path);
  const m = new WebAssembly.Module(bytes);
  const custom = {};
  for (const name of ["name", "producers", "target_features", ".debug_info", "linking"]) {
    const s = WebAssembly.Module.customSections(m, name);
    if (s.length) custom[name] = s.map((x) => x.byteLength);
  }
  const tf = WebAssembly.Module.customSections(m, "target_features").flatMap(features);
  console.log(
    JSON.stringify({
      path,
      raw: bytes.length,
      gzip9: gzipSync(bytes, { level: 9 }).length,
      exports: WebAssembly.Module.exports(m),
      imports: WebAssembly.Module.imports(m),
      custom,
      target_features: tf,
    }),
  );
}

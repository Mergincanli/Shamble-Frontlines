// The security seat's read-only probe (u4d2-review-security.md): each function export of a .wasm file with its type,
// read from the module's own type, import, function and export sections (the fact U4D2-Q7 (b) would have the loader
// read). Usage: node sec-probe-wasm.mjs <file.wasm>... It writes nothing.
import { readFileSync } from "node:fs";

const VAL = { 0x7f: "i32", 0x7e: "i64", 0x7d: "f32", 0x7c: "f64", 0x7b: "v128", 0x70: "funcref", 0x6f: "externref" };

function types(bytes) {
  let p = 8;
  const u = () => {
    let r = 0;
    let s = 0;
    for (;;) {
      const b = bytes[p++];
      r |= (b & 0x7f) << s;
      s += 7;
      if (!(b & 0x80)) return r >>> 0;
    }
  };
  const name = () => {
    const n = u();
    const t = Buffer.from(bytes.subarray(p, p + n)).toString("utf8");
    p += n;
    return t;
  };
  const sigs = [];
  let imported = 0;
  const funcs = [];
  const exports = [];
  while (p < bytes.length) {
    const id = bytes[p++];
    const size = u();
    const end = p + size;
    if (id === 1) {
      for (let n = u(), i = 0; i < n; i++) {
        p++; // 0x60
        const params = Array.from({ length: u() }, () => VAL[bytes[p++]] ?? "?");
        const results = Array.from({ length: u() }, () => VAL[bytes[p++]] ?? "?");
        sigs.push(`(${params.join(", ")}) -> (${results.join(", ")})`);
      }
    } else if (id === 2) {
      for (let n = u(), i = 0; i < n; i++) {
        name();
        name();
        const kind = bytes[p++];
        if (kind === 0) (u(), imported++);
        else break;
      }
    } else if (id === 3) {
      for (let n = u(), i = 0; i < n; i++) funcs.push(u());
    } else if (id === 7) {
      for (let n = u(), i = 0; i < n; i++) {
        const nm = name();
        const kind = bytes[p++];
        const idx = u();
        exports.push({ nm, kind, idx });
      }
    }
    p = end;
  }
  return exports.map(({ nm, kind, idx }) =>
    kind === 0 ? `${nm}: function ${sigs[funcs[idx - imported]] ?? "(an import)"}` : `${nm}: kind ${kind}`,
  );
}

for (const f of process.argv.slice(2)) {
  console.log(`--- ${f}`);
  for (const l of types(readFileSync(f))) console.log(`    ${l}`);
}

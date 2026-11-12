// The security seat's read-only probe (u4d2-review-security.md): each .wasm file's custom section names, and its
// `target_features` section decoded (count, then per feature a prefix byte `+`/`-`/`=` and a name). Writes nothing.
import { readFileSync } from "node:fs";

for (const f of process.argv.slice(2)) {
  const m = new WebAssembly.Module(readFileSync(f));
  const names = new Set();
  const bytes = readFileSync(f);
  // Custom section names, read from the binary (WebAssembly.Module.customSections needs the name).
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
  while (p < bytes.length) {
    const id = bytes[p++];
    const size = u();
    const end = p + size;
    if (id === 0) {
      const n = u();
      names.add(Buffer.from(bytes.subarray(p, p + n)).toString("utf8"));
    }
    p = end;
  }
  const tf = WebAssembly.Module.customSections(m, "target_features").map((b) => {
    const v = new Uint8Array(b);
    let q = 0;
    const lu = () => {
      let r = 0;
      let s = 0;
      for (;;) {
        const x = v[q++];
        r |= (x & 0x7f) << s;
        s += 7;
        if (!(x & 0x80)) return r >>> 0;
      }
    };
    const out = [];
    for (let n = lu(), i = 0; i < n; i++) {
      const pre = String.fromCharCode(v[q++]);
      const len = lu();
      out.push(`${pre}${Buffer.from(v.subarray(q, q + len)).toString("utf8")}`);
      q += len;
    }
    return out.join(" ");
  });
  console.log(`--- ${f}: custom sections [${[...names].join(", ")}]; target_features: ${tf.length ? tf.join(" | ") : "(none)"}`);
}

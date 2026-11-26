// Verifier scratch: real ties inside the corpus's tie section (body lines 12038..14037, 0-based), by places and sign.
import { readFileSync } from "node:fs";
const body = readFileSync(process.argv[2], "utf8").split("\n").slice(1, -1);
const dv = new DataView(new ArrayBuffer(8));
const sec = body.slice(2038 + 10000, 2038 + 12000);
const byP = new Map(); let pos = 0, neg = 0, real = 0;
for (const l of sec) {
  const [bits] = l.split(" ");
  let b = BigInt("0x" + bits); const sign = b >> 63n; b &= (1n << 63n) - 1n;
  const ex = Number(b >> 52n); let m = (b & ((1n << 52n) - 1n)) | (ex ? 1n << 52n : 0n); let e = ex ? ex - 1075 : -1074;
  while ((m & 1n) === 0n) { m >>= 1n; e++; }
  const p = -e; if (p < 1) { console.log("NOT FRACTIONAL:", l); continue; }
  dv.setBigUint64(0, BigInt("0x" + bits)); const v = dv.getFloat64(0);
  const low = (m * 5n ** BigInt(p)) / 10n;
  const s = (n) => { const str = n.toString().padStart(p, "0"); return (sign ? "-" : "") + (str.slice(0, str.length - (p - 1)) || "0") + (p > 1 ? "." + str.slice(str.length - (p - 1)) : ""); };
  const a = s(low), c = s(low + 1n);
  if (Object.is(Number(a), v) && Object.is(Number(c), v)) { real++; byP.set(p, (byP.get(p) ?? 0) + 1); sign ? neg++ : pos++; }
  else console.log("NOT A TIE:", l, "p", p, a, "->", Number(a), c, "->", Number(c));
}
console.log("tie section lines", sec.length, "real ties", real, "+", pos, "-", neg);
console.log([...byP].sort((a, b) => a[0] - b[0]).map(([p, n]) => `${p}:${n}`).join(" "));
console.log("first/last of section:", sec[0], "|", sec.at(-1));
console.log("next line (powers start):", body[2038 + 12000]);

// Verifier scratch: count real ties in the corpus by binary places and sign, independently of corpus-texts.mjs.
// A real tie: the double v = m / 2^p (m odd, p >= 1); its exact decimal has p places ending in 5; the two texts at
// p-1 places on either side both parse back to v (so v is exactly halfway between two texts that are both v's).
import { readFileSync } from "node:fs";
const file = process.argv[2];
const lines = readFileSync(file, "utf8").split("\n");
const header = lines[0].split(" ");
const body = lines.slice(1, -1);
console.log("header count", header[2], "body lines", body.length, "last line empty", lines.at(-1) === "");
const dv = new DataView(new ArrayBuffer(8));
function decompose(bits) {
  const b = BigInt("0x" + bits);
  const sign = b >> 63n;
  const ex = Number((b >> 52n) & 0x7ffn);
  let frac = b & ((1n << 52n) - 1n);
  let m, e;
  if (ex === 0) { m = frac; e = -1074; } else { m = frac | (1n << 52n); e = ex - 1075; }
  if (m === 0n) return null;
  while ((m & 1n) === 0n) { m >>= 1n; e += 1; }
  return { sign, m, e };
}
const ties = new Map();
let tieTotal = 0;
const tieIdx = [];
for (let i = 0; i < body.length; i++) {
  const [bits, text, canon] = body[i].split(" ");
  dv.setBigUint64(0, BigInt("0x" + bits));
  const v = dv.getFloat64(0);
  if (!Object.is(JSON.parse(text), v)) throw new Error(`line ${i + 2}: text ${text} does not parse to bits`);
  if (JSON.stringify(v) !== canon) throw new Error(`line ${i + 2}: canonical mismatch`);
  const d = decompose(bits);
  if (!d || d.e >= 0) continue;
  const p = -d.e;
  if (p > 40) continue; // far too many places to be a short-text tie
  const scaled = d.m * 5n ** BigInt(p); // exact value * 10^p
  const low = scaled / 10n; // truncated to p-1 places
  const s = (n) => {
    let str = n.toString().padStart(p, "0");
    const intPart = str.slice(0, str.length - (p - 1)) || "0";
    const fr = str.slice(str.length - (p - 1));
    return (d.sign ? "-" : "") + intPart + (fr ? "." + fr : "");
  };
  const lo = Number(s(low)), hi = Number(s(low + 1n));
  if (Object.is(lo, v) && Object.is(hi, v)) {
    const key = `${p} ${d.sign ? "-" : "+"}`;
    ties.set(key, (ties.get(key) ?? 0) + 1);
    tieTotal++;
    tieIdx.push(i);
  }
}
const byP = new Map();
for (const [k, n] of ties) { const p = Number(k.split(" ")[0]); byP.set(p, (byP.get(p) ?? 0) + n); }
console.log("real ties total", tieTotal, "line range", tieIdx[0] + 2, "..", tieIdx.at(-1) + 2);
console.log("by places:", [...byP.entries()].sort((a, b) => a[0] - b[0]).map(([p, n]) => `${p}:${n}`).join(" "));
let neg = 0, pos = 0; for (const [k, n] of ties) (k.endsWith("-") ? (neg += n) : (pos += n));
console.log("signs: +", pos, "-", neg, "; places >= 5:", [...byP.entries()].filter(([p]) => p >= 5).reduce((a, [, n]) => a + n, 0));
// Both signs of 2^-24 and its neighbours present?
const want = ["3e70000000000000", "be70000000000000", "3e6fffffffffffff", "be6fffffffffffff", "3e70000000000001", "be70000000000001"];
const set = new Set(body.map((l) => l.split(" ")[0]));
console.log("2^-24 family present:", want.map((w) => `${w}:${set.has(w)}`).join(" "));
// Parts by position, as corpusFile orders them.
const legacy = readFileSync(process.argv[3], "utf8").split("\n").slice(1, -1);
const ok = legacy.every((t, i) => body[i].split(" ")[1] === t);
console.log("first", legacy.length, "lines are legacy-numbers texts in order:", ok);
// Powers of two section: count lines whose double is +-2^k or a neighbour of one.
let pw = 0;
for (const l of body) {
  const b = BigInt("0x" + l.split(" ")[0]) & ((1n << 63n) - 1n);
  const isPow = (x) => { if (x <= 0n || x >= 0x7ff0000000000000n) return false; const f = x & ((1n << 52n) - 1n); return (x >> 52n) > 0n ? f === 0n : (f & (f - 1n)) === 0n; };
  const near = [b - 1n, b, b + 1n].some(isPow);
  if (near) pw++;
}
console.log("lines that are +-2^k or a neighbour (any part):", pw);

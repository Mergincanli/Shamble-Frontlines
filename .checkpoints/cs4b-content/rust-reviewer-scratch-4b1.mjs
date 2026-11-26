// rust-reviewer's scratch for change-set 4b-1: V8's String(x) against number_text, line by line.
// Usage: node rust-reviewer-scratch-4b1.mjs <file of "<16 hex bits> <text>" lines>
import { createReadStream } from "node:fs";
import { createInterface } from "node:readline";

const lines = createInterface({ input: createReadStream(process.argv[2]), crlfDelay: Infinity });
const view = new DataView(new ArrayBuffer(8));
let compared = 0;
const differ = [];
for await (const line of lines) {
  if (line === "") continue;
  const [hex, text] = line.split(" ");
  view.setBigUint64(0, BigInt("0x" + hex));
  const x = view.getFloat64(0);
  if (!Number.isFinite(x)) continue;
  compared += 1;
  const engine = String(x);
  if (engine !== text) differ.push(`${hex} number_text ${text} engine ${engine} engine-reads-back ${Number(text) === x}`);
}
console.log(`compared ${compared}, differ ${differ.length}`);
for (const line of differ.slice(0, 30)) console.log("  " + line);

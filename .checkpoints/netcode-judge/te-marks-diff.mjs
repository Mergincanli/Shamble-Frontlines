// test-engineer's independent check of marks.json's diff (part a): strip every `moved`, undo the three named edits,
// compare with the base; then list every label with its place and text. Usage: node te-marks-diff.mjs <base> <head>
import { readFileSync } from "node:fs";
import { isDeepStrictEqual } from "node:util";

const [base, head] = process.argv.slice(2).map((p) => JSON.parse(readFileSync(p, "utf8")));
const labels = [];
const strip = (x, where) => {
  if (Array.isArray(x)) return x.map((v, i) => strip(v, `${where}[${i}]`));
  if (x && typeof x === "object") {
    const out = {};
    for (const [k, v] of Object.entries(x)) {
      if (k === "moved") {
        labels.push({ where, name: x.id ?? x.name, ...v });
        continue;
      }
      out[k] = strip(v, `${where}.${k}`);
    }
    return out;
  }
  return x;
};
const h = strip(head, "");
// Undo -56: drop the added 4-item check; restore notes. Undo -7: notes.
const byId = (f, id) => f.marks.find((m) => m.id === id);
const b56 = byId(base, "PM-0001-56"), h56 = byId(h, "PM-0001-56");
const added = h56.checks.filter((c) => !b56.checks.some((bc) => bc.name === c.name));
console.log("-56 added checks:", JSON.stringify(added));
const old10 = b56.checks.find((c) => c.name.includes("10 items"));
console.log("-56 old 10-item check (base):", JSON.stringify(old10));
h56.checks = h56.checks.filter((c) => !added.includes(c));
console.log("-56 notes tail added:", JSON.stringify(h56.notes.slice(b56.notes.length)));
console.log("-56 notes start unchanged:", h56.notes.startsWith(b56.notes));
h56.notes = b56.notes;
const b7 = byId(base, "PM-0001-7"), h7 = byId(h, "PM-0001-7");
// find the changed span
let i = 0;
while (i < b7.notes.length && b7.notes[i] === h7.notes[i]) i++;
let j = 0;
while (j < b7.notes.length - i && b7.notes[b7.notes.length - 1 - j] === h7.notes[h7.notes.length - 1 - j]) j++;
console.log("-7 removed:", JSON.stringify(b7.notes.slice(i, b7.notes.length - j)));
console.log("-7 added:  ", JSON.stringify(h7.notes.slice(i, h7.notes.length - j)));
h7.notes = b7.notes;
console.log("rest equal after strip and undo:", isDeepStrictEqual(base, h));
if (!isDeepStrictEqual(base, h)) {
  for (let k = 0; k < base.marks.length; k++)
    if (!isDeepStrictEqual(base.marks[k], h.marks[k])) console.log("differs:", base.marks[k].id);
}
const kinds = { mark: 0, part: 0, check: 0, bite: 0 };
for (const l of labels) {
  const kind = /\.parts\[/.test(l.where) ? "part" : /\.checks\[/.test(l.where) ? "check" : /\.bites\[/.test(l.where) ? "bite" : "mark";
  kinds[kind]++;
  l.kind = kind;
}
console.log("counts:", JSON.stringify(kinds));
const bys = new Set(labels.map((l) => l.by));
console.log("by values:", JSON.stringify([...bys]));
for (const l of labels) console.log(`${l.kind}\t${l.where}\t${l.name}\t-> ${l.to}\t| ${l.why}`);

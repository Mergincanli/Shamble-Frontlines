import { execFileSync } from "node:child_process";
const HEAD = "d59ca43c559fe085ca90f45fdbac593236399533";
const get = (ref) => execFileSync("git", ["show", `${ref}:docs/DECISIONS.md`]).toString();
const sect = (s) => {
  const lines = s.split("\n");
  const i = lines.findIndex((l) => /^##+ .*Queued/.test(l));
  if (i < 0) return null;
  const lvl = lines[i].match(/^#+/)[0].length;
  let j = i + 1;
  while (j < lines.length && !(lines[j].startsWith("#") && lines[j].match(/^#+/)[0].length <= lvl)) j++;
  return { head: lines[i], body: lines.slice(i, j).join("\n"), from: i + 1, to: j };
};
const b = sect(get("4d21c1a"));
const h = sect(get(HEAD));
console.log("base Queued:", b && b.head, b && b.from, b && b.to);
console.log("head Queued:", h && h.head, h && h.from, h && h.to);
if (b && h) {
  const strip = (t) => t.split("\n").filter((l) => !l.includes("_Pointer (2026-11-13")).join("\n");
  console.log("Queued identical:", b.body === h.body);
  if (b.body !== h.body) {
    const bl = b.body.split("\n"), hl = h.body.split("\n");
    const onlyH = hl.filter((l) => !bl.includes(l));
    const onlyB = bl.filter((l) => !hl.includes(l));
    console.log("lines only in head:", onlyH.length, "only in base:", onlyB.length);
    for (const l of onlyH) console.log("  +", l.slice(0, 160));
    for (const l of onlyB) console.log("  -", l.slice(0, 160));
  }
}

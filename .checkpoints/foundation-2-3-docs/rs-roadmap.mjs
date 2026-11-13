import { execSync } from "node:child_process";
const HEAD = "d59ca43c559fe085ca90f45fdbac593236399533";
const b = execSync("git show 4d21c1a:docs/ROADMAP.md").toString().split("\n");
const h = execSync(`git show ${HEAD}:docs/ROADMAP.md`).toString().split("\n");
const bl = b.find((l) => l.startsWith("- **v2, next:**"));
const hl = h.find((l) => l.startsWith("- **v2, next:**"));
const anchor = "pointer, 2026-11-11: for foundation steps 2.1 and 2.2, overtaken by Mustafa's decision of 2026-11-10, `docs/DECISIONS.md`";
console.log("base has anchor:", bl.includes(anchor), "head has anchor:", hl.includes(anchor));
const i = hl.indexOf(anchor);
console.log("after anchor in head:", JSON.stringify(hl.slice(i + anchor.length, i + anchor.length + 40)));
const bi = bl.indexOf(anchor);
console.log("after anchor in base:", JSON.stringify(bl.slice(bi + anchor.length, bi + anchor.length + 40)));
const ins1 = " (PR #40); the foundation: 2.1a, 2.1b and 2.2 merged (PRs #50, #51, #48), step 2.3 planned after its first docs pass (PR #52) merges";
const p7s = hl.indexOf("; pointer, 2026-11-13:");
const p7 = hl.slice(p7s, hl.indexOf("\")", p7s) + 1);
const r = hl.replace(ins1, "").replace(p7, "");
console.log("head line minus the two insertions == base line:", r === bl);
if (r !== bl) {
  for (let k = 0; k < Math.max(r.length, bl.length); k++) {
    if (r[k] !== bl[k]) { console.log("diff at", k, JSON.stringify(r.slice(k - 30, k + 30)), JSON.stringify(bl.slice(k - 30, k + 30))); break; }
  }
}
console.log("P7 length:", p7.length);
console.log("ins1 found:", hl.includes(ins1));

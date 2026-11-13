import { execFileSync } from "node:child_process";
const HEAD = "d59ca43c559fe085ca90f45fdbac593236399533";
const get = (ref) => execFileSync("git", ["show", `${ref}:docs/STATUS.md`]).toString();
const block = (s) => {
  const i = s.indexOf("- **From foundation 2.2 (PR #48), for 2.3:**");
  const j = s.indexOf("\n## ", i);
  return s.slice(i, j).replace(/\s+/g, " ");
};
const ids = (t) => new Set((t.match(/\b[A-Z][A-Z0-9]+(?:-[A-Z0-9]+)+\b|\bG0\b/g) || []));
const b = ids(block(get("4d21c1a")));
const h = ids(block(get(HEAD)));
console.log("base ids:", b.size, "head ids:", h.size);
console.log("in base, not head:", [...b].filter((x) => !h.has(x)));
console.log("in head, not base:", [...h].filter((x) => !b.has(x)));

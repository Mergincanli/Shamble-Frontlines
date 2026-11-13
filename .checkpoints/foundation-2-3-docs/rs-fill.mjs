import { execFileSync } from "node:child_process";
import { readFileSync } from "node:fs";
const T = "/root/.claude/projects/-home-user-Shamble-Frontlines/7261f3d7-f651-586a-9c39-c052553b68a3.jsonl";
let a2 = null;
for (const line of readFileSync(T, "utf8").split("\n")) {
  if (!line.trim()) continue;
  let o; try { o = JSON.parse(line); } catch { continue; }
  if (o.uuid === "e5fa7ca0-6494-44d0-8366-81fc612c7958") a2 = o.message.content;
}
const src = execFileSync("git", ["show", "e6fc6a9:.checkpoints/foundation-2-3-docs/fillcount.mjs"]).toString();
const lits = [...src.matchAll(/(["'`])((?:\\.|(?!\1).)*)\1/gs)].map((m) => m[2]).filter((s) => s.length > 40);
console.log("A2 found:", typeof a2 === "string", "long literals in fillcount.mjs:", lits.length);
for (const s of lits) {
  const un = s.replace(/\\(["'`\\])/g, "$1").replace(/^"|"$/g, "");
  console.log("literal of", un.length, "chars; substring of A2:", a2.includes(un));
}

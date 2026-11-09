// Scratch (U4s last small round): every string literal that is exactly a bite id, in crates/<crate>/src/, by crate,
// with whether it sits inside a #[cfg(test)] item. Read-only. Run: node <this> <spike dir>
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { crateFiles, lexRust } from "../../spikes/netcode-proof/check.mjs";

const here = process.argv[2];
const { files } = crateFiles(join(here, "crates"));
const ID = /"(PM-\d{4}-\d+\/[a-z0-9-]+)"/g;
for (const f of files) {
  if (!/^crates\/[^/]+\/src\/.*\.rs$/.test(f)) continue;
  const { code, mask } = lexRust(readFileSync(join(here, f), "utf8"));
  for (const m of code.matchAll(ID)) {
    if (mask.slice(m.index, m.index + m[0].length).trim()) continue;
    const line = code.slice(0, m.index).split("\n").length;
    console.log(`${f}:${line} ${m[1]}`);
  }
}

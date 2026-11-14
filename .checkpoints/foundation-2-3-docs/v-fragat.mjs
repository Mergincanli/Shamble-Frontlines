// Verifier (read-only): for each <commit>:<path> argument, does that version exist, and how many 30-char windows of
// his words (A1 to A4, in memory only) does it hold? Prints counts only.
//   node .checkpoints/foundation-2-3-docs/v-fragat.mjs <commit>:<path> [...]
import { execFileSync } from 'node:child_process';
import { allItems } from './words.mjs';
const texts = allItems().map((it) => [it.id, it.text.replace(/\s+/g, ' ')]);
for (const spec of process.argv.slice(2)) {
  let t;
  try { t = execFileSync('git', ['show', spec], { encoding: 'utf8', stdio: ['ignore', 'pipe', 'ignore'] }).replace(/\s+/g, ' '); }
  catch { console.log(`${spec}: not found`); continue; }
  const hits = [];
  for (const [id, w] of texts) { let n = 0; for (let i = 0; i + 30 <= w.length; i += 1) if (t.includes(w.slice(i, i + 30))) n++; if (n) hits.push(`${id}: ${n}`); }
  console.log(`${spec}: ${hits.length ? hits.join(', ') : 'none'}`);
}

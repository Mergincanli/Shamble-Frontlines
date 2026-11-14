// Verifier (read-only): which files hold a run of 30 or more characters of his words (A1 to A4, via words.mjs, held in
// memory only)? Prints file names and window counts only, never the text.
//   node .checkpoints/foundation-2-3-docs/v-frag.mjs head      -> every notes file as committed at a1d0317
//   node .checkpoints/foundation-2-3-docs/v-frag.mjs mine      -> this verifier's own files in the working tree
import { execFileSync } from 'node:child_process';
import { readFileSync, readdirSync } from 'node:fs';
import { allItems } from './words.mjs';
const texts = allItems().map((it) => [it.id, it.text.replace(/\s+/g, ' ')]);
const count = (t) => {
  const hits = [];
  for (const [id, w] of texts) { let n = 0; for (let i = 0; i + 30 <= w.length; i += 1) if (t.includes(w.slice(i, i + 30))) n++; if (n) hits.push(`${id}: ${n}`); }
  return hits;
};
const mode = process.argv[2] ?? 'head';
const dir = '.checkpoints/foundation-2-3-docs';
let files;
if (mode === 'head') {
  files = execFileSync('git', ['ls-tree', '-r', '--name-only', 'a1d0317', '--', `${dir}/`], { encoding: 'utf8' }).split('\n').filter(Boolean);
  files.push('.checkpoints/foundation-2-3-docs.md');
} else {
  files = readdirSync(dir).filter((f) => f === 'verifier-1.md' || /^v-/.test(f)).map((f) => `${dir}/${f}`);
}
let withHits = 0;
for (const f of files) {
  const t = (mode === 'head' ? execFileSync('git', ['show', `a1d0317:${f}`], { encoding: 'utf8' }) : readFileSync(f, 'utf8')).replace(/\s+/g, ' ');
  const h = count(t);
  if (h.length) { withHits++; console.log(`${f.split('/').pop()}: ${h.join(', ')}`); }
}
console.log(`${files.length} files checked (${mode}); ${withHits} hold a 30-character run of his words`);

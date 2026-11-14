// Verifier (read-only): in given files, find the maximal runs (>= 30 chars) shared with his words (A1 to A4, in memory
// only) and classify each: 'sha' (hex), 'path' (has / or .md/.js), or 'PROSE'. Prints the run only for sha/path;
// for PROSE prints the file line number and the run's length, never the text.
//   node .checkpoints/foundation-2-3-docs/v-fragclass.mjs <file> [<file> ...]
import { readFileSync } from 'node:fs';
import { allItems } from './words.mjs';
const texts = allItems().map((it) => [it.id, it.text.replace(/\s+/g, ' ')]);
for (const f of process.argv.slice(2)) {
  const raw = readFileSync(f, 'utf8');
  const lines = raw.split('\n');
  for (let ln = 0; ln < lines.length; ln++) {
    const t = lines[ln].replace(/\s+/g, ' ');
    for (const [id, w] of texts) {
      let i = 0;
      while (i + 30 <= t.length) {
        if (!w.includes(t.slice(i, i + 30))) { i++; continue; }
        let j = i + 30;
        while (j < t.length && w.includes(t.slice(i, j + 1))) j++;
        const run = t.slice(i, j);
        const cls = /^[0-9a-f]{30,}$/.test(run.trim()) ? 'sha' : (/\//.test(run) || /\.(md|js|mjs|py)\b/.test(run)) && !/ [a-z]+ [a-z]+ [a-z]+ [a-z]+ /.test(run) ? 'path' : 'PROSE';
        console.log(`${f.split('/').pop()}:${ln + 1} ${id} ${cls} len ${run.length}${cls === 'PROSE' ? '' : ` :: ${run}`}`);
        i = j;
      }
    }
  }
}

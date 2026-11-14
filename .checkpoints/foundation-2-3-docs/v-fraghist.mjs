// Verifier (read-only): every pushed version of this verifier's own files (verifier-1.md, v-*), read with git show,
// checked for runs of >= 30 chars shared with his words (A1 to A4, in memory only); prints commit, file, line, class
// ('sha', 'path', 'PROSE') and length only.   node .checkpoints/foundation-2-3-docs/v-fraghist.mjs
import { execFileSync } from 'node:child_process';
import { allItems } from './words.mjs';
const texts = allItems().map((it) => [it.id, it.text.replace(/\s+/g, ' ')]);
const dir = '.checkpoints/foundation-2-3-docs';
const files = [...new Set(execFileSync('git', ['log', '--format=', '--name-only', '--', `${dir}/`], { encoding: 'utf8' })
  .split('\n').filter((f) => /\/(verifier-1\.md|v-[^/]+)$/.test(f)))].sort();
let n = 0;
for (const f of files) {
  for (const s of execFileSync('git', ['log', '--format=%h', '--', f], { encoding: 'utf8' }).split('\n').filter(Boolean)) {
    let raw; try { raw = execFileSync('git', ['show', `${s}:${f}`], { encoding: 'utf8', stdio: ['ignore', 'pipe', 'ignore'] }); } catch { continue; }
    n++;
    raw.split('\n').forEach((line, ln) => {
      const t = line.replace(/\s+/g, ' ');
      for (const [id, w] of texts) {
        let i = 0;
        while (i + 30 <= t.length) {
          if (!w.includes(t.slice(i, i + 30))) { i++; continue; }
          let j = i + 30;
          while (j < t.length && w.includes(t.slice(i, j + 1))) j++;
          const run = t.slice(i, j).trim();
          const cls = /^[0-9a-f() ]{30,}$/.test(run) ? 'sha' : /\// .test(run) && !/ [a-z]+ [a-z]+ [a-z]+ /.test(run) ? 'path' : (/^[A-Z0-9-]+(, [A-Z0-9-]+)+$/.test(run.replace(/[ ,]+$/, '')) ? 'codes' : 'PROSE');
          console.log(`${s} ${f.split('/').pop()}:${ln + 1} ${id} ${cls} len ${run.length}`);
          i = j;
        }
      }
    });
  }
}
console.log(`${files.length} files, ${n} pushed versions checked`);

// Round 2's fix pass: every pushed version of round 2's notes files, checked for runs of 30 or more characters of his
// words (A1 to A4). Prints commit, file and counts only.   node .checkpoints/foundation-2-3-docs/fp2-fragments-history.mjs
import { execFileSync } from 'node:child_process';
import { allItems } from './words.mjs';
const texts = allItems().map((it) => [it.id, it.text.replace(/\s+/g, ' ')]);
// Every file of round 2's agents in the notes folder (their names start with r2, r2w or sk-f), as listed in git's
// history, so a file deleted later is checked too.
const files = [...new Set(execFileSync('git', ['log', '--format=', '--name-only', '--', '.checkpoints/foundation-2-3-docs/'], { encoding: 'utf8' })
  .split('\n').filter((f) => /^\.checkpoints\/foundation-2-3-docs\/(r2|sk-f)/.test(f)))].sort();
console.log(`${files.length} files: ${files.map((f) => f.split('/').pop()).join(', ')}`);
for (const f of files) {
  const shas = execFileSync('git', ['log', '--format=%h', '--', f], { encoding: 'utf8' }).split('\n').filter(Boolean);
  for (const s of shas) {
    let t; try { t = execFileSync('git', ['show', `${s}:${f}`], { encoding: 'utf8', stdio: ['ignore', 'pipe', 'ignore'] }).replace(/\s+/g, ' '); } catch { continue; }
    const hits = [];
    for (const [id, w] of texts) { let n = 0; for (let i = 0; i + 30 <= w.length; i += 1) if (t.includes(w.slice(i, i + 30))) n++; if (n) hits.push(`${id}: ${n}`); }
    if (hits.length) console.log(`${s} ${f.split('/').pop()}: ${hits.join(', ')}`);
  }
}
console.log('done');

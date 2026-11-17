// Checks that every quoted span in a doc is in the sources, word for word (white space aside).
//   node .checkpoints/cs0-model-rules/quotes-check.mjs <doc.md> [--allow "<name>" ...]
// A span is the text between two straight double quote marks (the odd pieces when the text is split at the marks;
// a quote inside a quote keeps the count right). Spans shorter than 12 characters and the allowed names are skipped.
import { readFileSync } from 'node:fs';
const args = process.argv.slice(2); const doc = args[0];
const allow = new Set(); for (let i = 1; i < args.length; i++) if (args[i] === '--allow') allow.add(args[++i]);
const N = '.checkpoints/build-plan-slice-1/', C = '.checkpoints/cs0-model-rules/';
const sources = [N + 'mustafa-2026-11-14-task.md', N + 'mustafa-2026-11-14-addition.md', N + 'mustafa-2026-11-17-go.md',
  C + 'mustafa-2026-11-17-2020.md', C + 'mustafa-2026-11-17-2058.md', C + 'mustafa-2026-11-17-2133.md',
  N + 'inputs/ART_BRIEF_2026-11-14_1635_conscript_stand_in.md', N + 'inputs/ART_BRIEF_ADDITION_2026-11-15_poses_yard.md',
  N + 'inputs/ART_BRIEF_2026-11-15_1735_smg_stage1.md', N + 'inputs/ART_BRIEF_CORRECTION_2026-11-15_1840_gun_poses.md',
  ...args.filter((a, i) => i > 0 && args[i - 1] === '--source')];
const flat = (s) => s.replace(/\s+/g, ' ');
const corpus = flat(sources.map((f) => readFileSync(f, 'utf8')).join('\n'));
const parts = flat(readFileSync(doc, 'utf8').replace(/^\s*>\s?/gm, '')).split('"');
if (parts.length % 2 === 0) { console.log('odd number of quote marks in', doc); process.exit(1); }
let bad = 0, n = 0;
for (let i = 1; i < parts.length; i += 2) {
  const q = parts[i].trim(); if (q.length < 12 || allow.has(q)) continue; n++;
  if (!corpus.includes(q)) { bad++; console.log('NOT IN THE SOURCES:', q.slice(0, 110)); }
}
console.log(doc, 'quoted spans checked:', n, 'not found:', bad);
process.exit(bad ? 1 : 0);

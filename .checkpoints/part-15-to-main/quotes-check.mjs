// Checks that every quoted span in a doc is in the sources, word for word (white space aside).
//   node .checkpoints/part-15-to-main/quotes-check.mjs <doc.md> [--allow "<name>" ...] [--source <file> ...]
// A span is the text between two straight double quote marks (the odd pieces when the text is split at the marks;
// a quote inside a quote keeps the count right). Spans shorter than 12 characters and the allowed names are skipped.
import { readFileSync } from 'node:fs';
const args = process.argv.slice(2); const doc = args[0];
const allow = new Set(); for (let i = 1; i < args.length; i++) if (args[i] === '--allow') allow.add(args[++i]);
const P = '.checkpoints/part-15-to-main/';
const sources = [P + 'mustafa-2026-11-17-2306.md', P + 'mustafa-2026-11-17-2357-merge-56.md', P + 'part-15-the-second-lane.md',
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

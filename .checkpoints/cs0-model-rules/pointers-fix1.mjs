// Change-set 0, the verifier's finding 1: seven pointers quoted his rule of 2026-11-14 without its last sentence,
// "If a mark fails, the code that rests on it is redone." This replaces each pointer, as pointers-insert.mjs built it
// before, with the pointer as it builds it now. Run once from the repository root, after pointers-insert.mjs was fixed:
//   node .checkpoints/cs0-model-rules/pointers-fix1.mjs
import { readFileSync, writeFileSync } from 'node:fs';
const src = readFileSync('.checkpoints/cs0-model-rules/pointers-insert.mjs', 'utf8');
const LAST = ' If a mark fails, the code that rests on it is redone.';
if (!src.includes("only when its marks are met." + LAST + "',")) throw new Error('fix pointers-insert.mjs first');
// build the pointers twice from the script's own definitions: with the rule as it was, and as it is now
const head = src.slice(src.indexOf('const N ='), src.indexOf('const count ='));
const build = (code) => new Function('readFileSync', code + '; return edits;')(readFileSync);
const now = build(head), was = build(head.replace(LAST + "',", "',"));
const files = {};
let n = 0;
for (let i = 0; i < now.length; i++) {
  const a = was[i].add ?? was[i].replace, b = now[i].add ?? now[i].replace;
  if (a === b) continue;
  const f = now[i].file; files[f] ??= readFileSync(f, 'utf8');
  const c = files[f].split(a).length - 1;
  if (c !== now[i].n) throw new Error(`${f}: the old pointer stands ${c} times, wanted ${now[i].n}`);
  files[f] = files[f].split(a).join(b); n += c;
}
for (const [f, t] of Object.entries(files)) writeFileSync(f, t);
console.log('pointers rewritten:', n);

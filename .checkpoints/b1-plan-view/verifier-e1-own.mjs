// verifier's own E1 check (read-only): exact per-bullet equality, not substring. Run from repo root.
import { execFileSync } from 'node:child_process';
import { readFileSync } from 'node:fs';
const g = (r, p) => execFileSync('git', ['show', `${r}:${p}`], { encoding: 'utf8' });
const tb = md => [...md.matchAll(/```text\n([\s\S]*?)\n```/g)].map(m => m[1]);
const list = tb(g('4f268da', '.checkpoints/kit-v3-3/queue/mustafa-2026-11-03-list.md'))[0].split('\n');
const add = tb(g('4f268da', '.checkpoints/kit-v3-3/queue/mustafa-2026-11-05-a4-b1.md'))[0].split('\n');
const b1 = list.find(l => l.includes('Plan view: draw to build'));
const pre = ' 1. EDITOR_V2: add ';
if (!b1.startsWith(pre)) throw new Error('b1 prefix');
const b1text = b1.slice(pre.length);
const i = add.findIndex(l => l.startsWith('B.1 addition:'));
const eight = add.slice(i + 1).map(l => l.replace(/^- /, ''));
// expected order in EDITOR_V2: eight[0..3], plan view, eight[4..7]
const expected = [...eight.slice(0, 4), b1text, ...eight.slice(4)];
const ed = readFileSync('docs/EDITOR_V2.md', 'utf8').split('\n');
const s = ed.findIndex(l => l.startsWith('- **Decided 2026-11-05'));
const e = ed.findIndex((l, k) => k > s && /^(## |- )/.test(l));
const block = ed.slice(s + 1, e).join('\n');
const items = block.split(/\n(?=  - )/).slice(1).map(t => t.replace(/\s+/g, ' ').trim());
let fail = 0;
console.log(`block lines ${s + 1}-${e}, items ${items.length}`);
items.forEach((it, k) => {
  let body = it.replace(/^- `\[DECIDED\]` /, '');
  if (k === 4) body = body.replace(/^\*\*Mustafa, 2026-11-03 \(his B\.1\):\*\* /, '');
  const okk = body === expected[k].replace(/\s+/g, ' ').trim() && it.startsWith('- `[DECIDED]` ');
  if (!okk) { fail++; console.log('FAIL', k, '\n got:', body, '\n exp:', expected[k]); } else console.log('ok', k, body.slice(0, 50));
});
if (items.length !== 9) fail++;
// whole-file count of each piece's first 60 chars, raw-joined
const joined = ed.map(l => l.trim()).join(' ').replace(/\s+/g, ' ');
expected.forEach((p, k) => { const n = joined.split(p.replace(/\s+/g, ' ').trim()).length - 1; console.log('count', k, n); if (n !== 1) fail++; });
console.log(fail ? 'OWN-E1 FAIL' : 'OWN-E1 PASS');

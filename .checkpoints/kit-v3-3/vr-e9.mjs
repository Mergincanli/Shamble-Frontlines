// verifier E9: the quoted blocks in STATUS's "Queued, in Mustafa's words" against the queue files, whitespace-normalised.
import { readFileSync } from 'node:fs';
const [st, a4, list] = process.argv.slice(2).map((f) => readFileSync(f, 'utf8'));
const n = (s) => s.replace(/\s+/g, ' ').trim();
const quoted = (txt) => n(txt.split('\n').filter((l) => /^\s*>/.test(l)).map((l) => l.replace(/^\s*> ?/, '')).join('\n'));
const i = st.indexOf("- **Queued, in Mustafa's words**"), j = st.indexOf('- **Taken by Mustafa 2026-10-31', i);
const block = st.slice(i, j);
const parts = block.split(/\n  - /);
const a4St = quoted(parts.find((p) => p.startsWith('A4 (kit)'))), addSt = quoted(parts.find((p) => p.startsWith("A4's addition")));
const bSt = quoted(parts.find((p) => p.startsWith('B.1 and B.2')));
const a4Src = quoted(a4.split('### Addition')[0]), addSrc = quoted(a4.split('### Addition')[1].split('## For A4')[0]);
const fence = /```text\n([\s\S]*?)```/.exec(list)[1];
const lines = fence.split('\n');
const bSrc = n([lines[0], ...lines.slice(lines.findIndex((l) => l.startsWith('B. Docs')))].join('\n'));
const cmp = (name, a, b) => { console.log(`${name}: equal=${a === b} (status ${a.length} chars, source ${b.length})`);
  if (a !== b) { let k = 0; while (a[k] === b[k]) k++; console.log(`  first difference at ${k}: status "${a.slice(k - 20, k + 40)}" source "${b.slice(k - 20, k + 40)}"`); } };
cmp('A4', a4St, a4Src); cmp("A4's addition", addSt, addSrc); cmp('B.1/B.2 (+ header line)', bSt, bSrc);

// docs-keeper scratch: wrap a paragraph (argv[2] file holding one line) at 120 with a first-line prefix "- " and indent 2.
import { readFileSync } from 'node:fs';
const text = readFileSync(process.argv[2], 'utf8').trim();
const ind = process.argv[3] ?? '  ', first = process.argv[4] ?? '- ';
const out = []; let line = first;
for (const w of text.split(' ')) { const c = line === first || line === ind ? line + w : `${line} ${w}`;
  if (c.length > 120 && line !== first && line !== ind) { out.push(line); line = ind + w; } else line = c; }
out.push(line); console.log(out.join('\n')); console.error('lines', out.length);

// Verifier: for one transcript line, prints up to 3 short snippets (80 chars around) of each regex match in its text.
//   node verifier-grep-line.mjs <transcript.jsonl> <line> <regex>
import { createReadStream } from 'node:fs';
import { createInterface } from 'node:readline';
const [file, ln, re] = process.argv.slice(2); const rx = new RegExp(re, 'g');
const rl = createInterface({ input: createReadStream(file), crlfDelay: Infinity });
let n = 0;
for await (const line of rl) {
  n++; if (n !== Number(ln)) continue; const r = JSON.parse(line);
  const c = r.message && r.message.content; const blocks = typeof c === 'string' ? [{ text: c }] : (Array.isArray(c) ? c : []);
  const text = blocks.map((b) => b.text || (typeof b.content === 'string' ? b.content : Array.isArray(b.content) ? b.content.map((x) => x.text || '').join('') : '')).join('\n');
  console.log('line', n, r.timestamp, r.type, 'text length', text.length);
  let m, k = 0; while ((m = rx.exec(text)) && k < 3) { k++; console.log('  ...' + JSON.stringify(text.slice(Math.max(0, m.index - 60), m.index + 80)) + '...'); }
}

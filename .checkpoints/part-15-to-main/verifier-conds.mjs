// Verifier: prints the numbered lines (conditions) of one transcript record's text, each cut to 330 chars.
import { createReadStream } from 'node:fs';
import { createInterface } from 'node:readline';
const [file, ln] = process.argv.slice(2);
const rl = createInterface({ input: createReadStream(file), crlfDelay: Infinity }); let n = 0;
for await (const line of rl) { n++; if (n !== Number(ln)) continue; const r = JSON.parse(line);
  const c = r.message.content; const text = typeof c === 'string' ? c : c.map((b) => b.text || '').join('\n');
  for (const l of text.split('\n')) if (/^\s*\d+\.\s/.test(l)) console.log(l.trim().slice(0, 330)); }

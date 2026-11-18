// Verifier: lists every tool call (main or side) between two times whose input mentions a given string.
//   node verifier-touch.mjs <transcript.jsonl> <fromISO> <toISO> <needle>
import { createReadStream } from 'node:fs';
import { createInterface } from 'node:readline';
const [file, from, to, needle] = process.argv.slice(2);
const rl = createInterface({ input: createReadStream(file), crlfDelay: Infinity });
let n = 0;
for await (const line of rl) {
  n++; let r; try { r = JSON.parse(line); } catch { continue; }
  if (!r.timestamp || r.timestamp < from || r.timestamp > to) continue;
  const c = r.message && r.message.content; if (!Array.isArray(c)) continue;
  for (const b of c) if (b.type === 'tool_use') { const s = JSON.stringify(b.input); if (s.includes(needle)) console.log(n, r.timestamp, r.isSidechain ? 'SIDE' : 'MAIN', b.name, s.length, 'chars; edits?', /open\(p, 'w'|write_text|writeFileSync|"old_string"|> \.checkpoints\/part-15\/part-15|sed -i/.test(s)); }
}

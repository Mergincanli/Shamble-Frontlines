// Verifier: lists subagent hand-backs (main-thread user records whose text holds "Subagent hand-back") between two times,
// with a verdict line and whether given strings appear. Prints short snippets only.
//   node verifier-handbacks.mjs <transcript.jsonl> <fromISO> <toISO> [needle ...]
import { createReadStream } from 'node:fs';
import { createInterface } from 'node:readline';
const [file, from, to, ...needles] = process.argv.slice(2);
const rl = createInterface({ input: createReadStream(file), crlfDelay: Infinity });
let n = 0;
for await (const line of rl) {
  n++; let r; try { r = JSON.parse(line); } catch { continue; }
  if (!r.timestamp || r.timestamp < from || r.timestamp > to || r.type !== 'user' || r.isSidechain) continue;
  const c = r.message && r.message.content; const text = typeof c === 'string' ? c : (Array.isArray(c) ? c.map((b) => b.text || (typeof b.content === 'string' ? b.content : Array.isArray(b.content) ? b.content.map((x) => x.text || '').join('') : '')).join('\n') : '');
  if (!/hand-back|Subagent|SubagentHandback|task-notification/i.test(text)) continue;
  const from_ = (/agent-message from="([^"]+)"/.exec(text) || [])[1] || '';
  const v = (/(VERDICT|Verdict)[^\n]{0,120}/.exec(text) || [''])[0];
  console.log(n, r.timestamp, 'from', from_, '| len', text.length, '|', JSON.stringify(v.slice(0, 140)), '|', needles.map((s) => s + ':' + text.includes(s)).join(' '));
}

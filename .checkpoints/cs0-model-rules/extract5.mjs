// Copies Mustafa's message of 2026-11-17, 20:58 (Amsterdam) from the session's transcript, byte for byte. It arrived
// while the session's turn was running, so the transcript holds it as a queued message.
//   node .checkpoints/cs0-model-rules/extract5.mjs <transcript.jsonl> <out.md>
import { readFileSync, writeFileSync } from 'node:fs';
const [, , f, out] = process.argv;
const KEY = 'My words of 2026-11-17, 20:58 (Amsterdam time): four small things.';
let hit = null;
for (const l of readFileSync(f, 'utf8').split('\n')) {
  if (!l) continue; let o; try { o = JSON.parse(l); } catch { continue; }
  if (o.type !== 'queue-operation' || hit) continue;
  const c = typeof o.content === 'string' ? o.content : (Array.isArray(o.content) ? o.content.filter((b) => b.type === 'text').map((b) => b.text).join('') : '');
  const i = c.indexOf(KEY); if (i >= 0) hit = { ts: o.timestamp, op: o.operation, text: c.slice(i) };
}
if (!hit) { console.error('not found'); process.exit(1); }
writeFileSync(out, `# Mustafa, 2026-11-17: four small things (the pictures, the Advisor chat's log, the Art lead's correction, time stamps)\n\n_Copied by script (extract5.mjs) from this session's transcript, never retyped: the queued user message of ${hit.ts} (queue operation "${hit.op}"), from its first words ("My words of 2026-11-17, 20:58") to its end; the three attachment references before them are left out._\n\n---\n${hit.text}\n`);
console.log(hit.ts, hit.op, Buffer.byteLength(hit.text));

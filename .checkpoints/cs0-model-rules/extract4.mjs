// Copies Mustafa's message of 2026-11-17, 20:20 (Amsterdam) from the session's transcript, byte for byte.
//   node .checkpoints/cs0-model-rules/extract4.mjs <transcript.jsonl> <out.md>
import { readFileSync, writeFileSync } from 'node:fs';
const [, , f, out] = process.argv;
let hit = null;
for (const l of readFileSync(f, 'utf8').split('\n')) {
  if (!l) continue; let o; try { o = JSON.parse(l); } catch { continue; }
  if (o.type !== 'user' || !o.message || o.isCompactSummary) continue;
  const c = o.message.content;
  const texts = typeof c === 'string' ? [c] : (Array.isArray(c) ? c.filter((b) => b.type === 'text').map((b) => b.text) : []);
  for (const t of texts) { const i = t.indexOf('My words of 2026-11-17, 20:20 (Amsterdam time): three small things.'); if (i >= 0 && !hit) hit = { ts: o.timestamp, uuid: o.uuid, text: t.slice(i) }; }
}
if (!hit) { console.error('not found'); process.exit(1); }
writeFileSync(out, `# Mustafa, 2026-11-17: three small things (the pictures, the Advisor chat's log, time stamps)\n\n_Copied by script (extract4.mjs) from this session's transcript, never retyped: the user message of ${hit.ts} (uuid ${hit.uuid}), from its first words ("My words of 2026-11-17, 20:20") to its end; the attachment reference before them is left out._\n\n---\n${hit.text}\n`);
console.log(hit.ts, hit.uuid, Buffer.byteLength(hit.text));

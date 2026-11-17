// Copies Mustafa's message of 2026-11-17, 23:06 (Amsterdam) from the session's transcript, byte for byte.
//   node .checkpoints/part-15/extract7.mjs <transcript.jsonl> <out.md>
import { readFileSync, writeFileSync } from 'node:fs';
const [, , f, out] = process.argv;
const KEY = 'My words of 2026-11-17, 23:06 (Amsterdam time): my answers to Part 15';
let hit = null;
for (const l of readFileSync(f, 'utf8').split('\n')) {
  if (!l || hit) continue; let o; try { o = JSON.parse(l); } catch { continue; }
  if (o.type !== 'user' || !o.message || o.isCompactSummary) continue;
  const c = o.message.content;
  const texts = typeof c === 'string' ? [c] : (Array.isArray(c) ? c.filter((b) => b.type === 'text').map((b) => b.text) : []);
  for (const t of texts) if (t.startsWith(KEY) && !hit) hit = { ts: o.timestamp, uuid: o.uuid, text: t };
}
if (!hit) { console.error('not found'); process.exit(1); }
writeFileSync(out, `# Mustafa, 2026-11-17: his answers to Part 15's six questions; the stop line of change-set 0; pull request #56\n\n_Copied by script (extract7.mjs) from this session's transcript, never retyped: the user message of ${hit.ts} (uuid ${hit.uuid}), whole._\n\n---\n${hit.text}\n`);
console.log(hit.ts, hit.uuid, Buffer.byteLength(hit.text));

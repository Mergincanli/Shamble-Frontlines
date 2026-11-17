import { readFileSync, writeFileSync } from 'node:fs';
const [, , f, out] = process.argv;
let hit = null;
for (const l of readFileSync(f, 'utf8').split('\n')) {
  if (!l) continue; let o; try { o = JSON.parse(l); } catch { continue; }
  if (o.type !== 'user' || !o.message) continue;
  const c = o.message.content;
  const texts = typeof c === 'string' ? [c] : (Array.isArray(c) ? c.filter((b) => b.type === 'text').map((b) => b.text) : []);
  for (const t of texts) { const i = t.indexOf('My words of 2026-11-17, 20:10 (Amsterdam time): my go on your plan'); if (i >= 0) hit = { ts: o.timestamp, uuid: o.uuid, text: t.slice(i) }; }
}
if (!hit) { console.error('not found'); process.exit(1); }
writeFileSync(out, `# Mustafa, 2026-11-17: his go on the plan, his answers and four changes\n\n_Copied by script (extract3.mjs) from this session's transcript, never retyped: the user message of ${hit.ts} (uuid ${hit.uuid}), from its first words "My words of 2026-11-17, 20:10" to its end, ${Buffer.byteLength(hit.text)} bytes. The message began with five attachment references, left out here. The text below the line is his, byte for byte._\n\n---\n` + hit.text + '\n');
console.log(hit.ts, hit.uuid, Buffer.byteLength(hit.text));

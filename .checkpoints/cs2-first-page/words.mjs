// Copies one of Mustafa's chat messages from this session's transcript into the notes, byte for byte.
//   node .checkpoints/cs2-first-page/words.mjs <transcript.jsonl> <marker text> <out file> <title line>
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';
const [transcript, marker, out, title] = process.argv.slice(2);
const hits = [];
for (const line of readFileSync(transcript, 'utf8').split('\n')) {
  if (!line.includes(marker.slice(0, 20))) continue;
  let e; try { e = JSON.parse(line); } catch { continue; }
  if (e.type !== 'user' || e.isMeta || e.isCompactSummary || e.toolUseResult !== undefined) continue;
  const c = e.message?.content;
  const texts = typeof c === 'string' ? [c] : Array.isArray(c) ? c.filter((p) => p.type === 'text').map((p) => p.text) : [];
  for (const t of texts) if (t.includes(marker) && !t.startsWith('This session is being continued')) hits.push({ t, e });
}
if (hits.length !== 1) { console.error(`expected one message with the marker, found ${hits.length}`); process.exit(1); }
const { t, e } = hits[0];
const sha = createHash('sha256').update(t).digest('hex');
const head = `# ${title}\n\n_Copied by script from this session's transcript, never retyped: the user message of ${e.timestamp} (uuid ${e.uuid}), whole ` +
  `(${Buffer.byteLength(t)} bytes, sha256 ${sha})._\n\n---\n`;
writeFileSync(out, head + t + '\n');
console.log(`${out}: ${Buffer.byteLength(t)} bytes of his message, sha256 ${sha}, sent ${e.timestamp}, uuid ${e.uuid}`);

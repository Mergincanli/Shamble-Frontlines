// Copies one short chat message of Mustafa's (one whose whole text is given) from this session's transcript into the
// notes: the newest user message whose text equals <text> exactly and that was sent at or after <not before>.
//   node .checkpoints/cs2-first-page/words-exact.mjs <transcript.jsonl> <text> <not before, ISO> <out file> <title> <what it answers>
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';
const [transcript, text, notBefore, out, title, answers] = process.argv.slice(2);
const hits = [];
for (const line of readFileSync(transcript, 'utf8').split('\n')) {
  if (!line.includes('"type":"user"')) continue;
  let e; try { e = JSON.parse(line); } catch { continue; }
  if (e.type !== 'user' || e.isMeta || e.isCompactSummary || e.toolUseResult !== undefined) continue;
  const c = e.message?.content;
  const texts = typeof c === 'string' ? [c] : Array.isArray(c) ? c.filter((p) => p.type === 'text').map((p) => p.text) : [];
  if (texts.length === 1 && texts[0] === text && e.timestamp >= notBefore) hits.push(e);
}
if (hits.length !== 1) { console.error(`expected one message, found ${hits.length}`); process.exit(1); }
const e = hits[0];
const sha = createHash('sha256').update(text).digest('hex');
const n = Buffer.byteLength(text);
writeFileSync(out, `# ${title}\n\n_Copied by script from this session's transcript, never retyped: the user message of ${e.timestamp} (uuid ${e.uuid}), whole ` +
  `(${n} ${n === 1 ? 'byte' : 'bytes'}, sha256 ${sha})._\n\n_What it answers: ${answers}_\n\n---\n${text}\n`);
console.log(`${out}: ${n} byte(s), sha256 ${sha}, sent ${e.timestamp}, uuid ${e.uuid}`);

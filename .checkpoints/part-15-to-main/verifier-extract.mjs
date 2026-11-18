// Verifier: reads the session transcript line by line, finds two user records by uuid, and compares their text
// (sha256, bytes) with the words in the saved files. Prints structure, hashes and verdicts only, never the text.
//   node verifier-extract.mjs <transcript.jsonl>
import { createReadStream, readFileSync } from 'node:fs';
import { createInterface } from 'node:readline';
import { createHash } from 'node:crypto';
const sha = (s) => createHash('sha256').update(s).digest('hex');
const P = '/home/user/Shamble-Frontlines/.checkpoints/part-15-to-main/';
const words = (path) => { const t = readFileSync(path, 'utf8'); const i = t.indexOf('\n---\n'); return t.slice(i + 5).replace(/\n$/, ''); };
const want = { 'c4edbe94-57f5-451a-8778-59897cfd0cdf': P + 'mustafa-2026-11-17-2306.md', '987dac5c-6911-41b9-9d8e-2f803893370a': P + 'mustafa-2026-11-17-2357-merge-56.md' };
const rl = createInterface({ input: createReadStream(process.argv[2]), crlfDelay: Infinity });
let n = 0; const seen = {};
for await (const line of rl) {
  n++; if (!line.trim()) continue; let r; try { r = JSON.parse(line); } catch { console.log('line', n, 'not JSON'); continue; }
  if (!want[r.uuid]) continue;
  seen[r.uuid] = (seen[r.uuid] || 0) + 1;
  const c = r.message && r.message.content;
  const blocks = typeof c === 'string' ? [{ type: 'string', text: c }] : (Array.isArray(c) ? c : []);
  console.log('line', n, 'uuid', r.uuid.slice(0, 8), 'type', r.type, 'role', r.message && r.message.role, 'ts', r.timestamp,
    'isSidechain', r.isSidechain, 'userType', r.userType, 'blocks', blocks.map((b) => b.type + ':' + (b.text ? Buffer.byteLength(b.text) : '-')).join(','));
  const texts = blocks.filter((b) => b.type === 'text' || b.type === 'string').map((b) => b.text);
  const w = words(want[r.uuid]);
  texts.forEach((t, k) => {
    console.log('  text block', k, Buffer.byteLength(t), 'bytes sha', sha(t).slice(0, 16), '| saved words', Buffer.byteLength(w), 'bytes sha', sha(w).slice(0, 16),
      '| equal:', t === w, '| equal after trimming one final newline:', t.replace(/\n$/, '') === w, '| CR in text:', t.includes('\r'));
  });
}
console.log('lines read', n, 'records found', JSON.stringify(seen));

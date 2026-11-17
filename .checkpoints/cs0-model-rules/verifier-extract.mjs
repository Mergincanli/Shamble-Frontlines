// The verifier's own re-extraction (cs0-verifier). Reads the transcript line by line; for each saved message file it
// takes the file's first 50 characters below the '---' line as the key, finds every string in every record that holds
// the key, and compares the bytes from the key to the end of that string with the saved words (sha256). It prints
// only record kinds, times, ids, sizes and hashes, never message text.
//   node .checkpoints/cs0-model-rules/verifier-extract.mjs <transcript.jsonl>
import { createReadStream, readFileSync } from 'node:fs';
import { createInterface } from 'node:readline';
import { createHash } from 'node:crypto';
const [, , T] = process.argv;
const N = '.checkpoints/build-plan-slice-1/', C = '.checkpoints/cs0-model-rules/';
const files = { task: N + 'mustafa-2026-11-14-task.md', addition: N + 'mustafa-2026-11-14-addition.md',
  go: N + 'mustafa-2026-11-17-go.md', a8_2020: C + 'mustafa-2026-11-17-2020.md', n2058: C + 'mustafa-2026-11-17-2058.md',
  a2133: C + 'mustafa-2026-11-17-2133.md' };
const sha = (s) => createHash('sha256').update(s).digest('hex');
const saved = {};
for (const [k, f] of Object.entries(files)) {
  const t = readFileSync(f, 'utf8'); const i = t.indexOf('\n---\n');
  const w = t.slice(i + 5).replace(/\n$/, '');
  saved[k] = { w, key: w.slice(0, 50), sha: sha(w), bytes: Buffer.byteLength(w) };
}
const hits = Object.fromEntries(Object.keys(files).map((k) => [k, []]));
const walk = (v, path, cb) => {
  if (typeof v === 'string') cb(v, path);
  else if (Array.isArray(v)) v.forEach((x, i) => walk(x, `${path}[${i}]`, cb));
  else if (v && typeof v === 'object') for (const [k, x] of Object.entries(v)) walk(x, `${path}.${k}`, cb);
};
let n = 0;
const rl = createInterface({ input: createReadStream(T), crlfDelay: Infinity });
rl.on('line', (l) => {
  n++; if (!l) return; let o; try { o = JSON.parse(l); } catch { return; }
  const kind = o.type + (o.operation ? ':' + o.operation : '') + (o.attachment?.type ? ':' + o.attachment.type : '');
  walk(o, '', (s, path) => {
    for (const [k, m] of Object.entries(saved)) {
      const i = s.indexOf(m.key); if (i < 0) continue;
      const tail = s.slice(i);
      hits[k].push({ line: n, kind, ts: o.timestamp ?? '-', uuid: o.uuid ?? '-', path, prefix: i,
        tailBytes: Buffer.byteLength(tail), equal: tail === m.w, startsWithSaved: tail.startsWith(m.w),
        isMeta: !!o.isMeta, compact: !!o.isCompactSummary, toolResult: path.includes('tool_result') || /\.content\[\d+\]\.content/.test(path) });
    }
  });
});
rl.on('close', () => {
  console.log('transcript lines:', n);
  for (const [k, m] of Object.entries(saved)) {
    console.log(`== ${k}: saved ${m.bytes} B sha256 ${m.sha.slice(0, 16)}; hits ${hits[k].length}`);
    for (const h of hits[k]) console.log(`  L${h.line} ${h.kind} ts=${h.ts} uuid=${h.uuid} path=${h.path} prefix=${h.prefix} tail=${h.tailBytes}B equal=${h.equal}${h.equal ? '' : ' startsWithSaved=' + h.startsWithSaved}${h.isMeta ? ' META' : ''}${h.compact ? ' COMPACT' : ''}`);
  }
});

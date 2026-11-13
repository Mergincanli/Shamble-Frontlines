// R-WORDS scratch (read-only): check 4's search on joined text. For every tracked file at the reviewed head (outside
// docs/reports/ and .checkpoints/), each line's leading spaces and blockquote marks are dropped, the lines are joined
// with single spaces, and each pattern (given on the command line, case-insensitive) is searched there. A hit whose
// match spans a line break is one a line-based grep misses; only those are printed, with the line it starts on.
//   node .checkpoints/foundation-2-3-docs/rw-joined.mjs <sha> <pattern> [<pattern> ...]
import { execFileSync } from 'node:child_process';

const [sha, ...patterns] = process.argv.slice(2);
const git = (a) => execFileSync('git', a, { encoding: 'utf8', maxBuffer: 1 << 28 });
const files = git(['ls-tree', '-r', '--name-only', sha]).split('\n').filter(Boolean)
  .filter((f) => !f.startsWith('docs/reports/') && !f.startsWith('.checkpoints/'))
  .filter((f) => /\.(md|txt|json|js|mjs|cjs|ts|toml|yml|yaml|html|rs|sh|py)$/i.test(f) || !f.includes('.'));
let spans = 0;
let total = 0;
for (const f of files) {
  let text;
  try { text = git(['show', `${sha}:${f}`]); } catch { continue; }
  if (text.includes('\u0000')) continue;
  const lines = text.split('\n');
  let joined = '';
  const lineAt = [];
  lines.forEach((l, i) => {
    const s = l.replace(/^[\s>]*/, '').replace(/\s+/g, ' ');
    if (joined.length && !joined.endsWith(' ')) { joined += ' '; lineAt.push(i + 1); }
    for (const ch of s) { joined += ch; lineAt.push(i + 1); }
  });
  const low = joined.toLowerCase();
  for (const p of patterns) {
    const q = p.toLowerCase();
    let at = low.indexOf(q);
    while (at >= 0) {
      total++;
      const a = lineAt[at];
      const b = lineAt[at + q.length - 1];
      if (a !== b) {
        spans++;
        console.log(`${f}:${a}-${b} [${p}] ...${joined.slice(Math.max(0, at - 70), at + q.length + 50)}...`);
      }
      at = low.indexOf(q, at + 1);
    }
  }
}
console.log(`joined hits ${total}; hits that span a line break (missed by a line grep): ${spans}`);

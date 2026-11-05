// E1 (condition 3): Mustafa's words, byte for byte apart from wrapping. Read-only.
// Run from the repository root:
//   node .checkpoints/b1-plan-view/e1-words.mjs [--editor docs/EDITOR_V2.md] [--status docs/STATUS.md] [--also <file>]...
// Sources: A3's queue files at commit 4f268da (immutable git objects; the advisor diffed them identical to
// mustafa-words.md). If mustafa-words.md is present, the pieces are also checked against it.
// Matching rule, applied to the source pieces and to the target file alike, and nothing else:
//   per line: drop leading whitespace, then leading "> " quote marks, then one list marker ("- ", "* ", "+ ", "1. ");
//   drop the status tags (`[DECIDED]` and the others, with or without backticks); join all lines with a space;
//   collapse runs of whitespace into one space.
// The one change to his words: B.1's 2026-11-03 line loses its instruction words "EDITOR_V2: add " (the list marker
// " 1." goes by the rule above).
// Checks: EDITOR_V2 (and each --also file) holds each of the nine pieces exactly once (B.1's text, the addition's
// eight lines). STATUS holds B.2's line and his two lead lines exactly once each.
import { readFileSync, existsSync } from 'node:fs';
import { execFileSync } from 'node:child_process';
const args = process.argv.slice(2);
const opt = (k, d) => { const i = args.indexOf(k); return i >= 0 ? args[i + 1] : d; };
const also = args.flatMap((a, i) => (a === '--also' ? [args[i + 1]] : []));
const editor = opt('--editor', 'docs/EDITOR_V2.md');
const status = opt('--status', 'docs/STATUS.md');
const TAG = /`?\[(DECIDED|DEFAULT|OPEN|LATER|IDEA)\]`?/g;
function norm(text) {
  return text.split('\n').map(l => l.replace(/^\s*/, '').replace(/^(>\s?)+/, '').replace(/^\s*([-*+]|\d+\.)\s+/, '')
    .replace(TAG, '')).join(' ').replace(/\s+/g, ' ').trim();
}
function count(hay, needle) { let n = 0, i = 0; while ((i = hay.indexOf(needle, i)) >= 0) { n++; i += needle.length; } return n; }
function textBlocks(md) { return [...md.matchAll(/```text\n([\s\S]*?)\n```/g)].map(m => m[1]); }
const git = p => execFileSync('git', ['show', `4f268da:${p}`], { encoding: 'utf8' });
const list = textBlocks(git('.checkpoints/kit-v3-3/queue/mustafa-2026-11-03-list.md'))[0].split('\n');
const add = textBlocks(git('.checkpoints/kit-v3-3/queue/mustafa-2026-11-05-a4-b1.md'))[0].split('\n');
const leadLines = ['Small change-sets, one at a time, plan first (Mustafa, 2026-11-03).', 'B. Docs:'];
let fail = 0;
const ok = (c, msg) => { if (!c) fail++; console.log(`${c ? 'ok  ' : 'FAIL'} ${msg}`); };
for (const l of leadLines) ok(list.includes(l), `source holds lead line "${l}"`);
const b1src = list.find(l => l.startsWith(' 1. EDITOR_V2: add "Plan view'));
const b2src = list.find(l => l.startsWith(' 2. VISION_V2:'));
ok(!!b1src && !!b2src, 'source holds B.1 and B.2');
const b1 = norm(b1src).replace(/^EDITOR_V2: add /, '');
ok(b1.startsWith('"Plan view: draw to build". Two views') && b1.endsWith('no code now.'), 'B.1 piece spans "Plan view ... no code now."');
const addLines = add.slice(add.findIndex(l => l.startsWith('B.1 addition:')) + 1);
ok(addLines.length === 8, `the addition has 8 lines (found ${addLines.length})`);
const pieces = [['B.1 (2026-11-03)', b1], ...addLines.map((l, i) => [`addition line ${i + 1}`, norm(l)])];
if (existsSync('.checkpoints/b1-plan-view/mustafa-words.md')) {
  const mw = norm(textBlocks(readFileSync('.checkpoints/b1-plan-view/mustafa-words.md', 'utf8')).join('\n'));
  for (const [name, p] of pieces) ok(count(mw, p) === 1, `mustafa-words.md holds ${name} once`);
}
for (const f of [editor, ...also]) {
  const t = norm(readFileSync(f, 'utf8'));
  for (const [name, p] of pieces) { const n = count(t, p); ok(n === 1, `${f}: ${name} found ${n} time(s) (${p.slice(0, 48)}...)`); }
}
if (!args.includes('--no-status')) {
  const t = norm(readFileSync(status, 'utf8'));
  for (const [name, p] of [['B.2 (2026-11-03)', norm(b2src)], ['lead line 1', leadLines[0]], ['lead line 2', leadLines[1]]]) {
    const n = count(t, p); ok(n === 1, `${status}: ${name} found ${n} time(s) (${p.slice(0, 48)}...)`);
  }
}
console.log(fail ? `E1 FAIL (${fail})` : 'E1 PASS');
process.exit(fail ? 1 : 0);

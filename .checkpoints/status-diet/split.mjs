// STATUS-DIET's split (Mustafa, 2026-11-10): moves docs/STATUS.md at BASE into three files by line ranges.
// The templates in ./tpl/ hold the new text and these markers, each on a line of its own:
//   <<MOVE a-b>>        old STATUS lines a..b (1-based, inclusive), byte for byte
//   <<GITMSG sha a-b>>  lines a..b of commit sha's message, byte for byte
//   <<INDEX>>           DECISIONS.md's index, made here (pointers only)
// Usage: node .checkpoints/status-diet/split.mjs [--out <dir>]   (default .gate/status-diet/out; --out docs applies it)
import { execFileSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

export const BASE = '2d22b69';
export const FILES = ['STATUS.md', 'DECISIONS.md', 'STATUS_ARCHIVE.md'];
const HERE = path.dirname(fileURLToPath(import.meta.url));
const git = (...a) => execFileSync('git', a, { encoding: 'utf8', maxBuffer: 1 << 26 });

export function oldLines() {
  const t = git('show', `${BASE}:docs/STATUS.md`);
  if (!t.endsWith('\n')) throw new Error('STATUS at BASE has no final newline');
  return t.slice(0, -1).split('\n');
}
const msgCache = {};
function msgLines(sha) {
  if (!msgCache[sha]) msgCache[sha] = git('log', '-1', '--format=%B', sha).replace(/\n+$/, '').split('\n');
  return msgCache[sha];
}
export function readTemplate(f) { return fs.readFileSync(path.join(HERE, 'tpl', f), 'utf8').replace(/\n$/, '').split('\n'); }

// Answered codes such as ORDER (b), XP-TERM (a), WT-DAY (ii).
export const RA = /\b([A-Z][A-Z0-9]*(?:-[A-Z0-9]+)*) \(([a-eA-E]|i{1,3}|iv)\)/g;
// Every capital code with a hyphen (question ids, finding ids, decision codes).
export const RB = /\b[A-Z][A-Z0-9]*(?:-[A-Z0-9]+)+\b/g;
// A code that leads a list entry ("- DOMAIN (...)", "- LATER, Mustafa:").
export const RC = /^\s*- (?:\*\*)?([A-Z][A-Z0-9]*(?:-[A-Z0-9]+)*)(?:\*\*)?(?=[ ,:(])/;
// A line that names Mustafa with a date, or records a go, check, acceptance or approval of his.
export const RM = /(?=.*\bMustafa)(?=.*\b20\d\d-\d\d-\d\d\b)|Mustafa's go|\bhis go\b|Mustafa's "merge|Mustafa's Brave check|Mustafa (?:accepted|approved|confirmed|checked|un-paused|paused|archived|deleted)/;
// Lines the advisor named for the index that the pattern above does not catch, with what the index says of each.
export const MANUAL = {
  939: 'the three-teams plan, "APPROVED with D1-D16": the line names no approver; it was recorded on 2026-10-15 ' +
       '(a3f0471, team-prep-fixes\' docs pass), before the advisor agent existed (kit-v2-0, 2026-10-17), so the ' +
       'approval came through his chat (his own or the Advisor\'s relayed); not verified further',
};

// Expand one template: returns [{text, old}] where old is the old line number or null for new text.
export function expand(f, lines, index) {
  const out = [];
  for (const t of readTemplate(f)) {
    let m;
    if ((m = t.match(/^<<MOVE (\d+)-(\d+)>>$/))) {
      const a = +m[1], b = +m[2];
      if (!(a >= 1 && b >= a && b <= lines.length)) throw new Error(`bad range ${t} in ${f}`);
      for (let i = a; i <= b; i++) out.push({ text: lines[i - 1], old: i });
    } else if ((m = t.match(/^<<GITMSG ([0-9a-f]{7,40}) (\d+)-(\d+)>>$/))) {
      const ml = msgLines(m[1]);
      for (let i = +m[2]; i <= +m[3]; i++) out.push({ text: ml[i - 1], old: null, msg: `${m[1]}:${i}` });
    } else if (t === '<<INDEX>>') {
      for (const x of index || []) out.push({ text: x, old: null });
    } else if (/^<</.test(t)) {
      throw new Error(`unknown marker ${t} in ${f}`);
    } else out.push({ text: t, old: null });
  }
  return out;
}
const sectionOf = (rows, k) => { for (let i = k; i >= 0; i--) if (/^## /.test(rows[i].text)) return rows[i].text.slice(3); return '(top)'; };

export function build(lines) {
  const pass1 = Object.fromEntries(FILES.map(f => [f, expand(f, lines, [])]));
  const inDec = new Set();
  for (const r of pass1['DECISIONS.md']) if (r.old) for (const m of r.text.matchAll(RA)) inDec.add(m[0]);
  const codes = new Map(); // code -> Set of "file, section"
  const dated = new Map(); // "file, section" -> [old line numbers]
  for (const f of ['STATUS_ARCHIVE.md', 'STATUS.md']) {
    pass1[f].forEach((r, k) => {
      if (!r.old) return;
      const where = `${f}, "${sectionOf(pass1[f], k)}"`;
      for (const m of r.text.matchAll(RA)) if (!inDec.has(m[0])) {
        if (!codes.has(m[0])) codes.set(m[0], new Map());
        const w = codes.get(m[0]); if (!w.has(where)) w.set(where, []); w.get(where).push(r.old);
      }
      if (RM.test(r.text) || MANUAL[r.old]) { if (!dated.has(where)) dated.set(where, []); dated.get(where).push(r.old); }
    });
  }
  const index = ['- **Answered codes that stand only outside this file** (each with where it stands and its old lines):'];
  for (const [c, w] of [...codes].sort((x, y) => x[0].localeCompare(y[0])))
    index.push(`  - ${c}: ${[...w].map(([k, v]) => `${k}, old ${v.join(', ')}`).join('; ')}`);
  index.push('- **Lines that name Mustafa with a date, or record a go, check or approval of his** (by section; old lines):');
  for (const [k, v] of dated) index.push(`  - ${k}: ${v.join(', ')}`);
  for (const [n, note] of Object.entries(MANUAL)) index.push(`  - old line ${n}: ${note}`);
  const rows = Object.fromEntries(FILES.map(f => [f, expand(f, lines, f === 'DECISIONS.md' ? index : [])]));
  return { rows, text: Object.fromEntries(FILES.map(f => [f, rows[f].map(r => r.text).join('\n') + '\n'])) };
}

if (process.argv[1] && fileURLToPath(import.meta.url) === path.resolve(process.argv[1])) {
  const oi = process.argv.indexOf('--out');
  const out = oi > 0 ? process.argv[oi + 1] : '.gate/status-diet/out';
  const { text } = build(oldLines());
  fs.mkdirSync(out, { recursive: true });
  for (const f of FILES) { fs.writeFileSync(path.join(out, f), text[f]); console.log(`wrote ${path.join(out, f)}: ${text[f].split('\n').length - 1} lines`); }
}

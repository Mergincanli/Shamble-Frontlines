// E3 (kit-v3-3 step 6): the false-positive scan. Loads the SECRETS list (and ESC, if declared) from each given copy of
// checkpoint.mjs, runs every pattern over every file `git ls-files` lists (plus any extra paths after `--`), and prints
// per copy: counts, and each hit as label, kind, file:line, match length. Never prints matched text or its context.
//   node e3-scan.mjs <worktree> <label>=<checkpoint.mjs copy>... [-- <extra file>...]
import { readFileSync } from 'node:fs';
import { execFileSync } from 'node:child_process';
const [W, ...rest] = process.argv.slice(2);
const dash = rest.indexOf('--');
const copies = (dash < 0 ? rest : rest.slice(0, dash)).map((a) => a.split('='));
const extra = dash < 0 ? [] : rest.slice(dash + 1);
const load = (file) => {
  const src = readFileSync(file, 'utf8');
  const from = src.search(/^const (ESC|SECRETS) = /m), to = src.indexOf('\n];', src.indexOf('const SECRETS = '));
  return new Function(src.slice(from, to + 3) + '\nreturn SECRETS;')();
};
const files = execFileSync('git', ['-C', W, 'ls-files', '-z']).toString().split('\0').filter(Boolean).map((f) => W + '/' + f);
const texts = [];
let binary = 0;
for (const f of [...files, ...extra]) {
  let buf; try { buf = readFileSync(f); } catch { continue; }
  if (buf.includes(0)) { binary++; continue; }
  texts.push([f.startsWith(W + '/') ? f.slice(W.length + 1) : f, buf.toString('utf8')]);
}
console.log(`tracked=${files.length} extra=${extra.length} scanned=${texts.length} binary=${binary}`);
const keyed = {};
for (const [label, file] of copies) {
  const hits = [];
  for (const [kind, re] of load(file)) {
    const g = new RegExp(re.source, re.flags + 'g');
    for (const [f, t] of texts) for (const m of t.matchAll(g)) hits.push(`${kind}\t${f}:${t.slice(0, m.index).split('\n').length}\tlen=${m[0].length}`);
  }
  keyed[label] = new Set(hits);
  console.log(`[${label}] hits=${hits.length}`); for (const h of hits) console.log(`  ${h}`);
}
const [first, ...others] = copies.map(([l]) => l);
for (const l of others) console.log(`new in ${l} vs ${first}: ${[...keyed[l]].filter((h) => !keyed[first].has(h)).length}`);

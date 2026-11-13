// Inserts the dated pointers of 2026-11-13 (plan §4) right after 2.1a's pointers, and fills the {{P-2.3}} placeholder
// docs-keeper leaves in STATUS; then checks every site. Each pointer quotes his decision's first four sentences, cut by
// script from A2 (words.mjs), never retyped.
//   node .checkpoints/foundation-2-3-docs/pointers.mjs [--check] [--root <dir>] [--only P1,P2,...]
// --check writes nothing; --root runs on a copy of the tree (a scratch test); --only limits the sites (P8 waits for
// docs-keeper's placeholder). A site already carrying its pointer is left as it is. Exit 1 on any problem.
import { readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { messageText, ITEMS, decision } from './words.mjs';

export const ENTRY = "Taken by Mustafa, 2026-11-13, 18:47:55 UTC: his decision on step 2.3 (step 2.3 may be planned now), and step 2.3's task";
const LAST = 'Steps 2.4 to 2.6 still wait for their results.';
export const MARK = '_Pointer (2026-11-13, foundation 2.3';

// His decision's first four sentences: every limit he set on step 2.3.
export function four() {
  const d = decision(messageText(ITEMS[1].uuid));
  const i = d.indexOf(LAST);
  if (i < 0) throw new Error('the decision has no sentence "Steps 2.4 to 2.6 still wait for their results."');
  return d.slice(0, i + LAST.length);
}

const TEXT = {
  // P1 to P4: the sites that quote his words of 2026-11-10.
  quote: (q) => `_Pointer (2026-11-13, foundation 2.3's first docs pass): for foundation step 2.3 only, his words of 2026-11-10 ` +
    `quoted above are changed by his decision of 2026-11-13 (\`docs/DECISIONS.md\`, the entry "${ENTRY}"): "${q}"_`,
  // P5: SLICE-ORDER's condition 3.
  slice: (q) => `_Pointer (2026-11-13, foundation 2.3's first docs pass): for foundation step 2.3 only, overtaken in part by his ` +
    `decision of 2026-11-13: the entry "${ENTRY}", above; its words: "${q}"_`,
  // P7: ROADMAP's v2 line, inline after 2.1a's pointer.
  roadmap: (q) => `pointer, 2026-11-13: for foundation step 2.3 only, overtaken in part by his decision of 2026-11-13, \`docs/DECISIONS.md\`: "${q}"`,
  // P8: STATUS's Next item (docs-keeper's placeholder line).
  status: (q) => `_Pointer (2026-11-13): for foundation step 2.3 only, his decision of 2026-11-13 (\`docs/DECISIONS.md\`): "${q}"_`,
};

// Block sites: after the n-th 2.1a pointer block in a file, ended by the first line that ends with `end`.
export const SITES = [
  { id: 'P1', file: 'docs/SPIKE_PLAN.md', nth: 0, end: 'results."_', text: 'quote' },
  { id: 'P2', file: 'docs/SPIKE_PLAN.md', nth: 1, end: 'results."_', text: 'quote' },
  { id: 'P3', file: 'docs/SPIKE_PLAN.md', nth: 2, end: 'results."_', text: 'quote' },
  { id: 'P4', file: 'docs/adr/0002-code-structure-build-toolchain.md', nth: 0, end: 'results."_', text: 'quote' },
  { id: 'P5', file: 'docs/DECISIONS.md', nth: 0, end: 'above._', text: 'slice' },
];
const START = '_Pointer (2026-11-11, foundation 2.1a)';
const ROADMAP = { id: 'P7', file: 'docs/ROADMAP.md',
  anchor: "pointer, 2026-11-11: for foundation steps 2.1 and 2.2, overtaken by Mustafa's decision of 2026-11-10, `docs/DECISIONS.md`" };
const STATUS = { id: 'P8', file: 'docs/STATUS.md', placeholder: '{{P-2.3}}' };

export function wrap(text, indent, width = 120) {
  const out = [];
  let line = indent;
  for (const w of text.split(' ')) {
    const cand = line === indent ? indent + w : `${line} ${w}`;
    if (cand.length > width && line !== indent) { out.push(line); line = indent + w; } else line = cand;
  }
  out.push(line);
  return out;
}

function blockSite(lines, site) {
  const starts = lines.map((l, i) => (l.trimStart().startsWith(START) ? i : -1)).filter((i) => i >= 0);
  const s = starts[site.nth];
  if (s === undefined) throw new Error(`${site.id}: 2.1a's pointer number ${site.nth + 1} not found in ${site.file}`);
  let e = s;
  while (e < lines.length && !lines[e].endsWith(site.end)) e++;
  if (e >= lines.length || e - s > 6) throw new Error(`${site.id}: the end of 2.1a's pointer not found`);
  return { start: s, end: e, indent: lines[s].match(/^ */)[0] };
}

export function apply(root, only, write) {
  const q = four();
  const problems = [];
  const done = [];
  const files = new Map();
  const get = (f) => { if (!files.has(f)) files.set(f, readFileSync(join(root, f), 'utf8')); return files.get(f); };
  for (const site of SITES.filter((x) => !only || only.includes(x.id))) {
    const lines = get(site.file).split('\n');
    const b = blockSite(lines, site);
    const add = wrap(TEXT[site.text](q), b.indent);
    const next = lines.slice(b.end + 1, b.end + 1 + add.length);
    if (next.join('\n') === add.join('\n')) { done.push(`${site.id} present`); continue; }
    if (lines[b.end + 1]?.trimStart().startsWith(MARK)) { problems.push(`${site.id}: a different 2026-11-13 pointer stands there`); continue; }
    lines.splice(b.end + 1, 0, ...add);
    files.set(site.file, lines.join('\n'));
    done.push(`${site.id} added (${add.length} lines after ${site.file}:${b.end + 1})`);
  }
  if (!only || only.includes('P7')) {
    const t = get(ROADMAP.file);
    const want = `${ROADMAP.anchor}; ${TEXT.roadmap(q)})`;
    const n = t.split(ROADMAP.anchor).length - 1;
    if (t.includes(want)) done.push('P7 present');
    else if (n !== 1) problems.push(`P7: the anchor is found ${n} times in ${ROADMAP.file}`);
    else if (!t.includes(`${ROADMAP.anchor})`)) problems.push('P7: the anchor is not followed by its closing parenthesis');
    else { files.set(ROADMAP.file, t.replace(`${ROADMAP.anchor})`, want)); done.push('P7 added'); }
  }
  if (!only || only.includes('P8')) {
    const lines = get(STATUS.file).split('\n');
    const at = lines.map((l, i) => (l.trim() === STATUS.placeholder ? i : -1)).filter((i) => i >= 0);
    const present = get(STATUS.file).includes(`"${q}"`);
    if (at.length === 0 && present) done.push('P8 present');
    else if (at.length !== 1) problems.push(`P8: ${at.length} placeholder lines in ${STATUS.file} (the quote ${present ? 'is' : 'is not'} there)`);
    else {
      const indent = lines[at[0]].match(/^ */)[0];
      lines.splice(at[0], 1, ...wrap(TEXT.status(q), indent));
      files.set(STATUS.file, lines.join('\n'));
      done.push('P8 filled');
    }
  }
  if (write && !problems.length) for (const [f, t] of files) writeFileSync(join(root, f), t);
  return { problems, done, q };
}

// C2: every 2026-11-13 pointer in a file, its wrapped lines joined, indentation and italic marks stripped; its quote
// (the text between ': "' and the closing '"') must be a substring of the decision cut from A2.
export function joinedQuotes(text) {
  const lines = text.split('\n');
  const out = [];
  for (let i = 0; i < lines.length; i++) {
    if (!lines[i].includes(MARK.slice(1)) && !lines[i].includes('pointer, 2026-11-13:')) continue;
    let j = i;
    let joined = lines[i].trim();
    while (!/"_?\)?\s*$/.test(joined) && j + 1 < lines.length && lines[j + 1].trim() !== '') joined += ` ${lines[++j].trim()}`;
    joined = joined.replace(/^_|_$/g, '');
    const m = joined.match(/: "(.+?)"_?\)?(?:\s.*)?$/);
    out.push({ line: i + 1, quote: m ? m[1] : null, joined });
    i = j;
  }
  return out;
}

// The check: every site carries its pointer once, right after its anchor, with the quote equal to four().
export function verify(root) {
  const q = four();
  const problems = [];
  const d = decision(messageText(ITEMS[1].uuid));
  for (const f of [...new Set([...SITES.map((x) => x.file), ROADMAP.file, STATUS.file])]) {
    for (const p of joinedQuotes(readFileSync(join(root, f), 'utf8'))) {
      if (!p.quote) problems.push(`${f}:${p.line}: a 2026-11-13 pointer with no quote found`);
      else if (!d.includes(p.quote)) problems.push(`${f}:${p.line}: the quote is not a substring of his decision`);
    }
  }
  for (const site of SITES) {
    const lines = readFileSync(join(root, site.file), 'utf8').split('\n');
    const b = blockSite(lines, site);
    const add = wrap(TEXT[site.text](q), b.indent);
    if (lines.slice(b.end + 1, b.end + 1 + add.length).join('\n') !== add.join('\n')) problems.push(`${site.id}: the pointer is not right after 2.1a's pointer`);
  }
  const rm = readFileSync(join(root, ROADMAP.file), 'utf8');
  if (!rm.includes(`${ROADMAP.anchor}; ${TEXT.roadmap(q)})`)) problems.push('P7: missing');
  const st = readFileSync(join(root, STATUS.file), 'utf8');
  if (st.includes(STATUS.placeholder)) problems.push('P8: the placeholder is still there');
  if (!st.replace(/\n */g, ' ').includes(TEXT.status(q))) problems.push('P8: missing');
  return problems;
}

if (import.meta.url === `file://${process.argv[1]}`) {
  const ri = process.argv.indexOf('--root');
  const root = ri > 0 ? process.argv[ri + 1] : '.';
  const oi = process.argv.indexOf('--only');
  const only = oi > 0 ? process.argv[oi + 1].split(',') : null;
  if (process.argv.includes('--check')) {
    const p = verify(root);
    console.log(`${p.length ? 'FAIL' : 'OK'}: P1 to P5, P7 and P8 checked under ${root}; the quote is the decision's first ${four().length} characters`);
    for (const x of p) console.log(`  ${x}`);
    process.exit(p.length ? 1 : 0);
  }
  const r = apply(root, only, true);
  console.log(`${r.problems.length ? 'FAIL (nothing written)' : 'OK'}: ${r.done.join('; ')}`);
  for (const x of r.problems) console.log(`  ${x}`);
  process.exit(r.problems.length ? 1 : 0);
}

// Adds foundation 2.3's first docs pass entries to docs/DECISIONS.md: his words copied by script from the report's
// Appendix A (never retyped), each under a header written for this file, at its place by time (newest first), and one
// sentence in the header of his 2026-11-10 entry (that header's lines from the sentence on reflowed). Then it re-reads
// the file and checks: every new quote byte for byte against the appendix; the pointers' entry title present once; the
// 2026-11-10 header's words equal the old words plus the sentence, its lines before the sentence unchanged; and, against
// the base 4d21c1a, that the only removed lines are that header's reflowed lines.
//   node .checkpoints/foundation-2-3-docs/decisions.mjs [--check] [--negative] [--file <path>] [--report <path>]
// --check writes nothing; --negative runs the negative controls in memory (each must fail); --file and --report point
// at copies (a scratch test). Exit 1 on any problem.
import { readFileSync, writeFileSync } from 'node:fs';
import { execFileSync } from 'node:child_process';
import { reportBlocks, REPORT } from './report-appendix.mjs';
import { decision } from './words.mjs';
import { ENTRY } from './pointers.mjs';

const FILE = 'docs/DECISIONS.md';
const BASE = '4d21c1a7a4d39613bffce5d5f9bbc92f2f88a300';
const REP = '`docs/reports/2026-11-13-foundation-2-3-docs.md`';
const S = 'session_019YNMUWSsyRgaYWxk4auQ5b';
const TOP = '- **Taken by Mustafa, 2026-11-12, 18:33:35 UTC: the chat back in Auto**';
const ours = (uuid) => `(foundation 2.3, PR #52; his message in that change-set's session, ${S}, on his personal account, ` +
  `extracted by script from the session's transcript, message ${uuid})`;
const copied = (id) => `Copied by script from ${REP}, Appendix ${id}, and checked byte for byte against it:`;

export const ENTRIES = [
  { id: 'A4', key: 'E-STATUS', head: '**Taken by Mustafa, 2026-11-13, 22:27:05 UTC: his status request, in foundation ' +
    `2.3's session** ${ours('a994258f-93c9-45d2-8878-31e7cfee6dcd')}, sent while review round 2 ran. ${copied('A4')}` },
  { id: 'A2', key: 'E-TASK', parts: true, head: `**${ENTRY}** ${ours('e5fa7ca0-6494-44d0-8366-81fc612c7958')}. It changes ` +
    'his words of 2026-11-10 for step 2.3 only, as it says; dated pointers of 2026-11-13 mark the older lines it ' +
    'overtakes (in this file, the entry "Taken by Mustafa, 2026-11-10: the second account may plan and build foundation ' +
    'steps 2.1 and 2.2 now" and SLICE-ORDER\'s condition 3, below; SPIKE_PLAN, ADR 0002\'s migration step 1 and ROADMAP; ' +
    'STATUS\'s "Next" quotes it). The kit lines that still name steps 2.1 and 2.2 as the only exception (CLAUDE.md :22 and ' +
    ':55, `.claude/agents/advisor.md` :108, on main 4d21c1a) are not edited by this docs pass: his words of 2026-11-13 are ' +
    `the rule there too, and step 2.3's plan is to list their edits. ${copied('A2')}` },
  { id: 'A1', key: 'E-HI', head: '**Taken by Mustafa, 2026-11-13, 18:29:55 UTC: his message before the task, in foundation ' +
    `2.3's session** ${ours('b031379c-ea4c-47db-b279-54e937bf147a')}. His task message, the entry above, followed at ` +
    `18:47:55 UTC. ${copied('A1')}` },
  { id: 'A3', key: 'E-48', head: '**Taken by Mustafa, 2026-11-13, 16:16:47 UTC: his go for #48** (foundation 2.2, PR #48; ' +
    'given in that change-set\'s session, session_019YNMUSc7FA68ujhLgXfEJA, on his personal account, after the advisor ' +
    'agent\'s APPROVE of 9b20b67; carried out by the merge 4d21c1a; recorded here by foundation 2.3\'s first docs pass, ' +
    'PR #52, as his words in it ask). The lines below are merge commit 4d21c1a\'s record of his message (quoted there with ' +
    '"> ", captured there by script from that session\'s transcript, message 96467900-83d8-4b54-8f8b-7884a0dc0fde): a ' +
    'record of his words, not an instruction found in a commit. Extracted by script from the commit\'s message, where ' +
    're-quoting them gives its block back byte for byte, and checked byte for byte against that session\'s chat record ' +
    '(the same message, read with the sessions API, where its event is stamped 16:16:36 UTC). Its item 5 accepts four ' +
    'security items left open; its item 6 is carried out by foundation 2.3\'s first docs pass (the 2.2 report\'s fixes ' +
    `and this entry). ${copied('A3')}` },
];
const PART1 = '  - His decision, word for word (A2, from inside its quote marks):';
const PART2 = '  - His whole message (A2):';

// The 2026-11-10 entry's header and the sentence it gains.
const H10 = '- **Taken by Mustafa, 2026-11-10: the second account may plan and build foundation steps 2.1 and 2.2 now** (the';
const H10_END = '  - His decision (A1.1):';
const H10_AT = '(the entry "Taken by Mustafa, 2026-11-12, 15:35:17 UTC", above).';
export const OLD_WORDS = 'Steps 2.3 to 2.6 keep their order and wait for their results.';
export const SENTENCE = `Its sentence "${OLD_WORDS}" is changed for step 2.3 only by his decision of 2026-11-13 (the entry ` +
  `"${ENTRY}", above), and the dated pointers of 2026-11-13 carry its limits.`;

function wrap(text, first, rest, width = 120) {
  const out = [];
  let line = first;
  for (const w of text.split(' ')) {
    const cand = line === first ? first + w : `${line} ${w}`;
    if (cand.length > width && line !== first) { out.push(line); line = rest + w; } else line = cand;
  }
  out.push(line);
  return out;
}
const quote = (text, indent = '') => text.split('\n').map((l) => (l === '' ? `${indent}>` : `${indent}> ${l}`));
const norm = (s) => s.replace(/\s+/g, ' ').trim();

function headerBlock(lines) {
  const h = lines.findIndex((l) => l === H10);
  if (h < 0 || lines.filter((l) => l === H10).length !== 1) throw new Error('the 2026-11-10 header is not found once');
  const e = lines.findIndex((l, i) => i > h && l === H10_END);
  if (e < 0 || e - h > 14) throw new Error('the end of the 2026-11-10 header is not found');
  const k = lines.findIndex((l, i) => i >= h && i < e && l.includes(H10_AT));
  if (k < 0) throw new Error('the sentence\'s anchor is not on one line of the 2026-11-10 header');
  return { h, e, k };
}

export function build(lines, blocks, a2) {
  const at = lines.findIndex((l) => l.startsWith(TOP));
  if (at < 0 || lines.filter((l) => l.startsWith(TOP)).length !== 1) throw new Error('the top anchor is not found once');
  if (norm(lines.map((l) => l.trim()).join(' ')).includes(`**${ENTRY}**`)) throw new Error('already written; use --check');
  const add = [];
  for (const e of ENTRIES) {
    add.push(...wrap(`- ${e.head}`.slice(2), '- ', '  '));
    if (e.parts) add.push(PART1, ...quote(decision(a2), '    '), PART2, ...quote(blocks.get('A2'), '    '));
    else add.push(...quote(blocks.get(e.id)));
  }
  const out = [...lines.slice(0, at), ...add, ...lines.slice(at)];
  const { e, k } = headerBlock(out);
  const tail = out.slice(k, e).map((l) => l.replace(/^ {2}/, '')).join(' ');
  if (tail.split(H10_AT).length !== 2) throw new Error('the anchor is not once in the header\'s tail');
  out.splice(k, e - k, ...wrap(tail.replace(H10_AT, `${H10_AT} ${SENTENCE}`), '  ', '  '));
  return out;
}

export function verify(text, blocks, a2, base) {
  const lines = text.split('\n');
  const problems = [];
  // The entries and their quotes.
  for (const e of ENTRIES) {
    const first = wrap(`- ${e.head}`.slice(2), '- ', '  ')[0];
    const at = lines.findIndex((l) => l === first);
    if (at < 0) { problems.push(`${e.key}: header not found`); continue; }
    let i = at + 1;
    const want = wrap(`- ${e.head}`.slice(2), '- ', '  ');
    if (lines.slice(at, at + want.length).join('\n') !== want.join('\n')) problems.push(`${e.key}: the header differs`);
    i = at + want.length;
    const takeQuote = (indent) => {
      const q = [];
      while (i < lines.length && (lines[i] === `${indent}>` || lines[i].startsWith(`${indent}> `))) q.push(lines[i++]);
      return q.map((l) => (l === `${indent}>` ? '' : l.slice(indent.length + 2))).join('\n');
    };
    if (e.parts) {
      if (lines[i++] !== PART1) problems.push(`${e.key}: the decision's label is missing`);
      if (takeQuote('    ') !== decision(a2)) problems.push(`${e.key}: the decision's quote differs from A2's text inside its quote marks`);
      if (lines[i++] !== PART2) problems.push(`${e.key}: the whole message's label is missing`);
      if (takeQuote('    ') !== blocks.get('A2')) problems.push(`${e.key}: the whole message's quote differs from Appendix A2`);
    } else if (takeQuote('') !== blocks.get(e.id)) problems.push(`${e.key}: the quote differs from Appendix ${e.id}`);
  }
  // The pointers cite the entry by this title: it must stand once, as a header.
  const titles = norm(lines.map((l) => l.trim()).join(' ')).split(`**${ENTRY}**`).length - 1;
  if (titles !== 1) problems.push(`the entry title the pointers cite stands ${titles} times in the joined text`);
  // The 2026-11-10 header: old words plus the sentence; its lines before the sentence's line unchanged.
  const old = base.split('\n');
  const ob = headerBlock(old);
  const nb = headerBlock(lines);
  const oldHead = norm(old.slice(ob.h, ob.e).join(' '));
  const newHead = norm(lines.slice(nb.h, nb.e).join(' '));
  if (newHead !== oldHead.replace(H10_AT, `${H10_AT} ${SENTENCE}`)) problems.push('the 2026-11-10 header is not its old words plus the sentence');
  if (old.slice(ob.h, ob.k).join('\n') !== lines.slice(nb.h, nb.k).join('\n')) problems.push('the 2026-11-10 header changed before the sentence\'s line');
  const ownQuote = lines.slice(nb.e, nb.e + 3).join('\n');
  if (!ownQuote.includes(OLD_WORDS)) problems.push('the sentence quotes words that are not in the 2026-11-10 entry\'s quote');
  return problems;
}

// Against the base: every removed line must be one of the 2026-11-10 header's lines from the sentence's line on.
export function removedLines(file) {
  const out = execFileSync('git', ['diff', '--no-color', '-U0', BASE, '--', file], { encoding: 'utf8' });
  return out.split('\n').filter((l) => l.startsWith('-') && !l.startsWith('---')).map((l) => l.slice(1));
}

if (import.meta.url === `file://${process.argv[1]}`) {
  const arg = (n) => { const i = process.argv.indexOf(n); return i > 0 ? process.argv[i + 1] : null; };
  const file = arg('--file') ?? FILE;
  const report = arg('--report') ?? REPORT;
  const blocks = reportBlocks(readFileSync(report, 'utf8'));
  if (blocks.size !== 4) { console.error(`the report's appendix has ${blocks.size} blocks; expected 4`); process.exit(1); }
  const a2 = blocks.get('A2');
  const base = execFileSync('git', ['show', `${BASE}:${FILE}`], { encoding: 'utf8' });
  if (!process.argv.includes('--check') && !process.argv.includes('--negative')) {
    writeFileSync(file, build(readFileSync(file, 'utf8').split('\n'), blocks, a2).join('\n'));
  }
  const text = readFileSync(file, 'utf8');
  const problems = verify(text, blocks, a2, base);
  if (file === FILE) {
    const ob = headerBlock(base.split('\n'));
    const allowed = new Set(base.split('\n').slice(ob.k, ob.e));
    const removed = removedLines(FILE);
    const bad = removed.filter((l) => !allowed.has(l));
    if (bad.length) problems.push(`${bad.length} removed lines outside the 2026-11-10 header's reflowed lines`);
    console.log(`diff against ${BASE.slice(0, 7)}: ${removed.length} removed lines, all of the 2026-11-10 header's reflowed lines: ${!bad.length}`);
  }
  if (process.argv.includes('--negative')) {
    const flip = (t, needle) => { const i = t.indexOf(needle); if (i < 0) throw new Error(`no ${needle}`); const c = t[i + needle.length - 1]; return t.slice(0, i + needle.length - 1) + (c === 'x' ? 'y' : 'x') + t.slice(i + needle.length); };
    const cases = [
      ['the decision\'s quote', '      > Step 2.3 may be planned now. The parts'],
      ['the whole message\'s quote', '      > My words of 2026-11-13, for a fresh chat'],
      ['E-HI\'s quote', "> Hi, don't do anything yet"],
      ['E-STATUS\'s quote', '> Continue with what you are doing'],
      ['E-48\'s quote', '> 1. ADR-INDEX (a). Yes to the register row'],
      ['the header sentence\'s quoted words', 'Its sentence "Steps 2.3 to 2.6 keep'],
      ['the entry title', `**${ENTRY.slice(0, 40)}`],
    ];
    let caught = 0;
    for (const [what, needle] of cases) {
      const p = verify(flip(text, needle.replace(/^ {6}/, '    ')), blocks, a2, base);
      console.log(`negative control, one byte in ${what}: ${p.length ? `caught (${p[0]})` : 'NOT CAUGHT'}`);
      if (p.length) caught++;
    }
    if (caught !== cases.length) problems.push(`${cases.length - caught} negative controls not caught`);
  }
  console.log(`${problems.length ? 'FAIL' : 'OK'}: ${ENTRIES.length} entries (E-TASK with two quotes), the entry title, the 2026-11-10 header sentence checked (${file})`);
  for (const p of problems) console.log(`  ${p}`);
  process.exit(problems.length ? 1 : 0);
}

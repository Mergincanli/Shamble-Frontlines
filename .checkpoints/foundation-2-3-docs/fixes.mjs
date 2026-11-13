// (Q5's 2.1a follow-up text includes round 2's fix pass, revision 2d's D3: the other drafts' lines.)
// Step 3 of the plan: the five report fixes (M1, L1, L2, L3, L5), the header note, Q3's two archive lines and Q5's two
// pointing clauses. Each edit replaces one text inside its paragraph and reflows the paragraph from its own line on, or
// inserts a dated note. Then --check verifies: the 2.2 report's Appendix A byte-identical to the base's; 2.2's
// body-quote check on the report's own blocks; the old texts gone and the new ones present; and the diff hunks against
// the base (six in the report, C6; five in the archive while docs-keeper has not run).
//   node .checkpoints/foundation-2-3-docs/fixes.mjs [--check] [--root <dir>]
import { readFileSync, writeFileSync } from 'node:fs';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { join } from 'node:path';
import { ENTRY } from './pointers.mjs';

const BASE = '4d21c1a7a4d39613bffce5d5f9bbc92f2f88a300';
const REPORT = 'docs/reports/2026-11-12-foundation-2-2.md';
const ARCHIVE = 'docs/STATUS_ARCHIVE.md';
const OLD_GIT = "(the runner's version on 2026-11-10; 2.56.1 since 2026-11-12)";
// Worded as the seven job logs read (l2-git-versions.md): the runners differed on 2026-11-12.
const NEW_GIT = "(the runner's version on 2026-11-10; on 2026-11-12 the runners differed: 2.56.1 on c901187's pull_request " +
  "run at 15:34 UTC and on a5823cf's push run at 18:39 UTC, 2.56.0 on c901187's push run and on fd14855's and 45f1bb5's " +
  'runs up to 18:37 UTC)';
const SHA_NOTES = '2f0fef3b208c10d5c38b921d24fb5f90f71b7ecb';

const HEADER_NOTE = "_Corrected after the merge (2026-11-13) by step 2.3's first docs pass (PR #52), as his go for #48 asks " +
  "(item 6): the advisor agent's report lines from its review of 9b20b67, listed in PR #48's description: M1 (a dated " +
  'correction under "For Mustafa", step 3), L1 and L2 (the verifier paragraph in "Reviews"; L2 also in "Risks" and the ' +
  "CUT-ESCAPE row) and L3 (E12's row); L5 is in `docs/STATUS_ARCHIVE.md`. Appendix A is unchanged, and so is every other " +
  'word: the paragraphs these fixes touch are re-wrapped from the edited line on._';
const M1_NOTE = "_Corrected after the merge (2026-11-13, by step 2.3's first docs pass, PR #52; the advisor agent's M1 on " +
  '9b20b67): four security findings were left open, not one. By main\'s `.claude/agents/advisor.md` (its needs-mustafa ' +
  'triggers, step 4), a security finding not fixed in the PR, a follow-up included, counts as left open, so beside SA2-2 ' +
  "three of round 1's stay open for step 2.3, each a CI edit that asks him: SETPRIV-HARDEN (SA-2), the SETUP-NODE-CACHE " +
  'setting (SA-3) and CUT-ESCAPE (SA-1). His go for #48 accepted all four (item 5; `docs/DECISIONS.md`, 2026-11-13, ' +
  '16:16:47 UTC)._';
const L1_OLD = "Its eight Low notes are fixed in the last commit before ready: the runner's git version (2.56.1 since " +
  '2026-11-12), the PR\'s title and body, OVERFLOW-PANIC-RULE\'s due point in "For Mustafa", PM-0002-53\'s owner in the ' +
  'archive, the stale "last updated" and intro lines and ROADMAP\'s header, the checkpoint (deleted) and a comma in E2.';
const L1_NEW = "Of its eight Low notes, seven are fixed in the last commit before ready: the runner's git version (as worded " +
  "at 9b20b67; L2 corrected it after the merge, in Risks and the CUT-ESCAPE row), the PR's title and body, " +
  "OVERFLOW-PANIC-RULE's due point in \"For " +
  "Mustafa\", PM-0002-53's owner in the archive, the stale \"last updated\" and intro lines and ROADMAP's header, the " +
  "checkpoint (deleted) and a comma in E2; the eighth, that departure (OVERFLOW-XCRATE-CI's recorded line), was named " +
  "for the advisor agent's review of the final head, which accepted it.";
const L3_OLD = "rustc's overflow flag on `sim` only;";
const L3_NEW = "rustc's overflow flag on `sim` only (the saved run did not compile `protocol`, whose own block also sets it);";
const POINT = `(\`docs/DECISIONS.md\`, the entry "${ENTRY}")`;

// kind: 'reflow' (replace inside the paragraph that starts on the line holding `start`, ended before `stop`),
// 'inline' (replace inside one line), 'insert' (new lines after the line equal to `after`).
export const EDITS = [
  { id: 'header note', file: REPORT, kind: 'insert', after: 'the pull request is marked ready; the last commit that holds them is 2f0fef3b208c10d5c38b921d24fb5f90f71b7ecb._', text: HEADER_NOTE, indent: '', blank: true },
  { id: 'L3', file: REPORT, kind: 'inline', start: '| E12 |', old: L3_OLD, repl: L3_NEW },
  { id: 'L1 (with L2)', file: REPORT, kind: 'reflow', start: 'and the mirrored items equal in meaning. Its eight Low notes', stop: (l) => l === '', old: L1_OLD, repl: L1_NEW, cont: '' },
  { id: 'L2 (Risks)', file: REPORT, kind: 'reflow', start: `${OLD_GIT}; not run on the runner), files in the runner's work`, stop: (l) => l.startsWith('- '), old: OLD_GIT, repl: NEW_GIT, cont: '  ' },
  { id: 'L2 (the CUT-ESCAPE row)', file: REPORT, kind: 'inline', start: '| CUT-ESCAPE |', old: OLD_GIT, repl: NEW_GIT },
  { id: 'M1', file: REPORT, kind: 'insert', after: '   first code in `room` or a server crate.', text: M1_NOTE, indent: '   ', blank: false },
  { id: 'L5', file: ARCHIVE, kind: 'reflow', start: "`.checkpoints/foundation-toolchain/`, readable at the branch's last checkpoint commit): their CI runs", stop: (l) => l.startsWith('  - '), old: "readable at the branch's last checkpoint commit):", repl: `readable at the branch's last checkpoint commit, ${SHA_NOTES}):`, cont: '  ' },
  { id: 'Q3 (L2 in the archive)', file: ARCHIVE, kind: 'reflow', start: "actions/checkout's post step runs (shown locally with git 2.43.0 and in the source of git 2.56.0 (the runner's", stop: (l) => l.startsWith('    - '), old: OLD_GIT, repl: NEW_GIT, cont: '      ' },
  { id: 'Q3 (EMPTY-SLOTS)', file: ARCHIVE, kind: 'reflow', start: "- **EMPTY-SLOTS** (2.2's plan, Part 3, readable at the branch's last checkpoint commit;", stop: (l) => l.startsWith('    - '), old: "readable at the branch's last checkpoint commit;", repl: "readable at the branch's last checkpoint commit, 2f0fef3;", cont: '      ' },
  { id: 'Q5 (the 2.1a follow-up)', file: ARCHIVE, kind: 'reflow', start: '  - **For the drafts\' next revision:** five "before the foundation" lines with no pointer to his decision of 2026-11-10', stop: (l) => l.startsWith('  - '), old: "ADR 0002 :38, :724, :1115; SPIKE_PLAN :100 (the diagram's label) and :202.", repl: `ADR 0002 :38, :724, :1129 (:1115 when 2.1a counted); SPIKE_PLAN :106 (the diagram's label) and :214 (:100 and :202 before this PR's pointers); also ADR 0003 :51-52 and ADR 0006 :56-57, each wrapped across two lines; and in the other drafts: ADR 0001 :603, :1100, :1301, :1771; ADR 0003 :1373; ADR 0004 :1245, :1251; ADR 0005 :92; ADR 0006 :1469, :1482; the register (\`docs/adr/README.md\`) :72, :292. Nor to his decision of 2026-11-13 on step 2.3 ${POINT}, whose pointers stand beside 2.1a's (2026-11-13, foundation 2.3's first docs pass).`, cont: '    ' },
  { id: 'Q5 (BRIEF-STALE)', file: ARCHIVE, kind: 'reflow', start: '- **BRIEF-STALE**: `docs/V2_PHASE0_BRIEF.md:20-21`', stop: (l) => l === '', old: 'with no word of the exception for foundation steps 2.1 and 2.2;', repl: `with no word of the exception for foundation steps 2.1 and 2.2, nor of his decision of 2026-11-13 on step 2.3 ${POINT};`, cont: '      ' },
];

export function wrap(text, first, rest, width = 120) {
  const out = [];
  let line = first;
  for (const w of text.split(' ')) {
    const cand = line === first ? first + w : `${line} ${w}`;
    if (cand.length > width && line !== first) { out.push(line); line = rest + w; } else line = cand;
  }
  out.push(line);
  return out;
}
const once = (lines, pred, what) => {
  const hits = lines.map((l, i) => (pred(l) ? i : -1)).filter((i) => i >= 0);
  if (hits.length !== 1) throw new Error(`${what}: ${hits.length} lines match`);
  return hits[0];
};

export function apply(text, e) {
  const lines = text.split('\n');
  if (e.kind === 'insert') {
    const at = once(lines, (l) => l === e.after, e.id);
    const add = wrap(e.text, e.indent, e.indent);
    if (lines.slice(at + 1, at + 1 + add.length + (e.blank ? 1 : 0)).join('\n') === [...(e.blank ? [''] : []), ...add].join('\n')) return { text, done: 'present' };
    lines.splice(at + 1, 0, ...(e.blank ? [''] : []), ...add);
    return { text: lines.join('\n'), done: `inserted ${add.length} lines after line ${at + 1}` };
  }
  const s = once(lines, (l) => l.includes(e.start), e.id);
  if (e.kind === 'inline') {
    if (lines[s].includes(e.repl)) return { text, done: 'present' };
    if (lines[s].split(e.old).length !== 2) throw new Error(`${e.id}: the old text is not once on its line`);
    lines[s] = lines[s].replace(e.old, e.repl);
    return { text: lines.join('\n'), done: `replaced on line ${s + 1}` };
  }
  let end = s + 1;
  while (end < lines.length && !e.stop(lines[end])) end++;
  const first = lines[s].match(/^ */)[0];
  const orig = lines.slice(s, end);
  const joined = orig.map((l) => l.trim()).join(' ');
  if (joined.split(e.old).length !== 2) throw new Error(`${e.id}: the old text is found ${joined.split(e.old).length - 1} times in lines ${s + 1}-${end}`);
  const out = minimalReflow(orig, joined, e.old, e.repl, first, e.cont);
  lines.splice(s, end - s, ...out);
  return { text: lines.join('\n'), done: `reflowed from line ${s + 1} (${orig.length} lines became ${out.length})` };
}

// Greedy-wrap the edited paragraph, but stop as soon as a new line ends where an original line ended, past the edit:
// from there the original lines stand unchanged, so the diff holds only the lines the edit needs.
function minimalReflow(orig, joined, old, repl, first, cont, width = 120) {
  const ow = joined.split(' ');
  const editedText = joined.replace(old, repl);
  const nw = editedText.split(' ');
  const delta = nw.length - ow.length;
  const editEndOrig = joined.slice(0, joined.indexOf(old) + old.length).split(' ').length;
  const bounds = new Map();
  let count = 0;
  orig.forEach((l, i) => { bounds.set(count, i); count += l.trim().split(' ').length; });
  const out = [];
  let line = first;
  let lineStart = 0;
  for (let j = 0; j < nw.length; j++) {
    const cand = line === first || (out.length && line === cont) ? line + nw[j] : `${line} ${nw[j]}`;
    if (cand.length > width && j > lineStart) {
      out.push(line);
      const b = j - delta;
      if (b >= editEndOrig && bounds.has(b) && nw.slice(j).join(' ') === ow.slice(b).join(' ')) return [...out, ...orig.slice(bounds.get(b))];
      line = cont + nw[j];
      lineStart = j;
    } else line = cand;
  }
  out.push(line);
  return out;
}

const sha = (s) => createHash('sha256').update(s).digest('hex');
const appendixOf = (t) => t.slice(t.indexOf("## Appendix A. Mustafa's words for this change-set, in full"));
const norm = (s) => s.replace(/\s+/g, ' ');

export async function verify(root, checkHunks) {
  const problems = [];
  const report = readFileSync(join(root, REPORT), 'utf8');
  const archive = readFileSync(join(root, ARCHIVE), 'utf8');
  const baseReport = execFileSync('git', ['show', `${BASE}:${REPORT}`], { encoding: 'utf8' });
  if (sha(appendixOf(report)) !== sha(appendixOf(baseReport))) problems.push("the 2.2 report's Appendix A differs from the base's");
  const s22 = await import(join(process.cwd(), '.gate/fdn23-docs/s22/report-appendix.mjs'));
  const blocks = s22.reportBlocks(report);
  const r = s22.check(report, [...blocks].map(([id, text]) => ({ id, text })));
  if (r.problems.length) problems.push(...r.problems.map((p) => `2.2's body-quote check: ${p}`));
  if (r.quotes !== 10 || r.blocks !== 13) problems.push(`2.2's body-quote check counted ${r.blocks} blocks and ${r.quotes} quotes (13 and 10 expected)`);
  for (const [t, f] of [[report, REPORT], [archive, ARCHIVE]]) if (norm(t).includes(OLD_GIT)) problems.push(`${f}: "2.56.1 since 2026-11-12" is still there`);
  for (const e of EDITS) {
    const t = e.file === REPORT ? report : archive;
    if (!norm(t).includes(e.kind === 'insert' ? e.text : e.repl)) problems.push(`${e.id}: its new text is missing`);
    if (e.kind !== 'insert' && e.old !== OLD_GIT && norm(t).includes(e.old) && !norm(t).includes(e.repl)) problems.push(`${e.id}: its old text is still there`);
  }
  // Whole-file check: the base, whitespace-normalised, with the edits applied as plain string operations, equals the
  // new file whitespace-normalised: no other word changed (the 2.2 report always; the archive until docs-keeper runs).
  for (const [f, t, always] of [[REPORT, report, true], [ARCHIVE, archive, false]]) {
    if (!always && !checkHunks) continue;
    let b = norm(execFileSync('git', ['show', `${BASE}:${f}`], { encoding: 'utf8' }));
    for (const e of EDITS.filter((x) => x.file === f)) {
      if (e.kind === 'insert') {
        const a = norm(e.after);
        if (b.split(a).length !== 2) { problems.push(`${e.id}: its anchor is not once in the base`); continue; }
        b = b.replace(a, `${a} ${e.text}`);
      } else {
        if (b.split(e.old).length - 1 < 1) { problems.push(`${e.id}: its old text is not in the base`); continue; }
        const at = e.kind === 'inline' ? b.indexOf(e.old, b.indexOf(norm(e.start))) : b.indexOf(e.old);
        b = b.slice(0, at) + e.repl + b.slice(at + e.old.length);
      }
    }
    if (b !== norm(t)) problems.push(`${f}: the file is not the base plus the edits (whole-file check)`);
  }
  if (checkHunks && root === '.') {
    for (const [f, want] of [[REPORT, 6], [ARCHIVE, 5]]) {
      const d = execFileSync('git', ['diff', '--no-color', '-U0', BASE, '--', f], { encoding: 'utf8' });
      const n = d.split('\n').filter((l) => l.startsWith('@@')).length;
      if (n !== want) problems.push(`${f}: ${n} diff hunks against the base, ${want} expected`);
    }
  }
  return { problems, quotes: r.quotes };
}

if (import.meta.url === `file://${process.argv[1]}`) {
  const ri = process.argv.indexOf('--root');
  const root = ri > 0 ? process.argv[ri + 1] : '.';
  if (!process.argv.includes('--check')) {
    const rep = readFileSync(join(root, REPORT), 'utf8');
    if (norm(rep).includes(L1_NEW) || norm(rep).includes(HEADER_NOTE)) { console.error('already written; use --check'); process.exit(1); }
    const texts = new Map();
    for (const e of EDITS) {
      const before = texts.get(e.file) ?? readFileSync(join(root, e.file), 'utf8');
      const r = apply(before, e);
      texts.set(e.file, r.text);
      console.log(`${e.id}: ${r.done}`);
    }
    for (const [f, t] of texts) writeFileSync(join(root, f), t);
  }
  const v = await verify(root, process.argv.includes('--hunks'));
  console.log(`${v.problems.length ? 'FAIL' : 'OK'}: ${EDITS.length} edits checked; the 2.2 report's Appendix A equal to the base's; 2.2's body-quote check (${v.quotes} quotes)`);
  for (const p of v.problems) console.log(`  ${p}`);
  process.exit(v.problems.length ? 1 : 0);
}

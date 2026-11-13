// Review round 2's fix pass (plan revision 2d, the advisor agent's ruling of 2026-11-13 23:02:43 UTC): the tracked
// edits other than the report's text, each an exact replacement found once (fixes.mjs's apply: paragraphs re-wrapped
// only from the edited line until the old line breaks resume), and E-STATUS inserted at the top of DECISIONS.md's
// entries, its quote copied from the report's Appendix A4 (so the report's appendix must be written first, by
// report-appendix.mjs). decisions.mjs --check then verifies all four entries. Run once:
//   node .checkpoints/foundation-2-3-docs/fixpass2.mjs
import { readFileSync, writeFileSync } from 'node:fs';
import { apply } from './fixes.mjs';
import { ENTRIES } from './decisions.mjs';
import { reportBlocks, REPORT } from './report-appendix.mjs';

function wrap(text, first, rest, width = 120) {
  const out = []; let line = first;
  for (const w of text.split(' ')) { const cand = line === first ? first + w : `${line} ${w}`;
    if (cand.length > width && line !== first) { out.push(line); line = rest + w; } else line = cand; }
  out.push(line); return out;
}
const done = [];

// D2 and D3: the archive.
{
  const f = 'docs/STATUS_ARCHIVE.md';
  let t = readFileSync(f, 'utf8');
  for (const e of [
    { id: "D2, #48's row", kind: 'inline', start: '| **foundation 2.2: the toolchain**',
      old: "at 16:16:47 UTC (per the merge commit's message; the chat record's event is stamped 16:16:36 UTC) and quoted whole there",
      repl: "at 16:16:47 UTC by the merge commit's message, which quotes it whole (the chat record's event is stamped 16:16:36 UTC)" },
    { id: 'D3, the 2.1a follow-up', kind: 'reflow', start: '  - **For the drafts\' next revision:** five "before the foundation" lines', stop: (l) => l.startsWith('  - '),
      old: 'each wrapped across two lines. Nor to his decision',
      repl: 'each wrapped across two lines; and in the other drafts: ADR 0001 :603, :1100, :1301, :1771; ADR 0003 :1373; ADR 0004 :1245, :1251; ADR 0005 :92; ADR 0006 :1469, :1482; the register (`docs/adr/README.md`) :72, :292. Nor to his decision', cont: '    ' },
  ]) { const r = apply(t, e); t = r.text; done.push(`archive ${e.id}: ${r.done}`); }
  writeFileSync(f, t);
}

// D4: #52's waiting lines.
{
  const f = 'docs/STATUS.md';
  // Not apply()'s inline kind: the new text is a suffix of the old, which apply() would take as already present.
  const lines = readFileSync(f, 'utf8').split('\n');
  const at = lines.findIndex((l) => l.startsWith('| #52 |'));
  const OLD = "review round 1 (two reviewers), the verifier, the advisor agent's review of its final head; then his go (risk tier 1, decision docs)";
  const NEW = "the verifier, the advisor agent's review of its final head; then his go (risk tier 1, decision docs)";
  if (at < 0 || lines.filter((l) => l.startsWith('| #52 |')).length !== 1) throw new Error("STATUS: #52's row not found once");
  if (lines[at].split(OLD).length === 2) { lines[at] = lines[at].replace(OLD, NEW); done.push(`STATUS #52's row: replaced on line ${at + 1}`); }
  else if (lines[at].includes(`| ${NEW} |`)) done.push("STATUS #52's row: present");
  else throw new Error("STATUS: #52's waiting cell is neither the old nor the new text");
  writeFileSync(f, lines.join('\n'));
}
{
  const f = 'docs/ROADMAP.md';
  const r = apply(readFileSync(f, 'utf8'), { id: "D4, ROADMAP #52's Next", kind: 'reflow', start: '**The foundation, 2.3: the first docs pass [2026-11-13, PR #52: in review]**', stop: (l) => l === '',
    old: "Next: review round 1, the verifier, the advisor agent's review of the final head, then his go.",
    repl: "Next: the verifier, the advisor agent's review of the final head, then his go.", cont: '' });
  writeFileSync(f, r.text); done.push(`ROADMAP #52's Next: ${r.done}`);
}

// D6(a1): E-STATUS at the top of the entries, before E-TASK, in decisions.mjs's form.
{
  const f = 'docs/DECISIONS.md';
  const lines = readFileSync(f, 'utf8').split('\n');
  const e = ENTRIES.find((x) => x.key === 'E-STATUS');
  const head = wrap(`- ${e.head}`.slice(2), '- ', '  ');
  if (lines.includes(head[0])) throw new Error('E-STATUS is already written; use decisions.mjs --check');
  const a4 = reportBlocks(readFileSync(REPORT, 'utf8')).get('A4');
  if (!a4) throw new Error("the report's appendix has no A4 block: run report-appendix.mjs first");
  const task = lines.findIndex((l) => l.startsWith('- **Taken by Mustafa, 2026-11-13, 18:47:55 UTC: his decision on step 2.3'));
  if (task < 0 || lines.filter((l) => l.startsWith('- **Taken by Mustafa, 2026-11-13, 18:47:55 UTC: his decision on step 2.3')).length !== 1) throw new Error('E-TASK not found once');
  const quote = a4.split('\n').map((l) => (l === '' ? '>' : `> ${l}`));
  lines.splice(task, 0, ...head, ...quote);
  writeFileSync(f, lines.join('\n'));
  done.push(`DECISIONS: E-STATUS inserted before line ${task + 1} (${head.length} header lines, ${quote.length} quote line)`);
}
console.log(done.join('\n'));

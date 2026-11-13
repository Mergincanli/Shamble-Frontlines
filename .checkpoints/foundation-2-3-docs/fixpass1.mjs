// Review round 1's fix pass (plan revision 2c, the advisor agent's ruling): the doc edits, each an exact replacement
// found once, paragraphs re-wrapped only from the edited line until the old line breaks resume (fixes.mjs's apply).
// Updates first, in the scripts, the texts their --check verifies: decisions.mjs (E-TASK's head), fixes.mjs (L1_NEW),
// pointers.mjs (QKIT's last anchor). Run once:  node .checkpoints/foundation-2-3-docs/fixpass1.mjs
import { readFileSync, writeFileSync } from 'node:fs';
import { apply } from './fixes.mjs';
import { kitWords, wrap as wrapSame } from './pointers.mjs';
import { ENTRIES } from './decisions.mjs';

function wrap(text, first, rest, width = 120) {
  const out = []; let line = first;
  for (const w of text.split(' ')) { const cand = line === first ? first + w : `${line} ${w}`;
    if (cand.length > width && line !== first) { out.push(line); line = rest + w; } else line = cand; }
  out.push(line); return out;
}
const done = [];

// 1. E-TASK's header in DECISIONS.md (2c item 1), from decisions.mjs's head (already updated there).
{
  const lines = readFileSync('docs/DECISIONS.md', 'utf8').split('\n');
  const e = ENTRIES.find((x) => x.key === 'E-TASK');
  if (!e.head.includes('his words of 2026-11-13 are the rule there too')) throw new Error('decisions.mjs head not updated');
  const start = lines.findIndex((l) => l.startsWith('- **Taken by Mustafa, 2026-11-13, 18:47:55 UTC: his decision on step 2.3'));
  const end = lines.findIndex((l, i) => i > start && l === '  - His decision, word for word (A2, from inside its quote marks):');
  if (start < 0 || end < 0 || end - start > 14) throw new Error('E-TASK header block not found');
  if (!lines.slice(start, end).map((l) => l.trim()).join(' ').includes("his words are the rule there too, and step 2.3's plan lists their edits.")) throw new Error('E-TASK: the old wording is not there');
  lines.splice(start, end - start, ...wrap(`- ${e.head}`.slice(2), '- ', '  '));
  writeFileSync('docs/DECISIONS.md', lines.join('\n'));
  done.push('DECISIONS: E-TASK header');
}

// 2. The 2.2 report's L1 sentence (2c item 2).
{
  const f = 'docs/reports/2026-11-12-foundation-2-2.md';
  const r = apply(readFileSync(f, 'utf8'), { id: 'L1 attribution', kind: 'reflow', start: 'and the mirrored items equal in meaning. Of its eight Low notes', stop: (l) => l === '',
    old: "the runner's git version (on 2026-11-12, 2.56.0 or 2.56.1 by runner)",
    repl: "the runner's git version (as worded at 9b20b67; L2 corrected it after the merge, in Risks and the CUT-ESCAPE row)", cont: '' });
  writeFileSync(f, r.text);
  done.push(`2.2 report: L1 (${r.done})`);
}

// 3. STATUS's kit quote to the end of his paragraph (2c item 3); 4. STATUS's main line (2c item 4).
{
  const f = 'docs/STATUS.md';
  const lines = readFileSync(f, 'utf8').split('\n');
  const s = lines.findIndex((l) => l.startsWith('  "The kit\'s freeze (KIT-FREEZE-2) ends with this step.'));
  let e = s;
  while (e < lines.length && !lines[e].endsWith('not my yes."')) e++;
  if (s < 0 || e - s > 4) throw new Error('STATUS: the kit quote is not found');
  const k = kitWords();
  if (!k.endsWith('No such edit happens in the docs pass or while planning.')) throw new Error('pointers.mjs QKIT not updated');
  lines.splice(s, e - s + 1, ...wrapSame(`"${k}"`, '  '));
  let t = lines.join('\n');
  const r = apply(t, { id: 'STATUS main', kind: 'reflow', start: '- **main** is protected (a pull request with the `gate` and `smoke` checks green)', stop: (l) => l.startsWith('- ') || l === '' || l.startsWith('#'),
    old: 'is not a required check until step 2.3 (REQUIRED-CHECK)', repl: 'is not a required check (REQUIRED-CHECK, a finding for step 2.3)', cont: '  ' });
  writeFileSync(f, r.text);
  done.push(`STATUS: the kit quote; the main line (${r.done})`);
}

// 5. The archive (2c item 5, a to d).
{
  const f = 'docs/STATUS_ARCHIVE.md';
  let t = readFileSync(f, 'utf8');
  for (const e of [
    { id: '(a) BRIEF-STALE date', kind: 'reflow', start: '- **BRIEF-STALE**: `docs/V2_PHASE0_BRIEF.md:20-21`', stop: (l) => l === '', old: '[verified 2026-11-12]', repl: '[verified 2026-11-13]', cont: '      ' },
    { id: '(b) the 2.1a follow-up', kind: 'reflow', start: '  - **For the drafts\' next revision:** five "before the foundation" lines', stop: (l) => l.startsWith('  - '),
      old: "ADR 0002 :38, :724, :1115; SPIKE_PLAN :100 (the diagram's label) and :202.",
      repl: "ADR 0002 :38, :724, :1129 (:1115 when 2.1a counted); SPIKE_PLAN :106 (the diagram's label) and :214 (:100 and :202 before this PR's pointers); also ADR 0003 :51-52 and ADR 0006 :56-57, each wrapped across two lines.", cont: '    ' },
    { id: '(c) REQUIRED-CHECK', kind: 'reflow', start: 'for #48 itself, the session reads the two `toolchain` runs (push and pull_request)', stop: (l) => l.startsWith('    - '),
      old: 'for #48 itself, the session reads the two `toolchain` runs (push and pull_request) before it merges (the report\'s "For Mustafa", item 3).',
      repl: "for #48 itself, the session read the two `toolchain` runs (push 43452386039, pull_request 43452396473) before it merged (merge commit 4d21c1a's message).", cont: '      ' },
    { id: "(d) #48's row", kind: 'inline', start: '| **foundation 2.2: the toolchain**', old: "at 16:16:47 UTC and quoted whole in the merge commit's message",
      repl: "at 16:16:47 UTC (per the merge commit's message; the chat record's event is stamped 16:16:36 UTC) and quoted whole there" },
  ]) { const r = apply(t, e); t = r.text; done.push(`archive ${e.id}: ${r.done}`); }
  writeFileSync(f, t);
}
console.log(done.join('\n'));

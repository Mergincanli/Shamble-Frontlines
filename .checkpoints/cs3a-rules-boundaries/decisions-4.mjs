// The docs pass of change-set 3a: writes Mustafa's chat messages since his entry of 2026-11-23, 21:24:59 UTC into
// docs/DECISIONS.md, newest first. His words are never retyped: they come from the files chat-words.mjs saved from
// the chat's transcript, each with its byte count and sha256, and every quote of his inside a lead is checked
// against those saved words. Made from decisions-3.mjs as it stands at 5911b56 (the same helpers and entry form).
// The file is always rebuilt from BEFORE (main's copy), so a second run writes the same bytes. From the repository root:
//   node .checkpoints/cs3a-rules-boundaries/decisions-4.mjs [--check] [--out <file>]
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';

const DEC = 'docs/DECISIONS.md';
const at = process.argv.indexOf('--out');
const OUT = at === -1 ? DEC : process.argv[at + 1];
const NOTES = '.checkpoints/cs3a-rules-boundaries/';
const BEFORE = '1ec6f6c3852d2922989f299bfc053c403d0e52d3';
const V3 =
  'the builder chat he calls Builder V3, session_01WLuqnvpAh9XTitx8ehKJcQ, a Claude Code chat on the account that also holds his claude.ai project "Shamble Frontlines ALT"';
const sha = (s) => createHash('sha256').update(s).digest('hex');
const quote = (s) => s.split('\n').map((l) => (l === '' ? '>' : `> ${l}`)).join('\n');
const wrap = (text, rest = '  ') => {
  const out = [];
  let line = '';
  let empty = true;
  for (const w of text.split(' ')) {
    if (!empty && line.length + 1 + w.length > 120) {
      out.push(line);
      line = rest + w;
    } else {
      line += (empty ? '' : ' ') + w;
      empty = false;
    }
  }
  out.push(line);
  return out.join('\n');
};
function saved(file) {
  const note = readFileSync(NOTES + file, 'utf8');
  const cut = note.indexOf('\n---\n');
  if (cut < 0) throw new Error(`no --- in ${file}`);
  const head = note.slice(0, cut);
  const words = note.slice(cut + 5).replace(/\n$/, '');
  const count = head.match(/\((\d+) bytes, sha256 ([0-9a-f]{64})\)/);
  if (!count || Number(count[1]) !== Buffer.byteLength(words) || count[2] !== sha(words)) throw new Error(`${file}: the words differ from its own count or sha256`);
  const id = head.match(/\(uuid ([0-9a-f-]{36})\)/);
  const answers = head.match(/What it answers: ([\s\S]*)_\s*$/);
  if (!id || !answers) throw new Error(`${file}: no uuid or no "What it answers" in its header`);
  return { words, bytes: Buffer.byteLength(words), sha256: sha(words), id: id[1], answers: answers[1].trim() };
}
const said = (m, s) => {
  if (!m.words.includes(s)) throw new Error(`not in his saved words: ${s}`);
  return `"${s}"`;
};

// Newest first. Each lead is the header's text after "Taken by Mustafa, <when>: "; `records` holds what the
// repository's records show, marked as not his words.
const ENTRIES = [
  {
    when: '2026-11-24, 21:48:49 UTC',
    file: 'mustafa-2026-11-24T214849-browser-check.md',
    pointer: (m) =>
      `_A pointer, not a decision: **his browser check of change-set 3a's zip, 2026-11-24, 21:48:49 UTC.** One chat message (${V3}; his message ${m.id}; ${m.bytes} bytes of text, sha256 ${m.sha256}) with three "Copy report" texts, two from his own browser (Brave) and one from Firefox on his Windows computer, and two pictures. It stands whole, copied by script, in \`docs/reports/2026-11-24-cs3a-browser-check.md\`, Appendix A, with the build chat's reading of it against the zip's README: errors 0 in all three reports; the match stands still while the page is hidden (${said(m, 'lastPauseTicksWhileHidden: 0')}); Brave Shields was up; the page opened as a file shows its one line of text. One line is not what the README said to expect: the last pause reads 16.5 s, not about 60 s (six pauses in all; only the last one's length is in a report). In his words on the covered window: ${said(m, 'When I placed the thing explorer window in front of it: it dropped to 179 fps briefly')}. What it answers, in the chat's words: ${m.answers}_`,
  },
  {
    when: '2026-11-24, 15:58:25 UTC',
    file: 'mustafa-2026-11-24T155825-mul-add-a.md',
    title:
      'MUL-ADD (a): the fused multiply-add stays banned in the simulation, which writes a * b + c; the check on the built file comes with the first outside crate; the stop line of 4,100 lines stays; four decisions of his of 24 November (no separate animation editor; Testlines and the level editor; 3b first in line; the VFX repository stays separate for now); plain words with him from now on',
    lead: (m) =>
      `His answer: ${said(m, 'MUL-ADD: (a). The fused multiply-add stays banned in the simulation; it writes a * b + c.')} What stays: ${said(m, 'What 3a built for it stays as it is (the one-name ban, the 48 hard cases, exact natively, the 16 WebAssembly values pinned as a known toolchain defect).')} On the draft: ${said(m, 'No word of ADR 0004 changes in 3a; part 7 gets its dated pointer at its next revision, with its exact sentence shown to me first.')} His Advisor's two conditions, for the report and STATUS's follow-ups: ${said(m, 'The check on the built file (no fused multiply-add inside the players\' WebAssembly) is built in the same change-set that brings the first outside crate into the simulation (the collision library, planned for change-set 4), not later. That change-set\'s plan names it.')}; and reporting the defect to the Rust project is a follow-up: ${said(m, 'Nothing is posted anywhere without my go.')} The size: ${said(m, 'The stop line of 4,100 lines stays. If round 2\'s fixes would pass it, cut by the plan\'s ladder and tell me in three lines.')} His four decisions of 24 November, of which he says ${said(m, 'They change nothing in 3a')}: ${said(m, 'No separate animation editor. Rigging and animation happen in Blender, with the Blender MCP.')}; ${said(m, 'Testlines stays for feel and metrics. The real level editor goes in the main game (Editor V2).')}; ${said(m, 'The level editor can start against the offline core, but 3b stays first in line for the main builder.')}; ${said(m, 'VFX repo stays separate for now; we merge later. Both sides should agree on one effect file format, so the merge is easy.')} On how the chat writes to him: ${said(m, 'From now on use plain words with me: no tag or code without its explanation in the same sentence.')} On the pull request: ${said(m, 'When the pull request\'s title and text are in your notes folder, tell me, and my main Advisor opens the draft.')}`,
    records:
      'the question followed the advisor agent\'s interim ruling of the same day (the ban and the pinned values built meanwhile, the rule left to him); at his answer they stood in commit 79666ad9c8ac3a8d756f874873c820b1c178bc35, and nothing in ADR 0004 or the research was changed; his four decisions touch no file of change-set 3a',
  },
  {
    when: '2026-11-24, 11:03:53 UTC',
    file: 'mustafa-2026-11-24T110353-compact-done.md',
    title: 'the chat is compacted: rebuild from the repository, then build change-set 3a by the plan with no further word from him',
    lead: (m) =>
      `His steps for the rebuild: ${said(m, 'Read CLAUDE.md, change-set 3a\'s resume card, the plan it names (cs3-plan.md, draft 3) and my words of 23 and 24 November in docs/DECISIONS.md and in the notes.')} and ${said(m, 'Check git status and your branch against origin and main. Read your session\'s model and mode.')} His stop: ${said(m, 'If the card, the repo and the summary disagree, or something is missing, say what and stop.')} His word to build: ${said(m, 'Otherwise build change-set 3a by the plan without waiting for me.')}`,
    records:
      'the chat answered in six lines (model `claude-fable-5-2`, permission mode `auto` on the session record, context about 10 %; card, repository and summary agreed) and built change-set 3a; nothing in 3a is a kit, gate or CI write',
  },
  {
    when: '2026-11-24, 10:50:43 UTC',
    file: 'mustafa-2026-11-24T105043-compact-command.md',
    title: 'his /compact of the builder chat, with what its summary must keep',
    lead: (m) =>
      `What the summary must keep, in his words: ${said(m, 'change-set 3a\'s resume card (its path and commit) is the resume source')}; ${said(m, 'my words of 23 and 24 November in docs/DECISIONS.md and in the notes bind you (ANSWERS-7, BUILD-SEAT, FIREFOX-TESTS (b), GH-TOOLS and GH-NEXT, the mode rule of 17:15:07 UTC, LONG-RUNS)')}; ${said(m, 'your branch is claude/loving-hopper-6p0mg2 and this chat cannot write to GitHub')}.`,
    records: 'the resume card was `.checkpoints/cs3a-rules-boundaries.md` at 8a62e7904c2f2f664c8cd2f22277297625197b92',
  },
  {
    when: '2026-11-24, 10:16:15 UTC',
    file: 'mustafa-2026-11-24T101615-merged.md',
    title: '#64 is merged, by his main Advisor on his tap, with the expected head 8aadfb0; the chat carries on with point 4 of his block of 23:44:10 UTC',
    lead: (m) =>
      `In his words: ${said(m, 'I did not press the buttons myself: on my tap in my main Advisor\'s thread, the Advisor marked it ready and merged it with the expected head 8aadfb09640c9d37a3e6ad8be078165f5074f399.')} And: ${said(m, 'The merge commit\'s message records the tap.')}`,
    records:
      'main is 1ec6f6c3852d2922989f299bfc053c403d0e52d3 (2026-11-24, 10:15:31 UTC; its parents f5030a3 and 8aadfb0); gate, smoke and toolchain were green on it (read 10:17 UTC); #63 and #62 are closed',
  },
  {
    when: '2026-11-23, 23:51:57 UTC',
    file: 'mustafa-2026-11-23T235157-ready-then-merge.md',
    title: 'his short message while #64 still showed as a draft',
    lead: () =>
      'The chat could not tell whether it reported what he had done or asked the chat to do it, did neither (it cannot write to GitHub), and asked; his message of 2026-11-24, 10:16:15 UTC (the entry above) says what happened.',
    records: 'nothing was written or pushed on this message',
  },
  {
    when: '2026-11-23, 23:44:10 UTC',
    file: 'mustafa-2026-11-23T234410-go-64.md',
    title:
      'MERGE-64 (a), his go for #64 at 8aadfb0; GH-NEXT (a) with one change, for change-set 3a: the chat writes each pull request\'s title and text into the notes, his main Advisor opens the draft and sets them, he presses Ready and Merge; CLOSE-OLD (a); the order of work after the merge',
    lead: (m) =>
      `His go: ${said(m, 'MERGE-64: (a), go for 8aadfb09640c9d37a3e6ad8be078165f5074f399.')} On later pull requests: ${said(m, 'GH-NEXT: (a) for change-set 3a, with one change: you put each pull request\'s title and text in the notes folder, my main Advisor opens the draft and sets them, and I press Ready and Merge. (c) is planned with the gate step, not now.')} On the old drafts: ${said(m, 'CLOSE-OLD: (a). #63 and #62 are closed once #64 is merged; my Advisor or I do that.')} His point 4 ends: ${said(m, 'Build nothing before I have compacted this chat.')}`,
    records:
      'the advisor agent had approved 8aadfb09640c9d37a3e6ad8be078165f5074f399 with gate, smoke and toolchain green on it; the merge followed on 2026-11-24 (the entry of 10:16:15 UTC above)',
  },
];

const before = execFileSync('git', ['show', `${BEFORE}:${DEC}`], { encoding: 'utf8', maxBuffer: 64 * 1024 * 1024 });
const MARK = '## Decisions, newest first\n';
if (before.split(MARK).length !== 2) throw new Error('the heading "Decisions, newest first" must stand once');
const blocks = ENTRIES.map((entry) => {
  const m = saved(entry.file);
  if (entry.pointer) return `${wrap(`- ${entry.pointer(m)}`)}\n`;
  const header = `- **Taken by Mustafa, ${entry.when}: ${entry.title}** (${V3}; his message ${m.id}, whole; extracted by script from the chat's transcript). ${entry.lead(m)} What it answers, in the chat's words (not his): ${m.answers} Records, not his words: ${entry.records}. Copied by script (${m.bytes} bytes, sha256 ${m.sha256}):`;
  return `${wrap(header)}\n${quote(m.words)}\n`;
});
const after = before.replace(MARK, MARK + blocks.join(''));
if (process.argv.includes('--check')) {
  const now = readFileSync(DEC, 'utf8');
  console.log(now === after ? `${DEC} is what this script writes (${ENTRIES.length} entries on top of ${BEFORE.slice(0, 7)}'s)` : `${DEC} differs from what this script writes`);
  process.exit(now === after ? 0 : 1);
}
writeFileSync(OUT, after);
console.log(`${OUT}: ${ENTRIES.length} entries added on top of ${BEFORE.slice(0, 7)}'s, ${after.split('\n').length - before.split('\n').length} lines`);

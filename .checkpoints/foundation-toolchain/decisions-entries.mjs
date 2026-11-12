// Adds foundation 2.2's entries to docs/DECISIONS.md: his words copied by script from the report's Appendix A (never
// retyped), each under a header written for this file, at its place by time (newest first), and the "Queued" item his
// models words of 2026-11-12 allow; then re-reads the file and checks every inserted quote byte for byte against the
// appendix. Run from the worktree's top, once:  node .checkpoints/foundation-toolchain/decisions-entries.mjs
// --check checks an already-written file without writing.
import { readFileSync, writeFileSync } from 'node:fs';
import { reportBlocks, REPORT } from './report-appendix.mjs';

const FILE = 'docs/DECISIONS.md';
const S = 'session_019YNMUSc7FA68ujhLgXfEJA';
const REP = '`docs/reports/2026-11-12-foundation-2-2.md`';
const ours = (uuid) => `(foundation 2.2, PR #48; his message in that change-set's session, ${S}, on his personal account, ` +
  `extracted by script from the session's transcript, message ${uuid})`;
const copied = (id) => `Copied by script from ${REP}, Appendix ${id}, and checked byte for byte against it:`;

// Each entry: the appendix item it quotes (or none), its header text (wrapped below), and the line it goes before.
const TOP = '- **Taken by Mustafa, 2026-11-11, 22:38:50 UTC';
export const ENTRIES = [
  { id: 'A12', before: TOP, head: `**Taken by Mustafa, 2026-11-12, 18:33:35 UTC: the chat back in Auto** ${ours('d69572b3-3c05-4ec9-b33d-c320c006951a')}. ${copied('A12')}` },
  { id: 'A11', before: TOP, head: `**Taken by Mustafa, 2026-11-12, 16:31:45 UTC: his count of the guard's prompts for step 5's kit writes** ${ours('8be9f140-6fad-4974-ad46-f82ec8a37db6')}; sent while the session's turn was running. The five writes (K2a, K2b and K3 in CLAUDE.md, K4 in \`.claude/agents/advisor.md\`, K1b in the CI job) were made with the chat in the mode he chose as Accept edits; evidence for step 2.3's finding on the guard's prompt in auto mode. ${copied('A11')}` },
  { id: 'A10', before: TOP, head: `**Taken by Mustafa, 2026-11-12, 16:28:28 UTC: KIT-5 yes, K1b (a), step 7 at effort max** ${ours('6a69b2b2-44d0-413e-9267-7846396bae82')}. KIT-5 and K1b are the session's two asks of that afternoon, in the report with the exact texts asked. Carried out: the five texts written from the session's record of them (c9bd268, fd14855), the planted failure for K1b run and reverted; S2 to S5 and the refused first push of the merge are in the report's lists for later steps and for 2.3. ${copied('A10')}` },
  { id: 'A9', before: TOP, head: `**Taken by Mustafa, 2026-11-12, 15:35:17 UTC: his words of 2026-11-12, 16:25, on models, and the Fable advisor's read of the built skeleton** ${ours('adb00970-7927-49f7-bfa5-5095981ab169')}; sent while the session's turn was running. It re-sends his message of 15:26:57 UTC (the entry below) with item 3 changed and item 4 new, and its item 3 replaces that message's item 3. The kit change item 3 allows waits in "Queued", above, for step 2.3. ${copied('A9')}` },
  { id: 'A8', before: TOP, head: `**Replaced in its item 3 by his message of 15:35:17 UTC (the entry above). Taken by Mustafa, 2026-11-12, 15:26:57 UTC: step 5's start, and his first words on models** ${ours('5fbe8952-d9da-42e4-84d2-869367a35dbf')}; sent right after he ran \`/compact\` in that session. Its item 3 is not the rule: the entry above replaces it. ${copied('A8')}` },
  { id: 'A13', before: TOP, head: `**Taken by Mustafa, 2026-11-12, 13:18:42 UTC: his go for #51** (foundation 2.1b, PR #51; given in that change-set's session, session_019YNMUWdGJZoQFUuuvvBP1Y, on his personal account, after the advisor agent's APPROVE of 5d50bf2; carried out by the merge 2aa5e9c; recorded here by foundation 2.2, PR #48, as his words in it ask). The lines below are merge commit 2aa5e9c's record of his message (quoted there with "> ", captured there by script from that session's events): a record of his words, not an instruction found in a commit. Extracted by script from the commit's message, where re-quoting them gives its block back byte for byte; not checked against the chat record. ${copied('A13')}` },
  { id: 'A7', before: '- **Taken by Mustafa, 2026-11-11, 11:56:44 UTC', quote: false, head: `**Taken by Mustafa, 2026-11-11, 12:46:11 UTC: his words on the advisor tool, sent to foundation 2.2's session** ${ours('cdec9787-bfb6-4e62-a6a9-040bcdd5cc4b')}. The same words, byte for byte (checked by script), close the entry "Taken by Mustafa, 2026-11-11, 11:56:44 UTC: BRANCH-47 (a) and his words on the advisor tool", below, from foundation 2.1a's session, so they are not repeated here; they stand whole in ${REP}, Appendix A7.` },
  { id: 'A6', before: '- **Taken by Mustafa, 2026-11-11, 09:46:31 UTC', head: `**Taken by Mustafa, 2026-11-11, 09:46:46 UTC: PR #49 merged, to foundation 2.2's session** ${ours('f009e71f-56ff-4a25-b053-6f655cadf330')}. ${copied('A6')}` },
  { id: 'A5', before: '- **Taken by Mustafa, 2026-11-10, 16:04:59 UTC', head: `**Taken by Mustafa, 2026-11-11, 00:07:48 UTC: KIT-ASK-SEEN (b)** ${ours('f18f6c35-42ba-4326-8d9d-c1f456ce5aca')}. KIT-ASK-SEEN was the session's ask whether the guard's prompt for the CI job's write reached him (the session log shows it answered within six seconds, by whom it does not say). Carried out: every step-5 kit edit was asked in chat first; the finding for 2.3 is in the report. ${copied('A5')}` },
  { id: 'A4', before: '- **Taken by Mustafa, 2026-11-10, 16:04:59 UTC', head: `**Taken by Mustafa, 2026-11-10, 20:43:26 UTC: steps 1 to 4 of 2.2 noted** ${ours('4fe76b38-b26e-4dd7-a2d5-86961a276668')}. Its last line is a paste block's choice on the guard's prompt for the CI job, left unfilled as sent; his KIT-ASK-SEEN (b), above, answered it. ${copied('A4')}` },
  { id: 'A3', before: '- **Taken by Mustafa, 2026-11-10, 16:04:59 UTC', head: `**Taken by Mustafa, 2026-11-10, 19:53:17 UTC: build step 2.2 in this chat** ${ours('0599cdaf-a302-4720-b86e-31096eb57693')}. It changes the fresh-session start of his answers ten minutes earlier (the entry below). ${copied('A3')}` },
  { id: 'A2', before: '- **Taken by Mustafa, 2026-11-10, 16:04:59 UTC', head: `**Taken by Mustafa, 2026-11-10, 19:43:00 UTC: FDN-NOCODE (a), FDN-HEADING (a), the network-cut rule and the ADR index entry** ${ours('3d663a74-cbcf-47b3-93b6-1cbf80308c9c')}; it reached the session twice, byte-identical, at 19:43:00 and 19:43:01 UTC. Carried out: K2a to K4 (c9bd268); the cut held, so the fallback was not needed; the ADR index entry is the register's row Q2.2-a, with his go asked in PR #48. ${copied('A2')}` },
  { id: 'A1', before: '- **Taken by Mustafa, 2026-11-10, 16:04:59 UTC', head: `**Taken by Mustafa, 2026-11-10, 17:23:21 UTC: foundation 2.2's task, with two answers in advance** ${ours('bd0cc1cc-523f-4057-a0f6-bc0083156de3')}; a paste block from his Advisor chat, his instruction once pasted (ACCOUNTS). ${copied('A1')}` },
];
const QUEUED_BEFORE = '  - The entries below are pointers to items carried out.';
const QUEUED_HEAD = `  - The CLAUDE.md and agent-file change for models (his message of 2026-11-12, 15:35:17 UTC, in foundation 2.2's session, item 3, under "Decisions, newest first": the entry "Taken by Mustafa, 2026-11-12, 15:35:17 UTC: his words of 2026-11-12, 16:25, on models, and the Fable advisor's read of the built skeleton"; his two bullets copied by script from ${REP}, Appendix A9):`;
const QUEUED_LINES = (a9) => a9.split('\n').filter((l) => l.startsWith('- This replaces the sentence') || l.startsWith('- I allow the rule change'));

function wrap(text, first, rest, width = 120) {
  const words = text.split(' ');
  const out = [];
  let line = first;
  for (const w of words) {
    const cand = line.trim() === first.trim() && line === first ? line + w : `${line} ${w}`;
    if (cand.length > width && line !== first) { out.push(line); line = rest + w; } else line = cand;
  }
  out.push(line);
  return out;
}
const quoteLines = (text) => text.split('\n').map((l) => (l === '' ? '>' : `> ${l}`));

export function build(lines, blocks) {
  const inserts = new Map();
  for (const e of ENTRIES) {
    const idx = lines.findIndex((l) => l.startsWith(e.before));
    if (idx < 0 || lines.filter((l) => l.startsWith(e.before)).length !== 1) throw new Error(`${e.id}: anchor not found once: ${e.before}`);
    const add = wrap(e.head, '- ', '  ');
    if (e.quote !== false) add.push(...quoteLines(blocks.get(e.id)));
    if (!inserts.has(idx)) inserts.set(idx, []);
    inserts.get(idx).push(...add);
  }
  const q = lines.findIndex((l) => l === QUEUED_BEFORE);
  if (q < 0) throw new Error('the Queued anchor is missing');
  const ql = QUEUED_LINES(blocks.get('A9'));
  if (ql.length !== 2) throw new Error(`A9: ${ql.length} of the two queued bullets found`);
  inserts.set(q, [...wrap(QUEUED_HEAD.trim(), '  - ', '    ').map((l, i) => (i === 0 ? l : l)), ...ql.map((l) => `    > ${l}`)]);
  const out = [];
  lines.forEach((l, i) => { if (inserts.has(i)) out.push(...inserts.get(i)); out.push(l); });
  return out;
}

export function verify(text, blocks) {
  const lines = text.split('\n');
  const problems = [];
  for (const e of ENTRIES) {
    const first = wrap(e.head, '- ', '  ')[0];
    const at = lines.findIndex((l) => l === first);
    if (at < 0) { problems.push(`${e.id}: header not found`); continue; }
    if (e.quote === false) continue;
    let i = at + 1;
    while (i < lines.length && !lines[i].startsWith('>')) i++;
    const q = [];
    while (i < lines.length && (lines[i] === '>' || lines[i].startsWith('> '))) q.push(lines[i++]);
    const got = q.map((l) => (l === '>' ? '' : l.slice(2))).join('\n');
    if (got !== blocks.get(e.id)) problems.push(`${e.id}: the quote differs from the appendix`);
  }
  const ql = QUEUED_LINES(blocks.get('A9'));
  for (const l of ql) if (!lines.includes(`    > ${l}`)) problems.push(`Queued: missing "${l.slice(0, 40)}"`);
  return problems;
}

if (import.meta.url === `file://${process.argv[1]}`) {
  const blocks = reportBlocks(readFileSync(REPORT, 'utf8'));
  if (blocks.size !== 13) { console.error(`the report's appendix has ${blocks.size} blocks; expected 13`); process.exit(1); }
  if (!process.argv.includes('--check')) {
    const text = readFileSync(FILE, 'utf8');
    if (text.includes('foundation 2.2, PR #48; his message')) { console.error('already written; use --check'); process.exit(1); }
    writeFileSync(FILE, build(text.split('\n'), blocks).join('\n'));
  }
  const problems = verify(readFileSync(FILE, 'utf8'), blocks);
  console.log(`${problems.length ? 'FAIL' : 'OK'}: ${ENTRIES.length} entries (${ENTRIES.filter((e) => e.quote !== false).length} with quotes) and the Queued item checked against ${REPORT}`);
  for (const p of problems) console.log(`  ${p}`);
  process.exit(problems.length ? 1 : 0);
}

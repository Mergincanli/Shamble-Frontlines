// Writes the report's "Appendix A. Mustafa's words for this change-set, in full" from the sources (words-sources.mjs:
// his 12 messages from this session's transcript, his go for #51 from merge commit 2aa5e9c), never retyped; then
// re-reads the report and checks every block against its source byte for byte, and every quote of his words in the
// report's body written as "<words>" (A<n>) against the item it cites. Run from the worktree's top:
//   node .checkpoints/foundation-toolchain/report-appendix.mjs [--check]
// --check writes nothing and only checks. Exit 1 on any mismatch.
import { readFileSync, writeFileSync } from 'node:fs';
import { ITEMS, GO51, SESSION, messageText, go51 } from './words-sources.mjs';

export const REPORT = 'docs/reports/2026-11-12-foundation-2-2.md';
const HEAD = "## Appendix A. Mustafa's words for this change-set, in full";
const cap = (s) => s[0].toUpperCase() + s.slice(1);

export function sources() {
  const list = ITEMS.map((it) => ({ ...it, text: messageText(it.uuid),
    header: `### ${it.id}. ${cap(it.title)}, ${it.at} UTC (transcript message ${it.uuid}${it.also ? `; ${it.also}` : ''})` }));
  list.push({ ...GO51, text: go51(),
    header: `### ${GO51.id}. ${cap(GO51.title)}, ${GO51.at} UTC (the time and the session per merge commit ` +
      `${GO51.commit.slice(0, 7)}'s message, which quotes it whole; extracted from that message, and re-quoting it gives ` +
      "the commit's block back byte for byte; not checked against that session's chat record)" });
  return list;
}

function appendix(list) {
  const intro = `_Each block below is copied byte for byte from its source by \`report-appendix.mjs\` (in the notes folder, ` +
    `readable at this branch's last checkpoint commit), never retyped. A1 to A12 are his messages in this session ` +
    `(${SESSION}, his personal account), each read from the session's transcript by \`words-sources.mjs\`: a ` +
    `message's single text block, or, for A9 and A11, the text of a message he sent while a turn was running. A13 is ` +
    `his go for #51, read from merge commit ${GO51.commit.slice(0, 7)}'s message. The same script checks every block ` +
    `against its source, and every quote of his words in this report's body against the item it cites. His other ` +
    `input in this session was commands: \`/context\` (2026-11-10 19:54 UTC), \`/advisor fable\` (2026-11-11 10:50 ` +
    `UTC) and \`/compact\` (2026-11-12 15:22 UTC); the workflow-authoring skill was loaded into the session twice ` +
    `(2026-11-10 18:04 UTC, and 2026-11-12 18:33 UTC with A12). No send of his was interrupted, and A2 reached the ` +
    `session twice, byte-identical. The session's queue held nothing else from him: its own agents' hand-backs and ` +
    `task notifications, and one notice that the session's worker had restarted (2026-11-11 09:00 UTC). Entries in ` +
    `\`docs/DECISIONS.md\` copy their quotes from here, because the notes folder is deleted before the pull request ` +
    `is marked ready._`;
  const blocks = list.map((s) => {
    const fence = s.text.includes('```') ? '````' : '```';
    return `${s.header}\n\n${fence}text\n${s.text}\n${fence}\n`;
  });
  return `${HEAD}\n\n${intro}\n\n${blocks.join('\n')}`;
}

// Every block in the report's Appendix A, by id: the text between its fences.
export function reportBlocks(report) {
  const at = report.indexOf(HEAD);
  if (at < 0) return new Map();
  const part = report.slice(at);
  const out = new Map();
  const re = /^### (A\d+)\. .*\n\n(`{3,})text\n([\s\S]*?)\n\2\n/gm;
  for (const m of part.matchAll(re)) out.set(m[1], m[3]);
  return out;
}

export function check(report, list) {
  const problems = [];
  const blocks = reportBlocks(report);
  for (const s of list) {
    if (!blocks.has(s.id)) problems.push(`${s.id}: no block in the report`);
    else if (blocks.get(s.id) !== s.text) problems.push(`${s.id}: the block differs from its source`);
  }
  if (blocks.size !== list.length) problems.push(`the appendix has ${blocks.size} blocks, the sources ${list.length}`);
  const body = report.slice(0, report.indexOf(HEAD) < 0 ? report.length : report.indexOf(HEAD));
  const byId = new Map(list.map((s) => [s.id, s.text]));
  let quotes = 0;
  for (const m of body.matchAll(/"([^"\n]+)" \((A\d+)(?:, A\d+)*\)/g)) {
    quotes++;
    const t = byId.get(m[2]);
    if (t === undefined) problems.push(`quote cites ${m[2]}, which is not an item: "${m[1]}"`);
    else if (!t.includes(m[1])) problems.push(`quote not found in ${m[2]}: "${m[1]}"`);
  }
  return { problems, quotes, blocks: blocks.size };
}

if (import.meta.url === `file://${process.argv[1]}`) {
  const list = sources();
  let report = readFileSync(REPORT, 'utf8');
  if (!process.argv.includes('--check')) {
    const at = report.indexOf(HEAD);
    report = (at < 0 ? `${report.replace(/\s*$/, '')}\n\n` : report.slice(0, at)) + appendix(list);
    writeFileSync(REPORT, report);
  }
  const r = check(readFileSync(REPORT, 'utf8'), list);
  console.log(`${r.problems.length ? 'FAIL' : 'OK'}: ${r.blocks} blocks equal to their sources; ${r.quotes} quotes in the body checked`);
  for (const p of r.problems) console.log(`  ${p}`);
  process.exit(r.problems.length ? 1 : 0);
}

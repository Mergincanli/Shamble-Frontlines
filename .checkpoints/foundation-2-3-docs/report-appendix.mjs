// Writes the report's "Appendix A. Mustafa's words for this change-set, in full" from the sources (words.mjs), never
// retyped; then re-reads the report and checks every block against its source byte for byte, and every quote of his
// words in the report's body written as "<words>" (A<n>) against the item it cites (A2-DECISION counts as A2).
//   node .checkpoints/foundation-2-3-docs/report-appendix.mjs [--check] [--file <path>]
// --check writes nothing and only checks; --file points at another copy (a scratch test). Exit 1 on any mismatch.
import { readFileSync, writeFileSync } from 'node:fs';
import { allItems, chat48, SESSION, GO48 } from './words.mjs';

export const REPORT = 'docs/reports/2026-11-13-foundation-2-3-docs.md';
export const HEAD = "## Appendix A. Mustafa's words for this change-set, in full";
const cap = (s) => s[0].toUpperCase() + s.slice(1);

export function sources() {
  const chat = chat48();
  const stamp = chat ? `, where the event is stamped ${chat.createdAt.replace('T', ' ').replace(/\.\d+Z$/, '')} UTC` : '';
  return allItems().map((it) => ({ ...it, header: it.uuid
    ? `### ${it.id}. ${cap(it.title)}, ${it.at} UTC (transcript message ${it.uuid})`
    : `### ${it.id}. ${cap(it.title)}, ${it.at} UTC (the time and the session per merge commit ${it.commit.slice(0, 7)}'s ` +
      "message, which quotes it whole; extracted from that message, and re-quoting it gives the commit's block back byte " +
      `for byte; checked byte for byte against that session's chat record, message ${it.chatUuid}, read with the ` +
      `sessions API${stamp})` }));
}

function appendix(list) {
  const intro = '_Each block below is copied byte for byte from its source by `report-appendix.mjs` (in the notes ' +
    'folder, readable at this branch\'s last checkpoint commit, named in the header above), never retyped. A1 and A2 are ' +
    `his messages in this session (${SESSION}, his personal account), each read from the session's transcript by ` +
    "`words.mjs`: a message's single text. A3 is his go for #48, read from merge commit " +
    `${GO48.commit.slice(0, 7)}'s message and checked byte for byte against foundation 2.2's chat record. His decision ` +
    'of 2026-11-13 is the text inside its quote marks in A2, cut by `words.mjs` at its first sentence ("Step 2.3 may be ' +
    'planned now.") and its last words ("only exception."), since it holds quote marks of its own. The same script ' +
    "checks every block against its source, and every quote of his words in this report's body against the item it " +
    'cites. His other input in this session was a command: `/advisor fable` (2026-11-13 18:30:08 UTC), whose output said ' +
    'this conversation keeps Opus 5.5 as its advisor model until `/clear` or `/compact`; the workflow-authoring skill was ' +
    'loaded into the session with A2 (18:47:56 UTC). Entries in `docs/DECISIONS.md` copy their quotes from here, because ' +
    'the notes folder is deleted before the pull request is marked ready._';
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
  const out = new Map();
  const re = /^### (A\d+)\. .*\n\n(`{3,})text\n([\s\S]*?)\n\2\n/gm;
  for (const m of report.slice(at).matchAll(re)) out.set(m[1], m[3]);
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
  const cut = report.indexOf(HEAD);
  const body = report.slice(0, cut < 0 ? report.length : cut);
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
  const fi = process.argv.indexOf('--file');
  const file = fi > 0 ? process.argv[fi + 1] : REPORT;
  const list = sources();
  if (!process.argv.includes('--check')) {
    let report = readFileSync(file, 'utf8');
    const at = report.indexOf(HEAD);
    report = (at < 0 ? `${report.replace(/\s*$/, '')}\n\n` : report.slice(0, at)) + appendix(list);
    writeFileSync(file, report);
  }
  const r = check(readFileSync(file, 'utf8'), list);
  console.log(`${r.problems.length ? 'FAIL' : 'OK'}: ${r.blocks} blocks checked against their sources; ${r.quotes} quotes in the body checked (${file})`);
  for (const p of r.problems) console.log(`  ${p}`);
  process.exit(r.problems.length ? 1 : 0);
}

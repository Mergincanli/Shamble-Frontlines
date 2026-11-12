// Mustafa's words for step 2.2, from their sources, by script (never retyped). Used by report-appendix.mjs and
// decisions-entries.mjs, and run alone to compare each source with its note in this folder.
//   node .checkpoints/foundation-toolchain/words-sources.mjs
// The 12 messages come from this session's transcript (the .jsonl Claude Code keeps): a user entry's single text
// block, or a queued message's text (attachment of type queued_command, origin "human"). A13, his go for #51, comes
// from merge commit 2aa5e9c's message: the "> " lines after "quoted whole", unquoted, and re-quoting them must give
// the commit's block back byte for byte.
import { readFileSync, existsSync } from 'node:fs';
import { execFileSync } from 'node:child_process';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

export const TRANSCRIPT = '/root/.claude/projects/-home-user-Shamble-Frontlines/29a42dc7-ecd4-506e-b90a-121dd5e50988.jsonl';
export const SESSION = 'session_019YNMUSc7FA68ujhLgXfEJA';
const HERE = dirname(fileURLToPath(import.meta.url));

// id, transcript uuid (or commit), UTC time, the title used in the report and DECISIONS.md, the note in this folder.
export const ITEMS = [
  { id: 'A1', uuid: 'bd0cc1cc-523f-4057-a0f6-bc0083156de3', at: '2026-11-10 17:23:21', title: "his task message: plan step 2.2, the toolchain (a paste block from his Advisor chat)", note: 'mustafa-2026-11-10-session.md' },
  { id: 'A2', uuid: '3d663a74-cbcf-47b3-93b6-1cbf80308c9c', at: '2026-11-10 19:43:00', title: 'his answers for step 2.2: FDN-NOCODE (a), FDN-HEADING (a), the build start, the network-cut rule, the ADR index entry', note: 'mustafa-2026-11-10-answers-2-2.md', also: 'sent twice, byte-identical, at 19:43:00 and 19:43:01 UTC (the session queue)' },
  { id: 'A3', uuid: '0599cdaf-a302-4720-b86e-31096eb57693', at: '2026-11-10 19:53:17', title: 'his change of plan: build step 2.2 in this chat, on this branch', note: 'mustafa-2026-11-10-build-here.md' },
  { id: 'A4', uuid: '4fe76b38-b26e-4dd7-a2d5-86961a276668', at: '2026-11-10 20:43:26', title: 'steps 1 to 4 noted, PR #48 a draft (its choice on the workflow prompt left unfilled; his KIT-ASK-SEEN (b), A5, answered it)', note: null },
  { id: 'A5', uuid: 'f18f6c35-42ba-4326-8d9d-c1f456ce5aca', at: '2026-11-11 00:07:48', title: 'KIT-ASK-SEEN (b): step-5 kit edits asked in chat first; "auto mode can pass the guard\'s ask" a finding for 2.3', note: 'mustafa-2026-11-11-kit-ask-seen.md' },
  { id: 'A6', uuid: 'f009e71f-56ff-4a25-b053-6f655cadf330', at: '2026-11-11 09:46:46', title: 'PR #49 merged; step 5 still waits for 2.1b; the docs pass in the new layout', note: 'mustafa-2026-11-11-pr49-merged.md' },
  { id: 'A7', uuid: 'cdec9787-bfb6-4e62-a6a9-040bcdd5cc4b', at: '2026-11-11 12:46:11', title: 'his words on the advisor tool, sent to this session', note: 'mustafa-2026-11-11-advisor-tool.md' },
  { id: 'A8', uuid: '5fbe8952-d9da-42e4-84d2-869367a35dbf', at: '2026-11-12 15:26:57', title: 'step 5 starts (2.1b merged); his first words on models (replaced in item 3 by A9)', note: 'mustafa-2026-11-12-step5-models.md' },
  { id: 'A9', uuid: 'adb00970-7927-49f7-bfa5-5095981ab169', at: '2026-11-12 15:35:17', title: 'his words of 2026-11-12, 16:25, on models (replacing LANE-RULES\' "The main model and every agent stay Opus 5.5"), and the Fable advisor\'s read of the built skeleton', note: 'mustafa-2026-11-12-models-skeleton.md', also: 'sent while the session\'s turn was running (a queued message); it re-sends A8 with item 3 changed and item 4 new' },
  { id: 'A10', uuid: '6a69b2b2-44d0-413e-9267-7846396bae82', at: '2026-11-12 16:28:28', title: 'KIT-5 yes (K2a, K2b, K3, K4), K1b (a), step 7 at effort max, S2 to S5 recorded', note: 'mustafa-2026-11-12-kit-yes.md' },
  { id: 'A11', uuid: '8be9f140-6fad-4974-ad46-f82ec8a37db6', at: '2026-11-12 16:31:45', title: 'his count of the guard\'s prompts for the five kit writes', note: 'mustafa-2026-11-12-prompt-count.md', also: 'sent while the session\'s turn was running (a queued message)' },
  { id: 'A12', uuid: 'd69572b3-3c05-4ec9-b33d-c320c006951a', at: '2026-11-12 18:33:35', title: 'the chat back in Auto', note: 'mustafa-2026-11-12-back-in-auto.md' },
];
export const GO51 = { id: 'A13', commit: '2aa5e9ce03e9ab9ccfecd53e6aabf951abe705dd', at: '2026-11-12 13:18:42', title: 'his go for #51, in foundation 2.1b\'s session (session_019YNMUWdGJZoQFUuuvvBP1Y)' };

let cache = null;
function transcript() {
  if (cache) return cache;
  cache = readFileSync(TRANSCRIPT, 'utf8').split('\n').filter(Boolean).map((l) => { try { return JSON.parse(l); } catch { return null; } }).filter(Boolean);
  return cache;
}
export function messageText(uuid) {
  const hits = transcript().filter((o) => o.uuid === uuid);
  if (hits.length !== 1) throw new Error(`${uuid}: ${hits.length} transcript entries`);
  const o = hits[0];
  if (o.type === 'user') {
    const c = o.message?.content;
    const t = typeof c === 'string' ? [c] : (c ?? []).filter((b) => b.type === 'text').map((b) => b.text);
    if (t.length !== 1 || (Array.isArray(c) && c.length !== 1)) throw new Error(`${uuid}: not a single text block`);
    return t[0];
  }
  if (o.type === 'attachment' && o.attachment?.type === 'queued_command' && o.attachment?.origin?.kind === 'human') {
    return o.attachment.prompt;
  }
  throw new Error(`${uuid}: not a message from him`);
}
// A13: the quoted block of merge commit 2aa5e9c's message, unquoted; the round trip is checked here.
export function go51() {
  const msg = execFileSync('git', ['show', '-s', '--format=%B', GO51.commit], { encoding: 'utf8' });
  const lines = msg.split('\n');
  const start = lines.findIndex((l) => l.includes('quoted whole')) + 1;
  let i = start;
  while (i < lines.length && lines[i] === '') i++;
  const block = [];
  while (i < lines.length && (lines[i] === '>' || lines[i].startsWith('> '))) block.push(lines[i++]);
  if (!block.length) throw new Error('2aa5e9c: no quoted block after "quoted whole"');
  const text = block.map((l) => (l === '>' ? '' : l.slice(2))).join('\n');
  const requoted = text.split('\n').map((l) => (l === '' ? '>' : `> ${l}`)).join('\n');
  if (requoted !== block.join('\n')) throw new Error('2aa5e9c: re-quoting does not give the commit\'s block back');
  return text;
}
export function noteText(note) {
  const f = join(HERE, note);
  if (!existsSync(f)) return null;
  const s = readFileSync(f, 'utf8');
  const a = s.indexOf('```text\n');
  const b = s.lastIndexOf('\n```');
  return a < 0 || b < a ? null : s.slice(a + 8, b);
}

if (import.meta.url === `file://${process.argv[1]}`) {
  for (const it of ITEMS) {
    const t = messageText(it.uuid);
    const n = it.note ? noteText(it.note) : undefined;
    const cmp = n === undefined ? 'no note' : n === null ? 'note has no text fence' : n === t ? 'note identical' : `NOTE DIFFERS (note ${n.length} chars)`;
    console.log(`${it.id} ${it.at} ${it.uuid} ${t.length} chars, ${t.split('\n').length} lines; ${cmp}`);
  }
  const g = go51();
  console.log(`${GO51.id} ${GO51.at} commit ${GO51.commit.slice(0, 7)} ${g.length} chars, ${g.split('\n').length} lines; round trip OK`);
}

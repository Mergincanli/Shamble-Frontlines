// Mustafa's words for foundation 2.3's first docs pass, from their sources, by script (never retyped). Used by
// report-appendix.mjs and decisions.mjs; run alone to print each item and the checks (it writes nothing):
//   node .checkpoints/foundation-2-3-docs/words.mjs
// A1, A2 and A4 come from this session's transcript (the .jsonl Claude Code keeps): a user entry whose content is one
// string, found exactly once by its uuid (A4, his status request, was found by the scan after review round 2). A2-DECISION is cut from inside its quote marks in A2 by two anchors, because
// the decision holds nested quote marks. A3, his go for #48, comes from merge commit 4d21c1a's message: the "> " lines
// after "quoted whole", unquoted; re-quoting them must give the commit's block back byte for byte (2.2's go51 method).
// A3 is also compared with the 2.2 session's chat record: the event of message 96467900-... in a page of that
// session's events that the sessions API returned (list_events), saved by the harness to this session's tool results.
import { readFileSync, existsSync } from 'node:fs';
import { execFileSync } from 'node:child_process';

export const TRANSCRIPT = '/root/.claude/projects/-home-user-Shamble-Frontlines/7261f3d7-f651-586a-9c39-c052553b68a3.jsonl';
export const SESSION = 'session_019YNMUWSsyRgaYWxk4auQ5b';
export const PAGES48 = [
  '/root/.claude/projects/-home-user-Shamble-Frontlines/7261f3d7-f651-586a-9c39-c052553b68a3/tool-results/mcp-claude-code-remote-list_events-1794595937159.txt',
  '/root/.claude/projects/-home-user-Shamble-Frontlines/7261f3d7-f651-586a-9c39-c052553b68a3/tool-results/mcp-claude-code-remote-list_events-1794595922657.txt',
];
export const ITEMS = [
  { id: 'A1', uuid: 'b031379c-ea4c-47db-b279-54e937bf147a', at: '2026-11-13 18:29:55', title: 'his message before the task' },
  { id: 'A2', uuid: 'e5fa7ca0-6494-44d0-8366-81fc612c7958', at: '2026-11-13 18:47:55', title: "his task message: step 2.3's first docs pass and its plan, with his decision of 2026-11-13 on step 2.3" },
  { id: 'A4', uuid: 'a994258f-93c9-45d2-8878-31e7cfee6dcd', at: '2026-11-13 22:27:05', title: 'his status request, sent while review round 2 ran' },
];
export const GO48 = { id: 'A3', commit: '4d21c1a7a4d39613bffce5d5f9bbc92f2f88a300', at: '2026-11-13 16:16:47', chatUuid: '96467900-83d8-4b54-8f8b-7884a0dc0fde', title: "his go for #48, in foundation 2.2's session (session_019YNMUSc7FA68ujhLgXfEJA)" };
export const DECISION_START = 'Step 2.3 may be planned now.';
export const DECISION_END = 'only exception.';
const LEAD = 'to record word for word:\n"';

let cache = null;
function transcript() {
  if (cache) return cache;
  cache = readFileSync(TRANSCRIPT, 'utf8').split('\n').filter(Boolean)
    .map((l) => { try { return JSON.parse(l); } catch { return null; } }).filter(Boolean);
  return cache;
}

export function messageText(uuid) {
  const hits = transcript().filter((o) => o.uuid === uuid);
  if (hits.length !== 1) throw new Error(`${uuid}: ${hits.length} transcript entries`);
  const o = hits[0];
  if (o.type !== 'user') throw new Error(`${uuid}: not a user entry`);
  const c = o.message?.content;
  if (typeof c === 'string') return c;
  const t = (c ?? []).filter((b) => b.type === 'text').map((b) => b.text);
  if (t.length !== 1 || c.length !== 1) throw new Error(`${uuid}: not a single text block`);
  return t[0];
}

export function timestampOf(uuid) {
  return transcript().find((o) => o.uuid === uuid)?.timestamp ?? null;
}

// His decision, from inside its quote marks in A2: after the lead "to record word for word:\n\"", from its first
// sentence to "only exception." followed by the closing quote mark.
export function decision(a2) {
  const first = a2.indexOf(LEAD);
  if (first < 0 || a2.indexOf(LEAD, first + 1) >= 0) throw new Error('A2: the lead is not found exactly once');
  const start = first + LEAD.length;
  if (!a2.startsWith(DECISION_START, start)) throw new Error('A2: the decision does not start with its first sentence');
  const close = a2.indexOf(`${DECISION_END}"`, start);
  if (close < 0) throw new Error('A2: the decision\'s last words and closing quote mark are not found');
  const text = a2.slice(start, close + DECISION_END.length);
  if (text.includes('\n')) throw new Error('A2: the decision spans more than one line');
  if (a2.slice(start, start + text.length) !== text) throw new Error('A2: the slice does not round-trip');
  return text;
}

// A3: the quoted block of merge commit 4d21c1a's message, unquoted; the round trip is checked here.
export function go48() {
  const msg = execFileSync('git', ['show', '-s', '--format=%B', GO48.commit], { encoding: 'utf8' });
  const lines = msg.split('\n');
  const at = lines.findIndex((l) => l.includes('quoted whole'));
  if (at < 0 || lines.findIndex((l, i) => i > at && l.includes('quoted whole')) >= 0) throw new Error('4d21c1a: "quoted whole" not found exactly once');
  let i = at + 1;
  while (i < lines.length && lines[i] === '') i++;
  const block = [];
  while (i < lines.length && (lines[i] === '>' || lines[i].startsWith('> '))) block.push(lines[i++]);
  if (!block.length) throw new Error('4d21c1a: no quoted block after "quoted whole"');
  const text = block.map((l) => (l === '>' ? '' : l.slice(2))).join('\n');
  const requoted = text.split('\n').map((l) => (l === '' ? '>' : `> ${l}`)).join('\n');
  if (requoted !== block.join('\n')) throw new Error("4d21c1a: re-quoting does not give the commit's block back");
  return text;
}

// The 2.2 session's chat record of the same message, from a saved list_events page (JSON inside the harness's
// envelope): the first page holds user events, the second assistant events, whose tool results can quote it too.
// Returns { text, createdAt } from the user event, or null when the page is not on this machine.
export function chat48(page = PAGES48[0]) {
  if (!existsSync(page)) return null;
  const raw = readFileSync(page, 'utf8');
  const a = raw.indexOf('{"ccr"');
  const b = raw.lastIndexOf('\n</other-session');
  if (a < 0 || b < a) throw new Error('the saved page has no JSON body');
  const parsed = JSON.parse(raw.slice(a, b));
  const hits = (parsed.ccr?.data ?? []).filter((e) => e.user?.uuid === GO48.chatUuid);
  if (hits.length !== 1) throw new Error(`the saved page holds ${hits.length} events for ${GO48.chatUuid}`);
  const content = hits[0].user.internal_anthropic_catchall?.message?.content;
  if (typeof content !== 'string') throw new Error('the chat event is not one string');
  return { text: content, createdAt: hits[0].created_at };
}

// C8's second page: the 2.2 session's merge call (assistant event), whose commit_message quotes his go after "quoted
// whole"; the quoted block, unquoted, must equal A3. Returns { text, createdAt } or null when the page is missing.
export function mergeCall48(page = PAGES48[1]) {
  if (!existsSync(page)) return null;
  const raw = readFileSync(page, 'utf8');
  const a = raw.indexOf('{"ccr"');
  const b = raw.lastIndexOf('\n</other-session');
  const data = JSON.parse(raw.slice(a, b)).ccr?.data ?? [];
  const hits = data.filter((e) => (e.assistant?.internal_anthropic_catchall?.message?.content ?? [])
    .some((c) => typeof c?.input?.commit_message === 'string' && c.input.commit_message.includes(GO48.chatUuid)));
  if (hits.length !== 1) throw new Error(`the second page holds ${hits.length} merge calls naming ${GO48.chatUuid}`);
  const msg = hits[0].assistant.internal_anthropic_catchall.message.content.find((c) => c?.input?.commit_message).input.commit_message;
  return { text: unquoteAfter(msg, '4d21c1a\'s merge call'), createdAt: hits[0].created_at };
}

function unquoteAfter(msg, what) {
  const lines = msg.split('\n');
  const at = lines.findIndex((l) => l.includes('quoted whole'));
  if (at < 0) throw new Error(`${what}: "quoted whole" not found`);
  let i = at + 1;
  while (i < lines.length && lines[i] === '') i++;
  const block = [];
  while (i < lines.length && (lines[i] === '>' || lines[i].startsWith('> '))) block.push(lines[i++]);
  return block.map((l) => (l === '>' ? '' : l.slice(2))).join('\n');
}

// Every user-type transcript entry that is not a tool result, and every queued message, classified: his words (must be
// an item of the appendix), his command, the harness's (meta entries: a command's caveat, a skill's text, a worker
// restart), or this session's agents and tasks (queued peer hand-backs and task notifications). Anything else, or a
// message of his that is not an item, is a problem: the appendix would then not hold all his words.
export function scan() {
  const ids = new Map(ITEMS.map((it) => [it.uuid, it.id]));
  const rows = [];
  const problems = [];
  for (const o of transcript()) {
    if (o.type === 'user') {
      const c = o.message?.content;
      if (Array.isArray(c) && c.length && c.every((b) => b?.type === 'tool_result')) continue;
      const text = typeof c === 'string' ? c : (c ?? []).filter((b) => b?.type === 'text').map((b) => b.text).join(' ');
      let cls;
      const ok = o.origin?.kind;
      if (ids.has(o.uuid)) cls = `his words (${ids.get(o.uuid)})`;
      else if (o.isMeta) cls = 'harness (meta)';
      else if (o.isCompactSummary === true && text.startsWith('This session is being continued from a previous conversation')) cls = 'harness (the summary at a compaction)';
      else if (ok === 'task-notification' || ok === 'peer') cls = `this session's agents (${ok})`;
      else if (ok === 'human') { cls = 'HIS MESSAGE, NOT AN ITEM'; problems.push(`${o.uuid} ${o.timestamp}: a message of his that is not an item`); }
      else if (/^<command-name>\/[a-z-]+<\/command-name>/.test(text)) cls = 'his command';
      else { cls = 'UNCLASSIFIED'; problems.push(`${o.uuid} ${o.timestamp}: an unclassified user entry`); }
      rows.push({ at: o.timestamp, uuid: o.uuid, cls, preview: text.slice(0, 70).replace(/\s+/g, ' ') });
    } else if (o.type === 'attachment' && o.attachment?.type === 'queued_command') {
      const kind = o.attachment.origin?.kind ?? 'none';
      let cls;
      if (kind === 'human') { cls = 'HIS QUEUED MESSAGE'; problems.push(`${o.uuid} ${o.timestamp}: a message of his sent while a turn ran, not an item`); }
      else if (kind === 'peer' || kind === 'task-notification') cls = `this session's agents (${kind})`;
      else { cls = 'UNCLASSIFIED'; problems.push(`${o.uuid} ${o.timestamp}: an unclassified queued message (${kind})`); }
      rows.push({ at: o.timestamp, uuid: o.uuid, cls, preview: String(o.attachment.prompt ?? '').slice(0, 70).replace(/\s+/g, ' ') });
    }
  }
  for (const it of ITEMS) if (!rows.some((r) => r.uuid === it.uuid)) problems.push(`${it.id}: not found by the scan`);
  return { rows, problems };
}

export function allItems() {
  const list = ITEMS.map((it) => ({ ...it, text: messageText(it.uuid), timestamp: timestampOf(it.uuid) }));
  list.push({ ...GO48, text: go48() });
  return list.sort((a, b) => Number(a.id.slice(1)) - Number(b.id.slice(1)));
}
export const item = (list, id) => { const it = list.find((x) => x.id === id); if (!it) throw new Error(`${id}: not an item`); return it; };

if (import.meta.url === `file://${process.argv[1]}`) {
  const list = allItems();
  for (const it of list) console.log(`${it.id} ${it.at} UTC ${it.uuid ?? `commit ${it.commit.slice(0, 7)}`}${it.timestamp ? ` (transcript ${it.timestamp})` : ''}: ${it.text.length} chars, ${it.text.split('\n').length} lines`);
  const a2 = item(list, 'A2').text;
  const a3 = item(list, 'A3').text;
  const d = decision(a2);
  console.log(`A2-DECISION: ${d.length} chars, one line; starts "${d.slice(0, 28)}", ends "${d.slice(-15)}"; inside A2: ${a2.includes(d)}`);
  console.log('A3: the round trip OK (re-quoting gives the commit\'s block back byte for byte)');
  const c = chat48();
  if (!c) console.log('A3 vs the chat record: the saved page is not on this machine');
  else console.log(`A3 vs the chat record (event ${GO48.chatUuid}, created_at ${c.createdAt}): ${c.text === a3 ? 'identical, byte for byte' : `DIFFERS (chat ${c.text.length} chars, commit ${a3.length})`}`);
  const m = mergeCall48();
  if (!m) console.log('A3 vs the 2.2 session\'s merge call: the second page is not on this machine');
  else console.log(`A3 vs the 2.2 session's merge call (its commit_message, ${m.createdAt}): ${m.text === a3 ? 'identical, byte for byte' : `DIFFERS (${m.text.length} chars, commit ${a3.length})`}`);
  const sc = scan();
  console.log(`scan: ${sc.rows.length} entries classified: ${[...new Set(sc.rows.map((r) => r.cls))].map((k) => `${k} ${sc.rows.filter((r) => r.cls === k).length}`).join('; ')}`);
  for (const r of sc.rows) console.log(`  ${r.at} ${r.uuid.slice(0, 8)} ${r.cls}: ${r.preview}`);
  for (const p of sc.problems) console.log(`  PROBLEM ${p}`);
  if ((c && c.text !== a3) || (m && m.text !== a3) || sc.problems.length) process.exit(1);
}

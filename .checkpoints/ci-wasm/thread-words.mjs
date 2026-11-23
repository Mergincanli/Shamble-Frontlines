// Copies the messages of the project thread "Build the game from change-set 3" from its session's transcript into the
// notes, byte for byte, never retyped. His messages reach a project thread inside a wake: the text between
// <message ... sent-at="..."> and </message>, with XML entities for quotes and the like; this script decodes those
// entities and nothing else. The thread's own messages and cards are the inputs of its tool calls, taken by call id.
// A tap on a decision card arrives as a message the platform writes for him (the card's own wording, not typed text).
//   node .checkpoints/ci-wasm/thread-words.mjs <transcript.jsonl> [--check]
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';

const NOTES = '.checkpoints/ci-wasm/';
const [transcript] = process.argv.slice(2);
const check = process.argv.includes('--check');
const sha = (s) => createHash('sha256').update(s).digest('hex');
const NAMED = { amp: '&', lt: '<', gt: '>', quot: '"', apos: "'" };
const decode = (s) =>
  s.replace(/&(#x[0-9a-fA-F]+|#\d+|amp|lt|gt|quot|apos);/g, (_, n) =>
    n[0] !== '#' ? NAMED[n] : String.fromCodePoint(n[1] === 'x' ? parseInt(n.slice(2), 16) : Number(n.slice(1))),
  );

// Every text the harness delivered to the session (never a tool's output), and every tool call the session made.
const delivered = [];
const calls = new Map();
for (const line of readFileSync(transcript, 'utf8').split('\n')) {
  if (line === '') continue;
  let e;
  try { e = JSON.parse(line); } catch { continue; }
  if (e.type === 'user') {
    const c = e.message?.content;
    if (typeof c === 'string') delivered.push(c);
    else if (Array.isArray(c)) for (const p of c) if (p.type === 'text') delivered.push(p.text);
  } else if (e.type === 'queue-operation' && typeof e.content === 'string') delivered.push(e.content);
  else if (e.type === 'attachment' && typeof e.attachment?.prompt === 'string') delivered.push(e.attachment.prompt);
  else if (e.type === 'assistant' && Array.isArray(e.message?.content)) {
    for (const p of e.message.content) if (p.type === 'tool_use') calls.set(p.id, { input: p.input, name: p.name, at: e.timestamp });
  }
}
function his(sentAt) {
  const seen = new Set();
  let attrs = '';
  for (const s of delivered) {
    for (const m of s.matchAll(/<message ([^>]*)>([\s\S]*?)<\/message>/g)) {
      if (!m[1].includes('from="human"') || !m[1].includes('trust="principal"') || !m[1].includes(`sent-at="${sentAt}"`)) continue;
      seen.add(m[2]);
      attrs = m[1];
    }
  }
  if (seen.size !== 1) throw new Error(`${sentAt}: expected one text for his message, found ${seen.size}`);
  const id = attrs.match(/ id="([^"]+)"/);
  return { text: decode([...seen][0]), id: id ? id[1] : '(a tap on a decision card: the platform gives no message id)' };
}
function call(id, name) {
  const c = calls.get(id);
  if (!c || c.name !== name) throw new Error(`${id}: no ${name} call with that id`);
  return c;
}
const card = (c) =>
  [
    `Question: ${c.input.question}`,
    ...c.input.options.map((o, i) => `Option ${i + 1}${i === c.input.recommended ? ' (recommended)' : ''}: ${o.label}. ${o.consequence}`),
    `Why recommended: ${c.input.reason}`,
  ].join('\n');

const out = [];
function note(file, title, how, words) {
  const text = `# ${title}\n\n_${how} (${Buffer.byteLength(words)} bytes, sha256 ${sha(words)})._\n\n---\n${words}\n`;
  out.push({ file, text, bytes: Buffer.byteLength(words), sha256: sha(words) });
}
const FROM = "Copied by script from the session's transcript (session_01WLuqnwQ8c8JuNgfBX7Wixg), never retyped";

// 1. His first message in the thread (14:25:03 UTC). Of its 211 lines, 201 are text he pasted from three other chats;
// they are left out here and named. His own lines are kept whole.
{
  const m = his('2026-11-23T14:25:03Z');
  const lines = m.text.split('\n');
  if (lines.length !== 211) throw new Error(`his first message has ${lines.length} lines, not 211`);
  const keep = (a, b) => lines.slice(a - 1, b).join('\n');
  // What the third marker says: the text pasted under 'Block 2.3' is the text pasted under Builder V1.
  if (keep(5, 88) !== keep(124, 207)) throw new Error("the text pasted under 'Block 2.3' differs from Builder V1's");
  const words = [
    keep(1, 4),
    '[left out, lines 5 to 90: the last text of the chat he calls Builder V1, pasted by him]',
    keep(91, 92),
    '[left out, lines 93 to 121: the last text of the chat he calls Builder V2, pasted by him]',
    keep(122, 122),
    "[left out, lines 123 to 209: the last text of the chat he calls 'Block 2.3', pasted by him; equal to Builder V1's]",
    keep(210, 211),
  ].join('\n');
  note(
    'mustafa-2026-11-23T142503-builder-chat.md',
    'Mustafa, 2026-11-23, 14:25:03 UTC: he would rather go on today, and wants a new builder chat to continue building the game',
    `${FROM}: his message ${m.id}, sent 2026-11-23T14:25:03Z, the first of the thread. The whole message has ${Buffer.byteLength(m.text)} bytes, sha256 ${sha(m.text)}; the lines in square brackets stand for the text he pasted from other chats, which is left out; his own lines are whole`,
    words,
  );
}
// 2. The thread's message that states the seven answers back to him, and its card.
{
  const c = call('toolu_0131ioSAEjCfPXbqMgajDDTX', 'mcp__hearthbot__post_message');
  note('thread-2026-11-23T142825-seven-answers.md', "The thread's message of 2026-11-23, 14:28:25 UTC: the seven answers stated back to him (not his words)", `${FROM}: the text of the thread's post_message call ${'toolu_0131ioSAEjCfPXbqMgajDDTX'}, stamped ${c.at}`, c.input.text);
  // The part his tap answers: from the paragraph that lists the seven answers to the end. The two paragraphs before
  // it are about the thread itself and about his question on the chats' names.
  const start = '**The seven answers I will work from**';
  if (c.input.text.split(start).length !== 2) throw new Error('the paragraph of the seven answers must stand once');
  note('thread-2026-11-23T142825-seven-answers-part.md', "The thread's message of 2026-11-23, 14:28:25 UTC, from its third paragraph to its end: the seven answers stated back to him (not his words)", `${FROM}: the text of the thread's post_message call toolu_0131ioSAEjCfPXbqMgajDDTX, stamped ${c.at}, from the paragraph that lists the seven answers to the end; the whole message is thread-2026-11-23T142825-seven-answers.md`, start + c.input.text.split(start)[1]);
  const d = call('toolu_015HWn2nBDfXux9zUHAr6HrM', 'mcp__hearthbot__request_decision');
  note('thread-2026-11-23T142827-card-answers-7.md', "The thread's decision card of 2026-11-23, 14:28:27 UTC: ANSWERS-7 (not his words)", `${FROM}: the question, options and reason of the thread's request_decision call toolu_015HWn2nBDfXux9zUHAr6HrM, stamped ${d.at}, each field whole, laid out one per line`, card(d));
}
// 3. His tap on that card.
{
  const m = his('2026-11-23T14:31:06Z');
  note('mustafa-2026-11-23T143106-answers-7.md', 'Mustafa, 2026-11-23, 14:31:06 UTC: his tap on the card ANSWERS-7', `${FROM}: the message the platform wrote for his tap, sent 2026-11-23T14:31:06Z, whole`, m.text);
}
// 4. The card on long runs and his tap.
{
  const d = call('toolu_01Y5drpC6DSHdsBnUDRgnJgT', 'mcp__hearthbot__request_decision');
  note('thread-2026-11-23T143025-card-long-runs.md', "The thread's decision card of 2026-11-23, 14:30:25 UTC: LONG-RUNS (not his words)", `${FROM}: the question, options and reason of the thread's request_decision call toolu_01Y5drpC6DSHdsBnUDRgnJgT, stamped ${d.at}, each field whole, laid out one per line`, card(d));
  const m = his('2026-11-23T14:32:42Z');
  note('mustafa-2026-11-23T143242-long-runs.md', 'Mustafa, 2026-11-23, 14:32:42 UTC: his tap on the card LONG-RUNS', `${FROM}: the message the platform wrote for his tap, sent 2026-11-23T14:32:42Z, whole`, m.text);
}
// 5. The card on the CI file write in this thread, and his typed answer.
{
  const d = call('toolu_016ztjrTjbH2VTrFXzcsyCtg', 'mcp__hearthbot__request_decision');
  note('thread-2026-11-23T142918-card-write-thread.md', "The thread's decision card of 2026-11-23, 14:29:18 UTC: WRITE-THREAD (not his words)", `${FROM}: the question, options and reason of the thread's request_decision call toolu_016ztjrTjbH2VTrFXzcsyCtg, stamped ${d.at}, each field whole, laid out one per line`, card(d));
  const m = his('2026-11-23T14:32:43Z');
  note('mustafa-2026-11-23T143243-write-thread.md', 'Mustafa, 2026-11-23, 14:32:43 UTC: his typed answer to the card WRITE-THREAD, and his question', `${FROM}: his message ${m.id}, sent 2026-11-23T14:32:43Z, whole`, m.text);
  const a = call('toolu_01QxMBMpnjWYiRtCRG8FEpcN', 'mcp__hearthbot__post_message');
  note('thread-2026-11-23T143355-why-here.md', "The thread's answer of 2026-11-23, 14:33:55 UTC to his question (not his words)", `${FROM}: the text of the thread's post_message call toolu_01QxMBMpnjWYiRtCRG8FEpcN, stamped ${a.at}`, a.input.text);
}
// 6. The thread's short message after his tap on ANSWERS-7 (it says the CI file still waits for the second card).
{
  const c = call('toolu_016fiAnrb4RD3t5tTKLMZx1s', 'mcp__hearthbot__post_message');
  note('thread-2026-11-23T143117-got-your-yes.md', "The thread's message of 2026-11-23, 14:31:17 UTC, after his tap on ANSWERS-7 (not his words)", `${FROM}: the text of the thread's post_message call toolu_016fiAnrb4RD3t5tTKLMZx1s, stamped ${c.at}`, c.input.text);
}
// 7. The thread's correction of its 14:33:55 answer (what a normal Claude Code chat adds: the guard too, not only the
// mode switch), and the card that puts the place of the CI write to him again: WRITE-WHERE.
{
  const c = call('toolu_01XjPKKsssZnEvHM9J6zFg9o', 'mcp__hearthbot__post_message');
  note('thread-2026-11-23T145131-correction.md', "The thread's correction of 2026-11-23, 14:51:31 UTC to its answer of 14:33:55 UTC (not his words)", `${FROM}: the text of the thread's post_message call toolu_01XjPKKsssZnEvHM9J6zFg9o, stamped ${c.at}`, c.input.text);
  const d = call('toolu_018t2TERYYtwV727oqCuN3LG', 'mcp__hearthbot__request_decision');
  note('thread-2026-11-23T145133-card-write-where.md', "The thread's decision card of 2026-11-23, 14:51:33 UTC: WRITE-WHERE (not his words)", `${FROM}: the question, options and reason of the thread's request_decision call toolu_018t2TERYYtwV727oqCuN3LG, stamped ${d.at}, each field whole, laid out one per line`, card(d));
  const m = his('2026-11-23T14:54:47Z');
  note('mustafa-2026-11-23T145447-write-where.md', 'Mustafa, 2026-11-23, 14:54:47 UTC: his tap on the card WRITE-WHERE', `${FROM}: the message the platform wrote for his tap, sent 2026-11-23T14:54:47Z, whole`, m.text);
  const a = call('toolu_01J1wM7tj7RLQsh7h6PGnfvW', 'mcp__hearthbot__post_message');
  note('thread-2026-11-23T145639-new-chat.md', "The thread's message of 2026-11-23, 14:56:39 UTC, after his tap on WRITE-WHERE (not his words)", `${FROM}: the text of the thread's post_message call toolu_01J1wM7tj7RLQsh7h6PGnfvW, stamped ${a.at}`, a.input.text);
}
// 8. The handover: the thread's message with the block for him to paste into the new Claude Code chat, and its two
// cards (BUILD-SEAT, ACCT-3). The block inside the message must equal handover-block.txt, saved before the message.
{
  const c = call('toolu_01DnP7mtqK6QbonVaPGDEVGN', 'mcp__hearthbot__post_message');
  const parts = c.input.text.split('```\n');
  if (parts.length !== 3 || parts[1] !== readFileSync(NOTES + 'handover-block.txt', 'utf8')) throw new Error('the block in the message differs from handover-block.txt');
  note('thread-2026-11-23T150951-handover.md', "The thread's message of 2026-11-23, 15:09:51 UTC: the handover, with the block for him to paste into the new Claude Code chat (not his words until he pastes the block there)", `${FROM}: the text of the thread's post_message call toolu_01DnP7mtqK6QbonVaPGDEVGN, stamped ${c.at}; the block between its two fence lines equals handover-block.txt byte for byte`, c.input.text);
  const d = call('toolu_01TqVRR5AGJTkj158U1ugpAq', 'mcp__hearthbot__request_decision');
  note('thread-2026-11-23T150954-card-build-seat.md', "The thread's decision card of 2026-11-23, 15:09:54 UTC: BUILD-SEAT (not his words)", `${FROM}: the question, options and reason of the thread's request_decision call toolu_01TqVRR5AGJTkj158U1ugpAq, stamped ${d.at}, each field whole, laid out one per line`, card(d));
  const e = call('toolu_01Fb9XQyj49TyLjqhsqENji9', 'mcp__hearthbot__request_decision');
  note('thread-2026-11-23T150956-card-acct-3.md', "The thread's decision card of 2026-11-23, 15:09:56 UTC: ACCT-3 (not his words)", `${FROM}: the question, options and reason of the thread's request_decision call toolu_01Fb9XQyj49TyLjqhsqENji9, stamped ${e.at}, each field whole, laid out one per line`, card(e));
}
// 9. His taps on those two cards, and the thread's reply to the first.
{
  const b = his('2026-11-23T15:10:32Z');
  note('mustafa-2026-11-23T151032-build-seat.md', 'Mustafa, 2026-11-23, 15:10:32 UTC: his tap on the card BUILD-SEAT', `${FROM}: the message the platform wrote for his tap, sent 2026-11-23T15:10:32Z, whole`, b.text);
  const r = call('toolu_015q8WpBCg8SfkVuooUs9HtB', 'mcp__hearthbot__post_message');
  note('thread-2026-11-23T151058-build-seat-noted.md', "The thread's message of 2026-11-23, 15:10:58 UTC, after his tap on BUILD-SEAT (not his words)", `${FROM}: the text of the thread's post_message call toolu_015q8WpBCg8SfkVuooUs9HtB, stamped ${r.at}`, r.input.text);
  const a = his('2026-11-23T15:11:01Z');
  note('mustafa-2026-11-23T151101-acct-3.md', 'Mustafa, 2026-11-23, 15:11:01 UTC: his tap on the card ACCT-3', `${FROM}: the message the platform wrote for his tap, sent 2026-11-23T15:11:01Z, whole`, a.text);
}

let ok = true;
for (const n of out) {
  if (check) {
    const same = readFileSync(NOTES + n.file, 'utf8') === n.text;
    ok &&= same;
    console.log(`${same ? 'yes' : 'NO '}  ${n.file}: on disk equal to what the transcript gives (${n.bytes} bytes, sha256 ${n.sha256})`);
  } else {
    writeFileSync(NOTES + n.file, n.text);
    console.log(`${n.file}: ${n.bytes} bytes, sha256 ${n.sha256}`);
  }
}
process.exit(ok ? 0 : 1);

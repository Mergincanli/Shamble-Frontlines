// Builder V6's chat (session_01WLuqnt1v3UNwSqbJsJre7u, from 2026-11-27): copies one of Mustafa's messages from the
// chat's transcript into the notes, byte for byte, with the question it answers. A message typed while the chat was
// working is stored as a queued command, so both kinds are read. Builder V4's script (session_019YNMUVZR6ociHFWjByxkrq;
// the notes it saved keep that id), with the session changed, and <out file> a full path (his standing rule of
// 2026-11-27: every file a command writes is spelled out with its full path). Builder V3's script before that (its
// branch, .checkpoints/handover-builder-v3/). From the worktree's root:
//   node .checkpoints/cs4-walk/chat-words.mjs words <transcript.jsonl> <marker> <out file> <title> <what it answers>
//   node .checkpoints/cs4-walk/chat-words.mjs exact <transcript.jsonl> <the whole text> <out file> <title> <what it answers> [after]
//   node .checkpoints/cs4-walk/chat-words.mjs part  <saved file> <from line> <before line> <out file> <title>
//   node .checkpoints/cs4-walk/chat-words.mjs less  <transcript.jsonl> <marker> <line start> <out file> <title> <what it answers> <why> [after]
// <saved file> is a name in the notes folder; <out file> is always a full path, and the script refuses any other.
// `less` (4c, for his answer BOARD-WORDS (b); the advisor agent's F-S0-1): the message as `words` finds it, with the
// one line that starts with <line start> left out (for example "3. ", an item of a numbered message). The header gives
// the line left out by its number and <why>, and the bytes and sha256 of the text shown below it, which is what
// decisions-4c.mjs quotes and checks. No word is put in the left-out line's place. The whole message's bytes and
// sha256 are printed only, never written into the file: in a public repository a digest and a length of the whole
// beside the shown text would let a reader confirm a guess of the left-out line (the commit security review of
// 2026-11-28). The chat gives them to him, and `less` run again into a scratch file checks the saved file against
// the transcript.
// [after]: only messages sent after this time (as the transcript writes it), for a text he sent more than once.
// A queued command whose `source_uuid` is the id of a message already in the record is that message handed to the
// chat again (seen twice on 2026-11-26, each time right after the session's worker was restarted): it is no new
// message of his, it is never saved as one, and the script says how many it left out.
// `part` copies the lines of a saved text from the line that equals <from line> up to the line that equals <before line>.
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';
const NOTES = '.checkpoints/cs4-walk/';
const SESSION = 'session_01WLuqnt1v3UNwSqbJsJre7u';
const sha = (s) => createHash('sha256').update(s).digest('hex');
const size = (s) => `${Buffer.byteLength(s)} bytes, sha256 ${sha(s)}`;
const [mode, ...a] = process.argv.slice(2);
const full = (p) => {
  if (typeof p !== 'string' || !p.startsWith('/') || p.split('/').includes('..')) { console.error(`<out file> must be a full path: ${p}`); process.exit(2); }
  return p;
};
if (mode === 'words' || mode === 'exact' || mode === 'less') {
  // `exact`: the message whose whole text is the marker (a one-letter answer cannot be found as a part of a text).
  const exact = mode === 'exact';
  const less = mode === 'less';
  const [transcript, marker, ...rest] = a;
  const [start, outArg, title, answers, why, after = ''] = less ? rest : [null, ...rest.slice(0, 3), null, rest[3] ?? ''];
  const out = full(outArg);
  if (less && (typeof start !== 'string' || start === '' || typeof why !== 'string' || why === '')) { console.error('less needs a line start and a reason'); process.exit(2); }
  let hits = [];
  const seen = new Set();
  let repeats = 0;
  for (const line of readFileSync(transcript, 'utf8').split('\n')) {
    if (!line.includes(marker.slice(0, 20))) continue;
    let e;
    try { e = JSON.parse(line); } catch { continue; }
    if (e.isSidechain) continue;
    if (e.type === 'user' && !e.isMeta && !e.isCompactSummary && e.toolUseResult === undefined) {
      seen.add(e.uuid);
      const c = e.message?.content;
      const texts = typeof c === 'string' ? [c] : Array.isArray(c) ? c.filter((p) => p.type === 'text').map((p) => p.text) : [];
      for (const t of texts) if ((exact ? t.trim() === marker : t.includes(marker)) && !t.startsWith('This session is being continued')) hits.push({ t, at: e.timestamp, id: e.uuid, how: 'the user message' });
    } else if (e.type === 'attachment' && e.attachment?.type === 'queued_command' && typeof e.attachment.prompt === 'string' && (exact ? e.attachment.prompt.trim() === marker : e.attachment.prompt.includes(marker))) {
      if (seen.has(e.attachment.source_uuid)) { repeats += 1; continue; }
      hits.push({ t: e.attachment.prompt, at: e.timestamp, id: e.uuid, how: 'the message he sent while the chat was working (stored as a queued command)' });
    }
  }
  hits = hits.filter((hit) => String(hit.at) > after);
  if (repeats > 0) console.error(`left out: ${repeats} repeat(s) of a message already in the record (handed to the chat again, not sent again)`);
  if (hits.length !== 1) { console.error(`expected one message with the marker, found ${hits.length}`); process.exit(1); }
  const { t, at, id, how } = hits[0];
  if (less) {
    const lines = t.split('\n');
    const left = lines.flatMap((l, i) => (l.startsWith(start) ? [i] : []));
    if (left.length !== 1) { console.error(`expected one line that starts with the line start, found ${left.length}`); process.exit(1); }
    const shown = lines.filter((_, i) => i !== left[0]).join('\n');
    writeFileSync(out, `# ${title}\n\n_Copied by script from the chat's transcript (${SESSION}), never retyped: ${how} of ${at} (uuid ${id}), shown here without its line ${left[0] + 1} of ${lines.length}, which is left out because ${why} (${size(shown)}). What it answers: ${answers}_\n\n---\n${shown}\n`);
    console.log(`${out}: shown without line ${left[0] + 1} of ${lines.length}: ${size(shown)}; sent ${at}, uuid ${id}`);
    console.log(`not written to the file, for the chat only: the whole message, ${size(t)}`);
  } else {
    writeFileSync(out, `# ${title}\n\n_Copied by script from the chat's transcript (${SESSION}), never retyped: ${how} of ${at} (uuid ${id}), whole (${size(t)}). What it answers: ${answers}_\n\n---\n${t}\n`);
    console.log(`${out}: ${size(t)}, sent ${at}, uuid ${id}`);
  }
} else if (mode === 'part') {
  const [file, from, before, outArg, title] = a;
  const out = full(outArg);
  const note = readFileSync(NOTES + file, 'utf8');
  const lines = note.slice(note.indexOf('\n---\n') + 5).replace(/\n$/, '').split('\n');
  const i = lines.indexOf(from);
  const j = lines.indexOf(before);
  if (i < 0 || j <= i || lines.lastIndexOf(from) !== i || lines.lastIndexOf(before) !== j) { console.error('each of the two lines must stand once, in that order'); process.exit(1); }
  const t = lines.slice(i, j).join('\n').replace(/\n+$/, '');
  writeFileSync(out, `# ${title}\n\n_Copied by script from \`${file}\`, never retyped: its lines ${i + 1} to ${j} of ${lines.length} (${size(t)})._\n\n---\n${t}\n`);
  console.log(`${out}: lines ${i + 1} to ${j} of ${lines.length}, ${size(t)}`);
} else { console.error('words, exact, less or part'); process.exit(2); }

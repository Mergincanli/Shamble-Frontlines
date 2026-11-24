// CI Builder's chat (session_019YNMUUemXge56777MiUWFz; adapted from Builder V3's script at 6491b839): copies one of Mustafa's messages from the chat's transcript
// into the notes, byte for byte, with the question it answers. A message typed while the chat was working is stored
// as a queued command, so both kinds are read. From the repository root:
//   node .checkpoints/ci-after-3a/chat-words.mjs words <transcript.jsonl> <marker> <out file> <title> <what it answers>
//   node .checkpoints/ci-after-3a/chat-words.mjs part  <saved file> <from line> <before line> <out file> <title>
// `part` copies the lines of a saved text from the line that equals <from line> up to the line that equals <before line>.
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';
const NOTES = '.checkpoints/ci-after-3a/';
const sha = (s) => createHash('sha256').update(s).digest('hex');
const size = (s) => `${Buffer.byteLength(s)} bytes, sha256 ${sha(s)}`;
const [mode, ...a] = process.argv.slice(2);
if (mode === 'words') {
  const [transcript, marker, out, title, answers] = a;
  const hits = [];
  for (const line of readFileSync(transcript, 'utf8').split('\n')) {
    if (!line.includes(marker.slice(0, 20))) continue;
    let e;
    try { e = JSON.parse(line); } catch { continue; }
    if (e.isSidechain) continue;
    if (e.type === 'user' && !e.isMeta && !e.isCompactSummary && e.toolUseResult === undefined) {
      const c = e.message?.content;
      const texts = typeof c === 'string' ? [c] : Array.isArray(c) ? c.filter((p) => p.type === 'text').map((p) => p.text) : [];
      for (const t of texts) if (t.includes(marker) && !t.startsWith('This session is being continued')) hits.push({ t, at: e.timestamp, id: e.uuid, how: 'the user message' });
    } else if (e.type === 'attachment' && e.attachment?.type === 'queued_command' && typeof e.attachment.prompt === 'string' && e.attachment.prompt.includes(marker)) {
      hits.push({ t: e.attachment.prompt, at: e.timestamp, id: e.uuid, how: 'the message he sent while the chat was working (stored as a queued command)' });
    }
  }
  if (hits.length !== 1) { console.error(`expected one message with the marker, found ${hits.length}`); process.exit(1); }
  const { t, at, id, how } = hits[0];
  writeFileSync(NOTES + out, `# ${title}\n\n_Copied by script from the chat's transcript (session_019YNMUUemXge56777MiUWFz), never retyped: ${how} of ${at} (uuid ${id}), whole (${size(t)}). What it answers: ${answers}_\n\n---\n${t}\n`);
  console.log(`${out}: ${size(t)}, sent ${at}, uuid ${id}`);
} else if (mode === 'part') {
  const [file, from, before, out, title] = a;
  const note = readFileSync(NOTES + file, 'utf8');
  const lines = note.slice(note.indexOf('\n---\n') + 5).replace(/\n$/, '').split('\n');
  const i = lines.indexOf(from);
  const j = lines.indexOf(before);
  if (i < 0 || j <= i || lines.lastIndexOf(from) !== i || lines.lastIndexOf(before) !== j) { console.error('each of the two lines must stand once, in that order'); process.exit(1); }
  const t = lines.slice(i, j).join('\n').replace(/\n+$/, '');
  writeFileSync(NOTES + out, `# ${title}\n\n_Copied by script from \`${file}\`, never retyped: its lines ${i + 1} to ${j} of ${lines.length} (${size(t)})._\n\n---\n${t}\n`);
  console.log(`${out}: lines ${i + 1} to ${j} of ${lines.length}, ${size(t)}`);
} else { console.error('words or part'); process.exit(2); }

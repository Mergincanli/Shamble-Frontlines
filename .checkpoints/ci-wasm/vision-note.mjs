// The dated note for docs/VISION_V2.md, as the build chat put it to Mustafa at 2026-11-19, 14:29:51 UTC and as he
// answered "a, yes" to at 14:31:30 UTC (VISION-NOTE (a)). The note is never retyped: this script copies the chat's
// message from the session's transcript by its uuid, saves it whole beside this file, and takes from it the note's
// block, the "Updated on" sentence and the question with its options (the advisor agent's condition 3 on the plan).
//   node .checkpoints/ci-wasm/vision-note.mjs <transcript.jsonl>            save the message and print the parts
//   node .checkpoints/ci-wasm/vision-note.mjs --apply [--out <file>]        write the note into docs/VISION_V2.md
//   node .checkpoints/ci-wasm/vision-note.mjs --check [--out <file>]        compare the file on disk
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { existsSync, readFileSync, writeFileSync } from 'node:fs';
import { pathToFileURL } from 'node:url';

const UUID = '699736ec-f365-4458-aceb-592bba842d59';
const SAVED = '.checkpoints/ci-wasm/chat-2026-11-19T142951-vision-note.md';
const VISION = 'docs/VISION_V2.md';
const BEFORE = 'f5030a33185b1ea1b15329174e82823f0215f993';
const sha = (s) => createHash('sha256').update(s).digest('hex');
// Imported by decisions-3.mjs for the parts below: then it reads no argument and prints nothing.
const isMain = process.argv[1] !== undefined && import.meta.url === pathToFileURL(process.argv[1]).href;
const args = isMain ? process.argv.slice(2) : [];
const at = args.indexOf('--out');
const OUT = at === -1 ? VISION : args[at + 1];

const transcript = args.find((a) => a.endsWith('.jsonl'));
if (transcript !== undefined) {
  let hit;
  for (const line of readFileSync(transcript, 'utf8').split('\n')) {
    if (!line.includes(UUID)) continue;
    let e;
    try { e = JSON.parse(line); } catch { continue; }
    if (e.uuid !== UUID || e.type !== 'assistant') continue;
    const texts = e.message.content.filter((p) => p.type === 'text').map((p) => p.text);
    if (texts.length !== 1) throw new Error(`expected one text part, found ${texts.length}`);
    hit = { text: texts[0], time: e.timestamp };
  }
  if (hit === undefined) throw new Error('the message is not in the transcript');
  writeFileSync(
    SAVED,
    `# The build chat's message of ${hit.time} (uuid ${UUID}): the dated note's text and the question VISION-NOTE\n\n` +
      `_The chat's own message (not Mustafa's words), copied by script from the session's transcript, whole ` +
      `(${Buffer.byteLength(hit.text)} bytes, sha256 ${sha(hit.text)}). His answer to it: "a, yes", 14:31:30 UTC._\n\n---\n${hit.text}\n`,
  );
}
if (!existsSync(SAVED)) throw new Error(`run once with the transcript: ${SAVED} is missing`);
const saved = readFileSync(SAVED, 'utf8');
const text = saved.slice(saved.indexOf('\n---\n') + 5).replace(/\n$/, '');
const said = saved.match(/\((\d+) bytes, sha256 ([0-9a-f]{64})\)/);
if (!said || Number(said[1]) !== Buffer.byteLength(text) || said[2] !== sha(text)) throw new Error('the saved message differs from its own count or sha256');

// The two fenced blocks of the message, in order: the note, then the sentence for the "Updated on" lines.
const blocks = [...text.matchAll(/```\n([\s\S]*?)\n```/g)].map((m) => m[1]);
if (blocks.length !== 2) throw new Error(`expected two fenced blocks, found ${blocks.length}`);
export const NOTE = blocks[0];
export const SENTENCE = blocks[1];
const one = (re) => {
  const m = text.match(re);
  if (!m) throw new Error(`not in the message: ${re}`);
  return m[1];
};
export const QUESTION = one(/\*\*(VISION-NOTE: [^*]+)\*\*/);
export const OPTION_A = one(/^- (\(a\) [^*\n]+?)\s*\*\*Recommended/m);
export const OPTION_B = one(/^- (\(b\) [^\n]+)$/m);

if (args.includes('--apply') || args.includes('--check')) {
  const before = execFileSync('git', ['show', `${BEFORE}:${VISION}`], { maxBuffer: 1 << 24 }).toString('utf8');
  const LINE = '- `[DECIDED]` Desktop browsers first: Chrome, Edge, Firefox and Safari. Mobile comes later but is not designed out.\n';
  const UPDATED = "2026-11-04 (pillar 5, and the pointers in §5 and §7); on 2026-11-07, his private-match XP item of 2026-11-06 (§5).\n";
  if (before.split(LINE).length !== 2) throw new Error('the decided browser line must stand once');
  if (before.split(UPDATED).length !== 2) throw new Error('the last "Updated on" line must stand once');
  const after = before.replace(LINE, `${LINE}${NOTE}\n`).replace(UPDATED, `${UPDATED}${SENTENCE}\n`);
  if (args.includes('--apply')) writeFileSync(OUT, after);
  const disk = readFileSync(OUT, 'utf8');
  const ok = {
    'the file on disk is what the script writes': disk === after,
    'with the note and the sentence taken out, every byte equals main\'s file': disk.replace(`${NOTE}\n`, '').replace(`${SENTENCE}\n`, '') === before,
    'the decided line stands unchanged, with the note right under it': disk.includes(`${LINE}${NOTE}\n`),
    'no line over 120 characters added': `${NOTE}\n${SENTENCE}`.split('\n').every((l) => l.length <= 120),
  };
  for (const [k, v] of Object.entries(ok)) console.log(`${v ? 'yes' : 'NO '}  ${k}`);
  process.exitCode = Object.values(ok).every(Boolean) ? 0 : 1;
} else if (isMain) {
  console.log(`saved message: ${said[1]} bytes, sha256 ${said[2]}`);
  console.log(`--- the note (${Buffer.byteLength(NOTE)} bytes, sha256 ${sha(NOTE)}):\n${NOTE}`);
  console.log(`--- the sentence (${Buffer.byteLength(SENTENCE)} bytes, sha256 ${sha(SENTENCE)}):\n${SENTENCE}`);
  console.log(`--- the question: ${QUESTION}\n--- option a: ${OPTION_A}\n--- option b: ${OPTION_B}`);
}

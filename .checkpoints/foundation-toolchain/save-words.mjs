// Saves one of Mustafa's chat messages byte for byte from this session's transcript (the .jsonl Claude Code keeps), so
// his words reach the notes by script, never retyped. Usage:
//   node save-words.mjs <transcript.jsonl> <message uuid> <out.md> <header file>
// The out file is: the header file's text, a blank line, then the message inside a ```text fence. The script re-reads
// the out file and checks that the fenced text equals the transcript's text byte for byte; exit 1 if not.
import { readFileSync, writeFileSync } from 'node:fs';

const [jsonl, uuid, out, headerFile] = process.argv.slice(2);
if (!jsonl || !uuid || !out || !headerFile) { console.error('usage: node save-words.mjs <jsonl> <uuid> <out.md> <header>'); process.exit(2); }
let text = null, ts = null;
for (const line of readFileSync(jsonl, 'utf8').split('\n')) {
  if (!line) continue;
  let o; try { o = JSON.parse(line); } catch { continue; }
  if (o.type !== 'user' || o.uuid !== uuid) continue;
  const c = o.message?.content;
  const texts = typeof c === 'string' ? [c] : Array.isArray(c) ? c.filter((b) => b.type === 'text').map((b) => b.text) : [];
  if (texts.length !== 1) { console.error(`message ${uuid} has ${texts.length} text blocks; expected 1`); process.exit(1); }
  text = texts[0]; ts = o.timestamp;
}
if (text === null) { console.error(`no user message ${uuid} in ${jsonl}`); process.exit(1); }
if (text.includes('```')) { console.error('the message holds a ``` fence; this script cannot fence it safely'); process.exit(1); }
const header = readFileSync(headerFile, 'utf8').replace(/\s+$/, '');
writeFileSync(out, `${header}\n\n\`\`\`text\n${text}\n\`\`\`\n`);
const back = readFileSync(out, 'utf8');
const start = back.indexOf('```text\n') + 8, end = back.lastIndexOf('\n```\n');
const ok = back.slice(start, end) === text;
console.log(`${ok ? 'OK' : 'MISMATCH'}: ${uuid} (${ts}), ${Buffer.byteLength(text)} bytes, ${text.split('\n').length} lines -> ${out}`);
process.exit(ok ? 0 : 1);

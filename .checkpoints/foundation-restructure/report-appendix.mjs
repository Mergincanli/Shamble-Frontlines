// report-appendix.mjs (foundation-restructure, step 2.1b): builds the report's "Appendix A. Mustafa's words for this
// change-set, in full" from the capture files in the notes folder (each written by capture-message.mjs from the
// session's own events, never retyped), and checks it.
//   node .gate/foundation-restructure/report-appendix.mjs build   -> writes .checkpoints/foundation-restructure/appendix-a.md
//   node .gate/foundation-restructure/report-appendix.mjs check <report.md>
//        -> every block of the report's Appendix A equals its capture byte for byte, and the appendix equals a fresh build
// A message sent twice is one block; the script asserts the two sends are byte-identical first.
import { readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const REPO = '/home/user/Shamble-Frontlines';
const NOTES = join(REPO, '.checkpoints/foundation-restructure');
const SESSION = 'session_019YNMUWdGJZoQFUuuvvBP1Y';
const MSGS = [
  { id: 'A1', file: 'mustafa-2026-11-11-hi.md', title: 'His first message in this session' },
  { id: 'A2', file: 'mustafa-2026-11-11-task.md', title: 'His task message: step 2.1b, the move' },
  { id: 'A3', file: 'mustafa-2026-11-11-d1.md', also: 'mustafa-2026-11-11-d1-resent.md', title: 'His D1 message: D1 (a), his Windows run of the new gate, the Node 24 finding for 2.3' },
  { id: 'A4', file: 'mustafa-2026-11-11-permission.md', title: 'His message on auto mode' },
  { id: 'A5', file: 'mustafa-2026-11-11-answers.md', title: 'His answers: KIT-2.1B (a), E8-RUN (a), REG-TRUE (b), and the build go for steps 6 and 7' },
];
export function readCapture(file) {
  const s = readFileSync(join(NOTES, file), 'utf8');
  const ev = s.match(/event ([0-9a-f-]{36}), at\n(\S+), extracted/);
  const body = s.match(/\n```text\n([\s\S]*)\n```\n?$/);
  if (!ev || !body) throw new Error(`${file}: not a capture-message.mjs file`);
  return { event: ev[1], when: ev[2], text: body[1] };
}
const utc = w => w.replace(/^(\d{4}-\d\d-\d\d)T(\d\d:\d\d:\d\d)\.\d+Z$/, '$1 $2 UTC');
export function build() {
  let out = `## Appendix A. Mustafa's words for this change-set, in full\n\n_Each block below is copied byte for byte from its source by \`report-appendix.mjs\` (in the notes folder, readable at this branch's last checkpoint commit), never retyped: his messages in this session (${SESSION}, his personal account), each extracted by \`capture-message.mjs\` from the session's own events. The same script checks every block against its source. Entries in \`docs/DECISIONS.md\` copy their quotes from here, because the notes folder is deleted before the PR is marked ready._\n`;
  const blocks = [];
  for (const m of MSGS) {
    const c = readCapture(m.file);
    let head = `### ${m.id}. ${m.title}, ${utc(c.when)} (event ${c.event}: \`${m.file}\``;
    if (m.also) {
      const d = readCapture(m.also);
      if (d.text !== c.text) throw new Error(`${m.id}: the two sends differ`);
      head += `; sent again at ${utc(d.when).slice(11)}, event ${d.event}: \`${m.also}\`, byte-identical, checked by this script`;
    }
    head += ')';
    out += `\n${head}\n\n\`\`\`text\n${c.text}\n\`\`\`\n`;
    blocks.push({ id: m.id, ...c });
  }
  return { out, blocks };
}
export function appendixBlocks(md) {
  const at = md.indexOf("## Appendix A. Mustafa's words for this change-set, in full");
  if (at < 0) throw new Error('no Appendix A in the report');
  const rest = md.slice(at);
  const blocks = {};
  for (const m of rest.matchAll(/\n### (A\d+)\. [^\n]*\n\n```text\n([\s\S]*?)\n```\n/g)) blocks[m[1]] = m[2];
  return { text: rest, blocks };
}
const [cmd, report] = process.argv.slice(2);
if (cmd === 'build') {
  const { out, blocks } = build();
  writeFileSync(join(NOTES, 'appendix-a.md'), out);
  for (const b of blocks) console.log(`${b.id}: ${b.text.length} characters, event ${b.event}`);
  console.log('appendix-a.md written');
} else if (cmd === 'check') {
  const md = readFileSync(join(REPO, report), 'utf8');
  const { out } = build();
  const { text, blocks } = appendixBlocks(md);
  let ok = true;
  for (const m of MSGS) {
    const c = readCapture(m.file);
    const same = blocks[m.id] === c.text;
    if (!same) ok = false;
    console.log(`${same ? 'PASS' : 'FAIL'} ${m.id} equals ${m.file} byte for byte (${c.text.length} characters)`);
  }
  const whole = text.startsWith(out);
  if (!whole) ok = false;
  console.log(`${whole ? 'PASS' : 'FAIL'} the report's Appendix A starts with a fresh build, byte for byte`);
  console.log('appendix: ' + (ok ? 'PASS' : 'FAIL'));
  process.exit(ok ? 0 : 1);
} else if (cmd) { console.error('usage: report-appendix.mjs build | check <report.md>'); process.exit(2); }

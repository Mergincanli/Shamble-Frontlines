// report-appendix.mjs (foundation-restructure, step 2.1b): builds the report's "Appendix A. Mustafa's words for this
// change-set, in full" from the capture files in the notes folder (each written by capture-message.mjs from a session's
// own events, never retyped), and checks it.
//   node .gate/foundation-restructure/report-appendix.mjs build   -> writes .checkpoints/foundation-restructure/appendix-a.md
//   node .gate/foundation-restructure/report-appendix.mjs check <report.md>
//        -> every block of the report's Appendix A equals its capture byte for byte, the appendix equals a fresh build,
//           and A7 equals merge commit 76b4f7a's quoted record of it
// A message sent twice is one block (the sends asserted byte-identical first); an earlier, different message he
// interrupted is named in the next block's heading with its text from its own capture. Each heading carries exactly one
// "(event <uuid>": its block's own (decisions-entries.mjs keys on it); other sends are written ", event <uuid>".
import { readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { execFileSync } from 'node:child_process';

const REPO = '/home/user/Shamble-Frontlines';
const NOTES = join(REPO, '.checkpoints/foundation-restructure');
const SESSION = 'session_019YNMUWdGJZoQFUuuvvBP1Y';
const MSGS = [
  { id: 'A1', file: 'mustafa-2026-11-11-hi.md', title: 'His message before the task',
    earlierOther: { file: 'mustafa-2026-11-11-hi-first.md', interrupted: '20:20:24' } },
  { id: 'A2', file: 'mustafa-2026-11-11-task.md', title: 'His task message: step 2.1b, the move',
    earlierSame: { file: 'mustafa-2026-11-11-task-first.md', interrupted: '20:19:40' } },
  { id: 'A3', file: 'mustafa-2026-11-11-d1.md', also: 'mustafa-2026-11-11-d1-resent.md', title: 'His D1 message: D1 (a), his Windows run of the new gate, the Node 24 finding for 2.3' },
  { id: 'A4', file: 'mustafa-2026-11-11-permission.md', title: 'His message on auto mode' },
  { id: 'A5', file: 'mustafa-2026-11-11-answers.md', title: 'His answers: KIT-2.1B (a), E8-RUN (a), REG-TRUE (b), and the build go for steps 6 and 7' },
  { id: 'A6', file: 'mustafa-2026-11-11-auto.md', title: 'His second message on auto mode' },
  { id: 'A7', file: 'mustafa-2026-11-11-go50.md', title: "His go for #50, in foundation 2.1a's session (session_019YNMUWLffMoE47SbvkqJ5P)", mergeCommit: '76b4f7a' },
];
export function readCapture(file) {
  const s = readFileSync(join(NOTES, file), 'utf8');
  const ev = s.match(/event ([0-9a-f-]{36}), at\n(\S+), extracted/);
  const body = s.match(/\n```text\n([\s\S]*)\n```\n?$/);
  if (!ev || !body) throw new Error(`${file}: not a capture-message.mjs file`);
  return { event: ev[1], when: ev[2], text: body[1] };
}
// the merge commit's "> " record of his message, the prefix removed (a bare ">" is an empty line)
function mergeRecord(sha) {
  const msg = execFileSync('git', ['log', '-1', '--format=%B', sha], { cwd: REPO, encoding: 'utf8' }).split('\n');
  const i = msg.findIndex(l => l.startsWith('> ')); let j = i;
  while (j < msg.length && (msg[j] === '>' || msg[j].startsWith('> '))) j++;
  return msg.slice(i, j).map(l => (l === '>' ? '' : l.slice(2))).join('\n');
}
const utc = w => w.replace(/^(\d{4}-\d\d-\d\d)T(\d\d:\d\d:\d\d)\.\d+Z$/, '$1 $2 UTC');
const hhmmss = w => utc(w).slice(11, 19);
export function build() {
  let out = `## Appendix A. Mustafa's words for this change-set, in full\n\n_Each block below is copied byte for byte from its source by \`report-appendix.mjs\` (in the notes folder, readable at this branch's last checkpoint commit), never retyped: A1 to A6 are his messages in this session (${SESSION}, his personal account), each extracted by \`capture-message.mjs\` from the session's own events; A7 is his go for #50, extracted the same way from foundation 2.1a's session, and equal to merge commit 76b4f7a's quoted record of it. The same script checks every block against its source. His other input in this session was commands, not messages: \`/advisor fable\` four times (20:19 to 20:21 UTC) and \`/compact\` at 21:35:45 UTC with his instructions for the summary (the session log). Entries in \`docs/DECISIONS.md\` copy their quotes from here, because the notes folder is deleted before the PR is marked ready._\n`;
  const blocks = [];
  for (const m of MSGS) {
    const c = readCapture(m.file);
    let head = `### ${m.id}. ${m.title}, ${utc(c.when)} (event ${c.event}: \`${m.file}\``;
    if (m.earlierSame) {
      const d = readCapture(m.earlierSame.file);
      if (d.text !== c.text) throw new Error(`${m.id}: the first send differs`);
      head += `; first sent at ${hhmmss(d.when)} UTC, event ${d.event}: \`${m.earlierSame.file}\`, byte-identical (checked by this script), and interrupted by him at ${m.earlierSame.interrupted} UTC (the session log)`;
    }
    if (m.earlierOther) {
      const d = readCapture(m.earlierOther.file);
      if (d.text.includes('"') || d.text.includes('\n')) throw new Error(`${m.id}: the earlier message cannot sit in a heading`);
      head += `; before it, at ${hhmmss(d.when)} UTC, he sent "${d.text}", event ${d.event}: \`${m.earlierOther.file}\`, and interrupted it at ${m.earlierOther.interrupted} UTC (the session log)`;
    }
    if (m.also) {
      const d = readCapture(m.also);
      if (d.text !== c.text) throw new Error(`${m.id}: the two sends differ`);
      head += `; sent again at ${hhmmss(d.when)}, event ${d.event}: \`${m.also}\`, byte-identical, checked by this script`;
    }
    if (m.mergeCommit) {
      if (mergeRecord(m.mergeCommit) !== c.text) throw new Error(`${m.id}: differs from merge commit ${m.mergeCommit}'s record`);
      head += `; byte-identical to merge commit ${m.mergeCommit}'s record of it (quoted there with "> "), checked by this script`;
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
    if (m.mergeCommit) { const mm = mergeRecord(m.mergeCommit) === c.text; if (!mm) ok = false; console.log(`${mm ? 'PASS' : 'FAIL'} ${m.id} equals merge commit ${m.mergeCommit}'s record byte for byte`); }
  }
  const whole = text.startsWith(out);
  if (!whole) ok = false;
  console.log(`${whole ? 'PASS' : 'FAIL'} the report's Appendix A starts with a fresh build, byte for byte`);
  console.log('appendix: ' + (ok ? 'PASS' : 'FAIL'));
  process.exit(ok ? 0 : 1);
} else if (cmd) { console.error('usage: report-appendix.mjs build | check <report.md>'); process.exit(2); }

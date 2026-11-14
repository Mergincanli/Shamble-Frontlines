import { readFileSync, writeFileSync } from 'node:fs';
const [, , f, outDir] = process.argv;
const lines = readFileSync(f, 'utf8').split('\n').filter(Boolean);
let task = null, add = null;
for (const l of lines) {
  let o; try { o = JSON.parse(l); } catch { continue; }
  if (o.type === 'user' && o.message) {
    const c = o.message.content;
    const texts = typeof c === 'string' ? [c] : (Array.isArray(c) ? c.filter((b) => b.type === 'text').map((b) => b.text) : []);
    for (const t of texts) if (!task && t.startsWith('My words of 2026-11-14, for a fresh chat')) task = { ts: o.timestamp, uuid: o.uuid, text: t };
  }
  if (o.type === 'queue-operation' && !add) {
    const c = typeof o.content === 'string' ? o.content : (Array.isArray(o.content) ? o.content.filter((b) => b.type === 'text').map((b) => b.text).join('') : '');
    if (c.includes('an addition to my block of today')) add = { ts: o.timestamp, op: o.operation, text: c };
  }
}
if (!task || !add) { console.error('missing', !!task, !!add); process.exit(1); }
const head = (title, meta) => `# ${title}\n\n_Copied by script (extract2.mjs, in the session's scratchpad) from this session's transcript, never retyped: ${meta}. The text below the line is his, byte for byte._\n\n---\n`;
writeFileSync(`${outDir}/mustafa-2026-11-14-task.md`, head('Mustafa, 2026-11-14: the task for this chat', `the user message of ${task.ts} (uuid ${task.uuid}), ${Buffer.byteLength(task.text)} bytes`) + task.text + '\n');
writeFileSync(`${outDir}/mustafa-2026-11-14-addition.md`, head('Mustafa, 2026-11-14: his addition (Blender; items 11 to 13)', `the queued user message of ${add.ts} (queue operation "${add.op}"), ${Buffer.byteLength(add.text)} bytes`) + add.text + '\n');
console.log('task', task.ts, Buffer.byteLength(task.text), 'addition', add.ts, add.op, Buffer.byteLength(add.text));

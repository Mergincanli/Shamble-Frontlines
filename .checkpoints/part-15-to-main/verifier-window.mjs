// Verifier: lists the main thread's tool calls (name + command or description, first 160 chars) and the user/assistant
// records between two timestamps. Prints no tool output, only the first 160 chars of each result's text.
//   node verifier-window.mjs <transcript.jsonl> <fromISO> <toISO> [--results]
import { createReadStream } from 'node:fs';
import { createInterface } from 'node:readline';
const [file, from, to] = process.argv.slice(2); const showRes = process.argv.includes('--results');
const rl = createInterface({ input: createReadStream(file), crlfDelay: Infinity });
let n = 0;
for await (const line of rl) {
  n++; let r; try { r = JSON.parse(line); } catch { continue; }
  if (!r.timestamp || r.timestamp < from || r.timestamp > to) continue;
  const c = r.message && r.message.content; const blocks = typeof c === 'string' ? [{ type: 'string', text: c }] : (Array.isArray(c) ? c : []);
  for (const b of blocks) {
    if (b.type === 'tool_use') console.log(n, r.timestamp, r.isSidechain ? 'SIDE' : 'MAIN', 'TOOL', b.name, JSON.stringify(b.input.command || b.input.description || b.input.file_path || b.input.prompt || '').slice(0, 160));
    else if (b.type === 'tool_result' && showRes) { const t = typeof b.content === 'string' ? b.content : (Array.isArray(b.content) ? b.content.map((x) => x.text || '').join(' ') : ''); console.log(n, r.timestamp, 'RESULT', JSON.stringify(t.slice(0, 160))); }
    else if (b.type === 'string' || b.type === 'text') console.log(n, r.timestamp, r.type, r.userType || '', 'TEXT', JSON.stringify(b.text.slice(0, 100)));
  }
}

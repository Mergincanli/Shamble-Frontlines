// Saves a list_events page that came back inline: finds, in a subagent transcript, the tool result whose text is the
// {"ccr":...} object with the given first_id, and writes that text unchanged to <out>.
// Run: node extract-inline-page.mjs <transcript.jsonl> <first_id> <out.txt>
import { readFileSync, writeFileSync } from 'node:fs';
const [t, firstId, out] = process.argv.slice(2);
let found = null;
for (const line of readFileSync(t, 'utf8').split('\n')) {
  if (!line.includes(firstId)) continue;
  let j; try { j = JSON.parse(line); } catch { continue; }
  const c = j.message && Array.isArray(j.message.content) ? j.message.content : [];
  for (const b of c) {
    if (b.type !== 'tool_result') continue;
    const text = typeof b.content === 'string' ? b.content : (b.content || []).map(x => x.text || '').join('');
    if (!text.startsWith('{"ccr"')) continue;
    const d = JSON.parse(text).ccr;
    if (d.first_id === firstId) found = text;
  }
}
if (!found) { console.error('not found'); process.exit(1); }
writeFileSync(out, found);
console.log('wrote', out, found.length, 'chars');

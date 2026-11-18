// Verifier: prints each main-thread Agent call's subagent_type, description and effort (no prompt text) in a window.
import { createReadStream } from 'node:fs';
import { createInterface } from 'node:readline';
const [file, from, to] = process.argv.slice(2);
const rl = createInterface({ input: createReadStream(file), crlfDelay: Infinity }); let n = 0;
for await (const line of rl) { n++; let r; try { r = JSON.parse(line); } catch { continue; }
  if (!r.timestamp || r.timestamp < from || r.timestamp > to || r.isSidechain) continue;
  const c = r.message && r.message.content; if (!Array.isArray(c)) continue;
  for (const b of c) if (b.type === 'tool_use' && b.name === 'Agent') console.log(n, r.timestamp, b.input.subagent_type, '|', b.input.description, '|', b.input.model || '', b.input.effort || '');
  for (const b of c) if (b.type === 'tool_result') { const t = typeof b.content === 'string' ? b.content : Array.isArray(b.content) ? b.content.map((x) => x.text || '').join('') : ''; const m = /agentId: ([0-9a-f]+)/.exec(t) || /agent[_ ]?id[:=]\s*"?([0-9a-f]{10,})/i.exec(t); if (m) console.log(n, r.timestamp, 'agent id', m[1]); }
}

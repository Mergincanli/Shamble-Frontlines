// Read-only cross-check of prompt-tally.mjs's output: unique refusals by tool_use_id (server + transcripts), the asked
// calls' commands (first 140 chars), and transcript tool calls with no result in the server pages, by agent.
// Run: node tally-crosscheck.mjs --transcript <f> ... <page> ...
import { readFileSync } from 'node:fs';
const args = process.argv.slice(2); const tr = [];
for (let i = args.indexOf('--transcript'); i >= 0; i = args.indexOf('--transcript')) tr.push(args.splice(i, 2)[1]);
const RX = /hook error|has been denied|denied by|not allowed here/i;
const srv = new Map(), refusals = new Map(), asked = [], noDecision = [];
for (const f of args) {
  const raw = readFileSync(f, 'utf8');
  const d = JSON.parse(raw.slice(raw.indexOf('{"ccr"'), raw.lastIndexOf('}') + 1)).ccr;
  for (const e of d.data) { const inner = (e.user || {}).internal_anthropic_catchall || e.user; const m = inner && inner.message;
    if (!m || !Array.isArray(m.content)) continue;
    for (const b of m.content) if (b.type === 'tool_result') {
      const meta = (inner.tool_result_meta || []).find(x => x.id === b.tool_use_id);
      srv.set(b.tool_use_id, { meta: meta && meta.permission_decision, agent: inner.subagent_type || (inner.agent_id ? inner.agent_id : 'main') });
      const text = typeof b.content === 'string' ? b.content : (b.content || []).map(c => c.text || '').join('');
      if (RX.test(text) && b.is_error !== false) refusals.set(b.tool_use_id, 'server');
      if (!meta) noDecision.push(b.tool_use_id);
      if (meta && meta.permission_decision.source === 'user_temporary') asked.push(b.tool_use_id);
    } }
}
const uses = new Map(), missing = {};
for (const t of tr) { const who = t.includes('/subagents/') ? t.split('/').pop() : 'main';
  for (const line of readFileSync(t, 'utf8').split('\n')) { if (!line.trim()) continue; let j; try { j = JSON.parse(line); } catch { continue; }
    const c = j.message && Array.isArray(j.message.content) ? j.message.content : [];
    for (const b of c) { if (b.type === 'tool_use') uses.set(b.id, { who, name: b.name, cmd: String((b.input || {}).command || '').slice(0, 140), at: j.timestamp });
      if (b.type === 'tool_result') { const text = typeof b.content === 'string' ? b.content : (b.content || []).map(x => x.text || '').join('');
        if (b.is_error && RX.test(text) && !refusals.has(b.tool_use_id)) refusals.set(b.tool_use_id, 'transcript only'); } } } }
for (const [id, u] of uses) if (!srv.has(id)) { const k = u.who + (u.at && u.at > '2026-11-11T16:37:56' ? ' (after last page)' : ''); missing[k] = (missing[k] || 0) + 1; }
const rs = {}; for (const v of refusals.values()) rs[v] = (rs[v] || 0) + 1;
console.log('server results', srv.size, 'no decision', noDecision.length, 'transcript calls', uses.size);
console.log('unique refusals by tool_use_id', refusals.size, rs);
console.log('transcript calls with no server result, by transcript:', missing);
console.log('asked calls:'); for (const id of asked) { const u = uses.get(id); console.log(' -', u ? u.who + ' | ' + u.at + ' | ' + u.cmd.replace(/\s+/g, ' ') : id + ' (no tool_use found)'); }

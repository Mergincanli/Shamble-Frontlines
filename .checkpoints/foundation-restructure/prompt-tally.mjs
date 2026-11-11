// Tallies this session's permission decisions from saved list_events pages (kinds user + assistant): which tool calls
// were answered by Mustafa (source "user_temporary"), allowed by the auto classifier or by a settings rule, or denied; and
// every refusal by a hook (the guard: "PreToolUse:... hook error") or the permission system ("Permission to use ...
// has been denied"). Run: node .gate/foundation-restructure/prompt-tally.mjs <saved-file> [<saved-file> ...]
import { readFileSync, writeFileSync } from 'node:fs';

const args = process.argv.slice(2);
const transcripts = [];
for (let i = args.indexOf('--transcript'); i >= 0; i = args.indexOf('--transcript')) transcripts.push(args.splice(i, 2)[1]);
const events = new Map();
for (const f of args) {
  const raw = readFileSync(f, 'utf8');
  const d = JSON.parse(raw.slice(raw.indexOf('{"ccr"'), raw.lastIndexOf('}') + 1)).ccr;
  for (const e of d.data) {
    const body = e.user || e.assistant; if (!body) continue;
    events.set(body.uuid || JSON.stringify(e).slice(0, 200), e);
  }
}
const uses = new Map();      // tool_use_id -> { name, input, at }
const results = [];          // { id, meta, text, isError, at }
for (const e of events.values()) {
  const body = e.user || e.assistant; const inner = body.internal_anthropic_catchall || body;
  const msg = inner.message; if (!msg || !Array.isArray(msg.content)) continue;
  for (const b of msg.content) {
    if (b.type === 'tool_use') uses.set(b.id, { name: b.name, input: b.input || {}, at: e.created_at });
    if (b.type === 'tool_result') {
      const meta = (inner.tool_result_meta || []).find(m => m.id === b.tool_use_id);
      const text = typeof b.content === 'string' ? b.content : (b.content || []).map(c => c.text || '').join('');
      results.push({ id: b.tool_use_id, meta: meta && meta.permission_decision, text, isError: !!b.is_error, at: e.created_at });
    }
  }
}
// The session's local transcript (one JSON message per line) holds every tool call's id, name and input, including those
// on pages fetched with kinds ["user"] only.
const localResults = [];   // tool results seen in the transcripts (for refusals in subagents too)
for (const t of transcripts) {
  const who = t.includes('/subagents/') ? t.split('/').pop().replace(/\.jsonl$/, '') : 'main session';
  for (const line of readFileSync(t, 'utf8').split('\n')) {
    if (!line.trim()) continue;
    let j; try { j = JSON.parse(line); } catch { continue; }
    const c = j.message && Array.isArray(j.message.content) ? j.message.content : [];
    for (const b of c) {
      if (b.type === 'tool_use' && !uses.has(b.id)) uses.set(b.id, { name: b.name, input: b.input || {}, at: j.timestamp, who });
      if (b.type === 'tool_result') {
        const text = typeof b.content === 'string' ? b.content : (b.content || []).map(x => x.text || '').join('');
        localResults.push({ id: b.tool_use_id, text, isError: !!b.is_error, at: j.timestamp, who });
      }
    }
  }
}
const kind = u => {
  if (!u) return '(unknown call)';
  if (u.name !== 'Bash') return u.name;
  const first = String(u.input.command || '').trim().replace(/^cd [^&;]+&&\s*/, '').split(/\s+/).slice(0, 2).join(' ');
  return 'Bash: ' + first;
};
const bySource = {}, asked = {}, refusals = [];
for (const r of results) {
  const u = uses.get(r.id);
  const src = r.meta ? `${r.meta.decision}/${r.meta.source}${r.meta.reason_type ? '/' + r.meta.reason_type : ''}` : '(no decision recorded)';
  bySource[src] = (bySource[src] || 0) + 1;
  if (r.meta && r.meta.source === 'user_temporary') { const k = kind(u); asked[k] = (asked[k] || 0) + 1; }
  if (/hook error|has been denied|denied by|not allowed here/i.test(r.text) && r.isError !== false) {
    refusals.push({ at: r.at, call: kind(u), command: u && u.name === 'Bash' ? String(u.input.command).slice(0, 160) : (u ? u.name : ''), refusal: r.text.split('\n')[0].slice(0, 220) });
  }
}
const seen = new Set(refusals.map(r => r.at + r.refusal));
for (const r of localResults) {
  if (!r.isError || !/hook error|has been denied|denied by|not allowed here/i.test(r.text)) continue;
  const u = uses.get(r.id);
  const item = { at: r.at, who: r.who, call: kind(u), command: u && u.name === 'Bash' ? String(u.input.command).slice(0, 160) : (u ? u.name : ''), refusal: r.text.split('\n')[0].slice(0, 220) };
  if (!seen.has(item.at + item.refusal)) { refusals.push(item); seen.add(item.at + item.refusal); }
}
const out = { events: events.size, tool_calls: uses.size, tool_results: results.length, decisions_by_source: bySource,
  asked_mustafa_by_kind: Object.fromEntries(Object.entries(asked).sort((a, b) => b[1] - a[1])), refusals };
writeFileSync('.gate/foundation-restructure/prompt-tally.json', JSON.stringify(out, null, 1));
console.log(JSON.stringify({ events: out.events, tool_calls: out.tool_calls, decisions_by_source: bySource, asked_mustafa_by_kind: out.asked_mustafa_by_kind, refusals: refusals.length }, null, 1));

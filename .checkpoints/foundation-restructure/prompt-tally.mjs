// prompt-tally.mjs (foundation-restructure, step 2.1b): which tool calls of this session reached Mustafa as a prompt,
// and which were refused, from Claude Code's own session log (each tool result carries its permissionDecision). For the
// report's finding for 2.3 (he asked twice, 21:10:58 and 22:38:50 UTC, for the session to accept prompts itself).
// Read-only. Run from the repo root: node .gate/foundation-restructure/prompt-tally.mjs <session.jsonl>
import { readFileSync, writeFileSync } from 'node:fs';

const file = process.argv[2];
if (!file) { console.error('usage: prompt-tally.mjs <session.jsonl>'); process.exit(2); }
const uses = new Map(), results = new Map();
for (const line of readFileSync(file, 'utf8').split('\n')) {
  if (!line.trim()) continue;
  let o; try { o = JSON.parse(line); } catch { continue; }
  const content = o.message && Array.isArray(o.message.content) ? o.message.content : [];
  for (const c of content) {
    if (c.type === 'tool_use') uses.set(c.id, { name: c.name, input: c.input || {}, at: o.timestamp, sidechain: !!o.isSidechain });
    if (c.type === 'tool_result' && o.permissionDecision && !results.has(c.tool_use_id))
      results.set(c.tool_use_id, { pd: o.permissionDecision, at: o.timestamp, error: !!c.is_error, text: typeof c.content === 'string' ? c.content : '' });
  }
}
const what = u => {
  if (!u) return '(tool call not found)';
  const i = u.input;
  return (i.command || i.file_path || i.url || i.query || i.description || JSON.stringify(i)).toString().replace(/\s+/g, ' ').slice(0, 140);
};
const tally = {}, prompts = [], refused = [];
for (const [id, r] of results) {
  const u = uses.get(id);
  const key = `${r.pd.decision}/${r.pd.source}${r.pd.reasonType ? '/' + r.pd.reasonType : ''}`;
  tally[key] = (tally[key] || 0) + 1;
  if (/^user/.test(r.pd.source)) prompts.push({ at: r.at, tool: u && u.name, what: what(u), grant: r.pd.grantScope || '' });
  if (r.pd.decision !== 'accept') refused.push({ at: r.at, tool: u && u.name, what: what(u), reason: r.text.split('\n')[0].slice(0, 160) });
}
const sim = [...uses.entries()].filter(([, u]) => u.name === 'Bash' && /^(cd \/home\/user\/Shamble-Frontlines && )?xvfb-run -a node tools\/preview\.js legacy eval tools\/sim\.js/.test(u.input.command || ''))
  .map(([id, u]) => ({ at: u.at, decision: results.get(id) ? results.get(id).pd : null }));
const out = { file, toolResultsWithDecision: results.size, tally, prompts, refused, kitLine4PlainRun: sim };
console.log(`tool results with a permission decision: ${results.size}`);
for (const [k, n] of Object.entries(tally).sort((a, b) => b[1] - a[1])) console.log(`  ${String(n).padStart(4)}  ${k}`);
console.log(`prompts that reached him (${prompts.length}):`);
for (const p of prompts) console.log(`  ${p.at}  ${p.tool}  ${p.what}`);
console.log(`refused (${refused.length}):`);
for (const p of refused) console.log(`  ${p.at}  ${p.tool}  ${p.what}  ::  ${p.reason}`);
console.log(`kit line 4's command as a plain shell line: ${JSON.stringify(sim)}`);
writeFileSync('/home/user/Shamble-Frontlines/.checkpoints/foundation-restructure/prompt-tally.json', JSON.stringify(out, null, 1) + '\n');

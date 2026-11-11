// Lists every refusal of the day (a guard hook error or a permission denial) in this session's local transcripts, each
// with the next tool call the same agent made (its route), as Markdown rows for the report's appendix.
// Run from the repo root: node .gate/foundation-restructure/refusal-routes.mjs
import { readFileSync, readdirSync, writeFileSync } from 'node:fs';

const S = '/root/.claude/projects/-home-user-Shamble-Frontlines/f3ec06a0-d229-5ef6-b5ae-e19625948e5d';
const files = [[S + '.jsonl', 'main session']];
for (const f of readdirSync(S + '/subagents')) if (f.endsWith('.jsonl')) files.push([S + '/subagents/' + f, f.replace(/\.jsonl$/, '')]);
const NAMES = { 'agent-aeb43a73578680b74': 'docs-keeper', 'agent-a51b7436fac806231': 'security-auditor', 'agent-a32ae6022d969a3e4': 'test-engineer',
  'agent-acf1a81a89edf47ab': 'infra-engineer', 'agent-ab812320827d8e648': 'advisor', 'agent-adb4cc7bb6eb4f32c': 'the tally agent',
  'agent-a517397036328b589': 'security-auditor (round 2)', 'agent-a2fde4b931884093c': 'test-engineer (round 2)',
  'agent-ab1d3a0e83dc2557b': 'infra-engineer (round 2)', 'agent-a1b9adbd146df18e1': 'verifier' };
const short = s => String(s).replace(/\s+/g, ' ').replace(/\/home\/user\/Shamble-Frontlines\//g, '').replace(/\|/g, '/').slice(0, 110);
const show = u => !u ? '(none)' : u.name === 'Bash' ? '`' + short(u.input.command) + '`' : `${u.name} ${short(u.input.file_path || u.input.pattern || '')}`;
const reason = t => {
  if (/has been denied/.test(t)) return 'the permission system ("has been denied"; the text names no list)';
  const m = t.match(/hook error: (.*)/); const r = m ? m[1] : t;
  if (/no option before the subcommand/.test(r)) return 'guard: git -C (run git from the folder)';
  if (/git checkout is not allowed/.test(r)) return 'guard: git checkout';
  if (/git restore is not allowed/.test(r)) return 'guard: git restore';
  if (/worktree remove --force/.test(r)) return 'guard: git worktree remove --force';
  if (/after a cd the guard cannot follow/.test(r)) return 'guard: a write after a cd it cannot follow (agents)';
  if (/spells its write targets/.test(r)) return 'guard: a write target it cannot read (agents)';
  return 'guard: ' + short(r);
};
const rows = [];
for (const [file, who0] of files) {
  const who = NAMES[who0] || who0;
  const calls = []; const byId = new Map(); const pending = [];
  for (const line of readFileSync(file, 'utf8').split('\n')) {
    if (!line.trim()) continue; let j; try { j = JSON.parse(line); } catch { continue; }
    const c = j.message && Array.isArray(j.message.content) ? j.message.content : [];
    for (const b of c) {
      if (b.type === 'tool_use') { const u = { id: b.id, name: b.name, input: b.input || {}, at: j.timestamp }; calls.push(u); byId.set(b.id, u); }
      if (b.type === 'tool_result' && b.is_error) {
        const t = typeof b.content === 'string' ? b.content : (b.content || []).map(x => x.text || '').join('');
        if (!/hook error|has been denied/.test(t)) continue;
        const u = byId.get(b.tool_use_id);
        pending.push({ u, at: (u && u.at || j.timestamp || '').slice(11, 16), why: reason(t) });
      }
    }
  }
  // the route: the next call this agent made, known only once the whole transcript is read
  for (const p of pending) {
    const i = calls.indexOf(p.u);
    rows.push({ at: p.at, who, why: p.why, refused: show(p.u), route: show(i >= 0 ? calls[i + 1] : null) });
  }
}
rows.sort((a, b) => a.at.localeCompare(b.at));
const md = ['| UTC | Who | Refused by | The refused call | The route taken (the next call) |', '|---|---|---|---|---|',
  ...rows.map(r => `| ${r.at} | ${r.who} | ${r.why} | ${r.refused} | ${r.route} |`)].join('\n');
writeFileSync('.gate/foundation-restructure/refusal-routes.md', md + '\n');
const count = {}; for (const r of rows) count[r.why] = (count[r.why] || 0) + 1;
console.log(rows.length, 'refusals'); console.log(JSON.stringify(count, null, 1));

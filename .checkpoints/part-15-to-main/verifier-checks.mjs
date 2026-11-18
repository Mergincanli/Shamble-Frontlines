// Verifier: for given transcript lines, prints tool inputs (JSON keys and short values) and, for GitHub results,
// only selected fields (head sha, check names, conclusions, run numbers). No other output text.
//   node verifier-checks.mjs <transcript.jsonl> <line> [<line> ...]
import { createReadStream } from 'node:fs';
import { createInterface } from 'node:readline';
const [file, ...ls] = process.argv.slice(2); const want = new Set(ls.map(Number));
const rl = createInterface({ input: createReadStream(file), crlfDelay: Infinity });
let n = 0;
const pick = (o) => {
  if (Array.isArray(o)) return o.map(pick);
  if (o && typeof o === 'object') {
    if (o.check_runs) return { total: o.total_count, runs: o.check_runs.map((c) => ({ name: c.name, status: c.status, conclusion: c.conclusion, head_sha: c.head_sha, completed_at: c.completed_at })) };
    if (o.workflow_runs) return { total: o.total_count, runs: o.workflow_runs.slice(0, 8).map((w) => ({ name: w.name, run_number: w.run_number, event: w.event, head_branch: w.head_branch, head_sha: w.head_sha, status: w.status, conclusion: w.conclusion, created_at: w.created_at, updated_at: w.updated_at })) };
    if (o.head && o.base) return { number: o.number, state: o.state, draft: o.draft, head_sha: o.head.sha, head_ref: o.head.ref, base_sha: o.base.sha, merged: o.merged, mergeable_state: o.mergeable_state, merged_at: o.merged_at };
    if (o.sha !== undefined && o.merged !== undefined) return o;
  }
  return typeof o === 'string' ? o.slice(0, 200) : o;
};
for await (const line of rl) {
  n++; if (!want.has(n)) continue; const r = JSON.parse(line);
  const c = r.message && r.message.content; const blocks = Array.isArray(c) ? c : [];
  for (const b of blocks) {
    if (b.type === 'tool_use') console.log(n, r.timestamp, 'TOOL', b.name, JSON.stringify(b.input).slice(0, 600));
    if (b.type === 'tool_result') { const t = typeof b.content === 'string' ? b.content : (Array.isArray(b.content) ? b.content.map((x) => x.text || '').join('') : '');
      let j; try { j = JSON.parse(t); } catch { console.log(n, 'RESULT (not JSON)', JSON.stringify(t.slice(0, 400))); continue; }
      console.log(n, r.timestamp, 'RESULT', JSON.stringify(pick(j)).slice(0, 2500)); }
  }
}

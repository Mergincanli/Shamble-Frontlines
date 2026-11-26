// Replays this chat's tool calls from its transcript through main's guard rules (routine-guard.mjs decide(), the
// function the PreToolUse hook calls) and lists every call the rules would have asked about or refused. The verdicts
// are today's: decide() reads the repository as it is now, not as it was at each call. Read-only: the guard's own git
// lookups and file reads; GitHub is never called (io.gh is stubbed), so the guard's GitHub-tool checks are listed as
// "not replayed". Command text is not printed, only the tool, the shape's first word and the guard's reason.
// Usage (from the repository root): node <this file> <transcript.jsonl> [since-ISO] > <output.md>
import { readFileSync } from 'node:fs';
import { pathToFileURL } from 'node:url';
import { join } from 'node:path';

const [transcript, since = '2026-11-15T20:01:46'] = process.argv.slice(2);
const root = process.cwd();
const { decide, realIo } = await import(pathToFileURL(join(root, '.claude/hooks/routine-guard.mjs')).href);
const io = { ...realIo(), gh: () => ({ code: 1, out: '' }), env: { ...process.env, SHAMBLE_ROLE: '' } };
const rows = []; const skipped = []; let total = 0;
for (const line of readFileSync(transcript, 'utf8').split('\n')) {
  let e; try { e = JSON.parse(line); } catch { continue; }
  const ts = e.timestamp ?? '';
  if (ts < since || e.isSidechain) continue;
  const content = e.message?.content;
  if (!Array.isArray(content)) continue;
  for (const b of content) {
    if (b.type !== 'tool_use') continue;
    const tool = b.name ?? '';
    if (!['Bash', 'Edit', 'Write', 'NotebookEdit', 'MultiEdit'].includes(tool) && !tool.startsWith('mcp__')) continue;
    total += 1;
    if (tool.startsWith('mcp__github__') && /merge|create|update|push|delete|add_|write|request|resolve|fork|enable|disable/.test(tool)) {
      skipped.push([ts.slice(0, 19).replace('T', ' '), tool]); continue;
    }
    const input = { hook_event_name: 'PreToolUse', tool_name: tool, tool_input: b.input ?? {}, cwd: e.cwd || root, session_id: 'replay' };
    let d; try { d = await decide(input, io); } catch (err) { d = { deny: `replay error: ${String(err?.message ?? err).slice(0, 120)}` }; }
    if (!d) continue;
    const kind = d.ask !== undefined && d.deny === undefined ? 'ask' : 'deny';
    const shape = tool === 'Bash' ? String(b.input?.command ?? '').trim().split(/\s+/)[0] : (b.input?.file_path ?? '').replace(root + '/', '');
    rows.push([ts.slice(0, 19).replace('T', ' '), tool, kind, shape.slice(0, 60), String(d.ask ?? d.deny).replace(/\|/g, '/').replace(/\n/g, ' ').slice(0, 220)]);
  }
}
console.log(`# Guard replay: this chat's calls since ${since} UTC through today's guard rules\n`);
console.log(`Calls replayed: ${total - skipped.length} of ${total}. Would ask: ${rows.filter((r) => r[2] === 'ask').length}. ` +
  `Would refuse: ${rows.filter((r) => r[2] === 'deny').length}. GitHub write tools not replayed (their check reads GitHub): ${skipped.length}.\n`);
console.log('| Time (UTC) | Tool | Verdict | Shape (first word or file) | The guard\'s reason (cut) |\n|---|---|---|---|---|');
for (const r of rows) console.log(`| ${r[0]} | ${r[1]} | ${r[2]} | \`${r[3]}\` | ${r[4]} |`);
console.log('\n## GitHub write tools not replayed\n');
for (const [t, n] of skipped) console.log(`- ${t} ${n}`);

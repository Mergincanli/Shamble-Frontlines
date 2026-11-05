// verifier pass 3 probe 2: day git clone targets with the real io on the worktree (decide only, nothing runs)
const [G, P] = process.argv.slice(2);
process.env.CLAUDE_PROJECT_DIR = P;
const { decide, realIo } = await import(G);
const kind = (d) => (d ? (d.ask !== undefined && d.deny === undefined ? 'ask' : 'deny') : null);
for (const c of ['git clone https://example.invalid/o/tools.git /tmp/t', 'git clone https://example.invalid/o/r.git /tmp/t',
  'git clone https://example.invalid/o/CLAUDE.md.git /tmp/t']) {
  const d = await decide({ hook_event_name: 'PreToolUse', tool_name: 'Bash', tool_input: { command: c }, cwd: P }, { ...realIo(), env: {} });
  console.log(`${c}\t${kind(d)}\t${(d?.ask ?? d?.deny ?? '').slice(0, 120)}`);
}

// pass 4 day probe: D-ASK's five forms in places the reader skips (here-document bodies), and P3-7 clone targets.
// Real git + bash 5.2 in a scratch repo, main folder on main, origin with cs/evil (settings changed).
// usage: node sap4-day.mjs <guard.mjs> <scratch dir>
import { spawnSync } from 'node:child_process';
import { mkdirSync, rmSync, writeFileSync, readFileSync, realpathSync, existsSync } from 'node:fs';
import { join } from 'node:path';
import { pathToFileURL } from 'node:url';
const [guardPath, scratch] = process.argv.slice(2);
const { decide, realIo, parseShell } = await import(pathToFileURL(guardPath).href);
const root = join(scratch, 'sap4-day');
rmSync(root, { recursive: true, force: true }); mkdirSync(root, { recursive: true });
const R = realpathSync(root), O = join(R, 'origin.git'), P = join(R, 'proj');
const env = { ...process.env, GIT_AUTHOR_NAME: 't', GIT_AUTHOR_EMAIL: 't@t', GIT_COMMITTER_NAME: 't', GIT_COMMITTER_EMAIL: 't@t', GIT_CONFIG_GLOBAL: '/dev/null', GIT_CONFIG_NOSYSTEM: '1' };
delete env.CDPATH;
const g = (...a) => spawnSync('git', a, { cwd: P, env, encoding: 'utf8' });
const sh = (line) => spawnSync('bash', ['--norc', '--noprofile', '-c', line], { cwd: P, env, encoding: 'utf8' });
spawnSync('git', ['init', '-q', '--bare', '-b', 'main', O], { env });
mkdirSync(join(P, '.claude'), { recursive: true });
writeFileSync(join(P, 'CLAUDE.md'), 'rules\n'); writeFileSync(join(P, '.claude', 'settings.json'), '{"a":1}\n'); writeFileSync(join(P, '.gitignore'), '.gate/\n');
g('init', '-q', '-b', 'main'); g('add', '-A'); g('commit', '-q', '-m', 'base');
g('remote', 'add', 'origin', O); g('push', '-q', 'origin', 'main');
g('switch', '-q', '-c', 'cs/evil'); writeFileSync(join(P, '.claude', 'settings.json'), '{"a":2}\n'); g('commit', '-q', '-am', 'evil'); g('push', '-q', 'origin', 'cs/evil'); g('switch', '-q', 'main'); g('fetch', '-q', 'origin');
process.env.CLAUDE_PROJECT_DIR = P;
const kind = (d) => (d ? (d.ask !== undefined && d.deny === undefined ? 'ask' : 'deny') : 'null');
const row = async (label, line, real = true, agent = false) => {
  writeFileSync(join(P, 'CLAUDE.md'), 'rules\n'); g('switch', '-q', '-f', 'main');
  const ti = { hook_event_name: 'PreToolUse', tool_name: 'Bash', tool_input: { command: line }, cwd: P, scratchpad_dir: join(R, 's') };
  if (agent) ti.agent_id = 'a1';
  const d = await decide(ti, { ...realIo(), env: {} });
  let eff = '';
  if (real && !d) { const r = sh(line); eff = `bash exit ${r.status}; branch ${g('branch', '--show-current').stdout.trim()}; settings ${readFileSync(join(P, '.claude/settings.json'), 'utf8').trim()}; CLAUDE.md ${JSON.stringify(readFileSync(join(P, 'CLAUDE.md'), 'utf8'))}`; }
  console.log(label, JSON.stringify(line), '->', kind(d), d ? String(d.deny ?? d.ask).slice(0, 100) : eff, '| unreadable:', parseShell(line).unreadable);
};
await row('H1 continuation in a here-doc body joins the delimiter line', 'cat <<EOF >/dev/null\nE\\\nOF\ngit switch -q cs/evil\nEOF');
await row('H1a (agent)', 'cat <<EOF >/dev/null\nE\\\nOF\ngit switch -q cs/evil\nEOF', false, true);
await row('H2 command substitution in an unquoted here-doc body', 'cat <<EOF >/dev/null\n$(git switch -q cs/evil)\nEOF');
await row('H3 a backtick in an unquoted here-doc body', 'cat <<EOF >/dev/null\n`git switch -q cs/evil`\nEOF');
await row('ctl H2 quoted delimiter (no expansion in the body)', "cat <<'EOF' >/dev/null\n$(git switch -q cs/evil)\nEOF");
await row('A1 an ordinary comment line first', '# check the tree\ngit status', false);
await row('A2 ordinary continuation', 'git log \\\n  --oneline -1', false);
await row('A3 agent ordinary comment', 'git status # check', false, true);
await row('C1 clone --separate-git (abbreviated) into .claude', `git clone -q --separate-git .claude/x ${O} .gate/c`);
console.log('  .claude/x exists after:', existsSync(join(P, '.claude', 'x')));
await row('C1c clone --separate-git-dir (full) into .claude (control)', `git clone -q --separate-git-dir .claude/y ${O} .gate/d`, false);

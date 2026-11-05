// security-auditor delta probe (kit-v3-3 A3, head ed3ea8c). Real git repos under the given scratch dir. Prints row
// labels, the guard's decision kind and, for some rows, what real git/bash then did. No env values or payloads printed.
// usage: node sad-probe.mjs <guard.mjs path> <scratch dir>
import { spawnSync } from 'node:child_process';
import { mkdirSync, rmSync, writeFileSync, existsSync, readFileSync, realpathSync } from 'node:fs';
import { join } from 'node:path';
import { pathToFileURL } from 'node:url';
const [guardPath, scratch] = process.argv.slice(2);
const { decide, realIo } = await import(pathToFileURL(guardPath).href);
const root = join(scratch, 'sad-probe');
rmSync(root, { recursive: true, force: true });
mkdirSync(root, { recursive: true });
const R = realpathSync(root);
const O = join(R, 'origin.git'), P = join(R, 'proj'), Q = join(R, 'other');
const env = { ...process.env, GIT_AUTHOR_NAME: 't', GIT_AUTHOR_EMAIL: 't@t', GIT_COMMITTER_NAME: 't', GIT_COMMITTER_EMAIL: 't@t', GIT_CONFIG_GLOBAL: '/dev/null', GIT_CONFIG_NOSYSTEM: '1' };
delete env.CDPATH;
const gx = (cwd, ...a) => spawnSync('git', a, { cwd, env, encoding: 'utf8' });
const g = (...a) => gx(P, ...a);
const sh = (cwd, line) => spawnSync('bash', ['--norc', '--noprofile', '-c', line], { cwd, env, encoding: 'utf8' });
const rev = (cwd, r) => gx(cwd, 'rev-parse', '--verify', '-q', `${r}^{commit}`).stdout.trim();
spawnSync('git', ['init', '-q', '--bare', '-b', 'main', O], { env });
mkdirSync(join(P, '.claude', 'hooks'), { recursive: true });
mkdirSync(join(P, 'docs'), { recursive: true });
writeFileSync(join(P, 'CLAUDE.md'), 'rules\n');
writeFileSync(join(P, '.claude', 'settings.json'), '{"a":1}\n');
writeFileSync(join(P, '.claude', 'hooks', 'routine-guard.mjs'), 'export const x = 1;\n'.repeat(30));
writeFileSync(join(P, 'docs', 'a.md'), 'a\n');
g('init', '-q', '-b', 'main'); g('add', '-A'); g('commit', '-q', '-m', 'base');
g('remote', 'add', 'origin', O); g('push', '-q', 'origin', 'main'); g('fetch', '-q', 'origin');
const mk = (name, edit) => { g('switch', '-q', '-c', name, 'origin/main'); edit(); g('add', '-A'); g('commit', '-q', '-m', name); g('push', '-q', 'origin', name); g('switch', '-q', 'main'); };
mk('cs/x', () => writeFileSync(join(P, 'docs', 'a.md'), 'x1\n'));
mk('cs/n', () => writeFileSync(join(P, 'docs', 'a.md'), 'n1\n'));
mk('cs/evil', () => writeFileSync(join(P, '.claude', 'settings.json'), '{"a":2}\n'));
mk('cs/ren', () => { g('mv', '.claude/hooks/routine-guard.mjs', 'docs/old-guard.mjs'); });
g('fetch', '-q', 'origin');
g('branch', '-q', '-D', 'cs/x', 'cs/n', 'cs/evil', 'cs/ren');
// another clone moves origin cs/x to a kit-changing commit after P's last fetch
spawnSync('git', ['clone', '-q', O, Q], { env });
gx(Q, 'switch', '-q', 'cs/x'); writeFileSync(join(Q, '.claude', 'settings.json'), '{"a":3}\n'); gx(Q, 'commit', '-q', '-am', 'x2'); gx(Q, 'push', '-q', 'origin', 'cs/x');
process.env.CLAUDE_PROJECT_DIR = P;
const kind = (d) => (d ? (d.ask !== undefined && d.deny === undefined ? 'ask' : 'deny') : 'null');
const run = async (role, command, cwd = P, extra = {}) => kind(await decide({ hook_event_name: 'PreToolUse', tool_name: 'Bash', tool_input: { command }, cwd, scratchpad_dir: join(R, 'scratch'), ...extra }, { ...realIo(), env: role ? { SHAMBLE_ROLE: role } : {} }));
const out = (...a) => console.log(...a);
const settingsNow = (dir) => readFileSync(join(dir, '.claude', 'settings.json'), 'utf8').trim();
const reset = () => { g('switch', '-q', 'main'); for (const b of ['cs/x', 'cs/n']) g('branch', '-q', '-D', b); g('update-ref', 'refs/remotes/origin/cs/x', rev(Q, 'HEAD~1')); };

// C3: night handoff line, fetch then switch (origin/cs/x stale clean at hook time)
out('C3a night: git switch cs/x alone (stale clean ref):', await run('night-shift', 'git switch cs/x'));
const l1 = 'git fetch -q origin cs/x && git switch -q cs/x';
out('C3b night:', l1, '->', await run('night-shift', l1));
sh(P, l1); out('  real: on', g('branch', '--show-current').stdout.trim(), 'settings', settingsNow(P));
reset();
out('  after reset origin/cs/x is stale clean again:', settingsNow(P), await run('night-shift', 'git switch cs/x'));
// C3c: create a branch from an evil start, then switch to it by the name whose origin ref is clean
const l2 = 'git branch cs/n origin/cs/evil && git switch -q cs/n';
out('C3c night:', l2, '->', await run('night-shift', l2));
sh(P, l2); out('  real: on', g('branch', '--show-current').stdout.trim(), 'settings', settingsNow(P));
g('switch', '-q', 'main'); g('branch', '-q', '-D', 'cs/n');
// day: the same line from the main folder (main-folder check reads refs at hook time)
const l3 = 'git fetch -q origin cs/x && git switch --detach origin/cs/x';
out('C3d day:', l3, '->', await run('', l3));
g('update-ref', 'refs/remotes/origin/cs/x', rev(Q, 'HEAD~1'));

// C6 / F9-1: day main-folder switch to a branch that renames the guard file away
g('branch', '-q', 'cs/ren', 'origin/cs/ren'); g('branch', '-q', 'cs/evil', 'origin/cs/evil');
out('C6a day: git switch cs/evil (control, settings change):', await run('', 'git switch cs/evil'));
out('C6b day: git switch cs/ren (renames .claude/hooks/routine-guard.mjs away):', await run('', 'git switch cs/ren'));
out('C6c night: git switch cs/ren:', await run('night-shift', 'git switch cs/ren'));
g('switch', '-q', 'cs/ren'); out('  night push of a branch renaming the guard away:', await run('night-shift', 'git push origin cs/ren')); g('switch', '-q', 'main');

// C1/C2: day cd tracking with a worktree
g('worktree', 'add', '-q', '.gate/wt', '-b', 'cs/wt', 'origin/main');
const W = join(P, '.gate', 'wt');
writeFileSync(join(P, '.git', 'info', 'exclude'), '.gate/\n');
const l4 = `cd ${R}/nope && cd ${W}; git switch -q -c cs/c1 origin/cs/evil`;
out('C1a day:', 'cd <missing> && cd <abs wt>; git switch -c cs/c1 origin/cs/evil ->', await run('', l4));
sh(P, l4 + ' 2>/dev/null'); out('  real: main folder on', g('branch', '--show-current').stdout.trim()); g('switch', '-q', 'main');
const l5 = `cd ${R}/nope && cd ${W}; echo x > CLAUDE.md`;
out('C1b day: cd <missing> && cd <abs wt>; echo x > CLAUDE.md ->', await run('', l5));
const l5a = `cd ${R}/nope && cd ${W}; echo x > CLAUDE.md`;
out('C1b agent:', await run('', l5a, P, { agent_id: 'a1' }));
const l6 = `! cd ${P} && cd ./.gate/wt; git switch -q -c cs/c2 origin/cs/evil`;
out('C2 day: ! cd <main> && cd ./.gate/wt; git switch -c cs/c2 origin/cs/evil ->', await run('', l6));
sh(P, l6); out('  real: main folder on', g('branch', '--show-current').stdout.trim()); g('switch', '-q', 'main');
out('C1 control day: cd <abs wt> && git switch -c cs/c3 origin/cs/evil ->', await run('', `cd ${W} && git switch -q -c cs/c3 origin/cs/evil`));
out('C1 control day: cd <missing>; git switch -c cs/c4 origin/cs/evil ->', await run('', `cd ${R}/nope; git switch -c cs/c4 origin/cs/evil`));
out('C1 control day: cd <missing>; cd <abs wt>; git switch -c cs/c5 origin/cs/evil ->', await run('', `cd ${R}/nope; cd ${W}; git switch -c cs/c5 origin/cs/evil`));

const l7 = `cd ${R}/nope && cd ${R}; echo x > CLAUDE.md`;
out('C1c day: cd <missing> && cd <non-repo abs>; echo x > CLAUDE.md ->', await run('', l7));
sh(P, l7 + ' 2>/dev/null'); out('  real: main CLAUDE.md now', readFileSync(join(P, 'CLAUDE.md'), 'utf8').trim()); g('checkout', '--', 'CLAUDE.md');
out('C1c control: cd <non-repo abs>; echo x > CLAUDE.md ->', await run('', `cd ${R}; echo x > CLAUDE.md`));
// C4: perl bundles
for (const c of ["perl -pi -e 's/a/b/' CLAUDE.md", "perl -wpi -e 's/a/b/' CLAUDE.md", "perl -lpi -e 's/a/b/' CLAUDE.md", "perl -0pi -e 's/a/b/' CLAUDE.md", "perl -p -i -e 's/a/b/' CLAUDE.md"]) out('C4 day:', c, '->', await run('', c));
writeFileSync(join(R, 'f.txt'), 'a\n'); sh(R, "perl -wpi -e 's/a/b/' f.txt"); out('  real: perl -wpi edits in place:', readFileSync(join(R, 'f.txt'), 'utf8').trim() === 'b');

// auto-11: git clone value options
out('A11 day: git clone <O> .claude/sub (control) ->', await run('', `git clone -q ${O} .claude/sub`));
out('A11 day: git clone --revision main <O> .claude/sub ->', await run('', `git clone --revision main ${O} .claude/sub`));
out('A11 day: git clone --bran main <O> .claude/sub ->', await run('', `git clone --bran main ${O} .claude/sub`));
sh(P, `git clone -q --bran main ${O} ${R}/abbr`); out('  real: --bran abbreviation accepted by git clone:', existsSync(join(R, 'abbr', '.git')));
// auto-9: folder holding a worktree
out('A9 day: rm -rf .gate (holds a worktree with .claude/) ->', await run('', 'rm -rf .gate'));
out('A9 day: rm -rf .gate/wt ->', await run('', 'rm -rf .gate/wt'));
out('A9 control: rm -rf .claude ->', await run('', 'rm -rf .claude'));
console.log('git', gx(P, '--version').stdout.trim());
rmSync(root, { recursive: true, force: true });

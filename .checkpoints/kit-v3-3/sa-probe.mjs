// security-auditor probe for A3 (kit-v3-3). Usage: node sa-probe.mjs <worktree>
// Builds throwaway repos under <worktree>/.gate/kit-v3-3/sa-probe/ and asks the guard (head, base 12d7961, step-4
// f22c4df copies) for decisions. Prints decisions only (no environment values, no payloads).
import { spawnSync } from 'node:child_process';
import { mkdirSync, writeFileSync, rmSync, existsSync, readFileSync } from 'node:fs';
import { join } from 'node:path';

const WT = process.argv[2];
const D = join(WT, '.gate/kit-v3-3/sa-probe');
const ENV = { ...process.env, GIT_AUTHOR_NAME: 'p', GIT_AUTHOR_EMAIL: 'p@x', GIT_COMMITTER_NAME: 'p', GIT_COMMITTER_EMAIL: 'p@x', GIT_CONFIG_GLOBAL: '/dev/null' };
const sh = (cmd, cwd) => { const r = spawnSync('bash', ['-c', cmd], { cwd, env: ENV, encoding: 'utf8' }); return { code: r.status, out: (r.stdout || '').trim(), err: (r.stderr || '').trim().slice(0, 160) }; };

const O = join(D, 'origin.git'), P = join(D, 'proj'), PARENT = D;
rmSync(O, { recursive: true, force: true }); rmSync(P, { recursive: true, force: true });
sh(`git init -q --bare -b main ${O}`, D);
mkdirSync(join(P, '.claude'), { recursive: true }); mkdirSync(join(P, 'tools'), { recursive: true }); mkdirSync(join(P, 'docs/sub'), { recursive: true });
writeFileSync(join(P, 'CLAUDE.md'), 'rules\n'); writeFileSync(join(P, '.claude/settings.json'), '{"a":1}\n');
writeFileSync(join(P, 'tools/sim.js'), 'console.log(1)\n'); writeFileSync(join(P, 'docs/sub/CLAUDE.md'), 'sub\n');
sh(`git init -q -b main && git add -A && git commit -qm base && git remote add origin ${O} && git push -q origin main && git fetch -q origin`, P);
sh(`git switch -q -c cs/evil && echo '{"a":2}' > .claude/settings.json && echo 'console.log(2)' > tools/sim.js && git commit -qam evil && git push -q origin cs/evil && git switch -q main && git fetch -q origin`, P);

const load = async (f) => (await import(f + '?' + Math.random()));
const head = await load(join(WT, '.claude/hooks/routine-guard.mjs'));
const base = await load(join(D, 'base-guard.mjs'));
const s4 = await load(join(D, 's4-guard.mjs'));
process.env.CLAUDE_PROJECT_DIR = P;
const io = (m, role) => { const r = m.realIo(); return { ...r, projectDir: P, env: role ? { SHAMBLE_ROLE: role } : {} }; };
const fmt = (d) => d === null ? 'null' : d.ask ? 'ASK' : d.deny ? 'DENY' : JSON.stringify(d);
const ask = async (m, role, command, cwd = P, extra = {}) => fmt(await m.decide({ hook_event_name: 'PreToolUse', tool_name: 'Bash', tool_input: { command }, cwd, ...extra }, io(m, role)));
const mcp = async (m, role, name, ti) => fmt(await m.decide({ hook_event_name: 'PreToolUse', tool_name: `mcp__github__${name}`, tool_input: ti, cwd: P }, io(m, role)));
const row = async (id, role, cmd, guards = { head }) => {
  const parts = [];
  for (const [k, m] of Object.entries(guards)) parts.push(`${k}=${await ask(m, role, cmd)}`);
  console.log(`${id} [${role || 'day'}] ${cmd.replaceAll(P, '<P>').replaceAll(PARENT, '<D>')} -> ${parts.join(' ')}`);
};
const all3 = { head, s4, base };

console.log('--- day: controls');
await row('C1', '', 'echo x > CLAUDE.md', all3);
await row('C2', '', 'cd .claude && sed -i s/a/b/ settings.json', all3);
await row('C3', '', 'git switch cs/evil', all3);
console.log('--- day: F-f lost cd then relative write');
await row('Ff1', '', '(cd .claude && sed -i s/a/b/ settings.json)', all3);
await row('Ff2', '', 'cd .claude && sed -i s/a/b/ settings.json && git status | cat', all3);
await row('Ff3', '', 'cd .claude && echo x > settings.json; ls | head', all3);
console.log('--- day: F-a sed long-option abbreviation');
await row('Fa1', '', 'sed --in s/a/b/ CLAUDE.md', all3);
await row('Fa2', '', 'sed --in-pl s/a/b/ CLAUDE.md', all3);
writeFileSync(join(D, 'sedt.txt'), 'a\n'); sh('sed --in s/a/b/ sedt.txt', D); console.log('  real GNU sed --in edits in place:', readFileSync(join(D, 'sedt.txt'), 'utf8').trim() === 'b');
console.log('--- day: F-b shell forms not parsed');
await row('Fb1', '', `bash -c 'echo x > CLAUDE.md'`, all3);
await row('Fb2', '', `eval "echo x > CLAUDE.md"`, all3);
await row('Fb3', '', 'sudo tee CLAUDE.md', all3);
await row('Fb4', '', 'echo x | sudo cp /dev/stdin .claude/settings.json', all3);
console.log('--- day: F-c folders');
await row('Fc1', '', 'rm -rf docs', all3);
await row('Fc2', '', `rm -rf ${PARENT}`, all3);
await row('Fc3', '', `mv ${P} ${PARENT}/moved`, all3);
console.log('--- day: F-g git alias');
sh('git config alias.sw switch', P);
await row('Fg1', '', 'git sw cs/evil', all3);
console.log('--- day: F-i forge origin/main by fetch refspec, then the free update form');
await row('Fi1', '', 'git fetch origin cs/evil:refs/remotes/origin/main', all3);
await row('Fi2', '', 'git merge --ff-only origin/main', all3);
await row('Fi3', '', 'git fetch origin cs/evil:refs/remotes/origin/main && git merge --ff-only origin/main', all3);
console.log('--- routines: F-i / F-e');
await row('Ri1', 'night-shift', 'git fetch origin cs/evil:refs/remotes/origin/main', all3);
await row('Re1', 'night-shift', 'git branch origin/main cs/evil', all3);
await row('Re2', 'pr-advisor', 'git branch origin/main origin/cs/evil', all3);
await row('Re3', 'pr-advisor', 'git fetch origin cs/evil:refs/remotes/origin/main', all3);
// night shift on its branch with a protected change: push before and after forging
sh('git switch -q cs/evil', P);
await row('Rp1 (real origin/main)', 'night-shift', 'git push origin cs/evil', all3);
await row('Rn1 (real origin/main)', 'night-shift', 'node tools/sim.js', all3);
sh('git fetch -q origin cs/evil:refs/remotes/origin/main', P);
console.log('  forged: refs/remotes/origin/main == cs/evil:', sh('git rev-parse refs/remotes/origin/main', P).out === sh('git rev-parse cs/evil', P).out);
await row('Rp2 (forged by fetch)', 'night-shift', 'git push origin cs/evil', all3);
await row('Rn2 (forged by fetch)', 'night-shift', 'node tools/sim.js', all3);
// reset the remote-tracking ref (in the probe repo only), then shadow by a local branch
sh('git fetch -q origin +main:refs/remotes/origin/main', P);
await row('Rp3 (real again)', 'night-shift', 'git push origin cs/evil', { head });
sh('git branch -q origin/main cs/evil', P);
await row('Rp4 (shadowed by local branch origin/main)', 'night-shift', 'git push origin cs/evil', all3);
await row('Rn4 (shadowed)', 'night-shift', 'node tools/sim.js', all3);
await row('Rs4 (shadowed)', 'pr-advisor', 'git switch --detach origin/main', all3);
sh('git branch -q -D origin/main && git switch -q main', P);
console.log('--- routines: rev-list');
await row('Rr1', 'pr-advisor', 'git rev-list --count origin/main..HEAD', all3);
await row('Rr2', 'pr-advisor', `git rev-list --outp=${D}/x HEAD`, all3);
await row('Rr3', 'night-shift', `git rev-list --output=${D}/x HEAD`, all3);
const r = sh(`git rev-list --output=${D}/revout HEAD`, P); console.log('  real git rev-list --output writes a file:', existsSync(join(D, 'revout')), 'exit', r.code);
console.log('--- routines: Actions');
for (const [role, name, ti] of [
  ['pr-advisor', 'actions_get', { owner: 'Mergincanli', repo: 'Shamble-Frontlines', method: 'get_workflow_run', resource_id: '1' }],
  ['pr-advisor', 'actions_get', { owner: 'mergincanli', repo: 'shamble-frontlines', method: 'get_workflow_job', resource_id: '1' }],
  ['pr-advisor', 'actions_get', { owner: 'other', repo: 'Shamble-Frontlines', method: 'get_workflow_run', resource_id: '1' }],
  ['pr-advisor', 'actions_get', { owner: 'Mergincanli', repo: 'Shamble-Frontlines', method: ['get_workflow_run'], resource_id: '1' }],
  ['pr-advisor', 'actions_get', { owner: 'Mergincanli', repo: 'Shamble-Frontlines', resource_id: '1' }],
  ['pr-advisor', 'actions_get', { owner: 'Mergincanli', repo: 'Shamble-Frontlines', method: 'download_workflow_run_artifact', resource_id: '1' }],
  ['pr-advisor', 'actions_list', { owner: 'Mergincanli', repo: 'Shamble-Frontlines', method: 'list_workflow_runs' }],
  ['pr-advisor', 'actions_run_trigger', { owner: 'Mergincanli', repo: 'Shamble-Frontlines', method: 'rerun_workflow_run', run_id: 1 }],
  ['night-shift', 'actions_get', { owner: 'Mergincanli', repo: 'Shamble-Frontlines', method: 'get_workflow_run', resource_id: '1' }],
  ['night-shift', 'actions_list', { owner: 'Mergincanli', repo: 'Shamble-Frontlines', method: 'list_workflows' }],
]) console.log(`A [${role}] ${name} ${JSON.stringify({ ...ti, owner: ti.owner, repo: ti.repo })} -> head=${await mcp(head, role, name, ti)} base=${await mcp(base, role, name, ti)}`);

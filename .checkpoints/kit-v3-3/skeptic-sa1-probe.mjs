// skeptic SA-1 probe: decisions only. Usage: node skeptic-sa1-probe.mjs <worktree>
import { spawnSync } from 'node:child_process';
import { mkdirSync, writeFileSync, rmSync, readFileSync } from 'node:fs';
import { join } from 'node:path';
const WT = process.argv[2], D = join(WT, '.gate/kit-v3-3/skeptic-sa1'), P = join(D, 'proj');
rmSync(P, { recursive: true, force: true }); mkdirSync(join(P, '.claude/hooks'), { recursive: true });
writeFileSync(join(P, '.claude/settings.json'), 'a\n'); writeFileSync(join(P, '.claude/hooks/g.mjs'), 'a\n');
const env = { ...process.env, GIT_CONFIG_GLOBAL: '/dev/null', GIT_AUTHOR_NAME: 'p', GIT_AUTHOR_EMAIL: 'p@x', GIT_COMMITTER_NAME: 'p', GIT_COMMITTER_EMAIL: 'p@x' };
spawnSync('bash', ['-c', 'git init -q -b main && git add -A && git commit -qm b'], { cwd: P, env });
const head = await import(join(WT, '.claude/hooks/routine-guard.mjs'));
const s4 = await import(join(D, 's4-guard.mjs'));
process.env.CLAUDE_PROJECT_DIR = P;
const fmt = (d) => d === null ? 'null' : d.ask ? 'ASK' : d.deny ? 'DENY' : JSON.stringify(d);
const dec = async (m, command) => { const r = m.realIo(); return fmt(await m.decide({ hook_event_name: 'PreToolUse', tool_name: 'Bash', tool_input: { command }, cwd: P }, { ...r, projectDir: P, env: {} })); };
const cases = [
  'cd .claude && sed -i s/a/b/ settings.json',
  'cd .claude && sed -i s/a/b/ settings.json && git status | cat',
  '(cd .claude && sed -i s/a/b/ settings.json)',
  'cd .claude && echo x > settings.json; ls | head',
  'cd .claude/hooks && sed -i s/a/b/ g.mjs && node --version | tail -1',
  'cd .claude/hooks && sed -i s/a/b/ g.mjs &',
  'cd .claude && sed -i s/a/b/ settings.json && echo $(date)',
  'git status | cat; cd .claude && rm settings.json',
];
for (const c of cases) console.log(`head=${await dec(head, c)} s4=${await dec(s4, c)} :: ${c}`);
// real shell: does the forked-line write land in the kit file?
writeFileSync(join(P, '.claude/hooks/g.mjs'), 'a\n');
spawnSync('bash', ['-c', 'cd .claude/hooks && sed -i s/a/b/ g.mjs && node --version | tail -1'], { cwd: P, env });
console.log('real bash: .claude/hooks/g.mjs edited by the piped line:', readFileSync(join(P, '.claude/hooks/g.mjs'), 'utf8').trim() === 'b');

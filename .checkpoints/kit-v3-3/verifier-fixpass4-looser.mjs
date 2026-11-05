// verifier fix pass 4: extra no-looser check, old (60ae124) vs head, random token-soup lines (not filtered), both roles,
// lite io; plus Write/Edit on kit and non-kit paths. Prints counts and every looser line.
import { pathToFileURL } from 'node:url';
const [headG, oldG, nS = '20000', seedS = '5'] = process.argv.slice(2);
const H = await import(pathToFileURL(headG).href), O = await import(pathToFileURL(oldG).href);
let seed = Number(seedS);
const r = () => { seed = (seed + 0x6D2B79F5) | 0; let x = Math.imul(seed ^ (seed >>> 15), 1 | seed); x = (x + Math.imul(x ^ (x >>> 7), 61 | x)) ^ x; return ((x ^ (x >>> 14)) >>> 0) / 4294967296; };
const pick = (a) => a[Math.floor(r() * a.length)];
const P = ['git', 'gh', 'npm', 'node', 'python3', 'date', 'cd', 'env', 'timeout 5', 'xvfb-run -a', 'if', 'then', 'do', 'docs/do', '"git"', 'TZ=x', 'PATH=x', 'cat', 'ls', 'pushd', 'nohup', 'exec'];
const A = ['status', 'log', '-1', 'commit', '-m', '"x y"', 'push', 'origin', 'cs/x', 'switch', 'add', 'docs/a.md', 'CLAUDE.md', '.claude/settings.json',
  'tools/gate.mjs', '.checkpoints/n.md', 'ci', '--prefix', 'tools', '-q', 'fi', 'done', "'a*b'", '.gate/w/.claude/settings.json'];
const OP = [' ', ' ', ' ', ' > ', ' >> ', '>', ' 2>&1 ', ' >&2 ', ' < ', ' ; ', ';', ' && ', ' | ', ' & ', '\n', ' &> '];
const lite = (role) => ({ env: { SHAMBLE_ROLE: role }, projectDir: '/repo', home: '/home/u', gitLongOptions: () => [],
  git: (args) => (args.join(' ') === 'rev-parse --show-toplevel' ? { code: 0, out: '/repo\n' } : { code: 0, out: '' }),
  gh: () => ({ code: 1, out: '' }), exists: () => false, realpath: (p) => p, readFile: () => '{}' });
const ev = (tool, ti) => ({ hook_event_name: 'PreToolUse', tool_name: tool, tool_input: ti, cwd: '/repo', scratchpad_dir: '/tmp/s' });
const rank = (d) => (!d ? 0 : d.deny !== undefined ? 2 : 1);
let n = 0, looser = 0, stricter = 0, same = 0; const L = [];
const cmp = async (role, tool, ti) => {
  let a, b; try { a = rank(await O.decide(ev(tool, ti), lite(role))); } catch { a = 2; } try { b = rank(await H.decide(ev(tool, ti), lite(role))); } catch { b = -1; }
  n++; if (b < a) { looser++; L.push(`${role} ${tool} ${JSON.stringify(ti)} ${a}->${b}`); } else if (b > a) stricter++; else same++;
};
for (let k = 0; k < Number(nS); k++) {
  let s = (r() < 0.5 ? pick(P) : pick(['git', 'gh', 'date', 'node'])) ;
  for (let j = Math.floor(r() * 6); j > 0; j--) s += (r() < 0.7 ? ' ' + pick(A) : pick(OP) + pick(r() < 0.6 ? A : P));
  for (const role of ['night-shift', 'pr-advisor']) await cmp(role, 'Bash', { command: s });
}
for (const f of ['CLAUDE.md', '.claude/settings.json', '.github/workflows/ci.yml', 'docs/a.md', '.checkpoints/x.md', '.gate/w/.claude/settings.json', '/tmp/s/w/.claude/settings.json', '/tmp/s/n.md', 'tools/gate.mjs'])
  for (const role of ['night-shift', 'pr-advisor']) for (const tool of ['Write', 'Edit']) await cmp(role, tool, { file_path: f, content: 'x', old_string: 'a', new_string: 'b' });
console.log(`decisions ${n}: looser ${looser}, stricter ${stricter}, same ${same}`); for (const l of L.slice(0, 20)) console.log('LOOSER', l);

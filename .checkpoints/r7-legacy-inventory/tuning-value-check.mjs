// R7 advisor condition 2: dumped tuning values against their source literal at file:line (main 48ffb51; the engine
// files are unchanged on main since). Run from the repo root:
//   node .checkpoints/r7-legacy-inventory/tuning-value-check.mjs <dump.json>
import { readFileSync } from 'node:fs';
import { execFileSync } from 'node:child_process';
const dump = JSON.parse(readFileSync(process.argv[2], 'utf8')).defaults;
const src = (f) => execFileSync('git', ['show', `48ffb51:${f}`], { encoding: 'utf8' }).split('\n');
const files = { core: src('shamble-core.js'), game: src('shamble-game.js') };
const get = (path) => path.split('.').reduce((o, k) => (o == null ? undefined : o[k]), dump);
const CASES = [
  ['T.moveSpeed', 'core', 67, /moveSpeed\s*:\s*([-0-9.]+)/],
  ['T.playerHealth', 'core', 77, /playerHealth\s*:\s*([-0-9.]+)/],
  ['T.gravity', 'core', 82, /gravity\s*:\s*([-0-9.]+)/],
  ['TUNE.teams.size', 'core', 102, /size\s*:\s*([-0-9.]+)/],
  ['SUN_R', 'core', 167, /SUN_R\s*=\s*([-0-9.]+)/],
  ['GROUND_W', 'core', 224, /GROUND_W\s*=\s*([-0-9.]+)/],
  ['WORLD_OUTLINE', 'core', 241, /WORLD_OUTLINE\s*=\s*([-0-9.]+)/],
  ['OUTLINE_GROUND_Y', 'core', 294, /OUTLINE_GROUND_Y\s*=\s*([-0-9.]+)/],
  ['GRID_SIZE', 'core', 850, /GRID_SIZE\s*=\s*([-0-9.]+)/],
  ['RULES_DEFAULT.silo.capture', 'game', 40, /silo\s*:\s*\{\s*capture\s*:\s*([-0-9.]+)/],
  ['RULES_DEFAULT.tbox.active', 'game', 40, /tbox\s*:\s*\{\s*active\s*:\s*([-0-9.]+)/],
  ['ARM_L1', 'game', 209, /ARM_L1\s*=\s*([-0-9.]+)/],
  ['FIRE_H', 'game', 1419, /FIRE_H\s*=\s*([-0-9.]+)/],
  ['AIM_ARC_N', 'game', 1701, /AIM_ARC_N\s*=\s*([-0-9.]+)/],
  ['VIG_W', 'game', 2347, /VIG_W\s*=\s*([-0-9.]+)/],
  ['OCC_FADE', 'game', 2555, /OCC_FADE\s*=\s*([-0-9.]+)/],
  ['WR_DEPLOY_MS', 'game', 2999, /WR_DEPLOY_MS\s*=\s*([-0-9.]+)/],
];
let pass = 0, fail = 0;
for (const [path, file, line, re] of CASES) {
  const m = re.exec(files[file][line - 1] || '');
  const literal = m ? Number(m[1]) : NaN;
  const value = get(path);
  const ok = m && value === literal;
  ok ? pass++ : fail++;
  console.log(`${ok ? 'ok  ' : 'FAIL'} ${path} = ${JSON.stringify(value)} · literal ${m ? m[1] : 'not found'} at shamble-${file}.js:${line}`);
}
console.log(`${pass} pass, ${fail} fail`);
process.exit(fail ? 1 : 0);

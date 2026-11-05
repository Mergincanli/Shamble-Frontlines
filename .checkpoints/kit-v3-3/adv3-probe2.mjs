// advisor pass 3: nested double quotes inside "${...}" (usage: node adv3-probe2.mjs <routine-guard.mjs>)
import { execFileSync } from 'node:child_process';
const { parseShell } = await import(process.argv[2]);
const cases = {
  A5b_param_nested_dq: 'echo "${x:-"}"}"; echo HIDDEN; echo "x"',
  A5c_same_line_date: 'date +"${x:-"}"}"; echo HIDDEN; date +"x"',
};
for (const [k, src] of Object.entries(cases)) {
  let out;
  try { out = execFileSync('bash', ['--norc', '--noprofile', '-c', src], { encoding: 'utf8', stdio: ['ignore', 'pipe', 'pipe'] }).trim().replace(/\n/g, ' | '); }
  catch (e) { out = 'ERR ' + (e.stdout || '').trim().replace(/\n/g, ' | ') + ' / ' + (e.stderr || '').trim(); }
  const p = parseShell(src);
  console.log(`${k}\n  bash: ${out}\n  parser cmds: ${JSON.stringify(p.cmds.map((c) => c.argv))}`);
}

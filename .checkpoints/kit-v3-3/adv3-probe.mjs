// advisor pass 3: does parseShell agree with real bash on command boundaries for a few constructs the reviews did not name?
// usage: node adv3-probe.mjs <path to routine-guard.mjs>
import { execFileSync } from 'node:child_process';
const { parseShell } = await import(process.argv[2]);
const cases = {
  A5_param_nested_dq: 'echo "${x:-"}"; echo HIDDEN; echo "}"',
  A6_param_sq_in_unquoted: "echo ${x:-'}'}; echo HIDDEN",
  A7_arith_paren: 'echo $(( (1+2) )); echo HIDDEN',
  A8_case_paren: 'case a in a) echo A;; esac; echo HIDDEN',
  A9_dollar_dq: 'echo $"a"; echo HIDDEN',
  A10_brace_expand: 'echo CLAUDE.{md,x}',
  A11_hash_mid: 'echo a#b; echo HIDDEN',
  A12_bs_nl: 'echo a \\\necho HIDDEN',
};
for (const [k, src] of Object.entries(cases)) {
  let out;
  try { out = execFileSync('bash', ['--norc', '--noprofile', '-c', src], { encoding: 'utf8', stdio: ['ignore', 'pipe', 'pipe'] }).trim().replace(/\n/g, ' | '); }
  catch (e) { out = 'ERR ' + (e.stdout || '').trim().replace(/\n/g, ' | '); }
  const p = parseShell(src);
  console.log(`${k}\n  bash: ${out}\n  parser cmds: ${JSON.stringify(p.cmds.map((c) => c.argv))}`);
}

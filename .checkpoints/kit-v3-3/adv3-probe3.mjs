// advisor pass 3: `>&word` redirect (usage: node adv3-probe3.mjs <routine-guard.mjs> <scratch dir>)
import { execFileSync } from 'node:child_process';
import { readFileSync, existsSync, mkdirSync } from 'node:fs';
const { parseShell } = await import(process.argv[2]);
const dir = process.argv[3]; mkdirSync(dir, { recursive: true });
const src = 'date >&target.txt';
execFileSync('bash', ['--norc', '--noprofile', '-c', src], { cwd: dir });
console.log('bash wrote target.txt:', existsSync(`${dir}/target.txt`), JSON.stringify(readFileSync(`${dir}/target.txt`, 'utf8').slice(0, 4)) );
const p = parseShell(src);
console.log('parser:', JSON.stringify(p.cmds.map((c) => ({ argv: c.argv, redirects: c.redirects }))));

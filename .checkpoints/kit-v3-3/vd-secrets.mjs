// verifier delta: secret-looking strings in the delta's added lines (source and docs, notes excluded) and in this
// review's own notes. Prints file:line and the pattern name only, never the match. Usage: node vd-secrets.mjs <a3>
import { spawnSync } from 'node:child_process';
import { readdirSync, readFileSync } from 'node:fs';
import { join } from 'node:path';
const A3 = process.argv[2];
const P = { 'sk-': /sk-[A-Za-z0-9_-]{20,}/, ghp: /gh[pousr]_[A-Za-z0-9]{30,}/, github_pat: /github_pat_[A-Za-z0-9_]{20,}/,
  AKIA: /(AKIA|ASIA)[A-Z0-9]{16}/, slack: /xox[abposr]-[A-Za-z0-9-]{10,}/, pem: /-----BEGIN [A-Z ]*PRIVATE KEY-----/,
  jwt: /eyJ[A-Za-z0-9_-]{10,}\.[A-Za-z0-9_-]{10,}\./, bearer: /[Bb]earer [A-Za-z0-9._-]{20,}/, urlcred: /\/\/[^/\s:@]+:[^/\s@]{6,}@/ };
const d = spawnSync('git', ['diff', '-U0', '442fbf7', 'ed3ea8c', '--', '.', ':(exclude).checkpoints'], { cwd: A3, encoding: 'utf8', maxBuffer: 64e6 }).stdout;
let file = '', ln = 0, hits = 0, added = 0;
for (const l of d.split('\n')) {
  if (l.startsWith('+++ ')) { file = l.slice(6); continue; }
  const h = l.match(/^@@ -\S+ \+(\d+)/); if (h) { ln = +h[1]; continue; }
  if (l.startsWith('+')) { added++; for (const [k, r] of Object.entries(P)) if (r.test(l)) { hits++; console.log(`delta ${file}:${ln} ${k}`); } ln++; }
}
console.log(`delta added lines ${added}, hits ${hits}`);
const nd = join(A3, '.checkpoints/kit-v3-3'); let nh = 0;
for (const f of readdirSync(nd).filter((f) => f.startsWith('vd') || f.startsWith('verifier-delta'))) {
  readFileSync(join(nd, f), 'utf8').split('\n').forEach((l, i) => { for (const [k, r] of Object.entries(P)) if (r.test(l)) { nh++; console.log(`notes ${f}:${i + 1} ${k}`); } });
}
console.log(`my notes hits ${nh}`);

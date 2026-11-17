// Recomputes every block of the appendix as git stores it and compares it with the sha256 and size written under it.
//   node .checkpoints/cs0-model-rules/appendix-verify.mjs [<git ref>]     (default: the working copy)
import { readFileSync } from 'node:fs';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
const APP = 'docs/reports/2026-11-17-build-plan-slice-1-appendix.md';
const ref = process.argv[2];
const t = ref ? execFileSync('git', ['show', `${ref}:${APP}`], { maxBuffer: 1 << 26 }).toString('utf8') : readFileSync(APP, 'utf8');
const re = /## (A\d+)\. [^\n]*\n````text\n([\s\S]*?)\n````\nsha256 of the block: `([0-9a-f]{64})` \((\d+) bytes\)/g;
let m, n = 0, bad = 0;
while ((m = re.exec(t))) {
  n++; const sha = createHash('sha256').update(m[2]).digest('hex'); const bytes = Buffer.byteLength(m[2]);
  const ok = sha === m[3] && bytes === Number(m[4]); if (!ok) bad++;
  console.log(m[1], bytes, 'bytes', sha.slice(0, 16), ok ? 'matches its line' : 'DOES NOT MATCH');
}
console.log(ref ?? 'working copy', '| blocks:', n, '| mismatches:', bad, '| CR characters:', t.includes('\r') ? 'YES' : 'none');
process.exit(n === 11 && !bad && !t.includes('\r') ? 0 : 1);

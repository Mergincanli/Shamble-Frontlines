// verifier pass 3: secret-looking strings in the added source lines of a diff (stdin); prints counts and file:line only
import { readFileSync } from 'node:fs';
const diff = readFileSync(0, 'utf8').split('\n');
const pats = [['sk-', /sk-(ant-|proj-|svcacct-)?[A-Za-z0-9_-]{20,}/], ['github', /(ghp|gho|ghu|ghs|ghr)_[A-Za-z0-9]{30,}|github_pat_[A-Za-z0-9_]{30,}/],
  ['aws', /(AKIA|ASIA)[0-9A-Z]{16}/], ['slack', /xox[abprs]-[A-Za-z0-9-]{10,}/], ['pem', /-----BEGIN [A-Z ]*PRIVATE KEY/], ['google', /AIza[0-9A-Za-z_-]{35}/],
  ['bearer', /[Bb]earer [A-Za-z0-9._-]{20,}/], ['jwt', /eyJ[A-Za-z0-9_-]{10,}\.[A-Za-z0-9_-]{10,}\./]];
let file = '', ln = 0, added = 0; const hits = [];
for (const l of diff) {
  if (l.startsWith('+++ ')) { file = l.slice(6); continue; }
  const m = l.match(/^@@ -\d+(?:,\d+)? \+(\d+)/); if (m) { ln = +m[1]; continue; }
  if (l.startsWith('+')) { added++; for (const [k, re] of pats) if (re.test(l)) hits.push(`${k} ${file}:${ln}`); ln++; }
  else if (!l.startsWith('-')) ln++;
}
console.log(`added lines ${added}, hits ${hits.length}`); for (const h of hits) console.log(h);

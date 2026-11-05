// C-PLAIN teeth check (builder pass 3, routine side A): run the C-PLAIN test against parseShell mutants in a flat copy.
// usage: node builder-pass3-teeth.mjs <a3 root> <scratch dir>
import { readFileSync, writeFileSync, copyFileSync, mkdirSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import { join } from 'node:path';
const [a3, dir] = process.argv.slice(2);
mkdirSync(dir, { recursive: true });
const src = readFileSync(join(a3, '.claude/hooks/routine-guard.mjs'), 'utf8');
copyFileSync(join(a3, '.claude/hooks/routine-guard.test.mjs'), join(dir, 'routine-guard.test.mjs'));
const muts = [
  ['none', '', ''],
  ['tab is no blank', "if (c === ' ' || c === '\\t') { endWord(); i++; continue; }", "if (c === ' ') { endWord(); i++; continue; }"],
  ['a digit word before > stays a word', 'if (word !== null && /^\\d+$/.test(word)) word = null; else endWord();', 'endWord();'],
  ['single quotes kept in the word', "word = (word ?? '') + s.slice(i + 1, j < 0 ? s.length : j);",
    "word = (word ?? '') + s.slice(i, j < 0 ? s.length : j + 1);"],
  ['&> not a redirect', "if (c === '&' && s[i + 1] === '>') { endWord(); pendingRedirect = true;",
    "if (c === '&' && s[i + 1] === '>') { endWord(); pendingRedirect = false;"],
];
for (const [label, from, to] of muts) {
  if (from && src.split(from).length !== 2) { console.log(label, 'TARGET NOT UNIQUE'); continue; }
  writeFileSync(join(dir, 'routine-guard.mjs'), from ? src.replace(from, () => to) : src);
  const r = spawnSync('node', ['--test', '--test-name-pattern=C-PLAIN', 'routine-guard.test.mjs'], { cwd: dir, encoding: 'utf8' });
  const m = /(\d+) lines where parseShell and bash differ/.exec(r.stdout);
  const other = (r.stdout.match(/error: .*/) || [''])[0].slice(0, 100);
  console.log(label.padEnd(36), /# fail 0/.test(r.stdout) ? 'C-PLAIN passes' : `C-PLAIN fails (${m ? `${m[1]} lines differ` : `other: ${other}`})`);
}

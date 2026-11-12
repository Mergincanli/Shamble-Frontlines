// step7-commands.mjs (foundation-restructure, step 2.1b, step 7): the folder argument "." becomes "legacy" in every
// command of tools/README.md and the tool header comments (plan 1.3, "Not protected (2.1b)"). A controlled edit, not a
// blind one: each file must hold exactly the expected number of `tools/<battery|preview|pageshot>.js .` commands (the
// dot followed by a space or a closing backquote), or nothing is written. Prose, other files and kit files are untouched.
// Run from the repo root: node .gate/foundation-restructure/step7-commands.mjs
import { readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const REPO = '/home/user/Shamble-Frontlines';
const RE = /(tools\/(?:battery|preview|pageshot)\.js) \.(?=[ `])/g;
const WANT = {
  'tools/README.md': 9, 'tools/preview.js': 2, 'tools/pageshot.js': 1, 'tools/sim.js': 1, 'tools/reach.js': 1,
  'tools/perfbots.js': 1, 'tools/tuning-dump.js': 1, 'tools/aabbs.js': 1, 'tools/village.py': 1,
};
const plan = [];
for (const [f, n] of Object.entries(WANT)) {
  const text = readFileSync(join(REPO, f), 'utf8');
  const got = (text.match(RE) || []).length;
  if (got !== n) { console.error(`${f}: ${got} commands, expected ${n}: nothing written`); process.exit(1); }
  plan.push([f, text.replace(RE, '$1 legacy'), got]);
}
for (const [f, text, got] of plan) { writeFileSync(join(REPO, f), text); console.log(`${f}: ${got} replaced`); }
console.log(`step 7 commands: ${plan.reduce((s, p) => s + p[2], 0)} replaced in ${plan.length} files`);

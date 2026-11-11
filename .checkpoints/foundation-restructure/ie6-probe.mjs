// IE-6 probe: the revert direction (a legacy/ base, the head at the root) with a plain FILE named `legacy` in the head.
// Does the fixed gate's move check crash (readdirSync on a file: ENOTDIR, exit 1) or pass? One run, recorded.
// Run from the repo root: node .gate/foundation-restructure/ie6-probe.mjs
import { cpSync, mkdirSync, symlinkSync, copyFileSync, writeFileSync, existsSync } from 'node:fs';
import { join } from 'node:path';
import { spawnSync } from 'node:child_process';

const REPO = '/home/user/Shamble-Frontlines';
const S = join(REPO, '.gate/fdn-scratch-head');            // the fixed gate's tools (blob 7a8480c) and the five files
const DIR = join(REPO, '.gate/foundation-restructure/ie6-probe');
const LEGBASE = '89286ed2191438b58041640afddadf6b08a50149'; // 44446a5 with the five files in legacy/ (reproducible)
if (existsSync(DIR)) { console.error(DIR + ' exists: give a fresh folder'); process.exit(2); }
mkdirSync(DIR, { recursive: true });
cpSync(join(S, 'tools'), join(DIR, 'tools'), { recursive: true, filter: src => !src.endsWith('/node_modules') });
symlinkSync(join(REPO, 'tools/node_modules'), join(DIR, 'tools/node_modules'));
for (const f of ['shamble-core.js', 'shamble-game.js', 'shamble-editor.js', 'index.html', 'editor.html']) copyFileSync(join(S, f), join(DIR, f));
writeFileSync(join(DIR, 'legacy'), 'a plain file named like the old folder\n');
const p = spawnSync(process.execPath, ['tools/gate.mjs', LEGBASE, '--no-smoke'], { cwd: DIR, encoding: 'utf8', maxBuffer: 64 << 20 });
const out = `IE-6 probe (revert direction; head at the root with a plain file named legacy; base ${LEGBASE.slice(0, 7)} in legacy/):
gate blob: tools/gate.mjs from .gate/fdn-scratch-head (7a8480c)
exit: ${p.status}
stdout (first lines):
${(p.stdout || '').split('\n').slice(0, 3).join('\n')}
stderr (first lines):
${(p.stderr || '').split('\n').slice(0, 3).join('\n') || '(none)'}
`;
writeFileSync(join(REPO, '.checkpoints/foundation-restructure/evidence-f1/ie6-probe.txt'), out);
console.log(out);

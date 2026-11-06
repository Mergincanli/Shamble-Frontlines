// Read-only: word-for-word checks for docs-keeper's B.1 pass. Run from the repo root.
import { readFileSync } from 'node:fs';
import { execFileSync } from 'node:child_process';
const git = (r, p) => execFileSync('git', ['show', `${r}:${p}`], { encoding: 'utf8' });
const norm = t => t.split('\n').map(l => l.replace(/^\s*/, '').replace(/^(>\s?)+/, '').replace(/^\s*/, '')).join(' ').replace(/\s+/g, ' ').trim();
const count = (h, n) => { let c = 0, i = 0; while ((i = h.indexOf(n, i)) >= 0) { c++; i += n.length; } return c; };
const status = norm(readFileSync('docs/STATUS.md', 'utf8'));
const a4src = git('4f268da', '.checkpoints/kit-v3-3/queue/a4-spec-2026-11-04.md');
const quoteBlocks = [...a4src.matchAll(/(?:^>.*\n)+/gm)].map(m => m[0]);
let fail = 0; const ok = (c, m) => { if (!c) fail++; console.log(`${c ? 'ok  ' : 'FAIL'} ${m}`); };
ok(quoteBlocks.length === 2, `A4 source has 2 quote blocks (${quoteBlocks.length})`);
quoteBlocks.forEach((b, i) => { const n = count(status, norm(b)); ok(n === 1, `STATUS holds A4 quote block ${i + 1} once (${n})`); });
const fz = 'A4-FREEZE: (a). A4 waits until the netcode proof test is under way.';
ok(git('4f268da', '.checkpoints/kit-v3-3/queue/mustafa-2026-11-05-a4-b1.md').includes(fz), 'source holds A4-FREEZE line');
ok(count(status, fz) === 1, `STATUS holds A4-FREEZE once (${count(status, fz)})`);
const add = git('4f268da', '.checkpoints/kit-v3-3/queue/mustafa-2026-11-05-a4-b1.md');
for (const q of ['one consistent scale across the engine', '1 unit = 1 metre', 'textures tile in world space (they repeat per metre and never stretch)', 'you can keep editing while the match runs'])
  ok(add.includes(q) && status.includes(q), `his phrase in source and STATUS: "${q}"`);
ok(readFileSync('docs/adr/0004-simulation-core.md', 'utf8').split('\n')[260].includes('a map 2 km across'), 'ADR 0004:261 "a map 2 km across"');
ok(readFileSync('docs/adr/0005-content-abilities-assets.md', 'utf8').split('\n')[400].includes('metres per second'), 'ADR 0005:401 "metres per second"');
const a5 = readFileSync('docs/adr/0005-content-abilities-assets.md', 'utf8').split('\n').slice(668, 680).join(' ');
ok(a5.includes('OQ-0005-CD4') && a5.includes('next match') && a5.includes('R4-PHYS-REC2 keeps the static world for the match'), 'ADR 0005:669-680 OQ-0005-CD4, next match, R4-PHYS-REC2');
const ed = readFileSync('docs/EDITOR_V2.md', 'utf8').split('\n');
ok(ed[88].includes('It still runs by double-clicking editor.html'), 'EDITOR_V2.md:89');
ok(ed[89].includes('if ADR-002 moves the project to modules'), 'EDITOR_V2.md:90');
ok(ed.join('\n').includes('Static merging happens only when you press Test'), 'EDITOR_V2 §2 rule 4 text');
ok(readFileSync('docs/adr/README.md', 'utf8').split('\n')[117].startsWith('| OQ-0005-CD4 |') , 'adr/README.md:118 is the OQ-0005-CD4 row');
console.log(fail ? `VERBATIM FAIL (${fail})` : 'VERBATIM PASS'); process.exit(fail ? 1 : 0);

// docs-keeper scratch: replace STATUS's "From foundation 2.2" follow-ups bullet (8 lines now) with dk-para-wrapped.txt.
import { readFileSync, writeFileSync } from 'node:fs';
const f = '/home/user/Shamble-Frontlines/docs/STATUS.md';
const lines = readFileSync(f, 'utf8').split('\n');
const s = lines.findIndex((l) => l.startsWith('- **From foundation 2.2 (PR #48), for 2.3:**'));
let e = s; while (!lines[e].endsWith('crate or a server crate.')) e++;
if (s < 0 || e - s !== 7) throw new Error(`block ${s}-${e}`);
const add = readFileSync('/home/user/Shamble-Frontlines/.checkpoints/foundation-2-3-docs/dk-para-wrapped.txt', 'utf8').replace(/\n$/, '').split('\n');
lines.splice(s, e - s + 1, ...add);
writeFileSync(f, lines.join('\n'));
console.log(`replaced ${s + 1}-${e + 1} with ${add.length} lines`);

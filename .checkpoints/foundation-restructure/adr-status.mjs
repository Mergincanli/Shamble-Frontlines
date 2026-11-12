// adr-status.mjs (foundation-restructure, step 2.1b, step 10; the advisor agent's C10): ADR 0002's Status line records
// Mustafa's acceptance of part 1 by quoting his SCOPE (a) whole, copied by script from docs/DECISIONS.md (the line that
// starts "> SCOPE: (a). I accept", his record of 2026-11-10 in the foundation chat; never the kit chat's shorter wording
// further down), "Proposed" kept first. Only line 3 of the ADR changes.
// Checks: exactly one SCOPE line, ending "Nothing else in ADR 0002 is accepted by this."; the pointer's entry name exists
// in DECISIONS.md; the new line holds the quote byte for byte; every other line of the ADR is unchanged.
//   node .gate/foundation-restructure/adr-status.mjs dry | write
import { readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const REPO = '/home/user/Shamble-Frontlines';
const ADR = 'docs/adr/0002-code-structure-build-toolchain.md';
const mode = process.argv[2];
if (!['dry', 'write'].includes(mode)) { console.error('usage: adr-status.mjs dry | write'); process.exit(2); }
let ok = true;
const check = (name, pass, detail = '') => { if (!pass) ok = false; console.log(`${pass ? 'PASS' : 'FAIL'} ${name}${detail ? ' :: ' + detail : ''}`); };
const dec = readFileSync(join(REPO, 'docs/DECISIONS.md'), 'utf8');
const hits = dec.split('\n').filter(l => l.startsWith('> SCOPE: (a). I accept'));
check('exactly one SCOPE (a) line in DECISIONS.md', hits.length === 1, `${hits.length} found`);
const SCOPE = (hits[0] || '').slice(2);
check('it is the whole record, ending "Nothing else in ADR 0002 is accepted by this."', SCOPE.endsWith('Nothing else in ADR 0002 is accepted by this.'));
check('it holds no double quote (safe inside quote marks)', !SCOPE.includes('"'));
const ENTRY = 'Taken by Mustafa, 2026-11-10: FREEZE-FDN (a), SPLIT (a), SCOPE (a) and the build go for 2.1a, in the foundation chat';
check('the pointer names an entry that exists in DECISIONS.md', dec.replace(/\s+/g, ' ').includes('**' + ENTRY + '**'));
const adr = readFileSync(join(REPO, ADR), 'utf8').split('\n');
check(`${ADR}:3 is "**Status:** Proposed" before the edit`, adr[2] === '**Status:** Proposed', adr[2]);
const NEW = `**Status:** Proposed. Part 1 accepted by Mustafa on 2026-11-10, his SCOPE (a), word for word: "${SCOPE}" (\`docs/DECISIONS.md\`, the entry "${ENTRY}")`;
const out = [...adr]; out[2] = NEW;
check('the new line keeps "Proposed" first and holds the quote byte for byte', NEW.startsWith('**Status:** Proposed. ') && NEW.includes(`"${SCOPE}"`));
check('every other line of the ADR is unchanged', out.length === adr.length && out.every((l, i) => i === 2 || l === adr[i]));
if (!ok) { console.error('ADR status: FAIL (nothing written)'); process.exit(1); }
const target = mode === 'write' ? join(REPO, ADR) : join(REPO, '.gate/foundation-restructure/0002.dry.md');
writeFileSync(target, out.join('\n'));
console.log(`ADR status: PASS; written to ${target.replace(REPO + '/', '')}`);

// Adds the register row Q2.2-a to docs/adr/README.md ("Decisions the drafts ask for", after ADR 0002's last row): the
// reading that moves step 2.2's G0 mark to step 2.3, with Mustafa's sentence on it copied by script from the report's
// Appendix A2 (never retyped). Run once from the worktree's top: node .checkpoints/foundation-toolchain/register-row.mjs
import { readFileSync, writeFileSync } from 'node:fs';
import { reportBlocks, REPORT } from './report-appendix.mjs';

const FILE = 'docs/adr/README.md';
const AFTER = '| OQ-0002-9 (PORTAL-EMBED) |';
const a2 = reportBlocks(readFileSync(REPORT, 'utf8')).get('A2');
const his = a2.split('\n').find((l) => l.startsWith('The ADR index entry that moves the missing-toolchain-file check'));
if (!his) throw new Error('A2: his sentence on the ADR index entry is missing');
const row = '| Q2.2-a (G0\'s exit-2 mark; ADR 0002 migration plan, phase 2, step 2) | Step 2.2\'s done-when says "deleting the ' +
  'toolchain file fails G0 with exit 2", but G0 is a stage of the gate v2 (part 9), which step 2.3 builds. Answered by ' +
  'the advisor agent in foundation 2.2\'s plan review (2026-11-10, final): the mark moves to step 2.3, where `--prove` ' +
  'runs it (PM-0002-39). Until then, step 2.2\'s CI job fails closed before any cargo call when the file is gone (its ' +
  'planted failure M2, `docs/reports/2026-11-12-foundation-2-2.md`). Mustafa (2026-11-10): "' + his + '" His go for ' +
  'this row: PR #48\'s merge. | the advisor agent; Mustafa\'s go | [ADR 0002](0002-code-structure-build-toolchain.md) ' +
  'migration plan, phase 2, steps 2 and 3; PM-0002-39 |';

const lines = readFileSync(FILE, 'utf8').split('\n');
if (lines.some((l) => l.startsWith('| Q2.2-a '))) { console.error('the row is already there'); process.exit(1); }
const at = lines.findIndex((l) => l.startsWith(AFTER));
if (at < 0 || lines.filter((l) => l.startsWith(AFTER)).length !== 1) throw new Error('the anchor row is not there once');
lines.splice(at + 1, 0, row);
writeFileSync(FILE, lines.join('\n'));
const back = readFileSync(FILE, 'utf8');
console.log(back.includes(`"${his}"`) && back.split('\n')[at + 1] === row ? `OK: row Q2.2-a after line ${at + 1}, his sentence byte for byte from A2` : 'FAIL');

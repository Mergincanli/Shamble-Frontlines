// Assembles docs/reports/2026-11-11-foundation-2-1a.md: the report body from the scratchpad draft (placeholders filled
// from the arguments and the regenerated refusal table) above the existing "## Appendix A." marker, which stays as is.
// Run from the repo root after refusal-routes.mjs:
//   node .gate/foundation-restructure/assemble-report.mjs <draft.md> VERIFIER=... LAST_CKPT_SHA=... N_CONSULT=... CHAT_FULL=...
import { readFileSync, writeFileSync } from 'node:fs';

const [draftPath, ...pairs] = process.argv.slice(2);
const vals = Object.fromEntries(pairs.map(p => [p.slice(0, p.indexOf('=')), p.slice(p.indexOf('=') + 1)]));
const table = readFileSync('.gate/foundation-restructure/refusal-routes.md', 'utf8').trimEnd();
vals.REFUSAL_TABLE = table;
vals.REFUSAL_COUNT = String(table.split('\n').length - 2);
let top = readFileSync(draftPath, 'utf8');
for (const [k, v] of Object.entries(vals)) top = top.split(k).join(v);
const left = top.match(/\b(VERIFIER|LAST_CKPT_SHA|N_CONSULT|CHAT_FULL|REFUSAL_COUNT|REFUSAL_TABLE)\b/g);
if (left) { console.error('placeholders left: ' + [...new Set(left)].join(', ')); process.exit(1); }
const REPORT = 'docs/reports/2026-11-11-foundation-2-1a.md';
const old = readFileSync(REPORT, 'utf8');
const i = old.indexOf('## Appendix A.');
if (i < 0) { console.error('no Appendix A marker'); process.exit(1); }
writeFileSync(REPORT, top.trimEnd() + '\n\n' + old.slice(i));
console.log(`report assembled: ${top.split('\n').length} lines above Appendix A; ${vals.REFUSAL_COUNT} refusals`);

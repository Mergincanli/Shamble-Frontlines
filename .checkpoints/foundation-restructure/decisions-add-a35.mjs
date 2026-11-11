// Inserts Mustafa's correction of 2026-11-11 16:54:54 UTC (the report's Appendix A3.5) at the top of docs/DECISIONS.md's
// "Decisions, newest first", directly above the 16:32 entry it corrects, quoted line by line from the appendix (never
// retyped), under a header written for this file. Idempotent. Run from the repo root:
// node .gate/foundation-restructure/decisions-add-a35.mjs
import { readFileSync, writeFileSync } from 'node:fs';

const appendix = readFileSync('docs/reports/2026-11-11-foundation-2-1a.md', 'utf8').split('## Appendix A.')[1];
const m = appendix.match(/^### A3\.5\. .*?\n\n```text\n([\s\S]*?)\n```\n/m);
if (!m) { console.error('Appendix A3.5 not found'); process.exit(1); }
const block = m[1].split('\n');
if (!block[0].startsWith('My words of 2026-11-11, a correction to my words on permission prompts of 16:32 UTC')) {
  console.error('unexpected first line: ' + block[0]); process.exit(1);
}
const HEADER = `- **Taken by Mustafa, 2026-11-11, 16:54:54 UTC: a correction to his words on permission prompts (the entry directly
  below)** (foundation 2.1a, PR #50; his message in that change-set's session, session_019YNMUWLffMoE47SbvkqJ5P, on his
  personal account, extracted by script from the session's events, event a10ee1d6-3120-4eb1-b83a-4e84afe50a94). Placed
  next to the words it corrects, as he asked. Copied by script from \`docs/reports/2026-11-11-foundation-2-1a.md\`,
  Appendix A3.5, and checked byte for byte against it:`;
const quote = block.map(l => (l ? '> ' + l : '>')).join('\n');
const path = 'docs/DECISIONS.md';
let text = readFileSync(path, 'utf8');
if (text.includes(HEADER.split('\n')[0])) { console.log('already there: nothing to do'); process.exit(0); }
const anchor = '## Decisions, newest first\n';
const next = '- **Taken by Mustafa, 2026-11-11, 16:32:18 UTC: his words on permission prompts**';
if (text.split(anchor).length !== 2) { console.error('anchor not unique'); process.exit(1); }
if (!text.split(anchor)[1].startsWith(next)) { console.error('the 16:32 entry is not at the top: stop'); process.exit(1); }
text = text.replace(anchor, anchor + HEADER + '\n' + quote + '\n');
writeFileSync(path, text);
console.log(`inserted above the 16:32 entry: header ${HEADER.split('\n').length} lines, quote ${block.length} lines`);

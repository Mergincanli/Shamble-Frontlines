// Inserts Mustafa's message of 2026-11-11 16:32:18 UTC on permission prompts (the report's Appendix A3.4) at the top of
// docs/DECISIONS.md's "Decisions, newest first", quoted line by line from the appendix (never retyped), under a header
// written for this file. Idempotent. Run from the repo root: node .gate/foundation-restructure/decisions-add-a34.mjs
import { readFileSync, writeFileSync } from 'node:fs';

const appendix = readFileSync('docs/reports/2026-11-11-foundation-2-1a.md', 'utf8').split('## Appendix A.')[1];
const m = appendix.match(/^### A3\.4\. .*?\n\n```text\n([\s\S]*?)\n```\n/m);
if (!m) { console.error('Appendix A3.4 not found'); process.exit(1); }
const block = m[1].split('\n');
if (block[0] !== 'My words of 2026-11-11 on permission prompts, for the foundation chat (PR #50, foundation step 2.1a).') {
  console.error('unexpected first line: ' + block[0]); process.exit(1);
}
const HEADER = `- **Taken by Mustafa, 2026-11-11, 16:32:18 UTC: his words on permission prompts** (foundation 2.1a, PR #50; his message
  in that change-set's session, session_019YNMUWLffMoE47SbvkqJ5P, on his personal account, extracted by script from the
  session's events, event 557d0202-6418-4c26-9d03-9b1ee13fd5bf). Its two report items (the commands that asked him most
  that day; every refusal by the guard, the deny list or the classifier, with the route taken) are carried out by
  \`docs/reports/2026-11-11-foundation-2-1a.md\`, as findings for 2.3. Copied by script from
  \`docs/reports/2026-11-11-foundation-2-1a.md\`, Appendix A3.4, and checked byte for byte against it:`;
const quote = block.map(l => (l ? '> ' + l : '>')).join('\n');
const path = 'docs/DECISIONS.md';
let text = readFileSync(path, 'utf8');
if (text.includes(HEADER.split('\n')[0])) { console.log('already there: nothing to do'); process.exit(0); }
const anchor = '## Decisions, newest first\n';
if (text.split(anchor).length !== 2) { console.error('anchor not unique'); process.exit(1); }
text = text.replace(anchor, anchor + HEADER + '\n' + quote + '\n');
writeFileSync(path, text);
console.log(`inserted: header ${HEADER.split('\n').length} lines, quote ${block.length} lines`);

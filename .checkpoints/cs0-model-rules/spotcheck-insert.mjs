// Change-set 0, the advisor agent's condition C1: each research file gets its "Verifier spot-check" section
// (docs/research/README.md, the shape of a topic file, item 7). The verifier's verdict lines are copied from its
// notes, wrapped at 120 columns, never retyped. Run once from the repository root:
//   node .checkpoints/cs0-model-rules/spotcheck-insert.mjs
import { readFileSync, writeFileSync } from 'node:fs';
const notes = readFileSync('.checkpoints/cs0-model-rules/verifier-cs0.md', 'utf8').split('\n');
const wrap = (text) => { // greedy wrap at 120 columns, list continuation indented by two
  const out = []; let line = '';
  for (const w of text.split(' ')) {
    if (line && line.length + 1 + w.length > 120) { out.push(line); line = '  ' + w; } else line += (line ? ' ' : '') + w;
  }
  out.push(line); return out.join('\n');
};
const pick = (starts) => notes.filter((l) => starts.some((s) => l.startsWith(s)));
const jl = pick(['- JL']);
const aer = pick(['- AER-', '- A3 line refs', '- glTF spec', '- Blender exporter', '- three.js', '- Not reachable', '- More sources:',
  '- Agreement AER', '- Agreement with ADR', '- [inferred]:', '- Nothing unproven']);
const count = pick(['- SPOT-CHECK COUNT.']);
if (jl.length !== 14 || aer.length !== 11 || count.length !== 1) throw new Error(`lines found: ${jl.length}, ${aer.length}, ${count.length}`);
const head = (n, m, rest) => `## Verifier spot-check\nOne verifier pass (Opus 5.5, effort max) checked this file on 2026-11-17 at commit cacea55, and took a second look\nat the commits after it the same day. Verdict: PASS. ${n} of ${m} key claims were checked at their sources: ${n}\nconfirmed, 0 wrong, 0 unsupported${rest} The lines below are the verifier's own, copied by script from its notes\n(\`verifier-cs0.md\`, in the commit that this change-set's report names): claim, source, verdict, place. They describe\nthe file as it stood at cacea55.\n\n`;
const section = (h, lines) => h + lines.map(wrap).join('\n') + '\n';
const J = 'docs/research/soldier-joint-list.md', A = 'docs/research/asset-export-rules.md';
let tj = readFileSync(J, 'utf8'), ta = readFileSync(A, 'utf8');
if (tj.includes('## Verifier spot-check') || ta.includes('## Verifier spot-check')) throw new Error('already inserted');
const SRC = '## 9. Sources\n'; if (ta.split(SRC).length !== 2) throw new Error('the Sources heading must stand once');
tj = tj.replace(/\n*$/, '\n') + '\n' + section(head(36, 36, '.'), jl);
ta = ta.replace(SRC, section(head(35, 36, '; the one not checked is part 2\'s claim that no draft fixes slot names,\norigin, facing, joints or sockets. Its finding 5 came after: the citations of ADR 0005 and SPIKE_PLAN lines were\nrenumbered to this branch.'), [...aer, ...count]) + '\n' + SRC);
writeFileSync(J, tj); writeFileSync(A, ta);
console.log('inserted: joint list', jl.length, 'lines; rules', aer.length + count.length, 'lines');

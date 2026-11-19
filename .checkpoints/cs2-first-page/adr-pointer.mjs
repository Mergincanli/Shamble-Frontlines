// The docs pass of change-set 2: ADR 0002's dated pointer for SPLIT-25 (the build plan's report, line 23: "ADR 0002
// still says one change-set per step: change-set 2's docs pass carries that line"). His words are taken from
// docs/DECISIONS.md by script, never retyped. Run from the repository root:
//   node .checkpoints/cs2-first-page/adr-pointer.mjs [--check]
import { readFileSync, writeFileSync } from 'node:fs';
const ADR = 'docs/adr/0002-code-structure-build-toolchain.md';
const dec = readFileSync('docs/DECISIONS.md', 'utf8');
const ENTRY = 'Taken by Mustafa, 2026-11-17, 19:10:44 UTC: his go on the build plan';
const from = dec.indexOf(`- **${ENTRY}`);
if (from < 0) throw new Error('the entry of 2026-11-17 is not in DECISIONS');
const line = dec.slice(from).split('\n').find((l) => l.startsWith('> 1. SPLIT-25: '));
if (line === undefined) throw new Error('answer 1 (SPLIT-25) is not in that entry');
const his = line.slice('> 1. SPLIT-25: '.length);
const wrap = (text, indent) => { const out = []; let cur = indent;
  for (const w of text.split(' ')) { if (cur.trim() !== '' && cur.length + 1 + w.length > 120) { out.push(cur); cur = indent + w; } else cur += (cur.trim() === '' ? '' : ' ') + w; }
  out.push(cur); return out.join('\n'); };
const pointer = wrap(`_Pointer (2026-11-19, change-set 2 of the build plan): for this step, "one change-set per step" above is changed by Mustafa's answer SPLIT-25 of 2026-11-17 (\`docs/DECISIONS.md\`, the entry "${ENTRY}", answer 1): "${his}" They are the build plan's change-sets 2 ("First page": the path from the simulation to the screen, with one block per tick, the empty scene, its named backend and zero Wasm calls from the page) and 3 ("The loop closes": the messages over the loopback and the first recording). The "Done when" above is unchanged, and it is met only when both have landed._`, '      ');
const AFTER = "      match natively twice and in Wasm (ADR 0004's milestone).\n";
const BEFORE = "   6. *Previews.*";
let t = readFileSync(ADR, 'utf8');
if (!process.argv.includes('--check')) {
  if (t.includes('_Pointer (2026-11-19, change-set 2 of the build plan)')) throw new Error('already inserted');
  if (t.split(AFTER + BEFORE).length !== 2) throw new Error('the place (the end of step 5, before step 6) must stand once');
  t = t.replace(AFTER + BEFORE, AFTER + pointer + '\n' + BEFORE);
  writeFileSync(ADR, t);
}
t = readFileSync(ADR, 'utf8');
const ok = {
  'the pointer stands once, between step 5 and step 6': t.split(AFTER + pointer + '\n' + BEFORE).length === 2,
  'his words in it equal answer 1 in DECISIONS': pointer.replace(/\n {6}/g, ' ').includes(`"${his}"`) && his === '(a). Step 2.5 is built as two change-sets.'.slice(0, his.length) && his.length > 10,
  'no line of it over 120 characters': pointer.split('\n').every((l) => l.length <= 120),
};
console.log(pointer);
console.log(`his words (answer 1): ${JSON.stringify(his)}`);
for (const [k, v] of Object.entries(ok)) console.log(`${v ? 'yes' : 'NO '}  ${k}`);
process.exit(Object.values(ok).every(Boolean) ? 0 : 1);

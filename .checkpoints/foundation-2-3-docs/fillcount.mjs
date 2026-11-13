// docs-keeper scratch: how many lines each STATUS placeholder fill takes (wrap copied from pointers.mjs).
function wrap(text, indent, width = 120) { const out = []; let line = indent;
  for (const w of text.split(' ')) { const cand = line === indent ? indent + w : `${line} ${w}`;
    if (cand.length > width && line !== indent) { out.push(line); line = indent + w; } else line = cand; }
  out.push(line); return out; }
const q = 'Step 2.3 may be planned now. The parts of step 2.3 that need no proof test\'s result may be built before the proof tests report, after my go on the plan. Every gate check that needs a result joins later, when that result is in, with its pass marks unchanged. Steps 2.4 to 2.6 still wait for their results.';
const p8 = `_Pointer (2026-11-13): for foundation step 2.3 only, his decision of 2026-11-13 (\`docs/DECISIONS.md\`): "${q}"_`;
const k = '"The kit\'s freeze (KIT-FREEZE-2) ends with this step. That makes no edit unasked: every edit to a kit, gate or CI file is asked of me in chat with its exact text and written only after my yes, with the chat in Accept edits. A guard prompt alone is not my yes."';
console.log('P8 lines', wrap(p8, '   ').length, 'Q-KIT lines', wrap(k, '  ').length);

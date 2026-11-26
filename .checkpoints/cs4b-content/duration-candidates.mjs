// Lists every number of docs/legacy-tuning.json by its path, for the reading of which of them are durations (the
// time unit's rounding report, PM-0004-11, needs a reviewed list of duration paths with their unit). Array places are
// folded into [*] so that one line stands for one field; each line gives the count of places and the distinct
// texts as the file writes them. No value is typed here. From the worktree's root:
//   node .checkpoints/cs4b-content/duration-candidates.mjs
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';
const bytes = readFileSync('docs/legacy-tuning.json');
const fields = new Map();
const path = [];
JSON.parse(bytes.toString('utf8'), function (key, value, context) {
  return value;
});
// A walk of our own, so that each number keeps its path; the texts come from the reviver's source access.
const texts = [];
JSON.parse(bytes.toString('utf8'), (_key, value, context) => { if (typeof value === 'number') texts.push(context.source); return value; });
let next = 0;
const walk = (value, at) => {
  if (typeof value === 'number') {
    const field = at.replace(/\[\d+\]/g, '[*]');
    const entry = fields.get(field) ?? { count: 0, texts: new Set(), first: at };
    entry.count += 1;
    entry.texts.add(texts[next]);
    next += 1;
    fields.set(field, entry);
  } else if (value !== null && typeof value === 'object') {
    for (const [key, inner] of Object.entries(value)) walk(inner, Array.isArray(value) ? `${at}[${key}]` : at === '' ? key : `${at}.${key}`);
  }
};
walk(JSON.parse(bytes.toString('utf8')), '');
if (next !== texts.length) throw new Error(`the walk met ${next} numbers and the reviver ${texts.length}`);
const lines = [...fields].map(([field, entry]) => `${field}\t${entry.count}\t${[...entry.texts].slice(0, 12).join(' ')}${entry.texts.size > 12 ? ' ...' : ''}`);
const head = `# every number field of docs/legacy-tuning.json (sha256 ${createHash('sha256').update(bytes).digest('hex')}): ${texts.length} numbers in ${fields.size} fields; field, places, distinct texts (12 at most)`;
writeFileSync('.checkpoints/cs4b-content/duration-candidates.txt', `${head}\n${lines.join('\n')}\n`);
console.log(head);

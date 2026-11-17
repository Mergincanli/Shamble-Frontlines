// docs-keeper (cs0): fold ROADMAP's oldest "Recent work" entry (#42, one line) into "Build history" after the newest
// folded entry (#39), by line numbers, never retyped. Usage: node docs-keeper-fold.mjs <ROADMAP.md> [--write]
import { readFileSync, writeFileSync } from 'node:fs';

const [file, flag] = process.argv.slice(2);
const text = readFileSync(file, 'utf8');
const lines = text.split('\n'); // the last element is '' (the file ends with a newline)
const FROM = 239, AFTER = 87; // 1-based
const src = lines[FROM - 1];
const anchor = lines[AFTER - 1];
const ok =
  src.startsWith("**VISION_V2's community-servers note and B3's hand-over [2026-11-06, MERGED]**") &&
  anchor.startsWith("- **EDITOR_V2's plan view and Mustafa's editor decisions [2026-11-06, MERGED]**") &&
  lines[FROM - 2] === '' && lines.length === FROM + 1 && lines[FROM] === '';
if (!ok) { console.error('fold: the lines are not where expected; nothing written'); process.exit(1); }
const out = [...lines];
out.splice(FROM - 2, 2); // drop the blank line before the entry and the entry itself
out.splice(AFTER, 0, '- ' + src); // insert after the anchor (the anchor is above FROM, so its index is unchanged)
const result = out.join('\n');
const moved = result.split('\n').filter((l) => l === '- ' + src).length;
console.log(`lines before ${lines.length - 1}, after ${out.length - 1}; moved line found ${moved}x; bytes kept: ${src.length}`);
if (flag === '--write') { writeFileSync(file, result); console.log('written'); }

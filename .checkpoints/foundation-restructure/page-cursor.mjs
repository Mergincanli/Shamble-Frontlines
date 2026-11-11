// Prints the paging cursor of a saved list_events page: first_id, has_more, the event count and the time span.
// Run: node .gate/foundation-restructure/page-cursor.mjs <saved-file>
import { readFileSync } from 'node:fs';
const raw = readFileSync(process.argv[2], 'utf8');
const d = JSON.parse(raw.slice(raw.indexOf('{"ccr"'), raw.lastIndexOf('}') + 1)).ccr;
const t = d.data.map(e => e.created_at).sort();
console.log(JSON.stringify({ first_id: d.first_id, has_more: d.has_more, n: d.data.length, from: t[0], to: t[t.length - 1] }));

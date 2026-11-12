// Writes .checkpoints/foundation-restructure/<out>.md: one of Mustafa's messages in this session, found in a saved
// list_events page by its opening line, extracted verbatim (never retyped) into a fenced block with a source header.
// Adapted from 2.1a's capture-message.mjs (c8f4bb3). Run from the repo root:
//   node .gate/foundation-restructure/capture-message.mjs <events-file> <out-name> <lead line> <title>
import { readFileSync, writeFileSync } from 'node:fs';

const SESSION = 'session_019YNMUWdGJZoQFUuuvvBP1Y';
const [file, outName, lead, title] = process.argv.slice(2);
if (!file || !outName || !lead || !title) { console.error('usage: capture-message.mjs <events-file> <out-name> <lead line> <title>'); process.exit(2); }
const raw = readFileSync(file, 'utf8');
const data = JSON.parse(raw.slice(raw.indexOf('{"ccr"'), raw.lastIndexOf('}') + 1));
const found = [];
(function walk(o) {
  if (Array.isArray(o)) { o.forEach(walk); return; }
  if (!o || typeof o !== 'object') return;
  const u = o.user;
  const c = u && u.internal_anthropic_catchall && u.internal_anthropic_catchall.message && u.internal_anthropic_catchall.message.content;
  if (typeof c === 'string' && c.startsWith(lead)) found.push({ uuid: u.uuid, when: o.created_at, text: c });
  Object.values(o).forEach(walk);
})(data);
if (found.length !== 1) { console.error(`found ${found.length} events`); process.exit(1); }
const { uuid, when, text } = found[0];
if (text.includes('```')) { console.error('fence inside the text'); process.exit(1); }
const out = `# ${title}

Source: his chat message in ${SESSION} (his personal account, foundation step 2.1b), event ${uuid}, at
${when}, extracted by \`capture-message.mjs\` (run from \`.gate/foundation-restructure/\`; a copy beside this file) from
the session's own events (\`list_events\`), never retyped.

\`\`\`text
${text}
\`\`\`
`;
writeFileSync(`.checkpoints/foundation-restructure/${outName}.md`, out);
console.log(`wrote ${outName}.md; ${text.length} characters; event ${uuid} at ${when}`);

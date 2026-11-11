// Writes .checkpoints/foundation-restructure/mustafa-2026-11-11-prompts.md: Mustafa's message of 2026-11-11 on permission
// prompts, in this session, extracted from the session's own events (list_events output saved by the tools), never
// retyped. Run from the repo root: node .gate/foundation-restructure/capture-2026-11-11-prompts.mjs <events-file>
import { readFileSync, writeFileSync } from 'node:fs';

const LEAD = 'My words of 2026-11-11 on permission prompts, for the foundation chat (PR #50, foundation step 2.1a).';
const raw = readFileSync(process.argv[2], 'utf8');
const data = JSON.parse(raw.slice(raw.indexOf('{"ccr"'), raw.lastIndexOf('}') + 1));
const found = [];
(function walk(o) {
  if (Array.isArray(o)) { o.forEach(walk); return; }
  if (!o || typeof o !== 'object') return;
  const u = o.user;
  const c = u && u.internal_anthropic_catchall && u.internal_anthropic_catchall.message && u.internal_anthropic_catchall.message.content;
  if (typeof c === 'string' && c.startsWith(LEAD)) found.push({ uuid: u.uuid, when: o.created_at, text: c });
  Object.values(o).forEach(walk);
})(data);
if (found.length !== 1) { console.error(`found ${found.length} events`); process.exit(1); }
const { uuid, when, text } = found[0];
if (text.includes('```')) { console.error('fence inside the text'); process.exit(1); }
const out = `# Mustafa's message of 2026-11-11 on permission prompts, in the replacement session, word for word

Source: his chat message in session_019YNMUWLffMoE47SbvkqJ5P (his personal account, PR #50), event ${uuid}, at
${when}, extracted by \`capture-2026-11-11-prompts.mjs\` (run from \`.gate/foundation-restructure/\`; a copy beside this
file) from the session's own events (\`list_events\`), never retyped.

\`\`\`text
${text}
\`\`\`
`;
writeFileSync('.checkpoints/foundation-restructure/mustafa-2026-11-11-prompts.md', out);
console.log(`wrote mustafa-2026-11-11-prompts.md; ${text.length} characters; event ${uuid} at ${when}`);

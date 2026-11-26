// Saves Mustafa's Testlines Export at 30 steps from the chat's transcript, never retyped (his point 8 of 2026-11-25:
// "The values come from my Export and from nothing else"; his block of that day: "Save it whole by script with its
// byte count and sha256 before you read it"). His message of 2026-11-26 holds lines of his own and then the Export,
// so this script finds that one message (sent while the chat was working: a queued command in the transcript), takes
// the Export out of it byte for byte (from the message's first line that is `{` to its last line that is `}`, both
// included, no byte added), and writes it with a manifest. It then lists, by script, where its values differ from
// the Export at 60 steps that 4a's notes hold. From the worktree's root:
//   node .checkpoints/cs4b-content/export-save.mjs <transcript.jsonl> [message uuid]
// Node's built-in modules only. The whole message is saved beside his other messages by cs4-walk/chat-words.mjs.
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';
const NOTES = '.checkpoints/cs4b-content/';
const EARLIER = '.checkpoints/cs4-walk/testlines-export.json';
const GAME_STEP_RATE = 30;
const [transcript, wanted] = process.argv.slice(2);
if (!transcript) { console.error('usage: export-save.mjs <transcript.jsonl> [message uuid]'); process.exit(2); }
const sha = (s) => createHash('sha256').update(s).digest('hex');
const hits = [];
for (const line of readFileSync(transcript, 'utf8').split('\n')) {
  if (!line.includes('testlines-export')) continue;
  let e;
  try { e = JSON.parse(line); } catch { continue; }
  if (e.isSidechain) continue;
  let texts = [];
  let how = '';
  if (e.type === 'user' && !e.isMeta && !e.isCompactSummary && e.toolUseResult === undefined) {
    const c = e.message?.content;
    texts = typeof c === 'string' ? [c] : Array.isArray(c) ? c.filter((p) => p.type === 'text').map((p) => p.text) : [];
    how = 'the user message';
  } else if (e.type === 'attachment' && e.attachment?.type === 'queued_command' && typeof e.attachment.prompt === 'string') {
    texts = [e.attachment.prompt];
    how = 'the message he sent while the chat was working (stored as a queued command)';
  }
  for (const t of texts) {
    // The Export inside the message: from its first line that is `{` to its last line that is `}`.
    const start = t.startsWith('{\n') ? 0 : t.indexOf('\n{\n') + 1;
    const end = t.endsWith('\n}') ? t.length : t.lastIndexOf('\n}\n') + 2;
    if ((start === 0 && !t.startsWith('{\n')) || end < 2 || end <= start) continue;
    const text = t.slice(start, end);
    let doc;
    try { doc = JSON.parse(text); } catch { continue; }
    if (doc && doc.format === 'testlines-export' && doc.values?.simulation?.step_rate_hz === GAME_STEP_RATE) {
      hits.push({ text, doc, before: t.slice(0, start), after: t.slice(end), whole: t, at: e.timestamp, id: e.uuid, how });
    }
  }
}
const chosen = wanted ? hits.filter((h) => h.id === wanted) : hits;
if (chosen.length !== 1) { console.error(`expected one message with an Export at ${GAME_STEP_RATE} steps, found ${chosen.length}${hits.length ? ` (${hits.map((h) => h.id).join(', ')})` : ''}`); process.exit(1); }
const { text, doc, before, after, whole, at, id, how } = chosen[0];
writeFileSync(NOTES + 'testlines-export-30.json', text);
const warnings = Array.isArray(doc.warnings) ? doc.warnings : null;
const changed = Array.isArray(doc.changed_from_start) ? doc.changed_from_start : null;
const usable = warnings !== null && warnings.length === 0;
// Every leaf of a value with its path, for the comparison with the earlier Export.
const leaves = (value, path, out) => {
  if (value !== null && typeof value === 'object') for (const [key, inner] of Object.entries(value)) leaves(inner, path === '' ? key : `${path}.${key}`, out);
  else out.set(path, JSON.stringify(value));
  return out;
};
const earlierText = readFileSync(EARLIER, 'utf8');
const [now, then] = [leaves(doc, '', new Map()), leaves(JSON.parse(earlierText), '', new Map())];
const differing = [...new Set([...now.keys(), ...then.keys()])].filter((path) => now.get(path) !== then.get(path));
const lines = [
  `file: ${NOTES}testlines-export-30.json`,
  `bytes: ${Buffer.byteLength(text)}`,
  `sha256: ${sha(text)}`,
  `source: ${how} of ${at} (uuid ${id}) in the chat's transcript (session_019YNMUVZR6ociHFWjByxkrq): the message's text from its first line that is "{" to its last line that is "}", both included, byte for byte, no byte added`,
  `the message whole: ${Buffer.byteLength(whole)} bytes, sha256 ${sha(whole)}; ${Buffer.byteLength(before)} bytes of his own words before the Export, ${Buffer.byteLength(after)} after it`,
  `format: ${JSON.stringify(doc.format)}`,
  `format_version: ${JSON.stringify(doc.format_version)}`,
  `build: ${JSON.stringify(doc.build)}`,
  `build_commit: ${JSON.stringify(doc.build_commit)}`,
  `camera_profile: ${JSON.stringify(doc.camera_profile)}`,
  `exported_utc: ${JSON.stringify(doc.exported_utc)}`,
  `values.simulation.step_rate_hz: ${JSON.stringify(doc.values.simulation.step_rate_hz)}`,
  `warnings (${warnings === null ? 'not a list' : warnings.length}): ${JSON.stringify(warnings)}`,
  `changed_from_start: ${changed === null ? 'not a list' : `${changed.length} entries`}`,
  `usable as it is (step rate ${GAME_STEP_RATE} and no warning): ${usable ? 'yes' : 'no: his answer comes first'}`,
  `numbers in it: ${[...now.values()].filter((value) => /^-?[0-9]/.test(value)).length} (of ${now.size} values)`,
  `against the Export at 60 steps (${EARLIER}, sha256 ${sha(earlierText)}): ${differing.length} of ${new Set([...now.keys(), ...then.keys()]).size} values differ`,
  ...differing.map((path) => `  differs: ${path}: ${then.get(path) ?? '(not there)'} -> ${now.get(path) ?? '(not there)'}`),
];
writeFileSync(NOTES + 'testlines-export-30.manifest.txt', lines.join('\n') + '\n');
console.log(lines.join('\n'));

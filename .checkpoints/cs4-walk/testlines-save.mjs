// Saves Mustafa's Testlines Export from the chat's transcript, whole and never retyped (his point 8b and his block of
// 2026-11-25, 17:44:41 UTC: "Save it whole by script with its byte count and sha256 before you read it").
// From the worktree's root:
//   node .checkpoints/cs4-walk/testlines-save.mjs <transcript.jsonl> [message uuid]
// It finds the one message of his that is an Export (a JSON document whose "format" is "testlines-export"), writes the
// message's text byte for byte to testlines-export.json, writes a manifest beside it, and prints what he asked to be
// told. With more than one such message in the transcript, the uuid says which. Node's built-in modules only.
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';
const NOTES = '.checkpoints/cs4-walk/';
const GAME_STEP_RATE = 30;
const [transcript, wanted] = process.argv.slice(2);
if (!transcript) { console.error('usage: testlines-save.mjs <transcript.jsonl> [message uuid]'); process.exit(2); }
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
    if (!t.trimStart().startsWith('{')) continue;
    let doc;
    try { doc = JSON.parse(t); } catch { continue; }
    if (doc && doc.format === 'testlines-export') hits.push({ t, doc, at: e.timestamp, id: e.uuid, how });
  }
}
const chosen = wanted ? hits.filter((h) => h.id === wanted) : hits;
if (chosen.length !== 1) { console.error(`expected one Export message, found ${chosen.length}${hits.length ? ` (${hits.map((h) => h.id).join(', ')})` : ''}`); process.exit(1); }
const { t, doc, at, id, how } = chosen[0];
const bytes = Buffer.byteLength(t);
const digest = sha(t);
writeFileSync(NOTES + 'testlines-export.json', t);
const rate = doc.values?.simulation?.step_rate_hz;
const warnings = Array.isArray(doc.warnings) ? doc.warnings : null;
const changed = Array.isArray(doc.changed_from_start) ? doc.changed_from_start : null;
const usable = rate === GAME_STEP_RATE && warnings !== null && warnings.length === 0;
const lines = [
  `file: ${NOTES}testlines-export.json`,
  `bytes: ${bytes}`,
  `sha256: ${digest}`,
  `source: ${how} of ${at} (uuid ${id}) in the chat's transcript (session_019YNMUVZR6ociHFWjByxkrq), whole, byte for byte`,
  `format: ${JSON.stringify(doc.format)}`,
  `format_version: ${JSON.stringify(doc.format_version)}`,
  `build: ${JSON.stringify(doc.build)}`,
  `build_commit: ${JSON.stringify(doc.build_commit)}`,
  `camera_profile: ${JSON.stringify(doc.camera_profile)}`,
  `exported_utc: ${JSON.stringify(doc.exported_utc)}`,
  `values.simulation.step_rate_hz: ${JSON.stringify(rate)}`,
  `warnings (${warnings === null ? 'not a list' : warnings.length}): ${JSON.stringify(warnings)}`,
  `changed_from_start: ${changed === null ? 'not a list' : `${changed.length} entries`}`,
  `usable as it is (step rate ${GAME_STEP_RATE} and no warning): ${usable ? 'yes' : 'no: his answer comes first'}`,
];
writeFileSync(NOTES + 'testlines-export.manifest.txt', lines.join('\n') + '\n');
console.log(lines.join('\n'));
if (changed) for (const c of changed) console.log(`  changed: ${c.group}.${c.name}: ${JSON.stringify(c.start)} -> ${JSON.stringify(c.current)}`);

// Writes the manifest beside his Export in content/source/ (step 8's second part; Builder V6, 2026-11-27). The
// design's manifest (cs4-walk/testlines-cs4.md, his point b): "the file's byte count and sha256, the message's time
// and id, and the Export's own build, build_commit, exported_utc and camera_profile". No line is typed: each is worked
// out from the content Export's own bytes, or, for the two lines only the chat's record could give (the message's
// time and id, the message whole), copied byte for byte from the manifest export-save.mjs wrote beside the Export's
// first copy in the notes (Builder V4's transcript is not on this machine). Every line worked out here must equal
// the notes manifest's line of the same name, or nothing is written. The notes manifest's comparison with the
// Export at 60 steps is left out: it compares two notes files, which leave the head with the notes.
//   node .checkpoints/cs4b-content/content-source-manifest.mjs <content Export> <notes manifest> <first commit> <out file>
// The two files read and the one written are full paths; <first commit> is the full id of the commit that added the
// Export's first copy to the notes (from git log, never typed). Node's built-in modules only.
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';
const [exportPath, notesManifestPath, firstCommit, out] = process.argv.slice(2);
for (const p of [exportPath, notesManifestPath, out]) {
  if (typeof p !== 'string' || !p.startsWith('/') || p.split('/').includes('..')) { console.error(`a full path is wanted: ${p}`); process.exit(2); }
}
if (!/^[0-9a-f]{40}$/.test(firstCommit ?? '')) { console.error(`<first commit> is a full commit id: ${firstCommit}`); process.exit(2); }
const text = readFileSync(exportPath, 'utf8');
const doc = JSON.parse(text);
const notes = readFileSync(notesManifestPath, 'utf8').split('\n');
const sha = createHash('sha256').update(text).digest('hex');
const warnings = Array.isArray(doc.warnings) ? doc.warnings : null;
const changed = Array.isArray(doc.changed_from_start) ? doc.changed_from_start : null;
const leaves = (value, out) => {
  if (value !== null && typeof value === 'object') for (const inner of Object.values(value)) leaves(inner, out);
  else out.push(JSON.stringify(value));
  return out;
};
const all = leaves(doc, []);
// The lines worked out from the Export itself, each of which the notes manifest must hold as it is.
const worked = [
  `bytes: ${Buffer.byteLength(text)}`,
  `sha256: ${sha}`,
  `format: ${JSON.stringify(doc.format)}`,
  `format_version: ${JSON.stringify(doc.format_version)}`,
  `build: ${JSON.stringify(doc.build)}`,
  `build_commit: ${JSON.stringify(doc.build_commit)}`,
  `camera_profile: ${JSON.stringify(doc.camera_profile)}`,
  `exported_utc: ${JSON.stringify(doc.exported_utc)}`,
  `values.simulation.step_rate_hz: ${JSON.stringify(doc.values?.simulation?.step_rate_hz)}`,
  `warnings (${warnings === null ? 'not a list' : warnings.length}): ${JSON.stringify(warnings)}`,
  `changed_from_start: ${changed === null ? 'not a list' : `${changed.length} entries`}`,
  `usable as it is (step rate 30 and no warning): ${doc.values?.simulation?.step_rate_hz === 30 && warnings !== null && warnings.length === 0 ? 'yes' : 'no: his answer comes first'}`,
  `numbers in it: ${all.filter((value) => /^-?[0-9]/.test(value)).length} (of ${all.length} values)`,
];
const missing = worked.filter((line) => !notes.includes(line));
if (missing.length > 0) { console.error(`the notes manifest does not hold these lines of the content Export:\n  ${missing.join('\n  ')}`); process.exit(1); }
// The two lines only the chat's record could give: each must stand once in the notes manifest.
const copied = ['source: ', 'the message whole: '].map((head) => {
  const found = notes.filter((line) => line.startsWith(head));
  if (found.length !== 1) { console.error(`the notes manifest holds ${found.length} lines that start with "${head}"`); process.exit(1); }
  return found[0];
});
const firstFile = notes[0].startsWith('file: ') ? notes[0].slice(6) : null;
if (firstFile === null) { console.error('the notes manifest does not start with its file line'); process.exit(1); }
const lines = [
  'file: content/source/testlines-export.json',
  worked[0],
  worked[1],
  `saved first: ${firstFile} (the notes of change-set 4b, commit ${firstCommit}), by the notes' export-save.mjs; moved here byte for byte`,
  ...copied,
  ...worked.slice(2),
];
writeFileSync(out, lines.join('\n') + '\n');
console.log(lines.join('\n'));

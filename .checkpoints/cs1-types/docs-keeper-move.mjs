// docs-keeper-cs1: the byte-for-byte moves of change-set 1's docs pass (PR #58), under Mustafa's OLD-LINES
// (2026-11-18, 11:08:25 UTC, docs/DECISIONS.md): lines about merged pull requests move by script, never reworded.
// Run from the repository root: node .checkpoints/cs1-types/docs-keeper-move.mjs [--write]
// Without --write it checks the anchors and prints the plan. It refuses to run when an anchor line does not match
// (for example on a second run, after the moves). Own words (the new rows, the pointer, the notes) are written after
// it with the Edit tool, not here.
import { readFileSync, writeFileSync } from 'node:fs';
import { createHash } from 'node:crypto';

const WRITE = process.argv.includes('--write');
const read = (f) => readFileSync(f, 'utf8').split('\n'); // a file ends with "\n": the last element is ""
const sha = (lines) => createHash('sha256').update(lines.join('\n')).digest('hex').slice(0, 16);
const fail = (msg) => { process.stderr.write(`refused: ${msg}\n`); process.exit(1); };
const at = (arr, n) => arr[n - 1]; // 1-based line number
const expectStart = (file, arr, n, prefix) => {
  if (!(at(arr, n) ?? '').startsWith(prefix)) fail(`${file}:${n} does not start with ${JSON.stringify(prefix)}`);
};
const expectEq = (file, arr, n, text) => {
  if (at(arr, n) !== text) fail(`${file}:${n} is not ${JSON.stringify(text)}`);
};

const S = 'docs/STATUS.md', A = 'docs/STATUS_ARCHIVE.md', R = 'docs/ROADMAP.md';
const st = read(S), ar = read(A), rm = read(R);

// Anchors: the exact lines this script was written against (main 22c6afa, branch head daf3d98).
if (st.length !== 121 || st[120] !== '') fail(`${S} is not 120 lines`);
expectStart(S, st, 28, '| #40 | the netcode proof test');
expectStart(S, st, 29, '| #57 | Part 15 of the build plan, "The second lane"');
expectEq(S, st, 112, '');
expectStart(S, st, 113, "## Where STATUS's old sections went (STATUS-DIET, 2026-11-10;");
expectEq(S, st, 114, '| STATUS at 2d22b69 (lines) | Now |');
expectStart(S, st, 120, '| Decisions pending: Waiting (847-849) |');
if (ar.length !== 1273 || ar[1272] !== '') fail(`${A} is not 1272 lines`);
expectEq(A, ar, 158, '|---|---|---|---|');
expectStart(A, ar, 159, '| **change-set 0 of the build plan**');
expectStart(A, ar, 1272, '| Gate (official battery, older) |');
if (rm.length !== 258 || rm[257] !== '') fail(`${R} is not 257 lines`);
expectStart(R, rm, 88, "- **VISION_V2's community-servers note and B3's hand-over [2026-11-06, MERGED]**");
expectStart(R, rm, 89, '- **UI theme step 1 (2026-10-02):**');
expectEq(R, rm, 240, '');
expectStart(R, rm, 241, '**Kit v3.4, the review policy [2026-11-07 to 11-09, MERGED]**');
expectEq(R, rm, 255, 'next change-set, kit-v3-5.');
expectEq(R, rm, 256, '');
expectStart(R, rm, 257, '**The VFX editor and private-match XP [2026-11-07, MERGED]**');

// The blocks, copied from the files as they are (1-based, inclusive).
const row57 = st.slice(28, 29); // STATUS 29
const table = st.slice(112, 120); // STATUS 113-120
const kit44 = rm.slice(240, 255); // ROADMAP 241-255
const vfx43 = rm.slice(256, 257); // ROADMAP 257

// New STATUS: line 29 and lines 113-120 removed (the blank line 112 stays; the pointer is written after this).
const st2 = [...st.slice(0, 28), ...st.slice(29, 112), ...st.slice(120)];
// New archive: #57's row first in "In flight" (newest first, before line 159); the table at the end, after a blank.
const ar2 = [...ar.slice(0, 158), ...row57, ...ar.slice(158, 1272), '', ...table, ...ar.slice(1272)];
// New ROADMAP: lines 240-257 removed; after line 88, in date order: blank, #43, blank, #44, blank.
const rm2 = [...rm.slice(0, 88), '', ...vfx43, '', ...kit44, '', ...rm.slice(88, 239), ...rm.slice(257)];

// Checks: every moved block equals its source byte for byte at its new place; nothing else changed.
const same = (a, b) => a.join('\n') === b.join('\n');
const checks = [
  ['STATUS 29 -> archive 159 (#57 row)', same(ar2.slice(158, 159), row57)],
  ['archive old 159-1272 -> 160-1273', same(ar2.slice(159, 1273), ar.slice(158, 1272))],
  ['STATUS 113-120 -> archive 1275-1282 (table)', same(ar2.slice(1274, 1282), table) && ar2[1273] === ''],
  ['ROADMAP 257 -> 90 (#43)', same(rm2.slice(89, 90), vfx43)],
  ['ROADMAP 241-255 -> 92-106 (#44)', same(rm2.slice(91, 106), kit44)],
  ['ROADMAP old 89-239 -> 108-258', same(rm2.slice(107, 258), rm.slice(88, 239))],
  ['STATUS no longer holds the moved lines', !st2.includes(row57[0]) && table.every((l) => !st2.includes(l))],
  ['ROADMAP no longer holds them twice', rm2.filter((l) => l === vfx43[0] || l === kit44[0]).length === 2],
  ['line counts', st2.length - 1 === 111 && ar2.length - 1 === 1282 && rm2.length - 1 === 258],
];
for (const [name, ok] of checks) process.stdout.write(`${ok ? 'ok  ' : 'FAIL'} ${name}\n`);
if (checks.some(([, ok]) => !ok)) fail('a check failed; nothing written');
process.stdout.write(`sha256/16 #57 row ${sha(row57)} · table ${sha(table)} · #44 ${sha(kit44)} · #43 ${sha(vfx43)}\n`);
process.stdout.write(`at the new places: ${sha(ar2.slice(158, 159))} · ${sha(ar2.slice(1274, 1282))} · ` +
  `${sha(rm2.slice(91, 106))} · ${sha(rm2.slice(89, 90))}\n`);
process.stdout.write(`lines: STATUS 120 -> 111, archive 1272 -> 1282, ROADMAP 257 -> 258\n`);
if (WRITE) {
  writeFileSync(S, st2.join('\n'));
  writeFileSync(A, ar2.join('\n'));
  writeFileSync(R, rm2.join('\n'));
  process.stdout.write('written\n');
} else process.stdout.write('dry run: nothing written (add --write)\n');

// docs-keeper-cs2: the byte-for-byte moves of change-set 2's docs pass (PR #61), under Mustafa's OLD-LINES
// (2026-11-18, 11:08:25 UTC, docs/DECISIONS.md) and REC-OLD (2026-11-18, 20:20:09 UTC): lines about merged pull
// requests move by script, never reworded. Adapted from the kit hold's script
// (git show f167f061a9b0b694836c272e397c90c815d7829f:.checkpoints/kit-hold-ci/docs-keeper-move.mjs).
// Run from the repository root: node .checkpoints/cs2-first-page/docs-keeper-move.mjs [--write]
// Without --write it checks the anchors and prints the plan. It refuses to run when an anchor line does not match
// (for example on a second run, after the moves). Own words are written after it with the Edit tool, except one
// line: the dated pointer that closes #58's zip question, whose quote of his words is copied here from the notes
// file (never retyped).
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
const Z = '.checkpoints/cs2-first-page/mustafa-2026-11-18-zip-saved.md';
const st = read(S), ar = read(A), rm = read(R), zp = read(Z);

// Anchors: the exact lines this script was written against (main b763ba3; the three files equal on main and HEAD).
if (st.length !== 121 || st[120] !== '') fail(`${S} is not 120 lines`);
expectStart(S, st, 28, '| #40 | the netcode proof test');
expectStart(S, st, 29, '| #60 | the kit hold and two CI edits, by lane 1, on his words of 2026-11-18, 15:38:13 UTC');
expectEq(S, st, 30, '');
expectEq(S, st, 31, '## Next');
if (ar.length !== 1409 || ar[1408] !== '') fail(`${A} is not 1408 lines`);
expectEq(A, ar, 157, '| Work | Branch | State | Waiting on |');
expectEq(A, ar, 158, '|---|---|---|---|');
expectStart(A, ar, 159, '| **change-set 1 of the build plan, Types** (foundation step 2.4), by lane 1:');
expectEq(A, ar, 976, '    three files, so both stay for the next pull request that does [verified 2026-11-18].');
expectStart(A, ar, 977, '- **From the kit hold and two CI edits (PR #60)** (in flight;');
if (rm.length !== 283 || rm[282] !== '') fail(`${R} is not 282 lines`);
expectEq(R, rm, 119, "go after the advisor's APPROVE WITH NOTES of 0573db3; the PR Advisor routine re-enabled at 16:07 UTC.");
expectEq(R, rm, 120, '');
expectStart(R, rm, 121, '- **UI theme step 1 (2026-10-02):**');
expectEq(R, rm, 274, '');
expectStart(R, rm, 275, '**STATUS-DIET with LANE-RULES [2026-11-10 to 11-11, MERGED]**');
expectEq(R, rm, 282, "Mustafa's go after the advisor's APPROVE of 2e6e5d2; CI green on main (run 42991918886).");
// The notes file: a header, a blank, the source line, a blank, "---", then his message (26 bytes) and a final "\n".
expectEq(Z, zp, 5, '---');
const words = zp[5];
if (zp.length !== 7 || zp[6] !== '' || Buffer.byteLength(words) !== 26) fail(`${Z}: his message is not one 26-byte last line`);
if (createHash('sha256').update(words).digest('hex') !== '6616a05dbc94b7bb3bdc9f82634556e844bf332378567a7e3308df8a88284585') {
  fail(`${Z}: his message's sha256 differs from the one the notes file records`);
}

// The blocks, copied from the files as they are (1-based, inclusive).
const row60 = st.slice(28, 29); // STATUS 29
const diet = rm.slice(274, 282); // ROADMAP 275-282 (STATUS-DIET, the oldest entry of "Recent work")
// The dated pointer (own words, with his words copied from the notes file), after archive line 976.
const zipLine = [
  `  - _Since (change-set 2's docs pass, 2026-11-19):_ the question whether #58's test zip saved and unpacked on his`,
  `    laptop (STATUS, "Waiting on Mustafa", until this pass) is answered, his message of 2026-11-18, 20:44:29 UTC, with`,
  `    a screenshot: "${words}" (copied by script into change-set 2's notes,`,
  '    `.checkpoints/cs2-first-page/mustafa-2026-11-18-zip-saved.md`, kept at the commit its report names). As the build',
  '    chat read his screenshot, the SHA256 PowerShell printed for `packages\\types\\src\\generated\\block\\layout.ts`',
  '    equals the expected one.',
];

// New STATUS: line 29 removed.
const st2 = [...st.slice(0, 28), ...st.slice(29)];
// New archive: #60's row first in "In flight" (newest first, before line 159); the pointer after old line 976.
const ar2 = [...ar.slice(0, 158), ...row60, ...ar.slice(158, 976), ...zipLine, ...ar.slice(976)];
// New ROADMAP: blank 274 and STATUS-DIET's 275-282 removed; after line 119 (#46's folded entry): blank, STATUS-DIET.
const rm2 = [...rm.slice(0, 119), '', ...diet, ...rm.slice(119, 273), ...rm.slice(282)];

// Checks: every moved block equals its source byte for byte at its new place; nothing else changed.
const same = (a, b) => a.join('\n') === b.join('\n');
const Z0 = 977 + zipLine.length; // first archive line after the pointer, 1-based
const checks = [
  ['STATUS 29 -> archive 159 (#60 row)', same(ar2.slice(158, 159), row60)],
  ['archive old 159-976 -> 160-977', same(ar2.slice(159, 977), ar.slice(158, 976))],
  ['pointer at archive 978-983 holds his words once', ar2.slice(977, 977 + zipLine.length).join('\n').split(`"${words}"`).length === 2],
  [`archive old 977-1408 -> ${Z0 + 1}-${1408 + 1 + zipLine.length}`, same(ar2.slice(Z0, 1409 + zipLine.length), ar.slice(976, 1408)) && ar2[1409 + zipLine.length] === ''],
  ['ROADMAP 275-282 -> 121-128 (STATUS-DIET)', same(rm2.slice(120, 128), diet) && rm2[119] === ''],
  ['ROADMAP old 1-119 kept', same(rm2.slice(0, 119), rm.slice(0, 119))],
  ['ROADMAP old 120-273 -> 129-282', same(rm2.slice(128, 282), rm.slice(119, 273)) && rm2[282] === ''],
  ['STATUS no longer holds the moved row', !st2.includes(row60[0])],
  ['ROADMAP holds STATUS-DIET once', rm2.filter((l) => l === diet[0]).length === 1],
  ['line counts', st2.length - 1 === 119 && ar2.length - 1 === 1409 + zipLine.length && rm2.length - 1 === 282],
];
for (const [name, ok] of checks) process.stdout.write(`${ok ? 'ok  ' : 'FAIL'} ${name}\n`);
if (checks.some(([, ok]) => !ok)) fail('a check failed; nothing written');
process.stdout.write(`sha256/16 #60 row ${sha(row60)} · STATUS-DIET ${sha(diet)}\n`);
process.stdout.write(`at the new places: ${sha(ar2.slice(158, 159))} · ${sha(rm2.slice(120, 128))}\n`);
process.stdout.write(`lines: STATUS 120 -> 119, archive 1408 -> ${1408 + 1 + zipLine.length}, ROADMAP 282 -> 282\n`);
if (WRITE) {
  writeFileSync(S, st2.join('\n'));
  writeFileSync(A, ar2.join('\n'));
  writeFileSync(R, rm2.join('\n'));
  process.stdout.write('written\n');
} else process.stdout.write('dry run: nothing written (add --write)\n');

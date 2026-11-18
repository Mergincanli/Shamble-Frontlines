// docs-keeper-khc: the byte-for-byte moves of the kit hold's docs pass (PR #60), under Mustafa's OLD-LINES
// (2026-11-18, 11:08:25 UTC, docs/DECISIONS.md): lines about merged pull requests move by script, never reworded.
// Adapted from .checkpoints/cs1-types/docs-keeper-move.mjs (git show 76df3f3:<that path>).
// Run from the repository root: node .checkpoints/kit-hold-ci/docs-keeper-move.mjs [--write]
// Without --write it checks the anchors and prints the plan. It refuses to run when an anchor line does not match
// (for example on a second run, after the moves). Own words (the new rows, the headers, the notes) are written after
// it with the Edit tool, not here.
// Besides the two moves it makes one COPY (not a move; the source stays): #58's report, "Follow-ups", its lines
// 234-268, each line indented by two spaces, into the archive's live "Follow-ups" (the text of #58's follow-ups,
// which its STATUS row named "for Follow-ups once merged"). The copy is listed apart from the moved lines.
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
const P = 'docs/reports/2026-11-18-cs1-types.md';
const st = read(S), ar = read(A), rm = read(R), rp = read(P);

// Anchors: the exact lines this script was written against (main 9b4b518; the four files equal on main and HEAD).
if (st.length !== 121 || st[120] !== '') fail(`${S} is not 120 lines`);
expectStart(S, st, 28, '| #40 | the netcode proof test');
expectStart(S, st, 29, '| #58 | change-set 1 of the build plan, Types (foundation step 2.4), by lane 1:');
expectEq(S, st, 119, '');
expectStart(S, st, 120, "## Where STATUS's old sections went: the table is at the end of");
if (ar.length !== 1286 || ar[1285] !== '') fail(`${A} is not 1285 lines`);
expectEq(A, ar, 157, '| Work | Branch | State | Waiting on |');
expectEq(A, ar, 158, '|---|---|---|---|');
expectStart(A, ar, 159, '| **Part 15 of the build plan, "The second lane", to main**');
expectEq(A, ar, 929, "    in it to the draft's next revision.");
expectEq(A, ar, 930, '');
expectEq(A, ar, 931, '## Decisions pending: the rest');
if (rm.length !== 271 || rm[270] !== '') fail(`${R} is not 270 lines`);
expectEq(R, rm, 106, 'next change-set, kit-v3-5.');
expectEq(R, rm, 107, '');
expectStart(R, rm, 108, '- **UI theme step 1 (2026-10-02):**');
expectStart(R, rm, 257, "Mustafa's go after the advisor's APPROVE of 2e6e5d2; CI green on main (run 42991918886).");
expectEq(R, rm, 258, '');
expectStart(R, rm, 259, "**Kit v3.5, the guard's rename gap [2026-11-09 to 11-10, MERGED]**");
expectStart(R, rm, 270, "go after the advisor's APPROVE WITH NOTES of 0573db3; the PR Advisor routine re-enabled at");
expectEq(P, rp, 233, '## Follow-ups');
expectStart(P, rp, 234, '- **CI-CUT** (security, Medium, in CI, outside this change-set;');
expectStart(P, rp, 267, "- **The drafts' next revision (his go):**");
expectEq(P, rp, 268, "  the ADR 0002 pointer on how ts-rs's folder is named.");
expectEq(P, rp, 269, '');

// The blocks, copied from the files as they are (1-based, inclusive).
const row58 = st.slice(28, 29); // STATUS 29
const kit46 = rm.slice(258, 270); // ROADMAP 259-270
const fu58 = rp.slice(233, 268); // report 234-268 (a copy)
const fu58In = fu58.map((l) => (l === '' ? l : `  ${l}`));

// New STATUS: line 29 removed.
const st2 = [...st.slice(0, 28), ...st.slice(29)];
// New archive: #58's row first in "In flight" (newest first, before line 159); the copy after old line 929.
const ar2 = [...ar.slice(0, 158), ...row58, ...ar.slice(158, 929), ...fu58In, ...ar.slice(929)];
// New ROADMAP: blank 258 and #46's 259-270 removed; after line 106 (#44's folded entry): blank, #46.
const rm2 = [...rm.slice(0, 106), '', ...kit46, ...rm.slice(106, 257), ...rm.slice(270)];

// Checks: every moved block equals its source byte for byte at its new place; nothing else changed.
const same = (a, b) => a.join('\n') === b.join('\n');
const checks = [
  ['STATUS 29 -> archive 159 (#58 row)', same(ar2.slice(158, 159), row58)],
  ['archive old 159-929 -> 160-930', same(ar2.slice(159, 930), ar.slice(158, 929))],
  ['report 234-268 -> archive 931-965 (copy, indented)', same(ar2.slice(930, 965).map((l) => l.replace(/^ {2}/, '')), fu58)],
  ['archive old 930-1285 -> 966-1321', same(ar2.slice(965, 1321), ar.slice(929, 1285)) && ar2[1321] === ''],
  ['ROADMAP 259-270 -> 108-119 (#46)', same(rm2.slice(107, 119), kit46) && rm2[106] === ''],
  ['ROADMAP old 1-106 kept', same(rm2.slice(0, 106), rm.slice(0, 106))],
  ['ROADMAP old 107-257 -> 120-270', same(rm2.slice(119, 270), rm.slice(106, 257)) && rm2[270] === ''],
  ['STATUS no longer holds the moved row', !st2.includes(row58[0])],
  ['ROADMAP holds #46 once', rm2.filter((l) => l === kit46[0]).length === 1],
  ['line counts', st2.length - 1 === 119 && ar2.length - 1 === 1321 && rm2.length - 1 === 270],
];
for (const [name, ok] of checks) process.stdout.write(`${ok ? 'ok  ' : 'FAIL'} ${name}\n`);
if (checks.some(([, ok]) => !ok)) fail('a check failed; nothing written');
process.stdout.write(`sha256/16 #58 row ${sha(row58)} · #46 ${sha(kit46)} · #58 follow-ups (report) ${sha(fu58)}\n`);
process.stdout.write(`at the new places: ${sha(ar2.slice(158, 159))} · ${sha(rm2.slice(107, 119))} · ` +
  `${sha(ar2.slice(930, 965).map((l) => l.replace(/^ {2}/, '')))} (copy, indent removed)\n`);
process.stdout.write(`lines: STATUS 120 -> 119, archive 1285 -> 1321, ROADMAP 270 -> 270\n`);
if (WRITE) {
  writeFileSync(S, st2.join('\n'));
  writeFileSync(A, ar2.join('\n'));
  writeFileSync(R, rm2.join('\n'));
  process.stdout.write('written\n');
} else process.stdout.write('dry run: nothing written (add --write)\n');

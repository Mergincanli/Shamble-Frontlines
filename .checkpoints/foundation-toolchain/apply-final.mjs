// The report's final pass before the verifier (the Fable advisor's whole read, consultation 8): evidence lines pinned
// to a commit, E2's later runs, E9's re-run, the consultation log, "For Mustafa", one recorded line, STATUS and ROADMAP.
// Every target must match exactly once, or nothing is written. Usage: node .checkpoints/foundation-toolchain/apply-final.mjs [--dry]
import fs from 'node:fs';

const dry = process.argv.includes('--dry');
const files = {};
const get = (f) => (files[f] ??= fs.readFileSync(f, 'utf8'));
const set = (f, t) => { files[f] = t; };
const once = (f, old, neu) => {
  const t = get(f); const n = t.split(old).length - 1;
  if (n !== 1) throw new Error(`${f}: expected 1 match, got ${n}: ${old.slice(0, 90)}`);
  set(f, t.replace(old, () => neu));
};
const wrap = (text, first, rest, width = 120) => {
  const w = text.replace(/\s+/g, ' ').trim().split(' '); const out = []; let cur = first;
  for (const x of w) {
    const sep = cur === first || cur === rest ? '' : ' ';
    if (cur.length + sep.length + x.length > width && cur.trim()) { out.push(cur); cur = rest + x; } else cur += sep + x;
  }
  out.push(cur); return out.join('\n');
};
const rewrap = (f, first, stop, firstPrefix, rest, edit) => {
  const L = get(f).split('\n'); const s = L.findIndex((l) => l.startsWith(first));
  if (s < 0 || L.findIndex((l, i) => i > s && l.startsWith(first)) >= 0) throw new Error(`${f}: start not unique: ${first.slice(0, 80)}`);
  let e = s + 1; while (e < L.length && !stop(L[e])) e++;
  const joined = L.slice(s, e).map((l, i) => (i === 0 ? l.slice(firstPrefix.length) : l.trim())).join(' ').replace(/\s+/g, ' ');
  L.splice(s, e - s, ...wrap(edit(joined), firstPrefix, rest).split('\n')); set(f, L.join('\n'));
};
const sub = (old, neu) => (s) => { const n = s.split(old).length - 1; if (n !== 1) throw new Error(`expected 1 match, got ${n}: ${old.slice(0, 90)}`); return s.replace(old, () => neu); };

const RP = 'docs/reports/2026-11-12-foundation-2-2.md';
// E1, E2, E9, E11.
once(RP, "Locally on 4bd9dd5's code (route A):", "Locally on 790ac31's code (route A; `git diff 4bd9dd5 790ac31 -- Cargo.toml crates` is comments only):");
once(RP, "on d5316d6 (43310606920, 43310617930) and on the report's commit 03b5a54 (43310933872, 43310945546).",
  "on d5316d6 (43310606920, 43310617930), on the report's commit 03b5a54 (43310933872, 43310945546), on 4d78ad6 (43312017234, 43312027194), on 1de98a9 (43342852176, 43342861226), on 4bd9dd5 (43414840093, 43414847203), on M7's revert 8d622ac (43415458569, 43415465726), on 8a4f9b0 (43415740925, 43415748074), on 4bccf55 (43424298848, 43424306336) and on round 2's fix 790ac31 (43444015165, 43444024411).");
once(RP, 'on the final file list, 42 files outside `.checkpoints/`:', "on the final file list (re-run on 790ac31's list, 2026-11-13: the same), 42 files outside `.checkpoints/`:");
once(RP, '`package-lock.json` v3: 69 entries = 64 registry packages', '`package-lock.json` v3: 75 entries, the root, the five workspace folders and 69 under `node_modules/` = 64 registry packages');
// The evidence note after the table.
rewrap(RP, 'M5 and M6 ran on the merged tree', (l) => l.trim() === '', '', '', sub(
  'which changes nothing built or tested, and the release overflow setting (4bd9dd5, route A), which none of M1a to M4 touches.',
  "which changes nothing built or tested, the release overflow setting (4bd9dd5, route A), which none of M1a to M4 touches, and round 2's two comments (790ac31: `Cargo.toml`'s and sim-wasm's), which change nothing built or tested."));
// As built: the whole read ran.
rewrap(RP, "- **The Fable advisor's full read of the built skeleton ran early**", (l) => !l.startsWith('  '), '- ', '  ', sub(
  'it runs again, whole, where his item 4 places it: "At your moment 5, before the PR is marked ready" (A9). Its changes:',
  'it ran again, whole, where his item 4 places it: "At your moment 5, before the PR is marked ready" (A9), on 790ac31 (consultation 8, 2026-11-13): the skeleton stands, and no file changed. The early read\'s changes:'));
// OVERFLOW-XCRATE-CI: one recorded line, in the report and the archive.
const REC = 'The overflow tests\' doc comments in `sim` and `protocol` say `cargo test --release` "proves the release profile\'s setting", which M7 shows only for code compiled in their own crates (the wording R2-L1 fixed in this report); recorded only, reworded with that test (the Fable advisor\'s whole read, consultation 8).';
once(RP, "2.3's plan adds a test-only dependency or asks the advisor agent to move the anchor); a STATUS follow-up |",
  "2.3's plan adds a test-only dependency or asks the advisor agent to move the anchor); a STATUS follow-up. " + REC + ' |');
rewrap('docs/STATUS_ARCHIVE.md', '    - **OVERFLOW-XCRATE-CI**', (l) => l.startsWith('    - **'), '    - ', '      ',
  sub("2.2's wasm32 build is dev-profile only (WASM-ARTIFACT).", "2.2's wasm32 build is dev-profile only (WASM-ARTIFACT). " + REC));
// For Mustafa, item 3: his merge accepts one open security finding.
rewrap(RP, '3. When the advisor agent has approved the final head', (l) => !l.startsWith('   '), '3. ', '   ', sub(
  'nothing a player sees, hears or feels changes.',
  'nothing a player sees, hears or feels changes. Your "merge #48" also accepts one security finding left open: the rule that keeps player input from causing an overflow stop (security-auditor\'s SA2-2), handed to the drafts\' owners as the follow-up OVERFLOW-PANIC-RULE, due with the first code in `room`.'));
// The consultation log, before "Decisions and deviations".
const LOG = [
  '## The advisor tool: consultations (his words of 2026-11-11, A7)',
  wrap("Fable 5.2 advised this session through Claude Code's advisor tool. Its advice cannot be read back, so each consultation is logged in one line: when (Amsterdam time), the question, what changed. The full log is in the checkpoint at the branch's last checkpoint commit (`.checkpoints/foundation-toolchain.md`, \"Advisor tool consultations\"). Count: 8.", '', ''),
  ...[
    "2026-11-11 ~13:45, before plan revision 8: what is missing from the plan for steps 5 to 7? Changed: his advisor-tool words kept for step 6 and 2.3; this log started; the five moments mapped onto steps 5 to 7; E4b, E4d and E8 to E10 named as not yet run.",
    "2026-11-12 ~16:40, moments 3 and 4 and an early read of the skeleton: are the four kit texts, the mode and the effort right? Changed: two greps before his ask; the writes in Accept edits, with his count of the guard prompts; step 7's agents at Opus 5.5, effort max; both his messages of 2026-11-12 in DECISIONS.md; S1 offered to him as K1b, S2 to S5 recorded.",
    "2026-11-12 ~19:58, moment 5 for step 5 and moment 1 for steps 6 and 7: is step 5 done, and are the docs pass and the review round planned right? Changed: As built updated; his messages counted a second way; the kit table with his words; the review run shaped (three agents at a time, skeptics, absolute paths); the checkpoint's deletion commit made by hand.",
    "2026-11-12 ~20:40, his item 4's full read of the skeleton before the reviewers: what to change before building on it? Changed: the overflow tests expect their panic's message (70f1c31); L2 planted as M6; Node resolving up from `tools/` recorded (TOOLKIT-RESOLVES-UP); the findings for 2.3 from that read.",
    "2026-11-12 ~22:35, moment 5 for step 6 and moment 1 for the review round: is step 6 done, and is the review run right? Changed: three stale report lines fixed; four DECISIONS.md entries pointed to his words on models of 2026-11-12; reviewers and skeptics as two runs with the session's dedupe between, a skeptic's verdict an input, never a filter.",
    "2026-11-13 ~00:20, moment 4 on RR-1 (High): fix now, or ask him to defer? Changed: no `Cargo.toml` edit until the advisor agent ruled (its ruling D1); route A's evidence (M7, E12, M5 relabelled historical); OVERFLOW-ALL put to him; review round 2.",
    "2026-11-13 ~14:42, moment 4 on round 2's fix pass: ask the advisor agent first, and are the drafts right? Changed: its ruling D1-R2 before any tracked file, and its home in this report; STATUS's additions measured against the 120-line cap; E12's flag line corrected; the note that R2-L2 had no skeptic; a search for the replaced phrases; this pass named the budget's last.",
    "2026-11-13 ~15:37, moment 5, his item 4: the whole read of the built skeleton before ready. Changed: no skeleton file (it stands); the report's final pass before the verifier (E2's later runs, E9's re-run, the lines pinned to a commit, STATUS's row for #48); the advisor agent's head-review brief names where the fold departs from D1-R2's texts; \"For Mustafa\" names the open security finding; the PR's title and body; one line recorded under OVERFLOW-XCRATE-CI.",
  ].map((t, i) => wrap(t, `${i + 1}. `, '   ')),
  '',
].join('\n');
once(RP, '\n## Decisions and deviations\n', '\n' + LOG + '\n## Decisions and deviations\n');
// STATUS: #48's row and ROADMAP's 2.2 entry move past the whole read.
once('docs/STATUS.md', 'OVERFLOW-PANIC-RULE open)', 'OVERFLOW-PANIC-RULE open; 790ac31, CI green); the Fable advisor\'s whole read of the skeleton (no file changed)');
once('docs/STATUS.md', "the push and CI on the new head; the Fable advisor's whole read again, before ready (his item 4 of 2026-11-12); the verifier; the report; ", 'the verifier; ready; ');
rewrap('docs/ROADMAP.md', "workspace crate on the advisor agent's ruling (RR-1, High;", (l) => l.trim() === '', '', '', sub(
  "Next: the Fable advisor's whole read, the verifier, the report, the advisor's review of the final head and his go.",
  "The Fable advisor's whole read before ready changed no file. Next: the verifier, the advisor's review of the final head and his go."));

if (dry) { console.log('dry run: every target matched once; files:', Object.keys(files).join(', ')); process.exit(0); }
for (const [f, t] of Object.entries(files)) fs.writeFileSync(f, t);
console.log('written:', Object.keys(files).join(', '));

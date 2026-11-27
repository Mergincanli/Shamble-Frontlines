// Fix-pass mutation trials of track.mjs (the main session's evidence after the pre-apply fixes). Runs in a scratch copy
// under .gate/rt-track-mut/ (gitignored), never the build copy. Usage: node .checkpoints/review-tracks/fix-mutate.mjs
// Prints each mutant: KILLED (a test failed), SURVIVED, or MISSING (its target text is not found exactly once).
import { copyFileSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
const B = '/home/user/Shamble-Frontlines/.gate/review-tracks/build', M = '/home/user/Shamble-Frontlines/.gate/rt-track-mut';
mkdirSync(`${M}/.claude/scripts`, { recursive: true }); mkdirSync(`${M}/docs`, { recursive: true });
copyFileSync(`${B}/.claude/scripts/track.test.mjs`, `${M}/.claude/scripts/track.test.mjs`);
copyFileSync(`${B}/docs/DECISIONS.md`, `${M}/docs/DECISIONS.md`);
const src = readFileSync(`${B}/.claude/scripts/track.mjs`, 'utf8');
const MUT = [
  ['quotes outside DECISIONS not listed', 'forMergeCheck.push(`${f.path}: a quote of him or a [DECIDED] line outside DECISIONS`);', 'void 0;'],
  ['spikes/ clause dropped', " || lp.startsWith('spikes/')", ''],
  ['read() arguments swapped', 'const t = read(saved[2], saved[1]);', 'const t = read(saved[1], saved[2]);'],
  ['eyes: drop packages/editor/', 'packages\\/editor\\/|crates\\/sim', 'crates\\/sim'],
  ['eyes: drop .html', '\\.(html|css)$', '\\.(css)$'],
  ['toolSource: drop workflows', " || lp.startsWith('.github/workflows/');", ';'],
  ['toolSource: drop package.json', 'const TOOL_SOURCE = (lp) => /(^|\\/)(package\\.json|', 'const TOOL_SOURCE = (lp) => /(^|\\/)('],
  ['go: design docs only look_v2', 'look_v2|art_direction|game_design|design_notes|vision_v2|loop_v2|editor_v2|units|v2_phase0_brief', 'look_v2'],
  ['go: drop v2_phase0_brief', '|units|v2_phase0_brief)', '|units)'],
  ['rule files: drop npm-shrinkwrap', 'package-lock\\.json|npm-shrinkwrap\\.json)$/.test(p);', 'package-lock\\.json)$/.test(p);'],
  ['full tie order reversed', 'PRIORITY.indexOf(x) - PRIORITY.indexOf(y)', 'PRIORITY.indexOf(y) - PRIORITY.indexOf(x)'],
  ['manifests not skipped in picks', 'if (MANIFEST.test(f.path.toLowerCase())) continue;', ''],
  ['light tie order reversed', '(a[0] < b[0] ? -1 : 1)', '(a[0] < b[0] ? 1 : -1)'],
  ['no saved file named becomes a floor', "else if (!saved) result = 'no saved file named';", "else if (!saved) { result = 'no saved file named'; floor = true; }"],
  ['no stated sha is not a floor', "if (!m) { result = 'no stated bytes and sha256'; floor = true; }", "if (!m) { result = 'no stated bytes and sha256'; }"],
  ['newline form dropped', '[quote, `${quote}\\n`].find(', '[quote].find('],
  ['saved file may equal either form', 'else if (t !== form)', 'else if (t !== quote && t !== `${quote}\\n`)'],
  ['changed entry not floored', 'else if (core(h) !== core(e)) out.push(', 'else if (false) out.push('],
  ['removed entry not floored', 'if (!h) out.push(', 'if (false) out.push('],
  ['doubled first line not floored', 'else if (count.get(firstLine(e)) > 1) out.push(', 'else if (false) out.push('],
  ['otherAdded reads the whole entry', '.map(core).join', '.join'],
  ['otherAdded counts blank lines', "l.slice(1).trim() !== '' && ", ''],
  ['otherAdded not listed', 'if (otherAdded(ht, bt, dec.added)) forMergeCheck.push(', 'if (false) forMergeCheck.push('],
  ['twins outside the change', 'g.some((t) => changed.has(t))', 'true'],
  ['twins not used', 'files.push(...twins(', '[].push(...twins('],
  ['addedLines drops ++ lines', ".filter((l) => l.startsWith('+')).join('\\n');", ".filter((l) => l.startsWith('+') && !l.startsWith('+++')).join('\\n');"],
  ['ignore rules not read', 'const ignored = ignoredByBase(base, files.map((f) => f.path));', 'const ignored = new Set();'],
  ['check-ignore failure read as not ignored', 'if (e.status === 1) return new Set(); stop(', 'return new Set(); stop('],
  ['check-ignore with literal pathspecs', 'env: { ...ENV, GIT_DIR: gitDir, GIT_WORK_TREE: tmp }', "env: { ...ENV, GIT_DIR: gitDir, GIT_WORK_TREE: tmp, GIT_LITERAL_PATHSPECS: '1' }"],
  ['diff without --no-color', "'--no-renames', '--no-color', ", "'--no-renames', "],
  ['diff without --no-ext-diff', "'--no-ext-diff', ", ''],
  ['diff without --ignore-submodules=none', ", '--ignore-submodules=none'];", '];'],
  ['not run from the top folder', "try { TOP = git(['rev-parse', '--show-toplevel']).trim(); }", "try { git(['rev-parse', '--show-toplevel']); }"],
  ['GIT_* kept', "filter(([k]) => !/^GIT_/.test(k))", 'filter(() => true)'],
  ['DECISIONS not changed not said', "words = 'DECISIONS not changed';", ''],
  ['sha not listed', '${w.sha ? ` (sha256 ${w.sha})` : \'\'}', ''],
  ['--files picks not marked provisional', "if (argv[0] === '--files') c.reviewers.provisional = ", "if (false) c.reviewers.provisional = "],
  ['text triggers on every path', 'SEC_TEXT_PATH.test(f.path) && ', ''],
  ['XMLHttpRequest dropped', '|XMLHttpRequest', ''],
  ['WebSocket dropped', '|WebSocket', ''],
  ['.rs outside crates not owned', "!f.path.startsWith('crates/') && f.path.toLowerCase().endsWith('.rs')", 'false'],
  ['raw and numstat consistency not checked', "if (files.length !== meta.size || files.some((f) => !meta.has(f.path))) stop(", 'if (false) stop('],
  ['a tab cuts the path', "const a = n.slice(0, i), d = n.slice(i + 1, j), path = n.slice(j + 1);", "const [a, d, path] = n.split('\\t');"],
  ['binary flag never set', "binary: a === '-'", 'binary: false'],
  ['mode never set', 'mode: meta.get(path)', 'mode: undefined'],
  ['words floors not pushed', 'if (w.floor) c.goFloors.push(', 'if (false) c.goFloors.push('],
  ['TOOL-SOURCE prompt dropped', 'if (c.toolSource.length) forMergeCheck.push(', 'if (false) forMergeCheck.push('],
  ['allowed set takes ~', '/[^A-Za-z0-9_./-]/', '/[^A-Za-z0-9_./~-]/'],
  ['empty classify is light', "full || !files.length ? 'full'", "full ? 'full'"],
  ['entry id cut short', 'message ([0-9a-f-]{8,36})', 'message ([0-9a-f]{8})'],
  ['twins list without --full-tree', "git(['ls-tree', '-r', '--full-tree', '--name-only'", "git(['ls-tree', '-r', '--name-only'"],
];
let killed = 0, survived = [], missing = [];
for (const [label, from, to] of MUT) {
  if (src.split(from).length !== 2) { missing.push(label); console.log(`MISSING   ${label}`); continue; }
  writeFileSync(`${M}/.claude/scripts/track.mjs`, src.replace(from, to));
  const r = spawnSync(process.execPath, ['--test', '.claude/scripts/track.test.mjs'], { cwd: M, encoding: 'utf8' });
  const ok = r.status === 0;
  if (ok) survived.push(label); else killed++;
  console.log(`${ok ? 'SURVIVED' : 'KILLED  '}  ${label}`);
}
console.log(`\n${MUT.length} mutants: ${killed} killed, ${survived.length} survived, ${missing.length} missing`);
if (survived.length) console.log(`survived: ${survived.join('; ')}`);

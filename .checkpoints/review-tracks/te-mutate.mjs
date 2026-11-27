// test-engineer scratch: mutation trials of track.mjs in a scratch copy under .gate/te-track-mut/ (never the build copy).
// Usage: node te-mutate.mjs. Prints each mutant: KILLED (a test failed) or SURVIVED.
import { mkdirSync, copyFileSync, readFileSync, writeFileSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
const R = '/home/user/Shamble-Frontlines', B = `${R}/.gate/review-tracks/build`, M = `${R}/.gate/te-track-mut`;
mkdirSync(`${M}/.claude/scripts`, { recursive: true }); mkdirSync(`${M}/docs`, { recursive: true });
copyFileSync(`${B}/.claude/scripts/track.test.mjs`, `${M}/.claude/scripts/track.test.mjs`);
copyFileSync(`${B}/docs/DECISIONS.md`, `${M}/docs/DECISIONS.md`);
const src = readFileSync(`${B}/.claude/scripts/track.mjs`, 'utf8');
const MUT = [
  ["main: quotes outside DECISIONS not listed (valid)", "forMergeCheck.push(`${f.path}: a quote of him or a [DECIDED] line outside DECISIONS`);", "void 0;"],
  ["full: spikes/ clause dropped", " || lp.startsWith('spikes/')", ""],
  ["words: read() arguments swapped", "read(saved[2], saved[1])", "read(saved[1], saved[2])"],
  ['eyes: drop packages/editor/', 'packages\\/editor\\/|crates\\/sim', 'crates\\/sim'],
  ['eyes: drop .html', '\\.(html|css)$', '\\.(css)$'],
  ['toolSource: drop .github/workflows/', " || lp.startsWith('.github/workflows/')", ''],
  ['toolSource: drop package.json', "const TOOL_SOURCE = (lp) => /(^|\\/)(package\\.json|", "const TOOL_SOURCE = (lp) => /(^|\\/)("],
  ['go: design docs only look_v2', 'look_v2|art_direction|game_design|design_notes|vision_v2|loop_v2|editor_v2|units', 'look_v2'],
  ['rule files: drop npm-shrinkwrap', 'package-lock\\.json|npm-shrinkwrap\\.json)$/.test(p);', 'package-lock\\.json)$/.test(p);'],
  ['rule files: drop claude.local.md', 'claude\\.md|claude\\.local\\.md|', 'claude\\.md|'],
  ['full tie order reversed', 'PRIORITY.indexOf(x) - PRIORITY.indexOf(y)', 'PRIORITY.indexOf(y) - PRIORITY.indexOf(x)'],
  ['manifests not skipped in picks', 'if (MANIFEST.test(f.path.toLowerCase())) continue;', ''],
  ['light tie order reversed', "(a[0] < b[0] ? -1 : 1)", "(a[0] < b[0] ? 1 : -1)"],
  ['words: no saved file -> ok', "else if (!saved) result = 'no saved file named';", "else if (!saved) result = 'ok';"],
  ['words: no header -> ok', "if (!m) result = 'no stated bytes and sha256';", "if (!m) result = 'ok';"],
  ['words: byte count not checked', 'Buffer.byteLength(quote) !== Number(m[1]) || ', ''],
  ['main: ignored never set', 'ignored = true; }', 'ignored = false; }'],
  ['main: binary never set', 'binary: a === \'-\', mode', 'binary: false, mode'],
  ['main: modes never read', 'mode: meta.get(path),', 'mode: undefined,'],
  ['main: tree twin scan removed', 'if (seen.has(l) && !files.some((f) => f.path === t)) files.push({ path: t, lines: 0 });', ''],
  ['main: words floors not pushed', "c.goFloors.push(`docs/DECISIONS.md entry ${w.entry}: ${w.result}`);", ''],
  ['main: quotes outside DECISIONS not listed', "forMergeCheck.push(`${f.path}: a quote of him or a [DECIDED] line outside DECISIONS`);", ''],
  ['main: TOOL-SOURCE prompt dropped', "if (c.toolSource.length) forMergeCheck.push(", "if (false) forMergeCheck.push("],
  ['main: base from origin/main^ (a wrong base)', "['merge-base', 'origin/main', head]", "['merge-base', 'origin/main~1', head]"],
  ['sec trigger: drop WebSocket', '|WebSocket|', '|'],
  ['floor: allowed set gains ~', "/[^A-Za-z0-9_./-]/.test(p)", "/[^A-Za-z0-9_./~-]/.test(p)"],
  ['floor: binary not a floor', " : f.binary ? 'a binary file' : null", ' : null'],
  ['full: test-engineer dropped from picks', "picks: ['test-engineer', ...picks] };", 'picks };'],
  ['empty classify is light', "track: full || !files.length ? 'full' : 'light'", "track: full ? 'full' : 'light'"],
  ['words: id regex never matches', 'message ([0-9a-f-]{8,36})', 'messageX ([0-9a-f-]{8,36})'],
];
for (const [label, from, to] of MUT) {
  const n = src.split(from).length - 1;
  if (n !== 1) { console.log(`BADTARGET (${n}) ${label}`); continue; }
  writeFileSync(`${M}/.claude/scripts/track.mjs`, src.replace(from, to));
  const r = spawnSync(process.execPath, ['--test', `${M}/.claude/scripts/track.test.mjs`], { cwd: R, encoding: 'utf8' });
  console.log(`${r.status === 0 ? 'SURVIVED' : 'KILLED  '} ${label}`);
}
writeFileSync(`${M}/.claude/scripts/track.mjs`, src);
const base = spawnSync(process.execPath, ['--test', `${M}/.claude/scripts/track.test.mjs`], { cwd: R, encoding: 'utf8' });
console.log(`unmutated: exit ${base.status}`);

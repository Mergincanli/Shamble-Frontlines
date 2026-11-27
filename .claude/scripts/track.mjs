#!/usr/bin/env node
// track.mjs: the review track of a change, worked out from the files it touches, never picked by a person or a chat
// (Mustafa's review tracks, 2026-11-25, and his decisions of 2026-11-26; his words in docs/DECISIONS.md; the rules in
// CLAUDE.md "Review tracks" and advisor.md "His go keeps"). In doubt it is the full track.
//   node <main's copy>/.claude/scripts/track.mjs <head>        a branch or commit, against its merge base with this
//                                                                clone's own refs/remotes/origin/main (fetch it first)
//   node <main's copy>/.claude/scripts/track.mjs --files a b   at plan time, the planned paths (names only)
// Main's copy always decides: a change's own copy never classifies that change. Prints one JSON object (keys in a fixed
// order, paths sorted); exit 2 with a reason on stderr when there is no merge base, the diff is empty, or git fails.
// Deterministic: git only, no clock, no network; the same result from any folder of the repository, with GIT_*
// variables dropped, and under the diff settings that could change it (colour, prefixes, an external diff, text
// conversion, hidden submodules, the diff algorithm); the ignore rules are the base's alone, read in a bare temporary
// repository with no excludes file.
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { mkdtempSync, mkdirSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join, sep } from 'node:path';
import { pathToFileURL } from 'node:url';

const RULE_FILES = (p) => /(^|\/)(claude\.md|claude\.local\.md|agents\.md|\.npmrc)$/.test(p) || /(^|\/)\.claude\//.test(p) ||
  p.startsWith('.github/') || /^tools\/(gate\.mjs|smoke\.mjs|battery\.js|gate-globals\.json|package\.json|package-lock\.json|npm-shrinkwrap\.json)$/.test(p);
// Full rules compare in lower case (lp); light rules compare exactly (p). The first match wins.
const FULL = [
  ['rule files (risk tier 2)', RULE_FILES],
  ['anti-cheat', (lp) => /(^|\/)[^/]*anti-?cheat/.test(lp)],
  ['netcode', (lp) => /^(crates\/(protocol|room|client-endpoint|sim-wasm)|packages\/(net|sim-worker))\//.test(lp)],
  ['server code', (lp) => /^crates\/(server|supervisor)\//.test(lp)],
  ['Rust core', (lp) => lp.startsWith('crates/') || /^(cargo\.toml|cargo\.lock|rust-toolchain(\.toml)?)$/.test(lp) || lp.endsWith('.rs')],
  ['in doubt: the Rust boundary types', (lp) => lp.startsWith('packages/types/')],
  ['in doubt: serving and build setup', (lp) => /^packages\/client\/(scripts\/|serve\.mjs$|serve\.test\.mjs$)/.test(lp)],
  ['in doubt: a manifest or lockfile', (lp) => /(^|\/)(package\.json|package-lock\.json|npm-shrinkwrap\.json|cargo\.toml|cargo\.lock|rust-toolchain(\.toml)?|yarn\.lock|pnpm-lock\.yaml)$/.test(lp)],
  ['in doubt: decides what CI runs', (lp) => /(^|\/)(playwright\.config\.[^/]+|tsconfig[^/]*\.json|vite\.config\.[^/]+|vitest\.config\.[^/]+)$/.test(lp)],
  ['in doubt: a dotfile or spike', (lp) => /(^|\/)\.[^/]+/.test(lp) || lp.startsWith('spikes/')],
];
const LIGHT = [/^docs\//, /^README\.md$/, /^FIRST_PROMPTS\.md$/, /^tools\//, /^legacy\//, /^packages\/client\//, /^packages\/e2e\//, /^packages\/editor\//];
// "His go keeps" items a path alone shows (advisor.md); the others stay judged by the reviewers and his Advisor.
const GO = [
  ['rule files, secrets or infrastructure (risk tier 2)', (lp) => RULE_FILES(lp)],
  ['content/source/ (his SOURCE-GO (a))', (lp) => lp.startsWith('content/source/')],
  ['an ADR or the register (a decision of his)', (lp) => lp.startsWith('docs/adr/')],
  ['a design or look doc (a decision of his)', (lp) => /^docs\/(look_v2|art_direction|game_design|design_notes|vision_v2|loop_v2|editor_v2|units|v2_phase0_brief)\.md$/.test(lp)],
];
const EYES = (lp) => /^(legacy\/|packages\/client\/src\/|packages\/editor\/|crates\/sim\/|packages\/sim-worker\/|content\/)/.test(lp) || /\.(html|css)$/.test(lp);
const TOOL_SOURCE = (lp) => /(^|\/)(package\.json|package-lock\.json|npm-shrinkwrap\.json|cargo\.toml|cargo\.lock|rust-toolchain(\.toml)?)$/.test(lp) || lp.startsWith('.github/workflows/');
const SEC_TRIGGER = /innerHTML|outerHTML|insertAdjacentHTML|document\.write|\beval\(|new Function|localStorage|sessionStorage|indexedDB|\bfetch\(|WebSocket|WebTransport|XMLHttpRequest|postMessage|Content-Security-Policy/;
const SEC_TEXT_PATH = /^(packages|tools|legacy)\//; // the text triggers count only there (plan 3b)
const SEC_PATH = ['crates/protocol/', 'crates/room/', 'crates/server/', 'crates/supervisor/', 'packages/net/', 'packages/client/scripts/', 'packages/client/vite.config.mjs', 'packages/client/serve.mjs'];
const OWNER = [
  ['packages/client/scripts/', 'security-auditor'], ['packages/client/vite.config.mjs', 'security-auditor'], ['packages/client/serve.mjs', 'security-auditor'],
  ['packages/client/serve.test.mjs', 'security-auditor'], ['packages/client/src/render/', 'graphics-engineer'], ['packages/client/', 'perf-auditor'],
  ['packages/net/', 'netcode-architect'], ['packages/sim-worker/', 'engine-architect'], ['packages/types/', 'engine-architect'], ['packages/e2e/', 'test-engineer'],
  ['packages/', 'engine-architect'], ['crates/protocol/', 'netcode-architect'], ['crates/room/', 'netcode-architect'], ['crates/client-endpoint/', 'netcode-architect'],
  ['crates/server/', 'netcode-architect'], ['crates/supervisor/', 'infra-engineer'], ['crates/sim-wasm/', 'engine-architect'], ['crates/', 'rust-reviewer'],
  ['tools/', 'test-engineer'], ['legacy/', 'test-engineer'],
];
const PRIORITY = ['netcode-architect', 'security-auditor', 'rust-reviewer', 'engine-architect', 'perf-auditor', 'graphics-engineer', 'infra-engineer'];
const MANIFEST = /(^|\/)(cargo\.toml|cargo\.lock|package\.json|package-lock\.json|npm-shrinkwrap\.json)$/;

// files: [{ path, lines, mode, binary, ignored, added }] (added: the change's added text, for the security triggers).
export function classify(files) {
  const why = [], goFloors = [], eyesPaths = [], toolSource = [];
  const lower = new Map();
  for (const f of files) { const lp = f.path.toLowerCase(); lower.set(lp, (lower.get(lp) || 0) + 1); }
  let full = false;
  for (const f of [...files].sort((a, b) => (a.path < b.path ? -1 : 1))) {
    const p = f.path, lp = p.toLowerCase();
    const floor = f.ignored ? 'a gitignored path' : f.mode === '120000' ? 'a symlink' : f.mode === '160000' ? 'a submodule'
      : f.twin || lower.get(lp) > 1 ? 'a case-only twin' : /[^A-Za-z0-9_./-]/.test(p) ? 'a character outside the allowed set' : f.binary ? 'a binary file' : null;
    const hit = floor ? [`floor: ${floor}`] : FULL.find(([, t]) => t(lp));
    if (floor) goFloors.push(`${p}: ${floor} (risk tier 2)`);
    const rule = hit ? hit[0] : LIGHT.some((r) => r.test(p)) ? 'light' : 'in no list';
    if (rule !== 'light') full = true;
    why.push(`${p}: ${rule === 'light' ? 'light (fixed list)' : `full (${rule})`}`);
    for (const [g, t] of GO) if (t(lp)) goFloors.push(`${p}: ${g}`);
    if (EYES(lp)) eyesPaths.push(p);
    if (TOOL_SOURCE(lp)) toolSource.push(p);
  }
  return { track: full || !files.length ? 'full' : 'light', why, goFloors, eyesPaths, toolSource, reviewers: pick(files, full) };
}

export function pick(files, full) {
  const score = Object.fromEntries(PRIORITY.map((r) => [r, 0])), light = {};
  let sec = false;
  for (const f of files) {
    if (SEC_PATH.some((s) => f.path.startsWith(s)) || f.path === 'packages/client/index.html' || (f.added && SEC_TEXT_PATH.test(f.path) && SEC_TRIGGER.test(f.added))) sec = true;
    if (MANIFEST.test(f.path.toLowerCase())) continue;
    // a .rs file outside crates/ (a spike's) counts for rust-reviewer (inputs-B R5)
    const own = !f.path.startsWith('crates/') && f.path.toLowerCase().endsWith('.rs') ? ['', 'rust-reviewer'] : OWNER.find(([p]) => f.path.startsWith(p));
    if (!own) continue;
    light[own[1]] = (light[own[1]] || 0) + f.lines;
    if (own[1] in score) score[own[1]] += f.lines;
  }
  if (files.some((f) => RULE_FILES(f.path.toLowerCase()))) return { rule: 'risk tier 2: the advisor agent and security-auditor', picks: ['advisor', 'security-auditor'] };
  if (full) {
    const ranked = PRIORITY.filter((r) => score[r] > 0).sort((x, y) => score[y] - score[x] || PRIORITY.indexOf(x) - PRIORITY.indexOf(y));
    const picks = sec ? ['security-auditor'] : [];
    for (const r of ranked) { if (picks.length === 2) break; if (!picks.includes(r)) picks.push(r); }
    return { rule: 'full: test-engineer plus at most two (security slot first, then by lines owned)', score, picks: ['test-engineer', ...picks] };
  }
  const best = Object.entries(light).sort((a, b) => b[1] - a[1] || (a[0] < b[0] ? -1 : 1))[0];
  const one = best ? best[0] : 'test-engineer';
  return { rule: 'light: the one reviewer owning most changed lines (art-director if the plan\'s change type is visual; none at risk tier 0)', picks: sec && one !== 'security-auditor' ? [one, 'security-auditor'] : [one] };
}

// The copied-words check on docs/DECISIONS.md: each added entry's quote must match its stated bytes and sha256 (of the
// quote, or of the quote plus one final newline: both forms are in the record) and its named saved file at its commit,
// which must equal the form that matched, or be the record's third form: its header (a "# " title, a blank line, one
// "_Copied by script..._" line, a blank line), a line "---", then the quote. An existing entry whose header or quote
// changes, or that is removed, is a go floor. An integrity check, never proof that he typed it: every added entry also
// goes to his Advisor's merge check.
// read(commit, path) returns the saved file's text, or null when it cannot be read. floor: true puts it in goFloors.
const ENTRY = /\n(?=- \*\*)/;
const isEntry = (e) => e.startsWith('- **Taken by Mustafa');
const firstLine = (e) => e.split('\n')[0];
// An entry's own text: its header (its first line and the lines after it up to the first quote line, ending at a blank
// line or a heading) and its quote (the run of quote lines that follows). Notes after the quote are not part of it.
const parts = (e) => {
  const l = e.split('\n'); let k = 1;
  while (k < l.length && !/^>( |$)/.test(l[k]) && l[k].trim() !== '' && !/^#/.test(l[k])) k++;
  let q = k; while (q < l.length && /^>( |$)/.test(l[q])) q++;
  return { header: l.slice(0, k), quote: l.slice(k, q) };
};
const quoteOf = (e) => parts(e).quote.map((l) => l.replace(/^> ?/, '')).join('\n');
const core = (e) => { const { header, quote } = parts(e); return [...header, ...quote].join('\n'); };
// for an existing entry: its header and every quote line in it (a quote can go on after a "[left out]" note)
const kept = (e) => [...parts(e).header, ...e.split('\n').filter((l) => /^>( |$)/.test(l))].join('\n');
const idOf = (e) => (e.replace(/\n\s*/g, ' ').match(/message ([0-9a-f-]{8,36})/) || [, e.slice(4, 60)])[1];
const sha256 = (t) => createHash('sha256').update(t).digest('hex');
export function wordsCheck(headText, baseText, read) {
  const head = headText.split(ENTRY), base = baseText.split(ENTRY), out = [];
  const count = new Map(); for (const e of head) count.set(firstLine(e), (count.get(firstLine(e)) || 0) + 1);
  const headByFirst = new Map(head.map((e) => [firstLine(e), e]));
  for (const e of base.filter(isEntry)) {
    const h = headByFirst.get(firstLine(e));
    if (!h) out.push({ entry: idOf(e), sha: null, floor: true, result: 'an existing entry was removed or its first line changed' });
    else if (count.get(firstLine(e)) > 1) out.push({ entry: idOf(e), sha: null, floor: true, result: 'two entries share this first line' });
    else if (kept(h) !== kept(e)) out.push({ entry: idOf(e), sha: null, floor: true, result: "an existing entry's header or quote was changed" });
  }
  const before = new Set(base.map(firstLine));
  for (const e of head.filter(isEntry)) {
    if (before.has(firstLine(e))) continue;
    const flat = e.replace(/\n\s*/g, ' ');
    const m = flat.match(/Copied by script \((\d+) bytes, sha256 ([0-9a-f]{64})\):/);
    const quote = quoteOf(e);
    const saved = flat.match(/saved file `([^`]+)` \(readable at commit ([0-9a-f]{7,40})(?: [^)]*)?\)/);
    const form = m ? [quote, `${quote}\n`].find((q) => Buffer.byteLength(q) === Number(m[1]) && sha256(q) === m[2]) : undefined;
    let result = 'ok', floor = false;
    if (!m) { result = 'no stated bytes and sha256'; floor = true; }
    else if (form === undefined) { result = 'quote does not match its bytes and sha256'; floor = true; }
    else if (!saved) result = 'no saved file named'; // plan 3f compares only when the header names one: listed, not a floor
    else {
      const t = read(saved[2], saved[1]), i = t === null ? -1 : t.indexOf('\n---\n');
      const body = i >= 0 && /^# [^\n]+\n\n_Copied by script[^\n]*_\n$/.test(t.slice(0, i)) ? t.slice(i + 5) : null;
      if (t === null) { result = 'saved file cannot be read'; floor = true; }
      else if (t !== form && body !== quote && body !== `${quote}\n`) { result = 'saved file differs'; floor = true; }
    }
    out.push({ entry: idOf(e), sha: m ? m[2] : null, floor, result });
  }
  return out;
}

// True when DECISIONS gains a line outside a newly copied entry's header and quote (listed for the merge check, plan 3f).
// added: the diff's added lines, each starting with '+'.
export function otherAdded(headText, baseText, added) {
  const before = new Set(baseText.split(ENTRY).map(firstLine));
  const fresh = headText.split(ENTRY).filter((e) => isEntry(e) && !before.has(firstLine(e))).map(core).join('\n').split('\n');
  return added.split('\n').some((l) => l.startsWith('+') && l.slice(1).trim() !== '' && !fresh.includes(l.slice(1)));
}

// A case-only twin (plan 3a): a changed path that, or one of whose folders, differs only in case from another path or
// folder in the head's tree. Only the change's own paths are marked; no other path joins the list.
export function twinPaths(tree, changed) {
  const prefixes = (p) => p.split('/').map((_, i, a) => a.slice(0, i + 1).join('/'));
  const spellings = new Map();
  for (const t of tree) for (const q of prefixes(t)) { const l = q.toLowerCase(); if (!spellings.has(l)) spellings.set(l, new Set()); spellings.get(l).add(q); }
  return changed.filter((p) => prefixes(p).some((q) => (spellings.get(q.toLowerCase())?.size ?? 0) > 1));
}

// Added lines of one file's diff: the '+' lines after the first hunk header (a '+++' header never counts; an added line
// that starts with '++' does).
export function addedLines(diff) {
  const k = diff.indexOf('\n@@');
  return k < 0 ? '' : diff.slice(k + 1).split('\n').filter((l) => l.startsWith('+')).join('\n');
}

const ENV = Object.fromEntries(Object.entries(process.env).filter(([k]) => !/^GIT_/.test(k)));
const DIFF = ['--no-renames', '--no-color', '--no-ext-diff', '--no-textconv', '--ignore-submodules=none', '--diff-algorithm=myers'];
let TOP; // the top folder; every git call runs there
function git(args, opts = {}) {
  return execFileSync('git', args, { cwd: TOP, encoding: 'utf8', maxBuffer: 1 << 28, stdio: ['ignore', 'pipe', 'pipe'], ...opts, env: { ...ENV, GIT_LITERAL_PATHSPECS: '1', GIT_NO_REPLACE_OBJECTS: '1', GIT_GRAFT_FILE: '/dev/null', ...opts.env } });
}
function stop(why) { process.stderr.write(`track.mjs: ${why}\n`); process.exit(2); }

// The paths the base's ignore rules ignore: the base's .gitignore files are written into a new temporary repository (no
// template, no excludes file, no global or system config, case-sensitive) and git check-ignore reads them there, never
// the checked-out branch's rules or this machine's excludes. Exit 0 or 1 is an answer; anything else stops. check-ignore
// refuses literal pathspecs, so its calls run without them.
function ignoredByBase(base, paths) {
  const tmp = mkdtempSync(join(tmpdir(), 'track-ignore-'));
  const env = { ...ENV, GIT_CONFIG_NOSYSTEM: '1', GIT_CONFIG_GLOBAL: '/dev/null', GIT_NO_REPLACE_OBJECTS: '1' };
  const tg = (args, opts = {}) => execFileSync('git', args, { cwd: tmp, encoding: 'utf8', maxBuffer: 1 << 28, stdio: ['ignore', 'pipe', 'pipe'], env, ...opts });
  let out = null, why = null; // the temporary folder is removed before any stop
  try {
    tg(['init', '-q', '--template=']); tg(['config', 'core.excludesFile', '/dev/null']); tg(['config', 'core.ignorecase', 'false']);
    for (const l of git(['ls-tree', '-r', '--full-tree', '-z', base]).split('\0')) {
      const p = l.slice(l.indexOf('\t') + 1);
      if (!/^100(644|755) blob /.test(l) || !/(^|\/)\.gitignore$/.test(p)) continue;
      if (p.split('/').some((c) => ['', '.', '..', '.git'].includes(c))) { why = `an ignore file at a path git never holds (${JSON.stringify(p)})`; break; }
      mkdirSync(dirname(join(tmp, p)), { recursive: true }); writeFileSync(join(tmp, p), git(['show', `${base}:${p}`], { encoding: 'buffer' }));
    }
    if (!why) {
      try { out = new Set(tg(['check-ignore', '--no-index', '-z', '--stdin'], { input: `${paths.join('\0')}\0`, stdio: ['pipe', 'pipe', 'pipe'] }).split('\0').filter(Boolean)); }
      catch (e) { if (e.status === 1) out = new Set(); else why = `git check-ignore failed (exit ${e.status})`; }
    }
  } catch (e) { why = `the ignore check failed (${String(e?.message ?? e).split('\n')[0]})`; }
  finally { rmSync(tmp, { recursive: true, force: true }); }
  if (why) stop(`${why}; never guessed`);
  return out;
}

function main(argv) {
  let files, base = null, head = null, words = 'not checked at plan time', forMergeCheck = [];
  if (argv[0] === '--files') {
    files = argv.slice(1).map((path) => ({ path, lines: 1 }));
    if (!files.length) stop('no planned paths');
  } else {
    if (!argv[0]) stop('usage: track.mjs <head> | --files <path>...');
    try { TOP = git(['rev-parse', '--show-toplevel']).trim(); } catch { stop('not inside a git repository'); }
    try { head = git(['rev-parse', '--verify', '--end-of-options', `${argv[0]}^{commit}`]).trim(); base = git(['merge-base', 'refs/remotes/origin/main', head]).trim(); }
    catch { stop('no merge base with origin/main (a shallow clone? fetch more history); never guessed'); }
    const raw = git(['diff', '--raw', '-z', ...DIFF, base, head]).split('\0');
    const num = git(['diff', '--numstat', '-z', ...DIFF, base, head]).split('\0');
    const meta = new Map();
    for (let i = 0; i + 1 < raw.length; i += 2) meta.set(raw[i + 1], raw[i].split(' ')[1] === '000000' ? raw[i].split(' ')[0].slice(1) : raw[i].split(' ')[1]);
    files = [];
    for (const n of num) { const i = n.indexOf('\t'), j = i < 0 ? -1 : n.indexOf('\t', i + 1); if (j < 0) continue;
      const a = n.slice(0, i), d = n.slice(i + 1, j), path = n.slice(j + 1); if (!path) continue;
      const added = addedLines(git(['diff', ...DIFF, '-U0', base, head, '--', path]));
      files.push({ path, lines: a === '-' ? 1 : Number(a) + Number(d), binary: a === '-', mode: meta.get(path), added }); }
    if (!files.length) stop('the diff is empty');
    if (files.length !== meta.size || files.some((f) => !meta.has(f.path))) stop('the raw and numstat lists of the diff differ; never guessed');
    const ignored = ignoredByBase(base, files.map((f) => f.path));
    for (const f of files) f.ignored = ignored.has(f.path);
    const twin = new Set(twinPaths(git(['ls-tree', '-r', '--full-tree', '--name-only', '-z', head]).split('\0').filter(Boolean), files.map((f) => f.path)));
    for (const f of files) f.twin = twin.has(f.path);
    const dec = files.find((f) => f.path === 'docs/DECISIONS.md');
    words = 'DECISIONS not changed';
    if (dec) {
      const show = (c, p) => { try { return git(['show', `${c}:${p}`]); } catch { return null; } };
      const ht = show(head, 'docs/DECISIONS.md') ?? '', bt = show(base, 'docs/DECISIONS.md') ?? '';
      words = wordsCheck(ht, bt, show);
      for (const w of words) forMergeCheck.push(`DECISIONS entry ${w.entry}${w.sha ? ` (sha256 ${w.sha})` : ''}: ${w.result === 'ok' ? '' : `${w.result}; `}compare with his own message`);
      if (otherAdded(ht, bt, dec.added)) forMergeCheck.push('docs/DECISIONS.md: added text outside a newly copied entry');
    }
    for (const f of files) if (f.path !== 'docs/DECISIONS.md' && f.added && /\[DECIDED\]|^\+>/m.test(f.added)) forMergeCheck.push(`${f.path}: a quote of him or a [DECIDED] line outside DECISIONS`);
  }
  const c = classify(files);
  if (argv[0] === '--files') c.reviewers.provisional = 'by file count at plan time (each path counts 1); the head run decides';
  if (Array.isArray(words)) for (const w of words) if (w.floor) c.goFloors.push(`docs/DECISIONS.md entry ${w.entry}: ${w.result}`);
  if (c.toolSource.length) forMergeCheck.push('TOOL-SOURCE: a new download, version or checksum? (judged, never a pass)');
  const out = { base, head, track: c.track, why: c.why, reviewers: c.reviewers, goFloors: c.goFloors.sort(), forMergeCheck: forMergeCheck.sort(),
    eyesPaths: c.eyesPaths, toolSource: c.toolSource, wordsRecord: words };
  process.stdout.write(`${JSON.stringify(out, null, 2)}\n`);
}

if (import.meta.url === pathToFileURL(process.argv[1] || '').href) {
  try { main(process.argv.slice(2)); } catch (e) { stop(`git failed (${String(e?.message ?? e).split('\n')[0]}); never guessed`); }
}

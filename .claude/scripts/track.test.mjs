// Tests for track.mjs (the review tracks; plan: the review-tracks change-set). Run: node --test .claude/scripts/track.test.mjs
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { execFileSync, spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { mkdirSync, mkdtempSync, readdirSync, readFileSync, rmSync, symlinkSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { addedLines, classify, otherAdded, pick, twinPaths, wordsCheck } from './track.mjs';

const HERE = dirname(fileURLToPath(import.meta.url));
const TRACK = join(HERE, 'track.mjs');
const one = (path, extra = {}) => classify([{ path, lines: 1, ...extra }]);
const reason = (line) => line.slice(line.indexOf(': ') + 2); // the reason only, never the path
const sha256 = (t) => createHash('sha256').update(t).digest('hex');

// [path, expected track, a word the reason must contain]
const CASES = [
  ['crates/protocol/src/codec.rs', 'full', 'netcode'], ['crates/room/src/lib.rs', 'full', 'netcode'],
  ['crates/client-endpoint/src/block.rs', 'full', 'netcode'], ['crates/sim-wasm/src/lib.rs', 'full', 'netcode'],
  ['packages/net/src/x.ts', 'full', 'netcode'], ['packages/sim-worker/src/w.ts', 'full', 'netcode'],
  ['crates/sim/src/world.rs', 'full', 'Rust core'], ['crates/devtools/src/main.rs', 'full', 'Rust core'],
  ['Cargo.toml', 'full', 'Rust core'], ['Cargo.lock', 'full', 'Rust core'], ['rust-toolchain.toml', 'full', 'Rust core'],
  ['rust-toolchain', 'full', 'Rust core'], ['spikes/x/main.rs', 'full', 'Rust core'], ['crates/server/src/main.rs', 'full', 'server code'],
  ['crates/supervisor/src/lib.rs', 'full', 'server code'], ['packages/client/src/antiCheat/check.ts', 'full', 'anti-cheat'],
  ['docs/anti-cheat.md', 'full', 'anti-cheat'], ['CLAUDE.md', 'full', 'rule files'], ['docs/sub/CLAUDE.md', 'full', 'rule files'],
  ['Claude.md', 'full', 'rule files'], ['AGENTS.md', 'full', 'rule files'], ['CLAUDE.local.md', 'full', 'rule files'],
  ['.claude/hooks/routine-guard.mjs', 'full', 'rule files'], ['.Claude/agents/advisor.md', 'full', 'rule files'],
  ['.github/workflows/gate.yml', 'full', 'rule files'], ['tools/gate.mjs', 'full', 'rule files'], ['tools/smoke.mjs', 'full', 'rule files'],
  ['tools/battery.js', 'full', 'rule files'], ['tools/gate-globals.json', 'full', 'rule files'], ['tools/package-lock.json', 'full', 'rule files'],
  ['tools/npm-shrinkwrap.json', 'full', 'rule files'],
  ['packages/client/.npmrc', 'full', 'rule files'], ['packages/types/src/gen.ts', 'full', 'boundary'],
  ['packages/client/scripts/build.mjs', 'full', 'setup'], ['packages/client/serve.mjs', 'full', 'setup'],
  ['package.json', 'full', 'manifest'], ['packages/client/package.json', 'full', 'manifest'], ['package-lock.json', 'full', 'manifest'],
  ['tools/x/Cargo.toml', 'full', 'manifest'], ['packages/client/Cargo.toml', 'full', 'manifest'],
  ['packages/e2e/playwright.config.ts', 'full', 'CI'], ['packages/client/tsconfig.json', 'full', 'CI'], ['packages/client/vite.config.mjs', 'full', 'CI'],
  ['packages/client/vitest.config.ts', 'full', 'CI'], ['.gitignore', 'full', 'dotfile'], ['spikes/netcode-proof/README.md', 'full', 'spike'],
  ['newfolder/x.txt', 'full', 'in no list'], ['Docs/x.md', 'full', 'in no list'], ['content/source/units.json', 'full', 'in no list'],
  ['docs/STATUS.md', 'light', 'light'], ['docs/adr/0001-x.md', 'light', 'light'], ['README.md', 'light', 'light'], ['FIRST_PROMPTS.md', 'light', 'light'],
  ['docs/V2_PHASE0_BRIEF.md', 'light', 'light'], ['tools/preview.js', 'light', 'light'], ['legacy/shamble-game.js', 'light', 'light'],
  ['packages/client/src/hud.ts', 'light', 'light'], ['packages/e2e/tests/a.spec.ts', 'light', 'light'], ['packages/editor/src/x.ts', 'light', 'light'],
];
for (const [path, track, word] of CASES) {
  test(`track: ${path} is ${track} (${word})`, () => {
    const c = one(path);
    assert.equal(c.track, track);
    assert.match(reason(c.why[0]), new RegExp(word, 'i'));
  });
}

test('the tier-2 floor: gitignored, symlink, submodule, odd characters, binary, a case-only twin', () => {
  for (const [f, word] of [[{ path: 'packages/client/dist/a.js', ignored: true }, 'gitignored'], [{ path: 'docs/link', mode: '120000' }, 'symlink'],
    [{ path: 'docs/sub', mode: '160000' }, 'submodule'], [{ path: 'docs/a b.md' }, 'character'], [{ path: 'docs/a~.md' }, 'character'],
    [{ path: 'docs/a.png', binary: true }, 'binary']]) {
    const c = classify([{ lines: 1, ...f }]);
    assert.equal(c.track, 'full'); assert.match(c.goFloors.join(), new RegExp(word));
  }
  const c = classify([{ path: 'docs/A.md', lines: 1 }, { path: 'docs/a.md', lines: 1 }]);
  assert.equal(c.track, 'full'); assert.match(c.goFloors.join(), /case-only twin/);
  assert.equal(classify([]).track, 'full', 'nothing to read is never light');
});

test('twinPaths(): a changed path with a case-only twin, of the file or of a folder, in either sort order', () => {
  const tree = ['docs/A.md', 'docs/a.md', 'docs/x/a.md', 'docs/X/b.md', 'docs/y.md', 'z/B.md', 'z/b.md'];
  assert.deepEqual(twinPaths(tree, ['docs/a.md']), ['docs/a.md']);
  assert.deepEqual(twinPaths(tree, ['docs/A.md']), ['docs/A.md']);
  assert.deepEqual(twinPaths(tree, ['docs/x/a.md', 'docs/X/b.md']), ['docs/x/a.md', 'docs/X/b.md'], 'a folder-only twin');
  assert.deepEqual(twinPaths(tree, ['docs/y.md', 'gone.md']), [], 'a twin outside the change, a deleted path');
  const c = classify([{ path: 'docs/a.md', lines: 1, twin: true }]);
  assert.equal(c.track, 'full'); assert.deepEqual(c.goFloors, ['docs/a.md: a case-only twin (risk tier 2)']);
});

test('addedLines(): the "+++" header never counts; an added line starting with "++" does', () => {
  const diff = 'diff --git a/x b/x\n--- a/x\n+++ b/x\n@@ -0,0 +1,2 @@\n+++i; el.innerHTML = s\n+b';
  assert.equal(addedLines(diff), '+++i; el.innerHTML = s\n+b');
  assert.equal(addedLines('diff --git a/x b/x\nold mode 100644\nnew mode 100755'), '');
});

test('his go floors, the eyes paths and the tool-source list', () => {
  assert.match(one('content/source/x.json').goFloors.join(), /SOURCE-GO/);
  assert.match(one('docs/adr/0002-x.md').goFloors.join(), /ADR/);
  for (const n of ['LOOK_V2', 'ART_DIRECTION', 'GAME_DESIGN', 'DESIGN_NOTES', 'VISION_V2', 'LOOP_V2', 'EDITOR_V2', 'UNITS', 'V2_PHASE0_BRIEF'])
    assert.match(one(`docs/${n}.md`).goFloors.join(), /design or look/, n);
  assert.match(one('.claude/settings.json').goFloors.join(), /risk tier 2/);
  assert.deepEqual(one('docs/STATUS.md').goFloors, []);
  for (const p of ['legacy/index.html', 'packages/client/src/hud.ts', 'crates/sim/src/world.rs', 'packages/sim-worker/x.ts', 'content/tuning.json',
    'x/a.css', 'packages/editor/src/x.ts', 'a/b.html'])
    assert.deepEqual(one(p).eyesPaths, [p]);
  assert.deepEqual(one('tools/preview.js').eyesPaths, []);
  for (const p of ['Cargo.lock', 'package.json', 'packages/client/package.json', '.github/workflows/gate.yml', 'rust-toolchain', 'rust-toolchain.toml',
    'crates/sim/Cargo.toml'])
    assert.deepEqual(one(p).toolSource, [p]);
  assert.deepEqual(one('docs/x.md').toolSource, []);
});

// The scores inputs-B measured on the two merges, as one file per owner area (lines owned)
const S3A = [['crates/sim/x.rs', 1964], ['packages/sim-worker/x.ts', 703], ['packages/client/scripts/b.mjs', 647], ['packages/client/src/x.ts', 384]];
const S3B = [['crates/protocol/x.rs', 4515], ['packages/sim-worker/x.ts', 1238], ['packages/client/src/x.ts', 947], ['crates/sim/x.rs', 411], ['packages/client/scripts/c.mjs', 8]];
const files = (rows) => rows.map(([path, lines]) => ({ path, lines }));
test('full picks reproduce inputs-B on stops 3a and 3b; the tie order and the manifest skip', () => {
  assert.deepEqual(pick(files(S3A), true).picks, ['test-engineer', 'security-auditor', 'rust-reviewer']);
  assert.deepEqual(pick(files(S3B), true).picks, ['test-engineer', 'security-auditor', 'netcode-architect']);
  assert.deepEqual(pick([{ path: 'crates/sim/x.rs', lines: 5 }], true).picks, ['test-engineer', 'rust-reviewer']);
  assert.deepEqual(pick([{ path: 'spikes/a/README.md', lines: 5 }], true).picks, ['test-engineer']);
  assert.deepEqual(pick([{ path: 'spikes/a/src/main.rs', lines: 5 }], true).picks, ['test-engineer', 'rust-reviewer'], 'a spike\'s .rs (inputs-B R5)');
  assert.deepEqual(pick(files([['packages/client/src/x.ts', 10], ['packages/sim-worker/x.ts', 10], ['crates/sim/x.rs', 10]]), true).picks,
    ['test-engineer', 'rust-reviewer', 'engine-architect'], 'equal lines: the fixed tie order');
  assert.deepEqual(pick(files([['crates/sim/Cargo.toml', 500], ['packages/sim-worker/x.ts', 10]]), true).picks, ['test-engineer', 'engine-architect'],
    'manifest lines are not counted');
});
test('light pick, its fallback and tie order, the security triggers, and rule files', () => {
  assert.deepEqual(pick([{ path: 'packages/client/src/x.ts', lines: 3 }], false).picks, ['perf-auditor']);
  assert.deepEqual(pick([{ path: 'docs/x.md', lines: 3 }], false).picks, ['test-engineer']);
  assert.deepEqual(pick(files([['tools/x.js', 3], ['packages/client/src/x.ts', 3]]), false).picks, ['perf-auditor'], 'equal lines: by name');
  assert.deepEqual(pick([{ path: 'legacy/g.js', lines: 3, added: '+el.innerHTML = s' }], false).picks, ['test-engineer', 'security-auditor']);
  assert.deepEqual(pick([{ path: 'packages/client/index.html', lines: 3 }], false).picks, ['perf-auditor', 'security-auditor']);
  assert.deepEqual(pick([{ path: '.claude/x.md', lines: 1 }], true).picks, ['advisor', 'security-auditor']);
  for (const w of ['innerHTML', 'outerHTML', 'insertAdjacentHTML', 'document.write', 'eval(', 'new Function', 'localStorage', 'sessionStorage',
    'indexedDB', 'fetch(', 'WebSocket', 'WebTransport', 'XMLHttpRequest', 'postMessage', 'Content-Security-Policy'])
    for (const path of ['legacy/g.js', 'tools/x.js', 'packages/editor/x.ts'])
      assert.ok(pick([{ path, lines: 1, added: `+x ${w}` }], false).picks.includes('security-auditor'), `${w} in ${path}`);
  assert.deepEqual(pick([{ path: 'docs/s.md', lines: 1, added: '+fetch( localStorage XMLHttpRequest' }], false).picks, ['test-engineer'],
    'the text triggers count only under packages/, tools/ and legacy/');
});

test('the result is the same for shuffled input', () => {
  const a = classify(files([['docs/b.md', 1], ['docs/a.md', 1], ['crates/sim/x.rs', 2]]));
  const b = classify(files([['crates/sim/x.rs', 2], ['docs/a.md', 1], ['docs/b.md', 1]]));
  assert.deepEqual(a, b);
});

// The copied-words check on real DECISIONS entries
const DEC = readFileSync(join(HERE, '..', '..', 'docs', 'DECISIONS.md'), 'utf8');
const entry = (id) => DEC.split(/\n(?=- \*\*)/).find((e) => e.includes(id));
const quoteOf = (e) => { const l = e.split('\n'), k = l.findIndex((x) => /^>( |$)/.test(x)); let q = k;
  while (q < l.length && /^>( |$)/.test(l[q])) q++; return l.slice(k, q).map((x) => x.replace(/^> ?/, '')).join('\n'); };
const flat = (e) => e.replace(/\n\s*/g, ' ');
const savedOf = (e) => { const m = flat(e).match(/saved file `([^`]+)` \(readable at commit ([0-9a-f]{7,40})(?: [^)]*)?\)/); return m && { path: m[1], commit: m[2] }; };
// the recorded form of an entry's quote: the quote, or the quote plus one final newline, as its stated byte count says
const formOf = (e) => (Buffer.byteLength(quoteOf(e)) === Number(flat(e).match(/\((\d+) bytes, sha256/)[1]) ? quoteOf(e) : `${quoteOf(e)}\n`);
// a reader that answers only when asked for the entry's own named commit and file
const readerFor = (e, text) => (commit, path) => {
  const s = savedOf(e); assert.ok(s, 'the entry names a saved file');
  assert.equal(commit, s.commit); assert.equal(path, s.path); return text;
};
test('words check: real entries pass in both recorded forms; the saved file must equal the form that matched', () => {
  for (const id of ['31a506d9', 'de4e9b89', '2f215f0d']) {
    const e = entry(id); assert.ok(e, id);
    const [w, ...rest] = wordsCheck(e, '', readerFor(e, quoteOf(e)));
    assert.deepEqual(rest, []); assert.equal(w.result, 'ok', id); assert.equal(w.floor, false);
    assert.equal(w.sha, flat(e).match(/sha256 ([0-9a-f]{64})/)[1]);
  }
  for (const id of ['1a4fd6e1', 'a07f7ff8']) { // their stated bytes and sha256 cover the quote plus one final newline
    const e = entry(id); assert.ok(e, id);
    assert.equal(wordsCheck(e, '', readerFor(e, `${quoteOf(e)}\n`))[0].result, 'ok', id);
    assert.equal(wordsCheck(e, '', readerFor(e, quoteOf(e)))[0].result, 'saved file differs', id);
  }
  // a saved file named with its branch after the commit, and a header wrapped over many lines
  for (const id of ['90efc3d2', '809fbcd0', 'd736715d', '83ce083b', '7d208dc5']) {
    const e = entry(id); assert.ok(e, id); assert.ok(savedOf(e), id);
    assert.deepEqual(wordsCheck(e, '', readerFor(e, formOf(e))).map((w) => [w.result, w.floor]), [['ok', false]], id);
    const head = '# Mustafa to x\n\n_Copied by script from the transcript; what it answers_\n\n---\n'; // the record's third form
    assert.equal(wordsCheck(e, '', readerFor(e, `${head}${quoteOf(e)}\n`))[0].result, 'ok', id);
    for (const t of [`${head}${quoteOf(e)} more\n`, `${head}x\n${quoteOf(e)}\n`, `his first part\n---\n${quoteOf(e)}\n`, `# t\n\nnot a copy note\n\n---\n${quoteOf(e)}\n`])
      assert.equal(wordsCheck(e, '', readerFor(e, t))[0].result, 'saved file differs', `${id}: ${t.slice(0, 30)}`);
  }
  const msg = wordsCheck(entry('247437b9'), '', () => assert.fail('not read'))[0]; // its sha256 is of the whole message
  assert.deepEqual([msg.result, msg.floor], ['no stated bytes and sha256', true]);
});
test('words check: a tampered copy, a differing or unreadable saved file, a missing saved-file clause', () => {
  const e = entry('de4e9b89');
  const tampered = wordsCheck(e.replace('done:', 'Done:'), '', readerFor(e, quoteOf(e)))[0];
  assert.equal(tampered.result, 'quote does not match its bytes and sha256'); assert.equal(tampered.floor, true);
  const differs = wordsCheck(e, '', readerFor(e, 'something else'))[0];
  assert.equal(differs.result, 'saved file differs'); assert.equal(differs.floor, true);
  assert.equal(wordsCheck(e, '', readerFor(e, null))[0].result, 'saved file cannot be read');
  assert.deepEqual(wordsCheck(e, e, readerFor(e, null)), [], 'an unchanged entry already in the base is not checked again');
  const q = 'hello', bare = `- **Taken by Mustafa, 2026-11-01, x** (message 11111111-aaaa). Copied by script (5 bytes, sha256 ${sha256(q)}):\n> ${q}\n`;
  const none = wordsCheck(bare, '', () => assert.fail('no saved file is named, so none is read'))[0];
  assert.equal(none.result, 'no saved file named'); assert.equal(none.floor, false, 'listed for the merge check, not a floor (plan 3f)');
  assert.deepEqual(Object.values(wordsCheck(bare.replace(/ Copied by script[^\n]*/, ''), '', () => null)[0]).slice(1), [null, true, 'no stated bytes and sha256']);
});
test('words check: an existing entry changed, removed or doubled is a go floor; text after its quote is not', () => {
  const e = entry('de4e9b89'), r = readerFor(e, quoteOf(e));
  const changed = wordsCheck(e.replace('done:', 'Done:'), e, r);
  assert.deepEqual(changed.map((w) => [w.result, w.floor]), [["an existing entry's header or quote was changed", true]]);
  const w = entry('83ce083b'), sha = flat(w).match(/sha256 ([0-9a-f]{64})/)[1], rw = readerFor(w, formOf(w));
  assert.ok(w.split('\n').findIndex((l) => l.includes(sha)) > 0, 'the stated sha256 sits on a wrapped header line');
  assert.deepEqual(wordsCheck(w.replace(sha, 'f'.repeat(64)), w, rw).map((x) => x.result), ["an existing entry's header or quote was changed"]);
  const n = entry('14631e5d'), later = n.split('\n').find((l) => l.startsWith("> The kit rule's exact lines")); // a quote line after a note
  assert.ok(later && n.split('\n').indexOf(later) > n.split('\n').findIndex((l) => /left out/.test(l)));
  assert.deepEqual(wordsCheck(n.replace(`\n${later}`, ''), n, rw).map((x) => x.result), ["an existing entry's header or quote was changed"]);
  assert.deepEqual(wordsCheck('# D\n', e, r).map((w) => [w.result, w.floor]), [['an existing entry was removed or its first line changed', true]]);
  assert.deepEqual(wordsCheck(`${e}\n${e}`, e, r).map((w) => w.result), ['two entries share this first line']);
  assert.deepEqual(wordsCheck(`${e}\nA later note of the chat's.`, e, r), []);
});
test('otherAdded(): a stray added line is listed; the lines of a newly copied entry and blank lines are not', () => {
  const base = '# D\n\n', q = 'hello', fresh = `- **Taken by Mustafa, x** (message 22222222-bbbb). Copied by script (5 bytes, sha256 ${sha256(q)}):\n> ${q}`;
  const added = fresh.split('\n').map((l) => `+${l}`).join('\n');
  assert.equal(otherAdded(base + fresh, base, added), false);
  assert.equal(otherAdded(base + fresh, base, `${added}\n+`), false);
  assert.equal(otherAdded(`${base + fresh}\nstray`, base, `${added}\n+stray`), true);
  const w = entry('83ce083b'), wl = w.split('\n').map((l) => `+${l}`).join('\n');
  assert.equal(otherAdded(`${base}${w}`, base, wl), false, 'a new entry with a wrapped header');
  assert.equal(otherAdded(`${base}${w}\nnote\n> later`, base, `${wl}\n+note\n+> later`), true, 'a quote line after a note is not the entry\'s');
});

test('--files: planned paths, picks marked provisional; no paths exit 2 with its reason', () => {
  const run = (args) => spawnSync(process.execPath, [TRACK, ...args], { encoding: 'utf8' });
  const r = run(['--files', 'docs/a.md']);
  assert.equal(r.status, 0); const o = JSON.parse(r.stdout);
  assert.equal(o.track, 'light'); assert.match(o.reviewers.provisional, /head run decides/); assert.equal(o.wordsRecord, 'not checked at plan time');
  const e = run(['--files']); assert.equal(e.status, 2); assert.match(e.stderr, /no planned paths/);
});

// The git layer, on a temporary repository: a base commit that origin/main points at, one branch per case.
const ENV = Object.fromEntries(Object.entries(process.env).filter(([k]) => !/^GIT_/.test(k)));
test('git layer on a fixture repository', async (t) => {
  const dir = mkdtempSync(join(tmpdir(), 'track-fx-'));
  const g = (...a) => execFileSync('git', a, { cwd: dir, encoding: 'utf8', env: ENV, stdio: ['ignore', 'pipe', 'pipe'] });
  const w = (p, s) => { mkdirSync(dirname(join(dir, p)), { recursive: true }); writeFileSync(join(dir, p), s); };
  const run = (ref, opts = {}) => spawnSync(process.execPath, [TRACK, ref], { cwd: opts.cwd || dir, encoding: 'utf8', env: { ...ENV, ...opts.env } });
  const out = (ref, opts) => { const r = run(ref, opts); assert.equal(r.status, 0, r.stderr); return JSON.parse(r.stdout); };
  const branch = (name, make) => { g('switch', '-q', '-c', name, 'main'); make(); g('add', '-A'); g('commit', '-q', '-m', name); };
  const hello = `- **Taken by Mustafa, 2026-11-01, x** (message 11111111-aaaa). Copied by script (5 bytes, sha256 ${sha256('hello')}):\n> hello\n`;
  try {
    g('init', '-q', '-b', 'main'); g('config', 'user.email', 't@t'); g('config', 'user.name', 't'); g('config', 'core.ignorecase', 'false');
    w('.gitignore', 'dist/\n.env.*\n!.env.example\n'); w('docs/A.md', 'base\n'); w('docs/DECISIONS.md', `# D\n\n${hello}`); w('legacy/g.js', 'x\n');
    g('add', '-A'); g('commit', '-q', '-m', 'base'); g('update-ref', 'refs/remotes/origin/main', 'HEAD');

    await t.test('a new case-only twin', () => {
      branch('twin', () => w('docs/a.md', 'twin\n'));
      const o = out('twin');
      assert.equal(o.track, 'full'); assert.deepEqual(o.goFloors, ['docs/a.md: a case-only twin (risk tier 2)']);
      assert.equal(o.wordsRecord, 'DECISIONS not changed');
      assert.equal(run('twin', { cwd: join(dir, 'docs') }).stdout, JSON.stringify(o, null, 2) + '\n', 'the same from a subfolder');
    });
    await t.test('a folder-only twin', () => {
      branch('ftwin', () => w('DOCS/b.md', 'x\n'));
      assert.ok(out('ftwin').goFloors.includes('DOCS/b.md: a case-only twin (risk tier 2)'));
    });
    await t.test('the base ignore rules hold under core.ignorecase=true, and local excludes add nothing', () => {
      branch('envcase', () => { w('.env.Example', 'x\n'); g('add', '-f', '.env.Example'); });
      const o = out('envcase'); assert.ok(o.goFloors.includes('.env.Example: a gitignored path (risk tier 2)'));
      w('.git/info/exclude', '*.md\n'); g('config', 'core.ignorecase', 'true');
      try { assert.deepEqual(out('envcase'), o); assert.ok(!out('twin').goFloors.some((x) => /gitignored/.test(x))); }
      finally { g('config', 'core.ignorecase', 'false'); w('.git/info/exclude', ''); }
    });
    await t.test('a tag named origin/main does not move the base', () => {
      g('switch', '-q', '-c', 'tagged', 'main'); w('.claude/x.md', 'x\n'); g('add', '-A'); g('commit', '-q', '-m', 'rule');
      g('tag', 'origin/main'); w('docs/y.md', 'y\n'); g('add', '-A'); g('commit', '-q', '-m', 'doc');
      try { assert.ok(out('tagged').goFloors.includes('.claude/x.md: rule files, secrets or infrastructure (risk tier 2)')); }
      finally { g('tag', '-d', 'origin/main'); }
    });
    await t.test('a force-added ignored file stays floored when the head drops the ignore rule (the base rules decide)', () => {
      branch('ign', () => { w('dist/x.js', 'x\n'); w('.gitignore', '\n'); g('add', '-f', 'dist/x.js'); });
      assert.ok(out('ign').goFloors.includes('dist/x.js: a gitignored path (risk tier 2)'));
    });
    await t.test('a symlink, a submodule and a binary file', () => {
      branch('kinds', () => { symlinkSync('A.md', join(dir, 'docs/link')); w('docs/a.png', Buffer.from([0, 1, 2, 0, 255])); });
      g('update-index', '--add', '--cacheinfo', `160000,${g('rev-parse', 'HEAD').trim()},docs/sub`); g('commit', '-q', '-m', 'gitlink');
      const o = out('kinds');
      for (const f of ['docs/link: a symlink', 'docs/sub: a submodule', 'docs/a.png: a binary file']) assert.ok(o.goFloors.includes(`${f} (risk tier 2)`), f);
      g('config', 'diff.ignoreSubmodules', 'all');
      try { assert.deepEqual(out('kinds'), o, 'a config that hides submodules does not hide the floor'); }
      finally { g('config', '--unset', 'diff.ignoreSubmodules'); }
    });
    await t.test('a file name git check-ignore cannot read stops the run (exit 2), never "not ignored"', () => {
      branch('magic', () => w(':(icase)x', 'x\n'));
      const left = () => readdirSync(tmpdir()).filter((n) => n.startsWith('track-ignore-')).length, before = left();
      const r = run('magic'); assert.equal(r.status, 2, r.stdout); assert.match(r.stderr, /check-ignore failed/);
      assert.equal(left(), before, 'its temporary folder is removed before the stop');
    });
    await t.test('a path holding a tab is read whole (its floors stand)', () => {
      branch('tab', () => { symlinkSync('../.claude/settings.json', join(dir, 'docs', 'link\tx')); w('docs/page\t.html', '<p>x</p>\n'); });
      const o = out('tab');
      assert.equal(o.track, 'full');
      assert.ok(o.goFloors.includes('docs/link\tx: a symlink (risk tier 2)'), JSON.stringify(o.goFloors));
      assert.ok(o.goFloors.includes('docs/page\t.html: a character outside the allowed set (risk tier 2)'));
      assert.ok(o.eyesPaths.includes('docs/page\t.html'));
    });
    await t.test('DECISIONS: a new entry in the newline form is checked, listed with its sha256, and floors nothing', () => {
      g('switch', '-q', '-c', 'words', 'main'); w('docs/notes/q.md', 'hi there\n'); g('add', '-A'); g('commit', '-q', '-m', 'saved');
      const at = g('rev-parse', 'HEAD').trim(), sha = sha256('hi there\n');
      w('docs/DECISIONS.md', `# D\n\n${hello}- **Taken by Mustafa, 2026-11-02, y** (message 22222222-bbbb; copied by script from the chat's saved file ` +
        `\`docs/notes/q.md\` (readable at commit ${at})). Copied by script (9 bytes, sha256 ${sha}):\n> hi there\n`);
      g('commit', '-q', '-am', 'entry');
      const o = out('words');
      assert.deepEqual(o.wordsRecord, [{ entry: '22222222-bbbb', sha, floor: false, result: 'ok' }]);
      assert.ok(o.forMergeCheck.includes(`DECISIONS entry 22222222-bbbb (sha256 ${sha}): compare with his own message`), JSON.stringify(o.forMergeCheck));
      assert.ok(!o.forMergeCheck.some((l) => /outside a newly copied entry/.test(l)));
      assert.deepEqual(o.goFloors, []);
    });
    await t.test('DECISIONS: an existing quote changed, an entry removed, other added text', () => {
      branch('changed', () => w('docs/DECISIONS.md', `# D\n\n${hello.replace('> hello', '> HELLO')}`));
      assert.ok(out('changed').goFloors.includes("docs/DECISIONS.md entry 11111111-aaaa: an existing entry's header or quote was changed"));
      branch('removed', () => w('docs/DECISIONS.md', '# D\n\nsomething else\n'));
      assert.ok(out('removed').goFloors.includes('docs/DECISIONS.md entry 11111111-aaaa: an existing entry was removed or its first line changed'));
      branch('stray', () => w('docs/DECISIONS.md', `# D\n\n${hello}stray note\n`));
      const o = out('stray');
      assert.ok(o.forMergeCheck.includes('docs/DECISIONS.md: added text outside a newly copied entry')); assert.deepEqual(o.goFloors, []);
    });
    await t.test('a quote outside DECISIONS and the TOOL-SOURCE prompt are listed for the merge check', () => {
      branch('lists', () => { w('docs/x.md', 'see\n> a quote of him\n'); w('package.json', '{}\n'); });
      const o = out('lists');
      assert.ok(o.forMergeCheck.includes('docs/x.md: a quote of him or a [DECIDED] line outside DECISIONS'));
      assert.ok(o.forMergeCheck.includes('TOOL-SOURCE: a new download, version or checksum? (judged, never a pass)'));
      assert.deepEqual(o.toolSource, ['package.json']);
    });
    await t.test('a text trigger, the same from a subfolder, under odd git config and with GIT_DIR set', () => {
      branch('trig', () => { w('legacy/g.js', 'x\n++i; el.innerHTML = s\n'); w('legacy/h.js', 'new XMLHttpRequest()\n'); });
      const a = run('trig');
      assert.equal(a.status, 0, a.stderr); assert.ok(JSON.parse(a.stdout).reviewers.picks.includes('security-auditor'));
      assert.equal(run('trig', { cwd: join(dir, 'legacy') }).stdout, a.stdout, 'from a subfolder');
      assert.equal(run('trig', { env: { GIT_DIR: join(dir, 'no-such-dir') } }).stdout, a.stdout, 'GIT_* variables are dropped');
      g('config', 'color.ui', 'always'); g('config', 'diff.noprefix', 'true'); g('config', 'diff.external', 'false');
      try { assert.equal(run('trig').stdout, a.stdout, 'colour, prefixes and an external diff do not change it'); }
      finally { g('config', '--unset', 'color.ui'); g('config', '--unset', 'diff.noprefix'); g('config', '--unset', 'diff.external'); }
    });
    await t.test('exit 2 with its reason: an empty diff, no merge base, outside a repository', () => {
      const empty = run('main'); assert.equal(empty.status, 2); assert.match(empty.stderr, /the diff is empty/);
      g('switch', '-q', '--orphan', 'lone'); g('commit', '-q', '--allow-empty', '-m', 'lone');
      const lone = run('lone'); assert.equal(lone.status, 2); assert.match(lone.stderr, /no merge base/);
      const ref = run('no-such-ref-xyz'); assert.equal(ref.status, 2); assert.match(ref.stderr, /no merge base/);
      const bare = mkdtempSync(join(tmpdir(), 'track-norepo-'));
      try { const r = run('HEAD', { cwd: bare }); assert.equal(r.status, 2); assert.match(r.stderr, /not inside a git repository/); }
      finally { rmSync(bare, { recursive: true, force: true }); }
    });
  } finally { rmSync(dir, { recursive: true, force: true }); }
});

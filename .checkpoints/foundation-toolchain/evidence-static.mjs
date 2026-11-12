// Step 5's re-run of the static evidence (plan Part 7): E4a, E5, E6 (the root package.json's "type"), E7 and E11.
// Run from the branch worktree's top: node .checkpoints/foundation-toolchain/evidence-static.mjs
// Read-only; prints one line per check and exits 1 if any check fails.
import { execFileSync } from 'node:child_process';
import { existsSync, readFileSync, readdirSync } from 'node:fs';

let failed = 0;
const say = (id, ok, text) => { console.log(`${ok ? 'PASS' : 'FAIL'}  ${id}  ${text}`); if (!ok) failed++; };
const git = (args) => execFileSync('git', args, { encoding: 'utf8' });

// E4a: no tracked non-Markdown file outside spikes/, docs/ and .checkpoints/ names spikes/; no root .cargo/config.toml.
let hits = '';
try { hits = git(['grep', '-n', 'spikes/', '--', ':!spikes', ':!docs', ':!.checkpoints', ':!*.md']); } catch (e) { hits = e.status === 1 ? '' : `git grep failed: ${e.message}`; }
say('E4a', hits === '' && !existsSync('.cargo/config.toml') && !existsSync('.cargo/config'),
  hits === '' ? 'no tracked file outside spikes/, docs/, .checkpoints/ or *.md names spikes/; no root .cargo/config(.toml)' : hits.trim());

// E5: gate.yml is byte-identical to main's.
let e5 = true;
try { git(['diff', '--exit-code', '--quiet', 'origin/main', '--', '.github/workflows/gate.yml']); } catch { e5 = false; }
say('E5', e5, `.github/workflows/gate.yml ${e5 ? 'equals' : 'differs from'} origin/main's (${git(['rev-parse', '--short', 'origin/main']).trim()})`);

// E6 (second half): the root package.json has no "type" key.
const root = JSON.parse(readFileSync('package.json', 'utf8'));
say('E6', !('type' in root), `the root package.json has ${'type' in root ? `"type": ${JSON.stringify(root.type)}` : 'no "type" key'}`);

// E7: every package.json in the root and packages/* pins exact versions (no ^, ~, *, x, range or tag); TypeScript 6.
const files = ['package.json', ...readdirSync('packages').map((p) => `packages/${p}/package.json`).filter(existsSync)];
const exact = /^\d+\.\d+\.\d+$/;
const bad = [];
let ts = null;
for (const f of files) {
  const j = JSON.parse(readFileSync(f, 'utf8'));
  for (const k of ['dependencies', 'devDependencies', 'optionalDependencies', 'peerDependencies']) {
    for (const [name, v] of Object.entries(j[k] ?? {})) {
      if (!exact.test(v)) bad.push(`${f} ${k} ${name}@${v}`);
      if (name === 'typescript') ts = v;
    }
  }
}
say('E7', bad.length === 0 && ts !== null && ts.startsWith('6.'),
  bad.length ? bad.join('; ') : `${files.length} package.json files, every version exact; typescript ${ts}`);

// E11: package-lock.json: counts with and without the workspace links; install scripts; every node_modules/ entry is a
// workspace link (resolved "packages/<name>", link true) or a registry.npmjs.org tarball with an integrity hash.
const lock = JSON.parse(readFileSync('package-lock.json', 'utf8'));
const entries = Object.entries(lock.packages).filter(([k]) => k.startsWith('node_modules/') || k.includes('/node_modules/'));
const links = entries.filter(([, v]) => v.link === true && /^packages\/[a-z0-9-]+$/.test(v.resolved ?? ''));
const reg = entries.filter(([, v]) => !v.link && (v.resolved ?? '').startsWith('https://registry.npmjs.org/') && /^sha512-/.test(v.integrity ?? ''));
const odd = entries.filter(([k]) => !links.some(([l]) => l === k) && !reg.some(([r]) => r === k)).map(([k]) => k);
const scripts = entries.filter(([, v]) => v.hasInstallScript).map(([k, v]) => `${k}${v.optional ? ' (optional)' : ''}${v.os ? ` os=${v.os.join('/')}` : ''}`);
say('E11', odd.length === 0 && lock.lockfileVersion === 3,
  `lockfile v${lock.lockfileVersion}: ${entries.length} node_modules entries = ${reg.length} registry packages (npmjs, sha512) + ${links.length} workspace links; ` +
  `install scripts: ${scripts.join(', ') || 'none'}${odd.length ? `; neither: ${odd.join(', ')}` : ''}`);

process.exit(failed ? 1 : 0);

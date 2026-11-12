// Step 5's four kit edits, K2a, K2b, K3 and K4 (plan Part 5, Part 8 step 5), as exact old -> new texts. It writes
// nothing in the repository: it applies them to copies under the given folder, checks that each old text occurs exactly
// once in its file (so the Edit tool's unique match holds), reports the longest new line, and prints a unified diff.
// Usage: node k-texts.mjs <out-folder>
import { readFileSync, writeFileSync, mkdirSync } from 'node:fs';
import { execFileSync } from 'node:child_process';
import { join } from 'node:path';

const PAREN = "(Mustafa's decision of 2026-11-10 in `docs/DECISIONS.md`)";
export const EDITS = [
  { id: 'K2a', file: 'CLAUDE.md',
    old: '## Hard constraints (change ONLY through an accepted ADR signed off by Mustafa, never on your own)\n',
    new: '## Hard constraints of the legacy game, `legacy/` only (change ONLY through an accepted ADR signed off by Mustafa, never on your own)\n' },
  { id: 'K2b', file: 'CLAUDE.md',
    old: 'replace ADR-001..003). Phase 0 writes no v2 code. Constraints for v2 code come only from those drafts once Mustafa\n' +
         'accepts them. Until then, the constraints above stand.\n',
    new: "replace ADR-001..003). Phase 0 writes no v2 code, except foundation steps 2.1 and 2.2 (Mustafa's decision of\n" +
         '2026-11-10 in `docs/DECISIONS.md`). Constraints for v2 code come only from those drafts once Mustafa accepts them.\n' +
         'Until then, the constraints above stand.\n' },
  { id: 'K3', file: 'CLAUDE.md',
    old: '  Mustafa and the Advisor have approved the drafts; until then, stop.\n',
    new: '  Mustafa and the Advisor have approved the drafts; until then, stop. Exception: foundation steps 2.1 and 2.2\n' +
         `  ${PAREN}.\n` },
  // K1b, only on his yes (S1 of the Fable advisor's early skeleton read): CI proves the release overflow setting.
  { id: 'K1b', file: '.github/workflows/toolchain.yml',
    old: '              cargo test --workspace --locked\n',
    new: '              cargo test --workspace --locked\n' +
         '              cargo test --release -p sim -p protocol --locked\n' },
  { id: 'K4', file: '.claude/agents/advisor.md',
    old: '- No engine code. Throwaway spikes only, and only after Mustafa and you have approved the decision drafts.\n',
    new: '- No engine code. Throwaway spikes only, and only after Mustafa and you have approved the decision drafts. Exception:\n' +
         `  foundation steps 2.1 and 2.2 ${PAREN}.\n` },
];

if (import.meta.url === `file://${process.argv[1]}`) {
  const out = process.argv[2];
  if (!out) { console.error('usage: node k-texts.mjs <out-folder>'); process.exit(2); }
  const files = {};
  let bad = 0;
  for (const e of EDITS) {
    files[e.file] ??= readFileSync(e.file, 'utf8');
    const n = files[e.file].split(e.old).length - 1;
    const line = files[e.file].slice(0, files[e.file].indexOf(e.old)).split('\n').length;
    const longest = Math.max(...e.new.split('\n').map((l) => l.length));
    console.log(`${e.id} ${e.file}:${line} old text found ${n}x; longest new line ${longest} chars`);
    if (n !== 1) bad++;
    files[e.file] = files[e.file].replace(e.old, e.new);
  }
  for (const [f, text] of Object.entries(files)) {
    const dst = join(out, f.replace(/\//g, '__'));
    mkdirSync(out, { recursive: true });
    writeFileSync(dst, text);
    try { execFileSync('git', ['diff', '--no-index', '--', f, dst], { stdio: 'inherit' }); } catch { /* diff exits 1 */ }
  }
  process.exit(bad ? 1 : 0);
}

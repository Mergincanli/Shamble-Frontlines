// Round 2's fix pass, applied from the texts the advisor agent's ruling D1-R2 approved (advisor-ruling-d1-r2.md,
// its 14 code blocks), the drafts it approved as drafted (fix-r2-drafts.md, block B) and the R2-D4 skeptic's item (4)
// (review-r2-verdicts.json, F1). Every target must match exactly once, or the script stops before writing anything.
// Usage (from the worktree root): node .checkpoints/foundation-toolchain/apply-fix-r2.mjs [--dry]
import fs from 'node:fs';

const N = '.checkpoints/foundation-toolchain/';
const dry = process.argv.includes('--dry');
const blocks = (file) => {
  const t = fs.readFileSync(file, 'utf8').split('\n'); const out = []; let s = -1;
  for (let i = 0; i < t.length; i++) if (t[i].startsWith('```')) { if (s < 0) s = i; else { out.push(t.slice(s + 1, i).join('\n')); s = -1; } }
  return out;
};
const R = blocks(N + 'advisor-ruling-d1-r2.md');
if (R.length !== 14) throw new Error('expected 14 ruling blocks, got ' + R.length);
const DR = blocks(N + 'fix-r2-drafts.md');
const B = DR.find((b) => b.startsWith('//! The wasm-bindgen bindings'));
const V = JSON.parse(fs.readFileSync(N + 'review-r2-verdicts.json', 'utf8'));
const d4 = V.find((v) => v.id === 'R2-D4').fix_ok;
const f1m = d4.match(/\(4\) Archive :627-631\. Replace everything from "The fix:" through "whichever comes first\." with:\n"([\s\S]*?)"\n\nLeftovers/);
if (!f1m) throw new Error('F1 text not found in the R2-D4 verdict');
const F1 = f1m[1];
const D3 = R[3].split('\n').find((l) => l.startsWith("; `protocol`'s binary"));
const D4 = R[3].split('\n').find((l) => l.startsWith('the flag on every workspace crate'));
if (!D3 || !D4) throw new Error('D3 or D4 missing');

const files = {};
const get = (f) => (files[f] ??= fs.readFileSync(f, 'utf8'));
const set = (f, t) => { files[f] = t; };
const once = (f, old, neu) => {
  const t = get(f); const n = t.split(old).length - 1;
  if (n !== 1) throw new Error(`${f}: expected 1 match, got ${n}: ${old.slice(0, 90)}`);
  set(f, t.replace(old, () => neu));
};
// Replace a block of whole lines: from the line starting with `first` to the line before the first later line
// matching `stop` (a predicate), with `neu`.
const lines = (f, first, stop, neu) => {
  const L = get(f).split('\n'); const s = L.findIndex((l) => l.startsWith(first));
  if (s < 0 || L.findIndex((l, i) => i > s && l.startsWith(first)) >= 0) throw new Error(`${f}: start not unique: ${first.slice(0, 80)}`);
  let e = s + 1; while (e < L.length && !stop(L[e], e)) e++;
  L.splice(s, e - s, ...neu.split('\n')); set(f, L.join('\n'));
};
const wrap = (text, first, rest, width = 120) => {
  const w = text.replace(/\s+/g, ' ').trim().split(' '); const out = []; let cur = first;
  for (const x of w) {
    const sep = cur === first || cur === rest ? '' : ' ';
    if (cur.length + sep.length + x.length > width && cur.trim()) { out.push(cur); cur = rest + x; } else cur += sep + x;
  }
  out.push(cur); return out.join('\n');
};
// Rewrap the bullet or paragraph that starts with the line starting `first`, up to the first later line matching stop.
const rewrap = (f, first, stop, firstPrefix, rest, edit = (s) => s) => {
  const L = get(f).split('\n'); const s = L.findIndex((l) => l.startsWith(first));
  if (s < 0) throw new Error(`${f}: no line starts with ${first.slice(0, 80)}`);
  let e = s + 1; while (e < L.length && !stop(L[e])) e++;
  const joined = L.slice(s, e).map((l, i) => (i === 0 ? l.slice(firstPrefix.length) : l.trim())).join(' ');
  const edited = edit(joined.replace(/\s+/g, ' '));
  L.splice(s, e - s, ...wrap(edited, firstPrefix, rest).split('\n')); set(f, L.join('\n'));
};
const sub = (old, neu) => (s) => { const n = s.split(old).length - 1; if (n !== 1) throw new Error(`expected 1 match, got ${n}: ${old.slice(0, 90)}`); return s.replace(old, () => neu); };

// A. Cargo.toml's comment (Q6).
once('Cargo.toml', [
  '# Overflow checks are on in every profile for every workspace crate (ADR 0002 part 4). Not part 4\'s per-package form:',
  "# sim's inline, generic and small functions are compiled inside the crates that call them, which that form does not",
  '# reach (foundation 2.2\'s review, RR-1; the register row OVERFLOW-ALL). "*" matches only crates outside the workspace,',
  '# so dependencies keep the release default, unchecked.'].join('\n'), R[2]);
// B. sim-wasm's doc comment (as drafted).
once('crates/sim-wasm/src/lib.rs', [
  '//! The wasm-bindgen bindings the Worker loads, and nothing else (ADR 0002 parts 3 and 5). Empty, with no',
  "//! bindings, until the boundary spike fixes its exports (OQ-0002-3); wasm-bindgen's by-day reading (OQ-0002-7)",
  '//! comes first.'].join('\n'), B);
// C. The register row (Q2).
lines('docs/adr/README.md', "| OVERFLOW-ALL: how far part 4's overflow setting reaches |", (l, i) => true, R[0]);
// G. REPO_MAP's crates row.
once('docs/REPO_MAP.md', '`client-endpoint` (prediction and reconciliation, interpolation, snapshot decoding)',
  '`client-endpoint` (prediction and reconciliation, interpolation, snapshot decoding, the aim cast, the block writer and its layout table)');
// E. STATUS: the 2.2 bullet and the night-shift line (Q7 E).
const E = R[8].split('\n');
lines('docs/STATUS.md', '- **From foundation 2.2 (PR #48), for 2.3:**', (l) => !l.startsWith('  '), E.slice(0, 6).join('\n'));
lines('docs/STATUS.md', '- **Old "Night shift"** (trig_019LtRqmAdzddyk6K3e3kDJz):', (l) => !l.startsWith('  '), E[7]);
// F1. The archive's OVERFLOW-XCRATE-CI: "The fix:" through "whichever comes first." (the R2-D4 skeptic's item (4)).
rewrap('docs/STATUS_ARCHIVE.md', '    - **OVERFLOW-XCRATE-CI**', (l) => l.startsWith('    - **'), '    - ', '      ', (s) => {
  const a = s.indexOf('The fix: PM-0002-53'); const b = s.indexOf('whichever comes first.', a);
  if (a < 0 || b < 0) throw new Error('F1 span not found'); return s.slice(0, a) + F1 + s.slice(b + 'whichever comes first.'.length);
});
// F2 and F3. The archive's OVERFLOW-PER-CRATE, then the new OVERFLOW-PANIC-RULE.
lines('docs/STATUS_ARCHIVE.md', '    - **OVERFLOW-PER-CRATE** (S3;', (l) => l.startsWith('    - **'), R[9] + '\n' + R[10]);
// F4. The archive block's intro.
rewrap('docs/STATUS_ARCHIVE.md', '- **From foundation 2.2** (PR #48, in flight', (l) => l.startsWith('  - **'), '- ', '  ', (s) => [
  sub("list as review round 1's fix pass left them (1de98a9 to 8a4f9b0), and the advisor agent's ruling D1 of 2026-11-13;",
    "list as review round 1's and round 2's fix passes left them (1de98a9 to 8a4f9b0, then round 2's fix), and the advisor agent's ruling D1 of 2026-11-13 and its ruling D1-R2;"),
  sub('each then tested by a skeptic of its own;',
    "each then tested by a skeptic of its own; R2-D1 to R2-D4 and R2-L1 to R2-L8 are review round 2's items as the session deduped them (head 4bccf55, 2026-11-13; the report's \"Reviews\"), from rust-reviewer (RR2-*), security-auditor (SA2-*), test-engineer (TE2-*) and engine-architect (EA2-*), R2-D1 to R2-D4 each tested by a skeptic of its own;"),
  sub("The round-1 items rest on the reviewers' and skeptics' notes (`review-r1-findings.json`, `review-r1-verdicts.json` and `skeptic-r1-D*.md` in",
    "The round-1 and round-2 items rest on the reviewers' and skeptics' notes (`review-r1-findings.json`, `review-r1-verdicts.json`, `skeptic-r1-D*.md`, `review-r2-findings.md`, `review-r2-verdicts.json`, `skeptic-r2-R2-D*.md`, the reviewers' `*-review-r2.md` and the advisor agent's `advisor-ruling-d1-r2.md` in"),
].reduce((acc, f) => f(acc), s));

const RP = 'docs/reports/2026-11-12-foundation-2-2.md';
once(RP, 'PR body), revisions 1 to 9.', 'PR body), revisions 1 to 10.');
// Plan and approval: the D1-R2 bullet after the D1 bullet.
once(RP, 'conditions are folded into plan revision 10 and this report.\n', 'conditions are folded into plan revision 10 and this report.\n' + R[11] + '\n');
// D2: the K1b and M5 bullet.
rewrap(RP, '- **K1b and M5** were added', (l) => !l.startsWith('  '), '- ', '  ',
  sub('M7 proves the setting the tree ships.', 'M7 proves that CI turns red when the shipped release line goes, and E12 (local) shows how far that line reaches.'));
// As built: append to the overflow bullet.
once(RP, "asked (Decisions needed); ADR 0002's own text is not edited, its owner rewords part 4 at the next revision.\n",
  "asked (Decisions needed); ADR 0002's own text is not edited, its owner rewords part 4 at the next revision.\n" + R[12] + '\n');
// D3 and D4: the M7 and E12 rows.
once(RP, 'after the dev-profile tests passed; the gate stayed green)', 'after the dev-profile tests passed; the gate stayed green)' + D3);
once(RP, 'the flag on `sim`, `room` and `sim_wasm`, none on', D4);
// D5: the Reviews corrections.
once(RP, "| setpriv's flags; the cut asserts none of its properties |", "| setpriv's flags; the cut asserts none of its privilege properties |");
once(RP, "| D13 (EA-2) | Low | the crates' doc comments | fixed: 1de98a9 |", "| D13 (EA-2) | Low | the crates' doc comments | fixed: 1de98a9 (the crates); REPO_MAP's row in round 2's fix |");
rewrap(RP, 'security-auditor found nothing above Low:', (l) => l.trim() === '', '', '',
  sub('the bounding set inert under no_new_privs).', 'the bounding set inert under no_new_privs), shown locally by a run as root (util-linux 2.39.3) and, for the runner, by capabilities(7).'));
// Reviews: round 2's paragraph, run lines and table.
lines(RP, 'Round 2, on what changed (step 7):', () => true, R[13]);
// D6: Risks.
lines(RP, "- With route A, an overflow in `room`'s or the future server's own arithmetic panics in release instead of wrapping, as",
  (l) => !l.startsWith('  '), R[4]);
// D7: OVERFLOW-XCRATE-CI's row.
lines(RP, '| OVERFLOW-XCRATE-CI |', () => true, R[5]);
// D8 and D9: OVERFLOW-PER-CRATE, then OVERFLOW-PANIC-RULE.
lines(RP, '- OVERFLOW-PER-CRATE: workspace crates are covered', (l) => !l.startsWith('  '), R[6] + '\n' + R[7]);
// Q5: the decision entry, up to its Options line.
lines(RP, '- **OVERFLOW-ALL: may overflow checks stay on', (l) => l.startsWith('  - Options: (a) yes:'), R[1]);

if (dry) { console.log('dry run: every target matched once; files that would change:', Object.keys(files).join(', ')); process.exit(0); }
for (const [f, t] of Object.entries(files)) fs.writeFileSync(f, t);
console.log('written:', Object.keys(files).join(', '));

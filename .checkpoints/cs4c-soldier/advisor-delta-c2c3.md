# Advisor agent: departures of C2 and C3 from the build order (2026-11-28)

## Plan
1. Read the build order (C2, C3, C4), my delta verdict (G5, G10), rust-reviewer blocks 16 to 18, security-auditor-c3.md,
   the card's C2/C3 story.
2. Spot-check at the head: fmath.rs (exp_of, sincos_of, share_per_tick's signature), rules_scan.rs (RAW), marking.rs
   (`pub` before `mod`), devtools libm_ops.rs, libm_vectors.rs (write only under SHAMBLE_WRITE_LIBM=1), the header,
   first-loop.hashes against C1, ADR 0004 PM-0004-22's words, maths-allowed.mjs, CI on 9e9a1cc8.
3. Rule items 1 to 8; overall GO or HOLD for C4; DONE.

## Notes
- Checked: head 51d4bbcd (notes only after the code head 9e9a1cc8); worktree clean but this file.
- Checked: fmath.rs:39-51 `exp_of`, `sincos_of` hand on libm's own f64; :57-59 `share_per_tick(rate_per_s, dt_s)`
  calls `exp_of(-(rate*dt))`; :78 `facing_of` calls `sincos_of`. grep of crates/*.rs: the two names stand only in
  fmath.rs (own fn and one caller) and devtools/src/rules/libm_ops.rs:9,61,65 (the test-only vectors module).
  rules_scan.rs:109-110 MATHS_FILE/MATHS_CALLS on (exp_of, exp), (sincos_of, sincos); :118 RAW on (exp_of,
  share_per_tick), (sincos_of, facing_of); :558-560 the raw rule. rules.rs:220-221 easing rate LN_10/T, :235 push
  strength as written.
- Checked: rust-reviewer 11.5 had `share_per_tick(t_s, dt_s)` and the allowances on (share_per_tick, exp),
  (facing_of, sincos): both are departures, both answered YES in block 17 (Q1 with the raw-name rule, Q2).
- Checked: libm_vectors.rs:30 records only on SHAMBLE_WRITE_LIBM == "1"; libm-expected.txt:1 `shamble-libm-vectors 1
  libm =0.2.16 1077304 0ee2be59...d052`, then `exp 33646 ...`, `sincos 101017 ...` (exp is 33,646: block 17's 33,648
  less his two values, as the card says).
- Checked: ADR 0004:1143 PM-0004-22 asks bit-identical `to_bits` outputs natively, in Node and each browser engine on
  at least 10,000 committed inputs; a correctly rounded reference of `sqrt` only; the largest ulp error and libm's
  cost against std's are "reported". Its bite: one output moved by one ulp on one target (the card's PV1: Node red,
  native green). The recorded form meets the mark; the cost figure is not yet measured (a report item, not C4's).
- Checked: `git diff 14538bd7^ 14538bd7` on first-loop.hashes: one line changed, the first (`shamble-hashes 1 <rec
  sha256> 1800`); the 1,800 tick lines unchanged. `.rec` binary, `content.json` 1 line, manifest 3. Item 6 holds.
- Checked: marking.rs:194-201 takes `pub` with an optional balanced `(..)` before plain `mod NAME {`; nothing else is
  widened (fn, const, use, impl still refused). rules_scan.rs:929-935 CONTENT_BITES holds the former controls as
  bites with the narrow rule's reason; security-auditor ran the scan at 9e9a1cc8: 9 passed (real content tree green).
- Checked: build-checks.mjs:777-778 an entry covers itself and `entry::` children only, never a string-prefix sibling
  (`rem_pio2` does not cover `rem_pio2_large`). maths-allowed.mjs:7-12 says the sim's two join with C4.
  security-auditor-deps-4c-v2.md:85-88 F2: one entry per printed libm path, exact module paths, no sibling cover,
  never a `libm::math` prefix, each with its why. B17-6's ~14 dev-twin paths fit that shape.
- Checked: CI on 9e9a1cc8 (check runs): gate success, smoke success, toolchain in_progress (run 46169331285).

## Rulings
1. ACCEPTED. The allowances on `exp_of`/`sincos_of` with RAW (one file, own fn, one caller, test code included) and
   layering's refusal of both names in every other crate's src but sim and devtools are tighter than 11.5's form, not
   looser; F1 ("only sim names libm") holds (devtools names `sim::fmath::*`). The report names it as a departure from
   11.5 with block 17 Q1 as its ground.
2. ACCEPTED. Meets PM-0004-22 as written (bit-identical on every target; accuracy reported, not pinned). Conditions
   for the report: it says the expected digests are self-recorded natively and prove sameness across targets, not
   libm's fidelity to its algorithm (B18-2); it carries B17-2's accuracy figures with their method (block 18
   condition 3); libm's cost against std's (PM-0004-22's other "reported") is listed as not yet measured.
3. ACCEPTED. The report names the departure from 11.5's `t_s` signature (block 17 Q2: strength 0 gives exactly 0).
4. ACCEPTED. A visibility does not change what cfg(test) compiles out; security-auditor ruled it within its ruling
   and probed `pub(crate)` and `pub(in ..)`; a control covers it. No widening to fn, const, use or impl.
5. ACCEPTED. The narrow rule's design (a false red on the safe side); the real content tree is green under it.
6. ACCEPTED as a correction of my G5 wording, amended here: C2's evidence is the 1,800 tick lines and
   `first-loop.vector.txt` byte for byte, with the `.hashes` first line and the `.rec` header's bundle sha256 the only
   moves (both follow the bundle's re-freeze). The report states it in that form.
7. ACCEPTED as a correction of my G10 wording, amended here: "in C3 nothing in the players' build reaches exp or
   sincos" (B17-5 measured: release 0 libm names, dev 0 libm paths); the vectors' test-only Wasm module (outside the
   players' twins, B17-13) and devtools' native test call them by design.
8. ACCEPTED, the plan as stated, with these conditions in C4: (a) the entries are exactly the paths the real build's
   refusals print, for both twins, each an exact module path under build-checks.mjs's segment rule, no `libm::math`
   or other covering prefix; (b) any refused path outside libm's exp and sincos families (B17-5's positive control
   list), or a compiler_builtins or C-named one, is reported, not listed (G10's rule); (c) `rem_pio2_large`'s why
   says it is linked but unreachable on facing_of's [0, 360) domain (content refuses other headings, map.rs:66) and
   is the one target-dependent code (its table's length by pointer width); (d) security-auditor reads the real list
   in C4's commit against its F2 (four expected); a fix goes in its own commit before C5.

## Verdict: GO for C4
On: (1) the toolchain run on 9e9a1cc8 (46169331285, in progress at this read) green before C4 is pushed (C4's edits
may start now; standing rule); (2) item 8's conditions (a) to (d); (3) the report items of rulings 1, 2, 3, 6 and 7,
riding the next notes commit or the report, no push of their own. CD-4's departure (movement at version 2, no 1-to-2
migration) stays listed for Mustafa's call (VERSION-RULE condition 8); L5 stays deferred on security-auditor's three
conditions (named in the report or STATUS; local-checks.sh keeps the generator's --check before every push; any
change to libm-inputs.bin goes back to it). No Critical or High.

Not checked: nothing built or run (no gate, cargo, Node or browser); the plants' runs, the browser engines' 29 of 29,
the dev twin's 14-path count and the accuracy figures taken from the card and rust-reviewer's notes.
DONE

## Addendum (after the advisor tool's check)
- Checked: advisor-version-rule.md:41 condition 8 says what the brief quotes (the CD-4 departure listed for his call).
- Checked: advisor-steps1to7.md Q4 condition 2 (:142-144) names "the vectors (11.5)", whose form put the allowances on
  `share_per_tick`/`facing_of`. Amended here in writing: the vectors and allowances as block 17 Q1/Q5 and block 18 set
  them (rulings 1 and 2); the rest of condition 2 as G10 already amended it.
- F5 (the table comes back to its read): the owner already confirmed items 1 to 3 (rust-reviewer blocks 17, 18), and
  items 4 and 5 (security-auditor, C3 read (a) and item 5); 6 and 7 are my own wording. Condition: in the next notes
  commit the main session marks step1-build-order.md C2's "byte-identical", C3's allowance line and "only sim's tests
  call them", and C4's "expected four" entries as superseded (pointing to this file and B17-6); block 16's C2 and C3
  rows stay rust-reviewer's text, read through blocks 17 and 18 (no rewrite by another agent).
- Ruling 2, added condition (report): B17-2's accuracy figures were measured on the 33,648-input scratch set; the
  committed inputs hold 33,646 exp cases. The report re-measures on the committed libm-inputs.bin or says the figures
  are from the scratch set.
- CI re-polled at handback: 9e9a1cc8 gate success, smoke success, toolchain still in_progress: GO's condition (1)
  stands as worded.
DONE

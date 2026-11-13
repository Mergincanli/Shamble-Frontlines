# test-engineer, review round 2 of foundation 2.2 (PR #48): 4d78ad6..4bccf55 (excluding .checkpoints/)

Angle: is it proven? Only what changed since round 1's head 4d78ad6.
Scratch and builds: /home/user/Shamble-Frontlines/.gate/test-engineer-r2/ (nothing in the worktree but this file).
Agent memory: none in this environment (no .claude/agent-memory-local/); round-1 notes read
(.checkpoints/foundation-toolchain/test-engineer-review-r1.md).

## Plan (blocks; each marked DONE with its evidence when finished)
1. Read the diff 4d78ad6..4bccf55 (code: Cargo.toml, 4 crates' doc comments; docs: report, register row OVERFLOW-ALL,
   REPO_MAP, ROADMAP, STATUS, STATUS_ARCHIVE); the ruling advisor-ruling-d1.md (conditions 5, 7, 7e); ADR 0002 near
   :1319 (PM-0002-53) and the overflow parts; round-1 dedupe and verdicts for my own findings' dispositions.
2. M7: 297d48a touches only the release setting; 8d622ac exact revert (git diff 297d48a^ 8d622ac empty); CI runs
   43415093334 (job 131850863497) push and 43415101662 pull_request: which step is red, which binary, which message;
   compare with the report's M7 row word by word; does it claim to distinguish route A from part 4's blocks?
   Also: CI on 8d622ac and the head 4bccf55 (green? cancelled?).
3. Relabelled evidence: M5 and step 2's control marked historical; grep report/STATUS/register for every use of M5,
   "control", "step 2" and check nothing still leans on them for a current claim.
4. E12: read xcrate-check.sh and both outputs; check "before" really is the pre-fix tree (what did the script archive,
   when; compare the archive's Cargo.toml with 4d78ad6/1de98a9's); re-run on a scratch copy of 1de98a9 and of 4bccf55,
   native and wasm32; check the probe can fail (a caller crate's inline/generic sim code, debug vs release,
   wasm32 run or build only).
5. OVERFLOW-XCRATE-CI anchor: wording in report/register/STATUS; is there any mechanism (CI, gate, checklist) that
   fires on "the first change that adds a dependency on sim"; who notices.
6. PM-0002-53 placement: ADR 0002:1319 text vs the report's and the register row's text; testable as written?
7. Remaining new rows and dispositions (round-1 findings TE-1..TE-3 and others' that changed text); facts in E12/M7.
8. Findings, claim/proof/gap table, verdict; advisor tool before finalising.

## Findings (a line each, as found)

### Block 1 DONE (diff read)
- Code: Cargo.toml:21-29 (two route-A tables + comment), four crates' doc comments only (1de98a9). The worktree HEAD is
  4bccf55; commits aee92cc, 8c397ac between 1de98a9 and 4bd9dd5 touch only .checkpoints/advisor-ruling-d1.md.
- ADR 0002 part 3 (:186) says `protocol` depends on "`sim` and `content` types only"; room, client-endpoint, sim-wasm,
  server, navbake depend on sim. Today no crate depends on anything (six manifests: no [dependencies]).

### Block 2 DONE (M7; logs under /home/user/Shamble-Frontlines/.gate/test-engineer-r2/ci/; gh api, read-only: the
### GitHub MCP tools were not loadable in this agent, no ToolSearch)
- 297d48a touches only Cargo.toml: removes `[profile.release]` + `overflow-checks = true` (+ blank line), keeps "*".
  `git diff 297d48a^ 8d622ac` is empty even with .checkpoints/ (297d48a^ = 8c82be2). Exact revert: holds.
- Push 43415093334 (job 131850863497) and pull_request 43415101662 (job 131850893215), both on 297d48a, attempt 1,
  failure at step 7 inside the cut (uid 1001 nogroup only, only lo, curl (7)); dev `cargo test --workspace` 6 x
  "1 passed"; then release: protocol "overflow_checks_are_on - should panic ... FAILED", "note: test did not panic as
  expected at crates/protocol/src/lib.rs:10:8", exit 101. sim's release binary never ran (cargo stops at the first
  failing binary). Gate push 43415093339 / PR 43415101832 green on 297d48a.
- Revert 8d622ac: toolchain push 43415458569 (job 131852089087) and PR 43415465726 (job 131852114268) green; inside the
  cut both release binaries ran: protocol ... ok, sim ... ok; 16 "test result: ok"; Playwright 1 passed; tsc 6.0.3.
  Gate 43415458525 / 43415465755 green. Every run id in the M7 row matches its SHA, event and conclusion.
- CI on later heads: 8a4f9b0 toolchain 43415740925 / 43415748074, gate 43415740367 / 43415748089; 4bccf55 toolchain
  43424298848 / 43424306336, gate 43424299180 / 43424306390: all green, attempt 1. No cancelled run since 4d78ad6.

### Block 4 DONE (E12; my copies under /home/user/Shamble-Frontlines/.gate/test-engineer-r2/orig-before|orig-after)
- "Before" is the pre-fix tree: the session's folder .gate/xcrate-before-1de98a9/ws has the per-package blocks and file
  mtimes 01:10:10 = 1de98a9's commit time (git archive stamps the commit time); after-4bd9dd5 has route A, 01:31:43 =
  4bd9dd5's. The early run .gate/xcrate-fixed (01:31:21) archived 8c397ac (HEAD before the fix commit; mtime 01:30:01)
  and so tested the OLD blocks under a "fixed" name; it is cited nowhere (report, register, STATUS: grep below).
  1de98a9..4bd9dd5 differs only in Cargo.toml (non-checkpoint files), so before/after isolates the setting.
- The unmodified script re-run by me on 1de98a9 and 4bd9dd5: both outputs byte-identical to the recorded files (diff).
- Raw (unfiltered) results: before, release/bench/relcustom each "2 passed; 4 failed", the 4 = "test did not panic as
  expected" (room lib.rs:9, 15, 21, 27), never_inline ok; after, each "6 passed; 0 failed". The recorded files show
  neither "ok" nor the summary line (grep '^test tests' + --nocapture pushes "ok" to another line): a reader must read
  a blank as a pass. Format only; the claim holds.
- wasm32 control (my wasm-control.js, not in the script): non-overflowing calls (1+1, tick 5) return 2 and 6 in all
  four builds, before and after; overflowing calls trap in dev (both) and release-after, return 0 in release-before.
  So the release traps after the fix are the overflow check, not some other trap. The script has no such control.
- rustc's flag per crate after a clean: after, native release --workspace AND wasm32 release (sim, protocol, room,
  client-endpoint, sim-wasm, wasmcheck): every member on (client_endpoint, devtools, protocol, room, sim, sim_wasm,
  wasmcheck), extdep off. Before: sim and protocol only, native and wasm32. E12's three-crate flag list understates it;
  "every workspace crate" holds for all six real members, both targets.

### Block 3 DONE (relabelled evidence)
- M5 and step 2's control appear only at report :69-71 (As built: "historical now; M7 proves the setting the tree
  ships"), :106 (E1: "Historical, for the per-package blocks ...; E12 and M7 prove the shipped setting"), :115 (M5 row
  "(historical)") and :131 (the note: M5 ran on the merged tree). STATUS, the archive, the register, REPO_MAP and ROADMAP
  name neither; the archive's OVERFLOW-WASM ("sim's rests on step 2's local control") is gone, replaced by
  OVERFLOW-XCRATE-CI. Nothing current leans on them. Holds.

### Block 5/6 (anchor; PM-0002-53 placement)
- No tripwire: toolchain.yml has no dependency-graph step; the crate manifests are unguarded (PIN-GUARD); the legacy
  gate ignores crates/. STATUS.md:95-96 lists OVERFLOW-XCRATE-CI by name under "From foundation 2.2 (PR #48), for 2.3";
  the archive files it under "For the gate step (2.3)" (:617); the report under "Findings for 2.3" (:231). The first
  prong ("the first change that adds a dependency on sim") is written only inside the item's text.
- Mustafa's order (ADR 0002:1442 pointer: "Steps 2.3 to 2.6 keep their order") and CLAUDE.md (no v2 code but 2.1/2.2)
  put 2.3 first, so 2.3 is the binding trigger and its planner reads the 2.3 lists. But at 2.3 no crate depends on sim
  and sim is empty (code enters at 2.5), so 2.3 can land the test only by adding a dependency on sim and a planted
  counter in sim's normal build.
- ADR 0002 part 3 (:186): protocol depends on sim types. K1b already runs `-p protocol` in release, so a counter test
  in protocol needs no CI edit for its native half once protocol depends on sim; the wasm32 half does.
- Experiment (orig-before, the OLD blocks): an integration test in crates/sim/tests/ calling sim::add_inline and
  add_generic with 255+1, `cargo test --release -p sim --test xc`: 2 passed (both panicked). So "a test inside sim's
  own package cannot see the gap" holds, integration tests included (a separate crate, the same package override).
- Experiment (lintcheck, 4bd9dd5 + a `#[unsafe(no_mangle)]` export in sim-wasm, cdylib): `cargo build --release
  --target wasm32-unknown-unknown -p sim-wasm` -> "error: usage of the unsafe `no_mangle` attribute ... -D
  unsafe-code". E12's export method works only because its scratch crate skips `[lints] workspace = true`; a committed
  wasm32 half needs part 4's unsafe exception (invariant, test, #[allow] with reason) or wasm-bindgen + a runner.
- Design constraint not stated in the row: the planted counter must be in sim's normal build (cfg(test) items are not
  visible to dependents; a test-only feature breaks part 3's one feature set; part 4 makes real counters use
  wrapping_*/checked_*, so the `+` counter is planted scaffolding).
- Experiment (orig-after, route A, release; extdep = crate outside the workspace, called from room): plain Ok(0),
  inline Ok(0), concrete-in-generic Ok(0), trait-generic `T: Add` Err (panicked). Test profile: all four panic.
  So Cargo.toml:24 "dependencies keep the release default, unchecked" is wrong for a dependency's trait-generic
  arithmetic instantiated in a workspace crate (core's operator impls follow the compiling crate). The register row's
  "can still differ between dev and release builds" holds. Only Cargo.toml:24 says "unchecked" (grep).

### Block 7 (dispositions and rows)
- TE-1 fix: the four quick reverts 2ee2bff, 5c64404, b60a996, 00cee0b each equal 66d9888 outside .checkpoints/ (git
  diff --stat empty). "their tree is 66d9888's, which is green" holds. TE-3 (D17): G0's row lists the other branches
  as mutations; CUT-CARRY adds the --net-removed red run. Holds.
- Report :40-41 "revisions 1 to 9" vs :49 "folded into plan revision 10"; plan.md:1 is revision 10. Stale.
- E12 row: "(the notes)" resolves through the report's convention (:40 "readable at this branch's last checkpoint
  commit, named in the PR body"). OK, if the PR body names it.

### Extra checks DONE
- E1 on the head (fresh git archive of 4bccf55 = 4bd9dd5 for every Rust file): dev 6 x "1 passed"; K1b release 2/2;
  bench sim+protocol 2/2; wasm32 dev build of the five crates (5 rlibs); clippy -D warnings clean; fmt clean;
  Cargo.lock byte-identical after all runs; Cargo.lock unchanged 4d78ad6..4bccf55.
- Reach under the OLD blocks (orig-before, release): sim::add_inline called from room Ok(0); the same helpers reached
  through a sim `#[inline(never)] entry` panic (u8 and tick); control (no overflow) Ok((6, 6, 6)). So the RR-1 gap is
  sim code the caller compiles itself (inline, small, generic, called directly), not sim code behind sim's own
  non-inlined functions. The register row (:234) and As built (:72-74) say this right; the OVERFLOW-ALL decision entry
  (:298-300) says "misses most of the simulation's code as soon as another crate calls it": unmeasured "most".

## Draft findings (before the advisor tool)
- TE2-1 Low Cargo.toml:24 "unchecked" wrong for trait-generic dependency arithmetic instantiated in a member.
- TE2-2 Low OVERFLOW-XCRATE-CI's first prong: no tripwire; filed only under 2.3 headings; at 2.3 no crate depends on sim.
- TE2-3 Low PM-0002-53 placement right but omits: planted counter in sim's normal build; wasm32 runner (no_mangle denied).
- TE2-4 Low E12 record: no "ok"/summary lines, no wasm32 control, flag list understated (all close on my re-run).
- TE2-5 Low report :40-41 "revisions 1 to 9" (plan is revision 10, :49).
- TE2-6 Low M7 row: names no failing binary (protocol's), no word that sim's release binary never ran in CI.
- TE2-7 Low OVERFLOW-ALL decision entry :298-300 "most of the simulation's code": unmeasured; mechanism says narrower.
- Verdict draft: APPROVE, 0 Critical, 0 High, 0 Medium, 7 Low.

### Advisor tool consulted (after the drafts); its points applied
- Line numbers re-grepped: ADR 0002 protocol row is :188 (not :186); the "most" sentence is report :295-296; As built
  :72-78. Both probes re-run with panic messages kept: DEP's only panic "attempt to add with overflow" at
  extdep/src/lib.rs:7 (add_trait's `a + b`); REACH's two at sim lib.rs:24 (add_inline) and :40 (Tick::next).
- D3/D17 judged against the skeptics' refined fixes (review-r1-verdicts.json): D3's K1-GO wording matches its skeptic's
  suggestion, the trees named as 66d9888's; D17's mutations sit in G0's row ("beyond PM-0002-39's file deletion") and
  CUT-CARRY ("a red run with --net removed"); plan.md's "already show that the probe bites" is in no tracked doc.
- STATUS.md:29 read in full (M7 "went red and was reverted"; 17 findings "all hold"): holds. The 2.2 follow-up line
  lost its "(archive: ...)" tail, but :69 says "its text in the archive". HEAD 4f1a799 = 4bccf55 outside .checkpoints/.
- .gate/xcrate-fixed is cited nowhere (grep of .checkpoints/ and docs/). Main's advisor.md has no step that matches a
  plan's files against open follow-ups (:17 read STATUS; :169 STATUS updated).

## FINAL (the handback, as sent)
Verdict: APPROVE. 0 Critical, 0 High, 0 Medium, 7 Low (TE2-1..TE2-7; text in the handback). Highest value: TE2-7
(the decision entry Mustafa reads) and TE2-3 (what PM-0002-53's dependent-crate test needs) ride the fold commit.

### Findings (final text)
- TE2-1 Low, Cargo.toml:24. "so dependencies keep the release default, unchecked": "unchecked" is wrong for a
  dependency's trait-generic integer arithmetic instantiated in a workspace crate. Evidence: route A, release, extdep
  (outside the workspace) called from room: add Ok(0), #[inline] Ok(0), concrete + inside a generic fn Ok(0),
  add_trait::<u8> (T: Add) panics "attempt to add with overflow" (extdep lib.rs:7); test profile: all four panic. The
  ruling (advisor-ruling-d1.md:163-165) asked for "keep the release default setting" and called this untested. The
  register row's "can still differ between dev and release builds" holds. Fix: drop ", unchecked" (next Cargo.toml
  change, or the fold if Cargo.toml is touched anyway); give OVERFLOW-PER-CRATE (STATUS_ARCHIVE.md:779-784, report
  follow-ups) the split for the "*" decision.
- TE2-2 Low, report :231; STATUS.md:95-96; STATUS_ARCHIVE.md:572, :617-631. The anchor's first prong has no tripwire
  and is filed only under 2.3 headings. Facts both ways: ADR 0002:1442-1443 "Steps 2.3 to 2.6 keep their order" (2.3
  first, its planner reads its list) vs the ruling's R6 (advisor-ruling-d1.md:44-46: first cross-crate call from the
  next builder, "possibly before 2.3's gate"). At 2.3 no crate depends on sim and sim is empty. No CI/gate/guard check
  reads the dependency graph (PIN-GUARD); advisor.md has no follow-up match step. ADR 0002:188: protocol depends on sim
  types; K1b runs -p protocol in release, so a native counter test there needs no CI edit; wasm32 does. Fix (docs,
  fold): name the first prong in STATUS.md:96; say what 2.3 does if no crate depends on sim yet.
- TE2-3 Low, docs/adr/README.md:234; report :231; STATUS_ARCHIVE.md:627-631. The placement is right (old blocks: an
  integration test in crates/sim/tests/ panics, 2 passed) but omits: (1) the planted counter must be in sim's normal
  build (cfg(test) invisible to dependents; part 3's one feature set; part 4's wrapping_/checked_ counters); (2) no
  wasm32 runner in the tree: E12's #[unsafe(no_mangle)] export is refused by the workspace lints ("usage of the unsafe
  `no_mangle` attribute", -D unsafe-code, sim-wasm on 4bd9dd5): part 4's unsafe exception or wasm-bindgen + a runner;
  "(a CI edit: it asks him)" undercounts. "in release, native and wasm32" can read as replacing the four builds. Fix:
  docs, fold.
- TE2-4 Low, report :118; xcrate-check.sh:152, :159-168. The record shows less than E12 claims: no "ok"/summary lines,
  no wasm32 no-overflow control, a three-crate flag list. My re-runs close all three (before 2 passed 4 failed, after 6
  passed 0 failed; controls 2 and 6 in four builds; every member flagged, native and wasm32). Fix: keep summary lines
  and add the control at the next run; E12 may say "every workspace member".
- TE2-5 Low, report :40-41. "revisions 1 to 9" vs :49 and plan.md:1 (revision 10). Fix: "1 to 10".
- TE2-6 Low, report :117. M7 names no failing binary and does not say sim's release binary never ran in CI (job
  131850863497: only protocol's ran). Holds (one line governs both; sim's side: the ruling's C3, local). Fix: say so.
- TE2-7 Low, report :295-296. "misses most of the simulation's code" is unmeasured (no sim code; 4 of 5 kinds tried),
  and a counter-case is shown: old blocks, release, sim helpers reached through a non-inlined sim function panic (sim
  lib.rs:24, :40) while the same helper called directly wraps; control (6, 6, 6). Register row and As built word it
  right. Fix before the question goes to Mustafa (fold): "misses the simulation's small, inline and generic functions
  whenever another crate calls them directly (tick counters, maths helpers)".

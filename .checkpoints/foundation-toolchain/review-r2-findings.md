# Review round 2, PR #48 (foundation 2.2): the findings, deduped by the session

Head reviewed: 4bccf55 (4d78ad6..4bccf55 outside `.checkpoints/`). Reviewers, Opus 5.5 at max: rust-reviewer (RR2-*),
security-auditor (SA2-*), test-engineer (TE2-*), engine-architect (EA2-*, pending when this file was first written).
Full texts: `rust-reviewer-review-r2.md` (:157-230), `security-auditor-review-r2.md` (:139-200); test-engineer's notes
hold one line per finding (:123-129), so its full texts are copied below from its handback. Verdicts: rust-reviewer
APPROVE (0/0/1/3), security-auditor 0/0/2/1, test-engineer APPROVE (0/0/0/7).

## Deduped items

| Item | Severity (highest merged) | Merges | The session's one-line reading | Check |
|---|---|---|---|---|
| R2-D1 | Medium | RR2-1, TE2-1, RR2-2 | Cargo.toml:23-24's comment ("*" matches only crates outside the workspace, so dependencies keep the release default, unchecked) and the records that repeat it (the register row OVERFLOW-ALL, README:234; the report :19-20, :76-77, :254-257 OVERFLOW-PER-CRATE; REPO_MAP:16; STATUS_ARCHIVE OVERFLOW-PER-CRATE) are wrong in three ways: (RR2-1) at opt-levels 0, 1, "s", "z" a member reuses a dependency's unchecked copy of core's integer `Sum`/`Product` (share-generics), so the member's own `sum()` wraps; (TE2-1) a dependency's trait-generic arithmetic instantiated in a member is checked in release; (RR2-2) "*" follows membership, not folders | skeptic |
| R2-D2 | Medium | SA2-1 | the report's Risks :213-215 and OVERFLOW-ALL :299-302 say an overflow in room's or the server's code "would likely end that match (inferred)" and Risks ends with an IOU to round 2; the ADRs give three outcomes (tick thread: the match ends; one connection: only it closes; browser: a trap ends that player's instance, online it resyncs) | skeptic |
| R2-D3 | Medium | SA2-2 | no record hands route A's panic side to the ADR owners: part 4's explicit-operations rule covers only sim's and protocol's counters and tick numbers; part 20 names no overflow; nothing ties a rule for room and server code to room's first code; fix: a follow-up OVERFLOW-PANIC-RULE plus a clause in the row | skeptic |
| R2-D4 | Low | TE2-2, TE2-3 | OVERFLOW-XCRATE-CI's text: its first trigger ("the first change that adds a dependency on sim") has no tripwire and is filed only under 2.3 headings; PM-0002-53's new home omits two constraints (the planted counter in sim's normal build; no wasm32 runner, `#[unsafe(no_mangle)]` refused by `-D unsafe-code`) | skeptic |
| R2-L1 | Low | TE2-6, RR2-3 | the M7 row names no failing binary (protocol's) and does not say sim's release binary never ran in CI; :71 "M7 proves the setting the tree ships" reads wider than M7 shows | the session, inline: holds (the M5 row :115 states the limit, M7's :117 does not) |
| R2-L2 | Low | TE2-7 | OVERFLOW-ALL's "misses most of the simulation's code" is not measured; E12 tried 5 kinds of function, 4 missed | the session, inline: holds |
| R2-L3 | Low | TE2-4 | E12's saved record is thin (no "ok" or summary lines, no wasm32 no-overflow control, flags on 3 crates named, not every crate) | two reviewers' re-runs close all three; the advisor agent's final-head re-run keeps the summary lines |
| R2-L4 | Low | TE2-5 | report :40-41 "revisions 1 to 9"; the plan is revision 10 (:49, plan.md:1) | the session, inline: holds |
| R2-L5 | Low | SA2-3 | Reviews :148 (D5 row "asserts none of its properties") and :162-164 ("confirmed ... the capability sets") overstate round 1 | the session, inline: holds (D5 skeptic (2): only the privilege properties; the probe asserts the network one) |
| R2-L6 | Low | RR2-4 | sim-wasm/src/lib.rs:2-3 "wasm-bindgen's by-day reading (OQ-0002-7) comes first" leaves "first" open (it can read as before the boundary spike, against EMPTY-SLOTS); D13 is marked fixed though EA-2's REPO_MAP:18 half is not applied | the session, inline |

## test-engineer's full texts (from its handback, 2026-11-13)

TE2-1 · Low · `Cargo.toml:24`. Defect: the comment "so dependencies keep the release default, unchecked" is wrong in one
case: a dependency's generic arithmetic (`T: Add`), when one of our crates instantiates it, is checked in release.
Evidence: its probe under route A, in release, `extdep` a crate outside the workspace called from room: plain `add`
Ok(0) (wraps); `#[inline]` Ok(0); concrete `+` inside a generic function Ok(0); `add_trait::<u8>` (`T: Add`) panics,
"attempt to add with overflow", at `extdep/src/lib.rs:7`. In the test profile all four panic. The ruling itself asked
for the wording "keep the release default setting" and called dependency generic code untested
(`advisor-ruling-d1.md:163-165`). Fix: drop ", unchecked" at the next `Cargo.toml` change; give OVERFLOW-PER-CRATE this
split, for the `"*"` decision. Its probe trees: `/home/user/Shamble-Frontlines/.gate/test-engineer-r2/orig-after/`
(changed after its byte-identical run: `sim/tests/xc.rs`, sim's `entry`, room's `lib.rs`, extdep's `lib.rs`).

TE2-2 · Low · report :231; `STATUS.md:95-96`; `STATUS_ARCHIVE.md:572`, :617-631. Defect: OVERFLOW-XCRATE-CI's first
trigger ("the first change that adds a dependency on sim") has no tripwire; the item is filed only under 2.3 headings,
so a builder of that change would not see that it applies to them. Evidence: no CI, gate or guard check reads the
dependency graph; the manifests are unguarded (PIN-GUARD); main's `advisor.md` has no step that matches a plan's files
against open follow-ups. Mustafa's order "Steps 2.3 to 2.6 keep their order" (ADR 0002:1442-1443), so 2.3 likely comes
first; the ruling's R6 (`advisor-ruling-d1.md:44-46`) expects the first cross-crate call from the next builder,
"possibly before 2.3's gate". At 2.3 no crate depends on sim and sim is empty, so 2.3 could land the test only by adding
a dependency on sim and a planted counter. ADR 0002:188 makes protocol depend on sim's types, and K1b already runs
`-p protocol` in release, so a counter test in protocol needs no CI edit for its native half; the wasm32 half does.
Fix (docs): name the first trigger in `STATUS.md:96`; say in the item what 2.3 does if no crate depends on sim yet.

TE2-3 · Low · `docs/adr/README.md:234`; report :231; `STATUS_ARCHIVE.md:627-631`. Defect: PM-0002-53's new home ("from
a crate that depends on sim") is right, but the text leaves out two constraints that decide whether the test can be
written. Evidence: under the old blocks an integration test in `crates/sim/tests/` passes (2 passed), so a test inside
sim's package cannot see the gap. Constraint 1: the planted counter must be in sim's normal build: `#[cfg(test)]` items
are invisible to dependent crates; a test-only feature breaks part 3's one feature set; part 4 makes real counters use
`wrapping_*`/`checked_*`. Constraint 2: the tree has no wasm32 runner: E12's `#[unsafe(no_mangle)]` export is refused by
our own lint table ("usage of the unsafe `no_mangle` attribute", `-D unsafe-code`, shown in sim-wasm on 4bd9dd5), so
the wasm32 half needs part 4's unsafe exception, or wasm-bindgen plus a test runner; "(a CI edit: it asks him)"
undercounts the cost. "In release, native and wasm32" can be read as replacing the four builds PM-0002-53 asks for.
Fix: docs.

TE2-4 · Low · report :118; `xcrate-check.sh:152`, :159-168. E12's saved record shows less than E12 claims: no "ok" or
summary lines (with `--nocapture` and the grep a blank stands for a pass); the wasm32 calls have no no-overflow control
(a trap carries no message); it names only 3 crates as getting rustc's overflow flag. Its re-runs close all three:
unfiltered "2 passed; 4 failed" before, "6 passed; 0 failed" after; controls 1+1 = 2 and tick 5 = 6 in all four wasm32
builds; after a clean every crate gets the flag, native and wasm32. Fix: keep the summary lines and add the control at
the next run (the advisor's final-head re-run); E12 can say "every workspace crate".

TE2-5 · Low · report :40-41: "revisions 1 to 9", but :49 and `plan.md:1` say revision 10. Fix: "1 to 10".

TE2-6 · Low · report :117: the M7 row does not say which test binary failed (protocol's), nor that sim's release binary
never ran in CI; the M5 row says both. Job 131850863497: only protocol's binary ran ("did not panic as expected at
crates/protocol/src/lib.rs:10:8"). The claim still holds; that sim also fails without the line rests on the ruling's
local C3 run. Fix: say so in the row.

TE2-7 · Low · report :295-296: the OVERFLOW-ALL decision entry says the old setting "misses most of the simulation's
code", which is not measured. No sim code exists; E12 tried 5 kinds of function and 4 were missed. Counter-case under
the old blocks in release: sim's helpers reached through a sim function that is not inlined panic ("attempt to add
with overflow" at sim `lib.rs:24` and :40); the same helper called directly from room wraps; the control returns (6, 6,
6). The register row and As built (:72-78) word it right. Fix, before the question goes to Mustafa: "misses the
simulation's small, inline and generic functions whenever another crate calls them directly (tick counters, maths
helpers)".

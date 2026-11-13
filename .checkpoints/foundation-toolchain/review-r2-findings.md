# Review round 2, PR #48 (foundation 2.2): the findings, deduped by the session

Head reviewed: 4bccf55 (4d78ad6..4bccf55 outside `.checkpoints/`). Reviewers, Opus 5.5 at max: rust-reviewer (RR2-*),
security-auditor (SA2-*), test-engineer (TE2-*), engine-architect (EA2-*; it returned after the first skeptics started, and its items were added here then).
Full texts: `rust-reviewer-review-r2.md` (:157-230), `security-auditor-review-r2.md` (:139-200); test-engineer's notes
hold one line per finding (:123-129), so its full texts are copied below from its handback. Verdicts: rust-reviewer
APPROVE (0/0/1/3), security-auditor 0/0/2/1, test-engineer APPROVE (0/0/0/7), engine-architect APPROVE (0/0/1/3; notes
`engine-architect-review-r2.md`, its full texts copied below from its handback).

## Deduped items

| Item | Severity (highest merged) | Merges | The session's one-line reading | Check |
|---|---|---|---|---|
| R2-D1 | Medium | RR2-1, TE2-1, RR2-2 | Cargo.toml:23-24's comment ("*" matches only crates outside the workspace, so dependencies keep the release default, unchecked) and the records that repeat it (the register row OVERFLOW-ALL, README:234; the report :19-20, :76-77, :254-257 OVERFLOW-PER-CRATE; REPO_MAP:16; STATUS_ARCHIVE OVERFLOW-PER-CRATE) are wrong in three ways: (RR2-1) at opt-levels 0, 1, "s", "z" a member reuses a dependency's unchecked copy of core's integer `Sum`/`Product` (share-generics), so the member's own `sum()` wraps; (TE2-1) a dependency's trait-generic arithmetic instantiated in a member is checked in release; (RR2-2) "*" follows membership, not folders | skeptic |
| R2-D2 | Medium | SA2-1 | the report's Risks :213-215 and OVERFLOW-ALL :299-302 say an overflow in room's or the server's code "would likely end that match (inferred)" and Risks ends with an IOU to round 2; the ADRs give three outcomes (tick thread: the match ends; one connection: only it closes; browser: a trap ends that player's instance, online it resyncs) | skeptic |
| R2-D3 | Medium | SA2-2 | no record hands route A's panic side to the ADR owners: part 4's explicit-operations rule covers only sim's and protocol's counters and tick numbers; part 20 names no overflow; nothing ties a rule for room and server code to room's first code; fix: a follow-up OVERFLOW-PANIC-RULE plus a clause in the row | skeptic |
| R2-D4 | Low | TE2-2, TE2-3, EA2-4 | OVERFLOW-XCRATE-CI's text: its first trigger ("the first change that adds a dependency on sim") has no tripwire and is filed only under 2.3 headings; PM-0002-53's new home omits two constraints (the planted counter in sim's normal build; no wasm32 runner, `#[unsafe(no_mangle)]` refused by `-D unsafe-code`); (EA2-4) where the wasm32 half can live is a part 3 question (room and client-endpoint take no wasm-bindgen; PM-0002-44 plants wasm-bindgen in client-endpoint; sim-wasm holds only bindings), and the test crate must be a root-workspace member | skeptic |
| R2-L1 | Low | TE2-6, RR2-3 | the M7 row names no failing binary (protocol's) and does not say sim's release binary never ran in CI; :71 "M7 proves the setting the tree ships" reads wider than M7 shows | the session, inline: holds (the M5 row :115 states the limit, M7's :117 does not) |
| R2-L2 | Medium (EA2-1) | TE2-7, EA2-1 | OVERFLOW-ALL's "misses most of the simulation's code" is not measured; E12 tried 5 kinds of function, 4 missed; (EA2-1) it words RR-1 wider than RR-1 did, disagrees with the row his go covers, and :294 calls protocol "the simulation" | two reviewers converged independently; the session, inline |
| R2-L3 | Low | TE2-4 | E12's saved record is thin (no "ok" or summary lines, no wasm32 no-overflow control, flags on 3 crates named, not every crate) | two reviewers' re-runs close all three; the advisor agent's final-head re-run keeps the summary lines |
| R2-L4 | Low | TE2-5 | report :40-41 "revisions 1 to 9"; the plan is revision 10 (:49, plan.md:1) | the session, inline: holds |
| R2-L5 | Low | SA2-3 | Reviews :148 (D5 row "asserts none of its properties") and :162-164 ("confirmed ... the capability sets") overstate round 1 | the session, inline: holds (D5 skeptic (2): only the privilege properties; the probe asserts the network one) |
| R2-L7 | Low | EA2-2 | the OVERFLOW-ALL entry asks only about the setting, but his go for the row also records two rules (dependencies at the release default until the first third-party crate is decided; PM-0002-53's test moved to a crate that depends on sim) | the session, inline |
| R2-L8 | Low | EA2-3 | the row's list of what ADR 0002's owner rewords (part 4, Appendix A's R1-RR-OQ4 answer, PM-0002-53) leaves out Appendix A's row "R5-RUSTLINT-REC4 \| adopted \| part 4" (0002:1586), whose reason route A reverses | the session, inline |
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

## engine-architect's full texts (from its handback, 2026-11-13)

EA2-1 · Medium · report :295-296 (the OVERFLOW-ALL decision entry); concurs with TE2-7 (Low there). "A review found
that this misses most of the simulation's code as soon as another crate calls it" states a share nobody measured and
words the review's finding wider than the review did: RR-1 says "sim's small, #[inline] and generic functions are
generated in the crate that calls them" (review-r1-findings.json); the row (README:234), Cargo.toml:22 and As built
(report :74) all say "inline, generic and small functions"; no sim code exists. In the same paragraph :294 says "turns
on overflow checks for the simulation by naming two crates", but protocol is the message codec. Medium, not Low: it is
the text Mustafa decides from, and it disagrees with the row his go covers. Fix: "misses the simulation's small, inline
and generic functions whenever another crate calls them, because that code is then compiled inside the caller"; :294
"for the simulation and the message protocol, by naming their two crates".

EA2-2 · Low · report :293-310 vs README:234. The entry asks only about the setting, but his go "for this row" also
records two rules the entry never mentions: dependencies keep the release default until the first third-party crate
enters sim's dependency tree (decided then, its cost measured); PM-0002-53's counter test moves from "a committed test in
sim" (ADR 0002:1319) to a crate that depends on sim. CLAUDE.md asks every decision entry to be self-contained. Fix: one
sentence naming both rules.

EA2-3 · Low · README:234 (the row's state column; a gap in the ruling's condition 3, copied faithfully). The list of
what is reworded at ADR 0002's next revision names part 4, the Appendix A answer for R1-RR-OQ4 and PM-0002-53, and
leaves out Appendix A's row for R5-RUSTLINT-REC4 (0002:1586 "R5-RUSTLINT-REC4 | adopted | part 4"), whose
recommendation (r5-toolchain-quality.md:1353-1358) checks sim and protocol only because "a per-package override is
allowed for this setting (F13, inferred), so the rest of the workspace keeps the release default"; route A reverses that
reason. Fix: "part 4, its Appendix A rows for R1-RR-OQ4 and R5-RUSTLINT-REC4 (adopted becomes changed) and PM-0002-53".

EA2-4 · Low · README:234 and report :231 (OVERFLOW-XCRATE-CI; STATUS_ARCHIVE :617-631); concurs with TE2-3. "Runs from a
crate that depends on sim, in release, native and wasm32" leaves open where the wasm32 half can live under part 3: room
and client-endpoint take no wasm-bindgen (0002:202-205); G5's PM-0002-44 (:1310) plants exactly "wasm-bindgen ... added
to client-endpoint", so a wasm-bindgen-test dev-dependency there depends on whether 2.3's G5 counts dev-dependencies
(undecided); sim-wasm holds "the wasm-bindgen bindings ... and nothing else" (:192) and has none until the boundary
spike; a raw export is refused by the workspace lints (test-engineer's run), so it would need part 4's unsafe
exception; the anchor can fall at 2.3, before sim-wasm has bindings (TE2-2). The test crate must be a root-workspace
member ("Cargo only looks at the profile settings in the Cargo.toml manifest at the root of the workspace", the Cargo
Book). Fix: the row and the follow-up say "a root-workspace crate that depends on sim; where its wasm32 half lives is a
part 3 question for ADR 0002's owner, settled in 2.3's plan (the native half first if needed)".

Concurrences (not counted again): SA2-2: route A's panic side belongs in the row's list of what gets reworded;
extending part 4's explicit-operations rule to every workspace crate is engine-architect's to reword. RR2-1's wider
"*" trigger also covers its note that the anchor names only sim's tree while part 4 names sim and protocol. RR2-4 (a)
and (b): the same.

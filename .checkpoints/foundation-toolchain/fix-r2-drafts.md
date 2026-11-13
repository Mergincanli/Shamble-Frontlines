# Round 2's fix pass: the drafted texts (the session, 2026-11-13)

Sources: `review-r2-findings.md` (the deduped items), `review-r2-verdicts.json` (the four skeptics' verdicts, each with
its exact wording), the reviewers' notes `*-review-r2.md`. Every text below is the skeptic's wording where one gave it,
adjusted only to fit the other items in the same sentence. Line numbers are 4bccf55's.

## A. Cargo.toml:21-24 (R2-D1; the R2-D1 skeptic's text; built with it: `cargo build --workspace --locked` exit 0, Cargo.lock unchanged)
```
# Overflow checks are on in all profiles for every workspace crate (ADR 0002 part 4, widened on foundation 2.2's review,
# RR-1: sim's inline, generic and small functions compile in their callers; register row OVERFLOW-ALL). "*" matches
# every non-member, which keeps the release default setting. At opt-level 0, 1, "s" or "z" in release or an inheriting
# profile, our sum() and product() can run a dependency's unchecked copy: drop "*" or give it overflow-checks = true.
```

## B. crates/sim-wasm/src/lib.rs:1-3 (R2-L6 (a); EA-3's own wording)
```
//! The wasm-bindgen bindings the Worker loads, and nothing else (ADR 0002 parts 3 and 5). Empty, with no
//! bindings, until the boundary spike fixes its exports (OQ-0002-3) and wasm-bindgen's by-day reading is done
//! (OQ-0002-7).
```

## C. The register row OVERFLOW-ALL (docs/adr/README.md:234), one edit (R2-D1, R2-D3, R2-D4, R2-L8)
State column:
> the advisor agent's rulings (D1, 2026-11-13; its wording completed on review round 2, D1-R2, 2026-11-13), never his
> decision; his go for this row: PR #48's merge; part 4, its Appendix A answer for R1-RR-OQ4, its Appendix A row for
> R5-RUSTLINT-REC4 and PM-0002-53 are reworded by engine-architect at ADR 0002's next revision

Content column:
> ADR 0002 part 4 and PM-0002-53: part 4's named setting (`[profile.release.package.sim] overflow-checks = true`, the
> same for `protocol`) reaches only code generated in sim's and protocol's own compilation, because sim's inline,
> generic and small functions are generated in the crates that call them (foundation 2.2's review, RR-1). The root
> `Cargo.toml` sets `[profile.release] overflow-checks = true` and `[profile.release.package."*"] overflow-checks =
> false`: every workspace crate checks overflow in every profile, and every non-member keeps the release default
> setting, so its integer arithmetic can still differ between dev and release builds; while `"*"` stays, a release
> profile, or one inheriting it, at opt-level 0, 1, "s" or "z" also lets our crates reuse a dependency's unchecked copy
> of shared generic code (integer `sum()` and `product()`, review round 2). That is decided when the first third-party
> crate enters any workspace crate's dependency tree, with its cost measured: a named override for each such crate (it
> beats `"*"`), or dropping `"*"`, which would also check every dependency, the server's future network crates
> included. PM-0002-53's committed counter test runs, in its four builds, from a workspace crate other than protocol
> that depends on sim: part 4's two blocks check sim's and protocol's own packages, so a test inside either cannot see
> the gap; its planted failure sets `Cargo.toml` back to those blocks. The cost measurement (R5-RUSTLINT-OQ4) covers the
> checks as compiled in the calling crates. The panic side: an overflow in `room`'s own arithmetic now panics in
> release where part 4's blocks let it wrap, and a future server crate's code would do the same; the lines that keep a
> value a client or content can set from reaching such a panic (part 4's explicit operations, which name counters and
> tick numbers; ADR 0004 part 20, whose list of panic sources names no overflow; the fuzz marks PM-0004-60 and
> PM-0001-34) go to their owners to weigh (the follow-up OVERFLOW-PANIC-RULE). Shown by foundation 2.2's planted failure
> M7, its local cross-crate run and review round 2's local runs (`docs/reports/2026-11-12-foundation-2-2.md`)

## D. The report (docs/reports/2026-11-12-foundation-2-2.md)
D1 :40-41 (R2-L4): "revisions 1 to 9" -> "revisions 1 to 10".
D2 :70-71 (R2-L1): "M7 proves the setting the tree ships." -> "M7 proves that CI turns red when the shipped release
   line goes, and E12 (local) shows how far that line reaches."
D3 :117, the M7 row (R2-L1): after "the gate stayed green)" add "; `protocol`'s binary: cargo stops at the first
   failing test binary, so `sim`'s release test did not run in CI (it fails in the same state locally: the ruling's
   check C3)".
D4 :118, the E12 row (R2-L3): "the flag on `sim`, `room` and `sim_wasm`, none on" -> "the flag on every workspace crate
   (the saved run lists `sim`, `room` and `sim_wasm`; two reviewers' re-runs after a clean build show all six, native
   and wasm32), none on".
D5 Reviews (R2-L5, R2-L6): the D5 row "the cut asserts none of its privilege properties"; the D13 row "fixed: 1de98a9
   (the crates); REPO_MAP's row in round 2's fix"; the capability-set sentence adds "shown locally by a run as root
   (util-linux 2.39.3) and, for the runner, by capabilities(7)"; the "(pending)" line becomes round 2's paragraph and
   table (all eighteen findings, the twelve items, their dispositions).
D6 Risks :213-215 (R2-D2; the R2-D2 skeptic's text, the OVERFLOW-PANIC-RULE variant of its last line):
```
- With route A, an overflow in `room`'s or the future server's own arithmetic panics in release, as test builds do;
  the old blocks wrapped it silently (room's `100 - 150` as `u32` gave 4,294,967,246, a local run). By ADR 0001 part 11
  and ADR 0004 part 20 (Proposed), a panic on the tick thread, where `room` runs, ends the match (repeatably, if a
  client drives the value); one in a connection's work closes only that connection; a browser trap ends that player's
  sim instance (online it resyncs; offline: OQ-0004-7). No server code yet; the guard is unbuilt (OVERFLOW-PANIC-RULE).
```
D7 :231, OVERFLOW-XCRATE-CI (R2-D4; the R2-D4 skeptic's text): "PM-0002-53's committed counter test runs, in its four
   builds, from a workspace crate other than protocol that depends on sim (a test in protocol passes under part 4's two
   blocks too), and must go red with `Cargo.toml` set back to those blocks; it needs a planted counter in sim's normal
   build and a way to run Wasm, which the tree lacks (CI edits, each asked of him); anchored to the first change that
   adds a dependency on sim, or 2.3, whichever comes first (if nothing depends on sim when 2.3 starts, 2.3's plan adds a
   test-only dependency or asks the advisor agent to move the anchor); a STATUS follow-up"; evidence cell adds "the
   R2-D4 skeptic's run".
D8 :254-257, OVERFLOW-PER-CRATE (R2-D1, with TE2-1's split; R2-D3's "dropping" clause):
```
- OVERFLOW-PER-CRATE: workspace crates are covered by the root setting now; `"*"` keeps every non-member at the release
  default setting (a path crate inside the repository joins the workspace and is checked unless excluded). It leaves a
  dependency's own arithmetic unchecked, concrete arithmetic inside its generic functions included; arithmetic its
  generic code does through a trait (`T: Add`, `Sum`) takes our crate's setting when our crate compiles it, but not
  when rustc reuses the dependency's own copy (an `#[inline(never)]` function at any opt-level; any shared generic at
  opt-level 0, 1, "s" or "z", where our own integer `sum()` and `product()` can then wrap too). The first third-party
  crate that any workspace crate uses decides, with its cost measured, between a named override for each such crate (it
  beats `"*"`) and dropping `"*"` (which checks every dependency, the server's future network crates included); while
  `"*"` stays, moving the release profile, or one inheriting it, to opt-level 0, 1, "s" or "z" reopens it (the register
  row OVERFLOW-ALL).
```
D9 Follow-ups, new (R2-D3; the R2-D3 skeptic's report text):
```
- OVERFLOW-PANIC-RULE: with route A, an overflow in `room`'s or a future server crate's own arithmetic panics in release
  instead of wrapping. ADR 0004 part 20 already forbids a panic a client can reach, but the detailed lines are
  narrower: part 4's explicit operations name counters and tick numbers, part 20's list of causes names no overflow,
  and the fuzz marks name no build setting. The plan of the first change that gives `room` or a server crate code (2.5
  by ADR 0002's migration plan) states the rule its arithmetic follows; the lines go to their owners to weigh at each
  ADR's next revision (the register row OVERFLOW-ALL).
```
D10 The decision entry OVERFLOW-ALL (R2-L2 = TE2-7 + EA2-1, R2-L7 = EA2-2, R2-D1's plain words, R2-D4's test home,
   R2-D2's "Why it matters", R2-D3's addition):
```
- **OVERFLOW-ALL: may overflow checks stay on in release builds for all our own Rust crates, recorded in the ADR
  register?** ADR 0002 turns on overflow checks for the simulation and the message protocol by naming their two
  crates; an overflow check stops the program when a number grows too big for its type. A review found that this
  misses the simulation's small, inline and generic functions whenever another crate calls them (tick counters, maths
  helpers), because that code is then compiled inside the caller: a test build stops on such an overflow, but the
  released game would carry on with a wrong number. The fix turns the checks on in release for every crate of ours
  (the register row OVERFLOW-ALL). Your go for the row also covers its two other rules: outside libraries keep the
  default (no checks in release), and the choice about them comes with the first outside library in any of our crates,
  because a size-saving release build could then make our code use a library's unchecked copy of shared code; and ADR
  0002's overflow test moves out of the simulation's crate into another crate that calls it (not the protocol crate),
  since a test inside either of those two cannot see the gap. This is the advisor agent's technical ruling; your "no"
  overrules it.
  - Why it matters: the register is a decision doc, so the row needs your go. And an overflow in the future server's or
    client's own code then stops with an error instead of carrying on with a wrong number (with the old setting, a
    release build turned 100 minus 150 into 4,294,967,246). By the design drafts (not yet accepted), that ends the
    match if it happens in the server's match loop, drops only one player if it happens in that player's connection,
    and in the browser stops that player's game, which restarts and rejoins online (offline: still an open question).
    The design records already say no player input may crash a match, but their detailed lists do not name this cause
    yet, so the first plan that writes that code must say how it avoids it, and the records' owners weigh adding it
    (OVERFLOW-PANIC-RULE).
```
   Options and Recommendation unchanged (option (b) still holds: back to the two blocks, the row out, the gap an open
   High deferred on his word).

## E. docs/STATUS.md (no new line: the 120-line cap)
:96 "OVERFLOW-XCRATE-CI" -> "OVERFLOW-XCRATE-CI (due sooner if a change before 2.3 adds a dependency on sim)";
:99 "later: OVERFLOW-PER-CRATE," -> "later: OVERFLOW-PER-CRATE, OVERFLOW-PANIC-RULE (with room's first code),".
(Re-wrapped within the same lines.)

## F. docs/STATUS_ARCHIVE.md
F1 OVERFLOW-XCRATE-CI (:627-631): from "The fix:" through "whichever comes first." -> the R2-D4 skeptic's text (its
   item (4)), verbatim.
F2 OVERFLOW-PER-CRATE (:779-784): the same content as D8, with its provenance "(S3; rewritten on the advisor agent's
   ruling D1, condition 6; widened on review round 2, R2-D1)".
F3 New, "For later steps": OVERFLOW-PANIC-RULE, the R2-D3 skeptic's archive text, verbatim.
F4 The block's intro: round 2's ids (RR2-*, SA2-*, TE2-*, EA2-*; R2-D1 to R2-D4 and R2-L1 to R2-L8) and the skeptics'
   notes `skeptic-r2-*.md` added to what the items rest on.

## G. docs/REPO_MAP.md:18 (R2-L6 (b))
"`client-endpoint` (prediction and reconciliation, interpolation, snapshot decoding)" -> "`client-endpoint` (prediction
and reconciliation, interpolation, snapshot decoding, the aim cast, the block writer and its layout table)".

## Open for the advisor agent's ruling D1-R2 (before C and D10 are written)
1. Keep `"*"` after RR2-1 (dropping it sooner is its call; ruling D1, condition 1 keeps it)?
2. The row's rewording of its conditions 3, 5 and 6 (C: the dependency sentence and opt-level clause, the wider
   trigger, PM-0002-53's home "a workspace crate other than protocol" and its planted failure, the reword list with
   R5-RUSTLINT-REC4's Appendix A row).
3. May the row carry the panic clause (R2-D3; the skeptic proposed it marked "not part of the ruling"), or does it
   belong only in the follow-up?
4. The decision entry D10 (it presents its ruling to Mustafa).
5. The Cargo.toml comment A.

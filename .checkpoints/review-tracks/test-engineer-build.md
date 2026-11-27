# test-engineer: build review of track.mjs, track.test.mjs and the guard test changes (2026-11-27)

Plan: review the copy (.gate/review-tracks/build/) against plan.md 3a, 3c, 3d, 3f, Build steps, and inputs-B Part 2;
run the suites; mutation trials in my own scratch copy (.gate/te-track-mut/); real-history runs vs the plan's preview.

## Done
- Suites in the copy: 1636 pass (61 track + 1575 guard), 0 fail, 29 s.
- Read track.mjs (160 lines), track.test.mjs (116), guard + guard test diffs.

## Findings so far (notes, one line each; ids final in the report)
- guard tests: the fake answers the comments read for any args (no --paginate or --jq filter check, unlike the files
  read's TE1 hardening): a mutant dropping --paginate, or changing `m: .updated_at` to `.created_at` in the jq filter,
  survives. Plan's "a PASS found only on the second page" evidence case is missing.
- track: XMLHttpRequest missing from SEC_TRIGGER (plan 3b lists it); trigger fires on any path (plan: packages/, tools/,
  legacy/ only) - over-fires on docs.
- track: inputs-B R5: a spike's .rs lines count for rust-reviewer; pick() gives them to no one.
- track: words check skips an entry whose first line is in base: a changed quote under an unchanged header passes;
  removed quoted line not a floor; "other added text in DECISIONS" not listed (plan 3f). forMergeCheck line lacks sha256.
- track: case-twin scan in main() misses a new twin when the changed path sorts second (to verify on a fixture repo).
- track: check-ignore reads the cwd's .gitignore (not base/head), and a fatal error reads as "not ignored" (fail-open).

## Verified on fixtures and real history (scratch scripts in the session scratchpad: fx.mjs, ten.mjs, real.mjs)
- fixture: new docs/a.md beside existing docs/A.md -> light, no floor (twin scan misses it). CONFIRMED.
- fixture: existing DECISIONS entry, quote changed under the same header -> light, words [], forMergeCheck []. CONFIRMED.
- fixture: DECISIONS entry removed -> nothing. CONFIRMED. legacy adds XMLHttpRequest -> no security-auditor. CONFIRMED.
- fixture: docs/s.md naming fetch( / localStorage -> security-auditor joins (over-fires outside packages/tools/legacy).
- fixture: spikes/s/src/main.rs -> picks [test-engineer] only (inputs-B R5: .rs lines count for rust-reviewer).
- real: --files on #76,#74,#73,#70,#71,#67,#69,#68,#66,#65: tracks match the preview 10/10. Head mode on M^2 exits 2
  "the diff is empty" (merge-base(origin/main, M^2) = M^2), not a shallow-clone stop: no head-mode run on merged PRs.
- real numstat (M^1..M) into classify(): #65 picks te,sa,rust; #68 te,sa,netcode = inputs-B. --files on #68 gives
  te,sa,engine (lines=1 per path: file count, not lines).
- real words check on #76 (13 entries): 11 ok, 2 "quote does not match" (1a4fd6e1, a07f7ff8): their stated bytes and
  sha256 are of quote+"\n" (= the saved file, which ends in \n). Two conventions in the record; the script floors one.
  The preview says #76 needs no go: the script would print two go floors. Tests picked only passing entries.
- real words check on #65,#66,#67,#68,#70,#75: older entries -> "no saved file named" = go floor (plan 3f: compare only
  "when the header names a saved file"); #75 one entry with no header -> "no stated bytes and sha256".
- fixture: run from a subdirectory (cwd=tools/): legacy innerHTML no longer brings security-auditor (git diff -- path
  is cwd-relative; ls-tree too). Output depends on cwd. CONFIRMED.
- advisor.md (build copy :66) lists V2_PHASE0_BRIEF among design/look docs; track.mjs GO regex lacks it.
- tools/x/Cargo.toml (or packages/client/Cargo.toml) -> light (in-doubt manifest regex lacks cargo.toml; Rust-core rule
  matches root only). Low.

## Mutation trials (scripts: .checkpoints/review-tracks/te-mutate.mjs, te-mutate-guard.mjs; copy .gate/te-track-mut/)
track.mjs, 27 mutants: killed 4 genuinely (claude.local.md, wrong base, binary floor, test-engineer dropped), 1 by a
syntax error only (quotes-outside line removed leaves `if (...) const`), 22 survived:
eyes packages/editor, eyes .html, toolSource workflows, toolSource package.json, go design docs, rule npm-shrinkwrap,
full tie order, manifests not skipped, light tie order, words no-saved->ok, words no-header->ok, byte count, main:
ignored, binary, modes, tree twins, words floors not pushed, TOOL-SOURCE prompt, sec WebSocket, allowed set ~,
empty classify light, words id regex. => main()'s git layer is untested apart from exit codes.
guard (review-tracks tests, 24): survived: comments read without --paginate; jq m: .created_at (edited PASS would
allow); jq t: "User"; Math.max->Math.min of gate runs; no-op sanity survived as expected.

## Follow-ups (after the advisor's check)
- Valid re-run of the quotes-outside mutant (push -> `void 0;`): SURVIVED. spikes/ clause dropped: SURVIVED (the
  reason-word test matches the path). read() arguments swapped: SURVIVED. Totals: 30 mutants, 5 killed, 25 survived,
  2 of them equivalent (byte count beside the sha; empty classify behind main's stop) -> 23 real gaps.
- Guard: rethrow in the allow read's catch (a throw would refuse, not prompt): SURVIVED. Count cases with totalCount
  keep their own reason (count check runs before the toolchain loop): fine.
- cwd: with `/dist/` (anchored) run from tools/: the gitignored floor is lost. color.ui=always (env config): the
  security trigger is lost (added lines start with escape codes). CONFIRMED (te-fx-cwd.mjs).
- origin/main has no case-only twins today (ls-tree | uniq -d: none).
- Evidence scripts copied here: te-fx.mjs, te-fx-cwd.mjs, te-ten.mjs, te-real.mjs, te-mutate.mjs, te-mutate-guard.mjs.

## Verdict: REVISE. Ids TE-B1..TE-B18 in the report (3 High, 7 Medium, 8 Low).

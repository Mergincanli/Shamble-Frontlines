# Advisor report: cs1-types (2026-11-18, branch claude/wonderful-hamilton-jdmlnf, base main 22c6afa)
Change-set 1 of the build plan, "Types" (foundation step 2.4); pull request #58; risk tier 1 (game code and tests);
the branch stands in for `cs/cs1-types`. Built by the first builder chat (lane 1).

## Goal
The page and the simulation share one description of their data, generated from Rust and committed in
`packages/types`. A test fails when the two drift, and the page can refuse a block of an unknown layout. Nothing a
player sees.

## Outcome
DONE: built, reviewed in two rounds, verified; it waits for Mustafa's go on the merge. Two things went wrong on the
way; both come first.

## Two misses
1. **Code fetched from the internet ran in CI before its security check.** In substance this broke CLAUDE.md's Never
   line "run unreviewed code fetched from the internet" and the advisor agent's condition on the plan (the check comes
   before the first build).
   - The record: the commit that added the new dependency (566d1d1, 12:04:33 UTC) carried the skip-CI mark; the
     checkpoint commit made on top of it (da22f96, the same second) did not, because the checkpoint script leaves the
     mark out when a code commit is ahead, and GitHub reads only the last commit of a push. Both were pushed at
     12:04:37. `toolchain` run 44300471741 built the nine new packages on a GitHub runner from 12:04:55 to 12:05:07;
     the cancel, sent at 12:05:01, landed after the build. Nothing was compiled on this machine. Mustafa was told in
     chat at 12:06:24.
   - The security check answered at 12:27: all nine packages are clean (checksums equal crates.io's by two routes; no
     advisory; the three build scripts only start the compiler and write in their own output folder; the two macros
     only transform code).
   - Its reach, qualified: a read-only token that was off the disk before the build, no network inside the cut, no
     artifact. The branch's Actions caches could not be listed (the proxy refuses it), so "nothing persisted" rests
     on the code having been clean (see CI-CUT under "Follow-ups").
   - Severity Medium (security-auditor and the advisor agent). The advisor agent ruled the build did not need his
     word first; it is his to weigh here.
   - What binds this chat since: for every new or bumped dependency the manifest and lockfile edits stay uncommitted
     (a text patch in the checkpoint folder keeps them), nothing builds the dependency graph that holds them, and
     other code is committed by path only, until security-auditor has answered. The kit's part is his: DEP-HOLD.
2. **One evidence record did not hold; a reviewer caught it.** The first record of the 20 generator runs (12:49 UTC)
   hashed files those runs never wrote: the throwaway worktree for the planted defects built into the main folder's
   build folder, cargo reused the worktree's test binary in the main folder, and its compiled-in path pointed at the
   worktree. test-engineer found it in round 1 (High); engine-architect showed one more case. Closed at the root:
   both export tests stop when they do not run in the copy of the repository they were built in; no two trees share
   a build folder; a run counts only when every generated file is newer than its start. The runs were redone; the
   first record is withdrawn.

## Plan and approval
- His words (2026-11-18, 11:08:25 UTC): "Change-set 1, Types (foundation step 2.4): its plan, its review, the build and
  its pull request, then stop for my go."
- The plan: engine-architect's draft (effort max), adopted by the main session. The advisor agent, 11:58 UTC: APPROVE
  on four conditions (a verifier run; netcode-architect as a reviewer; the security check of every new lockfile entry
  before the first build; one more planted defect). Met, except the timing of the third (miss 1).
- The advisor agent on the security answer, 12:44 UTC: APPROVE, the build goes on, on six conditions; each is carried
  out or carried in this report.

## What changed
- `Cargo.toml`, `crates/protocol/Cargo.toml`, `Cargo.lock`: ts-rs `=12.0.1`, default features off, a dev-dependency
  of `protocol` only. Nine new lockfile entries: proc-macro2 1.0.107, quote 1.0.47, syn 2.0.119, syn 3.0.10, thiserror
  2.0.21, thiserror-impl 2.0.21, ts-rs 12.0.1, ts-rs-macros 12.0.1, unicode-ident 1.0.27. No shipped build holds any
  (`cargo tree -e normal`: `protocol` alone, on the host and on wasm32).
- `crates/protocol/src/messages.rs`: the first eight message types as plain Rust types, no codec yet (`Join`,
  `JoinAccepted`, `Input`, `SnapshotAck`, `Command`, `AimPoint`, `ViewTime`, `Snapshot`'s header); every field the
  drafts leave open is marked "provisional" in its doc line. `ts_export.rs` (test only): ts-rs writes their
  TypeScript into a new scratch folder; the test fails when the committed folder does not hold the same files.
- `crates/client-endpoint/src/layout.rs`: the layout table of the output block (`BLOCK_LAYOUT`: a header with `tick`
  at byte 0 and `layout_hash` at byte 4), its hash computed at compile time (`LAYOUT_HASH`, FNV-1a, 32 bits), their
  tests. `layout_ts.rs` (test only): our own small generator and its export test.
- `packages/types/src/`: `generated/protocol/` (eight files by ts-rs), `generated/block/layout.ts` (ours:
  `LAYOUT_HASH = 0x2df78c30` and one object `BLOCK_LAYOUT`), `layout-check.ts` (`checkLayoutHash`, the page's check
  of a block's hash), `index.ts`, their tests. To write the generated files again:
  `SHAMBLE_WRITE_TYPES=1 cargo test --workspace --locked`.
- `docs/DECISIONS.md`, by script: his go on the merge of #57 with his answer OLD-LINES (11:08:25 UTC), checked byte
  for byte against merge commit 22c6afa's message; one dated pointer in this lane's entry for RECORD-FILES.
- <<DOCS-KEEPER>>

## Evidence
- **Checks by hand on the final code (65a7b13), from an empty build folder, all exit 0:** `cargo fmt --all --check`;
  clippy on the workspace, all targets, `-D warnings`, host and wasm32; `cargo test --workspace --locked`
  (`client-endpoint` 5 tests, `protocol` 2); the release tests of `sim` and `protocol`; the wasm32 build of the five
  crates; pedantic clippy and `-W missing_docs` on both crates (0 warnings); the type check (below); Vitest in
  `packages/types` (7 tests); `node tools/gate.mjs --no-smoke` PASS; the repository map check; the netcode proof
  test's three rules (no root `.cargo/config.toml`; no "type" in the root `package.json`; no non-Markdown file of the
  set names the spike's folder). Smoke cannot start in this container (Chromium build 1194 wanted, 1243 installed):
  CI runs it.
- **CI on the final head:** <<CI>>
- **Planted defects, 30 runs, all as wanted** (a throwaway worktree with a build folder of its own; never committed;
  removed without force):
  - a field renamed in Rust without regenerating fails and names `Snapshot.ts`; one character changed in `Join.ts`
    fails and names it; one offset changed in `layout.ts` fails and names it;
  - an extra file in either generated folder, or directly in `generated/`, fails; a doc line that holds a comment end
    with other text fails;
  - the control (the rename, regenerated) passes in Rust and in the type check of the shipped sources; the package's
    own test fails the type check until it follows the rename;
  - a header field added without regenerating fails only the layout's export test; the hash moves from 0x2df78c30 to
    0x5d0e9efb;
  - the hash without its tag bytes, or skipping later sections, fails the hash test (its known answers bite);
  - `client-endpoint`'s test binary run outside the copy it was built in stops (`protocol`'s was shown the same way
    by two reviewers' own runs); write mode refuses a missing tree and a linked folder (a file planted outside
    survives); the page's check with a signed read fails in Vitest.
- **The 20 generator runs, redone on 65a7b13:** `SHAMBLE_WRITE_TYPES=1 cargo test --workspace --locked` with default
  threads, 20 times; in each, all nine generated files were newer than the run's start; one set of files (set sha256
  07cd492f…4bc8); no difference from the committed files.
- **The type check:** tsc 6.0.3 by its path, with `-p`, not `-b`, on two scratch configs that are not committed (type
  settings are the 2.3 chat's): ADR 0002 part 7's flags plus `noEmit`, `allowImportingTsExtensions`,
  `module: "preserve"`, `target` and `lib` es2023, `types: []`. One reads the shipped sources alone; one adds the
  tests and `skipLibCheck` (Vitest's own type files need DOM names). 0 errors each. A parameter property and an
  `enum` planted in the generated folder each fail with TS1294; a `declare enum` passes (recorded, not claimed).
- **The pass marks, in the build plan's words** ("met": every clause holds, CI included; "shown": the same check
  passed where this chat can run it):
  - PM-0002-15 (regenerating gives 0 changed bytes; a renamed field and a hand edit fail): **shown, not met.** Its
    first clause runs in today's CI: both export tests compare a fresh generation with every committed file inside
    `cargo test --workspace --locked`. Its planted defects and its control ran here. It is met when the gate's own
    planted runs carry them in CI (PM-0002-43, the 2.3 chat's work). The build plan's table says "met at 1"; by the
    plan's own definition of "met" this report reads "shown at 1" (engine-architect's finding).
  - PM-0002-16 (a layout change without regeneration fails, or the page refuses the block): **shown in two halves.**
    Rust: the layout's export test fails on a stale file. Page: `checkLayoutHash` refuses a hash off by its lowest or
    highest bit, a block of zero bytes and a hash written the wrong way round. No test yet feeds a block written by
    Rust into the page's check: the mark's own method needs change-set 2's writer.
  - PM-0002-17 (0 type errors with the strict flags): **shown**, with `-p` and scratch configs; not `-b`, not in CI.
  - PM-0002-19 (20 generator runs give 20 identical sets): **shown** by the redone runs; not in CI.
  - PM-0002-63 (the marks above hold in CI with the three generators): **shown in part, not met**: two of three
    generators exist (wasm-bindgen's comes with change-set 2).
  - Until the CI edit below lands, CI runs no type check: the package's type exports are checked by hand only.

## Reviews
- **Round 1** (five reviewers at effort max, 12:51 to 13:29 UTC): rust-reviewer APPROVE, six Lows; test-engineer
  REVISE (the High of miss 2; three Mediums: write mode made a missing folder tree again; one Vitest test that only
  the type check can fail; blind spots in the hash test; four Lows); security-auditor: nothing at Medium or above on
  this change-set's files, three Lows; engine-architect REVISE (the same High; one Medium on this report's word for
  PM-0002-15; Lows); netcode-architect APPROVE, seven Lows, all doc lines.
- **The fix pass** (13:30 to 13:41 UTC, commits 1a2ff13 and 65a7b13): every finding taken, except four with a written
  reason: a literal copy of the block's own hash in Rust (known answers from a second implementation hold the hash
  function; the committed file holds the block's value); refusing write mode when `CI` is set (the CI edit below runs
  it there on purpose); reading the hash before asking for the whole header (both are 8 bytes today; for change-set
  4); one constant for the 16 buttons (optional; for change-set 3).
- **Two runs of the planted-defect script were not all as wanted; both are kept in the record.** In the first, 3 of
  18 runs failed for another reason than planned: the script's own backup copies sat in the generated folders and
  the export tests refused them (that refusal became two more planted defects). In a later one, 1 of 30: a new check
  for doc comments counted comment starts against comment ends and the planted text held both; it now looks at lines.
- **Round 2** (on what changed since round 1, 13:42 to 14:18 UTC): engine-architect APPROVE (its Medium closes
  with this report's sentence for PM-0002-15); rust-reviewer APPROVE (its six Lows closed); test-engineer APPROVE (its
  High, three Mediums and four Lows closed; it re-derived every known answer with its own implementation and ran the
  shared-build-folder case again: both trees now stop). security-auditor: nothing at Medium or above, its three Lows closed (it tried
  the linked-folder cases again: all refused); netcode-architect APPROVE (its Lows closed). Round 2 left Lows only; by the review budget they
  are follow-ups, and the code is as round 2 reviewed it.
- **The verifier** (effort max): <<VERIFIER>>
- **The advisor agent on the final head:** its word is in the pull request's description.

## Decisions and deviations
- **Four deviations from the approved plan,** each accepted by the advisor agent or by engine-architect:
  1. The hash's byte stream has a tag byte in front of each section and each field: without it two different tables
     could give one stream (rust-reviewer found such a pair; it is now a test).
  2. `client-endpoint`'s export test compares in memory, with no scratch folder: our generator is a pure function.
  3. The layout rules are checked by a test helper with its own good and bad tables; one rule was added: a row's size
     is a multiple of its largest field.
  4. After round 1 the generated layout is one exported object, `BLOCK_LAYOUT`, without `align`: a section named like
     a reserved word would have broken a constant per section, and `align` was the one value the hash did not cover.
- **Added beside the plan's symbol list:** public `ElementType::name()` and `ElementType::size()`, and the derives
  `Debug, Clone, Copy, PartialEq, Eq` on the three table types; private: the tag constants, a compile-time check that
  the hash is not 0 (a block of zero bytes can never pass), the tests' helpers and tables. Removed as planned: the two
  stubs (`stub_builds`, `PACKAGE`).
- **A departure from ADR 0002's words, decided by the build plan:** the draft has ts-rs write "through
  `TS_RS_EXPORT_DIR` in `.cargo/config.toml`". Here the export test names the folder itself: no `#[ts(export)]`, no
  `TS_RS_EXPORT_DIR`, no `.cargo/config.toml` (the netcode proof test's rule bars a root one; `set_var` is unsafe in
  edition 2024 and `unsafe_code` is denied). A pointer in the ADR is a follow-up; the ADR is not edited here.
- **How this report reads the card on the header's counts:** the card puts counts per entity kind in the header. With
  no entity section yet there is nothing to count; each entity section adds its count when it enters, the first in
  change-set 4 (the plan's open point, confirmed by the advisor agent).
- **Two open questions of the research, answered for what was built.** R5-LAYOUT-OQ4 (can two generator runs race?):
  designed out: one test per crate writes, each into its own folder; it reopens with `#[ts(export)]` or a second
  writer. R5-TSLINT-OQ3 (does generated TypeScript pass `erasableSyntaxOnly`?): yes for ts-rs's default output and
  for ours, with one exception: ts-rs's `repr(enum)` attribute prints a TypeScript `enum`, so no exported type may
  use it; wasm-bindgen's files are answered in change-set 2.
- **The provisional fields, for change-set 3's plan** (each marked in its doc line): the join token's format; the
  widths of `protocol_version`, `slot`, `team`, `squad`, both rates, both `group` fields, `sequence`, `presses`,
  `move_x` and `move_y`; the move vector's scale; how squads are numbered; what `mode_hash` hashes; which buttons
  exist (16 is a capacity); `AimPoint` as a whole (its 1 cm step and widths are borrowed from the snapshot encoding of
  positions; which axis is up); `ViewTime`'s fraction; the form and the width of `Snapshot`'s two command
  acknowledgements (a sequence or a tick); the order of `Input`'s commands, and whether their sequence numbers must
  be consecutive (the doc line says "consecutive"; the drafts say only that sequences move forward: netcode-architect,
  round 2, a Low); the anchor of `Input`'s 16-bit tick offsets (the drafts give the full tick only to the snapshot
  header). What settles each of 18 items and the checks the decoders must make: `netcode-architect-review2.md`, "For
  change-set 3"; an anti-cheat table per field: `security-auditor-review1.md` (both in the commit named below).
- **Size:** <<SIZE>> hand-written lines added against 600 planned (<<RATIO>> times; his stop line is twice the plan,
  1,200), by `git diff --numstat` against main, with generated files, the lockfile and docs left out. What grew: the
  export tests' folder checks (after round 1), the test tables of the layout rules and of the hash, a doc line per
  message field, and rustfmt's default line widths (rustfmt settings are the 2.3 chat's).
- **A refusal, with the route taken:** Claude Code's built-in check refused an `rm` over a shell variable while
  leftover scratch folders in /tmp were cleaned; the form the check itself names (`"${d:?}"`) was used.
- **No "Lane requests" issue exists** (12:28 UTC: the only open issue is the Night queue, #16), so no request of the
  other lane was taken in (LANE-ISSUE below).
- **The Part 15 change's size:** its report says 220 written lines, as he accepted; a recount on its final head gave
  223 (the advisor agent's Low).
- This change-set's notes (the plan, the reviewers' notes, the scripts and their results) are deleted in the last
  commit; the commit before it holds them: <<NOTES-SHA>>.

## Not verified
Firefox, WebKit and Safari (nothing here runs in a browser yet); any timed mark; the gate's own planted runs (they do
not exist yet); `tsc -b` and the base tsconfig (the 2.3 chat's); git inside CI's network cut (the CI edit needs it);
CI-CUT on a real runner; `cargo audit` and `cargo deny` (not installed: the advisories were read from OSV and
rustsec.org); the export tests on Windows and macOS; smoke in this container.

## Follow-ups
- **CI-CUT** (security, Medium, in CI, outside this change-set; found by the advisor agent, confirmed and narrowed by
  security-auditor; not tried on a runner): in the `toolchain` job the network cut separates the network, not the
  files. Build code runs as the runner's own user and can rewrite the checkout action's file, which the job runs
  again after the cut with network and the job's cache-write token. Through it a hostile dependency could plant a
  cache entry that this branch's smoke job later unpacks and runs as its browser. It reaches no secret and not
  main's cache scope. The fix is a CI edit: decision CI-CUT.
- **From the security check:** ts-rs's packages ship no licence file (MIT is declared; no duty while it is
  test-only); the plan's draft said ts-rs needs Rust 1.88.0, its published manifest says 1.78.0 (both under our
  1.99.1); the network cut stops IP traffic only (for the gate v2's workflow).
- **From round 2 (Lows, not fixed here; the code is as reviewed):**
  - Two doc lines claim more than holds. Both export tests say write mode "never writes outside the repository":
    true for a linked generated folder or parent (refused), not when a crate's own folder is a link (security-auditor
    ran it: write mode then wrote outside the worktree). `Input.commands`'s doc says "consecutive" sequence numbers;
    the drafts say only that sequences move forward. Each takes a committed link or is a wording matter; fix with the
    next change to those files.
  - The doc-comment check catches a comment end inside a `///` line (the only form the message types use). A
    hand-written `#[doc = "…"]` attribute or a block doc can still put a line of valid TypeScript into a generated
    file; the fix is to refuse those two forms in `messages.rs`.
  - The fix pass's new guards have no committed test of their own (the planted script is deleted with the
    checkpoint): they belong in the gate v2's planted set.
  - The build-folder check compares path strings: an honest run through a second spelling of the same folder, or a
    moved clone that kept its build folder, stops until a rebuild, and the message does not say so.
  - `generated/` may hold a link or a third folder unnoticed; the tag-byte pair has no "must differ" line of its own;
    on a file system that ignores case a link named `X.TS` would survive write mode's delete loop (reasoned, not run).
- **Change-set 2:** an `exports` entry in `packages/types/package.json` before the first import by package name; the
  block writer takes offsets through a by-name handle that fails the build on a missing name; `Section::align()` in
  Rust only; the export tests stay native-only; run the layout check on the first block of every Worker start.
- **Change-set 3:** the provisional list above; decoders cap every sender-sized field before allocating; `Join` gets
  a redacting `Debug` once its token is a credential.
- **Change-set 4:** the header states each section's row count and byte offset; padding between sections has one
  owner; the page reads the hash as soon as its four bytes exist; content's export test shares the folder rules.
- **The 2.3 chat:** its formatter and linter skip `packages/types/src/generated/**` (the files are compared byte for
  byte) while the type check still reads it; the gate's own regeneration builds into a build folder of its own.
- **The drafts' next revision (his go):** who reads the message types in TypeScript (the page sees only the block);
  the ADR 0002 pointer on how ts-rs's folder is named.

## Risks
1. The provisional widths move when the netcode proof test reports and when the codec is built: one regeneration and
   a small diff.
2. Until the CI edit lands, a type error in `packages/types` is caught by hand only.
3. The 2.3 chat's later type and format settings may differ from the scratch configs used here: one mechanical commit.

## Proposed next step
Change-set 2 of the build plan (the skeleton, foundation step 2.5): its plan first.

## Decisions needed
**DEP-HOLD: should the checkpoint script itself hold a push that carries a dependency change until its security check
has answered?** Why it matters: on 2026-11-18 a checkpoint save published the new dependency without the skip mark and
CI built nine unchecked packages (miss 1); today only each chat's memory prevents a repeat, in either lane and in the
night shift. Options: (a) a kit change (risk tier 2, from a day session, its exact text shown to you first): when the
commits to push change a manifest or lockfile, `checkpoint.mjs save` commits locally, does not push, and says why;
one line in the checkpoint rules and a test; (b) no kit change: the hold stays a written procedure; (c) (a) now, and
with the next CI change a check that refuses to build when the lockfile gains an entry with no recorded review.
security-auditor adds to (a): count from the remote branch's tree; cover plain pushes in the guard too; release the
hold by a recorded review of the exact lockfile; also watch `rust-toolchain.toml`, `.cargo/config.toml`, `.npmrc`,
workflow files and `.gitmodules`. Recommendation: (c).

**CI-TYPES: may the `toolchain` job get a type check and a regenerate-and-compare step, with the exact text in the
appendix?** Why it matters: today CI prints only the type checker's version, so the package's type exports and "the
committed files are what the generators give" (in write mode) are checked by hand. Options: (a) yes, as its own pull
request (risk tier 2; it commits no tsconfig); (b) no: both wait for the 2.3 chat's gate v2. Recommendation: (a).
Your rule: every gate and CI edit is asked with its exact text; this is that ask.

**CI-CUT: may the `toolchain` job get the fix for the CI finding above, with the exact text in the appendix?** Why it
matters: without it a hostile dependency could reach this branch's later runs through the job's cache. Options: (a)
part 1 now (no cache in this job), in the same pull request as CI-TYPES, and part 2 (no JavaScript action in the job
that runs the cut) with the gate v2's workflow; (b) both parts now; (c) wait for the gate v2. The text was checked
against GitHub's documentation only, not on a run: its pull request must show "Cache mode: none" in the job's log.
Recommendation: (a).

**LANE-ISSUE: who opens the "Lane requests" issue that Part 15 names?** Why it matters: both builder chats are to read
it at the start of each change-set, and it does not exist. Options: (a) this chat opens it with a two-line body (what
it is for; only comments from your account count); (b) you open it; (c) no issue: requests go through you.
Recommendation: (a).

## For Mustafa
1. The file route for a zip: this chat sent you `shamble-cs1-types-test-archive.zip` (never committed). Save it,
   unpack it, and say whether that worked. It holds `README.txt`, the nine generated files and `sha256.txt`; the
   README says how to compare one checksum on Windows.
2. Answer DEP-HOLD, CI-TYPES, CI-CUT and LANE-ISSUE when it suits you; none blocks this merge.
3. Your go on merging pull request #58, after the Advisor's check. No Brave check: nothing a player sees. It is
   needs-mustafa also because two security findings stay open (miss 1 and CI-CUT).

## Appendix: the exact texts asked for
**CI-TYPES**, in `.github/workflows/toolchain.yml`, inside the single-quoted script of the step "Build and test with
the network cut off". After the line `cargo test --workspace --locked`:
```
              SHAMBLE_WRITE_TYPES=1 cargo test --workspace --locked
              changed=$(git status --porcelain --untracked-files=all -- packages/types/src/generated)
              [ -z "$changed" ] || { echo "writing the generated types again changed:"; echo "$changed"; exit 1; }
```
In place of the last line, `npx --no -- tsc --version`:
```
              npx --no -- tsc --version
              strict="--noEmit --strict --noUncheckedIndexedAccess --exactOptionalPropertyTypes --noImplicitOverride --noFallthroughCasesInSwitch --noImplicitReturns --isolatedModules --verbatimModuleSyntax --erasableSyntaxOnly --allowImportingTsExtensions --module preserve --target es2023 --lib es2023"
              shipped=$(find packages/types/src -name "*.ts" ! -name "*.test.ts" | LC_ALL=C sort)
              tests=$(find packages/types/src -name "*.test.ts" | LC_ALL=C sort)
              [ -n "$shipped" ]
              [ -n "$tests" ]
              npx --no -- tsc $strict $shipped
              npx --no -- tsc $strict --skipLibCheck $shipped $tests
```
Tried here by hand with the tools by their path: exit 0 each. Not tried: git inside the cut.

**CI-CUT**, part 1, in the same file, after the line `timeout-minutes: 15` of the job `toolchain`:
```
    # No cache in this job, restored or saved: code planted for a later step cannot reach another run through it.
    cache-mode: none
```
Part 2, for the gate v2's workflow, in place of the checkout and setup-node steps of the job that runs the cut:
```
      - name: Check out the commit (plain git, public repository, no token, no post step)
        run: |
          git init -q . && git fetch -q --no-tags --depth=1 "$GITHUB_SERVER_URL/$GITHUB_REPOSITORY" "$GITHUB_SHA"
          git checkout -q --detach FETCH_HEAD && [ "$(git rev-parse HEAD)" = "$GITHUB_SHA" ]
      - name: Node 22 from the image's tool cache (no action, no post step)
        run: |
          bin=$(ls -d "$RUNNER_TOOL_CACHE"/node/22.*/x64/bin | sort -V | tail -n 1); [ -x "$bin/node" ]
          echo "$bin" >> "$GITHUB_PATH"
```
Both parts are security-auditor's text, checked against GitHub's documentation only.

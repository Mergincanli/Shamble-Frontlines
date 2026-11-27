# Step 9's rest, in the order it is built (Builder V6, 2026-11-27; revision 2, the advisor tool's cuts)

The approved row is the sheet's row 9 (`reread-sheet.md:167`, revision 3, approved by the advisor agent); its design
is engine-architect's block 1.8 (`cs4-walk/engine-architect-cs4.md:232-246`) and netcode-architect's route (i)
(`netcode-architect-read.md`, Q2). Done already: the type writer's manifest line (c09f459), the generated content types
(6290c85), the page folder's plain files and the markup-sink scan (5c6738a). Nothing below changes what the row holds;
only the order and the cuts are the builder's, **told to the advisor agent at its final read**, as the earlier order
change is. **Each cut ends green**: the gate, the full local checks, the count, a commit, a push, CI read green
before the next code push (a code push cancels a toolchain run in progress). Revision 1 had five pieces, of which the
constructor's and the Worker's could not each end green (a fourth argument breaks the Worker, its type check and the
browser tests until the page sends it; a replay needs the frozen bundle first): hence these three cuts.

**Cut A, the build, alone.** `build.mjs` runs `content_bundle` into the page folder (`content.json`); `build-checks.mjs`
refuses a page folder without `content.json`, or with one over 8 MiB, with its tests and plants. Nothing reads the
file yet. Before it: every way the specs serve the page is listed (the built folder, Vite's development and preview
servers, the page opened as a file): each must serve `content.json`, or its spec must say what the page shows without
it (the page reads it only from cut C on). **Listed (2026-11-27, at cut A):** (1) Playwright's web server, `node
serve.mjs` in the built development folder (`playwright.config.ts:51`): it holds `content.json` from cut A on; (2)
Vite's preview server on the built folder (`isolation.spec.ts:32`): the same; (3) **Vite's development server on the
sources** (`isolation.spec.ts:31`): no `content.json` there, so cut C must make it served there (engine-architect's
block 1.9 plans a serve-only plugin for that, CD-7) or the spec must say what the page shows without it; (4) the
page opened as a file (`file-url.spec.ts:12`): it "either runs or says how to serve it", and a fetch from a file
address must end in that notice, never a broken page. Each is settled in cut C before its commit.
**Read at cut B (2026-11-27, 15:20 UTC):** the isolation spec under the development server asks the match to run
with no error (`isolation.spec.ts:36-41`), and that server answers a file that is not there with the page itself, 200
(`:48`): a fetch of `./content.json` there would hand the page's HTML to the loader. So cut C brings the
development-only server plugin engine-architect's block 1.9 plans (`cs4-walk/engine-architect-cs4.md:249-251`,
`apply: "serve"`, so in no build's output), its serving part only (the watch and the reload, CD-7 and PM-0005-10,
stay with their change-set): `/content.json` made by the same content tool the build runs, so the dev server's
verdict is the build's. Before it is built, its form goes to the advisor agent if it is more than block 1.9 says.

**Cut B, the recording's content, with the constructor untouched.** First the `git show` copies of main 2483e9a's
recording files, taken before any tool writes (`.gate/cs4b/main-recordings/`). Then
`crates/devtools/recordings/first-loop.content.json`, the bundle of today's `content/`, written only by
`make_recordings --freeze first-loop`, an input of `made()` and of the check (never written again by them), listed in
the manifest with its length and sha256. The header's existing content-hash field holds that sha256, and its map id
`test-scene`; format version 1 stays. `Replay::new` and every runner (native, the tool, Node with `node:crypto`, the
Chromium spec with WebCrypto) check the bundle's sha256 against the header; the Rust and the TypeScript checks refuse
a header of zeros (no content). The test-module exports `replay_hashes(rec, bundle)` and `replay_state_at(rec,
bundle, tick)`. **Proof:** `first-loop.hashes` lines 2 to 1,801, `first-loop.vector.txt` whole and the recording's
1,800 step records (122,400 bytes) equal the `git show` copies; by design line 1's digest, the header's hash and map
id, and three manifest lines change. Plant: a valid bundle that differs in one value is refused by name before the
match is made.

**Cut C, the constructor, its callers, the Worker and the page: one commit**, saved as a patch in the notes at least
every 25 minutes while it is built, the count taken at each save (5,633 falls inside it: his one sentence then goes in
the very next chat message).
1. First, as soon as the Rust compiles: the wasm-only build, whose built-file check is expected to stop on what
   content brings into the players' file (`C::round`, perhaps `C::__multi3`); the names it prints go to rust-reviewer,
   which decides each (an entry in `maths-allowed.mjs` with its reason, or a change of the code). No cargo runs in its
   target folder while it works. If its answer changes the content crate, 4b-1's proofs run again before anything
   is built on it.
2. `Sim::create(seed, mode, pad_bytes, bundle: &[u8])` and the bindings' `Sim::new` the same;
   `CreateError::Content(content::Error)` (the type loses `Copy` and `Eq`); `message(&self)`; the checks in the order
   seed, mode, content, padding, with a test of that order. The set is held for step 10's wiring; nothing of it enters
   the state. **One bundle for every caller that is no replay, decided once:** a test bundle whose map has the id step
   10 will ask for (`test-scene`), so step 10 does not touch every caller again.
3. Every Rust caller (`sim-wasm`'s tests, `tests/step_alloc.rs`, `devtools`' `replay.rs`, `bin/replay.rs`,
   `bin/make_recordings.rs`, `wasm.rs`, `tests/recordings.rs`, `two_instances.rs`, `two_processes.rs`).
4. The Worker and the page: the `start` message gains `bundle: ArrayBuffer`, refused unless it holds 1 to
   8,388,608 bytes, before the glue copies it (`messages.ts`, `worker.ts`), that number tied to the loader's own cap by
   a test; `packages/client/src/app/content.ts` (one fetch of `./content.json`; a copy kept before the transfer;
   `JSON.parse` of the copy only after the Worker's `ready`); `worker-link.ts`, `boot.ts`; the boundary tests
   (`sim-wasm.test.mjs`, `replay.test.mjs`) and the replay spec.
5. **Proof that loading content changed no state, sharper than lines:** after this cut every recording file is byte
   for byte as cut B left it. Plant: one content byte folded into the shared state at construction fails at tick 1
   natively, in Node and in Chromium (never with `make_recordings` run under the plant).

**Cut C in detail, in the order it is built (written at cut B, 2026-11-27):**
- a. `Sim::create(seed, mode, pad_bytes, bundle)`: the loader's `load(bundle)` between the mode and the padding
  checks; `CreateError::Content(content::Error)`, `message(&self)`. The set is validated and not yet held: a field
  nobody reads would be dead code, and step 10's wiring is what takes the map from it into the local match. Then at
  once the wasm-only build: the built-file check's names go to rust-reviewer (its decision, then its entries or the
  code change), before anything else of the cut is built on it.
- b. Every Rust caller; the non-replay callers pass `content::bundle::test_bundle()` (cut B's one test bundle, whose
  map is `test-scene`); the replays pass the frozen content (`Replay::new` already holds it).
- c. The Worker: `messages.ts` (`start` with `bundle: ArrayBuffer`, refused unless 1 to `MAX_BUNDLE_BYTES` bytes;
  that number tied by a test to `crates/content/src/bundle.rs`'s line), `worker.ts` (the bundle to `new Sim`).
- d. The page: `content.ts` (one fetch of `./content.json`, its bytes kept as a copy before the transfer, the copy
  parsed only after the Worker's `ready`), `worker-link.ts` (`start(padBytes, bundle)` transfers it), `boot.ts`
  (the content fetched before the Worker starts; a failure said on the page in plain text). The file-address spec
  stays as it is: in Chromium the page's module script does not run from a file, so the notice keeps
  `index.html`'s own "run \"node serve.mjs\"" line; in a browser that did run the script from a file, the
  content's fetch fails as the Worker's start already does today, and is said on the page in plain text the same
  way.
- e. The development server's `/content.json` (block 1.9's serving part, `apply: "serve"`), made by the content
  tool the build runs.
- f. The boundary tests (`sim-wasm.test.mjs`: the constructor with the test bundle, a malformed bundle refused as
  an `Error` naming the file and the place, never a trap; `replay.test.mjs`: the players' `Sim` made on the frozen
  content), the replay spec's Worker made on the frozen content, the page's own tests.
- g. The proof: after this cut every recording file is byte for byte as cut B left it; the plant (one content byte
  folded into the shared state at construction) fails at tick 1 natively, in Node and in Chromium.

Reviewers at the close for this step: netcode-architect (steps 9 and 10) with the round's others.

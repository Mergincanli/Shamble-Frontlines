# advisor agent: three rulings for 4b-2 step 9, cut C (the Worker and the page)

Asked by Builder V6 (the main session), 2026-11-27, about 15:55 UTC. The worktree is
`/home/user/Shamble-Frontlines/.gate/cs4b-2` (local branch `cs/4b-content`, pushed as `claude/ecstatic-wozniak-5cbdzs`;
head 0a065d0; cut C's code so far is uncommitted and saved as `.checkpoints/cs4b-content/cutC-wip.patch`).
The approved row is the sheet's row 9 (`.checkpoints/cs4b-content/reread-sheet.md:167`); the order and the three cuts
are `.checkpoints/cs4b-content/step9-order.md` (its line 30: "Before it is built, its form goes to the advisor agent
if it is more than block 1.9 says"); the approved test list is `.checkpoints/cs4b-content/test-list.md` (its rules
for the whole part at :16-26, step 9 at :137-152). The card (`.checkpoints/cs4b-content.md`, "A new threshold or
order goes back to the advisor agent first") is why each of these comes to you before it is built. Each question
below gives the approved words, the facts, and the builder's proposal. Rule APPROVE, REVISE (with what), or ESCALATE,
for each one on its own.

## Q1. The message caps in the Worker and on the page (S-must 6)
- **Approved words.** test-list.md:25-26: "Errors echo at most 64 bytes of input, each byte outside 0x20 to 0x7E
  written as `\xNN`, and a whole message is capped (about 512 bytes) in Rust, in the Worker's trap message and in
  the page's message check (S-must 6)." Its source, security-auditor's P10
  (`security-auditor-testlist.md:32`): "worker.ts:56-59: a refused `new Sim(...)` posts `trapMessage(...)` = the
  JsError's message, uncapped (only the panic buffer is capped at 1016); isWorkerToPage `error` takes any string
  length. Cap in Rust's Error::message, again in trapMessage, test with a 1 MiB key." security-auditor's 4b-1 review
  (`security-auditor-review-4b1.md:60`): "MET in Rust ... Worker trap and page check are 4b-2's."
- **Facts.** Rust: `content::Error::message` is cut to 512 bytes (`crates/content/src/error.rs:11`, `MESSAGE_BYTES`,
  and :285-292), every byte ASCII, and begins "the content is refused: ". Cut C's constructor gives that sentence
  whole (`crates/sim-wasm/src/lib.rs`, `CreateError::message`): at most 512 characters. (Corrected before 15:57:53 UTC,
  the next clock reading, and sent to the agent, whose message says "15:58": a time typed ahead of the clock. The
  first version said the constructor put its own "the content is refused: " in front, at most
  536 characters; that was a defect of the uncommitted code, which would have said it twice.) The Worker's other path,
  a panic's text, is read from the hook's buffer, whose room is 1,016 bytes (`packages/sim-worker/src/panic-message.ts:8`,
  `PANIC_TEXT_ROOM`), and told as "the simulation stopped: " (24 characters) and the text (:33): at most 1,040
  characters. A thrown value that is not the Wasm's (the glue's own TypeError, a failed allocation) has no bound
  today (:34). The page shows an `error` message's text with `textContent` only (`packages/client/src/app/boot.ts:31-35`,
  via `worker-link.ts:252-253`).
- **Proposal.** `trapMessage` cuts the non-panic path to at most 512 characters with a mark (tied by a test to
  content's `MESSAGE_BYTES`); the panic path stays on its room, uncut; `isWorkerToPage` refuses an
  `error` whose message is over 1,040 characters (`PANIC_TEXT_ROOM` plus the Worker's lead, tied by a test). So
  nothing the Wasm writes is cut twice, and the page's bound is the longest legitimate message. The page's bound is
  above "about 512": that is the departure to rule on. Tests: a 1 MiB thrown message is cut to the bound; a bundle
  with a 1 MiB key reaches the page as the loader's sentence, at most 512 characters (in the boundary test and the
  Chromium spec); an `error` of 1,041 characters is refused by the page's check.

## Q2. The page's parse: the narrow move of the map's field-by-field reading
- **Approved words.** Row 9: "`packages/client/src/app/content.ts` (one fetch; a copy kept before the transfer;
  `JSON.parse` only after the Worker's `ready`)" and "the generated types `packages/types/src/generated/content/` and
  `packages/types/src/index.ts` (their first reader is `content.ts` ...)". engine-architect's design 4.5
  (`.checkpoints/cs4-walk/engine-architect-cs4.md:677-680`): "`content.ts` (new): fetches `./content.json` once,
  parses it (`JSON.parse`, the files by path, the camera and the map read field by field into their generated types,
  never merged into other objects: CD-9 :454-455), and hands the bundle's bytes to the link for the Worker".
  test-list.md:146-150 (step 9): "one fetch; a copy kept before the transfer; the page's `JSON.parse` runs only after
  the Worker's `ready`. Tests: a unit test with a fake Worker that answers `error` (no parse, nothing drawn, exactly
  one fetch); a Chromium spec serving bundles through `page.route`: a duplicate key, `__proto__`, `1e400`, an extra
  file-entry key, a `source/` path, an unknown key named like an image tag; expected: the loader's sentence as text,
  no `img` element, the scene still empty."
- **Facts.** Nothing reads the parsed content before step 12's scene (row 12: `scene.ts`, `picture.ts`'s
  `setContent`, "the map only"); no camera content exists in 4b (the camera file is a later change-set's). Step 12
  is the second rung (it moves to the start of 4c if the count passes 8,000 at its look-ahead; the official count
  with cut C's work so far is 5,760). The field-by-field reading of a `MapDocument` (bounds, spawns, shapes, regions)
  is the part that costs lines (about 60 to 80, with tests about as many again).
- **Proposal.** Cut C builds `content.ts` with: one fetch of `./content.json` (the response's status checked; a
  failure said on the page in plain text); the copy kept before the transfer; after the Worker's `ready`,
  `JSON.parse` of the copy into the bundle's files by path (its `format` and `version` read, then path to the file's
  text), exposed by `content.ts` and read by its unit tests (the fake Worker answering `error`: no parse, exactly one
  fetch). The field-by-field reading of the map into `MapDocument` moves to step 12, with its first reader, the
  scene; if step 12 leaves for 4c, that reading goes with it, while the fetch, the copy and the parse after `ready`
  stay in 4b-2, with their tests. The alternative is to build the map's reading in cut C as 4.5 says.

## Q3. The development server's `/content.json`: its form
- **Approved words.** engine-architect's block 1.9 (`engine-architect-cs4.md:249-251`): "The Vite plugin (`apply:
  "serve"`, so never in any `vite build` output, the zip included): serves `/content.json` assembled from
  `content/**` exactly as the bundle format says; watches `content/`; on a change sends the custom event
  `shamble:content`." step9-order.md:28-30: "its serving part only (the watch and the reload, CD-7 and PM-0005-10,
  stay with their change-set): `/content.json` made by the same content tool the build runs, so the dev server's
  verdict is the build's."
- **Facts.** `packages/e2e/tests/skeleton/isolation.spec.ts:31-41` runs the page under Vite's dev server and expects
  the match to run with no error; that server answers a file that is not there with the page itself, 200 (:48), so
  without a plugin the page would hand its own HTML to the loader. The preview server serves the built folder, which
  holds `content.json` from cut A on. The build writes the bundle with `writeContentBundle(outDir)`
  (`packages/client/scripts/build.mjs:214-227`): `cargo run --quiet --locked -p devtools --bin content_bundle
  --target-dir .gate/target-content -- <outDir>`, after `refuseSettings()`, with `cargoEnv()` (the path remap flags);
  build.mjs runs its `main` only when it is run as a script (:313-318), so importing it runs nothing.
- **Proposal.** In `packages/client/vite.config.mjs`, one plugin `{ name: "shamble-content", apply: "serve",
  configureServer }` (Vite calls `configureServer` for the dev server only, so neither a build nor the preview server
  has it). Its middleware answers `GET /content.json` (by its path, the query ignored) and nothing else: it runs
  build.mjs's own `writeContentBundle` (exported; loaded by a dynamic import of build.mjs's file at the first
  request, so Vite's config bundler does not inline it) into `.gate/dev-content/` (outside every package and every
  folder the server serves; made fresh by the tool), one run at a time, then answers 200 with
  `Content-Type: application/json` and that file's bytes. On any failure: 500, `text/plain`, one fixed sentence
  ("the content tool refused the content: its words are in the dev server's output"); the tool's own words go to the
  server's console (as in a build). The same cargo call, environment and target folder as the build, so once a build
  has run (CI runs the builds before the browser tests) the request compiles nothing. No watch, no reload, no custom
  event. The isolation spec gains: under the dev server, `/content.json` is 200 `application/json` and its bytes equal
  the built page's `content.json`.

## What comes back
For each question: APPROVE, REVISE (what to change), or ESCALATE (to Mustafa, with why), one paragraph each, and
anything you find that the proposal gets wrong against the approved text. Read only what you need; edit no file of
the worktree but your notes.

# security-auditor: 4b test list read (not a review round; not the dependency check)

Date 2026-11-26. Worktree .gate/lane1-takeover, branch cs/4b-content (main 2483e9a). Read-only; notes only.
Agent memory: none found (no `.claude/agent-memory*` folder in the main checkout or the worktree), so the threat model
below is built from this read and 4a's notes (c17a054).

## Plan of this read
1. reread-sheet.md (done: B.1 cut grammar, B.2 aim reach, B.4 LINKS-CHECK, C steps 0-9).
2. plan.md sections 5, 9, 11, 14, 17.
3. engine-architect-cs4.md block 1 (1.3, 1.5, 1.8, 1.12); rust-reviewer-cs4.md blocks 1 and 2.
4. Today's code: serve.mjs, boot.ts, worker-link.ts, messages.ts, host.ts, sim-wasm lib.rs, devtools files.rs,
   build.mjs, build-checks.mjs.
5. 4a notes: security-auditor-r1.md and -r2.md at c17a054 (C1-C4, S3).
6. Answer a-f, ranked must / should / can wait.

## Threat model
(pending)

## Points (one line each, as found)
- [read] plan 5, 9, 11, 14, 17 and engine-architect 1.1-1.13 read. Candidate points so far (to rank at the end):
- P1 stage 1/bundle: file cap must apply to each `text` AFTER unescaping and to the bundle bytes before parse; no cap on the number of files or on tree nodes is stated: 8 MiB of `[0,0,...` = ~4M nodes with line+col each (~200+ MB) can exhaust wasm memory (abort = trap, no loader sentence). Test: worst-density file at cap and 8 such files, peak heap counted < stated ceiling, or a node cap refused by name.
- P2 stage 2: `1e400` parses to inf in `str::parse::<f64>` (finite grammar text, non-finite value); stage 5's finite check covers only fields with a range. Test: `1e400`, `-1e400` refused at stage 2 in a position with no range (e.g. `version`, an `at` component) + scan: every number field has a finite min and max.
- P3 stage 2 + manifest: `overflow-checks = true` in release makes any unchecked `+`/`*` on a client-sized count a panic (digit accumulation for the 2^53 test, exponent digits, line/col). The crate lints listed do not include `clippy::arithmetic_side_effects`. Test: 10,000-digit integer, exponent with 30 digits, both in release native + wasm.
- P4 bundle: `path` grammar (one known kind folder, lowercase, `.json`, no `..`, no `source/`, no backslash, no duplicates incl. case-folded), folder/format agreement, a `text` that is itself a bundle, extra keys in the bundle object and in a file entry.
- P5 errors: messages echo client-controlled keys/paths/values (1 MiB key; ESC/CSI/bidi bytes in the refusal for a bad string byte); cap echo length, render non-printables as escapes; same message reaches the page readout, "Copy report" and tool stderr / CI logs.
- [read] rust-reviewer blocks 1-2 (its 8 dependency-hold questions are NOT repeated here), serve.mjs (CSP: default-src 'none'; script-src 'self' 'wasm-unsafe-eval'; worker-src 'self'; connect-src 'self'; no inline; realpath + inside-folder + isFile; .json served), files.rs.
- P6 files.rs: `plain` is private and only an lstat pre-check (`Err(_) => Ok(())`), then `std::fs::read` follows whatever is there at open time; LINKS-CHECK needs a pub read function (lstat, open, fstat is_file, same dev+ino on unix) that every recording/content reader calls, plus a source scan so no test calls `fs::read`/`File::open` on the recordings folder directly. Parent folders are not checked (only the last component).
- P7 tools: `content-bundle` walk must use DirEntry::file_type (no follow) and refuse symlink files AND symlink folders, FIFOs, sockets; a link under content/ to a secret would otherwise be read and either shipped in content.json (if valid) or echoed in a refusal into CI logs.
- P8 `testlines write`: if the table's "home" column names the output file, a hostile `content/source/testlines-table.json` can make a cargo-run tool write `.claude/`, `.github/` or `content/source/` itself: a write the guard's file and shell patterns do not see (guard bypass = Critical class). Target must be a code constant (4b: content/movement/soldier.json only), lstat'ed, written via create_new temp + rename.
- P9 built-file check: imports should be an exact allow-list (wbg glue names), not a fma-name deny-list (a renamed host import slips); the check must throw, not return a list a careless caller logs; no env/flag may skip it or point it at another file; native: `rustc --print cfg` does not see `.cargo/config.toml` rustflags that `cargo test` would use; a `compile_error!` under cfg(target_feature="fma"|"avx2"|"simd128"|"relaxed-simd") in sim fails closed for whatever flags cargo passes.
- [read] boot.ts, worker.ts, worker-link.ts, messages.ts, host.ts, panic-message.ts, sim-wasm lib.rs, build.mjs, build-checks.mjs, 4a r1/r2 (C1-C4 are cs7/cs8/Slice 3/4c-4d: none is 4b's; S3 read side and R2-1 are the live carry-overs).
- P10 worker.ts:56-59: a refused `new Sim(...)` posts `trapMessage(...)` = the JsError's message, uncapped (only the panic buffer is capped at 1016); isWorkerToPage `error` takes any string length. Cap in Rust's Error::message, again in trapMessage, test with a 1 MiB key.
- P11 messages.ts:121-122 `start` checks padBytes only and ignores extra fields; 4b's `bundle` must be `instanceof ArrayBuffer`, 1..=8 MiB, checked before the glue copies it into Wasm memory (passArray8ToWasm0 mallocs the full length first). Second `start` already refused (worker.ts:54, host.ts:98).
- P12 page order: parse the bundle in the page only after the Worker's `ready` (keep a copy before transfer); then JSON.parse never sees a bundle Rust refused (deep nesting, `__proto__`, 1e400, dup keys). Test: content.ts with a fake link posting `error` -> parse never called, nothing drawn.
- P13 ids/enums/group strings: the cut grammar allows `\n` (needed only in the bundle's `text`); an id with `\n`, spaces, `<` or 1 MiB would reach errors, the readout's content rows and "Copy report" (pasted into Advisor chats). Ids need their own charset+length in the field table: `[a-z0-9_-]{1,64}`.
- P14 LINKS-CHECK places found: tests/recordings.rs:18, :83, :96; two_instances.rs:42, :44; two_processes.rs:113, :115; replay.test.mjs:15; determinism.spec.ts:68, :70, :72. Same class, outside the sheet's scope: f32_vectors.rs:22, :24, stream_vectors.rs:8, rules.test.mjs:15, skeleton.spec.ts:6, bin/replay.rs:16. CI: persist-credentials false + contents: read (gate.yml:27, toolchain.yml:56), so a linked fixture cannot print the checkout token; it can still read until memory runs out (/dev/zero) or echo a runner file in a failing diff.
- P15 measured (python3 -I on .checkpoints/cs4-walk/testlines-export.json, data only): 15,567 B, no BOM, 0 bytes above 0x7E, 0 tabs/CR, 0 backslash escapes, 351 numbers (max 7 significant digits), depth 5, and 5 `true` literals (0 false, 0 null). The cut grammar refuses `true`: `testlines` cannot read his Export with the content reader as proposed. Security side: no run-time grammar switch in the reader the players' file links.
- P16 recordings.rs:83, :96, :142 read the test's own scratch copy (CARGO_TARGET_TMPDIR): may stay. hash_cost.rs reads no file. files.rs:115 reads after an lstat pre-check (window).
- P17 build.mjs:176-182 filesOf: a Dirent that is a link is not a directory, so it is read as a file through the link (readFileSync follows). Vite empties outDir first, so only links made after that matter; content-bundle's output path must not be a tracked path (a committed link there would redirect the write).
- P18 advisor.md:93-104 (needs-mustafa triggers) names no content/source/ rule; plan 11b's "needs his go" is words only for a later Export-only PR (tier 1, `ready`).
- P19 rust-reviewer-cs4.md:176: PM-0004-25's side build "exempts only this build by name" from the 0xFD rule: in 4b the check must have no exemption parameter; the 4d reading builds elsewhere and never writes pkg/.
- P20 Cargo.toml:21 `unsafe_code = "deny"` (a local allow overrides it), Cargo.toml:30-31 overflow checks on in release for every workspace crate (content included): an unchecked count is a panic, a trap in Wasm. The content lint list (engine-architect 1.1) lacks clippy::arithmetic_side_effects.
- P21 vite.config.mjs:37 fs.allow serves packages/client/ whole; must not widen to content/ or the root.

## Threat model (filled)
- Sources of bytes in 4b: (1) content/** and content/source/** in the repository, which a hostile or careless branch can change, links included; they reach content-bundle, testlines, time_report and the tests (sessions' machines, CI), and through content.json every Worker that opens a zip or a preview; (2) the page's one same-origin fetch (origin control = page control, so nil gain, except a shared preview origin holding another PR's content.json one path away); (3) 4d's dev server (same files); (4) later: the server's own disk and player-made maps, where the same loader and static world face hostile data on the authority.
- Gains: Worker trap (no sentence) / later server crash; memory or time without bound; out-of-range or non-finite values into parry; page/loader differential (drawn walls not simulated); a tool reading or writing outside content/ via a link or a data-chosen path (invisible to the guard); a secret pulled into content.json (published by previews) or echoed by a refusal into CI logs; loader text as markup, terminal escapes, or text pasted into Advisor chats via "Copy report".

## Ranked answer
The final text handed back to the main session is the ranked list (must 1-12, should 13-22, can wait 23-27), the must-not list and the not-read list; it is built from P1-P21 above. Key numbers: Export 5 `true`; LINKS-CHECK Rust places recordings.rs:18, two_instances.rs:42/:44, two_processes.rs:113/:115, files.rs:115; TS places replay.test.mjs:15, determinism.spec.ts:68/:70/:72.
DONE

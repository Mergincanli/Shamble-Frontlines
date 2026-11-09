# U4s last small round: rust-reviewer (fix2-review)

Worktree: /home/user/Shamble-Frontlines/.claude/worktrees/wf_aa2cc979-004-1 (uncommitted; base c8abc85)
Inputs: worktree's .checkpoints/netcode-proof-test/u4s-build.md "## Last small round"; my earlier check
u4s-fix-review-rust.md (this folder): G1 (bite id in two crates), G2 (no rustdoc step), npt-sim membership nit.
Rules (§7.1): CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/shared-target; CARGO_NET_OFFLINE=true; touch crates
before each build; df -h /home/user before each build (< 2 GB stop); git only inside the copy (no -C/-c); Write only
for files (no Edit tool in this session: this notes file is rewritten whole; no redirects/heredocs/python -c); plants in
place one at a time by my plant script (u4s-fix2-rust-plant.py, this folder: reverse replacement, sha256 checked);
counts equal run to run; commit nothing.

## Plan
1. Fingerprint (sorted sha256 of every file under spikes/netcode-proof + .checkpoints in the copy) and git status;
   per-file sha256 of the files I will plant in.
2. Read the three changes (check.mjs G1/G2/nit, check.test.mjs tests, l1 lib.rs:241 doc fix).
3. Per change, plants (in place, one at a time, restore + sha256), preferring ones the builder did not run.
4. False positives on the committed tree (id in one crate's src and its tests; doc warnings on either build).
5. Final: touch, df, check.mjs once; 10/10; counts 275 / 292 / self 48; fingerprint equal to step 1.
6. Verdict, gaps with exact fix, regressions, check line. DONE.

## Progress
(started 2026-11-09)
- Step 1: base c8abc857; git status 13 M + 10 ?? (fix pass's 21 + l1 lib.rs M + u4s-last-probe.mjs ??). Fingerprint
  (394 files under spikes/netcode-proof + .checkpoints, sorted sha256sum | sha256sum):
  9340e9438a0460d730de536798fc6430d558b22ab252c0c8082f5ef3a35ae8ea. No target/ or .gate/ in the copy. Per file (16 hex):
  check.mjs 81181036709009e5, check.test.mjs 6c2fc9935d2cb2dd, grid ede88bbedfcc0413, sim lib 073aba2702a23c67, seam
  a203150c12361d0b, l1 lib 4a9ce2686399d2ac, client lib c4c4797df588d9ab, lab probe 0867d8395126f8aa, lab main
  3db2f26d07a87e7a, room lib fd4db3af919701ef = the builder's end hashes.
- Step 2 read: check.mjs :1-15 header, :78-131 LEAF loop + nit, :219-301 lexRust/rustAttributes, :523 crateFiles (fs
  walk, so untracked files are read), :671-737 G1, :1304 run(env), :1340-1380 lint/doc, :1428-1490 DEPS_CHECKED,
  depsStep, STEPS; check.test.mjs :160-177 (nit), :1031-1121 (deps fixture), :1440-1490 (BITE_CLEAN + G1 test); l1
  lib.rs diff (one doc line: [`CANONICAL_TAG`] -> `CANONICAL_TAG`).
- Nit: the LEAF loop (check.mjs:82-83) already pushed "npt-sim is not a workspace member" (npt-sim is in LEAF), so my
  earlier nit ("silent if npt-sim is missing") was wrong; the change is a de-duplicated second statement, and B4/B4b/
  B4c show the sentence held by either. No fail-when-reverted is possible for behaviour that was already held. Closed.
- FINDING G1a (low, false negative; the regex was my own fix text's): BITE_ID `^PM-\d{4}-\d+/[a-z0-9-]+$` misses 16 of
  marks.json's 114 bite ids (node over marks.json bites[].id): PM-0001-4/B-S, -5/B-S-sim, -5/B-S-server-only, -6/B-S,
  -7/B-S, -8/B-S-region, -8/B-S-rank-pack, -9/B-S-pair, -9/B-S-reflood, -11/B-S-send, -12/B-N-ES, -12/B-N-SS, -13/B-N,
  -35/quinn-proto-0.11.17, -39/B-S, -44/B-S (uppercase; dots). BITE_ID's doc says "marks.json's ids". The B-S ones are
  busy-waits in the sim step, client encode, rewound-shot query, region query: future sites in npt-sim/room/client.
  Wide form `[A-Za-z0-9.-]+` on the committed tree: the only extra literals are judge src/marks.rs:847/:886, both in its
  #[cfg(test)] mod (:819), one crate: no false positive.
  Probe u4s-fix2-probe.mjs (read-only, in-memory fixture, this folder): "marks.json bite ids: 114; BITE_ID reads 98;
  misses 16"; wide form reads 114 of 114; one id in crates/room/src/rewind.rs and crates/sim/src/step.rs:
  PM-0001-7/B-S PASSES, PM-0001-5/B-S-sim PASSES, PM-0001-35/quinn-proto-0.11.17 PASSES, PM-0001-7/untrimmed refused.
- Doc step reading: RUSTDOCFLAGS overrides build.rustdocflags (cargo precedence: CARGO_ENCODED_RUSTDOCFLAGS >
  RUSTDOCFLAGS > config), and the step drops the first; a [lints.rustdoc] table is refused by deps (check.test.mjs:710).
  No `doc = false` in any manifest. Host build only: tcpinfo's non-Linux stub (cfg(not(target_os = "linux"))) is not
  documented (nit, known limit). Nothing pins STEPS.doc or its flag in check.test.mjs (as for fmt/clippy/test; nit).
- BITE_CLEAN has client tests/ naming -41/-3 and sim tests/ naming -17/-3, so reading tests/ fails the clean test:
  the tests-not-read half is pinned.
- Plants (u4s-fix2-rust-plant.py; each: status all 4 == baseline, plant, df 13G free, touch, run, restore == baseline):
  R1-l1-id-in-sim (grid.rs + bites-only `L1_BITE = "PM-0001-3/rtt-each-way"`): deps FAIL 0.5 s "PM-0001-3/rtt-each-way
    is planted in 2 crates, crates/l1/src/lib.rs:390 and crates/sim/src/grid.rs:204"; restored ede88bbe.
  R2-test-mod-legit (-17's id inside `#[cfg(test)] mod bite_ids { #[test] ... }` in grid.rs): deps PASS (no false
    positive); restored ede88bbe.
  R3-not-test-mod (`#[cfg(not(test))] mod shipped { pub const ID = -17 }`): deps FAIL "... crates/client/src/lib.rs:12
    and crates/sim/src/grid.rs:205"; restored ede88bbe.
  R4-comment (`// "PM-0001-17/client-rounding" ...` in grid.rs): deps PASS (no false positive); restored ede88bbe.
  R5-mask-not-code (check.mjs: the grep reads lexRust's mask, literals blanked): self FAIL "46 of 48 tests passed";
    deps PASS on the real tree (the grep blind); restored 81181036.
  D1-bare-url (room lib.rs + `/// ... https://example.com/room.` on a pub const): doc FAIL 1.2 s "error: this URL is
    not a hyperlink --> crates/room/src/lib.rs:7:27, `-D rustdoc::bare-urls` implied by `-D warnings`". Same plant with
    CARGO_ENCODED_RUSTDOCFLAGS=--cap-lints=allow and RUSTDOCFLAGS=--cap-lints=allow in the parent env: doc still FAIL
    (the step drops the one and sets the other). Same plant, plain `cargo doc --locked --no-deps -p npt-room` without
    the flag: exit 0, "warning: this URL is not a hyperlink" (so the flag is what fails it). Restored fd4db3af.
  After all: status 4 of 4 == baseline.
- Final (touch; df 13G free, 67% used; CARGO_TARGET_DIR shared; CARGO_NET_OFFLINE=true): `node check.mjs` exit 0,
  "check: all 10 steps pass": fmt 0.4 s; clippy 3.2 s; clippy-wasm32 0.5 s; doc 3.7 s "measured build and bites build
  both documented, rustdoc warnings as errors"; test 79.7 s "measured build: 275 tests passed, 0 failed, 0 reported a
  skip; bites build: 292 tests passed, 0 failed, 0 reported a skip"; deps 0.5 s; refs 528 tracked files; marks 81;
  self 48 of 48; redact 10 of 10, 157 items, 0 findings. Counts = the builder's two runs (275 / 292 / 48).
- After the final run, one more plant to try G1a's fix: R6-wide-id (check.mjs:674 alphabet [A-Za-z0-9.-]; touch, df
  13G): deps PASS, self 48 of 48 on the committed tree; the probe then refuses all four two-crate cases; restored
  81181036 == baseline. The tree the final run read is byte-equal to the tree now (sha256 below), so its line stands.
- Unchanged by me: fingerprint after = 9340e9438a0460d730de536798fc6430d558b22ab252c0c8082f5ef3a35ae8ea (394 files),
  equal to step 1; git status 23 entries, the same list; no target/ or .gate/ in the copy; plant status 4 of 4 ==
  baseline. Only mtimes changed (the rule's touch).

## Verdict: GAPS (one low, in G1; G2 complete; nit closed; no regression)
- G2 complete: doc step = my fix text; fails on a bite the builder did not run (D1, a different lint and crate), and
  holds against a parent env that caps lints; clean on both builds of the committed tree (doc 3.7 s PASS).
- G1 does what my fix text asked and fails when reverted (builder B1-B3; my R5 46/48), no false positive (R2, R4, the
  committed tree with ids in src + tests/ of the same crate and other crates' tests/), refuses new pairs (R1, R3).
  Gap G1a: the id alphabet (my own text) misses 16 of marks.json's 114 bite ids. Exact fix: check.mjs:674
  `export const BITE_ID = /^PM-\d{4}-\d+\/[A-Za-z0-9.-]+$/;` and in check.test.mjs's G1 test (a) every marks.json bite
  id matches BITE_ID (read join(HERE, "marks", "marks.json"), .marks.flatMap(m => (m.bites ?? []).map(b => b.id)),
  assert.deepEqual(filter(!BITE_ID.test), [])), (b) one row: "PM-0001-7/B-S" in crates/room/src/lib.rs and
  crates/sim/src/grid.rs refused (biteTwice). R6 shows the committed tree passes with it (deps PASS, self 48/48).
- Nits (carry, no fix this round): nothing in check.test.mjs pins STEPS.doc or its -D warnings (as for fmt/clippy/test);
  the doc step documents the host build only (tcpinfo's non-Linux stub undocumented); lintBothBuilds says "both
  linted" on a bites-only failure (builder's own note); a measured-build doc failure's one-line summary is cargo's last
  line ("warning: build failed, waiting for other jobs to finish..."), the detail is in the 40 lines below it.
DONE 2026-11-09

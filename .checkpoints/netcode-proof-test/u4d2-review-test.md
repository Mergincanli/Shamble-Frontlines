# U4d2 round 1: test-engineer review notes (log prefix te)

Role: test-engineer (origin/main .claude/agents/test-engineer.md). Copy I run in: .claude/worktrees/u4d2-build (the only runner there now).
Read copy (byte-identical): .gate/u4d2-r1. Base copy: .gate/u4d2-prefix (51520b4, spikes/ = bfda3cd).
Every cargo/check.mjs/loader run goes through u4d2-run.sh; logs .gate/u4d2/r1-te-<name>.log.

## Plan
1. sha256 of every changed and new file under spikes/netcode-proof in u4d2-build; compare with u4d2-r1.
2. Bites (§5 U4d2 Bites line; §4.14 hash-map row; U4D2-Q5, Q11, Q12):
   - the builder's six plants (p1-features .. p6-usize): read the logs; re-plant in my own form where I doubt it;
     each seen to fail by name in the check or test that must catch it.
   - PM-0004-21 hash-map and std-sin in the wasm step: can either verdict pass for the wrong reason
     (Wasm leg not run, equal digests because nothing hashed, committed scan stale)?
   - export-signature bite (an export taking an f64, Q7): plant it if the builder did not.
3. h1..h8 (brief's numbering) and check.test.mjs's new and re-pinned self-tests: assertion vs print, wrong-reason pass,
   missing test/clause.
4. Carries with a test: 52 (generator refusal), 65 (failing-test names), 66 (#[ignore] test run by name): one plant each where cheap.
5. The builder's TE2-5 entry: the check.test.mjs test that failed mid-build; does it still pin what it pinned at <base>?
6. One counted `node check.mjs` with -x at the end; step lines; counts as deltas on 399 / 446 / 52 / 10 steps.
7. End: every file byte-identical to (1)'s hashes; `git diff --stat -- spikes/netcode-proof/crates/sim` empty.
Also: settle by run any finding of rust/security/infra notes a run can settle.

## Notes (a line after each finding)

### Step 1: sha256 at start (u4d2-build = u4d2-r1 for all 17; `diff -rq` of spikes/ empty)
6e628ab736b6957504049ee913533b9292a8065907092e1acda272333733e082 Cargo.lock
16b1353c97000f60b7a148e757062b93cb3d2323e64dc2e6e75aac2626fece78 Cargo.toml
e657f7b8a70efb6752f4238196209928752384283f66851cd21be7797d30710e check.mjs
04edf7955cba6943dba52071ae0bc251f04fb96b09fceb0e62cb3f9f966a17cf check.test.mjs
c68877f33877359caf2a30965b94015f6a3f8f3f9542d48273cd6c5d7d0fab20 crates/client-wasm/Cargo.toml
da4e9b7ee9345dcb49313005ecacca96286ee056c87845c90a1136f3bd023bf8 crates/lab/src/lib.rs
50531c4fab4e4f5329dee1bfe9394f24be2d81ca4af3408828f99886f229e5d2 crates/lab/src/main.rs
64f48629f8d75d71e2050fca977cff2f97c211628481559f84eeeeb6b645dfd1 crates/lab/src/probe.rs
c1cc693bb3887225a02e3f93176981aa9617dad53ed21e3501c6610e884e594e crates/proto/Cargo.toml
d3aef8209bbec3beeca3d02bb2cdd151b205f26e7496af2dc7f27f9c728b57a4 crates/hash-wasm/Cargo.toml
76326a6455d94746cdf4c536f9622fbd742c46098b587ca18e142cc50c36314d crates/hash-wasm/co-initial-dump.txt
7c3c214b33e7b9a44bd6dda19c21c271d7c201f9d8e4d7027e37206ddac2418c crates/hash-wasm/loader.mjs
9d339055169e22dcd5f19f41121d7af62c6613dc84e6b305fa824a69e9e52d8b crates/hash-wasm/src/lib.rs
08b20e900953e72e950bd2a68d4739b6f2b332d46c16ca8001d1dcfd20a18896 crates/lab/clippy.toml
9979c409286b58e77f9638d11f511bfcbd3cca14973a1b741efed8011c3e376f crates/lab/recordings.json
3b11bc2c38ba27fd360e346c4f610f04270253b3b07323e2ff6025ea38e55cbe crates/lab/src/hash.rs
905ae0576ae6146ffdefc09bd6d5540e5b522489f7b80ee157d6395d9e98bc2f crates/lab/std-sin-scan.txt

### Resume after a usage cut (2026-11-12, about 21:00 UTC)
- Re-hashed all 17 files in u4d2-build: each equals step 1's list (no plant was left in place); git status shows the
  same 9 modified + 8 new files; `git diff --stat -- spikes/netcode-proof/crates/sim` empty. No r1-te-* log existed, so
  no run of mine had started. Re-read: role file (origin/main), runner, packet §5 U4d2 / §4.11 / §4.14 / §4.15, brief
  2.1-2.7 and 3.1, the builder's record, the rust/security/infra notes, hash-wasm lib.rs and loader.mjs, lab lib.rs and
  hash.rs (legs, serve, compare, vectors, tests), probe.rs/main.rs diff, check.mjs's wasm step, check.test.mjs's new
  tests (1905-2638).
- The builder's six plant logs read (p1..p6): each fails by name in the check or test that must catch it (P1 deps names
  npt-sim [bites] in the hash-wasm build vs the server's and npt-lab's; P2 three FAILs, name scan and read-back; P3 deps
  lib.rs:212 + rustc exit 101; P4 deps misplaced allow + unsafe, loader "refused: ... "extra""; P5 h2 (i) FAILED and the
  wasm step's h2 word 3; P6 h2 (i) 262 vs 260 words, native 1-2 27000/0, native-Wasm 27000 mismatching from SS-s1 tick 0).
  Doubted: P1 (npt-sim's own feature, not a dependency below it, and not one target's), P3 (rustc refuses it too).

## My plants and runs (planned; each one runner call, -x -k, restored by Edit, sha256 checked)
- TE-R0 (no plant): hash-wasm tests both builds with --test-threads=1 (h3's "before begin" leans on a fresh thread_local).
- TP1 PM-0004-74 my form: crates/sim/Cargo.toml gains a wasm32-only `libm` feature (force-soft-floats); deps must FAIL
  naming libm and the builds; Cargo.lock's sha256 unchanged.
- TP2 flags, env only: CARGO_ENCODED_RUSTFLAGS -C\x1ftarget-feature=+simd128; CARGO_BUILD_RUSTFLAGS -Ctarget-cpu=native;
  and RR-M2's fail-open (CARGO_ENCODED_RUSTFLAGS bleeding-edge + RUSTFLAGS -simd128,-relaxed-simd: deps PASS expected).
- TP3 a cargo config source: .cargo/config.toml [build] rustflags ["--codegen", "target-feature=+relaxed-simd"].
- TP4 an `unsafe` block inside a listed export's body (RR-M1 / SEC-U4D2-2): rustc and clippy accept, deps must refuse.
- TP5 Q7's export taking an f64 (`feed(word: f64)`): wasm32 build exit 0, deps must FAIL by name; the loader on it.
- TP6 carry 52: the generator refusal moved after generation: h5's generator test must FAIL (kept != 0).
- TP7 carry 65: failingTests keeps only the first name: check.test.mjs's carry-65 test must FAIL.
- TP8 carry 66: the v1 tag dropped from v1_datagram_sha256: h7 (i) and h7 (ii) by name on release must FAIL.
- TP9 TE2-5: the builder's `&& all.has("npt-sim")` removed: the base test "npt-sim ships with libm..." must FAIL.
- End: one counted check.mjs (-x, the six pre-builds), hashes, crates/sim diff.

## Results (a line after each)
- TE-R0 r1-te-r0-threads.log (-x -k, exit 0): hash-wasm `--test-threads=1` "test result: ok. 5 passed; 0 failed" and
  bites "ok. 7 passed; 0 failed": h3's "before begin" holds when it does not run first (libtest gives each test its own
  thread, so the thread_local is fresh). No finding.
- TP1 r1-te-tp1-features.log (-x -k, exit 1 by design). Planted diff, crates/sim/Cargo.toml after `rand_core.workspace =
  true`: old "rand_core.workspace = true\n", new "rand_core.workspace = true\n\n[target.'cfg(target_arch = \"wasm32\")'
  .dependencies]\nlibm = { workspace = true, features = [\"force-soft-floats\"] }\n". deps FAIL, 4 lines: "npt-sim's libm
  v0.2.16 resolves [arch, default] in the npt-server (x86_64-unknown-linux-gnu, measured) build and [arch, default,
  force-soft-floats] in the npt-hash-wasm (wasm32-unknown-unknown, measured) build", the same for npt-lab measured and
  for both in the bites set. `cargo tree --locked ... -i libm` exit 0 (no lock change). Restored: fb98df48 = HEAD's
  blob; Cargo.lock 6e628ab7. So the live check reads cargo's per-target resolution below npt-sim, not only npt-sim's own
  features (the builder's P1). No finding.
- TP2 r1-te-tp2-flags.log (-x -k, no file changed): CARGO_ENCODED_RUSTFLAGS=-Ctarget-feature=+simd128 FAIL (name scan +
  read-back); CARGO_BUILD_RUSTFLAGS=-Ctarget-cpu=native FAIL (name scan, x86-64 fma, wasm32 non-vacuity);
  **CARGO_ENCODED_RUSTFLAGS=-Ctarget-cpu=bleeding-edge with RUSTFLAGS=-Ctarget-feature=-simd128,-relaxed-simd: PASS
  ("check: all 1 steps pass", exit 0)**; the control, bleeding-edge alone: FAIL on simd128 and relaxed-simd. Cargo uses
  the first set of CARGO_ENCODED_RUSTFLAGS, RUSTFLAGS, target and build rustflags (cargo config docs), so the read-back's
  superset is fail-open when a negation sits in a source cargo skips. RR-M2 settled by run a second time (agree, Medium,
  before merge). The self-test (check.test.mjs:2173-2186's argv pin) pins the superset and has no negation case.
- TP3 r1-te-tp3-config.log (-x -k, exit 1 by design). Planted: .cargo/config.toml old "target-dir = \"../../.gate/
  netcode-proof-test/target\"\n", new the same + "rustflags = [\"--codegen\", \"target-feature=+relaxed-simd\"]\n". deps
  FAIL: "<copy>/.cargo/config.toml's rustflags passes \"-C target-feature=+relaxed-simd\"" and the wasm32 read-back lists
  simd128 and relaxed-simd. Restored 9ff1aa86 = HEAD's blob. No finding.

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

Task: RNG-CRATE for change-set 3's plan: which library gives the simulation its random streams (rust-reviewer, part "cs3-rng").

## Plan (blocks, in order)
- Block 1: read cs2-plan.md section 16 (RNG-CRATE), verifier-claims.md, ADR 0004 part 8 + PM-0004-27..29 + S8 + R1-DET-OQ5/OQ11, build plan card cs3 (line 316 on), crates/sim/src (seed), root Cargo.toml, Cargo.lock, rust-toolchain.toml. Write 10 lines: what the sim needs from a generator.
- Block 2: rand book upgrade guide 0.10 (update-0.10.html) + rand CHANGELOG 0.10.0: exact quotes on chacha20's like-named generators vs rand_chacha, and StdRng. Plain meaning.
- Block 3: chacha20 0.10.0 and 0.10.1 yanks: crates.io version list, RustCrypto/stream-ciphers chacha20 CHANGELOG, the issues/PRs. What 0.10.2 fixed, newer releases, open issues on rng output.
- Block 4: options side by side: (a) rand_chacha 0.10.0, (b) chacha20 0.10.2+ feature rng, (c) own ChaCha8, (d) others. Registry metadata dependency trees, licences, no_std, unsafe/SIMD backends, seed/stream/word-pos calls, value-stability promise.
- Block 5: test vectors: independent ChaCha8 vectors, file format/size, native + Wasm test route in this repo.
- Block 6: recommendation + the self-contained question for Mustafa.
- Then DONE and the answer from these notes alone.

## Notes (one line per finding)

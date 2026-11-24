# rust-reviewer, review round 1 of cs3a (head 5c923119, base origin/main 1ec6f6c)

Plan: read plan section 4, the card's steps 4 to 10 and my RNG note; read the diff (crates, Cargo.*, client scripts);
check rng.rs against RFC 7539 layout + draft vectors (own python ref); probe the rules_scan lexer with planted
constructs (copies under .gate/rust-reviewer-cs3a/); check the alloc counter and panic hook (std 1.99.1 source);
build devtools for wasm32 and native into .gate/rust-reviewer-cs3a/target and read the codegen of floor/ceil/trunc/
round/mul_add; check build.mjs remap and the two cargo-output readers; verdict last.

## Findings (one line each, as found)

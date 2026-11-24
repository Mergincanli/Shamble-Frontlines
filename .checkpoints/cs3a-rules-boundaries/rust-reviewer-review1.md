# rust-reviewer, review round 1 of cs3a (head 5c923119, base origin/main 1ec6f6c)

Plan: read plan section 4, the card's steps 4 to 10 and my RNG note; read the diff (crates, Cargo.*, client scripts);
check rng.rs against RFC 7539 layout + draft vectors (own python ref); probe the rules_scan lexer with planted
constructs (copies under .gate/rust-reviewer-cs3a/); check the alloc counter and panic hook (std 1.99.1 source);
build devtools for wasm32 and native into .gate/rust-reviewer-cs3a/target and read the codegen of floor/ceil/trunc/
round/mul_add; check build.mjs remap and the two cargo-output readers; verdict last.

## Findings (one line each, as found)
- F1 (Medium) devtools/src/rules/f32_ops.rs:67 + make_f32_vectors.py:154: "from_f64" is the same lambda as "mul" in the reference and its digest equals mul's (f32-expected.txt: both 5f3751cdca1bbf4d): f64(a)*f64(b) is exact, so the cast of it IS the f32 product; f64->f32 rounding of an f64 with 49-53 significant bits is never tested, and LLVM may shrink fptrunc(fmul(fpext,fpext)) to fmul float (codegen to confirm). Fix now: f64::from_bits(wide(a, b)) as f32 (+ the reference: rounded(Fraction(double)) with the NaN/inf/zero cases), regenerate the line.
- F2 (Medium, doc + host fallback) sim-wasm/src/panic.rs:8-9: "Writing it allocates nothing: a panic may be a failed allocation" is wrong as a reason. std 1.99.1 panicking.rs:822-823 calls payload.get() before a custom hook, and FormatStringPayload::get -> fill() allocates a String and formats the message (619-628, 640-642) for every panic with runtime arguments (bounds checks, expect, unwrap, assert_eq); and on OOM std never calls the panic hook: alloc.rs:372-378 ("the only path taken on stable with std ... does *not* call any user-defined code"), rust_oom 435-441 prints and aborts. So out of memory traps with length 0. The hook's own write allocates nothing (panic.rs:198-209 Display; Location Display). Fix now (doc only): say the hook's write allocates nothing, std formats the payload first, OOM never reaches the hook (the host's fallback text then stands; host.ts already falls back).

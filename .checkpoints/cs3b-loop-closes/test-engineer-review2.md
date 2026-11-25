# test-engineer, review round 2 of change-set 3b (head 0dfca749; round 1 head bbbf09e4; base 28254a73)

My copy: /home/user/Shamble-Frontlines/.gate/ci-wasm-wt/.gate/test-engineer-cs3b2 (detached at 0dfca749).
Round-1 notes: test-engineer-review1.md (claims 1-5 done there; findings: one High proven, two High candidates, two Lows).

## Plan
A1. High (bite whole): read worker-link.ts olderPosts, press-tally.ts heldWithoutPress, presses.spec.ts, loop.spec.ts,
    the shim; plant `bite-counted-taken` from plants-3b.py IN MY COPY; confirm it fails on a mark quantity (which line);
    look for a Worker that takes an older post and still passes (e.g. older post with the same held bits; a post
    taken out of order but not "older than the tick before"; the counter derived from what the Worker reports).
A2. Releases: press-tally.ts doc vs ADR 0002 row PM-0002-12 word by word; met / partly / not met; lost vs doubled
    release unseen; severity; the main session's position (report says so; release counters = owner's decision).
A3. Allocation: allocation.spec.ts header and body; #done list; 2.5 blocks/frame refusal; per-frame limit; runner at
    ~9 fps / 3.3 blocks per frame: can it go red or green wrongly? Re-measure in my copy (one run, port check).
A4. Lows: decoder_inputs.rs cap per type; codec.rs five acks / nine commands whole-message cases.
B6. Claim 6: every added/changed test file; any_bytes.rs (per-step assertions, can they fail, end-of-run floors:
    flaky or blind?); endpoint.rs, queue.rs, local_match.rs tests; keyboard.test.ts, press-tally.test.ts,
    worker-link.test.ts, sim-wasm.test.mjs. Re-run a sample of plants-3b.py in my copy; plant >= 5 of my own.
B7. Claim 7: make_f32_vectors.py narrowing() (24 cases), native run.
B8. Claim 8: the card's "Current step" numbers vs what I measure (cargo counts, node counts, Wasm size/sha, browser).
C.  Browser runs one at a time, port 4173 check before each.

## Findings (one line each, as found)

# verifier fixcheck-d3 unit d: verdicts

Branch claude/laughing-lovelace-ox61se, HEAD 9e54015. Started 08:07 Amsterdam (2026-11-03).
Facts: (1) WebGPU error model behind PM-0003-2; (2) rustc 1.99.1 div_euclid behind ADR 0004 part 7;
(3) std Hasher / slice Hash behind ADR 0004 part 9.

## Time rule
`TZ=Europe/Amsterdam date +%H%M` printed 0807 before any fetch: before 08:30, so nothing is fetched this run.
Each block below holds the draft's exact text and what would confirm or refute it; verdicts wait for the next run.

## Fetch audit
| time | host | URL | status |
|---|---|---|---|
| (none) | | | no fetch: run started 08:07, before 08:30 |

## Fact 1: the WebGPU error model behind PM-0003-2 (ADR 0003 line 1136). PREP, not read
- Draft text (scratch draft-fact1-adr0003-pm2.txt): "counts as a device error each `uncapturederror` event of the WebGPU
  device, its `lost` promise resolving, and each `webglcontextlost` event on the canvas (proposed, not researched: a
  WebGPU validation error reaches the page through the device's error event rather than as a thrown error, and whether
  each browser also prints it to the console, as an error, a warning or not at all, was not verified ...)"; bite: "a
  planted bind group on the renderer's device that leaves out one entry of its layout, which must count as a device
  error whatever the console shows".
- Confirms if the WebGPU spec (https://www.w3.org/TR/webgpu/, "Errors & Debugging": the dispatch-error steps and
  `GPUUncapturedErrorEvent`; `GPUDevice.lost`; `createBindGroup`'s device-timeline validation) says: (a) an error no
  error scope captures is fired as an `uncapturederror` event at the device, and not thrown; (b) `lost` is a promise
  that resolves when the device is lost; (c) a bind group whose entries do not cover every layout entry fails
  validation ("generate a validation error", invalid object returned). MDN's `webglcontextlost` page: the event fires
  on the canvas (HTMLCanvasElement) when the WebGL context's drawing buffer is lost.
- Refutes if: uncaptured errors only reach the console; `lost` is not a promise or never resolves; a missing entry is
  defaulted rather than an error; `webglcontextlost` fires somewhere other than the canvas.
- Also to read and record either way: whether `lost` resolves on a deliberate `destroy()` too (reason "destroyed"):
  if so, a hook that counts every resolution would count a teardown destroy (a possible low note: count only
  reason "unknown", or never destroy before the count closes); and the spec's own words on surfacing uncaptured
  errors to the console ("should"/"may"), which bear on the draft's "not verified" console note.
- Verdict: none yet (not read: before 08:30).
END fact 1 prep

## Fact 2: rustc 1.99.1's `div_euclid` behind ADR 0004 part 7 (lines 270-278). PREP, not read
- Draft text (scratch draft-fact2-adr0004-part7.txt): "std's pages promise both methods "the rounded infinite-precision
  result" (1.99.1's, read by the verifier on 2026-11-02, and 1.97.0's), yet 1.97.0's `div_euclid` gives 3, not 2, for
  (3 + 2^-22) ÷ (1 + 2^-23) in `f32`, as does its `f64` analogue"; PM-0004-22's bite describes 1.97.0's steps as
  "`trunc(a / b)`, corrected by one only when `a % b < 0`". ADR 0002 line 155 pins 1.99.1.
- Known without a fetch: local rustc 1.97.0 gives 3 in f32 and f64 (fixcheck-d3a scratch div_euclid_check.out);
  skeptic-6's disassembly shows trunc-then-correct. The arithmetic: a = 2b + 1 exactly, so a/b = 2 + 1/b =
  3 - 2^-23 + 2^-46 - ...; in [2, 4) the f32 spacing is 2^-22, so a/b lies just above the midpoint of 3 - 2^-22 and 3
  and rounds to 3; trunc(3) = 3; a % b = 1 exactly (`%` is exact), not negative, so no correction: 3. Every step is
  exactly specified by IEEE 754, so any implementation of that shape gives 3 on every target.
- Confirms (part 7 holds for the pin, and the same case fails on 1.99.1) if 1.99.1's source of `div_euclid`, `f32` and
  `f64` (std/src/f32.rs, f64.rs, or core/src/num/f32.rs, f64.rs if std delegates to `core::f*::math`), is
  `q = trunc(self / rhs)`, then `q - 1` or `q + 1` only when `self % rhs < 0.0`.
- Refutes (part 7 then needs a line saying the pinned version computes it differently) if 1.99.1 uses another method.
- URLs to try, in order: https://doc.rust-lang.org/1.99.1/std/primitive.f32.html (the `div_euclid` [src] link),
  https://doc.rust-lang.org/1.99.1/src/std/f32.rs.html, https://doc.rust-lang.org/1.99.1/src/core/num/f32.rs.html,
  and the f64 analogues.
- Verdict: none yet (not read: before 08:30).
END fact 2 prep

## Fact 3: std's `Hasher` behind ADR 0004 part 9 (lines 334-341). PREP, not read
- Draft text (scratch draft-fact3-adr0004-part9.txt): "no `usize` in the serialized or hashed bytes (proposed: `usize`
  is 32 bits on wasm32 and 64 bits on x86-64, so a length written as `usize`, or a hasher fed through
  `std::hash::Hasher`, whose derived `Hash` writes slice lengths with `write_usize`, would differ by target at tick 0;
  PM-0004-21 checks it)"; PM-0004-21: "or a hasher fed through `write_usize`".
- Local evidence (no fetch; rustc 1.97.0 on x86-64; scratch hasher_probe.rs and hasher_probe.out): a logging Hasher
  receives `write_usize(len)` before the elements for `&[u32]`, `Vec<u32>`, `[u32; 2]` and a derived struct's `Vec` and
  array fields; `usize` itself goes to `write_usize`; `str` writes its bytes and a 1-byte 0xff, no length. So the
  slice-length claim holds in behaviour on 1.97.0; the doc read is still owed.
- Confirms if https://doc.rust-lang.org/std/hash/trait.Hasher.html lists `write_length_prefix` (nightly) saying the
  `Hash` impl for `[T]` calls it, and its default (source: https://doc.rust-lang.org/src/core/hash/mod.rs.html) is
  `self.write_usize(len)`; and `write_usize`'s default is `self.write(&i.to_ne_bytes())` (4 bytes on wasm32, 8 on
  x86-64). Also read the `Hash` page's "Portability" section, if present (std saying data fed by `Hash` is not portable
  across platforms would turn part 9's "proposed" into sourced).
- To record either way: a stable Hasher may override `write_usize` (the probe's does), so "would differ" holds for
  std's default `write_usize`, not for every hasher; part 9's rule (no `usize`, one byte string hashed) stands either way.
- Verdict: none yet (not read: before 08:30).
END fact 3 prep

## Run 1 result
Claims judged 0 of 3, confirmed 0: all three not read (before 08:30; clock 0807 at the start, 0810 at the end).
No finding against the drafts. Next run: fetch the URLs above (allow-listed hosts only), fill each fact's verdict below
its prep block, extend the fetch audit, then the counts and the verdict.
END run 1

# perf-auditor, cs3a round 1 (head 5c923119, base origin/main 1ec6f6c)

PLAN: read AUDIT_BUGS_PERF + plan s4 + resume card 4-10; diff crates+packages; (1) host.ts per-wake path vs main: code read +
Node harness on the real SimHost with the built Wasm (ns/wake, bytes/wake, opt status, --trace-deopt), base and head;
(2) worker-link/messages per-block path: code read + Node harness (heap bytes per receive) + logs of allocation.spec;
(3) Ring.mean / FrameTimes.fps cost at 600 samples (Node microbench); (4) Wasm bytes raw/gzip/brotli, section sizes, against
ADR 0002 marks (PM-0002-29); (5) ChaCha8 words/s native release (scratch crate under .gate/perf-auditor-cs3a), copy vs no
copy; (6) build/CI times from logs + warm/clean timings of checkCrates' cargo tree; (7) anything else. Machine: this container.
Scratch and builds: .gate/perf-auditor-cs3a/ only.

## Notes (one line per measurement or finding)

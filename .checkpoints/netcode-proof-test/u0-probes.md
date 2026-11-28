# netcode-proof-test: U0, the container probes (2026-11-06, about 09:35-09:50 UTC)
The first build step of the approved plan (plan §12.1). Probe code is throwaway, under `.gate/netcode-proof-test/u0-wt/`
(gitignored, never committed); only these results are kept.

| Probe | Result |
|---|---|
| Disk | 30 GB available (the session's allowance) |
| Rust 1.99.1 (the plan's pin) | installed through rustup in 18 s: rustc 1.99.1 (58cf8f9c3 2026-10-13), cargo 1.99.1; components clippy, rustfmt; targets x86_64-unknown-linux-gnu and wasm32-unknown-unknown |
| A dated nightly for cargo-fuzz | `nightly-2026-11-01` exists (rust 1.101.0-nightly, f9ceb4a4d 2026-10-31); installed at U5 when fuzzing starts |
| crates.io | reachable: wtransport 0.7.2, web-transport-quinn 0.13.6 (inside the drafts' 0.7.x and 0.13.x), wasm-bindgen-cli 0.2.130; the CLI is installed at U1/U7 at `Cargo.lock`'s version |
| Version floors (PM-0001-35), as resolved for the probe | quinn-proto 0.11.19 (≥ 0.11.18), rustls 0.23.46 (≥ 0.23.46), ring 0.17.14 (≥ 0.17.12), tokio 1.54.1 (≥ 1.54.1), quinn 0.11.12 |
| Chromium for local rough checks | Playwright's Chromium 153.0.8010.12 launches headless |
| WebTransport over loopback QUIC | **PASS:** a wtransport 0.7.2 echo server with `Identity::self_signed` (hash-pinned) on 127.0.0.1:4433; Chromium 153 on `http://localhost` (a secure context) opened it with `serverCertificateHashes` in 15 ms; datagram echo OK, bidi stream echo OK; Chromium's `maxDatagramSize` 1024 B, the server's `max_datagram_size` 1295; **control:** a wrong hash is refused ("Opening handshake failed") |
| IPv6 | this container has none: binding `[::]` fails ("Address family not supported by protocol"), so local tests bind 127.0.0.1 |

What this settles: the container can build and run the Rust side, drive Chromium against it over loopback QUIC, and
pin a certificate by hash, so U1-U8's local proofs can run here. What it does not: Firefox, Safari and Brave (R1's
smoke page on real machines), and R5-TEST-OQ2 (a custom quinn endpoint under each crate: U3).

# security-auditor: U6b Block 0 dependency read (DEP-HOLD), 2026-11-22

Subject: u6b-brief.md rev 1.1, section 0.1 condition 2, Block 0 (section 5); the eight items of
advisor-u6b-brief.md section 4 (:183-216); plus the ring-for-aws-lc-rs ruling. Read only: no install, no build;
read-only `cargo tree --locked --offline` / `cargo metadata --offline` on the lane's current lock; the after-state
modelled from the manifests and the published crates in the local registry cache.

## Plan
1. Baseline: lane head, hashes of the manifests and the lock, the current aws-lc-rs edges in the lock.
2. Item 1: the exact manifest content (workspace, npt-net, npt-server), U6-Q13 term (1).
3. Item 2: the four provider lines (wt.rs, wt_tests.rs, ws_config.rs) and web-transport-quinn's provider resolution.
4. Item 3: the exact lock diff (what reaches aws-lc-rs and its build-only crates; what else changes).
5. Item 4: the feature tree after (rustls, quinn-proto, rustls-webpki, web-transport-quinn, rcgen, time).
6. Item 5: what `default = ["wt-wtq"]` puts in the room binary (url 2.5.9, Url::origin()).
7. Item 6: bites gate.
8. Item 7: ring's key-exchange groups; the ruling on ring vs aws-lc-rs (anything weaker; rustls features).
9. Item 8: cargo-deny 0.22.0 / cargo-audit 0.23.0 yes; what "clean" means here.
10. Verdicts per item; binding check; DONE.

## Notes (saved as I go)

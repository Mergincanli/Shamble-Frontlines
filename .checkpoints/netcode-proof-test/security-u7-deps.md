# security-auditor: U7 Block 0 dependency short read (DEP-HOLD)

Reader: security-auditor (Opus 5.5), 2026-11-23. Branch claude/netcode-proof-test-plan-hcky5g.
Inputs: u7-brief.md rev 1.1 section 5 (Block 0) and section 0.1; advisor-u7-brief.md (sha256 65e109aa...044df3, checked)
C6 (:167-173), C7 (:175-181), C10 (:197-220, items (1)-(10)). Earlier reads reused, not redone: security-r1-deps.md,
security-u6c-deps.md.
Rules for this read: no npm install / ci / audit, no cargo fetch / update / build; read-only `cargo tree --locked
--offline` and registry/OSV reads only; no npmrc contents read (`npm config get` only).

## Plan
1. Confirm the hash and the five "covered already" cites C10 names.
2. Read the npm rule's sources: advisor-u8-brief.md C2, U8's form in wip/u8-build.patch; .gitignore for node_modules.
3. Registry facts (read only): typescript 6.0.3's publish time, dist.integrity (and an independent sha512 of the
   tarball in memory), install scripts, dependencies; newest 6.0.x; OSV for npm typescript 6.0.3 (npm audit's stand-in).
4. check.mjs's redact step (F26) against a lock of the expected shape.
5. Block 0b: today's manifests and lock; wasm-bindgen 0.2.130's manifest features; who brings wasm-bindgen into the lock
   today and with what features; the four array lines over U6c's modelled lock (count, hash); the feature tree
   expectations of item (9), including the dev edges `-e features` pulls in.
6. Answer items (1)-(10) by number; findings; per-half verdict; Block 0a's commands; dependency-check.md lines; 0b's
   binding check. DONE.

## Notes (one line per finding, saved as I go)
- Step 1: advisor-u7-brief.md sha256 65e109aa7732307310bbdee71053d72a96613ac65e3cc277581b4a6506044df3 = the brief's
  section 0.1. Cites hold as C10 states them: security-r1-deps.md:238 "typescript 6.0.3 exact (U7): CLEAR"; :245-247
  the npm rule (`npm ci --ignore-scripts`, web/.npmrc, committed lock, registry + sha512, `npm audit` recorded);
  :218-220 SEC-R1D-6 (`--before` 7 days back); :232-233 wasm-bindgen =0.2.130 CLEAR WITH CONDITIONS (std only, no
  serde-serialize, no web-sys / wasm-bindgen-futures, js-sys =0.3.107 allowed); :234-237 wasm-bindgen-cli 0.2.130 "(U7
  here, U8 on RS)" by `cargo install --locked`, default features, binary `wasm-bindgen` only; :276-277 its crate
  sha256 c37f6904...fc7a; security-u6c-deps.md:186-190 cargo-deny 0.22.0 and cargo-audit 0.23.0 "YES again".

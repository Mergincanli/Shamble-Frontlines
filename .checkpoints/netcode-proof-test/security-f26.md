# security-auditor: F26 short read (redact-check npm tarball skip, commit 2a9498d)

Reader: security-auditor (Opus 5.5), 2026-11-22. Branch claude/netcode-proof-test-plan-hcky5g.
Read-only: `git show 2a9498d`, redact-check.mjs, redact-check.test.mjs, advisor-u6c-brief.md section 6 (:367-388),
package-lock.json:606. Runs allowed: `node --test .../redact-check.test.mjs`, `node .../redact-check.mjs <spike>`.
Probes (if any) as scripts under .checkpoints/netcode-proof-test/security-u6c/ importing scanText (no edit of a copy).

## Plan
1. Read the commit whole, the scanner header, NPM_TARBALL, scanText's HOSTNAME loop, the tests, the ruling.
2. Q1: scope of the skip (host-name reading only; wholly inside an https registry.npmjs.org tarball path, plain/scoped).
3. Q2: no other host, subdomain, userinfo, port form, non-tarball path, query or text outside gains the skip (probes).
4. Q3: dotted, IPv6, mapped, URL-decimal, token, canary still scan that path (code + probes).
5. Q4: what could still hide (dashed address in a tarball file name) and does it matter for this threat model.
6. Q5: do the tests and the plant prove the narrowing; run the test file and the spike scan.
7. Findings, verdict, DONE.

## Notes (one line per finding, saved as I go)

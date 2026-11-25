# security-auditor: early read of preview-proposed.yml + the token check (advisor agent's Finding A), 2026-11-25
## Plan (blocks)
E1 sha256 + one-line diff vs texts/preview.yml. E2 read preview-proposed.yml whole against G1-G9 (plan notes).
E3 hosting/preview.mjs verifyToken + readUrl + comment (the token check's added duty, F3 split). E4 the window where
an unmatched link is used (environment URL, header fetch in `after`). E5 download-artifact v8.0.2 zip slip.
E6 O1 decision. E7 answer.

## E1
- sha256sum texts/preview-proposed.yml = 31119c06e7d095baa3c99ed54bbc1a2c72d05b8b3cab88ffd4512dc12acd3abc, 10,577 bytes;
  texts/preview.yml = c66c0b0a9351...; `diff`: only line 50 (`node --test hosting/` -> `node --test
  hosting/preview.test.mjs`). hosting/ holds one test file (preview.test.mjs, 805 lines), which carries the
  patterns, refuse list, check-build, recheck, find-pr, verify-token, read-url, check-headers... suites: the narrower
  argument drops no test file.

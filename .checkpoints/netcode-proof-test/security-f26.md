# security-auditor: F26 short read (redact-check npm tarball skip, commit 2a9498d)

Reader: security-auditor (Opus 5.5), 2026-11-22. Branch claude/netcode-proof-test-plan-hcky5g.
Read-only: `git show 2a9498d`, redact-check.mjs, redact-check.test.mjs, advisor-u6c-brief.md section 6 (:367-388),
package-lock.json:606. Runs allowed: `node --test .../redact-check.test.mjs`, `node .../redact-check.mjs <spike>`.
Probe: .checkpoints/netcode-proof-test/security-u6c/f26-probe.mjs (imports the real scanText; plants are in-memory
copies loaded through a data: URL, no file written, nothing edited).
This file names no address in a scannable form (samples are described, never written out).
Paths below: `rc` = spikes/netcode-proof/ops/operator/redact-check.mjs, `rct` = its .test.mjs.

## Plan
1. Read the commit whole, the scanner header, NPM_TARBALL, scanText's HOSTNAME loop, the tests, the ruling.
2. Q1: scope of the skip (host-name reading only; wholly inside an https registry.npmjs.org tarball path, plain/scoped).
3. Q2: no other host, subdomain, userinfo, port form, non-tarball path, query or text outside gains the skip (probes).
4. Q3: dotted, IPv6, mapped, URL-decimal, token, canary still scan that path (code + probes).
5. Q4: what could still hide (dashed address in a tarball file name) and does it matter for this threat model.
6. Q5: do the tests and the plant prove the narrowing; run the test file and the spike scan.
7. Findings, verdict, DONE.

## Notes (one line per finding, saved as I go)
- Step 1 done. Commit 2a9498d touches only rc (+12: NPM_TARBALL at :137-144, the skip at :237-241) and rct (+11: the
  F26 test at :52-61). No later commit touches either file (`git log`; `git diff 2a9498d -- ops/operator/` empty).
  sha256 of rc on disk: 2609bc87c9248558f8334731a07b0ce82479ced427f18d70af1ee7e130519d31 (the commit's 2609bc87).
- Runs: `node --test rct` 11/11 pass (test 3 is F26's); `node rc spikes/netcode-proof`: "340 item(s) checked, 0
  finding(s)". Both as the commit message says.
- Probe on the real code: 33 cases, all as expected once one expectation of mine was corrected (case AF: a mapped
  address inside a would-be path gives the mapped finding AND the host-name finding, because `:` breaks the tarball
  form, so no skip applies: stricter than I wrote, not a defect).
- Plants (in memory): any-host fails 2 of the F26 test's assertions (another host's path; address-shaped host); the
  plants no-`/-/`, no-`.tgz`, end-bound-to-text-end and skip-every-host-name-once-any-tarball-is-present all SURVIVE
  the committed tests -> SEC-F26-1.
- Residue (dashed / rDNS-embedded address inside a tarball-form path, wherever that text sits) -> SEC-F26-2, accepted.

## Q1. Scope of the skip: CONFIRMED
- Host-name reading only: `tarballPaths` is built at rc:238 and read once, at rc:241, inside the HOSTNAME loop
  (rc:239-249). DOTTED (rc:219-226), V6/mapped (rc:227-236), URL_DECIMAL (rc:250-253), SECRET_PATTERNS (rc:254-256),
  LONG_TOKEN (rc:257-259) and canaries (rc:260-262) never consult it.
- Wholly inside the path: rc:241 skips only when `from <= m.index && end <= to`, with `from` the first `/` at or after
  index 8 of the match (rc:238; "https://" is 8 characters and the host holds no slash, so it is the slash after the
  host) and `to` the end of `.tgz` (rc:238). A HOSTNAME match cannot cross `to` anyway: NPM_TARBALL's lookahead
  (rc:144) forbids every HOSTNAME character after `.tgz`.
- Exact host: NPM_TARBALL (rc:144) is the literal `https://registry\.npmjs\.org\/`, no `i` flag, `/` straight after
  `org`. Tarball form: optional `@<scope>/`, one `<name>` segment (no slash), literal `/-/`, one file segment, `.tgz`,
  then no path character. Plain and scoped both pass (rct:54-55; probe A, B).

## Q2. Nothing else gains the skip: CONFIRMED, with one precision note
- Each keeps the host-name reading (probe on the real code): another host (C, rct:56); `http://` (D); port `:443` (E);
  userinfo before the host (F); `registry.npmjs.org@other.host` (G); trailing-dot host (H); the registry name as a
  label of another host (I); an address-shaped subdomain of the registry, host and path both flagged (J, rct:58-60);
  upper-case host (AD); percent-encoded scope (AE); no `/-/` (K); a root path (L); not `.tgz` (M); a segment after
  `.tgz` (N); a query (O) or fragment (P) after `.tgz`; text after the URL on the same line (Q) or the next (R); a
  second URL to another host after a registry URL (AG).
- Precision note (folded into SEC-F26-2): NPM_TARBALL has no left boundary, so text shaped like a registry tarball URL
  gains the skip for its own path wherever it sits: after `git+` (AA), after any prefix such as `xhttps` (AB), or
  nested inside another URL's query (AC). The host is still the exact registry literal and the path still the tarball
  form, so this widens where the form may sit, not what can hide in it (Q4).

## Q3. The other rules still scan that path: CONFIRMED
- Dotted address inside the file segment (rct:57) and inside the name segment (V): "IPv4 address".
- Join token (Y) and long token (Z) as the file name: flagged.
- An IPv4 canary's dashed form in the file name (W): "canary".
- IPv6 and IPv4-mapped: `:` is in none of the path classes (rc:144), so such text breaks the tarball form and no skip
  exists at all (AF: mapped finding plus host-name finding).
- URL-decimal (rc:136) reads only the position after `://` (a URL's host), so inside a tarball path it could only fire
  on a nested `://`, which the path classes exclude; it is untouched by the skip either way.
- No ReDoS: every NPM_TARBALL attempt starts at the literal `https://registry.npmjs.org/`; the one backtracking point
  (the file class, which holds `.`, before `\.tgz`) is linear in the run, and runs end at the next `:`.

## Q4. What can still hide there, and whether it matters
- Hidden by the skip: a dashed or rDNS-embedded address (forward or reversed) inside the scope, name or file segment
  of text in registry tarball form, e.g. a file name made of `ip-` plus four dashed octets (S), the same as the name
  segment (T) or the scope (U); and an IPv4 canary's reversed-dashed form there (X), which the canary expansion
  (rc:185-204) lacks while the host-name reading would have caught it.
- Not hidden: the dotted form, IPv6 and mapped forms, tokens, keys, forward canary forms (Q3).
- Matters? No, for this repository's threat model. The scanner guards results and commits against careless leaks
  (logs, notes, command output, results files); none of those wraps an address in `https://registry.npmjs.org/<name>
  /-/<file>.tgz`, and npm writes such URLs from package names and versions, never from an address of ours. Hiding one
  there takes a deliberate committed change past review, which this threat model rates Low (memory: SEC-U4D2-4), and a
  deliberate leaker has easier forms no text scanner reads (spelled-out digits, base64). The laptop's pre-push hook
  runs the same file, so it has the same residue and no wider one.
- Outside F26 (pre-existing, not counted here): a dashed address in plain text with no dot is no HOSTNAME match, and a
  reversed-dashed canary outside any host name is caught by no rule; the header (rc:6-9) claims neither.

## Q5. Do the tests and the plant prove the narrowing: PARTLY -> SEC-F26-1
- Proven by the committed test and plant: the host condition (the any-host plant fails rct:56, and in my replay
  rct:60 too); the dotted rule inside the path (rct:57); an address-shaped subdomain keeps both readings (rct:58-60).
- Not pinned by any committed test: the path form (`/-/` and `.tgz`) and the end bound. In-memory plants that drop
  `/-/` (K, L widen), drop `.tgz` (M widens), set `to` to the text's end (O, P, Q, R, AG widen: every host name after
  the first registry tarball URL in a file is skipped), or skip every host name once any tarball URL is present, all
  pass rct 11/11. Today's code is correct (probe), so this is a regression guard, not a defect. rct:52's title ("and
  nothing else is excused") claims more than its assertions show.

## Findings
- **SEC-F26-1 (Low): the committed tests pin the host condition only.** What: no assertion fails if a later edit
  drops the `/-/` or `.tgz` part of NPM_TARBALL or widens the end bound at rc:238/:241. Exploit sketch: a careless
  "simplify the regex" edit makes every host name after the first registry URL in a package lock or results file
  unscanned, and 11/11 still pass. Evidence: rct:52-61; probe plants no-`/-/`, no-`.tgz`, end-to-text-end and
  skip-all survive. Fix (test-only, so rc's sha256 2609bc87 and the hook's EXPECTED do not move; U8 fix-pass item 6
  unaffected): add to rct's F26 test, with values built at run time, (a) `https://registry.npmjs.org/crc-32/<file>`
  (no `/-/`) and `.../crc-32/-/<file with .tar>` each give one "address in a host name"; (b) a lock line followed by
  ` then ip-<four dashed octets>.example` on the same line, and the same on the next line, each give one; plants:
  `/-/` made optional, and `to` set to `text.length`, each must fail an assertion; restore and sha256-check rct.
- **SEC-F26-2 (Low, accepted, no action): the residue of the skip.** What: a dashed or rDNS-embedded address (or an
  IPv4 canary's reversed-dashed form) inside a registry-tarball-form path, wherever that text sits (also after `git+`
  or any prefix, or nested in another URL's query), passes. Exploit sketch: a deliberate commit names a "package"
  whose name or file encodes an address in dashes. Evidence: probe S, T, U, X, AA, AB, AC. Why accepted: no careless
  route produces that form, a deliberate one is a committed change past review (Low in this threat model), and a text
  scanner cannot stop deliberate encodings anyway. This is the residue advisor-u6c-brief.md:373-378 chose over (ii).

## Verdict: CLEAR WITH CONDITIONS
The fix does what the ruling (advisor-u6c-brief.md:373-382) says and no more: the skip touches only the host-name
reading, only for a match wholly inside the path of `https://registry.npmjs.org/` text in npm's tarball form, plain
or scoped; every other host, scheme, port, userinfo, subdomain, path shape, query, fragment and following text keeps
the reading; the dotted, mapped, token and canary rules still scan the path (IPv6/mapped cannot sit inside it).
Tests 11/11, scan 340 items 0 findings, sha256 2609bc87 confirmed. No Critical, High or Medium.
Condition: SEC-F26-1's test-only addition (no change to rc, so nothing that pins rc's hash moves), in a lane commit
before U6a's PR is marked ready, or deferred explicitly by the advisor as a STATUS follow-up (it is Low; an open
security finding keeps the PR needs-mustafa, advisor.md PR review step 4). SEC-F26-2 accepted.

DONE

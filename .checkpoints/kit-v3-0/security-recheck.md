# security-auditor recheck of 1c259ce (kit-v3-0), 2026-10-25
- tests: `node --test --test-timeout=60000 .claude/scripts/checkpoint.test.mjs` 17/17 pass, 22.9 s.
- F1 (secret check): code part done (checkpoint.mjs:26-34, 98-107, 138-143). Repo push protection still OFF
  (gh api: secret_scanning + push_protection disabled). `if (!blob.ok) continue` fails open (line 103).
  diff-filter=AM: renames may skip the scan (to probe).
- F2 (web): domain list in settings.json:42-103 (62 domains); 7 agents have the rule. plan.md:165 still says "agents
  have no secrets to leak". Attacker-writable allowed domains: github.com, raw.githubusercontent.com, docs.rs, crates.io,
  developer.valvesoftware.com (wiki), datatracker (anyone submits I-Ds), learn.microsoft.com (Q&A). Exfil via them weak.
- F3: skip marks refused (:25, :63-66); file guard (:80, :134, :165, :177); merge-commit diff-tree part (c) not done (:90-93).
- F4: NIGHT_SHIFT rules 2 and 4 + CHECKPOINTS §8.1 done as proposed.
- F5: deferred; deferral not found written on the branch (grep). Narrow Edit list would not hit agent-memory.
- F6: header fixed (:2-3); watch skips push when code ahead (:150-153). TOCTOU: push sends HEAD, not the saved sha.
- F7: bounds (:67).
- next: probes in scratch repo (rename, URL-encoded token, false positive blocks whole batch, race).
## probes (scratch repos in $TMPDIR, drivers in the session scratchpad recheck/)
- 1c259ce: renamed note + token PUBLISHED exit 0; quoted/non-ASCII name + token PUBLISHED exit 0 (git show fails, skipped).
- 612cc5a (HEAD, private index, -z, --no-renames, unreadable = hit): both refused. 19/19 tests. No rename test.
- Both: `%3Dghp_...` committed (\b); JWT, npm_, AIza, sk-proj- committed; session's own CLAUDE_CODE_MESSAGING_TOKEN
  (32 hex) not matched by any pattern (names-only check, no values printed).
- Both: one false-positive file (postgres://app:<example password removed>@...) blocks all notes, exit 1.
- Both: watch published a code commit made during push retry (push sends HEAD, line 126); evil merge published with [skip ci].
- Docs (code.claude.com permissions + permission-modes): domain: exact host; Edit rules cover Write; .claude is a
  protected path (agent-memory .md exempt); explicit ask rules prompt even in auto and bypass, denied in dontAsk.
- WRITE-GUARD deferral written in docs/reports/2026-10-24-kit-v3-0.md (untracked draft). Accept, with corrections.

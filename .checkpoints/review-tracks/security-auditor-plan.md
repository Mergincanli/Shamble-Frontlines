# security-auditor: plan review, "review tracks and who merges" (plan.md as of 2026-11-26 18:50)

Verdict: REVISE. Public notes: ids, severities, class words only. Exploit forms live only in the session scratchpad.

## Sources read (summary each)
- plan.md (487 lines), inputs-A.md, inputs-B.md (R2/R3/R4 rows), checkpoint.md.
- routine-guard.mjs: mergeAllowed :84-96 (charset rule :85; Windows limits stated :80-81); mcp() :1505-1517 (day-session
  pass-through `!role` :1514; merge check wrapped fail-closed :1508); githubWrite :1520-1553 (routines may comment);
  mergeCheck :1555-1593 (no file list or comment check for day sessions; toolchain not required :1568).
- guard.mjs :40-48: a throw or load failure prints nothing without a role (day session fails open to the normal
  permission flow).
- settings.json: allow list :79-89 has add_issue_comment, update_issue_comment, issue_write, create/update PR; no
  merge tool; deny list has plain-spelling gh merge and gh api write patterns.
- Rulesets (gh api, read only): "Shamble Ruleset 1" requires gate and smoke from integration 15368 (GitHub Actions),
  PR required, bypass empty; toolchain NOT required. "main-wall": update rule, bypass RepositoryRole 5 exempt.
- docs.github.com REST issue comments: performed_via_github_app is in the schema only; no statement of which token
  types set it (undocumented: measure). Comments expose created_at/updated_at, no edit history.
- docs.github.com rulesets: a required check can name an expected source app; otherwise any writer may set the status.
- code.claude.com/docs/en/hooks (PreToolUse decision control): "allow" skips the permission prompt; deny and ask rules
  are still evaluated; exit 0 with no output = normal permission flow; failures block only with onFailure: block.
- agents/*.md :4: custom agents have no GitHub MCP tools; built-in and workflow agents inherit them.
- DECISIONS.md:183-200: entry headers state bytes and sha256 computed by the copying session from a session-saved file.
- advisor.md:66-67, :78-86 (tier-2 path list incl. gitignored, symlink, case twins); :103, :180-181, :267-268 and
  security-auditor.md:54 ("left open at any severity": a kit rule; no source in DECISIONS).

## Findings (id, severity, class)
- SA-RT-1 Critical: FORGE-MERGE (the one condition replacing his go is a comment any session, built-in subagent or
  routine writes with no prompt; with MERGE-PROMPT (a) the merge is promptless too; plan 4a "nothing stops it" is
  inaccurate: the guard reads every MCP call whole).
- SA-RT-2 High: AFTER-CHECK-BLIND (the after-check reads only what a forger writes; "without his go" unknowable).
- SA-RT-3 High: WORDS-HASH-SELF (DECISIONS hash is self-attesting; cannot tell his typed words from session text).
- SA-RT-4 High: PROMPT-WIDE (allow line removes the prompt for way (2) merges and subagents; guard load failure then
  merges unchecked; "every merge" claim does not hold).
- SA-RT-5 Medium: TRACK-SELF (which copy of track.mjs classifies; empty diff).
- SA-RT-6 Medium: FLOOR-NARROW (track's tier-2 floor narrower than advisor.md's: gitignored, modes, twins, charset,
  binaries).
- SA-RT-7 Medium: CI-SELF-GREEN (light paths decide what toolchain runs; toolchain not a ruleset check).
- SA-RT-8 Medium: LIGHT-SEC-TRIGGERS (light track drops step 5's security subjects; CSP page light).
- SA-RT-9 Medium: SEC-OPEN-RULE (row 10 conflicts with security-auditor.md:54 and advisor.md:267-268; no OLD/NEW).
- SA-RT-10 Medium: EYES-GAP (eyesPaths miss "hears/feels" paths; identical renders prove "sees" only).
- SA-RT-11 Medium: WAKE-AUTH (WAKE-END (b) moves the advisor approval onto a forgeable comment).
- SA-RT-12 Low (folded): binding, edit and time rules for the check comment; light-list case widening; T-MARK routes;
  wake events carry outsiders' text; guard edits land ahead of R4-1's change-set.

## R4-1 interplay (class words)
R4-1 is Bash-only; MCP calls are read whole, so the proposed guard fixes hold while it is open. Bash-route merges and
comments bypass the guard's reading but meet a prompt and GitHub's ruleset (PR + gate + smoke from Actions).

## Decisions I would answer differently
MERGE-MARK, MERGE-PROMPT, WAKE-END, PLAN-GO (revise first). Agree: GUARD-TOOLCHAIN (a) (+ ruleset follow-up),
TRACK-DOUBT (a) (+ additions), REVIEWER-SLOTS (a), PRADV-TRACK (a). No security view: LIGHT-EFFORT, ACCT-LINES.

## Status
Review complete; handed back.

## Round 2: revised plan (670 lines), re-review
Verdict: APPROVE. SA-RT-1..12 closed as written; departures accepted; nothing loosened beyond what the plan lists
(section 5 items 12-14). Open Lows for the build (no re-review of the plan needed):
- SA-RT2-1 Low: TABLE-DRIFT (the guard's allow reads track.mjs and itself from disk; allow only when both equal
  origin/main's blobs, else silence = prompt).
- SA-RT2-2 Low: AFTER-CHECK-EXCLUDE (excluding merges named in the routine's own notes trusts a writable note;
  verify those merges against the allow-list instead of skipping them).
- SA-RT2-3 Low: MARK-SCOPE (GUARD-MARK scans every string value of the tool input recursively; the reader matches
  exactly, no trim or Unicode folding; the comment list is read whole across pages).
- SA-RT2-4 Low: LEGACY-TRIGGER (light security triggers scoped to packages/ and tools/ leave legacy/ out).
Departures: names-only allow (content floors to his Advisor's PASS, which no GitHub tool can write) accepted; disk
table accepted with SA-RT2-1; 24-hour look-back accepted (his Advisor's list is the forgery check either way).
WAKE-END: narrowed (b) acceptable to security now (in-session advisor run required); (a) as default is fine.

## Quick look: MERGE-PROMPT (d) (his decision), plan at 705 lines
Verdict: sound; APPROVE; ready to build. Nothing loosened beyond his decision (the protected-path layer goes by his
choice; recorded in risk 1 and section 5). GUARD-MARK still closes SA-RT-1's promptless route.
- SA-RT3-1 Medium (build): BASH-MARK (under (d) a forged PASS reaches rule files, so the prompt-only Bash comment
  route weighs more; the guard should refuse the marker in plainly spelled gh comment, review and edit forms and
  file-sourced bodies for them; disguised forms still meet a prompt while R4-1 is open).
- SA-RT2-1 (guard-file half) Low: kept for the build review.

## Pre-apply review (build copy .gate/review-tracks/build, 2026-11-27)
Read: diffs of routine-guard.mjs, routine-guard.test.mjs, guard.mjs, advisor.md (both sides), security-auditor.md,
CLAUDE.md.new; track.mjs, track.test.mjs. Probes and patched copies: session scratchpad only.
Sources: code.claude.com/docs/en/hooks (agent_id: subagent or in-process teammate); gh 2.89.0 --help for every pr/issue
verb (comment, body and body-file flags; -R inherited); toolchain.yml:16-18 (no path filter); settings.json allow list.
Findings (id, severity, class):
- SA-PA-1 Medium: MARK-FORMS (SA-RT3-1 partly open; plain gh forms).
- SA-PA-2 Critical (by class; precondition unmeasured): MARK-THROW (fail open).
- SA-PA-3 Medium: PATH-TAB (track.mjs path parse; measured).
- SA-PA-4 Low: ARG-END (track.mjs end-of-options, literal pathspecs; no injection).
- SA-PA-5 High: RISK-INPUT-RUN (risk text does not hold; tied to INPUT-RUN; reach reasoned, not measured).
- SA-PA-6 Low: SERVER-NAME. SA-PA-7 Low: CRLF (fails closed). SA-PA-8 Low: AFTER-CHECK-WAY2. SA-PA-9 Low: TEST-PAGES.
- SA-PA-10 Critical (by class; pre-existing on main, measured; R4-1 class; meets a prompt): FLAG-POSITION.
- SA-PA-7 measured in part: API-posted comments carry no CR; a web-posted one not measured.
- SA-RT2-1 guard-file half: not needed now; accepted Low; revisit at the first change to passAllows or GUARD-MARK.
- Teammates and workflow agents: agent_id unverified for out-of-process teammates and workflow agents (not enabled /
  not measured); Agent-tool subagent measured live (this run's own calls took the agent branch).
Patched scratch copies: guard suite 1575/1575 (5 new mutants caught), track suite 61/61, plus a new git-backed tab case (passes patched, fails on the build copy).
Verdict: REVISE.

## Re-check of the changed hunks (build copy, 2026-11-27, not a new round)
Read: routine-guard.mjs diff vs main and vs my patched copy (only items 3-5 changed), track.mjs whole, the new tests.
Probes and a patched track.mjs: session scratchpad only (sa-recheck*.mjs, rc-patch.mjs, rc-patched/).
Suites re-run: guard 1581/1581, track 82/82; patched track copy 82/82 and every probe below fixed.
Findings (id, severity, class):
- SA-RC-1 Medium: BASE-REF (track.mjs base by short ref name; measured: floors lost; delivered by a plain fetch,
  measured; routines already refuse it before a script runs; day sessions not).
- SA-RC-2 Medium: IGNORE-CASE (repo-level git config drops the gitignored floor; measured with main's real .gitignore;
  the fixture pins the config, the code does not; global config alone does not, measured).
- SA-RC-3 Low: TEMP-LEAK (stop path skips the cleanup; measured, one folder per suite run).
- SA-RC-4 Low: CRAFTED-TREE (a folder made outside the temp folder before git refuses the path; measured; no file
  written; exit 1 not 2; reach: main's own tree only).
- SA-RC-5 Critical (by class; pre-existing on main; meets a prompt; reach unmeasured): MERGE-SCOPE. Not widened by
  this change-set; for Mustafa like SA-PA-10.
- SA-RC-6 Low: BUDGET-MSG.
Closed: SA-PA-4 (its defect fix sound apart from SA-RC-2/3/4; info/exclude and global excludes add only, measured with
negations); dir-only pattern without the folder covered by the fixture. SA-PA-7 closed (line-end handling, not
trimming; SA-RT2-3 holds). SA-PA-9 closed (filter text pinned by the mutants' exactly-once rule). SA-PA-6 closed.
Verdict: REVISE on SA-RC-1 and SA-RC-2 (both contradict track.mjs lines 9-10); fixes tested on the scratch copy.

## Narrow re-check of the built SA-RC fixes (build copy, 2026-11-27, not a new round)
Read: built track.mjs whole, routine-guard.mjs diff vs main (guard.mjs not re-read). Suites: guard 1581/1581, track 85/85.
Probes: session scratchpad only (sa-recheck5..8.mjs).
- Closed: SA-RC-1 (BASE-REF), SA-RC-2 (IGNORE-CASE), SA-RC-3 (TEMP-LEAK), SA-RC-4 (CRAFTED-TREE), SA-RC-6 (BUDGET-MSG).
- SA-RC-5 (MERGE-SCOPE): closed for the measured reach; same class remains as on main for Mustafa.
- New SA-RC-8 Medium: WORDS-SHAPE (measured on the built function).
- New SA-RC-7 Low: BASE-LOCAL (measured on the built script).
Verdict: REVISE on SA-RC-8; the other hunks approved.

## Re-check of the SA-RC-5 remainder, SA-RC-7, SA-RC-8 hunks (build copy, 2026-11-27, not a new round)
Read: routine-guard.mjs mcp() whole, track.mjs wordsCheck and git(). Suites: guard 1581/1581, track 85/85.
Probes: session scratchpad only (sa-recheck7..10.mjs).
- SA-RC-8 (WORDS-SHAPE): closed (tail-only quote floors; 24/24 real saved files with a "---" line match the shape).
- SA-RC-7 (BASE-LOCAL): closed (graft present: merge base unchanged, full track with the tier-2 floor).
- New SA-RC-9 Critical by class (MERGE-SCOPE remainder; ends at a prompt; day sessions only): measured on the built mcp().
Verdict: REVISE on SA-RC-9; SA-RC-7 and SA-RC-8 hunks approved.
SA-RC-9 measured further: routines (both roles) refuse every three-part name on main and on the copy; day session
returns no decision on main and on the copy (pre-existing, not widened; the hunk misses that shape). Trial fix (last
"__" segment, exact mcp__github__<name>) on a scratch copy: all shapes refused, the github merge still reaches the
merge check; suite 1580/1581 (only the mutation runner's text anchor for the changed line fails, expected).
Class: standard merge names closable here; other merge-capable tool names stay for Mustafa as on main.
SA-RC-9 hunk (build copy, 2026-11-27): APPROVE; closed for standard merge names (merge/auto-merge by last part, exact mcp__github__); RC3-L1 fold checked (comment-edit read on any connector, fail-closed on a non-GitHub tool, no widening); remainder class for Mustafa unchanged; probes sa-recheck9, sa-recheck11.

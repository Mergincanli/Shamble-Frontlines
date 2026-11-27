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

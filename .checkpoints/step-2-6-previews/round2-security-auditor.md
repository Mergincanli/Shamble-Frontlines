# security-auditor: round 2 on step 2.6 (PR #72), ad2658a..80a9b6a8, 2026-11-26
Read-only. Scratch: my scratchpad r2/probe.mjs (reads the repo, writes nothing); the te probes were NOT re-run (no copy
made). No wall detail, no guard-bypass detail here.

## Plan (blocks)
P1 diff list + preview.yml sha. P2 runbook Lows (4) with line numbers. P3 preview.mjs: reconcile dedup, hand-over
regex, token messages. P4 lint L2/L3: read, try shapes it misses (on a scratch copy). P5 tests run. P6 process finding
(B15, process-git-calls.md, guard_offline.mjs, diff --stat of what the move brought). P7 verdict.

## P1 diff + hashes (2026-11-26)
- `git diff --stat ad2658a 80a9b6a8 -- hosting/ docs/runbooks/ .github/`: runbook +108/-, preview.mjs 20, test 309; no
  .github change. sha256 preview.yml 31119c06...abc, preview-close.yml c2f1fa90...e66 (unchanged).
- `node --test hosting/preview.test.mjs` (real tree): tests 171, suites 13, pass 171, fail 0.

## P2 round-1 Lows in docs/runbooks/previews.md (at 80a9b6a8)
- Likely cause: :30-31 "The likely cause was the unsaved bypass box; the main rule may also have been saved only after
  22:29". CLOSED.
- Both checks after any change: :27 "After any change on that page, reload it and check the box and the rule again";
  renewal step 1 :93-94; rebuild step 3 :122-123. CLOSED.
- Environment secrets: :14-16 (environment page, never repository/Dependabot/Codespaces/variable); renewal :98-99
  (replace in the environment's box, then check no CLOUDFLARE_ name in Actions/Dependabot/Codespaces); rebuild :124-127
  ("Add environment secret" for both). CLOSED.
- Wrangler bump rule: :109-113 (tier 2, new dependency check: my answer on the two files, npm audit, signatures incl.
  the skipped package). CLOSED. The standing advisory watch for hosting/ (O2) is not in the runbook: stays with the
  security baseline (R6), as said in round 1; no finding here.
- Count as upper bound: runbook :80-85; code commentBody :443-445, log :502, warning :504. CLOSED.

## P3 preview.mjs changes
- Dedup listPreviews :568-570: keeps the first record per string id; non-string ids pass on and decide() refuses them
  (uuid check, R5). Only ever removes items; deletes by id, so a duplicate could only have caused a second DELETE of
  the same id (a red step). A record missed by the page shift is left for the next run (fail-safe). No finding.
- Hand-over regex :241: `/^[ \t]*name:[ \t]*(["']?)preview-site\1[ \t]*(?:#.*)?$/m`, linear (no nested quantifier),
  reads main's toolchain.yml only; a false match only turns a notice into a red error (fail loud). No finding.
- Token messages :291, :295: fixed text, runbook section names; nothing outside printed. No finding.

## P4 lint L2/L3 probes (my own, read-only, scratchpad r2/probe.mjs; loads lintWorkflow, plants in memory)
- Base passes. MISSED (lint returns []):
  a) a YAML anchor on the token check's env (`env: &x`) and an alias (`env: *x`) on the install step: the token reaches
     npm's step with no line naming secrets. GitHub Actions supports anchors and aliases (changelog 2025-09-18,
     https://github.blog/changelog/2025-09-18-actions-yaml-anchors-and-non-public-workflow-templates/; merge keys `<<`
     not supported per a third-party write-up, frenck.dev). L2's stated guarantee (secrets only in the three steps)
     is defeated silently. MEDIUM (test only; preview.yml is pinned by his NEW-TEXT sha and any edit is tier 2).
     Fix: L0 flags any anchor/alias (`&name`/`*name` as a value, `<<:`).
  b) L3: a folded (`run: >`) or plain multi-line run whose first line is the plain `npm ci --ignore-scripts ...` and
     whose next line is `--no-ignore-scripts` (or `false` after a trailing --ignore-scripts): YAML joins the lines, npm
     gets scripts on, the lint sees no `npm` on line 2. The `\` continuation is caught only inside `|`. LOW. Fix: L3
     accepts npm only in a `|` block line or a one-line run with no continuation line; flag any `>` header and any
     non-block line under a run key.
  c) Beyond L2/L3's stated claims (not misses of what they say): workflow `defaults.run.shell` or a step `shell:` with
     a custom command, workflow/job env NODE_OPTIONS or BASH_ENV, a job `container:`: each runs other code beside the
     token with no secrets line changed. LOW. Fix: lint pins `defaults.run.shell: bash`, no step `shell:`, no
     `container:`/`services:`, and an allow-list of env names.

## P6 process finding (B15; no guard-bypass detail beyond B15)
- guard_offline.mjs run by me (2026-11-26, repo root, read-only): cases A-D "ASK", E (`git merge --ff-only origin/main`)
  allowed. Matches B15.
- What the move brought: `git diff --stat 4fda80a4 e3b3642b -- .claude CLAUDE.md AGENTS.md .github tools/gate.mjs`:
  only .github/workflows/toolchain.yml (+18/-2), main's reviewed text. No .claude/, CLAUDE.md, AGENTS.md or gate file:
  the rules in force in this session did not change through it.
- For this change-set: nothing changed in what the PR carries. The protected files in the PR vs main
  (`git diff --stat origin/main...80a9b6a8 -- <protected>`): preview.yml (sha 31119c06 = his NEW-TEXT (a)),
  preview-close.yml (c2f1fa90 = approved), wall-test.yml (50 lines; planned to leave the branch, step (e)). The wall
  (PR review, his go on the exact SHA, branch protection) is unaffected.
- What it exposes: the "asks Mustafa first" seatbelt cannot be shown working in this session since 2026-11-15 (no hook
  activity in the record). Either repository hooks did not load in this session type, or an ask in auto mode was
  settled without a prompt. Every protected-file write by this session was therefore unseated; the content check by
  sha256 above is what covers PR #72.
- Severity: not a finding against PR #72's diff. For the kit: High while the cause is unknown; Critical if the hooks
  ran and an ask was settled without him (that is a way around the guard). Its own item for Mustafa, not a STATUS
  follow-up.
- Recommend in the report: state the fact plainly (B15 wording); list the PR's protected files with their sha256 vs his
  approved texts; a "Decisions needed" entry: find out whether hooks load / how an ask resolves in auto mode in this
  session type (a tier-2 kit change-set, plan first), and until known, day sessions that may touch protected files run
  in Manual or Accept-edits mode; wall-test.yml leaves the branch before ready.

## P7 verdict
APPROVE on ad2658a..80a9b6a8 (no Critical/High in the diff). Round-1 Lows: all closed. New: M (anchors/aliases defeat
L2), L (folded/plain multi-line run defeats L3), L (execution context beside the token not linted). STATUS follow-ups.
Process: kit item for Mustafa (High, Critical if hooks ran), outside this diff.

## P8 corrections after the advisor tool's read (2026-11-26)
- wall-test.yml sha256 3af06706...8507 = the sha his go names (.checkpoints/step-2-6-previews.md:47, DECISIONS :213).
  All three protected files in the PR equal his approved texts (preview.yml at 31119c06 per NEW-TEXT (a)).
- Anchor source read: docs.github.com "Reusing workflow configurations", section "YAML anchors and aliases": example
  `env: &env_vars` on job1 and `env: *env_vars` on job2 (job-level env; a step env is the same map shape: high
  confidence, not run on GitHub). Merge keys: not mentioned there.
- Literal plants (in memory on preview.yml):
  (a) token check: `        env: &cf` (was `env:`); install step, before `run: |`: `        env: *cf` -> lint [].
  (b) after the install step's `cd` line, a new step: `      - name: i2` / `        run: >` /
      `          npm ci --ignore-scripts --no-audit --no-fund` / `          --no-ignore-scripts` -> lint [].
      Plain form: `        run: npm ci --no-audit --no-fund --ignore-scripts` / `          false` -> lint [].
      Control: same folded block with `          && npm rebuild` on line 2 -> caught (L3).
- Ratings: (a) MEDIUM, new (not in TE-H's list 07/08/16/17/23/24). (b) MEDIUM, new: the lint's comment
  (preview.test.mjs:894-897) and B15 claim "no continuation"; that holds only for `\` inside a `|` block. Not High:
  the current text's `npm ci --ignore-scripts` is read directly (round 1 R3) and sha-pinned; the gap is future edits.
  The report must word L3 as checked. (c) LOW, extends TE-H (same class: execution context beside the token).
- Process item: one rating, HIGH for the kit, outside this diff; opens no third round; does not block #72 (protected
  files equal his approved texts by sha). First cause to check: CLAUDE.md "Models and effort", where repository
  hooks load (a cloud session with this one repository, outside a project thread).
- Not verified: te probes not re-run; npm flag behaviour taken from B15 (not re-measured); decide() not re-read this
  round (only run via guard_offline.mjs); environment settings and secret list unreadable (round 1: HTTP 403).

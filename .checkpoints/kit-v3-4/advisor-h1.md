# Advisor ruling: kit-v3-4 (A4), the verifier's H1 and its Lows, head 198c075 + notes 26255a9 (2026-11-09)

Role read from origin/main (`git show origin/main:.claude/agents/advisor.md`, `git show origin/main:CLAUDE.md`). Read:
`verifier-fixpass.md`, `session-h1.md`, `advisor-fix-pass.md` (§1 IE-2 and IE-3, §4), `kit-draft.md` §8 rows 66, 68,
76, 80 and 82, `infra-engineer.md` IE-2 (:60-68), the branch's advisor.md (:60-90, :207-298), CLAUDE.md (:98-115),
CHECKPOINTS.md §10 (:210-229), `routine-guard.mjs:32-43`.

## What I checked myself (git 2.43.0; throwaway repos under the session scratchpad's `advh1/`, outside the repo)
- **H1 holds.** A merge that turns `{"deny": ["Bash(rm -rf:*)"]}` into `{..., "Bash(curl:*)"]}` on one line; a fake
  "revert" that writes `["Bash(rm-rf:*)"]`: `--stable` ids equal (bf894c335054 inverse, fake, and an honest
  `git revert -m 1`); `--verbatim` tells them apart (inverse and honest 10af14bd4459, fake c102fd9dd748). The hole is
  a whitespace change on a line the inverse already removes or adds; a whitespace change on another line already
  differed under `--stable` too (a separate run, 07b005433d98 vs b29653d504ba).
- **`--verbatim` keeps what the check needs:** an honest revert's id equals the inverse's; file order does not change
  it (two files in either order: adb31919e3d6 both; `--unstable` differs), so it implies `--stable`'s ordering, and
  the verifier's "order-sensitive" aside is wrong (harmless, as the session found).
- **A mode change is caught under both flags** (a revert that also drops a script's executable bit, or flips one on a
  file the merge never touched: ids differ). So `--verbatim` closes the content hole and no other flag is needed.
- **New, mine (Low): empty equals empty.** `git patch-id` prints nothing when it fails (an unknown option, as on a git
  too old for `--verbatim`) or when its input is empty (a mistyped SHA): `[ "" = "" ]` is true. Two failed commands
  would "match". Four words close it (item 1).
- `protectedRel` (`routine-guard.mjs:35`) protects a last path part of `CLAUDE.md`, `.npmrc` or `.mcp.json` only:
  `AGENTS.md` is unprotected (L1 holds). No test or other kit file names `patch-id` (`git grep`: advisor.md:285-286
  only).
- Rows 80 and 82 against CHECKPOINTS.md:216-218 and :225-228: the cells quote earlier wording (L2 holds).

## 1. H1: fix it, `--verbatim` on both sides, plus "and neither is empty"
High stands (a rule looser than his "exact undo", CLAUDE.md:109). The defect was in my own fix-pass ruling's IE-2 text,
which took infra-engineer's "exact on content" (`infra-engineer.md:65`) without testing whitespace.

The new advisor.md:285-286 (indentation kept; 117 and 87 characters):
```
  - Its diff is the exact inverse of the merge: `git diff <merge-sha> <merge-sha>^1 | git patch-id --verbatim` equals
    `git diff origin/main...<headSHA> | git patch-id --verbatim`, and neither is empty.
```
Nothing else in the bullet changes. "And neither is empty" is my own Low (above), stricter, and rides this push.

Row 76, new cells:
- After: "the two diffs' `git patch-id --verbatim` are equal, and neither is empty"
- Verdict: "looser than the drafted text, same as his words"
- Why: "Equal content byte for byte, whitespace and file modes included; only hunk line numbers, `index` lines and file
  order are ignored, so a later STATUS merge's offsets do not count (the verifier's H1, the session's and the
  advisor's runs). `--stable` ignored whitespace, so `rm -rf` and `rm-rf` matched: looser than his 'the advisor checks
  it's an exact undo' (REVERT-EXCEPTION (a)). An empty id means a failed command, never a match."

The table's other note on IE-2: `infra-engineer.md:65` stays as written (a reviewer's notes, not the kit); this ruling
records the correction, and the report cites this file, not :65, for "exact".

## 2. No second round: my ruling plus the verifier's re-check of the new head
- H1 is the verifier's finding, so under my fix-pass ruling §4 "the reviewer whose finding it is" is the verifier,
  whose re-runs sit outside the budget (CLAUDE.md:105-106; advisor.md:246, :255). infra-engineer owns IE-2, but the
  defect is one flag whose behaviour three of us reproduced independently (the verifier, the session, me), and the
  check it guards is mechanical: a review round would add no evidence the verifier cannot re-run.
- Round 2 stays unspent (the budget allows it on what changed). It runs, infra-engineer on the Revert text only, if
  the verifier or I find a Critical or High in what this push changes.
- **The verifier re-checks the new head (not a round):** advisor.md:285-286 and the L1 bracket equal the texts here
  byte for byte, and no other advisor.md word changed (its word diff); its own `pid` scripts re-run against the new
  line (the whitespace fake fails, the honest revert and the STATUS-offset revert pass, an empty pair fails); rows 66,
  68, 76, 80 and 82 match this file; STATUS F5 present as in item 5; kit tests at main's counts (1034, 127, 27);
  `node tools/gate.mjs` +0 −0; repo-map check; no file outside advisor.md, STATUS.md and `.checkpoints/` changed;
  no added line over 120 characters. Then my review of the pushed head (not a round).
- Approval prompts for Mustafa in this push: one (advisor.md).

## 3. L1: fix now, it rides H1's push
New advisor.md:76-77 (lines 74-75 unchanged; 116 and 41 characters):
```
  Advisor's merge of those it can see; it does not yet see a renamed-away path, `CLAUDE.local.md` or `AGENTS.md`, so
  read the `--no-renames` list yourself).
```
Row 66's After becomes "... of those it can see; it does not yet see a renamed-away path, `CLAUDE.local.md` or
`AGENTS.md`, so read the `--no-renames` list yourself"; verdict stays stricter. STATUS F1 already names `AGENTS.md`.

## 4. L2: confirmed, the main session copies the landed words (notes only, no ask)
- Row 80, After: "For the netcode proof test's spike (`spikes/netcode-proof/`, on its branch until it merges): from its
  folder, `cargo clean` (its `.cargo/config.toml` names the folder); from the root, `cargo clean --manifest-path
  spikes/netcode-proof/Cargo.toml --target-dir .gate/netcode-proof-test/target`" (CHECKPOINTS.md:216-218). Verdict
  stays same. Why gains: "'on its branch until it merges' is the main session's, accepted: the spike is not on main".
- Row 82, After: adds "Not for two copies of the same workspace: Cargo gives their crates one slot, so a copy can run
  the other's build, and the folder's lock lets one build run at a time. The netcode spike keeps one per copy (its
  `.cargo/config.toml` puts it under the copy's `.gate/netcode-proof-test/target`)" (CHECKPOINTS.md:225-228). Verdict
  stays stricter.

## 5. L3: a STATUS follow-up F5 beside F4, named in the report; no change now
Strict side today: a kit merge that stops sessions working reaches the no-plan revert only through Mustafa's words,
or goes plan first. Adding "or the session's quoted refusal or error" would loosen the test: a session could certify
its own trigger and skip the plan. That needs his words, so it is not mine to add in a fix pass. F5 text for
STATUS, under F4:
```
    - **F5, the third "main breaks" case in the revert check** (next kit pass, with F4; the verifier's L3, Low):
      advisor.md's check of a revert PR accepts a red `gate` or `smoke` run or Mustafa's own words, so "a merged kit
      change stops sessions working" qualifies only through his report (the strict side). To decide then: accept it
      when the advisor reproduces the refusal or error on main itself (a loosening: needs his words), or drop the
      case from "Main breaks" and CLAUDE.md's Rollback line, leaving it to his report.
```

## 6. Row 68 (IE-3): stays looser, confirmed under E2; its Why gains one sentence
His words carry it: an exact undo (REVERT-EXCEPTION (a)) cannot also edit STATUS or ROADMAP, and "then fix in a new
PR" (A4 item 3) gives the record its home, which Revert step 4 names (row 69, stricter). "It merges by today's rules"
is about the merge path (classes, checks, his go), which stays today's; only the record moves. Row 68's Why gains:
"The merge path stays today's ('it merges by today's rules'); only the record moves to the fix PR, which an exact
undo cannot carry." No kit text changes. (Nothing stops a tier-0 docs PR recording the revert in STATUS while the
fix PR is pending; no rule is needed for that.)

## 7. The rest of the verifier's notes
- Its pass items 1-6 stand. Two corrections, both notes only: the "order-sensitive" aside is wrong (item 1's test);
  its quoted ids (50afd389efe7 "both") came from an earlier script (`session-h1.md` item 2), so the report quotes the
  reproduced ids above or the session's, not those.
- IE-8 and IE-9 were not re-run by the verifier or by me; the report says so (infra-engineer's runs, unchanged).

## Verdict: REVISE with the change
Apply item 1 (two lines, with "and neither is empty") and item 3 (two lines) in ONE advisor.md Write, items 4-6 in the
notes and STATUS (no prompt), push, then the verifier's re-check (item 2) and my review of the new head. No second
round unless that re-check finds a Critical or High. Nothing new for Mustafa beyond the one approval prompt; his
GUARD-RENAME answer and his go on the final head stay open as before.

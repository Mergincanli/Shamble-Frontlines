# Advisor ruling: kit-v3-4 (A4) fix pass, draft PR #44, head 94f1556 (2026-11-09)

Role read from origin/main (`git show origin/main:.claude/agents/advisor.md`, `CLAUDE.md`); the branch's budget rules
(CLAUDE.md:102-111, advisor.md "Review budget") read too, and applied: one round is done, this is its one fix pass.
Read: infra-engineer.md, verifier.md, security-auditor.md, advisor-plan-rev1.md (C1-C13), advisor-addendum-disk.md,
plan-rev1.md, Mustafa's answers of 2026-11-07 and 2026-11-08. Kit files reviewed at 94f1556 equal ad1f9bd's (the
three later commits add only `.checkpoints/` notes).

Checked myself: `.github/workflows/gate.yml:50` runs `node tools/gate.mjs "$BASE" --no-smoke` with no `--allow-drop`
(IE-1 holds). `routine-guard.mjs:1460` reads `--jq '.[].filename'` only, and `protectedRel` (:32-43) protects a last
path part of `CLAUDE.md`, `.npmrc` or `.mcp.json` only, so `CLAUDE.local.md` and `AGENTS.md` are not protected (SA-1
holds). `.gitignore:11` ignores `CLAUDE.local.md`. `security-auditor.md:54` points to advisor.md step 4 for "left
open", so SA-4's fix in advisor.md reaches it with no edit there. Not re-run: the reviewers' scratch repos.

## 1. The fix list

### Fix now, `.claude/agents/advisor.md` (ONE Write, one approval prompt)
Prepare the whole new text in the notes, read its diff against the current branch file, then one Write.
- **SA-2 (High).** (a) In "Risk tiers" (:54) and step 0 (:127): `git diff --name-only --no-renames
  origin/main...<headSHA>`. (b) Add to tier 0's "Not tier 0, whatever the file": "a file renamed or moved from a path
  outside tier 0, and any `CLAUDE.md`, `CLAUDE.local.md` or `AGENTS.md` anywhere (risk tier 2)". (c) Not optional:
  correct :73-74 so it no longer states a wall that does not hold, e.g. "(the guard refuses the PR Advisor's merge of
  those it can see; it does not yet see a renamed-away path or `CLAUDE.local.md`, so read the `--no-renames` list
  yourself)". Drop the bracket once the guard fix (follow-up F1) is on main.
- **IE-1 (High).** In "Revert", after the CI line: "If the revert drops game symbols (the reverted merge's '+ added'
  list), CI's `gate` fails by design (CI passes no `--allow-drop`). The day session runs the gate itself with
  `--allow-drop` naming exactly those symbols; the PR is needs-mustafa, and it goes to Mustafa under Decisions needed:
  a fix forward instead, or his own way past the required check. No session bypasses a required check." Plus
  follow-up F2.
- **IE-2 (Medium) and IE-2-SCOPE (mine, section 2).** The exact-undo check compares patch ids:
  `git diff <merge-sha> <merge-sha>^1 | git patch-id --stable` equals `git diff origin/main...<headSHA> | git patch-id
  --stable`. The later-merge test becomes: "no later merge on main touched the same files, `docs/STATUS.md` and
  `docs/ROADMAP.md` aside, when git applied them cleanly; if one did, or git needed a hand resolution, it is a normal
  change-set, plan first".
- **IE-3 (Medium).** "Every tier, check": "STATUS and ROADMAP updated (a pure revert aside: its fix PR records it)";
  and Revert step 4: "STATUS and ROADMAP are updated by the fix PR".
- **IE-4 (Medium).** In "Revert": "While main is broken and until its revert merges, merge no other PR (hold it and say
  why). After it merges, a PR whose last CI run started before the revert merged is brought up to date with main first
  (a new head: a new review). Check that CI ran to the end on the revert's main commit; re-run it once if it was
  cancelled."
- **IE-5 (Medium).** "If no session can work, Mustafa opens the revert from the merged PR's GitHub page (its Revert
  button); you check it as below, and it merges on his go."
- **IE-6 (Low, rides this Write).** Define "pinned": the red main commit's first parent is green (its CI, or the full
  gate run on it in a worktree); the re-run is that same commit's run, once (a day session, with an ask, or Mustafa).
- **IE-7 (Low, rides).** Step 1: `git fetch origin` first; `<n>` = the reverted PR's number; a cloud session limited
  to a `claude/…` branch uses that stand-in (CLAUDE.md, Standing permission); remove the copy after (CHECKPOINTS §10).
- **SA-3 (Medium).** Add to "Your check of a revert PR": `<merge-sha>` is a first-parent commit of main (in
  `git rev-list --first-parent origin/main`), and the trigger holds: the red `gate` or `smoke` run on main names that
  commit and its re-run is red too, or the report is Mustafa's own words; otherwise it is a normal change-set, plan
  first.
- **SA-4 (Medium).** In "Today's classes, mapped" (:92), step 4 (:167) and the budget's last bullet (:251-252): "a
  security finding not fixed in the PR (a STATUS follow-up included) counts as left open".
- **SA-6 (Low, rides).** In "Review budget": "A finding's severity is its reviewer's label; only you change it, in
  writing, with the reason."
- **SA-7 (Low, rides).** Revert step 2: "The main session runs `git revert`, never a subagent."

### Fix now, `.claude/CHECKPOINTS.md` §10 (ONE Write, one approval prompt)
His quote stays word for word; only the bullets under it change. The bullet I asked for in the addendum ("Git refuses
a copy that still has unsaved work") is wrong as written: git deletes ignored files and drops detached-HEAD commits
without a word (IE-8's scratch runs). It sits in text the night shift may follow, so it is fixed now.
- **IE-8 (Medium).** Replace that sentence with IE-8's text: git refuses changed or untracked files but deletes ignored
  files (`.gate/`, builds, logs) and drops commits on a detached HEAD; before removing: nothing to keep in
  `git -C <path> status --short --ignored`; its HEAD is on a pushed branch or inside main (`git merge-base
  --is-ancestor <HEAD> origin/main`); nested copies under it removed first; then `git worktree prune`. The night
  shift's removals make the same checks.
- **IE-9 (Medium).** Keep "To consider" (his word) and add IE-9's caution: not for two copies of the same workspace
  (Cargo gives their crates one slot, so a copy can run the other's build, and the lock runs one build at a time);
  today each copy keeps its own (`spikes/netcode-proof/.cargo/config.toml`).
- **IE-10 (Low, rides).** The command: from `spikes/netcode-proof/`, `cargo clean`; from the root, `cargo clean
  --manifest-path spikes/netcode-proof/Cargo.toml --target-dir .gate/netcode-proof-test/target`.

### Fix now, docs and notes (no prompt)
- **M1 (Medium), L2 (Low):** `.checkpoints/kit-v3-4/kit-draft.md` §8 gets the verifier's §10 rows and the tier-1
  "Check" row, then a row per sentence this fix pass adds or changes (each with its verdict; IE-2-SCOPE's row reads
  "looser than the drafted text, still stricter than his words"). The report carries the table.
- **L1 (Low), this PR's own text:** `docs/STATUS.md:528` README:183 → :184 and :539 README:103 → :104;
  `docs/reports/2026-11-07-vfx-xp-notes.md:88` :183 → :184, and :73, :78, :83 add "at 69ea88e".

### Not in this pass: STATUS follow-ups (named in the report)
- **F1 (risk tier 2, its own change-set; SA-1's fix, see section 3):** the merge check reads both names
  (`.filename, (.previous_filename // empty)`); refuse a pr-advisor merge when `changed_files` is 3000 or more (SA-8);
  `protectedRel` protects `CLAUDE.local.md` and `AGENTS.md` anywhere; test rows. SA-5 (Low: a mechanical refusal of
  pr-advisor merges of `docs/adr/**` and the decision docs) rides F1 if Mustafa wants it.
- **F2 (risk tier 2, its own change-set; IE-1's CI follow-up):** CI accepts a pure revert's drops, with SA-3's note:
  only for a first-parent merge of main named in "This reverts commit <sha>", and exactly that merge's added symbols.
- **F3 (docs-keeper; L1's second half):** main's own #45 line drift in STATUS (:116, :117, :465, :466, :484, :485,
  :492, :495, :545), lines this PR did not touch.
- **F4 (next kit pass; L3, Low):** CLAUDE.md's short table: row 0 "that adds or changes no word or decision" and
  "not CLAUDE.md, CLAUDE.local.md, AGENTS.md or a rename"; one clause "steps 4 and 5 apply at every tier; decision
  docs always need his go". Not now: advisor.md governs (CLAUDE.md:91 points there), every gap is on the strict side,
  and it would cost a third prompt.

### Declined
- IE-2's other option ("keep the later-merge rule and say it usually means plan first"): declined for the narrowing,
  which keeps the check where it matters (code and rules files).
- A security-auditor.md edit for SA-4: not needed (its :54 points to advisor.md step 4).
- CLAUDE.md and SKILL.md: not edited in this pass (no High lands there).

Prompts for Mustafa in this pass: two (advisor.md, CHECKPOINTS.md).

## 2. IE-2-SCOPE: mine, ruled
The later-merge test is not his: his words are "the advisor checks it's an exact undo, and it merges by today's rules".
The test is my own condition C8, a technical safety choice. Narrowing it still leaves the text stricter than his
words, so it loosens nothing beyond them (CLAUDE.md:109's High) and is mine to rule. Ruling: narrow it as in IE-2
above (STATUS and ROADMAP overlaps that git applied cleanly do not count; any other later merge on the same files, or
any hand resolution, means plan first). Why: every change-set edits STATUS and ROADMAP, so the drafted test fires on
every revert but the latest merge's, and the narrowing keeps it for code and rules files, where an untested mixed
state can break main. The E2 row and the report name it, so he sees it at his go.

## 3. SA-1: the question for Mustafa
**GUARD-RENAME:** The PR Advisor routine's last safety lock (the guard's merge check) can be walked around today, on
main, without A4: a pull request that only renames a rules file away (for example moves `.claude/settings.json` into
`docs/`), or adds a hidden instructions file (`CLAUDE.local.md`), looks like a docs change to the lock, so the routine
could merge it without you, and every later session would then run without its safety rules or with extra
instructions. A4 does not cause this and does not widen it (today's routine already merges docs PRs); A4's text fix
(SA-2) makes the routine read renames and treat those files as rules, which closes the honest-mistake path but not a
deliberate one. Why it matters: it is the one way found around the lock that protects your rules. Options:
- (a) fold the guard fix into A4: A4 grows a code change in the guard with its tests, needs another review round on
  that code, and waits longer;
- (b) A4 waits until a separate guard change-set has merged;
- (c) A4 merges with SA-2's text fix, and you accept the guard fix as the very next change-set (F1), ahead of other
  kit work.
**Recommendation: (c).** Holding A4 back does not shrink the risk (the gap is on main either way), the text fix closes
the misread path now, and a separate change-set gives the guard code its own plan, tests and security review instead
of riding a text PR's last pass.

## 4. The second round, and who checks the fix pass
- **No second round planned.** Both Highs (IE-1, SA-2) are text fixes given exactly above; SA-1 is outside the diff
  and goes to Mustafa (section 3), which no review round can settle.
- **Who checks:** the verifier, on the fix pass's head (not a round; never cut by the budget): E1 again (his words
  byte for byte, §10's quote included); every new or changed sentence mapped to an E2 row and matching this ruling;
  kit tests at main's counts (1034, 127, 27); `node tools/gate.mjs` +0 −0; repo-map check; no file outside this
  ruling's list changed. Then my review of the pushed head (not a round), which closes IE-1 and SA-2 against their
  reviewers' fix text.
- **A second round runs only if** the verifier or I find a Critical or High in the new text, or IE-1 or SA-2 not
  closed: then on what changed only, by the reviewer whose finding it is (infra-engineer for Revert and §10,
  security-auditor for the tier and security text).

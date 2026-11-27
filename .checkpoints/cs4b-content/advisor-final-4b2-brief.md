# advisor agent: change-set 4b-2 "The world" on its final head, for Mustafa's go

DRAFT until the verifier's PASS and docs-keeper's pass: the marks [after] are filled by the main session first.

Asked by Builder V6 (the main session), 2026-11-27. The worktree is `/home/user/Shamble-Frontlines/.gate/cs4b-2`
(local branch `cs/4b-content`, pushed as `claude/ecstatic-wozniak-5cbdzs`). **The final head: [after]** (the notes
out of it; the commit before it holds the notes, the report's sources, the pull request's text and the go-question
file). Main is b8f02b0, merged in at 35e96ad; main has not moved since (fetched [after]). Your agent file on main
last changed 2026-11-13 (8523d50), before this session started, and this branch does not change it.

**What it is.** Steps 6 to 11 of the re-read sheet's revision 3, section G (`reread-sheet.md`, which you approved on
2026-11-26 under seven conditions); step 12 moved to 4c by his STEP12-GO (a). The report:
`docs/reports/2026-11-27-cs4b2-world.md` [after]. The pull request's title and text: `pr-text-4b2.md` at the notes
commit [after] (this chat cannot open one: his item 11; his Advisor opens it as a draft). Risk tier 1 (game code,
tests, scripts, data, docs); `docs/DECISIONS.md` gains his six messages of the change-set by script, so his go is
needed for the merge in any case; no Brave check (nothing a player sees changes in a match that runs).

**Told to you, as the card records them:** the order of building departed from the sheet's (step 10's core and the
pieces of steps 9 and 10 that need no content file came before step 8's second part, while his DIFF-LIST was open;
step 9 in three cuts; cut C and step 10 committed before their full local checks ended, this environment's Stop hook
asking every change committed, each check then passing on the same files); every departure is in the report's
"Decisions and deviations".

**Reviews.** Round 1 on c442a80 (six reviewers) and its fix pass (70f11e1); round 2 on what changed (five
reviewers, every one PASS, Lows only) and its fix pass ([after: the code commit]). **No reviewer read round 2's fix
pass itself:** please read it (`git diff 70f11e130ee9dc0aabbb61a01bb9d3a6c013418e [after] -- . ":(exclude).checkpoints"`);
its one change to shipped code is `crates/content/src/map.rs`'s spawns check (now held to `MIN_SPAWNS` itself).

**Named questions for your ruling (APPROVE, REVISE with what, or ESCALATE to Mustafa with why):**
- **Q1.** Security-auditor left two risks for acceptance, both on the developer's own computer: (a) a browser that
  sends no `Sec-Fetch-Site` header is served by the dev server's `/content.json` as a tool would be (the content is
  public repository data; the server answers on 127.0.0.1 only; a request starts the content tool, which takes no
  input from the request); (b) a folder swapped for a link between the tools' check and their open is followed (a
  race on the developer's own disk, while a tool runs by hand). Accept both as recorded risks, or not.
- **Q2.** The final head as a whole: APPROVE for his go, or not.

**Evidence on the final code [after]:** the full local checks, the gate with smoke, CI (gate, smoke, toolchain), the
plants' whole run, the verifier's PASS, the official count (`advisor-count-rule.md`).

**The notes** are out of the final head (`.claude/CHECKPOINTS.md` §9); read them at the notes commit [after] with
`git show <that commit>:.checkpoints/cs4b-content.md` (the card) and the same for each notes file the report names.

**Rules for you.** Read-only: no source, doc or note edits; your verdict file goes to
`/home/user/Shamble-Frontlines/.gate/cs4b-2/.gate/advisor-final-4b2.md` (gitignored: the final head must not move
after your read); git read-only, each git command alone in its shell call; run the
gate yourself if you wish (`PLAYWRIGHT_BROWSERS_PATH=/home/user/Shamble-Frontlines/.gate/cs4b-2/.gate/pw-1194 node
tools/gate.mjs`); the repository is public. Your final answer: the verdict per question, then for Q2 the full head id
you approve, and what you did not check.

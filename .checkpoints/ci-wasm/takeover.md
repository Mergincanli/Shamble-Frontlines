# The takeover of the CI-WASM pull request by a project thread: an addendum to the plan (revision 3)

_Written by the project thread "Build the game from change-set 3" (session_01WLuqnwQ8c8JuNgfBX7Wixg), 2026-11-23,
14:40 UTC (its first commit, fcf6def, is stamped 14:40:57 UTC), for the advisor agent's read before the CI write.
It changes no step, no line of the CI text and no evidence of `plan.md` (revision 3, approved: `advisor-plan.md`,
"APPROVE holds for sha256 240c7db7..."). It says who carries the plan out now, and what that changes._

_**Overtaken in part on 2026-11-23, 14:54:47 UTC** (his tap on the card WRITE-WHERE: "In a new Claude Code chat";
`mustafa-2026-11-23T145447-write-where.md`). The thread makes no CI write. Section 3, items 4 and 5, and question 2
of section 5 no longer apply to this pull request: a new Claude Code chat takes it over, where the guard loads and
his WRITE-MODE words apply as written. Section 3, item 4's sentence that the thread "goes on unless he says
otherwise" rested on the thread's answer of 14:33:55 UTC, which was incomplete (a normal Claude Code chat also loads
the guard); the thread corrected it at 14:51:31 UTC and asked him again. Sections 2 and 4 stand. What the chat that
takes over needs is in the resume card's section "Handover"; the advisor agent's read of this addendum and of the
thread's records is `advisor-takeover.md`. The text below is left as it was written (but for the time in the
line above, which first read "14:41 UTC")._

## 1. Why
- The first builder chat (lane 1; he calls it Builder V1; session_019YNMUV9LFhASseJ9EP12kL, branch
  `claude/wonderful-hamilton-jdmlnf`) was paused by his words of 2026-11-19, 21:44:09 UTC, and its account is out of
  usage until 2026-11-24, 20:00 Amsterdam time (his message of 2026-11-23, 14:10:08 UTC in his project's main chat,
  read there; not copied into the notes).
- His words of 2026-11-23, 14:25:03 UTC (`mustafa-2026-11-23T142503-builder-chat.md`): "I would rather start today
  with the progression either way and would like to create a Fable 5.2 based Claude Code chat that will continue
  work on building the game". His project made this thread from that message.

## 2. What stays exactly as approved
- The CI text: `ci-edit.diff`, 93 added lines, sha256
  240c7db726dee600b54d10b0252b33e527482ecf88ba41af849660d55e8d2673; P-GH's two lines, `ci-plant-p-gh.diff`, sha256
  e4f21ab359bc9fd911c663a4fcbc892133195fefe44e88ad7ec51b50e3d0b1ac. Both files are unchanged since d064e17.
- The plan's steps 4 to 6, section 9 (the evidence, the stop rule on every run of the new lines, the two plants),
  the reviewers of section 12, the sizes and stop lines of section 14, the report's shape (section 7, item 5).
- The branch's code and docs at d064e17: the thread's branch starts at that commit.

## 3. What changes, each with its basis
1. **Who builds:** this thread, on his words of 14:25:03 UTC. The session's model was read from the session record
   at 14:28 UTC: configured and last served as his words of 2026-11-12 and 2026-11-23 ask of a builder chat.
2. **The branch and the pull request:** `claude/project-thread-qx9woh` (the harness limits the thread to it), made
   at d064e17 in the worktree `.gate/build-wt` and pushed; the main folder stays on main. A new draft pull request
   replaces #62. **#62 and the first builder chat's branch are not touched:** no push, no comment, no close; he is
   asked before #62 is closed (the project's note to the thread). LANE-RULES: "a lane never pushes to another
   lane's branch."
3. **His answers** (ANSWERS-7, his tap of 14:31:06 UTC on the thread's card, after the thread's message of 14:28:25
   UTC stated the seven back with the CI text attached; the attached file is byte-equal to `ci-edit.diff`):
   CHROME-SOURCE (a), CI-TEXT (a), FIREFOX-TESTS (c); and for change-set 3: RNG-SOURCE (c), SIZE-3 (b), PRESS-PACE
   (a), DEPART-3 (a). So the CI text needs no change (CHROME-SOURCE (a) is what it holds), and the Firefox pull
   request follows this one.
4. **The mode check before a CI write.** His WRITE-MODE words (2026-11-18): "From now on you read the chat's mode
   right before each kit write and stop unless it is Accept edits." This thread's session record reads "auto", the
   thread has no switch for it, and no repository hook loads in a project thread (CLAUDE.md, "Models and effort").
   So the thread stopped and asked him (the card WRITE-THREAD, 14:29:18 UTC). His answer, 14:32:43 UTC: "Yes, but why
   don't we make a new Claude Code chat running on Fable 5.2 creating those CI files? Instead trying to do it
   here?" The thread answered at 14:33:55 UTC (`thread-2026-11-23T143355-why-here.md`) and goes on unless he says
   otherwise. In place of the mode read: his yes on the exact text; the file's sha256 compared with its scratch
   copy after each of the three CI writes (the file; P-GH's two lines; their take-back); nothing else under
   `.github/`, `.claude/`, `CLAUDE.md` or the gate files is written. His WRITE-MODE words are not reworded: they
   stand for every chat that has the mode.
5. **Commands.** The first builder chat's container refused `npx`, `npm run`, `gh api` and others and used routes
   around them. Here no guard hook loads, and the harness offers GitHub only through `gh api` (REST). The kit's
   rules are kept by hand: no force push, no history rewrite, no `rm -rf` of a tree, cargo with `--locked`,
   `npm ci --ignore-scripts`, code committed by path, the main folder never switched.
6. **Records.** His messages reach a project thread inside a wake; `thread-words.mjs` copies them from the session's
   transcript by their send time and decodes the XML entities, nothing else. A tap on a decision card is recorded
   as a tap: the platform writes that message for him in the card's wording. The thread's own message and cards are
   copied the same way and shown as the thread's text, not his words. `decisions-3.mjs` gains four entries (plan
   section 7, item 1: "Every later message of his in this pull request ... is one more entry").
7. **Agents:** Opus 5.5; reviewers, the verifier and the advisor agent at effort max; at most 3 at a time; the saver
   while they run.
8. **Long runs** (LONG-RUNS, his tap of 14:32:42 UTC): a full suite or a long build is read from GitHub's checks on
   the pushed branch. The plan's evidence is unchanged: section 9 already reads the runner's runs; the short runs
   (unit tests by path, the legacy gate, the rehearsal below) still run here.

## 4. The rehearsal on this container (2026-11-23, 14:28 to 14:33 UTC)
As the plan's section 5 "Tried here", again, on a fresh container:
- The archive by the workflow's own curl line: "final address" on GitHub's release-asset host, "size: 11860397",
  sha256 OK; the member list is the folder and its six files; the program 8170784 bytes, sha256 OK.
- The proposed file rebuilt by applying `ci-edit.diff` to main's `toolchain.yml`: 193 lines, sha256
  d1dbcb533460936e9f4f3ca2e97ed28c6d586638e52c00fa034e875f7a4e975c; with the plant
  12b9024966abc4907b27c8c2b4c6cdccfa56ceea3d27d94c060b9ab2ca1f12ef. Both equal the card's values. It parses as
  YAML: one job, nine steps, `permissions: contents: read`, 15 minutes.
- `cargo test --workspace --locked`: 34 passed. The new in-cut lines as one script under the cut's bare variable
  list, with `/opt/pw-browsers/chromium` (Chrome for Testing 153.0.8010.12) for the runner's Chrome: exit 0; both
  builds; the Wasm 28,987 bytes; "Tests 7 passed"; the five type-check lines; "12 passed (1.1m)", one worker.
- Not run here, as before: GitHub's own check, `sudo -n install`, the network cut, the runner's Chrome 157.

## 5. Questions for the advisor agent
1. Does the APPROVE of revision 3 hold with section 3's changes, so that the CI write (plan step 4) may go ahead?
2. MERGE-SEAT: CLAUDE.md lets "the session Mustafa gives his go to" merge a risk-tier-2 pull request, "kit, CI,
   gate: a day session". This thread is a session he talks to directly, on a `claude/` branch, outside the guard.
   May it carry out the merge on his go (with the expected head, after your approval of that head and with `gate`,
   `smoke` and `toolchain` green), or does his go have to be given to another chat? The thread asks him before any
   merge either way.
3. Is anything else in the kit's rules changed by the thread's seat that needs his word before the CI write?

## 6. Not verified
- That the first builder chat stays stopped: nothing technical prevents it from pushing to #62 when its account is
  back. He is given a block to paste there.
- The thread cannot read the first builder chat's transcript: what that chat told him on 2026-11-19 is known only
  from its card, its plans and the text he pasted into this thread.

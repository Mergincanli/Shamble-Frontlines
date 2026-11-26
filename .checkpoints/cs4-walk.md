# Checkpoint: cs4-walk
- **Goal:** the plan of change-set 4, "Walk" (the build plan, `docs/reports/2026-11-17-build-plan-slice-1.md`,
  lines 250 and 334-358), with the recording as its first step by his ORDER-3C (c); then the advisor agent's review;
  then stop for his go on the plan. Nothing is built before that go · plan: `.checkpoints/cs4-walk/plan.md`
  (draft 1) · his words: `.checkpoints/cs4-walk/mustafa-2026-11-25-takeover.md`,
  `mustafa-2026-11-25-order-3c-answer.md`, `mustafa-2026-11-25-export-block.md`, and his Export
  `testlines-export.json` (all by script; the three messages go into `docs/DECISIONS.md` with the first pull request)
- **Branch:** `claude/cool-pasteur-oiqk1r` on GitHub (it stands in for `cs/4-walk`). Local: `cs/4-walk` in the
  worktree `.gate/lane1-takeover` (the folder keeps its first name); pushed with
  `git push origin cs/4-walk:claude/cool-pasteur-oiqk1r`, alone in its command; never a plain `git push`. The
  session's start folder is fast-forwarded to main 311c0484 (so the fixed guard loads) and is never switched ·
  **Base:** origin/main 311c0484f7a9697ae33e7a76a8cf0b507a2837c4 (taken in by merge commit 1802856)
- **Last update:** 2026-11-26 13:28 Amsterdam (from the clock)

## Steps done
1. The takeover and ORDER-3C: his answer **(c)** with four conditions (2a no proof thinner; 2b the recording built
   and green before any movement code; 2c every item named for 3c placed by name; 2d the fallback: the recording
   alone as a small change-set before Walk, told to him before anything moves).
2. Main 311c0484 taken in (merge commit 1802856); the start folder fast-forwarded; the fixed guard loads.
3. His Testlines Export saved whole by script: 15,567 bytes, sha256
   9ad1d9f914acc27c56cbfbe7c4e6c7e654ba24fa0e9ae75cb4e3b93103f06498 (`testlines-save.mjs`; the manifest says "not
   usable as it is": step rate 60, one warning). Compared by script with `docs/legacy-tuning.json`
   (`testlines-compare.mjs`, output `testlines-compare.txt`): 83 names, 60 equal, 23 different, 0 without a
   counterpart. No value is carried. STEP-60 asked of him in the chat (17:55 UTC): no answer yet.
4. Three designs, each ending DONE: `netcode-architect-cs4.md`, `engine-architect-cs4.md`, `rust-reviewer-cs4.md`.
   The main session's notes: `testlines-cs4.md`, `rules-cs4.md`.
5. The plan's draft 1 written and pushed (f789b24): Walk as four change-sets, 4a The recording, 4b Content and the
   world, 4c The first soldier, 4d Sights and the proofs that are left; then the link simulator's change-set.

6. game-designer's check of `rules-cs4.md` finished (`game-designer-cs4.md`, DONE; verdict REVISE on the draft:
   line and citation fixes, seven more differences D11 to D17, four questions). Folded into the plan: **draft 2**
   (sections 2, 6.1, 6.2, 10, 11c, 12, 15; MOVE-RULES rewritten in eight rows; ADS-DELAY became SPRINT-DELAY-4 with
   three options; NO-SOURCE new; the stamina bar in 4c's step 4; 4c now 6,665 / 12,000 / stop 15,000). The plan's
   section 19 lists what changed since draft 1.

7. The advisor agent's verdict on draft 1, re-checked by it against draft 2: **REVISE**, eleven conditions, two of
   them High (`advisor-cs4-plan.md`): proofs of 4b's and 4c's code sat in 4d against his condition (a); the early
   look came before the camera panel against his words "from its first playable build".
8. **Draft 3** answers the eleven; the plan's section 20 says where each is met. Proofs moved to the part that
   merges their code (content items 1 and 2 to 4b; the paced replay, the hostile commands and the far-corner map to
   4c; 4d makes the recording anew); the early look after the panel (4c's step 5); WALK-CUT with times by 3b's pace
   (7,014 lines from first commit to merge in about 16 hours); STEP-60 with what depends on the step rate;
   DIFF-LIST carries the list, printed by `testlines-difflist.mjs` (output `testlines-difflist.md`, and into the
   plan between two marker lines with `--into`); the Export, its manifest and the table go to `content/source/`,
   not `docs/`; GREY-COLOURS is a default he can veto.

9. The advisor agent's re-read of draft 3 (d9539e8): **APPROVE**, with eight fixes that need no further review
   from it (one Medium: 4d's ladder moved the SIMD reading out of Walk; seven Low). All eight are made: the plan is
   revision 4, and its section 20 lists them. 3b's pace is 14 hours 56 minutes for 7,014 lines (not 16 hours).

10. His go on the plan (2026-11-25, 19:36:25 UTC): WALK-CUT (a), four change-sets, each its own pull request;
    STEP-60 (a), a new Export at 30 steps before 4b's step 6.
11. **4a, the recording: built, reviewed and merged** (pull request #70; main 2483e9a, by merge commit, on his go of
    2026-11-26, 12:21:17 UTC, for the head 420b16b the advisor agent approved). Its report:
    `docs/reports/2026-11-25-cs4a-recording.md`; its notes are readable at c17a054.
12. His answers of 2026-11-26 (`mustafa-2026-11-26-go-merge-70.md`): BOARD-SLIP (a); CONTENT-SOURCE (c), the reader
    kept to what the content files need, everything else refused, fuzzed before the first value rests on it;
    DEPART-4 (a) with two notes (the 48 one named constant; the loader refuses a map beyond the aim codec's reach
    and STATUS names the change-set that widens it); EARLY-LOOK-ORDER (b); FIREFOX-REPLAY (a), from him to the CI
    Builder; LINKS-CHECK (c), about 15 lines in 4b.

## Current step
4b, content and the world: its own card `.checkpoints/cs4b-content.md`. These notes were put back from c17a054 byte
for byte in this branch's first commit.

## Next steps
1. 4b (the plan's section 5), then 4c (section 6; the early look inside it, with EARLY-LOOK-ORDER (b): the soft
   push and five small page pieces after the early look), then 4d (section 7), then the link simulator's
   change-set, merged before change-set 7 starts.
2. Still his, each before its part: DIFF-LIST with his new Export (before 4b's step 6); MOVE-RULES, RESUME-FLASH,
   TUNE-HOME, NO-SOURCE (before 4c; MOVE-RULES' rows 6 and 7 before 4d); SPRINT-DELAY-4 (before 4d); GREY-COLOURS
   is a default he can veto before 4b's step 9.
3. His messages of 2026-11-26 go into `docs/DECISIONS.md` by script with 4b's docs pass.

## Open findings
- The checkpoint script cannot push from this worktree: notes are committed by path and pushed by hand; no saver
  runs · this chat · open (the plan's section 3, item 6 says so).
- The first run of the comparison script counted an Export value of 0 as equal to a small old value (it rounded to
  the decimals the number printed with): caught against the Export's own "changed from start" list, fixed before
  the result was used; the kept tool gets a planted case for it (plan section 11d) · this chat · fixed in 2c3078e.
- The session's worker restarted once (about 18:00 Amsterdam); nothing was lost.

## Key numbers
- Walk bottom-up from the designs (draft 3): about 15,300 lines (4a 1,365; 4b 5,410; 4c 7,315; 4d 1,210), half
  tests; planned at 1.8 times: 2,450 / 9,750 / 13,150 / 2,200 = 27,550; stop lines (planned times 1.25): 3,050 /
  12,200 / 16,450 / 2,750. The card: 3,900. The recording alone: 1,365 (change-set 3's plan: 275).
- The pace used for times: change-set 3b, 7,014 lines (docs, notes and lockfiles left out) from its first commit
  (2026-11-24 23:00:50 UTC) to its merge (2026-11-25 13:57:02 UTC), 14 hours 56 minutes with reviews and his
  check: the early look about 49 hours of work away, the judged soldier about 54.
- The collision library: parry3d =0.31.1; 33 new lockfile packages, 31 in the players' file's tree (rust-reviewer's
  paper reading, to be confirmed by `cargo tree` after the hold).
- The time unit: 3,000 units a second (rust-reviewer, block 4).

## Decisions taken
- ORDER-3C (c) · Mustafa · 2026-11-25, 16:59:52 UTC.
- His rules for this chat that are not on main yet: no CI, kit or gate file is written here; never a push to
  another chat's branch; until a second guard fix is merged, git and gh always start their own command, with no
  other program in front of them (this session and every agent).
- The board add-on is active for this chat (since 2026-11-25).

## Running work
- No agent runs. The advisor agent's notes: `.checkpoints/cs4-walk/advisor-cs4-plan.md` (ends DONE).
- No saver runs: the main session commits the notes by hand at each update.

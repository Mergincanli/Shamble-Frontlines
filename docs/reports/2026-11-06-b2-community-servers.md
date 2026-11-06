# Advisor report: b2-community-servers (2026-11-06, branch `claude/sleepy-tesla-ofsxsr` for cs/b2-community-servers, base main ee736ec)
## Goal
B.2 on Mustafa's list: his "Community servers (later)" note (2026-11-03) in `docs/VISION_V2.md`, word for word; and
B3's hand-over: three VISION_V2 lines brought in line with his design walk of 2026-11-04. Docs only; also the docs
after PRs #39 and #41.
## Outcome
DONE: the note is in VISION_V2 §5, pillar 5 holds his decided Factions line, and two pointers carry his 2026-11-04
words; the review round found nothing Critical or High, and its one fix pass is in.
## Plan and approval
- Plan revision 1; the advisor's **APPROVE WITH CONDITIONS (9)**, all met: C1 the note's last line names its open
  question (ADR 0006 OQ-0006-21), not what the drafts "keep"; C2 the trust rule answers nothing else (game-designer
  checked); C3 E1 checks his Factions sentence only, the parenthetical is the one deviation (below); C4 this report's
  For Mustafa; C5 #41's merge in STATUS:3, and #40's two stale lines named here; C6 README:294-298; C7 B3's ROADMAP
  entry "taken by B.2 (PR #42)"; C8 one register clause, appended; C9 stale citations reported, not edited.
- Rulings: PLACE (a) §5; TAG (a); HANDOVER (b); REG (a); DK-41 (b), narrowed (the plan's premise was false); HEADER (a).
## What changed (vs main ee736ec; symbols +0 −0, as planned: docs only)
- `docs/VISION_V2.md` (+18 −3): a dated header line; pillar 5's `[OPEN]` replaced by his `[DECIDED]` Factions line;
  pointers on §5's account line (LOOP_V2 §8, B3-Q2 (a)) and §7's faction question; the note at the end of §5.
- `docs/adr/README.md:160`: OQ-0006-21's row gains "His note in VISION_V2 §5 (2026-11-03, the same day) gives why,
  the trust rule and the precedents." No ADR draft edited.
- STATUS and ROADMAP: #39 and #41 merged; B.2 in flight; B3's hand-over taken; B.1's register citations moved to
  main's lines; B.2's queued words removed (they stand in VISION_V2); his VFX editor item of 2026-11-06 queued word
  for word; A4's kit freeze in the past tense; follow-ups "From B.2". The B.1 report: a correction after the merge.
## Evidence
- E1 (`e-checks.py`; wrapped text joined): PASS. His four B.2 pieces once each, matching 4f268da; his two 2026-11-04
  sentences match his walk (c5d86cb) and LOOP_V2; A4's entries unchanged; the VFX quote equals his message; the
  appendices equal their sources. E2: five change blocks, headings unchanged. E3: `[DECIDED]` 10 to 12, `[OPEN]` 4
  to 3, `[LATER]` 4 to 5, the rest unchanged, as written down before the build.
- E4: every `VISION_V2 §N` citation resolves. Line citations B.2 moves (ADR 0005:27, :48-50, :2550) and R1's stale
  :3301 and :3535 are STATUS follow-ups, not edited. E5: #39 merged 17:09:30 UTC, #41 13:35:37 UTC (GitHub API, the
  verifier); every changed line number checked. E6: `gate.mjs --no-smoke` PASS, symbols +0 −0 (three runs); repo map
  OK; CI gate and smoke green on 67978c2 and 26a5455. E7: only the five docs, this report and the notes.
- **Not verified:** smoke locally (CI runs it); his 2026-11-03 and 2026-11-04 words against the original chats
  (checked against the copies committed at 4f268da and c5d86cb); his UNDER-WAY (a), read from the netcode session's
  records (its plan §0.4, "his words in short", 411824a), not his own message.
## Reviews
- The verifier, PASS WITH NOTES: V1 (Medium, ADR 0005's line ranges move: a follow-up; the widened E4 found a fifth);
  V2-V6 (Low: this report, A4's tense, the order marked as the session's reading, the 2026-11-03 list as pointers,
  B3's line numbers dated): all fixed.
- game-designer, PASS WITH NOTES: GD-1 (Low, rented official private servers might read as counting: a follow-up and
  For Mustafa); GD-2 (Low, "the war around the matches" missing from VISION_V2 §7: a follow-up).
- infra-engineer, PASS WITH NOTES: IE-1 (Low, the trust rule adds no field now; part 12 stands: in the follow-up);
  IE-2 is GD-1. Nothing Critical or High: no skeptics, no second round (the review budget).
## Decisions and deviations
- His words: unchanged. The note's lead-ins ("Why:", "The trust rule:", "Precedents:") follow his outline, and its
  last line is the session's pointer. Pillar 5's bracket differs from LOOP_V2:75's (C3). The replaced sub-bullet is in
  Appendix B.
- His VFX editor item (2026-11-06, 23:27 UTC) arrived during docs-keeper's pass: queued word for word in STATUS, not
  built here (B.2's scope stays as approved); VFX-ORDER below.
- #40's two stale lines about #41 (its STATUS:31 and :173, "waiting for his go") are the netcode session's to fix.
- docs-keeper moved ROADMAP's R7 entry into Build history (its 8-12 recent entries rule): kept.
## Risks
1. Pillar 5 is decided at the level of principle only; which units are shared stays open (VISION_V2 §7, the register's
   OQ-0005-CD5, his unit list). He may prefer pointers only (option (a)).
2. Rented official private servers and the trust rule: not decided, and a reader might assume they count.
3. PR #40 also edits STATUS and already conflicts with main; B.2's STATUS lines add to that. Both say the same about
   #41 and the freeze, so whichever merges second resolves it.
## Proposed next step
The VFX editor note in EDITOR_V2 (his item of 2026-11-06), its own docs change-set, plan first (VFX-ORDER (a)).
## Decisions needed
- **VFX-ORDER:** what comes right after B.2? Why: it sets the next change-set. Options: (a) the VFX editor note as
  its own small docs change-set, then A4; (b) fold it into B.2 (a plan revision and the advisor's re-check, so B.2
  waits); (c) A4 first, then the VFX note. Recommendation: (a), since he wrote "For the next docs pass".
## For Mustafa
1. Your go to merge PR #42 at its final head, once the advisor has approved that head. Confirm: (a) the note's words
   and marks (the note `[LATER]`, the trust rule `[DECIDED]`); (b) pillar 5: your Factions line in place of the open
   question, as built, or pointers only instead; (c) the pointers read your 2026-11-04 words as you meant. The note
   does not decide whether rented official private servers count. No Brave check.
2. VFX-ORDER, only if you want (b) or (c).

## Appendix A: his words, whole (his task list of 2026-11-03, copied from A3's queue file at 4f268da)
2026-11-03, his task list (session_0173vVo5sT21uP4DcQtKvmAU, 2026-11-03T22:08:25Z); B.2 is its last item:
```text
Small change-sets, one at a time, plan first (Mustafa, 2026-11-03).
A. Kit (rule changes: each needs my go).
 1. Update PR #30 (kit-v3-1) onto main, resolve its conflicts, have it re-reviewed, then wait for my go.
 2. kit-v3-2, into .claude/CHECKPOINTS.md for every session: at most 3 agents at a time; every agent writes its plan into its notes before writing, then saves as it goes (each finished block to disk at once, a notes line after each finding, a restart continues after the last complete block); scratch files live in the notes folder, text and small scripts only, never build output or binaries (builds go to a temp folder); the background saver is checked and restarted at every checkpoint update. Also TOY-BOX: reword CLAUDE.md's "Red Alert 3 toy-box look" and the art-director's matching lines to point at LOOK_V2.
B. Docs:
 1. EDITOR_V2: add "Plan view: draw to build". Two views of the same map document side by side: a top-down plan where you draw outlines on a grid (lines, rectangles, hexagons, octagons, free polygons), and the 3D scene, which builds itself live from the drawing. A drawn outline is a footprint with a height and a kind (wall, building, floor, cover, ramp, zone); the kind decides what it becomes, e.g. a building footprint becomes a building-kit house with floors and a roof. Anything can be moved, rescaled and reshaped in either view; both stay in sync because both read the same document. Works with three-team mirroring (draw in one third; the others follow at 120°) and with floors. Precedents: Hammer's 2D views, Doom-style sector editors, SketchUp's push-pull. Built with Editor v2 (slice 5); no code now.
 2. VISION_V2: a short "Community servers (later)" note: why (bigger or longer matches, community growth, a safety net if the game grows faster than our funds, possible revenue such as renting out official private servers), the trust rule (progression counts on official servers only), and precedents (Counter-Strike and Rust community servers, Battlefield's rented servers, Minecraft Realms).
```

## Appendix B: the replaced pillar-5 sub-bullet (VISION_V2:36-37 at ee736ec), word for word
```text
   - `[OPEN]` How much is faction-specific? LOOP_V2 says "infantry shared, tanks per faction". Faction pride pulls
     toward more unique units. Needs a design session.
```

## Appendix C: the steps
Plan revision 1 (8ee1b5b); the advisor's APPROVE (e0b0687); the build 67978c2; E1-E4 and the gate (f979f80); his VFX
item saved (a79fef1); docs-keeper's pass and the VFX queue entry (26a5455); the review round and the fix pass
(8d1540f); this report. The notes (`.checkpoints/b2-community-servers/`) are deleted in the last commit, as
CHECKPOINTS.md §9 says; they stay in the branch history, last at this report's commit.

# B.2 plan, revision 1 (b2-community-servers, 2026-11-06): VISION_V2's community-servers note and B3's hand-over

Branch `claude/sleepy-tesla-ofsxsr` stands in for `cs/b2-community-servers`. The worktree is from main ee736ec (PR #39
merged). Docs only: no code, so symbols +0 −0. Not player-facing. Not kit, so the kit freeze does not apply.

## 1. Goal
Bring `docs/VISION_V2.md` up to date with Mustafa's words, as his queue asks:
- **B.2**, his item of 2026-11-03: a short "Community servers (later)" note (why, the trust rule, precedents).
- **B3's hand-over** (B3-VISION, approved in B3's plan; STATUS.md:34-36, ROADMAP.md:73). VISION_V2 lines 36-37, 59
  and 86 still describe LOOP_V2 as it was before his design walk of 2026-11-04.
- Also: the docs after PR #39's merge.

## 2. The sources (word for word in `mustafa-words.md`)
- **B.2** (2026-11-03): his task list, session_0173vVo5sT21uP4DcQtKvmAU, 22:08:25Z. It is in STATUS's "Queued, in
  Mustafa's words" (STATUS.md:498-506).
- **COMMUNITY-SERVERS** (2026-11-03, about 17:31 Amsterdam, the same day). It is already in the drafts: ADR 0006
  OQ-0006-21 (lines 1135-1149) and the register (`docs/adr/README.md:160`).
  - It holds the same trust rule ("so progression can count on official servers only").
  - It says "keep the door open, don't design it".
  - It is not edited; the note points at it.
- **His design walk** (2026-11-04), already in LOOP_V2:
  - §5, lines 74-75 (Factions) and 82-84 (B3-Q1 (c));
  - §8, lines 136-138 (B3-Q2 (a)) and 146 (the catalogue, `[LATER]`).

## 3. The edits to `docs/VISION_V2.md` (88 lines today)
1. **The note**, at the end of §5 "Accounts and business", after line 60 (PLACE and TAG, §7):
   ```
   - `[LATER]` **Community servers (later).** Mustafa, 2026-11-03:
     - Why: bigger or longer matches, community growth, a safety net if the game grows faster than our funds,
       possible revenue such as renting out official private servers.
     - `[DECIDED]` The trust rule: progression counts on official servers only.
     - Precedents: Counter-Strike and Rust community servers, Battlefield's rented servers, Minecraft Realms.
     - What the drafts keep possible for it: ADR 0006, OQ-0006-21 (COMMUNITY-SERVERS, his words of 2026-11-03:
       "keep the door open, don't design it").
   ```
   - His four pieces stay whole: the title, the why list, the trust rule, the precedents.
   - The only words added: the lead-ins ("Why:", "The trust rule:", "Precedents:"), taken from his own outline, and
     the pointer line.
2. **B3's hand-over** (HANDOVER, §7).
   - **Pillar 5's `[OPEN]` (lines 36-37).** Its question now has his answer, and its quote of LOOP_V2 ("infantry
     shared, tanks per faction") is no longer true. Recommended (b): replace the sub-bullet with his `[DECIDED]` line,
     copied byte for byte from LOOP_V2:74-75, and a pointer to the unit-list rule:
     ```
        - `[DECIDED]` Factions: the counter triangle and tier one stay shared; the differences grow with tech (tier-two
          specialists and vehicles are unique). (Mustafa, 2026-11-04, LOOP_V2 §5. Shared or per faction: decided unit
          by unit in the unit list, with two fixed rules, B3-Q1 (c).)
     ```
     The old sub-bullet goes word for word into the report's appendix. VISION_V2 has no "supersedes" section; B3
     recorded LOOP_V2's replaced lines in its §12.
   - **§5's account line (59)** keeps its words and gains a pointer: `(What account progression gives, since
     2026-11-04: "Account progression gives class variants and cosmetics only. New classes come through the team's
     tech in the match, so nobody is ever missing a counter." Mustafa, LOOP_V2 §8, B3-Q2 (a). The unlock catalogue
     itself is still for later: LOOP_V2 §8.)`
   - **§7's faction question (86)** keeps its words, since the register's OQ-0005-CD5 row names it as its owner. It
     gains a pointer: `(Mustafa, 2026-11-04: pillar 5 above. Which units are shared or per faction waits on his unit
     list.)`
   - Option (a), not recommended: pointers only on all three lines, with the tags unchanged. The pillar-5 pointer
     would also say that LOOP_V2 no longer says "infantry shared, tanks per faction".
3. **A dated header line** under the intro, as LOOP_V2:5 and EDITOR_V2:6 have (HEADER, §7): "Updated on 2026-11-06
   with Mustafa's words: his community-servers note of 2026-11-03 (§5) and his design walk of 2026-11-04 (pillar 5,
   and the pointers in §5 and §7)."
4. **Not touched, and why:**
   - §1's table is his words. "Finally unlocked that weapon" meets "class variants and cosmetics only", and the
     register's WEAPON-UNLOCKS row (OQ-0005-AB5) already holds that tension.
   - Pillar 7 is still true.
   - §6 is the drafts' only source of needs. The note adds no need, and OQ-0006-21 says "don't design it".
   - The header's tag line: no new tag.

## 4. The other docs
- **The register** (REG, §7): one clause in OQ-0006-21's row (`docs/adr/README.md:160`): "His note in VISION_V2 §5
  (2026-11-03, the same day) gives why, the trust rule and the precedents." No ADR file is edited.
- **STATUS, PR #39 merged** (ee736ec, 2026-11-06 17:09:30 UTC, `merged_at`, by merge commit, on Mustafa's go ("merge
  #39") after the advisor's APPROVE of 4f5976b; CI green on main). Read from the GitHub API at build time:
  - the header (:3) and Phase (:29-36): B.1 merged, B.2 in flight, the queue;
  - B.1's row (:99) to merged, and B.2's new row;
  - B3's row (:102): its hand-over gains "taken by B.2 (PR #N)";
  - "Queued, in Mustafa's words": B.2's entry and his two lead lines (:498-506) go once VISION_V2 holds them. A
    one-line pointer stays, like B.1's (:507-508). His whole list stays readable in the B.2 report's appendix.
- **B.1's line numbers in the register are one too low on main.** #41 merged before #39 and added a line above them:
  - STATUS:99 and :447: `README.md:118` becomes :119;
  - :446: :292-297 becomes :293-298;
  - :465: :201 becomes :202;
  - :466: :191-192 becomes :192-193;
  - the B.1 report's :25 gets a dated correction after the merge, as B.1 did for the A3 report.
- **ROADMAP:** B.1's entry (:93) to merged, B.2's entry, and the "v2, next" queue text (:73).
- **PR #41 itself is left to the netcode session** (DK-41, §7). Its open draft PR #40 already records #41 in STATUS.
- **A4:** unchanged. When the netcode proof test counts as under way is #40's question UNDER-WAY, which that session
  put to Mustafa. B.2 records nothing on it.
- **REPO_MAP:** unchanged (VISION_V2's row stays true).

## 5. Evidence
Read-only scripts in the notes folder do the checks.
- **E1, his words byte for byte:**
  - each of his four B.2 pieces is found exactly once in VISION_V2;
  - each LOOP_V2 quote in the hand-over matches LOOP_V2 byte for byte;
  - in STATUS, B.2's entry is gone and its pointer is there; A4's words are untouched.
- **E2, nothing else changed in VISION_V2.** A line diff names each changed line: the header line, the note, pillar
  5's sub-bullet and the two pointers. Every other line is unchanged, and so is every heading (§1-§7, in order).
- **E3, tag counts** (`[DECIDED]`, `[DEFAULT]`, `[OPEN]`, `[LATER]`, `[IDEA]`), before and after. The expected change is
  written down before the build. With TAG (a) and HANDOVER (b): `[DECIDED]` +2, `[OPEN]` −1, `[LATER]` +1; the others
  unchanged.
- **E4, links and citations:**
  - every `VISION_V2 §N` citation in docs/ still resolves (no renumbering);
  - the register's OQ-0005-CD5 owner cell still names a line in VISION_V2 §7;
  - OQ-0006-21 is where the note says.
- **E5, STATUS and ROADMAP are true.** #39's merge facts come from the GitHub API. Every changed line number is checked
  against main's files.
- **E6:** `node tools/gate.mjs --no-smoke` (docs only; CI runs smoke) and `node .claude/scripts/repo-map-check.mjs`.
- **E7, scope:** `git diff --name-only main...HEAD` lists only:
  - VISION_V2, `docs/adr/README.md` (one clause), STATUS, ROADMAP;
  - the B.1 report and the B.2 report;
  - the notes, while in progress.
  - No ADR draft and no code.

## 6. Build and reviews
- **The build:** VISION_V2 and the register clause in the main session; then E1 to E4 and the gate.
- **docs-keeper:** STATUS, ROADMAP and the B.1 report's correction; then E5 and E6.
- **One review round on the final diff**, at most 3 agents at a time, Opus 5.5 at high:
  - **the verifier:** E1 to E7;
  - **game-designer:** the hand-over against LOOP_V2 §5 and §8. It must add nothing his words don't hold and decide
    nothing he left to the unit list. It also checks the trust rule against LOOP_V2 §8's Service points;
  - **infra-engineer:** the note against ADR 0006's OQ-0006-21 and part 12. It must not contradict them, design
    anything, or add an engine need.
- **Review budget:** one fix pass for the in-scope findings. Skeptics only for a finding at High or above. Anything
  else becomes a STATUS follow-up.
- Notes in `.checkpoints/b2-community-servers/`, committed as they go, never only in /tmp.

## 7. Decisions for the advisor
- **PLACE, where the note goes.**
  - (a) A `[LATER]` bullet at the end of §5, "Accounts and business". Recommended: progression and revenue live there.
  - (b) A new §8, "Community servers (later)", after §7. §1-§7 keep their numbers. A whole section for a short note.
  - (c) §6 item 11, `[LATER]`. Not recommended: §6 is what the engine must serve, and his COMMUNITY-SERVERS says
    "don't design it".
- **TAG, how the note is marked.**
  - (a) `[LATER]` on the note (his "(later)"), and `[DECIDED]` on the trust rule, his rule in both messages of
    2026-11-03. The why and the precedents carry no tag: they are reasons and examples. Recommended.
  - (b) `[DECIDED]` `[LATER]` on the whole note, as line 59 marks cosmetics. Not recommended: whether community
    servers come at all is still open ("players may one day"; OQ-0006-21, "Could the game one day…"). That tag would
    decide it by label.
  - (c) `[LATER]` only, with no tag on the trust rule.
- **HANDOVER, pillar 5's `[OPEN]`.**
  - (b) Replace it with his `[DECIDED]` Factions line; pointers on lines 59 and 86. Recommended:
    - the header's `[OPEN]` means "needs a decision", and this one has his;
    - its LOOP_V2 quote is false today;
    - B3 replaced LOOP_V2's own lines the same way.
  - (a) Pointers only on all three lines, with the tags unchanged.
- **REG, the register.**
  - (a) One clause in OQ-0006-21's row pointing at the note. Recommended: whoever decides OQ-0006-21 finds his
    reasons.
  - (b) No register edit: the note points at the row, and one direction is enough.
- **DK-41, PR #41's merge in STATUS.**
  - (a) Leave it to the netcode session. Recommended: its #40 already records #41, and a second entry would add to
    #40's conflicts.
  - (b) Record it here too.
- **HEADER, the dated update line.**
  - (a) Add it, as LOOP_V2 and EDITOR_V2 have. Recommended.
  - (b) None.

## 8. Mustafa's check
His go to merge, as for B.1 and B3. He confirms three things:
- his note's words and marks;
- the replaced pillar-5 line;
- that the hand-over reads his 2026-11-04 words as he meant them.

No Brave check.

## 9. Risks
1. **Deciding by placement or label** (the tags, pillar 5). Mitigations:
   - the choices in §7 go to the advisor;
   - his words stay byte for byte (E1);
   - the tag changes are written down before the build (E3);
   - he gives his go.
2. **The note read as an engine need.** It goes in §5, not §6, and points at OQ-0006-21 for what the drafts keep
   possible. No ADR is edited (E7).
3. **PR #40 also edits STATUS.** It is the netcode proof test's plan: a draft, from another session, and already in
   conflict with main. B.2 edits only its own lines in STATUS:
   - B.1's, B.2's and B3's rows;
   - the queue lines and B.2's queued words;
   - the five line numbers.
   The report names the overlap.
4. **Line numbers moving again** if another PR merges first. Every edit is anchored on its old text. E5 rechecks the
   numbers on the final head against main.

## 10. Steps
1. The advisor reviews this plan: APPROVE, REVISE or ESCALATE.
2. The build and E1 to E4 (the main session).
3. docs-keeper's pass; then E5 to E7 and the gate.
4. The review round (the verifier, game-designer and infra-engineer, in parallel), then the one fix pass.
5. The report, `docs/reports/2026-11-06-b2-community-servers.md`. Its appendix holds his words and the replaced pillar-5
   sub-bullet word for word.
6. The notes deleted in the last commit (not [skip ci]), then the PR made ready.
7. The advisor's final-head review, then `needs-mustafa` and his go.

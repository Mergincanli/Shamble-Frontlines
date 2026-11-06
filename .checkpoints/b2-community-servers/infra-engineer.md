# infra-engineer review of B.2 (b2-community-servers), 2026-11-06

Worktree head 26a5455 (draft PR #42), base ee736ec. Role read from `git show origin/main:.claude/agents/infra-engineer.md`
and `git show origin/main:CLAUDE.md`. Docs only; no web research.

## Plan
1. Read the diff `ee736ec..26a5455` for docs/VISION_V2.md, docs/adr/README.md, docs/STATUS.md.
2. Read ADR 0006 part 12 (:495-521) and OQ-0006-21 (:1135-1148); the register's "Needs outside VISION_V2 §6"
   (README:294-298) and decision map J3 (README:408); plan-rev1, advisor-plan-rev1 (C1, C2, C8), mustafa-words.md.
3. Judge: (1) contradiction with OQ-0006-21 / part 12; (2) C1 pointer wording; (3) engine need or design added;
   (4) C8 register clause and STATUS follow-up. Plus my standing review points: cost, exposure, provider risk, rollback.

## Checks done (evidence)
- **The note** (VISION_V2:67-73, end of §5): why, precedents and trust rule match his B.2 item of 2026-11-03
  (mustafa-words.md §1) word for word in substance: "bigger or longer matches, community growth, a safety net if the
  game grows faster than our funds, possible revenue such as renting out official private servers", "progression
  counts on official servers only", "Counter-Strike and Rust community servers, Battlefield's rented servers,
  Minecraft Realms".
- **(1) No contradiction with OQ-0006-21.** "renting out official private servers" (his B.2 item, our side of the
  deal) and "official private servers they rent" (his COMMUNITY-SERVERS words, ADR 0006:1137-1141, the players' side)
  describe the same thing. The trust rule is the purpose clause of his own OQ words ("so progression can count on
  official servers only", ADR 0006:1141), so `[DECIDED]` on it does not pre-empt the open question, which is whether
  and how community servers come (ADR 0006:1135-1136, "Options: none weighed").
- **(1) No contradiction with part 12.** ADR 0006:514-515: "No field marks a match as official or community
  (OQ-0006-21, community servers): adding one is design for later". The note states a rule for when community servers
  exist; it names no field, record, message or check. The parent `[LATER]` (VISION_V2:67) keeps the mechanism later,
  which is what part 12 says. See finding 1 for the one reading risk.
- **(2) C1 holds.** VISION_V2:72-73: "Its open question in the drafts: ADR 0006, OQ-0006-21 (COMMUNITY-SERVERS, his
  words of 2026-11-03: "keep the door open, don't design it")." The id and label match the register (README:160) and
  the quote matches ADR 0006:1137-1138 exactly. It claims nothing about what the drafts keep possible, so it cannot
  clash with part 12's "No field". True to the drafts.
- **(3) No engine need, nothing designed.** `git diff ee736ec 26a5455 -- docs/VISION_V2.md` touches §1 header, pillar
  5, §5 and §7 only; §6 (VISION_V2:75-94 at head) is unchanged, its only progression item still §6.10 `[LATER]`
  Accounts (:94). The note names no server program, config file, version/content-hash check or match flag (the
  "must not lock it out" list stays only in ADR 0006:1139-1141, untouched). The register's "Needs outside VISION_V2 §6"
  (README:294-298+) is unchanged; J3 (README:408) "no feature for `[LATER]` or `[OPEN]` items" is respected.
- **(4) C8 holds.** README diff is one changed line (:160); the clause "His note in VISION_V2 §5 (2026-11-03, the
  same day) gives why, the trust rule and the precedents." is appended after "No option is weighed and no timing is
  set.", which stays. STATUS:474-478 ("From B.2 ... **OQ-0006-21's pointer**, for the decision drafts' next revision
  (B.2 edits no ADR): ADR 0006's OQ-0006-21 entry may point at VISION_V2 §5 ..."). No ADR file in the diff stat.
- **Infra review points.** Cost: none (no hosting, CI or provider change; `git diff ee736ec 26a5455 -- .github` is
  empty). Exposure: none. Provider risk: none. Rollback: revert the PR (docs only). Gate `--no-smoke` PASS, symbols
  +0 -0 (`gate-build.txt`). Not verified by me: CI status on head 26a5455 (the `gh pr view` GraphQL call was refused;
  STATUS:103 says CI green on 67978c2).

## Verdict: PASS WITH NOTES

## Findings
1. **Low. The `[DECIDED]` trust rule could be read later as asking for part 12's official/community field now.**
   Evidence: VISION_V2:70 `[DECIDED]` "progression counts on official servers only" vs ADR 0006:514-515 "No field marks
   a match as official or community ... adding one is design for later". Not a contradiction (the rule is a policy for
   when community servers come; the parent bullet at :67 is `[LATER]`; J3 at README:408 bars a feature for it), but a
   future drafts revision reading VISION_V2 alone sees a decided rule and no word that its mechanism is deferred.
   Suggested fix: no edit to the note. Optionally, the STATUS follow-up (STATUS:476-478) or the report adds one clause:
   the trust rule adds no field or check now; part 12's "adding one is design for later" stands.
2. **Low. Whether rented "official private servers" count as "official" for progression is not answered, and the note
   does not answer it (correctly).** Evidence: VISION_V2:69 ("renting out official private servers") beside :70
   ("official servers only"); related open questions R2-ROOM-OQ7 (private matches and Service points) and R6-TEL-OQ2
   (ADR 0006:1145-1147). Infra side: rented official private servers would run on our hosts, so they carry a cost and
   capacity question for later (ADR 0006's cost parts), nothing now. Suggested fix: no edit; the report keeps this
   under what is not decided (advisor condition C2; game-designer's check).

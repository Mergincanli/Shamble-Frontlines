# Mustafa's words for B.2, word for word

Each block is copied byte for byte from the source named above it. Nothing here is a paraphrase.

## 1. B.2, his task list of 2026-11-03
Source: session_0173vVo5sT21uP4DcQtKvmAU, user message at 2026-11-03T22:08:25Z; copied from
`git show 4f268da:.checkpoints/kit-v3-3/queue/mustafa-2026-11-03-list.md`, lines 6, 10 and 12 (the lead line, the
B heading and item B.2). The same words stand in STATUS.md:498-506 at ee736ec ("Queued, in Mustafa's words").

```text
Small change-sets, one at a time, plan first (Mustafa, 2026-11-03).
B. Docs:
 2. VISION_V2: a short "Community servers (later)" note: why (bigger or longer matches, community growth, a safety net if the game grows faster than our funds, possible revenue such as renting out official private servers), the trust rule (progression counts on official servers only), and precedents (Counter-Strike and Rust community servers, Battlefield's rented servers, Minecraft Realms).
```

## 2. COMMUNITY-SERVERS, his answer of 2026-11-03 (about 17:31 Amsterdam), for reference only
Source: the decision-drafts session's chat, recorded word for word in
`docs/reports/2026-11-01-v2-decision-drafts-appendix.md:2218` and in ADR 0006 OQ-0006-21 (lines 1137-1141 at
ee736ec). B.2 does not edit it; the new note points at OQ-0006-21.

```text
- COMMUNITY-SERVERS (new; keep the door open, don't design it): players may one day host their own servers, possibly with revenue such as official private servers they rent. Add it as an open question to ADR 0006 and the README. The drafts must not lock it out: the room server stays one self-contained program with a config file (players, length, rules), checks the game's version and content hash, and marks each match as official or community, so progression can count on official servers only.
```

## 3. B3's hand-over: his words of 2026-11-04 in LOOP_V2 (at ee736ec), which the three VISION_V2 lines meet
Source: `docs/LOOP_V2.md` at ee736ec, lines 74-75 (§5, Factions), 82-84 (§5, B3-Q1 (c)), 136 and 137-138 (§8, B3-Q2
(a)) and 146 (§8, the catalogue). They are already his words in LOOP_V2 (PR #36); B.2 quotes them, it does not
edit LOOP_V2. Line 84 runs on past his words: its last two words, "This covers", start B3's own note (lines
84-87), which is not his.

```text
- `[DECIDED]` Factions: the counter triangle and tier one stay shared; the differences grow with tech (tier-two
  specialists and vehicles are unique). (Mustafa, 2026-11-04; personalities and each faction's units: UNITS.md §0.)
- `[DECIDED]` Shared or per faction: decided unit by unit when Mustafa writes the unit list, with two fixed rules:
  every faction has an anti-vehicle answer from tech 1; and a role every faction has (anti-vehicle, infiltration)
  shares one core with a faction twist on top, like the infiltrators. (Mustafa, 2026-11-04, B3-Q1 (c).) This covers
- `[DECIDED]` Account progression gives breadth only (class variants, cosmetics), never power. (Mustafa, 2026-11-04)
- `[DECIDED]` Account progression gives class variants and cosmetics only. New classes come through the team's
  tech in the match, so nobody is ever missing a counter. (Mustafa, 2026-11-04, B3-Q2 (a).)
- `[LATER]` The unlock catalogue itself. The prototype only records and shows Service points (KVStore).
```

## 4. The three VISION_V2 lines B3 handed over (at ee736ec)
Source: `docs/VISION_V2.md` at ee736ec, lines 36-37, 59 and 86; named in STATUS.md:34-36 and ROADMAP.md:73.

```text
   - `[OPEN]` How much is faction-specific? LOOP_V2 says "infantry shared, tanks per faction". Faction pride pulls
     toward more unique units. Needs a design session.
- `[DECIDED]` `[LATER]` Cosmetics and account progression get designed when their time comes.
- **Faction asymmetry:** shared core infantry plus faction specialists and vehicles, or more? (Pillar 5.)
```

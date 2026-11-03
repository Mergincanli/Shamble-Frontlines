# stop-game-designer fixes (game-designer, ADR 0005 "The ability and effect system")

Rows: C9 A1 A2 A3 B1 B2 LA3. Blocks apply in file order.

### FIX A2
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
- Every GAS fact R1 uses comes from one community write-up (tranek's GASDocumentation, current with UE 5.3); Epic's
  own documentation and the engine source were not read (r1-simulation-core.md, Findings §5, method note, lines
  1185–1189; R1-ABL-OQ1, lines 2756–2758).
AFTER:
- Every GAS fact R1 uses comes from one community write-up (tranek's GASDocumentation, current with UE 5.3); Epic's
  own documentation and the engine source were not read (r1-simulation-core.md, Findings §5, method note, lines
  1185–1189; R1-ABL-OQ1, lines 2756–2758). The Advisor's answer to OQ-0005-AB12 (2026-11-03): the write-up "is
  accepted for concepts", and "any specific claim a record relies on is checked against Epic's documentation once it
  can be read".
END

### NOTE C9
no change: AB-3 already says "The feel kit is a separate client definition keyed by the same id", which is what the
Advisor's answer to OQ-0005-1 decides ("the client's data, keyed by the same id; the content check fails if a
simulation id has no client entry."). AB-4's unit kind names no model and lists the feel kit as "(client only)", so no
simulation kind in this section holds a presentation field. The frame's decider line ("the game-designer changes
AB-4's kinds if the answer needs it", 0005:2055) is met with no change: the answer needs none in AB-3 or AB-4.
END

### NOTE C9-2
no change: the confirmation the verdict asks for (OQ-0005-1 ruling: "engine-architect names the presented kinds in
CD-2, from AP-2, CD-13 and AB-3 (game-designer confirms the ability kinds)"), checked against engine-architect's
CD-2 list (stop-engine-architect-a-fixes.md, FIX C4-2). Of AB-4's kinds, as the drafts stand on 2026-11-03:
- Something to present, so each id needs a client entry keyed by it: weapons (the feel kit, AB-3 and CD-14, and the
  model, AP-2); units and vehicles (the model, AP-2); factions (the colours and the uniform models, CD-13 and AP-2).
  Confirmed as C4-2 lists them.
- Nothing to present of their own, so no entry: the tag registry, the attribute sets, effects, abilities, upgrade
  tracks, and the drop and pickup values. Confirmed as C4-2 lists them. Reasons: the feel kit is per weapon (LOOP_V2
  §7 `[DECIDED]`: "A per-weapon feel kit, kept as data"); an ability's thrown or granted weapon (the Molotov, the
  Banzai blade's Katana) presents through that weapon's entry (AB-3, AB-6); the Javelin's flinch is the weapon's feel
  kit (AB-6); a weapon pickup shows its weapon's model, from the weapon's entry; an attribute or a tag shows only as a
  value or a state the HUD and the soldier read, with no look keyed by its id in any draft.
- Two of those kinds may gain a presentation side once it is designed, and then join the first list in that change,
  as C4-2's last sentence already says ("A kind that gains a presentation side joins the first list in the same
  change"): abilities, whose activation cues AB-1 and AB-9 keep, keyed by (entity, ability or weapon, command
  sequence) (R1-ABL-REC4) and sent to other clients as tick-stamped cosmetic events ("an ability's cue", ADR 0001
  part 6), though no draft gives an ability's cue a look or a sound beyond its weapon's, and ability feel is first
  judged in the Feel Lab test (this section's spike item 8); and upgrade tracks,
  whose offer of three ADR 0003 draws (AB-8; OQ-0005-AB8), though no track content exists and how a candidate is
  shown is not designed. Effects and tags shown on a soldier (a mark, VISION_V2 §6.4; spawn protection, LOOP_V2 §2)
  have no designed look either (what marking does is R1-ABL-OQ6, Mustafa's): they join the same way if one is
  designed.
- So C4-2's two lists stand as written; nothing changes in this section (NOTE C9) or in C4-2.
END

### FIX A3
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
- **The strongest argument against:** every GAS fact behind AB-1 comes from one community write-up (R1-ABL-OQ1), and
  the closest shipped team-shooter precedent, Overwatch's scripted abilities, was not read (R1-ABL-OQ2). So the case
AFTER:
- **The strongest argument against:** every GAS fact behind AB-1 comes from one community write-up (R1-ABL-OQ1; the
  Advisor accepted it "for concepts" on 2026-11-03, OQ-0005-AB12), and the closest shipped team-shooter precedent,
  Overwatch's scripted abilities, was not read (R1-ABL-OQ2). So the case
END

### FIX B2
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
  Rust change. The content and ability spike's marks 1 and 2 test exactly this; OQ-0005-AB13 names the trigger.
AFTER:
  Rust change. The content and ability spike's marks 1 and 2 test exactly this. When scripting comes back is
  OQ-0005-AB13's answer (the Advisor, 2026-11-03): "scripting comes back only when a designed ability can't be built
  from data." A new reviewed generic operation still counts as built from data, so it takes one designed ability that
  needs ability-specific code no operation can express (the advisor's reading, 2026-11-03).
END

### FIX A1
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
  - Proposed: (a), with (b) as a cheap check if the kit adds the domain. Decider: the Advisor. What decides it: whether
    the next kit PR adds Epic's domain. Research ids: R1-ABL-OQ1.
AFTER:
  - Answered by the Advisor (2026-11-03): "tranek's write-up is accepted for concepts; any specific claim a record
    relies on is checked against Epic's documentation once it can be read."
  - Decided: (a), for concepts. The specific claims a record relies on are checked against Epic's documentation once
    it can be read (SRC-ACCESS (a) lets a kit PR add official-docs domains such as Epic's). Decider: the Advisor.
    Research ids: R1-ABL-OQ1.
END

### FIX B1
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
  - Proposed: (a), and (b) whenever a copy appears. Decider: the Advisor (the source). What decides it: the content and
    ability spike's marks 1 and 2 (the trigger). Research ids: R1-ABL-OQ2, R1-ABL-OQ9.
AFTER:
  - Answered by the Advisor (2026-11-03): "scripting comes back only when a designed ability can't be built from
    data."
  - Decided (the advisor's reading, 2026-11-03): a new reviewed generic operation counts as built from data, as AB-1
    and AB-6 already have it ("every new operation is a reviewed Rust change"), so scripting comes back when one
    designed ability needs ability-specific code that no operation can express. New operations being "needed often"
    is no longer a trigger, and (b), reading the talk, stays a weighed option that nothing schedules. Decider: the
    Advisor (the source). Research ids: R1-ABL-OQ2, R1-ABL-OQ9.
END

### NOTE LA3
no change: the advisor's ruling (LATER): "The drafts' own entries for these questions are not edited (LA3). README
gets one sentence and a marker on each row." This section's five entries in the LATER list, OQ-0005-AB1 (0005:1188),
AB2 (:1200), AB3 (:1208), AB14 (:1349) and AB15 (:1367), keep their deciders and their "What decides it" lines as
written; each already names "a design session" (AB3: "the snowball design session VISION_V2 §2 and §7 call for"),
which Mustafa's answer ("a design session after the netcode proof test") times without changing. The same holds for
this section's other design questions (AB4 to AB11), which join that session's agenda with their deciders unchanged.
README's sentence and markers are engine-architect's (LA1).
END


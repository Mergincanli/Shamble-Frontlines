#!/usr/bin/env python3
"""Follow-up batch B: BEFORE pulled by line range from ADR 0003 as it is on disk now; checks each BEFORE occurs once,
no overlaps, applies to a scratch copy for review, writes graphics-engineer-fixes-final-b.md."""
import sys, pathlib
DOC = pathlib.Path(sys.argv[1]); OUT = pathlib.Path(sys.argv[2]); SCR = pathlib.Path(sys.argv[3])
F3 = "docs/adr/0003-rendering.md"
L = DOC.read_text().split("\n")
FIXES = []
def fix(label, s, e, after): FIXES.append((label, s, e, after.strip("\n")))

fix("part 7, OQ-0003-16 routing (the tier bullet names the look checklist's step 31)", 392, 393, """
     shows that frame beside High and legacy, so Mustafa judges the look the minimum class sees (the look
     checklist's step 31). *Need:* VISION_V2 §4; LOOK_V2 (the floor).
""")

fix("PA-7 (part 8 points to OQ-0003-20: the soaked set)", 399, 399, """
     (R3-MEASURE-REC1, Recommendations Q6, lines 1181–1193). On the minimum laptop the same run set is also taken
     after at least 15 minutes of play (the pass marks' soaked set S); which of the two sets gates is OQ-0003-20.
""")

fix("GE-6 (part 10: the teardown rule between matches)", 457, 459, """
    ADR 0002's layout (*answers* R3-XT-OQ7 and R1-XT-OQ16). Nothing in this flow allocates per frame (R3-CHAR-REC3's
    mark; R3-MEASURE-REC3 gates it). *Between matches:* when a match ends, the page disposes what it loaded for that
    match (its models with their geometries and textures, and any render target made for it), keeps the visual-effect
    pools and the renderer, and releases a shared material only once no scene uses it (ADR 0005's GPU part 7: "never
    disposed while shared"); so GPU memory and the render counters (`renderer.info.memory`, R3-MEASURE-F3, lines
    617–622) come back to the same baseline after every match in one tab (PM-0003-45). This is the code rule "dispose
    GPU resources on teardown, never dispose pooled/shared materials" (`CLAUDE.md`, code rules) carried into v2, as
    the Consequences say. *Need:* VISION_V2 §6.2 (aiming at height), §6.9; LOOP_V2 §7, §11; UPGRADE-SCOPE (the track
    display); VISION_V2 §4 for the teardown (the budgets hold match after match).
""")

fix("AD-12 routing (part 11 names the test's trees: GW18)", 472, 473, """
    Beside them run ADR 0005's ported kit builders, for the asset families OQ-0005-AP2 asks Mustafa to judge
    (authored model or procedural builder). The test's trees are legacy's tree builders ported as placeholders (GW18,
    `LEGACY_INVENTORY.md` line 206), judged for variety and lushness only: the tree style stays `[OPEN]` (OQ-0003-9).
""")

fix("AD-12 routing (trace table: the tree style's row names GW18)", 519, 519, """
| §8 `[OPEN]` the tree style | nothing decided; the test's trees are legacy's ported tree builders (GW18), labelled placeholders | – | – | Mustafa (OQ-0003-9 lists it) |
""")

fix("OQ-0003-16 routing (its decider's evidence is the look checklist's step 31)", 903, 904, """
  meanwhile, since the costs are unknown today. Decider: Mustafa. What decides it: the look checklist's step 31
  (the Low frame) and PM-0003-9's costs on the minimum laptop. Research ids: R3-BUDGET-REC3,
""")

fix("PA-7 (new OQ-0003-20, HEAT-SOAK: the Advisor; test-engineer's held block, adapted)", 928, 929, """
  fight view through a colour-blindness simulation, and Mustafa judges it; (c) faction shape cues, left to a later
  design session. Decider: Mustafa. What decides it: his judgement in the look proof test. Research ids: none.
- **OQ-0003-20. Is the minimum laptop's budget judged after 15 minutes of play, or on a cold laptop?**
  Why it matters: a match runs about 20 minutes (LOOP_V2 §8), while one H run takes about 30 s at 30 fps (900 frames:
  300 warm-up and 600 measured; about 110 s in the tail form, 3,300 frames); in a thin laptop the CPU and the
  integrated GPU share one power and heat budget, so a cold run can measure a short boost the same laptop no longer
  holds at minute 15 and pass a row it then fails (engineering experience, not researched: the chosen laptop's
  boost behaviour is unknown). Options: (a) the soaked set S gates PM-0003-18 to PM-0003-21 on ML, and the cold set
  is reported beside it; (b) the cold set only, R3's 3 runs as written (R3-BUDGET-REC2, lines 1162–1164); (c) both
  must pass. (a) and (c) go beyond R3's 3 runs: proposed, for the reason above. Part 8 and the pass marks hold to
  (a) until it is answered. Recommendation: (a), since VISION_V2 §4's "30+ fps on Low" holds over a whole match.
  Decider: the Advisor. What decides it: the Advisor's reading of VISION_V2 §4's "30+ fps on Low" over a whole
  match; the bench's cold and soaked sets, once measured, show how far apart they lie. Research ids: R3-BUDGET-REC2
  (lines 1162–1164, its 3 runs; it stays adopted, no new disposition).
""")

text = "\n".join(L); errs = []
for lab, s, e, a in FIXES:
    b = "\n".join(L[s-1:e]); c = ("\n"+text+"\n").count("\n"+b+"\n")
    if c != 1: errs.append(f"{lab}: {c}")
rng = sorted((s, e, lab) for lab, s, e, a in FIXES)
for (s1, e1, l1), (s2, e2, l2) in zip(rng, rng[1:]):
    if s2 <= e1: errs.append(f"overlap {l1} / {l2}")
if errs: print("\n".join(errs)); sys.exit(1)
new = list(L)
for lab, s, e, a in sorted(FIXES, key=lambda x: -x[1]): new[s-1:e] = a.split("\n")
(SCR / "0003.b.after.md").write_text("\n".join(new))
out = ["# graphics-engineer: follow-up batch B for ADR 0003 (the session's message, about 20:42)", "",
       "BEFORE blocks are whole lines copied by script from docs/adr/0003-rendering.md as it is on disk now (319d3b6 plus",
       "test-engineer's uncommitted pass-mark batch); each occurs exactly once, and no two overlap.", ""]
for lab, s, e, a in FIXES:
    out += [f"### FIX {lab}", f"FILE: {F3}", "BEFORE:", "\n".join(L[s-1:e]), "AFTER:", a, "END", ""]
out += ["### NOTE PA-7 (OQ-0003-20 adopted, adapted)",
        "Adopted with test-engineer's number. Adapted: the run's 900 and 3,300 frames spelt out; the review reference",
        "dropped from the recommendation (the ADR cites research, not reviews); (a) and (c) marked as proposed beyond R3's",
        "3 runs (plan rule 2); part 8 now points to it. engine-architect: a README row for OQ-0003-20 (the Advisor).",
        "END", ""]
OUT.write_text("\n".join(out)); print(len(FIXES), "fixes ok")

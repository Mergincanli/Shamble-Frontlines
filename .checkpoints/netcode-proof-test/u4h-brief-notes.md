# U4h brief — netcode-architect notes
(No Edit tool in this run: the notes file is rewritten whole with Write after each block.)

## Rev 1 (done 2026-11-20; 296 lines; read at head 33dabe8)
Plan: TEST-FIRST rule 3, SIZE-CHECK; the form (u4g-brief rev 2.1, u4f2-brief rev 1.2, advisor-u4g-brief); the plan's U4h
entry, §4.9, §4.14, T14, RQ12, lanes; plan.md and marks.json rows; u4s-carries U4h section and unregistered routes;
crates; runs.txt; size, disk, U4g dependency; write the brief.
Findings kept from rev 1: carry 59's pointer in (u4s-carries.md:284-285); U4h needs Interp + ClientSettings send/clamp
(U4g2 under U4G-Q1 (b)); plan.md -7 line now :724; four planned judge figures (figures.rs:841-860), rule 2 -> his;
SERIES.md:86 sign reversed; unregistered N2-L3 (b), INF-U4R-6 + INF-R2-2, pump_report's RTT; disk ~10.1 GB free after a
part-clean, a check.mjs run fills 6.3-7.1 GiB.
Advisor's check of rev 1: advisor-u4h-brief.md (361 lines), REVISE, one High (ADV-U4HB-1).

## Rev 2 (2026-11-20)
### Plan
1. Read advisor-u4h-brief.md whole (findings :58-142, rulings :143-211, item 1 :212-235, texts :274-331, list :332-343).
2. Check the code facts rev 2 must state: TC inherits CO's weapon and health (scene.rs:236-246, :263); the target dies
   at 10 hits (world.rs:1683-1705); Shot and Hit carry (slot, seq) (seam.rs:466-471; world.rs:1618-1621, :1688-1693);
   Scene's fields crate-private (scene.rs:100-135), `with_stand_ins` the one builder (:328-331); the controller's margin
   (client lib.rs:634-646); ClientSettings in the catalogue (docs/design/netcode-proof-messages.md:96).
3. Apply list items 1-8 exactly in u4h-brief.md, marked rev 2 with a change list at the top.
4. Recompute section 6 item by item (ADV-U4HB-9's additions placed by crate); put the figures into the three texts.
5. List the eight register items for the main session (item 1); leave u4s-carries.md alone.
6. One line per change below as each is made.
STATUS: DONE. u4h-brief.md rev 2, 369 lines. Back to the advisor for sections 1-4, 6 and 8 only.

### Changes (one line each)
- Facts checked: TC = `..Scene::course()` (scene.rs:263), CO weapon spread 5,000 µrad, 30 rounds, 2 s reload, health 100
  (:236-246); damage() kills a non-plate at health 0, respawn 1 s (world.rs:1683-1705), so CO's 100 kills TC's target
  every 10 hits: added beside item 1 (mine, flagged as such in the rev list). Shot/Hit carry (slot, seq): the shot id's
  key. Scene fields pub(crate) (scene.rs:100-135): Q6's "no sim code" needs one `with_*` builder: flagged for the advisor.
  The catalogue: ClientSettings sent "when its delay changes" (netcode-proof-messages.md:96), so a default is needed:
  Interp's starting 3 intervals (u4g-brief.md:88).
- Item 1: block 1 (TC weapon spread 0, magazine/reload met via the own-state record, health above damage x shots, header
  via carry 60, synthetic TC entry re-pinned only if moved); 1.2 lane (crates/sim scene.rs); 1.3 TC row; section 3 row
  (server ray through the aim point; target survives; plant: CO's spread); section 9 register item 8.
- Item 2: block 6 (press edges only, reruns fire nothing: u4-build-plan.md:247-252), block 7 (shot id = (shooter slot,
  firing seq), SERIES.md definition, contract assert; view test only for fired presses; refused = "not fired", new name;
  contract case with a rerun and a refused press); section 3 rows (reruns/presses; shot id).
- Item 3: Q2's six terms in blocks 2, 6-8 and section 2 (`--cap open|test|rule`, open default clipping at the ring's
  reach; SERIES.md:450 `open`; HISTORY_MS readable; TC B1 seed 3 at the default; SS-P2-s1/ES-P3-s2 re-pinned if moved;
  default delay and the brawl's Interp stamp; the SS P2 brawl row with the 1 % route; R2 takes the span). Q5's five
  terms in block 3 (TOLERANCE_NS 33,330,000, inclusive) and section 8 item 5. Q6 in block 13 and section 2 (grenades
  and stand-ins).
- Item 4: block 2: cap = min(400 ms, RTT + 3 tick periods), from the pump's RTT, the tick period and LEAD-BOUND only,
  reasoned from client/src/lib.rs:634-646; capped waits counted; section 3 row (honest P4 B never meets it; a
  stamp-ahead at 400 ms buys nothing past it); security-auditor's read named in section 3 and 7.
- Item 5: 1.4 (public items by crate, A19 u4-build-plan.md:102): npt-sim's shared blend-and-cast and CHEST_CM pub
  (EA-U4R-4), EA-B1-1's later half if U4g leaves it; the room, server, client, proto, bots items. Block 5: AimCast on the
  shared function via `World::hit_volumes_from` (world.rs:933).
- Item 6: section 6 recomputed by crate: lag half 2,080-3,850 (said 2,100-3,850), aim half 750-1,515 (said 750-1,500),
  judge 400-900; whole without judge 2,830-5,365 (said 2,850-5,350; mid ~4,100); at 1.5: 4,245-8,048; U4h top 5,775;
  U4h2 top 2,273 (stop past 2,300); judge planned ~650, stop past 1,300; judge inside -> ~7,100. Section 8's three texts
  copied from advisor-u4h-brief.md:276-323 with U4h2's top changed 1,550 -> 1,500 (the only figure that moved). J1-J3
  are a change of their own; block 9 keeps the no-answer line.
- Item 7: section 8 rulings Q1-Q3, Q5-Q12 one "Ruled" line each with advisor-u4h-brief.md lines; Q4 "his, text:
  advisor-u4h-brief.md". Q1 reads the margin inside the cap (T14's words), the ring = cap + blend tick (noted). Q8's band
  fixed: half at centre, half uniform within ±2 half-widths (about 1,500 hits, 500 misses).
- Item 8: -10 the calibration's figure (section 2 row); -11 Q10 settle-first; -12 section 9; -13: block 4's hand-off
  (lab and npt-server's tests; src only the accessor; U6's TickLoop), block 7's frame_wait source (lab from the shooter's
  frame record; SERIES.md:211's writer cell flagged under Q7 for the advisor), Q1's under 14 KB, disk table with the
  recorded runs (about 85-95 runs, 1.0-4.0 GiB), 1.1's U4c C2, section 9's after-close line (marks.json:9413), block
  11's "differs".
- Register: section 9 lists the eight items (advisor-u4h-brief.md:217-231); u4s-carries.md untouched.
- Two points left for the advisor's check, not decided here: Q6's scene builder (sim code against "no sim code"); Q7's
  edit taking SERIES.md:211's writer cell.

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

### Changes (one line each)

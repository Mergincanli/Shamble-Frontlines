skeptic2-GA: second skeptic for area GA rows GA1 GA5 GA8 GA15
- facts: all GA1 line refs hold (132-193,153-163,172-177,179-185,187-191,197,231-240,230). No `new` inside updateSoldierAnim 271-444; audit CS11 says anim garbage is V8 boxed floats (AUDIT 111, 78).
- facts: muzzle is child of wpn (core 696-697) -> grenade/molotov launch + arc preview from animated muzzle (game 1406,1733,1760). Bullets from playerFireOrigin 1472.
- facts: extra aim-pitch writer 2928 (wpn.rotation.x*=0.9); editor callers 385/388 (_alSt 323) and 672.
- facts: ROADMAP 44 planted feet Brave-confirmed; ROADMAP 70 draw/bayonet/throws awaiting Brave. LOOP_V2 §4 line 58/63-66 tier1 abilities no bayonet.
- GA1 corrected: refs hold; qualify zero-alloc (source has no new; V8 boxed floats per AUDIT CS11 line 111/78).
- GA5 confirmed: 311 baseY, 313-344 all hold; ROADMAP 44 Brave-confirmed planted feet; ANIMATION_PLAN 6/8.5 hold.
- GA8 corrected: add reachFrac 165; sub-ranges owned by GA9/GA10/GA15; grenade/molotov launch from animated muzzle (core 696-697, game 1406/1733/1760); draw/bayonet awaiting Brave (ROADMAP 70).
- GA15 corrected: extra writer 2928, editor callers 385/388/672 (_alSt 323); bots lean=0 too; STATUS 48-soldier MB is whole update(); runtime 0.18ms/7 ~26us.
DONE

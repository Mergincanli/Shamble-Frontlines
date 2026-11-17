Task: joint-list (engine-architect design ruling for change-set 0: the soldier's engine joint list; output joint-list.md)

## Plan (blocks, in order)
B1. Read LEGACY_INVENTORY rows GA1-GA15, CC9, CC10.
B2. Read ADR 0003 lines 301-323 (part 6) and 790-799 (what ports).
B3. Read ADR 0005 lines 1549-1555 (AP-1) and 1771-1776 (GPU rule 3).
B4. Read the Art lead's briefs in .checkpoints/build-plan-slice-1/inputs/ (blockout part list, addition: three poses, SMG brief).
B5. Read Mustafa's words 2026-11-17 CHANGE 3 (mustafa-2026-11-17-go.md).
B6. Read plan.md "The names this change-set fixes" + advisor-plan-r1.md findings 1, 2, 3, 16.
B7. Read asset-rules.md section 2 (loader facts).
B8. Check legacy code only where a GA/CC row's cited line is needed (rig parents, fist offset, hip/coat).
B9. Write joint-list.md (<=130 lines), sections 1-7.
B10. DONE; answer from notes.

## Notes
- Memory: engine-architect has memory: local; no agent-memory folder exists in this environment (fresh). Nothing to load.

## B1 DONE: LEGACY_INVENTORY GA1-GA16, CC5, CC8-CC11 (docs/LEGACY_INVENTORY.md)
- GA2 (:219): two-bone IK, arms AND legs (shoulder-elbow, hip-knee), pole vector; leg calls game:358,472; arm calls 415,416,427,432,435,486,488; metrics leg IK stretch 0.98, hands on grip/rail 0.0 cm. => needs upper+lower leg joints (knee) and upper+lower arm (elbow).
- GA3 (:220): _animInit measures where the FIST sits beyond the WRIST (palm measure game:243-247), re-hangs boot parts under a PROCEDURAL ANKLE pivot (253-264) "because the rig has no ankle joint"; greatcoat lookup 265-268. v2 note: "A v2 character builder should emit an ankle joint and a hand socket so this fix-up step disappears." Rig facts (ANIMATION_PLAN §5): pelvis y 0.98, thigh 0.46, knee to ankle 0.36, ankle to sole 0.16. Metric: boots below ground 0 cm all gaits.
- GA4 (:221): hip yaw toward movement within twist budget (297-303) => pelvis/hips joint yaws separately from chest.
- GA5 (:222): planted feet; airborne tuck feet 0.18 m above root, pitched 0.25 rad (foot pitch => ankle/foot joint).
- GA6 (:223): pelvis bob/sway/roll + crouch; leg IK onto ANKLE point; boot held flat, turned with foot, toe-off/heel-strike roll (354-360) => ankle joint (foot child of lower leg).
- GA7 (:224): counter-twists SPINE and CHEST toward aim + blade; speed lean, breathing, stride roll, recoil+reload dip on chest; HEAD turns to aim (361-366). => spine + chest + head(neck) joints.
- GA8 (:225): gun-first stance; trigger hand IK onto grip socket; support hand slides along rail; GUN_KIT sockets grip/stock/rail/muzzle/mass/hold/reload (core:785-791); muzzle child of wpn (core:696-697); metric hand on grip/rail 0.0 cm (target <1 cm).
- GA9 (:226): reload: support hand between rail, belt pouch/quiver, gun reload point (hand targets only).
- GA10 (:227): throw: LEFT hand throws, right keeps gun.
- GA11 (:228): shield: left hand turned to face shield + arm solved; katana: right wrist travels, left fist follows hilt; props hung on the hands. Slice 1? (abilities; Riot Shield/Banzai blade are Tier 1 abilities -> check Slice 1 scope).
- GA12 (:229): flinch snaps upper body (gun and hands) + whips head; applied after holds+arm IK; only greatcoat panels updated after.
- GA13 (:230): 4 greatcoat skirt panels gc_skFL/FR/BL/BR rotate WITH THE THIGH ON THEIR SIDE + flare at speed via springs; death: fixed tilt; v2 note "Prefer a shared double-sided material or skinned panels."
- GA14 (:231): death: KNEES buckle into kneel, feet tucked, upper body tips FROM THE PELVIS (inverted pendulum); ROOT never tilts or sinks.
- GA15 (:232): driver contract; LOD needed at 48; "rigid skinning of the parts would cut the per-part draw calls and Object3D matrix updates"; ~51 us/soldier headless, ~26 us runtime.
- CC5 (:136): legacy joint map: pelvis, spine, chest, neck, hips, knees, shoulders, elbows, hands (CHAR_* joints core:454 etc). Rig facts: pelvis y 0.98, chest joint 1.75, shoulders +-0.6, upper arm 0.58, forearm 0.59, hips +-0.21, thigh 0.46. Hit shape separate (core:455/462), visual rig never drives hit.
- CC8 (:138): gun model space: trigger grip near origin, barrel runs +Z.
- CC9 (:139): GUN_KIT sockets grip (trigger hand), stock (butt), rail [zMin,zMax,y] (support hand), muzzle (FX), mass, hold, reload{style,at}; cosmetic only; shots from playerFireOrigin (game:1420).
- CC10 (:140): rest arm poses _ARMP per armMode (core:800-812); armMode enum is the live contract (game:288).
- Next: B2 (ADR 0003 301-323, 790-799).

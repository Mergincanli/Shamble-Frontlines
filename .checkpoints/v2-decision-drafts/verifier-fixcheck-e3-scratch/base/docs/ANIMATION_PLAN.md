# Shamble Frontlines — Animation System (ANIM v2): reference + plan

> What the soldiers' animation is, how it's built, how to extend it, and what's next. Grep targets are names, not
> line numbers. Values tagged **[verified]** were measured on the build (2026-09-26, headless `preview.js`).

## 1. The one rule
Animation is **cosmetic only**. The player fires from `playerFireOrigin()` (a fixed point: 1.9 m up, 0.35 m ahead,
0.3 m to the gun side, along the facing) and bots from their fixed chest point — never from the animated muzzle.
Muzzle flashes and tracers still start at the animated muzzle. Nothing in the animation can change a hit. **[verified]**

## 2. Where it lives
| Piece | File | Grep |
|---|---|---|
| Gun models + sockets | core | `GUN_KIT`, `buildGunModel`, `makeBagpipeGun` … `makeTorqueBow` |
| Part ids on nodes (`userData.pid`), four-panel greatcoat skirt | core | `node.userData.pid`, `gc_skFR` |
| The tick + knobs | game | `function updateSoldierAnim`, `const ANIM`, `_animInit` |
| Held gun per soldier | game | `setHeldGun` (player: `updateHeldModel`; bots: `setEnemyType`) |
| Action hooks | game | `animHit`, `animThrow`, `deathStart`, `animDeath` |
| Anim Lab | editor | `alTick`, `AL_ACTIONS`, `alEnsureUI` (Character Lab → "Anim Lab") |

Signature (unchanged, so every caller kept working): `updateSoldierAnim(g, st, dt)` with
`st = { aimYaw, t, kick, dip, lean, grounded, ads, swap, rel, throwC }` — `rel` = reload progress 0..1 (−1 = none),
`throwC` = grenade charge time (−1 = none), `lean` = bayonet charge, `swap` = weapon-draw timer 1→0.

## 3. Per-frame layers (in order)
1. **Velocity** from position delta (smoothed); teleports re-plant the feet.
2. **Stance weights** (springs): aim (ADS) / sprint carry / low-ready carry; chest **blade** toward the gun side.
3. **Hip yaw**: continuous in the move angle (no flip at the backpedal switch), sprung in world space; the spring
   stiffens with twist, so aim flicks swing the hips round instead of popping them. Twist budget `twistMax`.
4. **Gait**: the pelvis settles into a running crouch (`drop`), and stride is computed from the legs' real reach at
   that height, so feet always land within reach. Duty factor falls with speed (flight phase when running).
5. **Feet**: planted in the world → swing (moves **relative to the moving stance spot**, so it never trails) →
   land. Timed steps for idle re-steps (turn-in-place) and catch-ups (a planted foot left beyond reach).
6. **Pelvis**: small bob (low at mid-stance), sway, roll; a reach cap with a floor as a safety net only.
7. **Leg IK to the ankle** (`solveArmIK`, knees forward) + a **procedural ankle**: boot parts re-hung from a pivot at
   the top of the foot, held flat, turned with the foot, toe-off / heel-strike roll during the swing.
8. **Upper body**: counter-twist to the aim + blade, lean, breath, stride roll; the head faces the aim.
9. **Weapon, gun first**: the stance places the gun's **stock** in the aim frame, then adds reload choreography,
   draw, bayonet thrust, throw lowering, turn sway + bob (by mass), recoil (by mass).
10. **Arm IK onto the gun's sockets**: trigger hand on `grip`; support hand slides along `rail` to the furthest
    point it can reach (hands spread on long guns); reload/throw targets blend over it.
11. **Shield / katana** (arm modes `shield` / `melee`): the hand's orientation is set (shield faces the aim; katana
    blade raised), then the arm is solved; the katana's left fist follows the hilt through the slash.
12. **Hit flinch**: the upper body (gun and hands with it) snaps away from the hit — applied last.
13. **Greatcoat panels** follow the thigh on their side (front/back), flare a touch at speed.
Deaths bypass the tick: `animDeath` buckles the knees, tips the upper body from the pelvis under gravity, one
bounce, settles; the next regular tick (respawn) clears it. The root never tilts or sinks. **[verified]**

## 4. Data: add a gun
1. Model in core (`_gunParts` helpers): grip near the origin, barrel +Z; `userData.part='body'` for thin ink.
2. `GUN_KIT` entry: `grip`, `stock` (butt end), `rail:[zMin,zMax,y]`, `muzzle`, `mass`, `hold` (`rifle` / `smg` /
   `scatter`), `reload:{style, at}` (`ram` / `break` / `drum` / `mag` / `nock`, `at` = where the support hand works).
3. The gun's `WEAPONS.id` must equal its `GUN_KIT` key; `setHeldGun` does the rest.
4. Check in the Anim Lab (every action, every stance), then the metrics below.
New **hold styles** go in `ANIM.hold` (carry / ads / sprint: stock `at`, `rot` [pitch|'aim', yaw, roll], `blade`);
new **reload styles** in `ANIM.reload.styles` (`pose`, `dir`, `keys` = [p, pouch w, socket w, stroke m]).

## 5. Rig facts [verified]
Pelvis y 0.98 · chest joint 1.75 · shoulders chest-local ±0.6 · upper arm 0.58, forearm 0.59 · hips ±0.21 ·
thigh 0.46 · knee → ankle 0.36, ankle → sole 0.16 (procedural ankle) · weapon mount on the chest. The very wide
toy torso vs arm length is why holds blade the chest and the support hand is reach-aware.

## 6. Verification targets (headless snippets; rebuild them from this list if lost)
| Metric | Target | Measured |
|---|---|---|
| Hand on grip / rail, all guns, carry / aim / sprint | < 1 cm | 0.0 cm **[verified]** |
| Hands during every reload | < 1 cm | 0.0 cm **[verified]** |
| Planted-foot drift (ankle over its spot) | ≈ 0 | walk/strafe/backpedal 0.0, sprint mean 0.14 (one push-off frame 5.8 cm) **[verified]** |
| Boots below ground | 0 | 0 cm, all gaits **[verified]** |
| Leg IK stretch | ≤ 0.98 | 0.98 **[verified]** |
| Hip world-yaw jump per frame | ≈ 0 | 0 **[verified]** |
| Katana left fist to hilt during a slash | < 3 cm | 2.7 cm **[verified]** |
| Shield face vs aim (dot) | > 0.98 | 0.993 **[verified]** |
| CPU per soldier per frame (headless) | < 60 µs | ~51 µs **[verified]** |

## 7. Known limits
Props don't move yet (no magazine / shell / grenade meshes in the hand). The left hand throws (the right keeps the gun).
Bots show no reloads, throws or melee (their AI doesn't do them visibly). The Character Lab's pose tool
(`applyRifleHold` / `ANIM.aim`) is legacy — it no longer drives gameplay holds. The Juggernaut walker is separate.

## 8. Next
1. **Props**: magazine / drum / shell / bolt meshes that move with the reload keys; a grenade in the throwing hand.
2. **Bots**: raise the bayonet / shield / katana behaviours to the same system as their AI gains those abilities.
3. **Polish from the Anim Lab**: tune holds per gun, bake Mustafa's "Copy values" JSON into `ANIM`.
4. **Hit reactions by body part** (head snap vs leg stagger), turn-in-place pivot steps, per-faction idle variety.
5. **Juggernaut**: reuse the foot planner for its walker legs.

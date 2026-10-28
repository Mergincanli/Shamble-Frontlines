Task: verify R1 (simulation core) R1-ABL-* recommendations, wave 1 step 3 (verifier, part: abilities)
Scope: R1-ABL-REC1..REC6; basis F1-F19 (REC1: F1-F11,F16,F19; REC2: F9,F17,F22; REC3: F3,F22,F26; REC4: F12-F16,F21; REC5: F6,F8; REC6: none) + sourced numbers (legacy CD12/CD13/CD14/GC5/GC7/GC11/GC15, VISION_V2 §3/§4/§6, LOOP_V2 §4/§6/§7, brief).
Method: raw files curl'd from allow-listed domains (raw.githubusercontent.com, doc.rust-lang.org) into scratchpad dl dir, verbatim grep.
Source GAS: tranek README.md (raw.githubusercontent.com, curl 2026-10-28, 331992 B; line 6 "current with **Unreal Engine 5.3**"). Misses in exact match were only markdown link/backtick markup; checked by eye.
- F1 confirmed: §4.3.2 line 510, both quotes verbatim.
- F2 confirmed; location note: duration table is in §4.5.1 "Gameplay Effect Definition" (l.833-841), not §4.5 proper. Instant/Duration/Infinite quotes verbatim; Duration row adds "that will be removed when the GameplayEffect expires or is manually removed".
- F3 confirmed: §4.5.1 l.843 "`Duration` and `Infinite` `GameplayEffects` have the option of applying `Periodic Effects` that apply its `Modifiers` and `Executions` every `X` seconds as defined by its `Period`." + "**Note:** `Periodic Effects` cannot be [predicted](#concepts-p)."; §4.10 l.2569 "GameplayEffect periodic effects (dots ticking)".
- F4 confirmed (stronger than stated): §4.5.4 l.909-911 formula; summing quote is §4.5.4.1 l.933 ("...before multiplying or dividing them into the `Attribute`'s `BaseValue`."); Override is VERBATIM l.914 "Any `Override` `Modifiers` will override the final value with the last applied `Modifier` taking precedence." (finding labels it inferred).
- F5 confirmed: §4.5.5 l.1041-1050, all quotes verbatim.
- F6 confirmed: §4.2 l.419 (with link markup) + l.427 TagMap quote.
- F7 confirmed: §4.5.7 table l.1081-1082.
- F8 confirmed: §4.5.6 l.1060 + removal policy table l.1068-1070.
- F9 confirmed: §4.5.12 l.1251 ("can not be [predicted](#concepts-p) and they must be implemented in C++."); §4.10 l.2561.
- F10 confirmed: §4.5.14 l.1363; §4.5.15 l.1399.
- F11 confirmed: §4.6.9 l.2089-2093; §5.1 l.2885 ("given the stun `GameplayTag` in their [`Activation Blocked Tags` `GameplayTagContainer`]").
- F12 confirmed: §4.6.8 l.2076.
- F13 confirmed: §4.10 l.2557-2569, lists verbatim.
- F14 confirmed; location §4.10.1 "Prediction Key" l.2598, l.2610 (bullets, with code backticks/bold **all**).
- F15 confirmed: §4.10 l.2551 + l.2553; attribution VERBATIM l.2555 "*Dave Ratti from Epic's comment from the new [Network Prediction Plugin]*". Note "Epic's mindset is to only predict what you 'can get away with'" is the write-up author's words.
- F16 confirmed; location error: first quote §4.8.1 l.2303; "considered unreliable" quote is §4.8.9 "Gameplay Cue Reliability" l.2507, not §4.8/§4.8.1.
- F17 confirmed: §11.1.1 Community Questions 1, Q7 l.3352, answer l.3358; high-latency part VERBATIM in Q7 ("In scenarios of high latency, this would let the Owning Client "try" to activate the ability again...").
- F18 confirmed: §5.5 table l.2942; "always starts at zero each time the game starts" and "client could easily hack their game to only send the same seed value every time" also VERBATIM.
- F19 confirmed: §5.7 l.2959.
Next: Quake III bg_pmove.c (F21, F22), HashMap (F26), then legacy numbers.
Source Q3: bg_pmove.c (curl 2026-10-28, master) + q_shared.h + bg_local.h.
- F21 confirmed: bg_pmove.c l.23 "// bg_pmove.c -- both games player movement code", l.24 "// takes a playerstate and a usercmd as input and returns a modifed playerstate"; PM_Weapon (l.1538) l.1644 "PM_AddEvent( EV_FIRE_WEAPON );".
- F22 confirmed: PM_Weapon l.1576-1577 decrement, l.1589-1590 return, l.1610-1611 release resets to 0, l.1640 ammo--, l.1657-1658 "case WP_MACHINEGUN:" "addTime = 100;", l.1705 "pm->ps->weaponTime += addTime;". Integer: q_shared.h l.1157 "int weaponTime;", bg_local.h l.43 "int msec;". Carry reading correct (weaponTime may be <=0 when += runs). Note: haste/scout divide addTime (l.1693-1702) so not purely constants; one shot per PM_Weapon call.
Source Rust std: doc.rust-lang.org HashMap (page "std1.99.1 (58cf8f9c3 2026-10-13)").
- F26 confirmed: overview "The algorithm is randomly seeded, and a reasonable best-effort is made to generate this seed from a high quality, secure source of randomness provided by the host without blocking the program."; iter "An iterator visiting all key-value pairs in arbitrary order."
Next: legacy numbers in REC1-REC6 (LEGACY_INVENTORY / LEGACY_TUNING / code), VISION_V2 §3 §4 §6, LOOP_V2 §4 §6 §7, brief example pass mark.
Sourced numbers (repo at 351d780):
- N1 48 soldiers = VISION_V2 §3 l.44 "12v12v12 or 24v24": confirmed.
- N2 brief example: V2_PHASE0_BRIEF.md l.138-139 "48 simulated players at a 30 Hz server tick within X ms": confirmed.
- N3 VISION_V2 §4 l.54 "Matches feel fair up to about 100 ms of ping.": confirmed.
- N4 Shabzon fireInterval 0.072 (core:114) -> 13.9/s: confirmed (13.9 stated in GC5 row, not CD12; REC2 cites CD12 - minor).
- N5 GC5 (LEGACY_INVENTORY l.244) "10.0 / 12.0 / 13.1 shots per second at 30 / 60 / 144 Hz, against 13.9": confirmed; 28 % = (13.9-10.0)/13.9 = 28.1 % arithmetic ok. Not re-measured (no benchmarks).
- N6 CD13 (l.94) "a Warrior kills a Conscript in 0.86 s": confirmed + recomputed: Warrior wpn 2 = shabzon dmg 8, Conscript maxHealth 100 (core:128) -> 13 shots, 12 x 0.072 = 0.864 s.
- N7 head 2.6 / body 1.0 / legs 0.65 core:88; Bagpipe headMult 1.2 core:112: confirmed.
- N8 Shabzon falloffStart 15, falloffEnd 35, falloffMin 0.2 core:114; falloffMult game:1484: confirmed.
- N9 Torque Bow fuse 0.55 game:1787 BOLT_FUSE=0.55: confirmed.
- N10 Shabzon damage 8 core:114: confirmed.
- N11 Banzai 1.3x speed game:2774, shieldMoveMult 0.72 core:85: confirmed.
- N12 LOOP_V2 §6 l.86 "Stolen items only last until the Hammer dies.": confirmed (it is a tank's *item* taken on "the last hit or tick", REC1 mark 3 says "granted ability ... on-kill": interpretation).
- N13 LOOP_V2 §4 l.50/58 abilities locked until Tech 1: confirmed.
- N14 LOOP_V2 §7 l.90-94 feel kit, split feedback, active reload "short damage boost": confirmed.
- N15 VISION_V2 §6.4 l.71 "(leech, marking)": confirmed for names; "effect bound to source's life" (leech) supported by LOOP_V2 §6; "marking = effect that grants a tag" is inference.
- N16 CS1 (l.169) input command "(sequence, tick, move, aim, buttons)": confirmed.
- Proposed numbers (1e-6, 1 %, 3 seeds/120 s, 1,000 presses, 10,000 fuzz, 8 effects, 1 ms): all marked proposed.
Rec-level claims:
- C1 REC1 "F5 says GAS's own policies are only documented in tooltips": unsupported. Source §4.5.5 l.1050 only: "They have helpful hover tooltips in the `GameplayEffect` Blueprint." No "only". (F5's "explained only by editor tooltips" overstates too.)
- C2 REC2 reason "both keep the hottest path ... out of effect bookkeeping": unsupported as F17's motive. Source §11.1.1 Q7 motive is latency: "Higher latency connections would have a lower ROF with basic attacks." ... "I would recommend this if this is a critical problem for your game." Conclusion (B1) still follows, for a different reason.
Flags (ranked): 1 REC6 rests on nothing (disclosed; numbers proposed; serves no §6 item, only §3/§4). 2 REC4 predicts cooldowns/weapon timer but omits F17's Paragon latency/ROF problem (server overwriting predicted cooldowns). 3 REC2 reason misreads F17 (C2). 4 REC4 predicts Banzai speed/granted tags but not effect removal (F13) -> predicted buffs linger ~ until server's removal arrives; unaddressed. 5 REC3 body uses F18 but Rests on omits it. 6 REC5 rests on F6/F8 only; its real basis (legacy CD12-14, CS3, CC9) is not in its list; F6/F8 do not choose E1 over alternatives. 7 REC1 C1 overstatement. 8 REC1 §6.4 marking-as-tag and mark 3 "ability" vs LOOP_V2 "item": inference. 9 single-source: all GAS facts from tranek (UE 5.3, tier other), disclosed OQ1. 10 labels/locations: inferred-but-verbatim F4 Override, F15 attribution, F17 latency, F18 zero/forgeable; locations F2/F3 §4.5.1, F4 summing §4.5.4.1, F14 §4.10.1, F16 second quote §4.8.9.
## Table (id · claim · source · verdict · location)
F1 attribute base+current · tranek GAS README · confirmed · §4.3.2 l.510
F2 three duration types · tranek · confirmed · §4.5.1 table l.839-841 (doc says §4.5)
F3 periodic, not predicted · tranek · confirmed · §4.5.1 l.843; §4.10 l.2569
F4 aggregation formula, mult summed, override · tranek · confirmed · §4.5.4 l.909-914; §4.5.4.1 l.933
F5 stacking Duration/Infinite, by source/target, policies · tranek · confirmed ("only by tooltips" overstated) · §4.5.5 l.1041-1050
F6 hierarchical, counted tags · tranek · confirmed · §4.2 l.419, l.427
F7 granted tags, ongoing requirements · tranek · confirmed · §4.5.7 l.1081-1082
F8 effects grant abilities, removal policy · tranek · confirmed · §4.5.6 l.1060-1070
F9 executions not predicted, C++ · tranek · confirmed · §4.5.12 l.1251; §4.10 l.2561
F10 cost Instant, cooldown Duration + tag · tranek · confirmed · §4.5.14 l.1363; §4.5.15 l.1399
F11 required/blocked/cancel tags; stun · tranek · confirmed · §4.6.9 l.2089-2093; §5.1 l.2885
F12 Local Predicted client first · tranek · confirmed · §4.6.8 l.2076
F13 predicted / not-predicted lists · tranek · confirmed · §4.10 l.2557-2569
F14 prediction key round trip · tranek · confirmed · §4.10.1 l.2598, l.2610 (doc says §4.10)
F15 Paragon/Fortnite no damage prediction; minimum · tranek · confirmed · §4.10 l.2551-2555
F16 cues non-gameplay, unreliable · tranek · confirmed · §4.8.1 l.2303; §4.8.9 l.2507 (doc says §4.8.1)
F17 Fortnite own weapon-rate bookkeeping · tranek · confirmed · §11.1.1 Q7 l.3352-3358
F18 int16 activation key as seed · tranek · confirmed · §5.5 l.2942
F19 Paragon slows: strongest only · tranek · confirmed · §5.7 l.2959
F21 Q3 shared player code, fire event · bg_pmove.c · confirmed · l.23-24, PM_Weapon l.1644
F22 Q3 int ms timer, += carry, release reset, ammo, switch · bg_pmove.c (+q_shared.h l.1157) · confirmed · PM_Weapon l.1576-1705
F26 HashMap random seed, arbitrary order · doc.rust-lang.org std 1.99.1 · confirmed · overview, iter
N1-N16 numbers · VISION_V2/BRIEF/LOOP_V2/LEGACY_INVENTORY/code · confirmed (16)
C1 REC1 "GAS policies only documented in tooltips" · tranek §4.5.5 l.1050 · unsupported
C2 REC2 F17 motive = hot path · tranek §11.1.1 Q7 · unsupported (source motive is latency)
Counts: 38 confirmed, 0 wrong, 2 unsupported.
DONE
